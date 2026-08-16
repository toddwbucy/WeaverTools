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
//!     flush: weaver_spu::decoder::backend::FlushMechanism::TruncateToPosition,
//!     taps_readout: true,
//! };
//! assert!(SPARSE.shards_across(4));
//! assert!(!SPARSE.shards_across(2));
//! assert!(!SPARSE.shards_across(3));
//! ```

use weaver_types::{LifecycleRefusal, ToolName};

use crate::decoder::backend::FlushMechanism;

pub mod gpt_oss;
pub mod llama;
pub mod qwen2;

/// One canonical message, as the harness holds it.
///
/// The floor carries no message type yet, the decode contract's payload shapes
/// being deferred, so the shape lives here and travels no seam. It is what the
/// renderers read and nothing else consumes it, which is what keeps it from
/// being a slot reserved against a reader that does not exist.
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub role: String,
    pub content: String,
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
pub trait Family {
    /// Render an identity prefix from canonical messages.
    fn render_identity(&self, messages: &[Message]) -> String;

    /// Render a turn's delta.
    fn render_delta(&self, message: &Message) -> String;

    /// Parse an emission into canonical content, this family's markers
    /// recognised.
    fn parse(&self, emission: &str) -> Parsed;

    /// The stop conditions this family declares.
    fn stop_conditions(&self) -> &'static [&'static str];

    /// Whether the session's state permits truncation.
    fn permits_truncation(&self) -> bool;

    /// The capabilities admission judges against.
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
#[derive(Debug, Clone, Copy, PartialEq)]
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
        flush: FlushMechanism::TruncateToPosition,
        taps_readout: false,
    },
    Declaration {
        family: "qwen2",
        shard_widths: &[1, 2],
        template: qwen2::TEMPLATE,
        generation_opener: qwen2::GENERATION_OPENER,
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
        family: "qwen35",
        shard_widths: &[1, 2],
        template: qwen2::TEMPLATE,
        generation_opener: qwen2::GENERATION_OPENER,
        flush: FlushMechanism::TruncateToPosition,
        taps_readout: false,
    },
    Declaration {
        // The sparse sibling declares its own architecture again, so it is its
        // own key and its own probe. Two keys citing one module is a claim
        // about their markers agreeing, and this one is measured rather than
        // inherited from the dense entry above.
        family: "qwen35moe",
        shard_widths: &[1, 2],
        template: qwen2::TEMPLATE,
        generation_opener: qwen2::GENERATION_OPENER,
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
            FamilyRefusal::UnknownFamily(_) => LifecycleRefusal::ArtifactUnreadable,
            FamilyRefusal::WidthNotDeclared { .. } => LifecycleRefusal::DeviceCannotAdmit,
        }
    }
}

/// Look a family up in the compile-time table.
///
/// **No silent substitution.** A miss is a refusal naming the family, never a
/// nearest match and never a default declaration.
pub fn lookup(name: &FamilyName) -> Result<&'static Declaration, FamilyRefusal> {
    REGISTRY
        .iter()
        .find(|declaration| declaration.family == name.0)
        .ok_or_else(|| FamilyRefusal::UnknownFamily(name.clone()))
}

/// Judge a requested shard width against what the family declares.
///
/// The width condition reads nothing, which is why Spec section 3 judges it
/// before the room and reach conditions that each cost a driver query.
pub fn judge_width(name: &FamilyName, requested: u32) -> Result<(), FamilyRefusal> {
    let declaration = lookup(name)?;
    if declaration.shards_across(requested) {
        return Ok(());
    }
    Err(FamilyRefusal::WidthNotDeclared {
        family: name.clone(),
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
        assert_eq!(
            lookup(&absent),
            Err(FamilyRefusal::UnknownFamily(absent.clone())),
            "the refusal carries the family the header named"
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
        let llama = FamilyName("llama".into());
        assert_eq!(judge_width(&llama, 1), Ok(()));
        assert_eq!(judge_width(&llama, 2), Ok(()));
        assert!(matches!(
            judge_width(&llama, 3),
            Err(FamilyRefusal::WidthNotDeclared { requested: 3, .. })
        ));

        // The membership property, on a set no maximum describes.
        const SPARSE: Declaration = Declaration {
            family: "sparse",
            shard_widths: &[1, 4],
            template: "{message}",
            generation_opener: "",
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
                declaration.generation_opener, qwen2::GENERATION_OPENER,
                "{family}"
            );
        }
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
