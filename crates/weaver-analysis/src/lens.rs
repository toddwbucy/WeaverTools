//! conforms: analysis-control-gates-the-reading
//! conforms: analysis-threaded-head-is-bit-identical
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
    pub model_safetensors_sha256: String,
    #[serde(default)]
    pub dtype: String,
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
    /// recomputed rather than trusted from the name.
    WeightsDisagree {
        held: String,
        manifest: String,
    },
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
    let digest = &manifest.fitted_for.model_safetensors_sha256;
    if digest.len() != 64
        || !digest
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(LensRefusal::DigestMalformed {
            digest: digest.clone(),
        });
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

/// The model's own final norm and unembedding, read from the same weights
/// the manifest's hash identifies. **No inference runtime**: the norm is
/// the family's RMS form and the unembedding is one matrix multiply
/// against the embedding matrix, which this family ties to its head.
pub struct Unembedding {
    embedding: Vec<f32>,
    norm: Vec<f32>,
    epsilon: f32,
    width: usize,
    vocabulary: usize,
}

impl Unembedding {
    /// Read the two tensors the readout needs from a model's safetensors.
    /// The tied-embedding case is the one this family presents: where no
    /// head tensor stands, the embedding is the head, per the model's own
    /// configuration.
    pub fn open(weights: &Path, epsilon: f32) -> Result<Unembedding, LensRefusal> {
        let held = std::fs::read(weights).map_err(|error| LensRefusal::ArtifactUnreadable {
            detail: format!("{}: {error}", weights.display()),
        })?;
        let file = safetensors::SafeTensors::deserialize(&held).map_err(|error| {
            LensRefusal::ArtifactUnreadable {
                detail: error.to_string(),
            }
        })?;
        let head = file
            .tensor("lm_head.weight")
            .or_else(|_| file.tensor("model.embed_tokens.weight"))
            .map_err(|error| LensRefusal::ArtifactUnreadable {
                detail: format!("no unembedding tensor: {error}"),
            })?;
        let norm =
            file.tensor("model.norm.weight")
                .map_err(|error| LensRefusal::ArtifactUnreadable {
                    detail: format!("no final norm: {error}"),
                })?;
        let shape = head.shape().to_vec();
        if shape.len() != 2 || shape[0] == 0 {
            return Err(LensRefusal::ArtifactUnreadable {
                detail: format!("the unembedding is not a matrix with rows: {shape:?}"),
            });
        }
        let norm_values = f32_of(norm.data(), norm.dtype())?;
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
            embedding: f32_of(head.data(), head.dtype())?,
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
        let workers = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        self.logits_with_workers(residual, workers)
    }

    /// The logits with the split stated: `workers` row ranges, so a watch
    /// can force a real partition whatever the box reports.
    ///
    /// **The head is applied across the cores by disjoint row ranges**, per
    /// Spec section 5: every row's sum still runs in one thread in one
    /// order, so the logits are the single-thread reading's to the bit, and
    /// the exactness the compare rests on is untouched. The standard
    /// library's scoped threads, because the dependency set is four crates
    /// and this is not a reason to make it five.
    pub fn logits_with_workers(&self, residual: &[f32], workers: usize) -> Option<Vec<f32>> {
        let normalized = self.normalized(residual)?;
        let mut logits = vec![0.0f32; self.vocabulary];
        // A head has rows, judged at `open`, so the chunk is never empty.
        let workers = workers.clamp(1, self.vocabulary.max(1));
        let rows_per = self.vocabulary.div_ceil(workers).max(1);
        std::thread::scope(|scope| {
            for (chunk, out) in logits.chunks_mut(rows_per).enumerate() {
                let first = chunk * rows_per;
                let normalized = &normalized;
                scope.spawn(move || {
                    // The ranges are this function's own, so the rows fit.
                    self.logits_rows(normalized, first, out)
                        .expect("a chunk of the head's own rows fits the head");
                });
            }
        });
        Some(logits)
    }

    /// The logits for the rows `first..first + out.len()`, each row's dot
    /// product summed in index order by the one thread that owns it. The
    /// single-thread reading is this over every row at once. `None` where
    /// the residual is not this model's width or the rows run past the
    /// head: a zip over a short residual would sum a prefix and call it a
    /// logit, which is the shape of defect the fault rule forbids.
    pub fn logits_rows(&self, normalized: &[f32], first: usize, out: &mut [f32]) -> Option<()> {
        if normalized.len() != self.width || first.checked_add(out.len())? > self.vocabulary {
            return None;
        }
        for (offset, slot) in out.iter_mut().enumerate() {
            let base = (first + offset) * self.width;
            *slot = self.embedding[base..base + self.width]
                .iter()
                .zip(normalized)
                .map(|(e, h)| e * h)
                .sum();
        }
        Some(())
    }

    /// The normalized residual the head is applied to: the family's RMS
    /// form, exposed so a watch can take the single-thread reading through
    /// `logits_rows` and hold the threaded one to it.
    pub fn normalized(&self, residual: &[f32]) -> Option<Vec<f32>> {
        if residual.len() != self.width {
            return None;
        }
        let mean_square = residual.iter().map(|x| x * x).sum::<f32>() / self.width as f32;
        let scale = 1.0 / (mean_square + self.epsilon).sqrt();
        Some(
            residual
                .iter()
                .zip(&self.norm)
                .map(|(x, w)| x * scale * w)
                .collect(),
        )
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
