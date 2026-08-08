//! The llama family, per `weaver-spu-Spec` section 5.
//!
//! Everything llama defines sits here: its header markers, the rendering that
//! uses them, the parse of its own emissions, and its stop conditions. The
//! scanning beneath is [`super::scan`], shared and marker-free.

use weaver_types::ToolName;

use super::{Declaration, Family, FamilyName, Markers, Message, Parsed, lookup};

/// The header markers. Written here and nowhere else in the crate.
pub const TEXT_BEGIN: &str = "<|begin_of_text|>";
pub const HEADER_OPEN: &str = "<|start_header_id|>";
pub const HEADER_CLOSE: &str = "<|end_header_id|>";
pub const TURN_END: &str = "<|eot_id|>";

/// What opens a call in this family's emissions, and what ends one.
pub const CALL_OPEN: &str = "<|python_tag|>";
pub const CALL_CLOSE: &str = "<|eom_id|>";

/// Every control marker this family declares.
pub const CONTROL_MARKERS: &[&str] = &[
    TEXT_BEGIN,
    HEADER_OPEN,
    HEADER_CLOSE,
    TURN_END,
    CALL_OPEN,
    CALL_CLOSE,
];

/// This family's template, the one authority for it. The registry cites this
/// constant and [`Family::render_delta`] renders through it.
pub const TEMPLATE: &str = "<|start_header_id|>{role}<|end_header_id|>\n\n{message}<|eot_id|>";

const STOP: &[&str] = &[TURN_END, CALL_CLOSE];

pub struct Llama;

impl Llama {
    fn markers() -> Markers {
        Markers {
            call_open: CALL_OPEN,
            call_close: Some(CALL_CLOSE),
        }
    }

    /// Recover a call's name from a llama fragment.
    ///
    /// The family emits a JSON object carrying `name`. Anything else is an
    /// attempted call whose name did not come back.
    fn recover(fragment: &str) -> Option<(ToolName, String)> {
        let value: serde_json::Value = serde_json::from_str(fragment.trim()).ok()?;
        let name = value.get("name")?.as_str()?;
        if name.is_empty() {
            return None;
        }
        let arguments = value
            .get("parameters")
            .or_else(|| value.get("arguments"))
            .map(|a| a.to_string())
            .unwrap_or_else(|| "{}".to_string());
        Some((ToolName(name.to_string()), arguments))
    }
}

impl Family for Llama {
    fn render_identity(&self, messages: &[Message]) -> String {
        let mut rendered = String::from(TEXT_BEGIN);
        for message in messages {
            rendered.push_str(&self.render_delta(message));
        }
        rendered
    }

    fn render_delta(&self, message: &Message) -> String {
        super::render_template(TEMPLATE, &message.role, &message.content)
    }

    fn parse(&self, emission: &str) -> Parsed {
        super::scan(emission, Self::markers(), Self::recover)
    }

    fn stop_conditions(&self) -> &'static [&'static str] {
        STOP
    }

    fn permits_truncation(&self) -> bool {
        true
    }

    fn declaration(&self) -> &'static Declaration {
        lookup(&FamilyName("llama".into())).expect("the registry carries llama")
    }
}
