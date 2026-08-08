//! conforms: traits-tool-dyn-compatible
//! conforms: traits-tool-boxed-future-send
//!
//! The tool contract, blocked. This module exists as a placement, per
//! `weaver-traits-Spec` section 5: `tool-trait` is blocked by the charter's own
//! section 3.1, tool dispatch being harness-internal with no seam crossing it, so
//! no vocabulary clause draws the definition and specifying it here would be the
//! anticipatory contract the charter forbids, written in Rust rather than prose.
//!
//! Four constraints the tool workflow inherits when it charters the trait:
//!
//! 1. The trait is dyn-compatible - the engine dispatches tools it does not know
//!    the identity of, which is object dispatch by definition.
//! 2. Async methods return an explicitly boxed future rather than using
//!    `async fn` in trait, which is not dyn-compatible, and without `async-trait`,
//!    a proc-macro dependency the boxing writes out by hand.
//! 3. The boxed future carries `+ Send` - the harness drives tool calls on a
//!    runtime this crate does not name, and a future that cannot cross a thread
//!    would bound the executor the composition root may choose.
//! 4. No safety classification of any kind, per the charter's section 3.1 and
//!    apex section 3 step 7, which the tool workflow may not weaken. Step 7 was
//!    rewritten by the egress ruling of 2026-08-07 and the ground survived the
//!    rewrite: both readings put the judgment somewhere other than a method the
//!    tool answers, which is the whole of what this negative needs.
//!
//! The trait serves the elected-and-outward corner of the tool taxonomy alone,
//! and the port ruling of 2026-08-07 narrows what occupies that corner: a tool
//! that binds a listening port is external and reaches the agent through the
//! gate, and one that binds none is internal whatever its bytes reach. Bash is
//! internal by that test. The inward corners are function loops the harness runs
//! beside its control loops and reach no trait here.
