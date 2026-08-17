//! conforms: spu-nothing-laid-in-for-later-operations
//!
//! The decode submodule, per `weaver-spu-Spec` section 4.
//!
//! **One submodule exists**, and a later operation type is a sibling of this
//! directory in its own process rather than a variant inside it. Nothing here
//! is laid in for one: adding an operation type is adding a directory beside
//! this one, which is the reversibility test passing on this crate's own
//! future.
//!
//! The family library sits above this submodule and the device beneath it. The
//! seam between is `backend.rs`, and the loop that drives it is `session.rs`.

pub mod backend;
pub mod session;

/// The GGUF backend. Its residency half is real: llama.cpp holds the model
/// and dropping the residency frees the device. Its decode half, the
/// [`backend::Backend`] implementation, is the turn path's act, so
/// `backend::for_container` still refuses both containers.
#[cfg(feature = "gguf")]
pub mod gguf;

/// The candle-native backend, the second peer of the seam. Its residency
/// half loads safetensors onto the admitted device through the pinned candle
/// fork, and its decode half implements the five primitives over a session's
/// clone of the resident model. One family and one device this stage, per
/// the module's own header.
#[cfg(feature = "cuda")]
pub mod native;
