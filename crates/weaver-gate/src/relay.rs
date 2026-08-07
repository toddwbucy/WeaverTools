//! conforms: gate-client-content-unread
//!
//! The pass-through, deferred, per `weaver-gate-Spec` section 4.
//!
//! This module is a placement and holds no code, the way the harness Spec
//! places its deferred modules. What it will carry is inbound octets to the
//! harness and outbound octets to the client, order preserved, nothing
//! retained, per charter section 2. Everything it needs is the token
//! workflow's, the turn exchanges toward the harness above all, and nothing is
//! shaped here ahead of that charter.
//!
//! **The relay reads no content**, and that claim binds the suite as much as
//! the build: no test in this crate parses a client line, and a test that did
//! would be the opacity rule breached by the suite rather than by the code. The
//! claim is review's at both ends, because no instrument distinguishes octets
//! forwarded from octets read.
//!
//! Nothing is laid in for it: no trait, no variant, no buffer sized early. That
//! adding the relay is filling this file, rather than unpicking a shape guessed
//! for it, is the reversibility test passing on this crate's own future.
