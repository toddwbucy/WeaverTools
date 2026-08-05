//! conforms: harness-loop-mints-no-port
//! conforms: harness-extension-seam-at-loaded-and-idle
//!
//! Loop 1's seat, per `weaver-harness-Spec` section 6. The loop itself is the
//! builder's, written at the worker composition root and compiled into the
//! worker binary. What this crate holds is the seat and the granted surface
//! the loop composes against.
//!
//! **The extension seam is crossed at loaded-and-idle itself**: loop 0 hands a
//! standing interior to whatever loop 1 the binary carries, and takes it back
//! at the stop and at the leave, the bracket discipline being loop 0's for
//! every loop alike.
//!
//! **The blade is structural.** A port is a type this crate owns with a
//! constructor no consumer can reach, so a loop composes against the granted
//! surface or does not compile. There is no call by which a loop mints a port:
//! [`Ports`] has private fields and no public constructor, and a loop that
//! needs a port this surface does not offer is a capability change entering
//! through the front door as a charter and contract edit.

use crate::assembly::Prompt;

/// The granted surface handed to loop 1 at loaded-and-idle. Its fields are
/// private and its only constructor is crate-private, which is the blade: a
/// builder's loop composes what this offers and cannot mint a port of its own.
#[derive(Debug)]
pub struct Ports {
    // The decode surface a real loop needs - the exchanges, sessions,
    // sampling, and the flush call - arrives with the token workflow, per
    // `weaver-spu-PRD` section 8. What stands today is the seat itself.
    pub(crate) assembled: Option<Prompt>,
}

impl Ports {
    /// Crate-private: no consumer can reach it, which is what makes the blade
    /// a compile property rather than a convention. Loop 0 calls it when it
    /// hands a standing interior across the extension seam at
    /// loaded-and-idle, which the decode surface completes with the token
    /// workflow.
    pub(crate) fn grant(assembled: Option<Prompt>) -> Self {
        Ports { assembled }
    }

    /// The one read a loop has today: the prompt loop 0 assembled from the
    /// working structure over the granted surface.
    pub fn assembled(&self) -> Option<&Prompt> {
        self.assembled.as_ref()
    }
}
