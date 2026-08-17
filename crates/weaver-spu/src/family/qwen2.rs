//! The qwen2 family, per `weaver-spu-Spec` section 5.
//!
//! Everything qwen2 defines sits here: its ChatML markers, the rendering that
//! uses them, the parse of its own emissions, and its stop conditions. The
//! scanning beneath is [`super::scan`], shared with every other family and
//! holding no marker of its own.

use weaver_types::ToolName;

use super::{Declaration, Family, FamilyName, Markers, Message, Parsed, RenderRefusal, lookup};

/// The ChatML turn markers. Written here and nowhere else in the crate.
pub const TURN_OPEN: &str = "<|im_start|>";
pub const TURN_CLOSE: &str = "<|im_end|>";

/// What opens and closes a call in this family's emissions.
pub const CALL_OPEN: &str = "<tool_call>";
pub const CALL_CLOSE: &str = "</tool_call>";

/// **The markers this family's renderer emits into a prompt**, which is what
/// the inbound marker-promotion test of Spec section 10 tokenizes. The set is
/// the template's, because the template is what the renderer writes.
pub const RENDERED_MARKERS: &[&str] = &[TURN_OPEN, TURN_CLOSE];

/// The markers this family's **model** emits and this module's parser reads.
///
/// Outbound, so promotion does not bind them the way it binds
/// [`RENDERED_MARKERS`]: nothing here is tokenized by this crate, it is matched
/// as text against an emission. Both happen to promote under Qwen2.5's
/// tokenizer, which is a fact rather than a requirement this act asserts.
pub const PARSED_MARKERS: &[&str] = &[CALL_OPEN, CALL_CLOSE];

/// This family's template, the one authority for it. The registry cites this
/// constant and [`Family::render_delta`] renders through it, so the shape is
/// stated once.
pub const TEMPLATE: &str = "<|im_start|>{role}\n{message}<|im_end|>\n";

/// **What the delta's rendering closes with: the assistant's turn, opened and
/// left unfinished.** The generation continues from here, so the model
/// completes its own turn rather than electing a speaker, and the terminator
/// the engine makes resident after the stream is exactly this turn's close. A
/// rendering that ends at the user's `<|im_end|>` instead leaves the model
/// choosing who speaks next, which is how a completion opens with `user`. The
/// string restates [`TURN_OPEN`] because a const cannot concatenate one - the
/// marker-promotion test binds the marker itself, and this opener renders it
/// with the fixed role.
pub const GENERATION_OPENER: &str = "<|im_start|>assistant\n";

const STOP: &[&str] = &[TURN_CLOSE];

/// This family's renderer, cited by every registry entry it serves.
pub fn renderer() -> &'static dyn Family {
    &Qwen2
}

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
    fn render_identity(&self, messages: &[Message]) -> Result<String, RenderRefusal> {
        super::render_each(self, messages)
    }

    /// **The wire role is the canonical one here**, so this family calls the
    /// shared name kernel rather than holding a map of its own. ChatML's roles
    /// and the floor's are the same three words.
    fn render_delta(&self, message: &Message) -> Result<String, RenderRefusal> {
        Ok(super::render_template(
            TEMPLATE,
            super::common_role_name(&message.role)?,
            &super::text_content(message)?,
        ))
    }

    fn parse(&self, emission: &str) -> Parsed {
        super::scan(emission, Self::markers(), Self::recover)
    }

    fn stop_conditions(&self) -> &'static [&'static str] {
        STOP
    }

    fn declaration(&self) -> &'static Declaration {
        lookup(&FamilyName("qwen2".into())).expect("the registry carries qwen2")
    }
}
