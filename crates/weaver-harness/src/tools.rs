//! The tool system, blocked. This module is a placement, per
//! `weaver-harness-Spec` section 6: `tool-trait` is held by
//! `weaver-traits-PRD` section 3.1 until the tool workflow, so nothing is
//! specified here beyond what the charter already fixes.
//!
//! What the charter fixes, and what the tool workflow inherits:
//!
//! - Permission modes are consultation policy and not a boundary.
//! - The kernel bounds what a tool reaches, through the uid it runs as.
//! - No safety classification exists here or is coming - that absence is
//!   pinned at `weaver-traits-Spec` section 5, where the trait lives.
//! - The tool subprocess inherits no descriptor this program holds, which
//!   `channel.rs` delivers by construction through the atomic close-on-exec
//!   at every pair's creation rather than by a per-tool argument.
