//! The artifact: resolution, the header read, and the weights hash.
//!
//! These are the free steps of the admit, per `weaver-spu-Spec` section 3.
//! Reading what an artifact declares about itself answers what family this is
//! and what its dimensions are **without touching tensor data or the device**,
//! which is the salvaged mechanic the survey names: it converts the common shape
//! of a bad binding, an artifact present and wrong, into a refusal costing no
//! device work.

use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use weaver_types::{ArtifactRef, LifecycleRefusal};

use crate::family::FamilyName;

/// What an artifact declares about itself, read from its header alone.
#[derive(Debug, Clone, PartialEq)]
pub struct ArtifactHeader {
    /// Which container the bytes are in, which selects the backend.
    pub container: Container,
    /// The family this artifact declares, which keys the family registry.
    pub family: FamilyName,
    /// The hidden width, where the header declares one.
    pub hidden_size: Option<u64>,
    /// The layer count, where the header declares one.
    pub layer_count: Option<u64>,
}

/// The two containers this crate reads, which are peers rather than a default
/// and a fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Container {
    /// A GGUF file, served by the llama.cpp backend.
    Gguf,
    /// Safetensors, served by the candle-native backend.
    Safetensors,
}

/// Step one: resolve the binding to an artifact.
///
/// Resolution is a path question and nothing more. Fetching one would make this
/// crate a provisioner of the operator's own artifact, per charter section 4.1,
/// which is why no HTTP client is in the dependency set.
pub fn resolve(reference: &ArtifactRef) -> Result<PathBuf, LifecycleRefusal> {
    let path = PathBuf::from(&reference.0);
    if !path.exists() {
        return Err(LifecycleRefusal::ArtifactUnresolvable);
    }
    Ok(path)
}

/// Step two: read what the artifact declares about itself.
///
/// **This function opens the artifact and reads its header. It touches no
/// device and loads no tensor data.** A directory resolves to its single
/// container file where it holds exactly one, which is the shape a safetensors
/// export takes.
pub fn read_header(path: &Path) -> Result<ArtifactHeader, LifecycleRefusal> {
    let file = if path.is_dir() {
        container_within(path)?
    } else {
        path.to_path_buf()
    };
    let handle = File::open(&file).map_err(|_| LifecycleRefusal::ArtifactUnreadable)?;
    let mut reader = BufReader::new(handle);

    let mut magic = [0u8; 4];
    reader
        .read_exact(&mut magic)
        .map_err(|_| LifecycleRefusal::ArtifactUnreadable)?;

    if &magic == b"GGUF" {
        read_gguf_header(&mut reader)
    } else {
        reader
            .seek(SeekFrom::Start(0))
            .map_err(|_| LifecycleRefusal::ArtifactUnreadable)?;
        read_safetensors_header(&mut reader)
    }
}

/// The single container file inside a directory artifact.
fn container_within(dir: &Path) -> Result<PathBuf, LifecycleRefusal> {
    let mut found = Vec::new();
    let entries = std::fs::read_dir(dir).map_err(|_| LifecycleRefusal::ArtifactUnreadable)?;
    for entry in entries.flatten() {
        let path = entry.path();
        let is_container = path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e == "gguf" || e == "safetensors");
        if is_container {
            found.push(path);
        }
    }
    found.sort();
    match found.len() {
        0 => Err(LifecycleRefusal::ArtifactUnresolvable),
        _ => Ok(found.remove(0)),
    }
}

/// The GGUF header: magic already consumed, then version, tensor count, and the
/// key-value metadata block. Only the metadata is walked, never the tensor data.
fn read_gguf_header<R: Read>(reader: &mut R) -> Result<ArtifactHeader, LifecycleRefusal> {
    let _version = read_u32(reader)?;
    let _tensor_count = read_u64(reader)?;
    let kv_count = read_u64(reader)?;

    let mut family = None;
    let mut hidden_size = None;
    let mut layer_count = None;

    for _ in 0..kv_count.min(4096) {
        let key = read_gguf_string(reader)?;
        let value = read_gguf_value(reader)?;
        match (key.as_str(), &value) {
            ("general.architecture", GgufValue::Text(text)) => family = Some(text.clone()),
            (k, GgufValue::Number(n)) if k.ends_with(".embedding_length") => {
                hidden_size = Some(*n);
            }
            (k, GgufValue::Number(n)) if k.ends_with(".block_count") => layer_count = Some(*n),
            _ => {}
        }
    }

    Ok(ArtifactHeader {
        container: Container::Gguf,
        family: FamilyName(family.ok_or(LifecycleRefusal::ArtifactUnreadable)?),
        hidden_size,
        layer_count,
    })
}

/// The safetensors header: an eight-byte little-endian length, then that many
/// bytes of JSON. The tensor data beyond it is never read.
fn read_safetensors_header<R: Read>(reader: &mut R) -> Result<ArtifactHeader, LifecycleRefusal> {
    let length = read_u64(reader)?;
    if length == 0 || length > 100 * 1024 * 1024 {
        return Err(LifecycleRefusal::ArtifactUnreadable);
    }
    let mut body = vec![0u8; length as usize];
    reader
        .read_exact(&mut body)
        .map_err(|_| LifecycleRefusal::ArtifactUnreadable)?;
    let parsed: serde_json::Value =
        serde_json::from_slice(&body).map_err(|_| LifecycleRefusal::ArtifactUnreadable)?;

    // The family travels in the `__metadata__` block, which is where a
    // safetensors export carries anything that is not a tensor.
    let metadata = parsed.get("__metadata__");
    let family = metadata
        .and_then(|m| m.get("architecture").or_else(|| m.get("model_type")))
        .and_then(|v| v.as_str())
        .ok_or(LifecycleRefusal::ArtifactUnreadable)?;

    Ok(ArtifactHeader {
        container: Container::Safetensors,
        family: FamilyName(family.to_string()),
        hidden_size: metadata
            .and_then(|m| m.get("hidden_size"))
            .and_then(json_number),
        layer_count: metadata
            .and_then(|m| m.get("num_hidden_layers"))
            .and_then(json_number),
    })
}

fn json_number(value: &serde_json::Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|s| s.parse().ok()))
}

/// One GGUF metadata value, reduced to the two shapes this header read uses.
enum GgufValue {
    Text(String),
    Number(u64),
    Other,
}

fn read_u32<R: Read>(reader: &mut R) -> Result<u32, LifecycleRefusal> {
    let mut buffer = [0u8; 4];
    reader
        .read_exact(&mut buffer)
        .map_err(|_| LifecycleRefusal::ArtifactUnreadable)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_u64<R: Read>(reader: &mut R) -> Result<u64, LifecycleRefusal> {
    let mut buffer = [0u8; 8];
    reader
        .read_exact(&mut buffer)
        .map_err(|_| LifecycleRefusal::ArtifactUnreadable)?;
    Ok(u64::from_le_bytes(buffer))
}

fn read_gguf_string<R: Read>(reader: &mut R) -> Result<String, LifecycleRefusal> {
    let length = read_u64(reader)?;
    if length > 64 * 1024 {
        return Err(LifecycleRefusal::ArtifactUnreadable);
    }
    let mut body = vec![0u8; length as usize];
    reader
        .read_exact(&mut body)
        .map_err(|_| LifecycleRefusal::ArtifactUnreadable)?;
    String::from_utf8(body).map_err(|_| LifecycleRefusal::ArtifactUnreadable)
}

/// Read one typed GGUF value, skipping what this header read does not use. The
/// type codes are GGUF's own.
fn read_gguf_value<R: Read>(reader: &mut R) -> Result<GgufValue, LifecycleRefusal> {
    let kind = read_u32(reader)?;
    read_gguf_typed(reader, kind)
}

fn read_gguf_typed<R: Read>(reader: &mut R, kind: u32) -> Result<GgufValue, LifecycleRefusal> {
    let mut skip = |n: usize| -> Result<(), LifecycleRefusal> {
        let mut sink = vec![0u8; n];
        reader
            .read_exact(&mut sink)
            .map_err(|_| LifecycleRefusal::ArtifactUnreadable)
    };
    match kind {
        0 | 1 => {
            let mut b = [0u8; 1];
            reader
                .read_exact(&mut b)
                .map_err(|_| LifecycleRefusal::ArtifactUnreadable)?;
            Ok(GgufValue::Number(b[0] as u64))
        }
        2 | 3 => {
            let mut b = [0u8; 2];
            reader
                .read_exact(&mut b)
                .map_err(|_| LifecycleRefusal::ArtifactUnreadable)?;
            Ok(GgufValue::Number(u16::from_le_bytes(b) as u64))
        }
        4 | 5 => Ok(GgufValue::Number(read_u32(reader)? as u64)),
        6 => {
            skip(4)?;
            Ok(GgufValue::Other)
        }
        7 => {
            skip(1)?;
            Ok(GgufValue::Other)
        }
        8 => Ok(GgufValue::Text(read_gguf_string(reader)?)),
        9 => {
            // An array: element type, count, then the elements. Walked rather
            // than skipped wholesale, because element widths vary.
            let element = read_u32(reader)?;
            let count = read_u64(reader)?;
            for _ in 0..count.min(1024 * 1024) {
                read_gguf_typed(reader, element)?;
            }
            Ok(GgufValue::Other)
        }
        10 | 11 => Ok(GgufValue::Number(read_u64(reader)?)),
        12 => {
            skip(8)?;
            Ok(GgufValue::Other)
        }
        _ => Err(LifecycleRefusal::ArtifactUnreadable),
    }
}

/// The weights hash: BLAKE3 over a canonical manifest, a single file or a
/// walked directory.
///
/// **The empty-string sentinel on every failure path is the property worth
/// carrying verbatim.** A hash that cannot be computed reports that it could
/// not rather than reporting a wrong value, and apex section 8 rests replay on
/// the identity being right. This function therefore returns
/// [`WeightsHash::sentinel`] rather than an error: a caller cannot accidentally
/// treat a failure as a value, because the failure is a value it can test.
///
/// The hash is computed from the bytes this process loaded rather than from a
/// manifest handed to it, and it is computed fresh on each call with no cache
/// across an artifact change, which is what makes the third walk's alteration
/// visible.
pub fn weights_hash(path: &Path) -> crate::residency::WeightsHash {
    match hash_canonical(path) {
        Ok(value) => crate::residency::WeightsHash(value),
        Err(()) => crate::residency::WeightsHash::sentinel(),
    }
}

fn hash_canonical(path: &Path) -> Result<String, ()> {
    let mut hasher = blake3::Hasher::new();
    if path.is_dir() {
        // A walked directory, in sorted order, so the manifest is canonical
        // rather than dependent on readdir order. Each file contributes its
        // relative path and then its bytes, so a rename is a different hash.
        let mut files = Vec::new();
        for entry in walkdir::WalkDir::new(path).sort_by_file_name() {
            let entry = entry.map_err(|_| ())?;
            if entry.file_type().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }
        for file in files {
            let relative = file.strip_prefix(path).map_err(|_| ())?;
            hasher.update(relative.to_string_lossy().as_bytes());
            hash_file_into(&file, &mut hasher)?;
        }
    } else {
        hash_file_into(path, &mut hasher)?;
    }
    Ok(hasher.finalize().to_hex().to_string())
}

fn hash_file_into(path: &Path, hasher: &mut blake3::Hasher) -> Result<(), ()> {
    let mut handle = File::open(path).map_err(|_| ())?;
    let mut buffer = vec![0u8; 1024 * 1024];
    loop {
        let read = handle.read(&mut buffer).map_err(|_| ())?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(())
}
