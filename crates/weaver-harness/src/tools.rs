//! conforms: harness-tool-result-granted-not-minted
//!
//! The granted tool result, per `weaver-harness-Spec` section 6 and the tool
//! workflow's opening act: the loop closes at the type level here.
//!
//! **The grant is constructed at exactly one site**, the completion of the
//! execution exchange of `weaver-harness-gate-contract` section 2, whichever
//! of the exchange's three contents arrived - the tool's result, the tool's
//! failure, or the gate's refusal of an unheld name - because each is a fact
//! the model must learn, and a refusal that could not author a tool-result
//! turn would leave the conversation with a hole where the answer goes.
//!
//! **Three negatives, each a compile property**, the `PeerIdentity` pattern
//! one seam over: the gate exchange supplies it and no code asserts it.
//!
//! No `Deserialize` (`harness-tool-result-granted-not-minted`): bytes never
//! mint a grant, on any seam, in any test.
//!
//! ```compile_fail
//! fn de<T: for<'de> serde::Deserialize<'de>>() {}
//! de::<weaver_harness::ToolResult>();
//! ```
//!
//! No public construction: the fields are private and the one constructor is
//! `pub(crate)`, so the struct literal below does not compile and no public
//! function returns one from anything a consumer can supply.
//!
//! ```compile_fail
//! let fake = weaver_harness::ToolResult { content: String::new() };
//! ```
//!
//! No conversion from the record: the conversation's `ToolResultBlock` is a
//! serde record that grants nothing, and no `From` bridges it back.
//!
//! ```compile_fail
//! fn from<T: From<weaver_traits::ToolResultBlock>>() {}
//! from::<weaver_harness::ToolResult>();
//! ```

use weaver_traits::ToolResultBlock;
use weaver_types::ToolOutcome;

/// How many execution rounds one turn may run before the harness stops
/// re-generating, the composition root's bound like `HEADROOM_BYTES`: a model
/// that answers every result with another call would otherwise hold the turn
/// open without limit. The bound is a refusal of further calls rather than of
/// the turn: the final generation's emission stands.
pub(crate) const MAX_TOOL_ROUNDS: usize = 4;

/// A tool result the gate exchange granted.
///
/// Obtainable from that exchange's completion alone, per the module note. The
/// interior is deliberately not public: nobody inspects a tool result's
/// fields to know it came through the gate - holding the value is the proof,
/// which is what box granularity means.
pub struct ToolResult {
    content: String,
}

impl ToolResult {
    /// The one construction site's door, crate-private: called where the
    /// execution exchange completes and nowhere else.
    ///
    /// Every outcome becomes renderable content, in words the model reasons
    /// over: a result is itself, a failure names itself as the tool's, and an
    /// unheld name says the tool does not exist.
    pub(crate) fn granted(outcome: &ToolOutcome) -> ToolResult {
        let content = match outcome {
            ToolOutcome::Result { content } => content.clone(),
            ToolOutcome::Failure { detail } => {
                format!("the tool failed: {detail}")
            }
            ToolOutcome::Unheld { name } => {
                format!("no tool named {} exists", name.0)
            }
        };
        ToolResult { content }
    }

    /// The conversation's record of this grant, minted at the author door and
    /// nowhere else: the one direction the two types convert, grant to
    /// record, never back.
    pub(crate) fn block(&self) -> ToolResultBlock {
        ToolResultBlock {
            content: self.content.clone(),
        }
    }
}
