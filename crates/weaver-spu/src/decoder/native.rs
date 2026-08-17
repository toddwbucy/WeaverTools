//! The candle-native backend, per `weaver-spu-Spec` section 4.1.
//!
//! **The second peer, standing.** GGUF owns quantized artifacts on consumer
//! devices, and this path owns what a tensor-parallel forward and a
//! fine-tunable artifact need, since a GGUF cannot be fine-tuned and a program
//! that intends training as a continuation cannot let that path decay. The
//! model runs through the pinned candle fork, which is the readout's working
//! path: `forward_with_intermediates` is only a route to per-layer activations
//! if candle runs the forward, so serving through candle is what keeps the
//! readout election honorable when its act arrives.
//!
//! **Stage one serves one family on one device.** The registry's qwen2 entry
//! is the family, `candle_transformers::models::qwen2` is the forward, and a
//! binding naming more than one device refuses by name: the salvaged
//! two-device path is its own act, entering with the all-reduce it needs, and
//! a width this file cannot serve must refuse rather than serve it wrong.
//!
//! **The resident model is pristine and the engine decodes against a clone.**
//! Candle's model holds its KV cache inside the model value, so a session's
//! state would otherwise live in the residency and survive the session. The
//! clone is cheap, the weight tensors sharing storage underneath, and it is
//! what makes `close` true: dropping the engine drops the session's state and
//! nothing else.
//!
//! **Truncation is reached by re-decoding the retained prefix.** The family
//! declares `TruncateToPosition` and the fork's cache exposes clear and
//! nothing finer, so this engine retains what it decoded and reaches the
//! truncated state by clearing and re-decoding the front of it. The outcome
//! is a true truncation, the resident state after holding exactly the first
//! `position` tokens, and the cost is a prefill the GGUF engine does not pay.
//! What Spec section 4.4 forbids is a truncation that returns success while
//! recurrent state stays, and none stays here. A cache that can narrow is
//! fork work for a later act, and this paragraph is what it would buy.

use std::path::Path;

use candle_core::{DType, Device, Tensor};
use candle_transformers::generation::{LogitsProcessor, Sampling};
use candle_transformers::models::qwen2::{Config, ModelForCausalLM};

use super::backend::{Backend, DecodeFault, TokenId};
use crate::residency::{Admission, AdmitRefusal};
use crate::sampling::EffectiveKnobs;

/// The weights resident on the device, held by the residency.
///
/// Holding it is the residency: the tensors live on the admitted device and
/// dropping this frees them, so the release ordering is by construction, the
/// same property the GGUF peer states.
pub struct ResidentModel {
    model: ModelForCausalLM,
    tokenizer: tokenizers::Tokenizer,
    /// The admitted device's handle, held once at load: the tensors know
    /// where they live, but candle carries no model-level accessor, and
    /// reconstructing the handle per session would be a second account of a
    /// fact this struct already witnessed.
    device: Device,
}

impl ResidentModel {
    /// Load the admission's artifact onto the admission's device.
    ///
    /// The artifact is the directory the resolution step found the container
    /// in: the weights are the container's, the shapes are `config.json`'s,
    /// and the vocabulary is `tokenizer.json`'s, all read from beside the
    /// container because a safetensors export is a directory-shaped artifact
    /// and its parts do not travel inside one file the way a GGUF's do.
    pub fn load(admission: &Admission<'_>) -> Result<ResidentModel, AdmitRefusal> {
        let devices = admission.devices();
        // **Stage one is a one-device path and says so.** A pair refuses by
        // name rather than serving a width this file cannot shard: the
        // salvaged two-device forward is its own act.
        let [ordinal] = devices else {
            return Err(AdmitRefusal::LoadFailed {
                detail: format!(
                    "the native path serves one device and the binding names {}",
                    devices.len()
                ),
            });
        };
        let device =
            Device::new_cuda(ordinal.0 as usize).map_err(|error| AdmitRefusal::LoadFailed {
                detail: format!("cuda device {}: {error}", ordinal.0),
            })?;

        let dir = sidecar_dir(admission.path())?;
        let config = read_config(&dir.join("config.json"))?;
        let tokenizer =
            tokenizers::Tokenizer::from_file(dir.join("tokenizer.json")).map_err(|error| {
                AdmitRefusal::LoadFailed {
                    detail: format!("tokenizer.json: {error}"),
                }
            })?;

        // BF16 on the device, which is what the artifact holds and what the
        // fork's kernels serve. The mmap is unsafe by the crate's own
        // signature: the file must not change underneath the map, which is
        // the standing assumption every reader of a pinned artifact makes.
        let vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(
                &[admission.path().to_path_buf()],
                DType::BF16,
                &device,
            )
        }
        .map_err(|error| AdmitRefusal::LoadFailed {
            detail: format!("safetensors map: {error}"),
        })?;
        let model =
            ModelForCausalLM::new(&config, vb).map_err(|error| AdmitRefusal::LoadFailed {
                detail: format!("model construction: {error}"),
            })?;

        Ok(ResidentModel {
            model,
            tokenizer,
            device,
        })
    }

    /// Tokenize against the artifact's own vocabulary.
    pub(crate) fn tokenize(&self, text: &str) -> Result<Vec<TokenId>, DecodeFault> {
        let encoding = self
            .tokenizer
            .encode(text, false)
            .map_err(|error| DecodeFault::Engine {
                detail: format!("encode: {error}"),
            })?;
        Ok(encoding.get_ids().iter().map(|&id| TokenId(id)).collect())
    }

    /// Render token ids back to text.
    pub(crate) fn detokenize(&self, tokens: &[TokenId]) -> Result<String, DecodeFault> {
        let ids: Vec<u32> = tokens.iter().map(|token| token.0).collect();
        self.tokenizer
            .decode(&ids, false)
            .map_err(|error| DecodeFault::Engine {
                detail: format!("decode: {error}"),
            })
    }
}

/// The directory the artifact's sidecar files live in.
///
/// The admission's path is the pin's `/proc/self/fd/N` on purpose, so the
/// real location is recovered by asking the kernel what it currently calls
/// the pinned inode, the same recovery the header read makes and for the
/// same stated limit: a sidecar is an open by name and cannot ride the
/// descriptor.
fn sidecar_dir(path: &Path) -> Result<std::path::PathBuf, AdmitRefusal> {
    let real = std::fs::read_link(path).unwrap_or_else(|_| path.to_path_buf());
    real.parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| AdmitRefusal::LoadFailed {
            detail: format!("{} has no parent directory", real.display()),
        })
}

/// Read the model's `config.json` into the fork's own shape.
///
/// Monomorphic on purpose: the crate carries `serde_json` and not `serde`
/// itself, per the Spec's dependency list, so a generic bound would need a
/// dependency this signature does not justify. The one consumer is the
/// config.
fn read_config(path: &Path) -> Result<Config, AdmitRefusal> {
    let text = std::fs::read_to_string(path).map_err(|error| AdmitRefusal::LoadFailed {
        detail: format!("{}: {error}", path.display()),
    })?;
    serde_json::from_str(&text).map_err(|error| AdmitRefusal::LoadFailed {
        detail: format!("{}: {error}", path.display()),
    })
}

/// The native engine: the five primitives over a session's clone of the
/// resident model.
pub struct NativeEngine {
    model: ModelForCausalLM,
    device: Device,
    /// Every token decoded in order, the one account this engine holds of its
    /// own state, retained because truncation re-decodes the front of it.
    resident: Vec<TokenId>,
    /// The last decode's logits, the only position `distribution` and
    /// `sample` may read. `None` before the first decode, so a sample before
    /// any decode refuses rather than reading a buffer nothing filled.
    logits: Option<Vec<f32>>,
    sampler: LogitsProcessor,
    capacity: usize,
    closed: bool,
}

impl NativeEngine {
    /// Open a session over the residency.
    pub fn open(
        model: &ResidentModel,
        knobs: &EffectiveKnobs,
        capacity: u32,
    ) -> Result<NativeEngine, DecodeFault> {
        if capacity == 0 {
            return Err(DecodeFault::Engine {
                detail: "a session capacity of zero serves nothing".into(),
            });
        }
        let device = model.device.clone();
        // The sampling chain mirrors the effective knobs the way the GGUF
        // engine's sampler does: temperature zero is argmax, and the top-k
        // and top-p gates compose where both are live.
        let sampling = if knobs.temperature <= 0.0 {
            Sampling::ArgMax
        } else {
            Sampling::TopKThenTopP {
                k: knobs.top_k as usize,
                p: knobs.top_p as f64,
                temperature: knobs.temperature as f64,
            }
        };
        Ok(NativeEngine {
            model: model.model.clone(),
            device,
            resident: Vec::new(),
            logits: None,
            sampler: LogitsProcessor::from_sampling(knobs.seed, sampling),
            capacity: capacity as usize,
            closed: false,
        })
    }

    fn engine_fault(detail: &str) -> DecodeFault {
        DecodeFault::Engine {
            detail: detail.into(),
        }
    }

    /// One forward over `tokens` at the engine's own resident length.
    fn forward(&mut self, tokens: &[TokenId]) -> Result<Vec<f32>, DecodeFault> {
        let ids: Vec<u32> = tokens.iter().map(|token| token.0).collect();
        let input = Tensor::new(ids.as_slice(), &self.device)
            .and_then(|t| t.unsqueeze(0))
            .map_err(|error| Self::engine_fault(&format!("input tensor: {error}")))?;
        let logits = self
            .model
            .forward(&input, self.resident.len())
            .and_then(|t| t.squeeze(0))
            .and_then(|t| t.squeeze(0))
            .and_then(|t| t.to_dtype(DType::F32))
            .and_then(|t| t.to_vec1::<f32>())
            .map_err(|error| Self::engine_fault(&format!("forward: {error}")))?;
        Ok(logits)
    }
}

impl Backend for NativeEngine {
    fn decode_at(&mut self, tokens: &[TokenId], position: usize) -> Result<(), DecodeFault> {
        if self.closed {
            return Err(Self::engine_fault("the engine is closed"));
        }
        // **The caller's absolute position must agree with the engine's own
        // account**, or one of the two has lost the session. The session
        // holds the one account of what is resident, and this check is what
        // keeps a backend from silently disagreeing with it.
        if position != self.resident.len() {
            return Err(Self::engine_fault(&format!(
                "position {position} against a resident length of {}",
                self.resident.len()
            )));
        }
        // **An empty decode is a no-op, not a kernel launch.** The session
        // sends the delta as given, and a turn whose delta is empty samples
        // from the distribution the prefix's own decode left standing. A
        // zero-length tensor reaching the embedding kernel is an invalid
        // argument at the driver, measured on this workshop, so the empty
        // case returns before the device is asked.
        if tokens.is_empty() {
            return Ok(());
        }
        if self.resident.len() + tokens.len() > self.capacity {
            return Err(DecodeFault::Overflow {
                resident: self.resident.len(),
                requested: tokens.len(),
                capacity: self.capacity,
            });
        }
        let logits = self.forward(tokens)?;
        self.resident.extend_from_slice(tokens);
        self.logits = Some(logits);
        Ok(())
    }

    fn distribution(&self) -> Result<&[f32], DecodeFault> {
        self.logits
            .as_deref()
            .ok_or_else(|| Self::engine_fault("no decode has produced a distribution"))
    }

    fn sample(&mut self) -> Result<TokenId, DecodeFault> {
        let logits = self
            .logits
            .as_deref()
            .ok_or_else(|| Self::engine_fault("no decode has produced a distribution"))?;
        let tensor = Tensor::new(logits, &Device::Cpu)
            .map_err(|error| Self::engine_fault(&format!("logits tensor: {error}")))?;
        let token = self
            .sampler
            .sample(&tensor)
            .map_err(|error| Self::engine_fault(&format!("sample: {error}")))?;
        Ok(TokenId(token))
    }

    fn truncate_to(&mut self, position: usize) -> Result<(), DecodeFault> {
        if position > self.resident.len() {
            return Err(Self::engine_fault(&format!(
                "truncate to {position} beyond a resident length of {}",
                self.resident.len()
            )));
        }
        // Clear, then re-decode the retained front: the state after holds
        // exactly the first `position` tokens, a true truncation reached the
        // expensive way, per this module's header.
        self.model.clear_kv_cache();
        let front: Vec<TokenId> = self.resident[..position].to_vec();
        self.resident.clear();
        self.logits = None;
        if !front.is_empty() {
            let logits = self.forward(&front)?;
            self.resident = front;
            self.logits = Some(logits);
        }
        Ok(())
    }

    fn reestablish(&mut self) -> Result<(), DecodeFault> {
        self.model.clear_kv_cache();
        self.resident.clear();
        self.logits = None;
        Ok(())
    }

    fn close(&mut self) {
        self.model.clear_kv_cache();
        self.resident.clear();
        self.logits = None;
        self.closed = true;
    }
}
