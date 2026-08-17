//! conforms: traits-tool-dyn-compatible
//! conforms: traits-tool-boxed-future-send
//!
//! The tool contract, chartered with the tool workflow's opening act of
//! 2026-08-17, per `weaver-traits-Spec` section 5.
//!
//! The block this module carried lifted on its own terms: it held because tool
//! dispatch was harness-internal and no seam crossed it, and the ratified loop
//! boundary of 2026-08-11 inverted that ground - every tool sits outside the
//! reasoning loop, dispatch crosses the gate seam, and
//! `weaver-harness-gate-contract` section 7 draws this definition. The trait
//! is the gate's executor surface: the gate resolves a name against the tools
//! it holds and calls through it, and the harness dispatches the exchange and
//! never the trait, which is the loop boundary held at the type level.
//!
//! **The name and the schema are primitives on purpose.** `tool-name` is
//! `weaver-types` vocabulary and that crate names this one as its one
//! floor-link, so a trait naming that type would close a dependency cycle -
//! and the floor invariant is the reason this crate refuses internal
//! dependencies at all. The gate depends on both crates and is the one party
//! that compares the drawn name against what a tool answers.
//!
//! **No safety classification of any kind**, per `weaver-traits-PRD` section
//! 3.1, which the workflow did not weaken: a trait method asking a tool
//! whether it is dangerous is a heuristic standing where a boundary already
//! stands.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

/// A tool's own account of its failure: content the conversation carries and
/// the model reasons over, never a channel fault, per the layer split the
/// execution exchange states.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolFailure {
    pub detail: String,
}

/// The tool contract, per `weaver-traits-Spec` section 5.
///
/// Dyn-compatible, because the gate dispatches tools it does not know the
/// identity of. The future is explicitly boxed rather than `async fn` in
/// trait, which is not dyn-compatible, and carries `Send` so no executor
/// election leaks onto the floor.
pub trait Tool {
    /// The name the model calls, as the gate compares it against the drawn
    /// `tool-name` that crossed the exchange.
    fn name(&self) -> &str;

    /// The schema this tool advertises to the model, the charter's own
    /// vocabulary item carried by the signature. How an advertisement reaches
    /// the prompt assembly that renders it is the schema act's.
    fn schema(&self) -> &str;

    /// Execute one call. The arguments arrive as the model spoke them,
    /// uninterpreted by any party between the parse and this method.
    fn execute(
        &self,
        arguments: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, ToolFailure>> + Send + '_>>;
}
