//! conforms: harness-fault-below-the-exchange-layer
//! conforms: harness-outcome-two-cases
//! conforms: harness-failed-set-refuses-construction
//!
//! The failure vocabulary, per `weaver-harness-Spec` section 7. A refusal is a
//! typed answer on an exchange and is always the floor's `LifecycleRefusal`,
//! drawn and never twinned. A channel fault is a failure below the exchange
//! layer, which is why it is a different type rather than a wider refusal set.

/// A failure below the exchange layer: no exchange can carry it, so no refusal
/// answers it and the service ends.
///
/// `Truncated` is a read the kernel shortened, `Undecodable` is octets that do
/// not decode to an envelope and so cannot be attributed to any exchange for a
/// refusal to answer, and `Closed` is the far process gone, observed as death
/// and never synthesized into an answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelFault {
    Truncated { bound: usize },
    Undecodable,
    Closed,
}

/// The two ways service ends, so the composition root branches on a value
/// rather than a guess. Exhaustive: a third case reaches every caller loudly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Left,
    ChannelClosed,
}

/// Why adoption refused. Adoption fails only when a set fails: a hygiene call
/// that errors leaves the worker attachable or the end inheritable, so `adopt`
/// returns the fault naming the set rather than proceeding unset, and a
/// `Harness` in hand means the hygiene held.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdoptionFault {
    /// The close-on-exec set on the adopted coordination end failed.
    CloseOnExecUnset { errno: i32 },
    /// The dumpable clear failed, leaving the worker attachable.
    DumpableNotCleared { errno: i32 },
}

impl std::fmt::Display for ChannelFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelFault::Truncated { bound } => {
                write!(f, "read truncated against the {bound} byte envelope bound")
            }
            ChannelFault::Undecodable => write!(f, "octets do not decode to an envelope"),
            ChannelFault::Closed => write!(f, "channel closed"),
        }
    }
}

impl std::error::Error for ChannelFault {}

impl std::fmt::Display for AdoptionFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdoptionFault::CloseOnExecUnset { errno } => {
                write!(
                    f,
                    "close-on-exec could not be set on the adopted end ({errno})"
                )
            }
            AdoptionFault::DumpableNotCleared { errno } => {
                write!(f, "the dumpable flag could not be cleared ({errno})")
            }
        }
    }
}

impl std::error::Error for AdoptionFault {}

/// Why an authored message was refused before submit: the licensing rule of
/// `weaver-traits-Spec` section 3, enforced here because the recorder cannot
/// hold it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnlicensedMessage {
    pub role: &'static str,
    pub block: &'static str,
}

impl std::fmt::Display for UnlicensedMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "a {} message may not carry a {} block",
            self.role, self.block
        )
    }
}

impl std::error::Error for UnlicensedMessage {}
