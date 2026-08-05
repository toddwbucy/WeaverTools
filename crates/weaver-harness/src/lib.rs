//! conforms: harness-one-constructor
//! conforms: harness-path-shapes-pinned-by-doctest
//! conforms: harness-outcome-two-cases
//! conforms: harness-descriptors-owned-types
//! conforms: harness-loop-mints-no-port
//! conforms: harness-no-runtime-no-logging-no-http
//!
//! The hub: loop 0, the channels and their custody, trace authorship, prompt
//! assembly's deterministic floor, and loop 1's seat.
//!
//! The doctests below are the compile-time instruments of
//! `weaver-harness-Spec` section 8.
//!
//! Descriptors are owned types end to end
//! (`harness-descriptors-owned-types`): a handle that escapes its owner is a
//! move the borrow checker sees, not an integer copied silently. `adopt`
//! consumes the `OwnedFd` it is handed, so this compiles:
//!
//! ```
//! use std::os::fd::OwnedFd;
//! use weaver_harness::{AdoptionFault, Harness, OrganBinaries};
//! fn shape(end: OwnedFd, organs: OrganBinaries) {
//!     let _adopted: Result<Harness, AdoptionFault> = Harness::adopt(end, organs);
//! }
//! ```
//!
//! ...and reusing the moved descriptor does not:
//!
//! ```compile_fail
//! use std::os::fd::{AsRawFd, OwnedFd};
//! use weaver_harness::{Harness, OrganBinaries};
//! fn f(end: OwnedFd, organs: OrganBinaries) {
//!     let _ = Harness::adopt(end, organs);
//!     let _ = end.as_raw_fd();
//! }
//! ```
//!
//! One constructor (`harness-one-constructor`): the fields are private and no
//! second path exists, so a struct literal fails to compile.
//!
//! ```compile_fail
//! let _ = weaver_harness::Harness { coordination: todo!(), organs: todo!() };
//! ```
//!
//! No path-taking surface (`harness-path-shapes-pinned-by-doctest`): the three
//! named shapes each fail to compile where the channel takes owned
//! descriptors. The general prohibition stays review's - these three reach the
//! shapes they name and not the open set of every way a path becomes a call
//! argument.
//!
//! ```compile_fail
//! use weaver_harness::{Harness, OrganBinaries};
//! let _ = Harness::adopt("/run/weaver/alpha/coordination.sock", OrganBinaries {
//!     spu: "/opt/weaver/bin/spu".into(), gate: "/opt/weaver/bin/gate".into(),
//! });
//! ```
//!
//! ```compile_fail
//! use weaver_harness::{Harness, OrganBinaries};
//! let path = String::from("/run/weaver/alpha/coordination.sock");
//! let _ = Harness::adopt(path, OrganBinaries {
//!     spu: "/opt/weaver/bin/spu".into(), gate: "/opt/weaver/bin/gate".into(),
//! });
//! ```
//!
//! ```compile_fail
//! use std::path::PathBuf;
//! use weaver_harness::{Harness, OrganBinaries};
//! let _ = Harness::adopt(PathBuf::from("/run/weaver/alpha/coordination.sock"),
//!     OrganBinaries { spu: "/opt/weaver/bin/spu".into(), gate: "/opt/weaver/bin/gate".into() });
//! ```
//!
//! The outcome enum is exhaustive (`harness-outcome-two-cases`), so a third
//! case reaches every caller loudly:
//!
//! ```
//! use weaver_harness::Outcome;
//! fn name(outcome: Outcome) -> &'static str {
//!     match outcome {
//!         Outcome::Left => "left",
//!         Outcome::ChannelClosed => "channel_closed",
//!     }
//! }
//! ```
//!
//! A loop mints no port (`harness-loop-mints-no-port`): `Ports` has private
//! fields and no reachable constructor, so a builder's loop composes the
//! granted surface or does not compile.
//!
//! ```compile_fail
//! let _ = weaver_harness::Ports::grant(None);
//! ```
//!
//! The channel state's three positions and the floor's exhaustive wire enums
//! together make every out-of-order directive reach a match arm rather than a
//! flag check, which `serve` holds structurally.

mod assembly;
mod authorship;
mod channel;
mod engine;
mod failure;
mod lifecycle;
mod spawn;
mod tools;

pub use assembly::{Prompt, assemble};
pub use authorship::{Author, licensed};
pub use channel::{
    ChildEnd, DecodeChannel, FIRST_ORGAN_DESCRIPTOR, OrganChannel, place_child_ends,
};
pub use engine::Ports;
pub use failure::{AdoptionFault, ChannelFault, Outcome, UnlicensedMessage};
pub use lifecycle::{Harness, OrganBinaries};
pub use spawn::fork_organ;
