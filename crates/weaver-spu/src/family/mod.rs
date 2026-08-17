//! conforms: spu-shard-widths-are-a-set
//! conforms: spu-widths-set-pinned-by-doctest
//! conforms: spu-registry-no-silent-substitution
//! conforms: spu-share-kernels-own-orchestration
//! conforms: spu-parse-reports-unrecovered-call
//!
//! The family surface and its registry, per `weaver-spu-Spec` section 5.
//!
//! **One module per family, and the kernels shared beneath.** Everything a
//! family defines lives in that family's module: its marker vocabulary, its
//! template rendering, the parse of its own output, and its stop conditions.
//! What is common sits here as machinery the families drive rather than as
//! behaviour they inherit, which is the archived tree's share-kernels-own-
//! orchestration rule promoted to structure. [`scan`] is that kernel: it walks
//! an emission against whatever [`Markers`] a family hands it and owns no
//! marker of its own.
//!
//! The placement is the whole of the claim, and Spec section 5 buys it by
//! non-purchase: a module boundary reads the same to a test as to a reader, so
//! no test is written for it here. What a reader checks is that no marker
//! string appears in this file and that each family's constants sit in its own.
//!
//! **The registry is compile-time and admission consults it.** A family the
//! binary does not carry is a refused admit **naming the family**, which is the
//! archived tree's own no-silent-substitution ruling carried forward from its
//! encoder registry. Nothing here falls back to a nearest match: a substitution
//! that succeeds quietly is how a model runs under the wrong template.
//!
//! **The shard widths are a set rather than a maximum.** The field's type is a
//! set, so a maximum can no longer be declared, only read wrongly. The doctest
//! below reads a declaration carrying a non-contiguous set literal, which is
//! what makes the type unable to express the maximum it replaced:
//!
//! ```
//! use weaver_spu::family::Declaration;
//! // A non-contiguous set: this backend shards across one device or four, and
//! // not across two or three. No maximum describes that, which is the point.
//! const SPARSE: Declaration = Declaration {
//!     family: "sparse-example",
//!     shard_widths: &[1, 4],
//!     template: "{message}",
//!     generation_opener: "",
//!     renderer: weaver_spu::family::qwen2::renderer,
//!     // Empty: this example's architecture is its own, so nothing competes
//!     // with it and no marker set is read to choose.
//!     selecting_markers: &[],
//!     flush: weaver_spu::decoder::backend::FlushMechanism::TruncateToPosition,
//!     taps_readout: true,
//! };
//! assert!(SPARSE.shards_across(4));
//! assert!(!SPARSE.shards_across(2));
//! assert!(!SPARSE.shards_across(3));
//! ```

use weaver_traits::{ContentBlock, Message, Role};
use weaver_types::{LifecycleRefusal, TokenRefusal, ToolName};

use crate::decoder::backend::FlushMechanism;

pub mod gemma4;
pub mod gpt_oss;
pub mod llama;
pub mod mistral3;
pub mod phi;
pub mod qwen2;

/// Why a family could not render a message.
///
/// **The crate's own vocabulary rather than the floor's**, for the reason
/// [`FamilyRefusal`] is: what a render refuses on is a family fact, and the
/// wire's word for it is the composition root's to choose. The [`From`] below
/// is where the two meet.
///
/// One case, and both of the floor's growth points feed it. `Role` and
/// `ContentBlock` are each `non_exhaustive`, so a family's rendering is a match
/// with a wildcard arm, and what the wildcard means is that this family has no
/// rendering for what arrived. **That is the contract's own case rather than
/// one invented here:** the tool shapes are blocked with the tool workflow and
/// the families render text today.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderRefusal {
    /// The message carries a role or a block this family has no rendering for.
    MalformedForFamily,
}

impl From<RenderRefusal> for TokenRefusal {
    /// What crosses the seam. A render that refused is the delta malformed for
    /// the family, which is the only shape the decode contract carries for it.
    fn from(_: RenderRefusal) -> Self {
        TokenRefusal::MalformedDelta
    }
}

/// **The role names most families render, as a shared kernel they drive.**
///
/// A family calls this because its wire vocabulary happens to be the canonical
/// one, not because it inherits it. A family whose wire name differs writes its
/// own match and does not call this, which is the whole of how [`gemma4`]
/// renders its assistant as `model` without any other family learning the word.
///
/// The wildcard arm is the floor's growth point, `Role` being `non_exhaustive`.
/// No test can construct its subject today, every current case being rendered.
pub fn common_role_name(role: &Role) -> Result<&'static str, RenderRefusal> {
    Ok(match role {
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::ToolResult => "tool",
        _ => return Err(RenderRefusal::MalformedForFamily),
    })
}

/// A message's text, joined, for the families that render text and nothing
/// else.
///
/// A block that is not text is unrenderable rather than skipped: a tool call
/// silently dropped from a prompt is a turn the model answers without knowing
/// what it was asked, which is the silent substitution this crate refuses
/// everywhere else.
pub fn text_content(message: &Message) -> Result<String, RenderRefusal> {
    let mut content = String::new();
    for block in &message.content {
        match block {
            ContentBlock::Text { text } => content.push_str(text),
            _ => return Err(RenderRefusal::MalformedForFamily),
        }
    }
    Ok(content)
}

/// **The shared rendering kernel: every message through the family's own delta
/// rendering, in order, and nothing else.**
///
/// It holds no marker, no role name, and no preamble, which is what keeps a
/// family's whole vocabulary inside that family's module. A family whose
/// identity prefix opens with something the turns do not repeat adds it around
/// this call rather than inside it - see [`gemma4::Gemma4::render_identity`],
/// where the once-per-prefix `<bos>` lives.
pub fn render_each(family: &dyn Family, messages: &[Message]) -> Result<String, RenderRefusal> {
    let mut rendered = String::new();
    for message in messages {
        rendered.push_str(&family.render_delta(message)?);
    }
    Ok(rendered)
}

/// One piece of an emission, recovered.
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    /// Ordinary assistant text.
    Text(String),
    /// A call the parser recovered whole: the marker opened and the name came
    /// back.
    Call { name: ToolName, arguments: String },
}

/// A call the emission attempted and the parser could not recover.
///
/// **This is its own reported fact rather than a clean turn**, per Spec section
/// 5. The archived tree drew the same line between a call that could not be
/// rendered and a call whose name could not be recovered, and the second is
/// this. The fragment travels with it so the record holds what was attempted,
/// and it is deliberately not also emitted as [`Content::Text`]: a fragment
/// that arrives as ordinary prose is a failed call the turn reads as success.
#[derive(Debug, Clone, PartialEq)]
pub struct Unrecovered {
    /// What sat between the call marker and the end of the emission or its
    /// closing marker.
    pub fragment: String,
}

/// What a parse answers with.
///
/// The verbatim emission rides alongside the canonical form because the decode
/// contract has both reaching the record, per the operator's end-to-end
/// requirement.
#[derive(Debug, Clone, PartialEq)]
pub struct Parsed {
    pub verbatim: String,
    pub content: Vec<Content>,
    pub unrecovered: Vec<Unrecovered>,
}

impl Parsed {
    /// Whether the emission attempted a call the parse could not recover.
    pub fn has_unrecovered_call(&self) -> bool {
        !self.unrecovered.is_empty()
    }

    /// The assistant text alone, joined. Used by tests that must show a
    /// fragment did **not** arrive as prose.
    pub fn text(&self) -> String {
        self.content
            .iter()
            .filter_map(|piece| match piece {
                Content::Text(text) => Some(text.as_str()),
                Content::Call { .. } => None,
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

/// A family's control markers, handed to the shared kernel.
///
/// Every string here is the family's own and none is written in this module,
/// which is the placement rule stated as a type: the kernel cannot recognise a
/// marker no family gave it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Markers {
    /// What opens a call in this family's emissions.
    pub call_open: &'static str,
    /// What closes one, where the family closes them. `None` means the call
    /// runs to the end of the emission.
    pub call_close: Option<&'static str>,
}

/// The surface a family implements, small and named, per Spec section 5 and
/// charter section 14.
///
/// Six members and no seventh. Membership is the charter's enumeration and
/// takes no record of its own.
///
/// **The messages are the floor's, which is what section 5 says and what this
/// surface now reads.** It formerly took a message shape local to this module,
/// and nothing on the decode path called it: the render ran in the composition
/// root off the [`Declaration`]'s template string, so a family could hold no
/// rendering fact beyond that one constant. A family whose wire role differs
/// from the canonical one, or whose prefix opens with a string its turns do not
/// repeat, had nowhere to say so. Both arrived with [`gemma4`], and the repair
/// is this surface carrying the traffic Spec section 5 always said it carried.
pub trait Family {
    /// Render an identity prefix from canonical messages.
    ///
    /// The prefix's turns are all complete, so no generation opener belongs
    /// here. What may belong is a once-per-prefix preamble the per-turn
    /// rendering must not repeat.
    fn render_identity(&self, messages: &[Message]) -> Result<String, RenderRefusal>;

    /// Render a turn's delta.
    fn render_delta(&self, message: &Message) -> Result<String, RenderRefusal>;

    /// Parse an emission into canonical content, this family's markers
    /// recognised.
    fn parse(&self, emission: &str) -> Parsed;

    /// The stop conditions this family declares.
    fn stop_conditions(&self) -> &'static [&'static str];

    /// The capabilities admission judges against.
    ///
    /// **A module serving several architecture keys answers for the key it is
    /// named after, not for the artifact in hand.** [`qwen2`] serves five keys,
    /// so `Qwen2::declaration()` is qwen2's row whichever of them was admitted.
    /// Read a per-artifact fact through [`lookup`] against the header's family
    /// instead, which is what the decode path does.
    fn declaration(&self) -> &'static Declaration;
}

/// The shared parse kernel.
///
/// **It owns no marker.** Every string it matches on arrives in `markers`, so a
/// family's vocabulary lives in the family's module and this function is the
/// orchestration the families drive rather than inherit.
///
/// `recover` is the family's own name extraction, because how a name sits
/// inside a call fragment is a family fact. Returning `None` is what produces
/// an [`Unrecovered`], and the fragment does not also become text.
pub fn scan(
    emission: &str,
    markers: Markers,
    recover: impl Fn(&str) -> Option<(ToolName, String)>,
) -> Parsed {
    let mut content = Vec::new();
    let mut unrecovered = Vec::new();
    let mut rest = emission;

    while let Some(open) = rest.find(markers.call_open) {
        let (before, after_open) = rest.split_at(open);
        if !before.is_empty() {
            content.push(Content::Text(before.to_string()));
        }
        let after_open = &after_open[markers.call_open.len()..];

        // Where the fragment ends: the family's closing marker, or the whole
        // remainder where the family closes nothing.
        let (fragment, remainder) = match markers.call_close {
            Some(close) => match after_open.find(close) {
                Some(at) => (&after_open[..at], &after_open[at + close.len()..]),
                // The marker opened and never closed. That is an attempted call
                // whose extent is the rest of the emission.
                None => (after_open, ""),
            },
            None => (after_open, ""),
        };

        match recover(fragment) {
            Some((name, arguments)) => content.push(Content::Call { name, arguments }),
            None => unrecovered.push(Unrecovered {
                fragment: fragment.to_string(),
            }),
        }
        rest = remainder;
    }

    if !rest.is_empty() {
        content.push(Content::Text(rest.to_string()));
    }

    Parsed {
        verbatim: emission.to_string(),
        content,
        unrecovered,
    }
}

/// A family's name, as the artifact header declares it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FamilyName(pub String);

/// What one family declares about itself, at compile time.
///
/// **No [`PartialEq`], and the reason is [`Declaration::renderer`].** A
/// function pointer's address is not guaranteed unique, so a derived comparison
/// would answer about addresses rather than about families and the compiler
/// says so. Nothing compares declarations - what callers read is a field - so
/// the derive is dropped rather than hand-written around the one member that
/// cannot answer.
#[derive(Debug, Clone, Copy)]
pub struct Declaration {
    /// The name an artifact header must carry to select this family.
    pub family: &'static str,
    /// **The widths this backend can shard across, as a set.** Membership is
    /// the test, never a comparison against a bound: a set is what lets a
    /// backend declare that it shards across one or four and not two.
    pub shard_widths: &'static [u32],
    /// The template this family renders the harness's canonical messages
    /// through.
    pub template: &'static str,
    /// **What a delta's rendering closes with: the assistant's turn opened
    /// and left unfinished**, so the generation completes that turn rather
    /// than electing a speaker. Appended to the delta's rendering only, never
    /// to the identity prefix, whose turns are all complete.
    pub generation_opener: &'static str,
    /// **The family's renderer, cited rather than written here**, for the
    /// reason [`Declaration::template`] is cited: the module is the authority
    /// and this table points at it.
    ///
    /// A function returning the object rather than the object itself, because
    /// a `&dyn` field would take this struct's derived [`PartialEq`] with it
    /// and the width judgment reads declarations by value. **One table, not
    /// two:** a second map from family name to renderer would be the same fact
    /// in two places with no authority named, which G5 files as a defect.
    pub renderer: fn() -> &'static dyn Family,
    /// **The markers this entry renders, which select it among the entries
    /// sharing its architecture**, per Spec section 5.
    ///
    /// Cited from the family module rather than written here, for the reason
    /// [`Declaration::template`] is cited: the module is the authority for
    /// what it emits, and a second spelling here would be one fact in two
    /// places with no authority named.
    ///
    /// **Read only where an architecture is contested.** An architecture
    /// carrying one entry selects on the architecture and never renders, which
    /// is what keeps a family the template detector does not recognise
    /// admissible. Gemma4 is such a family today.
    pub selecting_markers: &'static [&'static str],
    /// **How this family's flush reaches its fixed outcome**, per Spec section
    /// 4.4. Declared here rather than inferred from a version string, because
    /// a truncation that returns success while recurrent state stays is the
    /// silent failure the append-only discipline exists to prevent.
    pub flush: FlushMechanism,
    /// **Whether this family's engine can tap for residual readout**, per Spec
    /// section 5's capability list and section 7's admit judgment. Declared
    /// rather than probed, because an election judged at admit cannot wait for
    /// a forward to find out.
    ///
    /// **Every shipped family declares `false` today and that is the truthful
    /// value, not a placeholder.** Nothing implements [`crate::readout::Tap`],
    /// because neither backend exists, so a family advertising a tap it cannot
    /// perform would be admitted under an election nothing could honor, which
    /// is the expensive lie the admit judgment exists to prevent. Each flips in
    /// the act that stands its engine's tap up, and not before.
    pub taps_readout: bool,
}

impl Declaration {
    /// **Whether this family's session state permits truncation to a
    /// position**, per Spec section 5's surface and section 4.4's mechanism.
    ///
    /// Derived from [`Declaration::flush`] rather than declared beside it,
    /// because 4.4 makes them one fact: truncation is permitted exactly where
    /// the flush is reached by truncating. It was a second declaration on the
    /// [`Family`] trait until 2026-08-17, which is one fact in two places with
    /// no authority named, and the two could disagree - every family said
    /// `true` there, including the two whose engines refuse to roll back.
    ///
    /// **It lives here and not on the trait because this is keyed per
    /// architecture.** A module serving several keys has one trait object and
    /// several rows, and the answer differs between them: the qwen2 module
    /// serves both a truncating qwen2 and a re-establishing qwen35.
    pub const fn permits_truncation(&self) -> bool {
        matches!(self.flush, FlushMechanism::TruncateToPosition)
    }

    /// Whether this backend shards across exactly this many devices.
    ///
    /// Membership rather than a bound. A wider set refuses against the
    /// declaration rather than against a hidden limit, so the day an N-way path
    /// lands the declaration changes and nothing else does.
    pub const fn shards_across(&self, width: u32) -> bool {
        let mut index = 0;
        while index < self.shard_widths.len() {
            if self.shard_widths[index] == width {
                return true;
            }
            index += 1;
        }
        false
    }
}

/// The compile-time table.
///
/// **Today the salvaged tensor-parallel path is a two-device implementation,**
/// `forward_tp2` with an all-reduce kernel written for a pair, so the declared
/// set is one or two. It is written as a set rather than as the maximum two so
/// that a three-device path arriving without a two-device path is expressible.
/// **The template strings are each family's own and are referenced here rather
/// than written here**, per the placement rule of Spec section 5. A template
/// spelled in this file would be family-specific material outside its module,
/// and once the family module renders through the same constant it would also
/// be one fact in two places with no authority named, which G5 files as a
/// defect rather than something to resolve by picking. The family module is the
/// authority and this table cites it.
pub const REGISTRY: &[Declaration] = &[
    Declaration {
        family: "llama",
        shard_widths: &[1, 2],
        template: llama::TEMPLATE,
        generation_opener: llama::GENERATION_OPENER,
        renderer: llama::renderer,
        selecting_markers: llama::RENDERED_MARKERS,
        flush: FlushMechanism::TruncateToPosition,
        taps_readout: false,
    },
    Declaration {
        // **The llama architecture's second reading, and the entry that makes
        // the architecture contested.** SmolLM2 declares `general.architecture
        // = llama` and renders ChatML, so the entry above hands it a Llama 3
        // stop set it was never trained against and the load refuses. The
        // refusal is correct and the gap was the table, which had one row for
        // an architecture its artifacts read two ways.
        //
        // **Everything here is the ChatML module's**, cited for the reason
        // `nemotron_h_moe` cites it: the format is that module's to explain
        // and this row points at it. What distinguishes this row from that one
        // is the architecture it answers for, which is the key.
        family: "llama",
        shard_widths: &[1, 2],
        template: qwen2::TEMPLATE,
        generation_opener: qwen2::GENERATION_OPENER,
        renderer: qwen2::renderer,
        selecting_markers: qwen2::RENDERED_MARKERS,
        flush: FlushMechanism::TruncateToPosition,
        taps_readout: false,
    },
    Declaration {
        family: "qwen2",
        shard_widths: &[1, 2],
        template: qwen2::TEMPLATE,
        generation_opener: qwen2::GENERATION_OPENER,
        renderer: qwen2::renderer,
        selecting_markers: qwen2::RENDERED_MARKERS,
        flush: FlushMechanism::TruncateToPosition,
        taps_readout: false,
    },
    Declaration {
        // **Qwen3 declares its own architecture and renders the same ChatML
        // scaffolding**, so it cites the qwen2 module rather than growing a
        // second one that would hold the same two markers. Measured: the
        // marker vocabulary is identical and both promote to one token, and the
        // turn shape this template encodes is what both models' own templates
        // produce.
        //
        // Their full templates are not identical. Qwen3's is longer and adds
        // `<think>` reasoning blocks and an `enable_thinking` switch. That is
        // outbound, so it reaches the parse rather than the render: a reasoning
        // block arrives as ordinary assistant text today. Named here so the day
        // it needs separating is a change with a stated starting point rather
        // than a surprise in a trace.
        family: "qwen3",
        shard_widths: &[1, 2],
        template: qwen2::TEMPLATE,
        generation_opener: qwen2::GENERATION_OPENER,
        renderer: qwen2::renderer,
        selecting_markers: qwen2::RENDERED_MARKERS,
        flush: FlushMechanism::TruncateToPosition,
        taps_readout: false,
    },
    Declaration {
        // **Qwen3's sparse sibling declares its own architecture**, so it is
        // its own key citing the qwen2 module for the reason the dense one
        // does. Measured by the marker probe against a Qwen3-30B-A3B
        // tokenizer: the rendered set is qwen2's exactly, carrying not even
        // the vision markers the qwen35 artifacts do.
        family: "qwen3moe",
        shard_widths: &[1, 2],
        template: qwen2::TEMPLATE,
        generation_opener: qwen2::GENERATION_OPENER,
        renderer: qwen2::renderer,
        selecting_markers: qwen2::RENDERED_MARKERS,
        flush: FlushMechanism::TruncateToPosition,
        taps_readout: false,
    },
    Declaration {
        // **Qwen3.5 declares its own architecture and renders the same ChatML
        // scaffolding**, so it cites the qwen2 module for the reason qwen3
        // does rather than growing a third holding the same two markers.
        // Measured by the marker probe of `tests/markers.rs` against a
        // Qwen3.6 tokenizer, which is what declares this architecture: the
        // rendered set promotes to one token each, and a lookalike that no
        // family declares does not, so the vocabulary agrees rather than the
        // tokenizer promoting everything.
        //
        // The artifacts also carry vision markers, `<|vision_start|>` and its
        // neighbours, which no text turn renders. They are named here as
        // present and unused rather than left for a later reader to wonder at.
        //
        // **The flush is by re-establishing, and this is the first entry where
        // it is.** Corrected 2026-08-17: it declared truncation, copied from
        // the qwen2 entry along with the template it legitimately shares.
        // Sharing a marker vocabulary says nothing about how state rolls back.
        //
        // `llm_arch_is_hybrid` in the pinned llama.cpp names `QWEN35` and
        // `QWEN35MOE`, so these artifacts carry recurrent layers beside their
        // attention. A recurrent state is a running summary rather than a
        // per-position cache, so it cannot be partially erased, and the engine
        // says so: measured on a Qwen3.6 artifact, `seq_rm` answered `false`
        // while the seam reported the turn flushed.
        family: "qwen35",
        shard_widths: &[1, 2],
        template: qwen2::TEMPLATE,
        generation_opener: qwen2::GENERATION_OPENER,
        renderer: qwen2::renderer,
        selecting_markers: qwen2::RENDERED_MARKERS,
        flush: FlushMechanism::ReestablishAndReprefill,
        taps_readout: false,
    },
    Declaration {
        // The sparse sibling declares its own architecture again, so it is its
        // own key and its own probe. Two keys citing one module is a claim
        // about their markers agreeing, and this one is measured rather than
        // inherited from the dense entry above.
        // Its sparse sibling is hybrid for the same reason and by the same
        // list, and it is the artifact the measurement above was taken on.
        family: "qwen35moe",
        shard_widths: &[1, 2],
        template: qwen2::TEMPLATE,
        generation_opener: qwen2::GENERATION_OPENER,
        renderer: qwen2::renderer,
        selecting_markers: qwen2::RENDERED_MARKERS,
        flush: FlushMechanism::ReestablishAndReprefill,
        taps_readout: false,
    },
    Declaration {
        // **The first entry that had to grow a module rather than cite one.**
        // Every qwen key above renders qwen2's scaffolding under its own
        // architecture, so the entry was the whole act. This family renames a
        // role and opens its prefix with a token its turns do not repeat, and
        // neither is a string a table row can hold.
        //
        // Measured by the marker probe of `tests/markers.rs` against
        // `gemma-4-26B-A4B-it-qat-UD-Q4_K_XL.gguf`, whose header declares this
        // architecture: all five rendered markers promote to exactly one token,
        // and the lookalike control does not, so the vocabulary agrees rather
        // than the tokenizer promoting everything. Two of the five, the channel
        // pair, are `USER_DEFINED` rather than `CONTROL` in that artifact's
        // token table, which is why reading the template was not enough.
        //
        // The flush is by truncation for the reason every entry above declares
        // it: attention KV rolls back by position. The interleaved sliding
        // window this family's attention uses is a window over positions and
        // does not change that. It is declared rather than inferred from the
        // architecture string, per Spec section 4.4.
        family: "gemma4",
        shard_widths: &[1, 2],
        template: gemma4::TEMPLATE,
        generation_opener: gemma4::GENERATION_OPENER,
        renderer: gemma4::renderer,
        selecting_markers: gemma4::RENDERED_MARKERS,
        flush: FlushMechanism::TruncateToPosition,
        taps_readout: false,
    },
    Declaration {
        // **The same module as the qwen keys, and a different flush.** Its
        // template is `<|im_start|>role\n ... <|im_end|>\n`, qwen2's
        // scaffolding exactly, so it cites that module for the reason qwen3
        // does. Measured by the marker probe against a Nemotron 3 Nano 30B A3B
        // artifact: the rendered set is qwen2's and both markers promote.
        //
        // **What it does not share is how state rolls back.**
        // `llm_arch_is_hybrid` names `NEMOTRON_H_MOE`, so this entry declares
        // `ReestablishAndReprefill` beside four qwen keys citing the same
        // module, two of which truncate and two of which do not. That is the
        // whole reason the flush is declared per architecture here rather than
        // answered by the shared renderer: one module, one trait object, and
        // three different answers across the rows it serves.
        //
        // The `<think>` pair its model emits is outbound, reaching the parse
        // rather than the render, on the same footing as qwen3's and named at
        // that entry.
        family: "nemotron_h_moe",
        shard_widths: &[1, 2],
        template: qwen2::TEMPLATE,
        generation_opener: qwen2::GENERATION_OPENER,
        renderer: qwen2::renderer,
        selecting_markers: qwen2::RENDERED_MARKERS,
        flush: FlushMechanism::ReestablishAndReprefill,
        taps_readout: false,
    },
    Declaration {
        // **The first family that names no role**, wrapping the user's text in
        // `[INST]` and `[/INST]` and leaving the model's bare, so the shape
        // carries the speaker and there is nothing for a placeholder to fill.
        // Its generation opener is empty because `[/INST]` is the assistant's
        // turn already opened. Both are the module's to explain.
        //
        // Measured by the marker probe against
        // `Devstral-Small-2-24B-Instruct-2512-Q4_K_M.gguf`: all four rendered
        // markers promote to one token, at ids 1, 2, 3 and 4, and the lookalike
        // control does not.
        //
        // **Most Mistral artifacts do not declare this architecture.**
        // Mistral-Small 3.1, Mistral-Small 3.2 and Magistral-Small report
        // `llama` and would resolve to that entry's llama-3 header markers,
        // which their tokenizers do not promote. That is #88's open case and
        // this entry does not close it: what this entry serves is the artifacts
        // whose header says `mistral3`.
        //
        // The flush is by truncation: this family is absent from
        // `llm_arch_is_hybrid` and from `llm_arch_is_recurrent`, so its
        // attention KV rolls back by position, and `tests/markers.rs` checks
        // that declaration against the artifact rather than trusting this
        // comment.
        family: "mistral3",
        shard_widths: &[1, 2],
        template: mistral3::TEMPLATE,
        generation_opener: mistral3::GENERATION_OPENER,
        renderer: mistral3::renderer,
        selecting_markers: mistral3::RENDERED_MARKERS,
        flush: FlushMechanism::TruncateToPosition,
        taps_readout: false,
    },
    Declaration {
        // **The second contested architecture, and the first where a vendor's
        // own current line is what contests it.** Three first-party Microsoft
        // artifacts declare `phi3`: the minis render role tags and Phi-4 14B
        // renders ChatML with a separator token. The marker sets are disjoint
        // to the point that the mini's vocabulary carries no `<|im_start|>`
        // at all, so the two rows cannot be ambiguous under any rendering.
        //
        // Measured by the marker probe against
        // `microsoft_Phi-4-mini-instruct-Q4_K_M.gguf`: the three tag markers
        // promote to one token each, ids 200021, 200019, 200020, and the
        // lookalike control does not. The flush is by truncation, `phi3`
        // absent from both of the engine's cannot-roll-back lists, and the
        // probe's cross-check reads that against the artifact rather than
        // trusting this comment.
        family: "phi3",
        shard_widths: &[1, 2],
        template: phi::TAG_TEMPLATE,
        generation_opener: phi::TAG_GENERATION_OPENER,
        renderer: phi::tag_renderer,
        selecting_markers: phi::TAG_RENDERED_MARKERS,
        flush: FlushMechanism::TruncateToPosition,
        taps_readout: false,
    },
    Declaration {
        // The separator row of the pair above, serving Phi-4 14B artifacts.
        // **Not a qwen2 citation on purpose**: `<|im_sep|>` stands where
        // qwen2's ChatML puts a newline and no newline follows the close, so
        // the format is phi's own and lives in phi's module.
        family: "phi3",
        shard_widths: &[1, 2],
        template: phi::SEP_TEMPLATE,
        generation_opener: phi::SEP_GENERATION_OPENER,
        renderer: phi::sep_renderer,
        selecting_markers: phi::SEP_RENDERED_MARKERS,
        flush: FlushMechanism::TruncateToPosition,
        taps_readout: false,
    },
    Declaration {
        // The key is what llama.cpp writes into `general.architecture`, which
        // is hyphenated. A key spelled any other way is a family no artifact
        // header ever selects, unreachable rather than wrong-looking.
        family: "gpt-oss",
        shard_widths: &[1, 2],
        template: gpt_oss::TEMPLATE,
        generation_opener: gpt_oss::GENERATION_OPENER,
        renderer: gpt_oss::renderer,
        selecting_markers: gpt_oss::RENDERED_MARKERS,
        flush: FlushMechanism::TruncateToPosition,
        taps_readout: false,
    },
];

/// Render one message through a family's template, in a single pass.
///
/// **The shared kernel again, and it holds no marker.** The two placeholders
/// are the template's own vocabulary rather than any family's. A single pass is
/// what keeps a message whose text contains `{role}` from being substituted
/// into: successive `replace` calls would rewrite the output of the one before.
pub fn render_template(template: &str, role: &str, message: &str) -> String {
    let mut rendered = String::with_capacity(template.len() + role.len() + message.len());
    let mut rest = template;
    while let Some(at) = rest.find('{') {
        rendered.push_str(&rest[..at]);
        let tail = &rest[at..];
        if let Some(stripped) = tail.strip_prefix("{role}") {
            rendered.push_str(role);
            rest = stripped;
        } else if let Some(stripped) = tail.strip_prefix("{message}") {
            rendered.push_str(message);
            rest = stripped;
        } else {
            // Not a placeholder. Emit the brace and continue past it, so a
            // template carrying a literal brace renders unharmed.
            rendered.push('{');
            rest = &tail[1..];
        }
    }
    rendered.push_str(rest);
    rendered
}

/// Why a family lookup or a width judgment refused.
///
/// This is the crate's own vocabulary rather than the floor's, because the
/// floor's refusal set carries no case that names a family, and **what the
/// no-silent-substitution test reads is the family the refusal names.** A
/// refusal arriving on some other ground does not satisfy it, so the name has to
/// survive to the place the test reads.
#[derive(Debug, Clone, PartialEq)]
pub enum FamilyRefusal {
    /// The artifact header names a family this binary does not carry. The name
    /// travels with the refusal rather than being flattened to a generic
    /// unreadable, which is what makes the substitution visible.
    UnknownFamily(FamilyName),
    /// **More than one entry declares this architecture, and the caller asked
    /// with the architecture alone.** Answering by position would make the
    /// table's order the selector, so this is a refusal rather than a first
    /// match. The caller holding the artifact's template reaches [`select`].
    ArchitectureContested(FamilyName),
    /// The architecture is contested and the artifact declares no chat
    /// template, so the fact that would choose between the entries is absent.
    TemplateAbsent(FamilyName),
    /// The architecture is contested and the template detector did not
    /// recognise this artifact's template, so no marker set was derived.
    TemplateUnrecognised(FamilyName),
    /// The artifact rendered, and its markers match no entry sharing its
    /// architecture. The refusal names the architecture it declared, per Spec
    /// section 5, rather than reporting the family as uncarried.
    MarkersMatchNoEntry(FamilyName),
    /// **The artifact's markers match more than one entry.** The compile-time
    /// uniqueness pin should make this unreachable, and it is carried because
    /// a table this crate cannot read unambiguously is one it must not read by
    /// position. Belt and braces, per Spec section 5: order decides nothing at
    /// either.
    MarkersAmbiguous(FamilyName),
    /// The family is carried, but it does not shard across the requested width.
    WidthNotDeclared {
        family: FamilyName,
        requested: u32,
        declared: &'static [u32],
    },
}

impl From<FamilyRefusal> for LifecycleRefusal {
    /// What crosses the seam. The floor's set is closed at `weaver-types` and
    /// carries no family-naming case, so the name is lost at the boundary and
    /// kept on this side of it. Both cases are admission judgments about a
    /// device set or an artifact this binary cannot serve.
    fn from(refusal: FamilyRefusal) -> Self {
        match refusal {
            FamilyRefusal::UnknownFamily(_)
            // **Every unresolved selection crosses as unreadable**, which is
            // what the floor's closed set carries for an artifact this binary
            // cannot serve. The distinctions above are this crate's to act on
            // and the operator's to read in the refusal, and none of them is a
            // device condition, so none maps to `DeviceCannotAdmit`.
            | FamilyRefusal::ArchitectureContested(_)
            | FamilyRefusal::TemplateAbsent(_)
            | FamilyRefusal::TemplateUnrecognised(_)
            | FamilyRefusal::MarkersMatchNoEntry(_)
            | FamilyRefusal::MarkersAmbiguous(_) => LifecycleRefusal::ArtifactUnreadable,
            FamilyRefusal::WidthNotDeclared { .. } => LifecycleRefusal::DeviceCannotAdmit,
        }
    }
}

/// Byte equality for two string constants, in a const context.
const fn same_text(left: &str, right: &str) -> bool {
    let (left, right) = (left.as_bytes(), right.as_bytes());
    if left.len() != right.len() {
        return false;
    }
    let mut index = 0;
    while index < left.len() {
        if left[index] != right[index] {
            return false;
        }
        index += 1;
    }
    true
}

/// Whether every marker in `inner` appears in `outer`.
const fn contained_in(inner: &[&str], outer: &[&str]) -> bool {
    let mut i = 0;
    while i < inner.len() {
        let mut found = false;
        let mut j = 0;
        while j < outer.len() {
            if same_text(inner[i], outer[j]) {
                found = true;
            }
            j += 1;
        }
        if !found {
            return false;
        }
        i += 1;
    }
    true
}

/// **Whether the table can be read unambiguously**, which is the compile-time
/// property Spec section 5 requires of it.
///
/// **The pin is subset-freedom rather than inequality, and the difference is
/// what makes the guarantee true.** Selection matches an entry when every one
/// of its markers appears in what the artifact rendered, so two entries whose
/// sets merely differ can still both match: where one set is contained in the
/// other, an artifact rendering the larger satisfies both. Pinning inequality
/// alone would leave ambiguity reachable while reading as though it were
/// closed, and section 5 says the compile-time check is what makes ambiguity
/// unreachable. Equal sets are contained in each other, so this subsumes the
/// uniqueness the section names rather than replacing it.
///
/// Marker sets are compared only within one architecture, because entries that
/// declare different architectures never compete.
const fn table_reads_unambiguously() -> bool {
    let mut i = 0;
    while i < REGISTRY.len() {
        let mut j = i + 1;
        while j < REGISTRY.len() {
            if same_text(REGISTRY[i].family, REGISTRY[j].family)
                && (contained_in(REGISTRY[i].selecting_markers, REGISTRY[j].selecting_markers)
                    || contained_in(REGISTRY[j].selecting_markers, REGISTRY[i].selecting_markers))
            {
                return false;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

const _: () = assert!(
    table_reads_unambiguously(),
    "two registry entries share an architecture and neither marker set \
     distinguishes them, so the table's order would be the selector"
);

/// The conversation every contested artifact is asked to render.
///
/// **Frozen here rather than built where it is called**, per Spec section 5. A
/// derivation whose input varies by call site compares two artifacts against
/// two different questions, and the comparison is only meaningful where every
/// artifact answered the same one. The contents carry no meaning: what is read
/// is the scaffolding the template puts around them, so the bodies are single
/// characters and the roles are the three the canonical shape names.
///
/// Read only by the detector arm below, so a build carrying no engine has no
/// reader for it. The allow says that rather than widening the build's surface
/// to quiet a lint, which is the form [`crate::residency`] already uses for the
/// flush mechanism.
#[cfg_attr(not(feature = "gguf"), allow(dead_code))]
const PROBE_CONVERSATION: &[(&str, &str)] = &[("system", "s"), ("user", "u"), ("assistant", "a")];

/// Render the probe through the template detector and return what it emits.
///
/// **This is a detector rather than an evaluator**, per Spec section 5: it
/// matches the template text against a table of families it carries and emits
/// its own rendering of the one it settles on. It is not a jinja parser and
/// upstream says so. It takes no model handle, so this reaches no weights.
#[cfg(feature = "gguf")]
fn render_probe(template: &str) -> Option<String> {
    use std::ffi::CString;
    let template = CString::new(template).ok()?;
    let held: Vec<(CString, CString)> = PROBE_CONVERSATION
        .iter()
        .map(|(role, body)| (CString::new(*role).unwrap(), CString::new(*body).unwrap()))
        .collect();
    let chat: Vec<llama_cpp_sys_2::llama_chat_message> = held
        .iter()
        .map(|(role, body)| llama_cpp_sys_2::llama_chat_message {
            role: role.as_ptr(),
            content: body.as_ptr(),
        })
        .collect();
    // The rendering is the probe's scaffolding around six bytes of body, so a
    // fixed buffer is sufficient and a refusal is what an overrun becomes.
    let mut buffer = vec![0u8; 8192];
    // SAFETY: the template and the messages outlive the call, `chat` describes
    // its own length, and the buffer's length is passed as written.
    let written = unsafe {
        llama_cpp_sys_2::llama_chat_apply_template(
            template.as_ptr(),
            chat.as_ptr(),
            chat.len(),
            true,
            buffer.as_mut_ptr().cast::<std::os::raw::c_char>(),
            buffer.len() as i32,
        )
    };
    let written = usize::try_from(written).ok()?;
    if written > buffer.len() {
        return None;
    }
    buffer.truncate(written);
    String::from_utf8(buffer).ok()
}

/// **A build carrying no GGUF backend carries no detector**, so a contested
/// architecture has no derivation available and refuses rather than picking.
/// Such a build serves no GGUF artifact at all, so nothing that could reach
/// this is admissible for other reasons first.
#[cfg(not(feature = "gguf"))]
fn render_probe(_template: &str) -> Option<String> {
    None
}

/// Look a family up in the compile-time table.
///
/// **No silent substitution.** A miss is a refusal naming the family, never a
/// nearest match and never a default declaration.
///
/// **A contested architecture refuses here rather than answering by position.**
/// Where more than one entry declares the name, this function cannot choose
/// between them and returning the first would make the table's order the
/// selector. [`select`] is the door that can choose, because it holds the
/// artifact's template as well as its architecture.
pub fn lookup(name: &FamilyName) -> Result<&'static Declaration, FamilyRefusal> {
    let mut found = None;
    for declaration in REGISTRY {
        if declaration.family == name.0 {
            if found.is_some() {
                return Err(FamilyRefusal::ArchitectureContested(name.clone()));
            }
            found = Some(declaration);
        }
    }
    found.ok_or_else(|| FamilyRefusal::UnknownFamily(name.clone()))
}

/// Select the entry this artifact belongs to, from its architecture and, where
/// the architecture is contested, the marker set it renders.
///
/// **Where one architecture carries one entry the key is unchanged and nothing
/// is rendered**, per Spec section 5. That is every family this binary carried
/// before the act that added this function, and it is what keeps a family the
/// detector does not recognise admissible: gemma4's template returns an error
/// rather than a rendering, so an unconditional render would refuse a family
/// the registry serves.
///
/// **Where it is contested the artifact's own rendering decides**, and a
/// derivation that produces no set refuses rather than falling back to the
/// architecture, because falling back is the silent substitution this reached
/// for the template to prevent.
pub fn select(
    name: &FamilyName,
    chat_template: Option<&str>,
) -> Result<&'static Declaration, FamilyRefusal> {
    let candidates: Vec<&'static Declaration> = REGISTRY
        .iter()
        .filter(|declaration| declaration.family == name.0)
        .collect();
    match candidates.as_slice() {
        [] => return Err(FamilyRefusal::UnknownFamily(name.clone())),
        [only] => return Ok(only),
        _ => {}
    }

    let Some(template) = chat_template else {
        return Err(FamilyRefusal::TemplateAbsent(name.clone()));
    };
    let Some(rendered) = render_probe(template) else {
        return Err(FamilyRefusal::TemplateUnrecognised(name.clone()));
    };

    let mut matched = candidates.iter().filter(|declaration| {
        declaration
            .selecting_markers
            .iter()
            .all(|marker| rendered.contains(marker))
    });
    match (matched.next(), matched.next()) {
        (Some(one), None) => Ok(one),
        (Some(_), Some(_)) => Err(FamilyRefusal::MarkersAmbiguous(name.clone())),
        (None, _) => Err(FamilyRefusal::MarkersMatchNoEntry(name.clone())),
    }
}

/// Judge a requested shard width against what the selected entry declares.
///
/// The width condition reads nothing, which is why Spec section 3 judges it
/// before the room and reach conditions that each cost a driver query.
///
/// **It takes the entry rather than the architecture, because an architecture
/// is not always enough to find one.** Where two entries share an
/// architecture, resolving it needs the artifact's template, and the caller
/// has already done that: admit selects once and judges width against what it
/// selected, so the width answer and everything after it read one declaration
/// rather than two resolutions that could disagree.
pub fn judge_width(declaration: &Declaration, requested: u32) -> Result<(), FamilyRefusal> {
    if declaration.shards_across(requested) {
        return Ok(());
    }
    Err(FamilyRefusal::WidthNotDeclared {
        family: FamilyName(declaration.family.to_string()),
        requested,
        declared: declaration.shard_widths,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **A family the binary does not carry is refused, and the refusal names
    /// the family.** What this test reads is the name, not the load's outcome,
    /// so a refusal arriving on some other ground does not satisfy it.
    ///
    /// Perturbation: make `lookup` fall back to `REGISTRY[0]` on a miss and
    /// this test fails, because the lookup then succeeds and no refusal is
    /// produced at all. Watched under exactly that substitution.
    #[test]
    fn an_uncarried_family_refuses_by_name() {
        let absent = FamilyName("not-a-family-this-binary-carries".into());
        let outcome = lookup(&absent);
        // Read as a pattern rather than by equality: [`Declaration`] carries a
        // function pointer and so cannot answer `==` meaningfully. The claim is
        // unchanged, and a lookup that fell back to a nearest match returns
        // `Ok` and fails this the same way.
        assert!(
            matches!(&outcome, Err(FamilyRefusal::UnknownFamily(named)) if named == &absent),
            "the refusal carries the family the header named, got {:?}",
            outcome.as_ref().map(|declaration| declaration.family)
        );
    }

    /// **The width is judged by membership rather than against a bound.**
    ///
    /// Perturbation: change `shards_across` to `width <= max(shard_widths)` and
    /// this test still passes on the registry's contiguous sets, which is why
    /// the non-contiguous case is asserted here as well as pinned by the
    /// module doctest. Under that change the sparse assertion below fails.
    #[test]
    fn a_width_outside_the_declared_set_refuses() {
        // Judged against an entry, because `llama` is a contested
        // architecture and naming it alone reaches no single declaration.
        let entry = select(&FamilyName("qwen2".into()), None).expect("the registry carries qwen2");
        assert_eq!(judge_width(entry, 1), Ok(()));
        assert_eq!(judge_width(entry, 2), Ok(()));
        assert!(matches!(
            judge_width(entry, 3),
            Err(FamilyRefusal::WidthNotDeclared { requested: 3, .. })
        ));

        // The membership property, on a set no maximum describes.
        const SPARSE: Declaration = Declaration {
            family: "sparse",
            shard_widths: &[1, 4],
            template: "{message}",
            generation_opener: "",
            renderer: qwen2::renderer,
            selecting_markers: qwen2::RENDERED_MARKERS,
            flush: FlushMechanism::TruncateToPosition,
            taps_readout: true,
        };
        assert!(SPARSE.shards_across(1));
        assert!(!SPARSE.shards_across(2), "a bound would admit this");
        assert!(!SPARSE.shards_across(3), "a bound would admit this");
        assert!(SPARSE.shards_across(4));
    }

    /// **An emission that opens a call marker and names nothing the parser can
    /// recover answers with that fact rather than with a clean turn.**
    ///
    /// The fixture is a rendered string against each family module, so this
    /// reads the parse's own answer and reaches neither a model nor a device,
    /// per Spec section 10. Every family is exercised, because the claim is
    /// about each family's parser and one passing says nothing about the rest.
    ///
    /// What it reads is two things and both matter: that the unrecovered fact
    /// is reported, and that the fragment did **not** also arrive as assistant
    /// text. The second is the half a collapse into the text path would break
    /// while the first went on looking fine.
    ///
    /// Perturbation: in `scan`, replace the `None` arm's push to `unrecovered`
    /// with `content.push(Content::Text(fragment.to_string()))` and this test
    /// fails, because the fragment then arrives as ordinary assistant text and
    /// no fact is reported. Watched under exactly that collapse.
    #[test]
    fn an_unrecoverable_call_is_reported_rather_than_read_as_prose() {
        let cases: Vec<(&str, Box<dyn Family>, String)> = vec![
            (
                "qwen2",
                Box::new(qwen2::Qwen2),
                format!(
                    "here you go{}not json at all{}",
                    qwen2::CALL_OPEN,
                    qwen2::CALL_CLOSE
                ),
            ),
            (
                "llama",
                Box::new(llama::Llama),
                format!(
                    "here you go{}not json at all{}",
                    llama::CALL_OPEN,
                    llama::CALL_CLOSE
                ),
            ),
            (
                "gpt-oss",
                Box::new(gpt_oss::GptOss),
                format!(
                    "here you go{}no recipient here{}",
                    gpt_oss::CALL_OPEN,
                    gpt_oss::TURN_END
                ),
            ),
        ];

        for (name, family, emission) in cases {
            let parsed = family.parse(&emission);

            assert!(
                parsed.has_unrecovered_call(),
                "{name}: the attempted call is its own reported fact"
            );

            let text = parsed.text();
            assert!(
                !text.contains("not json at all") && !text.contains("no recipient here"),
                "{name}: the fragment must not arrive as assistant text, got {text:?}"
            );
            assert!(
                text.contains("here you go"),
                "{name}: the prose before the marker is still text, got {text:?}"
            );

            // The verbatim emission survives whatever the parse concluded,
            // because the contract has both reaching the record.
            assert_eq!(parsed.verbatim, emission, "{name}: the verbatim is kept");
        }
    }

    /// **Qwen3 is carried, and it is served by the qwen2 module.**
    ///
    /// The architecture is its own, so without an entry the lookup refuses
    /// `UnknownFamily("qwen3")` before any device call, correctly and
    /// uselessly. What this guards is the entry existing: the marker probe of
    /// `tests/markers.rs` verifies the vocabularies agree, and needs the `gguf`
    /// feature to do it, so this side of the claim is asserted where every
    /// build can see it.
    /// **Qwen3.5 and its sparse sibling are carried, and served by the qwen2
    /// module.** Each declares its own architecture, so without an entry the
    /// lookup refuses `UnknownFamily` before any device call, correctly and
    /// uselessly. What this guards is the entries existing.
    ///
    /// The claim they rest on, that the marker vocabularies agree, is
    /// measured by `tests/markers.rs` against a tokenizer of each rather than
    /// inherited from qwen2 or from each other.
    #[test]
    fn the_qwen_family_keys_resolve_through_the_qwen2_module() {
        for family in ["qwen3moe", "qwen35", "qwen35moe"] {
            let declaration = lookup(&FamilyName(family.into()))
                .unwrap_or_else(|_| panic!("{family} is carried"));
            assert_eq!(declaration.template, qwen2::TEMPLATE, "{family}");
            assert_eq!(
                declaration.generation_opener,
                qwen2::GENERATION_OPENER,
                "{family}"
            );
        }
    }

    /// **gemma4 is carried, and it resolves to its own module rather than to
    /// another family's constants.**
    ///
    /// Without an entry the lookup refuses `UnknownFamily("gemma4")` before any
    /// device call, correctly and uselessly. What this guards is the entry
    /// existing and citing the right module: the template and the opener are
    /// gemma4's own, and the renderer the declaration hands out is the one that
    /// knows the word `model`.
    ///
    /// The marker vocabulary it rests on is measured by `tests/markers.rs`
    /// against a gemma4 tokenizer, which needs the `gguf` feature, so this side
    /// of the claim is asserted where every build can see it.
    #[test]
    fn gemma4_resolves_to_its_own_module() {
        let declaration = lookup(&FamilyName("gemma4".into())).expect("gemma4 is carried");
        assert_eq!(declaration.template, gemma4::TEMPLATE);
        assert_eq!(declaration.generation_opener, gemma4::GENERATION_OPENER);
        assert!(
            declaration
                .generation_opener
                .contains(gemma4::CHANNEL_CLOSE),
            "the opener carries the closed thought channel the artifact's own \
             template emits with thinking off"
        );

        // The renderer is this family's, which is what carries the role map and
        // the preamble. Read through a rendering rather than by comparing
        // pointers, a function pointer's address answering nothing.
        let rendered = (declaration.renderer)()
            .render_identity(&[said(Role::Assistant, "hi")])
            .expect("gemma4 renders");
        assert!(
            rendered.starts_with("<bos><|turn>model\n"),
            "the declaration hands out gemma4's own renderer, got {rendered:?}"
        );
    }

    /// **A family can carry the speaker in the turn's shape rather than in a
    /// word, and mistral3 is the first that does.**
    ///
    /// The user's text is wrapped and the model's is bare, so the two roles
    /// render to structurally different strings from the same content. Both
    /// halves are read, because a renderer that wrapped everything or wrapped
    /// nothing would satisfy either assertion alone.
    ///
    /// Perturbation: render the assistant through the user's arm and the
    /// second assertion fails, the model's turn arriving inside an instruction
    /// block the model was trained to read as the user speaking.
    #[test]
    fn a_family_may_carry_the_role_in_the_turns_shape() {
        let user = render_each(mistral3::renderer(), &[said(Role::User, "ping")])
            .expect("mistral3 renders a user turn");
        assert_eq!(user, "[INST]ping[/INST]", "the user's text is wrapped");

        let assistant = render_each(mistral3::renderer(), &[said(Role::Assistant, "pong")])
            .expect("mistral3 renders an assistant turn");
        assert_eq!(
            assistant, "pong</s>",
            "the model's text is bare and closed by the turn's end"
        );
    }

    /// **An empty generation opener is a declaration, not an omission.**
    ///
    /// Every other carried family appends an opener so the generation completes
    /// the assistant's turn rather than electing a speaker. mistral3 appends
    /// nothing because `[/INST]` already did both, and reading the two together
    /// is what says so: the delta ends at the marker that opens the model's
    /// turn.
    ///
    /// Perturbation: give the entry qwen2's opener and the second assertion
    /// fails, a ChatML turn header landing after an instruction block.
    #[test]
    fn the_mistral3_delta_ends_at_the_marker_that_opens_the_model_turn() {
        let declaration = lookup(&FamilyName("mistral3".into())).expect("mistral3 is carried");
        let delta = render_each(mistral3::renderer(), &[said(Role::User, "ping")])
            .expect("mistral3 renders a delta");

        assert!(
            delta.ends_with(mistral3::TURN_CLOSE),
            "the delta closes with the marker that opens the model's turn, got {delta:?}"
        );
        assert_eq!(
            declaration.generation_opener, "",
            "so nothing is appended after it"
        );

        // And the prefix carries the preamble the turns do not repeat, the
        // same joint gemma4 needed for a different reason.
        let prefix = mistral3::renderer()
            .render_identity(&[said(Role::User, "ping")])
            .expect("mistral3 renders a prefix");
        assert_eq!(prefix, "<s>[INST]ping[/INST]");
        assert!(!delta.contains(mistral3::BOS), "and a delta does not");
    }

    /// **One module, one trait object, and three different flushes across the
    /// rows it serves.**
    ///
    /// Six architecture keys cite the qwen2 module, and what they share is a
    /// rendering, not a mechanism: qwen2, qwen3 and qwen3moe truncate, qwen35
    /// and qwen35moe re-establish because they are hybrid, and
    /// `nemotron_h_moe` re-establishes for the same reason while being a
    /// different vendor's model entirely. **This is why the flush is read from
    /// the row rather than from the renderer**, and why `permits_truncation`
    /// could not stay on the trait: `Qwen2::declaration()` answers for qwen2
    /// whichever key was admitted, and here that answer is wrong for three of
    /// the six.
    ///
    /// **The sparse siblings are the pair worth reading twice.** `qwen3moe`
    /// truncates and `qwen35moe` does not, though both are Qwen mixtures of
    /// experts citing one module, because sparsity is not what decides this
    /// and the recurrent layers are.
    ///
    /// Perturbation: read the flush through `qwen2::renderer().declaration()`
    /// instead of through `lookup` and every non-qwen2 assertion below fails.
    #[test]
    fn one_module_serves_keys_whose_flush_mechanisms_differ() {
        let flush_of = |family: &str| {
            lookup(&FamilyName(family.into()))
                .unwrap_or_else(|_| panic!("{family} is carried"))
                .flush
        };

        for truncating in ["qwen2", "qwen3", "qwen3moe"] {
            assert_eq!(
                flush_of(truncating),
                FlushMechanism::TruncateToPosition,
                "{truncating} rolls back by position"
            );
            assert!(
                lookup(&FamilyName(truncating.into()))
                    .unwrap()
                    .permits_truncation(),
                "{truncating}"
            );
        }

        for reestablishing in ["qwen35", "qwen35moe", "nemotron_h_moe"] {
            assert_eq!(
                flush_of(reestablishing),
                FlushMechanism::ReestablishAndReprefill,
                "{reestablishing} carries recurrent state and cannot be partially erased"
            );
            assert!(
                !lookup(&FamilyName(reestablishing.into()))
                    .unwrap()
                    .permits_truncation(),
                "{reestablishing}"
            );
        }

        // All five render through the one module, which is the half that makes
        // the differing flush interesting rather than incidental.
        for shared in [
            "qwen2",
            "qwen3",
            "qwen3moe",
            "qwen35",
            "qwen35moe",
            "nemotron_h_moe",
        ] {
            assert_eq!(
                lookup(&FamilyName(shared.into())).unwrap().template,
                qwen2::TEMPLATE,
                "{shared} renders qwen2's scaffolding"
            );
        }
    }

    /// **One architecture, two formats, and the role tag is the marker.**
    ///
    /// The phi3 pair is the second contested architecture and the first where
    /// the role substitutes into the marker itself, so the tag renderer's
    /// output carries `<|user|>` as a built token rather than a role word
    /// between markers. Both formats are read so the disjointness the
    /// registry's selection rests on is a rendered fact rather than a table
    /// comment.
    ///
    /// Perturbation: render `PhiSep` through `TAG_TEMPLATE` and the second
    /// assertion fails, the separator format then rendering tags. Watched
    /// under exactly that.
    #[test]
    fn the_phi_formats_render_disjoint_markers_from_one_architecture() {
        let turn = [said(Role::User, "hi")];

        let tag = render_each(phi::tag_renderer(), &turn).expect("the tag format renders");
        assert_eq!(tag, "<|user|>hi<|end|>");

        let sep = render_each(phi::sep_renderer(), &turn).expect("the sep format renders");
        assert_eq!(sep, "<|im_start|>user<|im_sep|>hi<|im_end|>");

        // The disjointness selection rests on: no marker of one format
        // appears in the other's rendering.
        for marker in phi::TAG_RENDERED_MARKERS {
            assert!(
                !sep.contains(marker),
                "{marker} leaked into the sep rendering"
            );
        }
        for marker in phi::SEP_RENDERED_MARKERS {
            assert!(
                !tag.contains(marker),
                "{marker} leaked into the tag rendering"
            );
        }

        // And the tool role refuses under both, neither template carrying a
        // tool turn.
        let tool = [said(Role::ToolResult, "42")];
        assert!(render_each(phi::tag_renderer(), &tool).is_err());
        assert!(render_each(phi::sep_renderer(), &tool).is_err());
    }

    /// **Each phi row is reachable by its markers and answers its own
    /// declaration**, which is what keeps `Family::declaration` truthful on
    /// the first module whose architecture a by-name lookup refuses.
    #[test]
    fn the_phi_rows_select_by_their_own_marker_sets() {
        let name = FamilyName("phi3".into());
        assert!(
            matches!(lookup(&name), Err(FamilyRefusal::ArchitectureContested(_))),
            "phi3 is contested, so the by-name door refuses"
        );

        let tag = phi::tag_renderer().declaration();
        assert_eq!(tag.template, phi::TAG_TEMPLATE);
        assert_eq!(tag.generation_opener, phi::TAG_GENERATION_OPENER);

        let sep = phi::sep_renderer().declaration();
        assert_eq!(sep.template, phi::SEP_TEMPLATE);
        assert_eq!(sep.generation_opener, phi::SEP_GENERATION_OPENER);
    }

    #[test]
    fn qwen3_resolves_through_the_qwen2_module() {
        let declaration = lookup(&FamilyName("qwen3".into())).expect("qwen3 is carried");
        assert_eq!(
            declaration.template,
            qwen2::TEMPLATE,
            "the two keys share one module's template rather than a copy of it"
        );
    }

    /// One text message, for the rendering tests below.
    fn said(role: Role, text: &str) -> Message {
        Message {
            role,
            content: vec![ContentBlock::Text {
                text: text.to_string(),
            }],
        }
    }

    /// **A family's wire role is the family's own, and gemma4's differs.**
    ///
    /// The floor's `Role::Assistant` renders as `model` here and as `assistant`
    /// under a family that calls the shared name kernel. Both halves are read,
    /// because the claim is a difference and a test of one side alone would
    /// pass under a shared map that had simply been renamed.
    ///
    /// Perturbation: give `Gemma4::render_delta` the shared
    /// [`common_role_name`] and the gemma4 half fails, the turn then opening
    /// `<|turn>assistant` against an artifact whose own template never writes
    /// that word.
    #[test]
    fn the_wire_role_is_the_familys_own() {
        let turn = [said(Role::Assistant, "hello")];

        let rendered = render_each(gemma4::renderer(), &turn).expect("gemma4 renders an assistant");
        assert!(
            rendered.starts_with("<|turn>model\n"),
            "gemma4 calls the assistant `model`, got {rendered:?}"
        );

        let rendered = render_each(qwen2::renderer(), &turn).expect("qwen2 renders an assistant");
        assert!(
            rendered.starts_with("<|im_start|>assistant\n"),
            "the canonical name is unchanged where the family shares it, got {rendered:?}"
        );
    }

    /// **A family's identity prefix carries its preamble once, and no delta
    /// carries it at all.**
    ///
    /// This is the distinction the surface exists for. The prefix opens a
    /// session and every later turn appends to it, so a preamble repeated per
    /// turn is a control token in the middle of a conversation, and a preamble
    /// missing from the prefix is a model reading its first turn without the
    /// token it was trained to start from.
    ///
    /// Both carried preambles are read, gemma4's `<bos>` and llama's
    /// `<|begin_of_text|>`, and the count is asserted rather than the presence:
    /// two messages through the prefix must still yield one.
    ///
    /// Perturbation: move the preamble into the family's `TEMPLATE` and this
    /// fails twice over - the prefix carries two and the delta carries one.
    /// Watched under exactly that move.
    #[test]
    fn the_preamble_belongs_to_the_prefix_and_appears_once() {
        let cases: [(&str, &'static dyn Family, &str); 2] = [
            ("gemma4", gemma4::renderer(), gemma4::BOS),
            ("llama", llama::renderer(), llama::TEXT_BEGIN),
        ];
        let turns = [said(Role::User, "one"), said(Role::Assistant, "two")];

        for (name, family, preamble) in cases {
            let prefix = family
                .render_identity(&turns)
                .unwrap_or_else(|_| panic!("{name} renders an identity prefix"));
            assert_eq!(
                prefix.matches(preamble).count(),
                1,
                "{name}: the prefix carries its preamble exactly once, got {prefix:?}"
            );
            assert!(
                prefix.starts_with(preamble),
                "{name}: and it opens with it, got {prefix:?}"
            );

            let delta = render_each(family, &turns[1..])
                .unwrap_or_else(|_| panic!("{name} renders a delta"));
            assert!(
                !delta.contains(preamble),
                "{name}: a delta appends to an open session and must not repeat it, got {delta:?}"
            );
        }
    }

    /// **The template renders in one pass, so neither field can be substituted
    /// into.** Successive `replace` calls protect only whichever field is
    /// substituted last: the first call's output is still in the string when
    /// the second one runs, so a placeholder carried by the earlier field is
    /// rewritten. A single pass has no second call to be caught by.
    ///
    /// The role case is the one that discriminates, and the message case is
    /// kept beside it to say why. Under
    /// `replace("{role}", role).replace("{message}", message)` a message
    /// carrying `{role}` comes back **intact**, the role pass having already
    /// run, so asserting only that would be a test the perturbation cannot
    /// fail. Watched: that implementation leaves the message case passing and
    /// fails the role case.
    #[test]
    fn neither_field_is_substituted_into() {
        // The discriminating half: the field substituted first carries the
        // other's placeholder.
        assert_eq!(
            render_template("[{role}]{message}", "weird{message}role", "BODY"),
            "[weird{message}role]BODY",
            "the role's placeholder is not filled by the message"
        );

        // The other direction, which successive replaces also survive. Kept so
        // a later reader does not mistake it for what buys the claim.
        assert_eq!(
            render_template("[{role}]{message}", "assistant", "the user typed {role}"),
            "[assistant]the user typed {role}"
        );
    }

    /// A brace that opens no placeholder renders unharmed, which is the branch
    /// the walk needs so a template carrying JSON is not silently eaten.
    #[test]
    fn a_literal_brace_survives_the_render() {
        assert_eq!(
            render_template("{{\"a\":1}} {role}", "user", "ignored"),
            "{{\"a\":1}} user"
        );
    }

    /// A call the parser **can** recover comes back as a call, which is what
    /// makes the test above a distinction rather than a parser that reports
    /// everything as unrecoverable.
    #[test]
    fn a_recoverable_call_comes_back_whole() {
        let emission = format!(
            "{}{{\"name\":\"read_file\",\"arguments\":{{\"path\":\"/tmp/x\"}}}}{}",
            qwen2::CALL_OPEN,
            qwen2::CALL_CLOSE
        );
        let parsed = qwen2::Qwen2.parse(&emission);
        assert!(!parsed.has_unrecovered_call(), "the name came back");
        assert!(matches!(
            parsed.content.as_slice(),
            [Content::Call { name, .. }] if name.0 == "read_file"
        ));
    }
}
