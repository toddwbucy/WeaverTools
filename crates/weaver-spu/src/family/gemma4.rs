//! The gemma4 family, per `weaver-spu-Spec` section 5.
//!
//! Everything gemma4 defines sits here: its turn markers, the rendering that
//! uses them, the role it calls the assistant by, the preamble its prefix opens
//! with, the parse of its own emissions, and its stop conditions. The scanning
//! beneath is [`super::scan`], shared and marker-free.
//!
//! **This is the first family whose wire role is not the canonical one.** The
//! floor's [`Role::Assistant`] renders as `model` here, and the authority is
//! the artifact's own chat template, which performs exactly this substitution:
//! `set role = 'model' if message['role'] == 'assistant' else message['role']`.
//! The map is written in this module and nowhere else, so no other family and
//! no shared kernel learns the word.
//!
//! **It is also the first family with a once-per-prefix preamble that the
//! turns do not repeat.** The template emits `bos_token` before anything and
//! the artifact declares `add_bos_token: false`, so nothing downstream will add
//! it: [`crate::residency::Resident::tokenize`] calls `str_to_token` with
//! `AddBos::Never`. The prefix renders it and a delta must not, which is what
//! [`Family::render_identity`] and [`Family::render_delta`] are for.
//!
//! **The markers are asymmetric**, `<|turn>` opening and `<turn|>` closing,
//! where every other carried family uses a symmetric pair. Read them carefully:
//! the pipe moves rather than the brackets.
//!
//! Measured against `gemma-4-26B-A4B-it-qat-UD-Q4_K_XL.gguf`, whose header
//! declares `general.architecture = gemma4`, on 2026-08-16.

use weaver_traits::Role;
use weaver_types::ToolName;

use super::{Declaration, Family, FamilyName, Markers, Message, Parsed, RenderRefusal, lookup};

/// The turn markers. Written here and nowhere else in the crate.
pub const TURN_OPEN: &str = "<|turn>";
pub const TURN_CLOSE: &str = "<turn|>";

/// The channel markers, which this family **renders** rather than only reads.
///
/// The distinction matters and it is where this family parts from
/// [`crate::family::gpt_oss`], whose `CHANNEL` is a marker the model emits. Here
/// the generation prompt carries a pre-closed empty thought channel, so both
/// strings go into a prompt and both fall under [`RENDERED_MARKERS`] and the
/// promotion test that reads it.
pub const CHANNEL_OPEN: &str = "<|channel>";
pub const CHANNEL_CLOSE: &str = "<channel|>";

/// **The preamble the identity prefix opens with, once.**
///
/// The artifact's template emits `bos_token` before the first turn and declares
/// `add_bos_token: false`, and this crate tokenizes with `AddBos::Never`, so a
/// prompt that does not carry this string carries no BOS at all.
pub const BOS: &str = "<bos>";

/// What opens and closes a call in this family's emissions.
pub const CALL_OPEN: &str = "<|tool_call>";
pub const CALL_CLOSE: &str = "<tool_call|>";

/// **The markers this family's renderer emits into a prompt**, which is what
/// the inbound marker-promotion test of Spec section 10 tokenizes.
///
/// Five, where the other families carry two to four, because this family's
/// generation opener carries the channel pair and its prefix carries the BOS.
/// **Every string a prompt can contain is here**, which is the property that
/// makes the promotion test's answer about this family rather than about the
/// subset someone remembered to list.
pub const RENDERED_MARKERS: &[&str] = &[TURN_OPEN, TURN_CLOSE, CHANNEL_OPEN, CHANNEL_CLOSE, BOS];

/// The markers this family's **model** emits and this module's parser reads.
///
/// Outbound, so promotion does not bind them the way it binds
/// [`RENDERED_MARKERS`]: nothing here is tokenized by this crate, it is matched
/// as text against an emission. Both promote under this artifact's tokenizer,
/// which is a fact rather than a requirement this act asserts.
pub const PARSED_MARKERS: &[&str] = &[CALL_OPEN, CALL_CLOSE];

/// This family's template, the one authority for it. The registry cites this
/// constant and [`Family::render_delta`] renders through it.
pub const TEMPLATE: &str = "<|turn>{role}\n{message}<turn|>\n";

/// **What the delta's rendering closes with: the model's turn opened, and an
/// empty thought channel opened and closed behind it.**
///
/// The second half is not decoration and not a reasoning election made here.
/// The artifact's own template emits it whenever thinking is off, which is its
/// default, and the archived tree records what happens without it: the model
/// dangles after `<|turn>model\n` and degenerates into looping empty
/// `<|channel>thought\n<channel|>` markers until the token cap. **Reasoning-on
/// is a different opener and a different election**, not a value this constant
/// can carry, so the day a deployment wants it the change starts here.
///
/// Restates [`TURN_OPEN`] and the channel pair because a const cannot
/// concatenate one.
pub const GENERATION_OPENER: &str = "<|turn>model\n<|channel>thought\n<channel|>";

/// **The wire name of the assistant, and the whole of why this family exists
/// as a module rather than as a registry entry citing another.**
pub const ASSISTANT_ROLE: &str = "model";

/// The turn's close is the stop condition, and it is what the artifact
/// declares: `tokenizer.ggml.eot_token_id` is 106, which is [`TURN_CLOSE`].
const STOP: &[&str] = &[TURN_CLOSE];

/// This family's renderer, cited by its registry entry.
pub fn renderer() -> &'static dyn Family {
    &Gemma4
}

pub struct Gemma4;

impl Gemma4 {
    fn markers() -> Markers {
        Markers {
            call_open: CALL_OPEN,
            call_close: Some(CALL_CLOSE),
        }
    }

    /// Recover a call's name from a gemma4 fragment.
    ///
    /// The family emits `call:<name>{...}`, a routing word rather than the JSON
    /// object qwen2 emits, so the recovery reads the prefix and takes the name
    /// up to the brace. A fragment carrying no `call:` is an attempted call
    /// whose name did not come back, and the caller turns that into a reported
    /// fact rather than into text.
    fn recover(fragment: &str) -> Option<(ToolName, String)> {
        const CALL: &str = "call:";
        let at = fragment.find(CALL)? + CALL.len();
        let tail = &fragment[at..];
        let name: String = tail
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
            .collect();
        if name.is_empty() {
            return None;
        }
        let arguments = match tail.find('{') {
            Some(at) => tail[at..].trim().to_string(),
            None => "{}".to_string(),
        };
        Some((ToolName(name), arguments))
    }

    /// **This family's role map, which is why it does not call
    /// [`super::common_role_name`].**
    ///
    /// `Role::System` refuses rather than rendering, and this function is
    /// the refusal point: this family's template carries no system turn, so
    /// `role_name` answers `RenderRefusal::MalformedForFamily` rather than
    /// inventing `<|turn>system`, the silent substitution this crate
    /// refuses everywhere. A loop driving a gemma agent frames the field in
    /// its user turns, as every loop did before the slot existed.
    ///
    /// `Role::ToolResult` refuses rather than rendering as a turn. This family
    /// has no tool turn: its template carries tool results as standalone
    /// `<|tool_response>` blocks outside the turn structure, and inventing
    /// `<|turn>tool` would be the silent substitution the registry refuses one
    /// level up. It renders when the tool workflow lands and names the block
    /// shape, and not before.
    fn role_name(role: &Role) -> Result<&'static str, RenderRefusal> {
        Ok(match role {
            Role::User => "user",
            Role::Assistant => ASSISTANT_ROLE,
            _ => return Err(RenderRefusal::MalformedForFamily),
        })
    }
}

impl Family for Gemma4 {
    /// The prefix opens with [`BOS`], once, and the turns never repeat it.
    fn render_identity(&self, messages: &[Message]) -> Result<String, RenderRefusal> {
        let mut rendered = String::from(BOS);
        rendered.push_str(&super::render_each(self, messages)?);
        Ok(rendered)
    }

    fn render_delta(&self, message: &Message) -> Result<String, RenderRefusal> {
        Ok(super::render_template(
            TEMPLATE,
            Self::role_name(&message.role)?,
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
        lookup(&FamilyName("gemma4".into())).expect("the registry carries gemma4")
    }
}
