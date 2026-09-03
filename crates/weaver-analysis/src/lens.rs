//! conforms: analysis-control-gates-the-reading
//! conforms: analysis-lens-refuses-other-weights
//!
//! The lens artifact, loaded and applied, per `weaver-analysis-Spec`
//! sections 3 and 5. The manifest is judged whole before the file is
//! opened, the header's tensor names answer before any tensor's data
//! materializes - which is what the format election bought - and the
//! matrices are then held to the manifest one layer for one.
//!
//! **The application is the source's own arithmetic, restated rather than
//! invented**: `unembed(J_l @ h)` - the transport at the layer the column
//! came from, then the model's own final norm and unembedding. The weights
//! for that step are the artifact's own, the same safetensors the
//! manifest's hash identifies, so no inference runtime enters: a matrix
//! multiply, a norm, and a second multiply are the whole of it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// The manifest beside the matrices, per Spec section 3. Every member the
/// identity rests on is required, so a manifest missing one is refused as
/// malformed rather than read past.
#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub lens: String,
    pub fitted_for: FittedFor,
    pub lens_shape: LensShape,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FittedFor {
    pub model: String,
    pub model_safetensors_sha256: WeightsDigest,
    #[serde(default)]
    pub dtype: String,
}

/// The weights' content hash, in the shape the model on disk takes, per
/// Spec section 3: one digest for a model kept in one file, and for a
/// sharded model a map from each shard's file name to its digest. **The
/// shape is the model's and not the manifest author's choice**, because the
/// reader recomputes against the files it opens, and a single digest over a
/// sharded model would name a file that does not exist to hash.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WeightsDigest {
    OneFile(String),
    Sharded(BTreeMap<String, String>),
}

impl WeightsDigest {
    /// Every digest the manifest carries, for judging their spelling.
    pub fn digests(&self) -> Vec<&String> {
        match self {
            WeightsDigest::OneFile(digest) => vec![digest],
            WeightsDigest::Sharded(shards) => shards.values().collect(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LensShape {
    pub d_model: u32,
    pub source_layers: Vec<u32>,
    #[serde(default)]
    pub n_prompts: u64,
}

/// Why a lens was refused. Every arm names the member that disagreed: a
/// reader that said only "refused" would send its operator to guess among
/// the identity's parts.
#[derive(Debug, Clone, PartialEq)]
pub enum LensRefusal {
    ManifestUnreadable {
        detail: String,
    },
    ManifestNamesAnotherLens {
        named: String,
        held: String,
    },
    DigestMalformed {
        digest: String,
    },
    LayerSetUnsorted {
        layers: Vec<u32>,
    },
    LayerSetEmpty,
    ArtifactUnreadable {
        detail: String,
    },
    /// The artifact's tensor names against the manifest's layer set, read
    /// from the header before any data: a missing layer and an extra
    /// tensor alike.
    LayersDisagree {
        artifact: Vec<u32>,
        manifest: Vec<u32>,
    },
    WidthDisagrees {
        artifact: u32,
        manifest: u32,
    },
    /// The weights in hand are not the ones the fit ran against, the hash
    /// recomputed rather than trusted from the name, and the file named so
    /// a sharded model says which shard.
    WeightsDisagree {
        file: String,
        held: String,
        manifest: String,
    },
    /// The manifest names one file and a directory was handed, or the
    /// reverse: the identity's shape and the model's disagree before any
    /// hash is taken.
    WeightsShapeDisagrees {
        manifest: String,
        held: String,
    },
    /// A shard the read must open that the manifest's map does not name.
    /// An unnamed shard is not "unverified", it is a file the fit never
    /// saw, so it refuses like a wrong digest does.
    ShardUnnamed {
        shard: String,
    },
    /// A sharded digest naming no shard at all.
    ShardSetEmpty,
}

/// The manifest that identifies a lens, derived from the lens's own name:
/// the fit writes `jacobian_lens_...{tag}.safetensors` beside
/// `lens-manifest{tag}.json`, so a tagged lens meets its own manifest and
/// never an untagged sibling's.
pub fn manifest_path_for(lens: &Path) -> PathBuf {
    let name = lens.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let tag = name
        .strip_suffix(".safetensors")
        .and_then(|stem| stem.split_once("bf16"))
        .map(|(_, tail)| tail)
        .unwrap_or("");
    lens.parent()
        .unwrap_or(Path::new("."))
        .join(format!("lens-manifest{tag}.json"))
}

/// The manifest read and judged whole, before the artifact is opened.
pub fn read_manifest(lens: &Path) -> Result<Manifest, LensRefusal> {
    let path = manifest_path_for(lens);
    let text = std::fs::read_to_string(&path).map_err(|error| LensRefusal::ManifestUnreadable {
        detail: format!("{}: {error}", path.display()),
    })?;
    let manifest: Manifest =
        serde_json::from_str(&text).map_err(|error| LensRefusal::ManifestUnreadable {
            detail: error.to_string(),
        })?;
    let named = lens.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if manifest.lens != named {
        return Err(LensRefusal::ManifestNamesAnotherLens {
            named: manifest.lens,
            held: named.to_string(),
        });
    }
    // **A digest that cannot be one refuses here**, before the weights are
    // hashed: the comparison reads the whole file, so a malformed value
    // would cost that read to reach a mismatch it could never avoid, and
    // would name the weights where the manifest was at fault.
    let digests = manifest.fitted_for.model_safetensors_sha256.digests();
    if digests.is_empty() {
        return Err(LensRefusal::ShardSetEmpty);
    }
    for digest in digests {
        if digest.len() != 64
            || !digest
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(LensRefusal::DigestMalformed {
                digest: digest.clone(),
            });
        }
    }
    if manifest.lens_shape.source_layers.is_empty() {
        return Err(LensRefusal::LayerSetEmpty);
    }
    let mut sorted = manifest.lens_shape.source_layers.clone();
    sorted.sort_unstable();
    sorted.dedup();
    if sorted != manifest.lens_shape.source_layers {
        return Err(LensRefusal::LayerSetUnsorted {
            layers: manifest.lens_shape.source_layers,
        });
    }
    Ok(manifest)
}

/// The fitted transport: one matrix per source layer, row-major
/// `[d_model, d_model]`, held as `f32`.
pub struct Lens {
    matrices: BTreeMap<u32, Vec<f32>>,
    d_model: usize,
}

impl Lens {
    /// The artifact opened against its manifest: the header's names first,
    /// then the data, then the width. Nothing materializes before the
    /// names agree.
    pub fn open(lens: &Path, manifest: &Manifest) -> Result<Lens, LensRefusal> {
        let held = std::fs::read(lens).map_err(|error| LensRefusal::ArtifactUnreadable {
            detail: format!("{}: {error}", lens.display()),
        })?;
        let file = safetensors::SafeTensors::deserialize(&held).map_err(|error| {
            LensRefusal::ArtifactUnreadable {
                detail: error.to_string(),
            }
        })?;
        // **Every name is accounted for.** A name this reader cannot parse
        // is an artifact carrying something the manifest does not describe,
        // and dropping it silently would let an extra tensor ride along
        // unnamed - the absence the layer check exists to catch.
        let mut named: Vec<u32> = Vec::new();
        for name in file.names() {
            match name.parse() {
                Ok(layer) => named.push(layer),
                Err(_) => {
                    return Err(LensRefusal::ArtifactUnreadable {
                        detail: format!(
                            "the artifact holds a tensor named {name:?},                                          which is no layer index"
                        ),
                    });
                }
            }
        }
        named.sort_unstable();
        if named != manifest.lens_shape.source_layers {
            return Err(LensRefusal::LayersDisagree {
                artifact: named,
                manifest: manifest.lens_shape.source_layers.clone(),
            });
        }
        let d_model = manifest.lens_shape.d_model as usize;
        let mut matrices = BTreeMap::new();
        for layer in &named {
            let view = file.tensor(&layer.to_string()).map_err(|error| {
                LensRefusal::ArtifactUnreadable {
                    detail: error.to_string(),
                }
            })?;
            if view.shape() != [d_model, d_model] {
                return Err(LensRefusal::WidthDisagrees {
                    artifact: *view.shape().first().unwrap_or(&0) as u32,
                    manifest: manifest.lens_shape.d_model,
                });
            }
            matrices.insert(*layer, f32_of(view.data(), view.dtype())?);
        }
        Ok(Lens { matrices, d_model })
    }

    pub fn source_layers(&self) -> Vec<u32> {
        self.matrices.keys().copied().collect()
    }

    pub fn d_model(&self) -> usize {
        self.d_model
    }

    /// `J_l @ h`, the transport at one layer. `None` where the lens holds
    /// no matrix for the layer, which the caller reads as a layer outside
    /// the fit rather than an error of the artifact.
    pub fn transport(&self, layer: u32, residual: &[f32]) -> Option<Vec<f32>> {
        let matrix = self.matrices.get(&layer)?;
        if residual.len() != self.d_model {
            return None;
        }
        let mut out = vec![0.0f32; self.d_model];
        for (row, slot) in out.iter_mut().enumerate() {
            let base = row * self.d_model;
            *slot = matrix[base..base + self.d_model]
                .iter()
                .zip(residual)
                .map(|(m, h)| m * h)
                .sum();
        }
        Some(out)
    }
}

/// The files a read opens for the weights it was handed: the file itself,
/// or for a directory the shards its own index names as holding the head
/// and the final norm, and nothing else. **The index is the model's own
/// statement of where its tensors live**, so following it is reading the
/// model rather than guessing at it, and a directory without one refuses.
pub fn shards_for(weights: &Path) -> Result<Vec<PathBuf>, LensRefusal> {
    if weights.is_file() {
        return Ok(vec![weights.to_path_buf()]);
    }
    if !weights.is_dir() {
        return Err(LensRefusal::ArtifactUnreadable {
            detail: format!("{}: neither a file nor a directory", weights.display()),
        });
    }
    let index_path = weights.join("model.safetensors.index.json");
    let text =
        std::fs::read_to_string(&index_path).map_err(|error| LensRefusal::ArtifactUnreadable {
            detail: format!("{}: {error}", index_path.display()),
        })?;
    #[derive(Deserialize)]
    struct Index {
        weight_map: BTreeMap<String, String>,
    }
    let index: Index =
        serde_json::from_str(&text).map_err(|error| LensRefusal::ArtifactUnreadable {
            detail: format!("{}: {error}", index_path.display()),
        })?;
    let head = index
        .weight_map
        .get("lm_head.weight")
        .or_else(|| index.weight_map.get("model.embed_tokens.weight"))
        .ok_or_else(|| LensRefusal::ArtifactUnreadable {
            detail: "the index names no unembedding tensor".to_string(),
        })?;
    let norm = index.weight_map.get("model.norm.weight").ok_or_else(|| {
        LensRefusal::ArtifactUnreadable {
            detail: "the index names no final norm".to_string(),
        }
    })?;
    let mut shards = vec![head.clone(), norm.clone()];
    shards.sort();
    shards.dedup();
    Ok(shards
        .into_iter()
        .map(|shard| weights.join(shard))
        .collect())
}

/// The identity judged against the files the read will open, each hash
/// recomputed against bytes in hand and never trusted from a name. A
/// one-file digest meets one file, a sharded map meets each opened shard
/// under its own name, and the two shapes crossing refuse before a byte is
/// hashed. Returns what was verified, file name and digest, for the record.
pub fn verify_weights(
    weights: &Path,
    manifest: &Manifest,
) -> Result<Vec<(String, String)>, LensRefusal> {
    let shards = shards_for(weights)?;
    let name_of = |path: &Path| -> String {
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string()
    };
    let mut verified = Vec::new();
    match &manifest.fitted_for.model_safetensors_sha256 {
        WeightsDigest::OneFile(expected) => {
            if weights.is_dir() {
                return Err(LensRefusal::WeightsShapeDisagrees {
                    manifest: "one file".to_string(),
                    held: "a sharded model".to_string(),
                });
            }
            let held = sha256_hex_of_file(&shards[0]).map_err(|error| {
                LensRefusal::ArtifactUnreadable {
                    detail: format!("{}: {error}", shards[0].display()),
                }
            })?;
            if &held != expected {
                return Err(LensRefusal::WeightsDisagree {
                    file: name_of(&shards[0]),
                    held,
                    manifest: expected.clone(),
                });
            }
            verified.push((name_of(&shards[0]), held));
        }
        WeightsDigest::Sharded(map) => {
            if weights.is_file() {
                return Err(LensRefusal::WeightsShapeDisagrees {
                    manifest: "a sharded model".to_string(),
                    held: "one file".to_string(),
                });
            }
            for shard in &shards {
                let name = name_of(shard);
                let expected = map.get(&name).ok_or_else(|| LensRefusal::ShardUnnamed {
                    shard: name.clone(),
                })?;
                let held =
                    sha256_hex_of_file(shard).map_err(|error| LensRefusal::ArtifactUnreadable {
                        detail: format!("{}: {error}", shard.display()),
                    })?;
                if &held != expected {
                    return Err(LensRefusal::WeightsDisagree {
                        file: name,
                        held,
                        manifest: expected.clone(),
                    });
                }
                verified.push((name, held));
            }
        }
    }
    Ok(verified)
}

/// The model's own final norm and unembedding, read from the same weights
/// the manifest's hash identifies. **No inference runtime**: the norm is
/// the family's RMS form and the unembedding is one matrix multiply
/// against the head where the model has one and against the embedding
/// matrix where it ties the two, per the model's own configuration.
pub struct Unembedding {
    embedding: Vec<f32>,
    norm: Vec<f32>,
    epsilon: f32,
    width: usize,
    vocabulary: usize,
}

impl Unembedding {
    /// Read the two tensors the readout needs from a model's safetensors,
    /// opening only the files `shards_for` names. Where no head tensor
    /// stands the embedding is the head, per the model's own configuration,
    /// which is the tied case the 0.5b presents and the 8B does not.
    pub fn open(weights: &Path, epsilon: f32) -> Result<Unembedding, LensRefusal> {
        let mut head: Option<(Vec<f32>, Vec<usize>)> = None;
        let mut embedding: Option<(Vec<f32>, Vec<usize>)> = None;
        let mut norm_values: Option<Vec<f32>> = None;
        for shard in shards_for(weights)? {
            let held = std::fs::read(&shard).map_err(|error| LensRefusal::ArtifactUnreadable {
                detail: format!("{}: {error}", shard.display()),
            })?;
            let file = safetensors::SafeTensors::deserialize(&held).map_err(|error| {
                LensRefusal::ArtifactUnreadable {
                    detail: format!("{}: {error}", shard.display()),
                }
            })?;
            if head.is_none()
                && let Ok(tensor) = file.tensor("lm_head.weight")
            {
                head = Some((
                    f32_of(tensor.data(), tensor.dtype())?,
                    tensor.shape().to_vec(),
                ));
            }
            if embedding.is_none()
                && let Ok(tensor) = file.tensor("model.embed_tokens.weight")
            {
                embedding = Some((
                    f32_of(tensor.data(), tensor.dtype())?,
                    tensor.shape().to_vec(),
                ));
            }
            if norm_values.is_none()
                && let Ok(tensor) = file.tensor("model.norm.weight")
            {
                norm_values = Some(f32_of(tensor.data(), tensor.dtype())?);
            }
        }
        let (head_values, shape) =
            head.or(embedding)
                .ok_or_else(|| LensRefusal::ArtifactUnreadable {
                    detail: "no unembedding tensor in the files opened".to_string(),
                })?;
        let norm_values = norm_values.ok_or_else(|| LensRefusal::ArtifactUnreadable {
            detail: "no final norm in the files opened".to_string(),
        })?;
        if shape.len() != 2 {
            return Err(LensRefusal::ArtifactUnreadable {
                detail: format!("the unembedding is not a matrix: {shape:?}"),
            });
        }
        // The norm is applied elementwise across the width, and a zip over
        // a short one would normalize a prefix and leave the rest raw -
        // silently, which is the shape of defect the fault rule forbids.
        if norm_values.len() != shape[1] {
            return Err(LensRefusal::WidthDisagrees {
                artifact: norm_values.len() as u32,
                manifest: shape[1] as u32,
            });
        }
        Ok(Unembedding {
            embedding: head_values,
            norm: norm_values,
            epsilon,
            vocabulary: shape[0],
            width: shape[1],
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn vocabulary(&self) -> usize {
        self.vocabulary
    }

    /// The logits for one residual: normalize, then project. Returns
    /// `None` where the residual is not this model's width.
    pub fn logits(&self, residual: &[f32]) -> Option<Vec<f32>> {
        if residual.len() != self.width {
            return None;
        }
        let mean_square = residual.iter().map(|x| x * x).sum::<f32>() / self.width as f32;
        let scale = 1.0 / (mean_square + self.epsilon).sqrt();
        let normalized: Vec<f32> = residual
            .iter()
            .zip(&self.norm)
            .map(|(x, w)| x * scale * w)
            .collect();
        let mut logits = vec![0.0f32; self.vocabulary];
        for (token, slot) in logits.iter_mut().enumerate() {
            let base = token * self.width;
            *slot = self.embedding[base..base + self.width]
                .iter()
                .zip(&normalized)
                .map(|(e, h)| e * h)
                .sum();
        }
        Some(logits)
    }

    /// How many tokens outrank this one: the rank the control reads, taken
    /// without sorting the whole vocabulary.
    pub fn rank_of(logits: &[f32], token: usize) -> Option<usize> {
        let held = *logits.get(token)?;
        Some(logits.iter().filter(|value| **value > held).count())
    }

    /// The top `k` token identifiers, most likely first.
    pub fn top_k(logits: &[f32], k: usize) -> Vec<u32> {
        let mut ordered: Vec<(usize, f32)> = logits.iter().copied().enumerate().collect();
        ordered.sort_unstable_by(|a, b| b.1.total_cmp(&a.1));
        ordered.into_iter().take(k).map(|(t, _)| t as u32).collect()
    }
}

/// The digest a manifest names, recomputed against bytes in hand. Lowercase
/// hex, which is the only spelling a manifest may carry, per Spec section
/// 3.
pub fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// The RMS norm's epsilon, parsed and judged. **A value that parses is not
/// yet a value that norms**: zero or negative puts a non-positive quantity
/// under the reciprocal square root, and either infinity or a NaN carries
/// through every logit the readout produces, so a reading taken under one
/// would be arithmetic wearing a number's clothes. `None` refuses.
pub fn rms_epsilon(text: &str) -> Option<f32> {
    let value: f32 = text.parse().ok()?;
    (value.is_finite() && value > 0.0).then_some(value)
}

/// The digest of a file, read in bounded chunks: identifying a model does
/// not require holding it. Lowercase hex, the only spelling a manifest may
/// carry.
pub fn sha256_hex_of_file(path: &Path) -> std::io::Result<String> {
    use sha2::{Digest, Sha256};
    use std::io::Read;
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut block = vec![0u8; 1 << 20];
    loop {
        let read = file.read(&mut block)?;
        if read == 0 {
            break;
        }
        hasher.update(&block[..read]);
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

/// Tensor bytes as `f32`, from the two dtypes these artifacts carry: the
/// lens is written `F32` and a model's weights are commonly `BF16`. A
/// third dtype refuses rather than being reinterpreted.
fn f32_of(bytes: &[u8], dtype: safetensors::Dtype) -> Result<Vec<f32>, LensRefusal> {
    match dtype {
        safetensors::Dtype::F32 => Ok(bytes
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect()),
        // The upper half of an `f32` is a `bf16`, which is the whole of the
        // conversion: no rounding decision is taken here.
        safetensors::Dtype::BF16 => Ok(bytes
            .chunks_exact(2)
            .map(|b| f32::from_le_bytes([0, 0, b[0], b[1]]))
            .collect()),
        other => Err(LensRefusal::ArtifactUnreadable {
            detail: format!("the tensor is {other:?}, not F32 or BF16"),
        }),
    }
}
