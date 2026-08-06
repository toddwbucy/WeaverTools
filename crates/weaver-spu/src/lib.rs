//! conforms: spu-one-binary
//! conforms: spu-nothing-laid-in-for-later-operations
//! conforms: spu-two-floor-links-types-without-config
//! conforms: spu-fork-pins-resolve-as-pinned
//! conforms: spu-cudarc-caret-pinned
//! conforms: spu-no-runtime-no-logging-no-http
//! conforms: spu-two-feature-gates
//!
//! The organ that holds the model, per `weaver-spu-Spec`.
//!
//! One binary, forked and exec'd by the harness during the enter fan-out. This
//! lib target is not an API for a consumer: nothing links this crate, the seam
//! being a socket rather than a link. It exists so that the Spec's doctest
//! instruments execute, which they cannot do under a bin-only crate.
//!
//! **One submodule exists.** A later operation type is a sibling of `decoder/`,
//! in its own process, and nothing is laid in for it: no trait, no variant, no
//! feature, no socket bound early. That adding one is adding a directory beside
//! `decoder/` is the reversibility test passing on this crate's own future.

pub mod artifact;
pub mod channel;
pub mod family;
pub mod residency;
