//! conforms: traits-role-set-three
//! conforms: traits-content-is-block-sequence
//! conforms: traits-role-plain-string
//! conforms: traits-content-block-internally-tagged
//! conforms: traits-non-exhaustive-per-charter
//! conforms: traits-serde-from-derives
//!
//! The message model: provider-agnostic conversation content, per
//! `weaver-traits-Spec` section 3. The harness assembles prompts from it and the
//! trace records `message.user`, `message.assistant`, and `message.tool_result`
//! payloads carrying it, opaque to the recorder.

use serde::{Deserialize, Serialize};

/// One conversation message: a role and a sequence of content blocks.
///
/// Content is a sequence rather than a string because an assistant turn is not
/// one: prose and tool calls arrive in one emission and stay distinguishable from
/// authoring through to the trace.
///
/// The licensed role-to-block combinations are stated in `weaver-traits-Spec`
/// section 3 and enforced by the harness, the sole author; the recorder judges
/// the envelope and never the interior, so the rule is not represented here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Vec<ContentBlock>,
}

/// The message's role, mapping one to one onto the trace's three
/// `message.*` event kinds.
///
/// Fieldless, so it serializes as a plain renamed string, `"user"`, per the
/// tagging test the two floor Specs share. The set grows with the conversation
/// model, so the enum is non-exhaustive and consumers keep a wildcard arm.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
    ToolResult,
}

/// One block of a message's content.
///
/// Every variant is struct-shaped or wraps a struct, so the enum is internally
/// tagged and a block reads as `{"type": "text", "text": "..."}`, keyed on a
/// stable member name for the non-Rust consumer on the stream. The set grows
/// with the conversation model, so the enum is non-exhaustive.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text { text: String },
    ToolCall(ToolCall),
    ToolResult(ToolResultBlock),
}

/// The conversation's view of a tool invocation.
///
/// Named here and shaped with the tool workflow, per `weaver-traits-Spec`
/// section 3: the field list follows the tool protocol that section 5 holds
/// blocked, so this declaration is placement and carriage, not an election.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCall {}

/// The conversation's view of a tool result's content.
///
/// Named here and shaped with the tool workflow, per `weaver-traits-Spec`
/// section 3, on the same deferral as [`ToolCall`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolResultBlock {}
