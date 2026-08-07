//! conforms: spu-weights-hash-at-admit
//! conforms: spu-hash-failure-sentinel
//!
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
use std::os::fd::AsRawFd;
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

/// Step one: resolve the binding to an artifact file.
///
/// Resolution is a path question and nothing more. Fetching one would make this
/// crate a provisioner of the operator's own artifact, per charter section 4.1,
/// which is why no HTTP client is in the dependency set. A directory holding
/// exactly one container file resolves to that file, which is the shape a
/// safetensors export takes, and a directory holding several is refused as
/// ambiguous. **What resolves is a regular file and nothing else**, so a
/// named pipe cannot turn a bad binding into an admission that blocks forever
/// on an open waiting for a writer.
pub fn resolve(reference: &ArtifactRef) -> Result<PathBuf, LifecycleRefusal> {
    let path = PathBuf::from(&reference.0);
    if !path.exists() {
        return Err(LifecycleRefusal::ArtifactUnresolvable);
    }
    // A directory resolves here, at step one, to the single container file it
    // holds. Resolving it later would leave the admission carrying the
    // directory while the header read blessed the file inside, and the loader
    // would then be handed a path it cannot open, refusing as a device
    // condition three steps after the fact that caused it.
    if path.is_dir() {
        return container_within(&path);
    }
    // Only a regular file resolves. A FIFO with a container's name would open
    // and then block until a writer connects, which turns a bad binding into
    // an admission that never returns, where every other malformation on this
    // path is a refusal costing no device work.
    if !path.is_file() {
        return Err(LifecycleRefusal::ArtifactUnresolvable);
    }
    Ok(path)
}

/// An artifact opened once and held open for the whole admit.
///
/// **The open is the check.** A path tested with `is_file` and opened later is
/// two different files if the name is replaced in between, so this type opens
/// first and asks the descriptor what it got. Everything after, the header
/// read, the load, and the hash, goes through this one descriptor, so the three
/// cannot observe three different files.
///
/// The open takes `O_NONBLOCK` so a named pipe cannot block it: a FIFO opens
/// immediately and is then refused by the kind check, where a blocking open
/// would hold the admit forever before any check could run.
pub struct PinnedArtifact {
    file: File,
}

impl PinnedArtifact {
    /// The path that reaches this descriptor's file whatever the original name
    /// now points at. The descriptor holds the inode, so a replacement of the
    /// name does not move it.
    pub fn path(&self) -> PathBuf {
        PathBuf::from(format!("/proc/self/fd/{}", self.file.as_raw_fd()))
    }

    /// The artifact's size, from the descriptor rather than from the name.
    pub fn len(&self) -> Result<u64, LifecycleRefusal> {
        self.file
            .metadata()
            .map(|m| m.len())
            .map_err(|_| LifecycleRefusal::ArtifactUnreadable)
    }

    pub fn is_empty(&self) -> bool {
        self.len().map(|n| n == 0).unwrap_or(true)
    }

    fn rewind(&mut self) -> Result<(), LifecycleRefusal> {
        self.file
            .seek(SeekFrom::Start(0))
            .map(|_| ())
            .map_err(|_| LifecycleRefusal::ArtifactUnreadable)
    }
}

/// Open the resolved artifact and hold it, refusing anything that is not a
/// regular file once opened.
pub fn pin(path: &Path) -> Result<PinnedArtifact, LifecycleRefusal> {
    use std::os::unix::fs::OpenOptionsExt;
    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(nix::libc::O_NONBLOCK)
        .open(path)
        .map_err(|_| LifecycleRefusal::ArtifactUnreadable)?;
    let kind = file
        .metadata()
        .map_err(|_| LifecycleRefusal::ArtifactUnreadable)?;
    if !kind.is_file() {
        return Err(LifecycleRefusal::ArtifactUnresolvable);
    }
    Ok(PinnedArtifact { file })
}

/// Step two: read what the artifact declares about itself.
///
/// **This function opens the artifact file and reads its header. It touches no
/// device and loads no tensor data.**
pub fn read_header(pinned: &mut PinnedArtifact) -> Result<ArtifactHeader, LifecycleRefusal> {
    pinned.rewind()?;
    let mut reader = BufReader::new(&mut pinned.file);

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
        // Regular files only, on the same ground: a FIFO inside a directory
        // artifact would be selected by name and block the open.
        let is_container = path.is_file()
            && path
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| e == "gguf" || e == "safetensors");
        if is_container {
            found.push(path);
        }
    }
    match found.len() {
        0 => Err(LifecycleRefusal::ArtifactUnresolvable),
        1 => Ok(found.remove(0)),
        // A directory holding several containers is ambiguous, and picking one
        // would be this crate deciding which artifact the operator meant. The
        // binding names one artifact; a directory that does not is refused.
        _ => Err(LifecycleRefusal::ArtifactUnresolvable),
    }
}

/// The GGUF header: magic already consumed, then version, tensor count, and the
/// key-value metadata block. Only the metadata is walked, never the tensor data.
fn read_gguf_header<R: Read>(reader: &mut R) -> Result<ArtifactHeader, LifecycleRefusal> {
    let _version = read_u32(reader)?;
    let _tensor_count = read_u64(reader)?;
    let kv_count = read_u64(reader)?;
    // The caps refuse rather than truncate. A walk cut short mid-structure
    // leaves the reader misaligned, so an oversized-but-valid header would
    // parse garbage instead of refusing, and a header claiming more metadata
    // than any real artifact carries is not an artifact this crate reads.
    if kv_count > 4096 {
        return Err(LifecycleRefusal::ArtifactUnreadable);
    }

    let mut family = None;
    let mut hidden_size = None;
    let mut layer_count = None;

    for _ in 0..kv_count {
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

/// How deep a nested GGUF array may go before the read refuses. GGUF metadata
/// has no legitimate deep case, and without a bound a crafted file of repeated
/// array-of-array headers drives the recursion to a stack overflow, an abort
/// reachable from an untrusted artifact at step two of the admit. Every other
/// malformation on this path is a clean refusal, and this one must be too.
const MAX_ARRAY_DEPTH: u32 = 8;

/// Read one typed GGUF value, skipping what this header read does not use. The
/// type codes are GGUF's own.
fn read_gguf_value<R: Read>(reader: &mut R) -> Result<GgufValue, LifecycleRefusal> {
    let kind = read_u32(reader)?;
    read_gguf_typed(reader, kind, 0)
}

fn read_gguf_typed<R: Read>(
    reader: &mut R,
    kind: u32,
    depth: u32,
) -> Result<GgufValue, LifecycleRefusal> {
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
            // than skipped wholesale, because element widths vary. Depth and
            // count refuse rather than truncate, on the same ground the
            // kv-count cap does: a walk cut short leaves the reader misaligned.
            if depth >= MAX_ARRAY_DEPTH {
                return Err(LifecycleRefusal::ArtifactUnreadable);
            }
            let element = read_u32(reader)?;
            let count = read_u64(reader)?;
            if count > 1024 * 1024 {
                return Err(LifecycleRefusal::ArtifactUnreadable);
            }
            for _ in 0..count {
                read_gguf_typed(reader, element, depth + 1)?;
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
/// The hash is computed at admit by reading the artifact, never from a manifest
/// handed to this process, and it is computed fresh on each call with no cache
/// across an artifact change, which is what makes the third walk's alteration
/// visible. **For a single-file artifact it reads the descriptor the load
/// used**, so a name replaced during the load cannot change what is hashed:
/// the descriptor holds the inode. What remains open is a directory-shaped
/// artifact, whose members beyond the loaded container are named rather than
/// pinned, and that closes with the native path that reads them.
pub fn weights_hash(
    reference: &Path,
    pinned: &mut PinnedArtifact,
) -> crate::residency::WeightsHash {
    // A reference naming one file hashes the descriptor the load used, which
    // is what closes the swap window on the single-file case. A reference
    // naming a directory hashes the walked directory, per the Spec's canonical
    // manifest: its members beyond the container are not pinned, so that case
    // keeps the window and the native path is where it closes.
    let hashed = if reference.is_dir() {
        hash_canonical(reference)
    } else {
        pinned.rewind().map_err(|_| ()).and_then(|()| {
            let mut hasher = blake3::Hasher::new();
            hash_reader_into(&mut pinned.file, &mut hasher)?;
            Ok(hasher.finalize().to_hex().to_string())
        })
    };
    match hashed {
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
    hash_reader_into(&mut handle, hasher)
}

fn hash_reader_into<R: Read>(reader: &mut R, hasher: &mut blake3::Hasher) -> Result<(), ()> {
    let mut buffer = vec![0u8; 1024 * 1024];
    loop {
        let read = reader.read(&mut buffer).map_err(|_| ())?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// A minimal GGUF whose one metadata value is an array nested past the
    /// depth bound: each level is `element_type=9` and `count=1`, so the file
    /// is small while the recursion it asks for is not.
    fn nested_gguf(depth: u32) -> Vec<u8> {
        let mut file = Vec::new();
        file.extend_from_slice(b"GGUF");
        file.extend_from_slice(&3u32.to_le_bytes()); // version
        file.extend_from_slice(&0u64.to_le_bytes()); // tensor count
        file.extend_from_slice(&1u64.to_le_bytes()); // kv count
        let key = b"a";
        file.extend_from_slice(&(key.len() as u64).to_le_bytes());
        file.extend_from_slice(key);
        file.extend_from_slice(&9u32.to_le_bytes()); // the value is an array
        for _ in 0..depth {
            file.extend_from_slice(&9u32.to_le_bytes()); // of arrays
            file.extend_from_slice(&1u64.to_le_bytes()); // one element each
        }
        file.extend_from_slice(&0u32.to_le_bytes()); // ending in u8
        file.extend_from_slice(&0u64.to_le_bytes()); // of zero elements
        file
    }

    /// **A crafted GGUF refuses instead of aborting.** Without the depth bound
    /// a file of repeated array-of-array headers drives the recursion to a
    /// stack overflow, an abort reachable from an untrusted artifact at step
    /// two of the admit. Every other malformation on this path is a clean
    /// refusal, and this one must be too.
    ///
    /// Perturbation: remove the `depth >= MAX_ARRAY_DEPTH` branch from the
    /// array arm and this test dies by stack overflow rather than failing an
    /// assertion, which is the abort the bound exists to prevent. Watched
    /// under exactly that removal.
    #[test]
    fn a_deeply_nested_gguf_refuses_rather_than_aborting() {
        let dir = std::env::temp_dir().join(format!(
            "weaver-spu-artifact-{}-{}",
            std::process::id(),
            line!()
        ));
        std::fs::create_dir_all(&dir).expect("a scratch dir");
        let path = dir.join("nested.gguf");
        let mut handle = File::create(&path).expect("a fixture file");
        handle
            .write_all(&nested_gguf(200_000))
            .expect("the fixture is written");
        drop(handle);

        assert_eq!(
            read_header(&mut pin(&path).expect("the fixture pins")),
            Err(LifecycleRefusal::ArtifactUnreadable),
            "a nesting no real artifact carries is a refusal, never an abort"
        );

        // A shallow nesting inside the bound still reads to the family error,
        // which is what shows the bound refuses depth rather than arrays.
        let shallow = dir.join("shallow.gguf");
        let mut handle = File::create(&shallow).expect("a fixture file");
        handle
            .write_all(&nested_gguf(3))
            .expect("the fixture is written");
        drop(handle);
        assert_eq!(
            read_header(&mut pin(&shallow).expect("the fixture pins")),
            Err(LifecycleRefusal::ArtifactUnreadable),
            "no general.architecture key, so the refusal is the absent family"
        );

        std::fs::remove_dir_all(&dir).ok();
    }

    /// **A directory holding several containers refuses on ambiguity** rather
    /// than silently resolving to the alphabetically first.
    ///
    /// Perturbation: restore the sort-and-take-first and this test fails,
    /// because the header read succeeds against whichever file sorts first.
    /// Watched under exactly that restoration.
    #[test]
    fn an_ambiguous_directory_refuses() {
        let dir = std::env::temp_dir().join(format!(
            "weaver-spu-artifact-{}-{}",
            std::process::id(),
            line!()
        ));
        std::fs::create_dir_all(&dir).expect("a scratch dir");
        for name in ["a.gguf", "b.gguf"] {
            let mut handle = File::create(dir.join(name)).expect("a fixture file");
            handle.write_all(&super::tests::nested_gguf(0)).expect("written");
        }
        assert_eq!(
            resolve(&ArtifactRef(dir.to_string_lossy().into_owned())),
            Err(LifecycleRefusal::ArtifactUnresolvable),
            "two containers is an ambiguity, not a choice"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    /// **A named pipe does not resolve.** `File::open` on a FIFO blocks until a
    /// writer connects, so an artifact reference naming one would hold the
    /// admit open indefinitely where every other malformation refuses.
    ///
    /// Perturbation: drop the `is_file` guard from `resolve` and this test
    /// fails. Watched under exactly that removal.
    #[test]
    fn a_named_pipe_does_not_resolve() {
        let dir = std::env::temp_dir().join(format!(
            "weaver-spu-artifact-{}-{}",
            std::process::id(),
            line!()
        ));
        std::fs::create_dir_all(&dir).expect("a scratch dir");
        let fifo = dir.join("pipe.gguf");
        nix::unistd::mkfifo(&fifo, nix::sys::stat::Mode::S_IRWXU).expect("a fifo");

        assert_eq!(
            resolve(&ArtifactRef(fifo.to_string_lossy().into_owned())),
            Err(LifecycleRefusal::ArtifactUnresolvable),
            "a pipe named like a container is not an artifact"
        );
        // And the same inside a directory, where it would be selected by name.
        assert_eq!(
            resolve(&ArtifactRef(dir.to_string_lossy().into_owned())),
            Err(LifecycleRefusal::ArtifactUnresolvable),
            "a directory holding only a pipe holds no container"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    /// **The pin refuses what is not a regular file, judged on the descriptor
    /// rather than on the name.** A pipe opens without blocking because of
    /// `O_NONBLOCK` and is refused by the kind check, where a blocking open
    /// would hold the admit open before any check could run.
    ///
    /// Perturbation: drop the `is_file` branch from `pin` and this test fails.
    /// Watched under exactly that removal.
    #[test]
    fn the_pin_refuses_a_pipe_on_the_descriptor() {
        let dir = std::env::temp_dir().join(format!(
            "weaver-spu-artifact-{}-{}",
            std::process::id(),
            line!()
        ));
        std::fs::create_dir_all(&dir).expect("a scratch dir");
        let fifo = dir.join("pipe.gguf");
        nix::unistd::mkfifo(&fifo, nix::sys::stat::Mode::S_IRWXU).expect("a fifo");
        assert!(
            matches!(pin(&fifo), Err(LifecycleRefusal::ArtifactUnresolvable)),
            "the descriptor says pipe, so the pin refuses"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    /// **A directory artifact resolves to the file inside it, at step one.**
    /// An earlier cut resolved the container inside the header read, so the
    /// admission carried the directory while the judgments blessed the file,
    /// and the loader was handed a path it cannot open, refusing as a device
    /// condition three steps after the cause.
    ///
    /// Perturbation: return the directory itself from `resolve` and this test
    /// fails on the extension assertion. Watched under exactly that change.
    #[test]
    fn a_directory_artifact_resolves_to_its_container_file() {
        let dir = std::env::temp_dir().join(format!(
            "weaver-spu-artifact-{}-{}",
            std::process::id(),
            line!()
        ));
        std::fs::create_dir_all(&dir).expect("a scratch dir");
        let mut handle = File::create(dir.join("only.gguf")).expect("a fixture file");
        handle.write_all(&nested_gguf(0)).expect("written");
        drop(handle);

        let resolved = resolve(&ArtifactRef(dir.to_string_lossy().into_owned()))
            .expect("a one-container directory resolves");
        assert!(
            resolved.is_file(),
            "the admission carries the file, never the directory"
        );
        assert_eq!(resolved.extension().and_then(|e| e.to_str()), Some("gguf"));
        std::fs::remove_dir_all(&dir).ok();
    }
}
