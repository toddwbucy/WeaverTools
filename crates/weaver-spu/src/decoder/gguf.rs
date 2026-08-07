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
//! // the pin makes this stop compiling, which is the pin firing.
//! let params = LlamaContextParams::default();
//! assert!(!params.has_eval_callback());
//! let params = unsafe { params.with_eval_callback(None, std::ptr::null_mut()) };
//! assert!(!params.has_eval_callback());
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

use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::model::LlamaModel;
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

        let model = LlamaModel::load_from_file(backend, admission.path(), &params).map_err(
            |error| AdmitRefusal::LoadFailed {
                detail: format!("load_from_file: {error}"),
            },
        )?;
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
