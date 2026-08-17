//! conforms: gate-one-binary
//! conforms: gate-floor-link-types-without-config
//! conforms: gate-no-runtime-no-logging-no-yaml
//!
//! The loop's membrane, per `weaver-gate-Spec` and the ratified
//! reasoning-loop boundary of 2026-08-11: the agent's membrane is the network
//! boundary, and this crate is the loop's.
//!
//! One binary, forked and exec'd by the harness during the enter fan-out, and
//! nothing links it. This lib target is not an API for a consumer: it exists so
//! the Spec's bind-shape pins execute, which they cannot do under a bin-only
//! crate.
//!
//! One module per obligation: `channel.rs` holds the seam end and the
//! exchange service, `hook.rs` the instruction's resolution and the bind and
//! the predicate, `relay.rs` the pass-through, filled with the wiring act
//! per `weaver-gate-Spec` section 4.

pub mod channel;
pub mod hook;
pub mod relay;
