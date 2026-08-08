//! The qwen2 family, per `weaver-spu-Spec` section 5.
//!
//! Everything qwen2 defines sits here: its ChatML markers, the rendering that
//! uses them, the parse of its own emissions, and its stop conditions. The
//! scanning beneath is [`super::scan`], shared with every other family and
//! holding no marker of its own.

use weaver_types::ToolName;

use super::{Declaration, Family, FamilyName, Markers, Message, Parsed, lookup};

/// The ChatML turn markers. Written here and nowhere else in the crate.
pub const TURN_OPEN: &str = "<|im_start|>";
pub const TURN_CLOSE: &str = "<|im_end|>";

/// What opens and closes a call in this family's emissions.
pub const CALL_OPEN: &str = "<tool_call>";
pub const CALL_CLOSE: &str = "</tool_call>";

/// Every control marker this family declares, which is what the inbound
/// marker-promotion test of Spec section 10 tokenizes.
pub const CONTROL_MARKERS: &[&str] = &[TURN_OPEN, TURN_CLOSE, CALL_OPEN, CALL_CLOSE];

/// This family's template, the one authority for it. The registry cites this
/// constant and [`Family::render_delta`] renders through it, so the shape is
/// stated once.
pub const TEMPLATE: &str = "<|im_start|>{role}\n{message}<|im_end|>\n";

const STOP: &[&str] = &[TURN_CLOSE];

pub struct Qwen2;

impl Qwen2 {
    fn markers() -> Markers {
        Markers {
            call_open: CALL_OPEN,
            call_close: Some(CALL_CLOSE),
        }
    }

    /// Recover a call's name from a qwen2 fragment.
    ///
    /// The family emits a JSON object carrying `name`. A fragment that is not
    /// that, or that is an object with no usable `name`, is unrecoverable, and
    /// the caller turns that into a reported fact rather than into text.
    fn recover(fragment: &str) -> Option<(ToolName, String)> {
        let value: serde_json::Value = serde_json::from_str(fragment.trim()).ok()?;
        let name = value.get("name")?.as_str()?;
        if name.is_empty() {
            return None;
        }
        let arguments = value
            .get("arguments")
            .map(|a| a.to_string())
            .unwrap_or_else(|| "{}".to_string());
        Some((ToolName(name.to_string()), arguments))
    }
}

impl Family for Qwen2 {
    fn render_identity(&self, messages: &[Message]) -> String {
        messages
            .iter()
            .map(|message| self.render_delta(message))
            .collect::<Vec<_>>()
            .join("")
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
        lookup(&FamilyName("qwen2".into())).expect("the registry carries qwen2")
    }
}
