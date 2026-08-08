//! conforms: spu-eval-callback-pinned-by-doctest
//!
//! The GGUF backend's residency half, per `weaver-spu-Spec` sections 3 and 4.1:
//! llama.cpp holds the model, the weights go to the binding's devices, and
//! dropping [`ResidentModel`] is what frees them. The decode half, the
//! [`crate::decoder::backend::Backend`] implementation over a llama.cpp
//! context, is the turn path's act and is not here.
//!
//! **The fork seam is a compile-time pin rather than a comment.** The fork
//! exists for one reason, the ggml scheduler's eval callback, which is the only
//! route to per-layer activations from a GGUF model without replacing the
//! engine, and the survey names this pin as the one thing standing between the
//! fork and a quiet loss. The doctest below calls the setter: it compiles
//! against the fork and fails to compile against the upstream crate, so
//! reverting the pin breaks the build rather than silently removing the
//! readout capability.
//!
//! ```
//! use llama_cpp_2::context::params::LlamaContextParams;
//! // The eval-callback seam: present in the fork, absent upstream. Reverting
//! // the pin makes this stop compiling, which is the pin firing. The callback
//! // is a real one and the assertion reads the state change, so the pin holds
//! // the seam open rather than only naming it.
//! unsafe extern "C" fn tap(
//!     _tensor: *mut llama_cpp_sys_2::ggml_tensor,
//!     _ask: bool,
//!     _user_data: *mut std::ffi::c_void,
//! ) -> bool {
//!     true
//! }
//! let params = LlamaContextParams::default();
//! assert!(!params.has_eval_callback());
//! let params = unsafe { params.with_eval_callback(Some(tap), std::ptr::null_mut()) };
//! assert!(params.has_eval_callback());
//! ```
//!
//! **The two loader shapes the Spec names cannot reach this module.** The one
//! entry is [`ResidentModel::load`], which takes the admission's proof, so a
//! bare `&str` or a `PathBuf` outside the admission path fails to compile:
//!
//! ```compile_fail
//! fn pin(path: &str) -> weaver_spu::decoder::gguf::ResidentModel {
//!     weaver_spu::decoder::gguf::ResidentModel::load(path).unwrap()
//! }
//! ```

use std::sync::OnceLock;

use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::sampling::LlamaSampler;
use llama_cpp_2::token::LlamaToken;

use crate::decoder::backend::{Backend, DecodeFault, TokenId};
use crate::sampling::EffectiveKnobs;
use llama_cpp_2::model::params::{LlamaModelParams, LlamaSplitMode};

use crate::residency::{Admission, AdmitRefusal};

/// The one llama.cpp backend this process holds.
///
/// `LlamaBackend::init` sets up global engine state and may be called once per
/// process, so it lives behind a `OnceLock`, the same one-owner discipline the
/// archived tree's own note records against itself. A failure is held as the
/// failure, so a second admit attempt reports the same fact instead of
/// re-initializing global state that half-exists.
static BACKEND: OnceLock<Result<LlamaBackend, String>> = OnceLock::new();

fn backend() -> Result<&'static LlamaBackend, AdmitRefusal> {
    let state = BACKEND.get_or_init(|| {
        LlamaBackend::init().map_err(|error| format!("llama.cpp backend init: {error}"))
    });
    match state {
        Ok(backend) => Ok(backend),
        Err(detail) => Err(AdmitRefusal::LoadFailed {
            detail: detail.clone(),
        }),
    }
}

/// A GGUF model resident on the binding's devices.
///
/// Holding this is the residency. The drop frees the device: llama.cpp returns
/// the weights' memory when the model is dropped, which is what lets the
/// release's ordering hold by construction.
pub struct ResidentModel {
    model: LlamaModel,
}

impl ResidentModel {
    /// Load the admission's artifact onto the admission's devices.
    ///
    /// The devices are the binding's and this crate selects none: one device
    /// pins `main_gpu` with no split, and a pair splits by layer across
    /// exactly the named ordinals, never whatever else the driver can see.
    /// The parameter shapes carry the archived tree's own working
    /// configuration forward.
    ///
    /// **An assumption crosses here and is named rather than silent.** The
    /// admission judges room and reach against CUDA ordinals, while
    /// `main_gpu` and `with_devices` index ggml's registered-device
    /// enumeration. The two numberings coincide while the libllama this
    /// build links registers CUDA devices only and in ordinal order, which
    /// holds for the library the pinned build script compiles. A deployment
    /// linking a libllama with another backend registered, RPC or Vulkan or
    /// SYCL, breaks the coincidence and puts weights on a card the admission
    /// never judged. The mapping through the ggml device registry belongs to
    /// the decode act, which touches the engine's device surface anyway, and
    /// until then this paragraph is the boundary of what is verified.
    pub fn load(admission: &Admission<'_>) -> Result<ResidentModel, AdmitRefusal> {
        let backend = backend()?;
        let devices = admission.devices();

        // Every layer goes to the device: a partial offload would leave part
        // of the model in host memory, a residency the admission never judged.
        let mut params = LlamaModelParams::default()
            .with_n_gpu_layers(u32::MAX)
            .with_main_gpu(devices[0].0 as i32)
            .with_split_mode(if devices.len() == 1 {
                LlamaSplitMode::None
            } else {
                LlamaSplitMode::Layer
            });
        if devices.len() > 1 {
            let ordinals: Vec<usize> = devices.iter().map(|d| d.0 as usize).collect();
            params = params
                .with_devices(&ordinals)
                .map_err(|error| AdmitRefusal::LoadFailed {
                    detail: format!("device set {ordinals:?}: {error}"),
                })?;
        }

        let model =
            LlamaModel::load_from_file(backend, admission.path(), &params).map_err(|error| {
                AdmitRefusal::LoadFailed {
                    detail: format!("load_from_file: {error}"),
                }
            })?;
        Ok(ResidentModel { model })
    }

    /// The engine's handle, for the decode acts that follow. Crate-visible so
    /// nothing outside reaches the engine around the seam. Nothing calls it
    /// yet, and the allow says so instead of widening the visibility to quiet
    /// the lint: the decode half is this module's next act, and the field it
    /// reads is what holding the residency means.
    #[allow(dead_code)]
    pub(crate) fn model(&self) -> &LlamaModel {
        &self.model
    }
}

/// The GGUF engine, per `weaver-spu-Spec` section 4.
///
/// **It borrows the residency it decodes against.** `LlamaContext` borrows the
/// model, and that is the relationship in life as well as in types: a session
/// cannot outlive the residency it was opened over, and the borrow is what says
/// so without a rule anyone has to remember.
///
/// **The loop above this is `session.rs`'s and stays there.** What lives here is
/// the five primitives the seam declares and nothing else, so the terminator
/// discipline and the cancel bound hold for this engine by construction rather
/// than by this file remembering them.
pub struct GgufEngine<'a> {
    context: LlamaContext<'a>,
    sampler: LlamaSampler,
    /// The batch index whose logits the last decode left standing, which is the
    /// only position `distribution` and `sample` may read. `None` before the
    /// first decode, so a sample before any decode refuses rather than reading
    /// a buffer nothing filled.
    logits_at: Option<i32>,
    closed: bool,
}

impl<'a> GgufEngine<'a> {
    /// Open a context over a resident model and build its sampler from the
    /// binary's resolved knobs.
    ///
    /// **The sampler is built once, from the effective values.** The knobs
    /// resolved at the composition root per Spec section 8, so nothing here
    /// re-reads a disposition or learns which side supplied a value.
    pub fn open(
        model: &'a ResidentModel,
        knobs: &EffectiveKnobs,
        capacity: u32,
    ) -> Result<GgufEngine<'a>, DecodeFault> {
        let backend = backend().map_err(|_| DecodeFault::Engine {
            detail: "the llama backend is not initialised".into(),
        })?;
        let params = LlamaContextParams::default()
            .with_n_ctx(std::num::NonZeroU32::new(capacity))
            .with_n_batch(capacity);
        let context = model
            .model()
            .new_context(backend, params)
            .map_err(|error| DecodeFault::Engine {
                detail: format!("new_context: {error}"),
            })?;

        // The chain order is the one llama.cpp's own samplers assume:
        // penalties, then the truncating filters, then temperature, then the
        // draw. A distribution sampler last is what makes the seed mean
        // something.
        let sampler = LlamaSampler::chain_simple([
            LlamaSampler::penalties(
                knobs.repetition_window as i32,
                knobs.repetition_penalty,
                0.0,
                0.0,
            ),
            LlamaSampler::top_k(knobs.top_k as i32),
            LlamaSampler::top_p(knobs.top_p, 1),
            LlamaSampler::temp(knobs.temperature),
            LlamaSampler::dist(knobs.seed as u32),
        ]);

        Ok(GgufEngine {
            context,
            sampler,
            logits_at: None,
            closed: false,
        })
    }

    fn engine_fault(detail: impl Into<String>) -> DecodeFault {
        DecodeFault::Engine {
            detail: detail.into(),
        }
    }
}

impl Backend for GgufEngine<'_> {
    fn decode_at(&mut self, tokens: &[TokenId], position: usize) -> Result<(), DecodeFault> {
        if self.closed {
            return Err(Self::engine_fault("the engine is closed"));
        }
        if tokens.is_empty() {
            return Ok(());
        }
        let mut batch = LlamaBatch::new(tokens.len(), 1);
        let last = tokens.len() - 1;
        for (offset, token) in tokens.iter().enumerate() {
            let pos = i32::try_from(position + offset)
                .map_err(|_| Self::engine_fault("position exceeds the engine's range"))?;
            // **Logits are asked for on the last token only.** Every earlier
            // token is context being made resident, and asking for its
            // distribution would allocate a vocabulary-wide buffer per token
            // for a reading nothing takes.
            batch
                .add(LlamaToken(token.0 as i32), pos, &[0], offset == last)
                .map_err(|error| Self::engine_fault(format!("batch add: {error}")))?;
        }
        self.context
            .decode(&mut batch)
            .map_err(|error| Self::engine_fault(format!("decode: {error}")))?;
        self.logits_at = Some(last as i32);
        Ok(())
    }

    fn distribution(&self) -> Result<&[f32], DecodeFault> {
        if self.closed {
            return Err(Self::engine_fault("the engine is closed"));
        }
        // **Nothing to read before a decode has left logits standing.** A
        // zeroed buffer would measure as a uniform distribution over the
        // vocabulary, which is a reading rather than an absence.
        let at = self
            .logits_at
            .ok_or_else(|| Self::engine_fault("no decode has left a distribution to read"))?;
        Ok(self.context.get_logits_ith(at))
    }

    fn sample(&mut self) -> Result<TokenId, DecodeFault> {
        if self.closed {
            return Err(Self::engine_fault("the engine is closed"));
        }
        let at = self
            .logits_at
            .ok_or_else(|| Self::engine_fault("no decode has left a distribution to sample"))?;
        let token = self.sampler.sample(&self.context, at);
        // The penalty window is a function of what was drawn, so the sampler is
        // told what it drew.
        self.sampler.accept(token);
        u32::try_from(token.0)
            .map(TokenId)
            .map_err(|_| Self::engine_fault("the engine answered a negative token"))
    }

    fn truncate_to(&mut self, position: usize) -> Result<(), DecodeFault> {
        if self.closed {
            return Err(Self::engine_fault("the engine is closed"));
        }
        let from = i32::try_from(position)
            .map_err(|_| Self::engine_fault("position exceeds the engine's range"))?;
        // Everything at or after the position leaves the cache. The open end is
        // what makes this a truncation rather than a hole.
        self.context
            .clear_kv_cache_seq(Some(0), Some(from as u32), None)
            .map_err(|error| Self::engine_fault(format!("clear_kv_cache_seq: {error}")))?;
        // **The standing distribution belongs to a position that is gone.**
        // Leaving it readable would let a sample after a truncation draw from
        // the state the truncation removed.
        self.logits_at = None;
        Ok(())
    }

    fn reestablish(&mut self) -> Result<(), DecodeFault> {
        if self.closed {
            return Err(Self::engine_fault("the engine is closed"));
        }
        self.context.clear_kv_cache();
        self.logits_at = None;
        Ok(())
    }

    fn close(&mut self) {
        // The context and the sampler release with this value. What close marks
        // is that nothing further may be asked, which every method above reads.
        self.closed = true;
        self.logits_at = None;
    }
}
