//! conforms: spu-backends-are-peers
//! conforms: spu-backend-from-artifact
//!
//! The backend seam, per `weaver-spu-Spec` section 4.1.
//!
//! **One trait, two backends, peers rather than a legacy and a target.** The
//! archived tree's ruling of 2026-06-11 made the GGUF and native paths
//! first-class peers and the survey carries the reasoning forward: GGUF owns
//! quantized artifacts on consumer devices, and the native path owns what a
//! tensor-parallel forward and a fine-tunable artifact need, since a GGUF
//! cannot be fine-tuned and a program that intends training as a continuation
//! cannot let that path decay.
//!
//! **Which backend serves is a property of the artifact, decided at admit.**
//! The header read of section 3 already answers it, so nothing elects a backend
//! separately and no configuration field names one. [`for_container`] is that
//! derivation, and it is total over [`Container`]: a container this build
//! cannot serve refuses rather than falling back to the other, which would be
//! the silent substitution the family registry also forbids.
//!
//! **Neither implementation is written yet**, so [`for_container`] refuses for
//! both containers today. The derivation's shape is here and the engines behind
//! it are the remaining work of Spec section 4.
//!
//! ## Why the generation loop is not behind this seam
//!
//! The Spec names the seam as one trait over open, append-and-generate, cancel,
//! flush, and close. That is the seam's vocabulary. **This trait carries the
//! primitives and `session.rs` carries the loop,** which is a representation
//! choice this document makes and states rather than assumes.
//!
//! The ground is that two of section 4's properties bind both backends: the
//! turn terminator is made resident on every path including the cancelled one,
//! and the cancel is checked between sampled tokens. Written inside each
//! backend, both properties would hold only as long as each implementer
//! remembered them, and a third backend would arrive owing two obligations no
//! type states. Written once above the seam, they hold for every backend by
//! construction, and the instruments that watch them watch one loop rather than
//! one loop per backend.

use crate::artifact::Container;

/// One token, as the family's tokenizer numbers it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenId(pub u32);

/// What a decode operation refuses on.
#[derive(Debug, Clone, PartialEq)]
pub enum DecodeFault {
    /// The delta would exceed the session's capacity. Carries the session's own
    /// account of itself, because the harness decides what a full context means
    /// and cannot decide it without the numbers.
    Overflow {
        resident: usize,
        requested: usize,
        capacity: usize,
    },
    /// The device or the engine failed underneath.
    Engine { detail: String },
    /// This build was not compiled to serve this artifact's container.
    ContainerNotBuilt { container: Container },
    /// The session has not been opened, so there is no prefix to append to.
    /// A session an engine fault or a failed flush left unusable answers this
    /// too: what it says is that no serviceable prefix stands.
    NotOpen,
    /// Open is first and happens once, per the decode contract's ordering. A
    /// second open would silently rewind the resident length over accumulated
    /// turns, which is the exact failure the append-only discipline forbids.
    AlreadyOpen,
}

/// The primitives a backend supplies. The loop above them is `session.rs`'s.
///
/// The two implementations are peers. Neither is a default and neither is a
/// fallback, which is why this trait names no preferred one and why
/// [`for_container`] refuses rather than substituting.
pub trait Backend {
    /// Decode tokens at an absolute position, extending the resident state.
    ///
    /// **The position is absolute and supplied by the caller** rather than
    /// tracked here, so the session holds the one account of what is resident
    /// and a backend cannot disagree with it.
    fn decode_at(&mut self, tokens: &[TokenId], position: usize) -> Result<(), DecodeFault>;

    /// Sample one token from the current state.
    fn sample(&mut self) -> Result<TokenId, DecodeFault>;

    /// Truncate resident state back to a position.
    ///
    /// **Only a family declaring [`FlushMechanism::TruncateToPosition`] may
    /// have this called on it**, per Spec section 4.4. A family whose state
    /// cannot roll back is re-established instead, because a truncation that
    /// returns success while recurrent state stays is the silent failure the
    /// append-only discipline exists to prevent.
    fn truncate_to(&mut self, position: usize) -> Result<(), DecodeFault>;

    /// Discard all state and return to an empty session.
    ///
    /// The expensive half of the flush, and the correct one where the cheap
    /// path is silently wrong.
    fn reestablish(&mut self) -> Result<(), DecodeFault>;

    /// Release the engine's resources.
    fn close(&mut self);
}

/// How a family's flush reaches its fixed outcome, per Spec section 4.4.
///
/// **The family declares which it is** and the decode path reads the
/// declaration rather than inferring it from a version string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlushMechanism {
    /// The family's state permits truncation to a position, so the outcome is
    /// reached by truncating to the prefix's recorded length.
    TruncateToPosition,
    /// The family cannot roll back, so the outcome is reached by re-establishing
    /// the session and decoding the prefix again. Expensive and correct.
    ReestablishAndReprefill,
}

/// Derive the backend from the artifact's container.
///
/// **Nothing elects a backend separately.** This is the whole derivation, and
/// it is a function of the header read at admit. A container this build was not
/// compiled to serve refuses by name rather than falling back to the peer.
pub fn for_container(container: Container) -> Result<Box<dyn Backend>, DecodeFault> {
    // **Neither backend is written yet**, so this refuses for both containers
    // regardless of which features are on. The refusal names the container
    // rather than reporting a device or artifact problem, so a reader meeting
    // it is told what is missing.
    //
    // This is deliberately not a stub that returns something: a backend that
    // decoded nothing and reported success would make every property above this
    // seam untestable while appearing to pass. The two implementations are the
    // remaining work of Spec section 4, and the `cuda` and `gguf` features buy
    // the kernels and the fork dependencies today, not a serving path.
    Err(DecodeFault::ContainerNotBuilt { container })
}
