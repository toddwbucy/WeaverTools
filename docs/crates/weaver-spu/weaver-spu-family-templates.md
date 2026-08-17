# Model families and their chat templates, in the SPU

**DRAFT, 2026-08-17. Carries no decisions, per Working Process section 2.**
Drafted against the repository - the registry, the family modules, the PR
history and the issues - rather than against a prior document. It may not
contradict `weaver-spu-PRD` or `weaver-spu-Spec`, and where the code's family
representation has no counterpart in those, the gap is returned at the end
rather than settled here.

Reconciled against `2359e6f`, after issue #88 closed and the phi pair
landed. Written for a reader
coming to the SPU cold. Out of scope: tokenizer behaviour beyond template
rendering, the decode path and its latency, and any family not yet in the code.

## The question this answers

A prompt is not a conversation. A conversation is a list of messages with
roles; a prompt is one string of tokens, and every model vendor has its own
opinion about how the second is built from the first. The SPU holds a family
abstraction because that opinion differs per model, differs in more ways than a
reader expects, and because getting it wrong is quiet: a model handed the wrong
markers does not fault, it answers slightly worse, and the trace records a
clean turn.

The evidence is the accumulated divergence below. **Thirteen registry
entries across eleven architecture strings, served by six modules, descending
from six template ancestors.** Those three numbers being different from each
other is the whole subject.

## The two workflows

**Template resolution: family identified through to prompt rendered.** The
artifact's header declares `general.architecture`. `family::select` filters
`REGISTRY` on that string:

- **No entry** - refused, `UnknownFamily`, naming the architecture.
- **One entry** - that entry, and **nothing is rendered**. This is the path
  every family took before #178 and it is still the common path.
- **Several entries** - the architecture is *contested*, and the artifact's own
  `tokenizer.chat_template` decides. **The template is required before anything
  is rendered:** an artifact declaring none refuses `TemplateAbsent` at that
  point. Otherwise a frozen three-turn probe conversation is rendered through
  the template, and the entry whose `selecting_markers` all appear in the
  output wins. A derivation that produces no rendering refuses
  `TemplateUnrecognised` - chiefly the detector not recognising the template,
  and also a build carrying no backend to detect with. An output matching no
  entry refuses `MarkersMatchNoEntry`; one matching more than one refuses
  `MarkersAmbiguous`.

Once an entry is selected, its `renderer` renders: `render_identity` for the
session prefix, the shared `render_each` per turn, then the entry's
`generation_opener` appended to the delta. The result is tokenized and decoded.

**Family registration: what adding a family requires end to end.** Add a probe
entry to `crates/weaver-spu/tests/markers.rs` naming an artifact, and run it
*before writing anything else*. The probe asserts every marker the family
renders tokenizes to exactly one token, because a marker that degrades to
sub-word text is structure the model reads as prose. Then the module constants,
the `Family` impl, the registry entry with its `selecting_markers` and its
`flush`. Two later gates catch what the probe cannot: a hybrid architecture
declaring a truncating flush fails `tests/markers.rs`, and an entry that makes
an architecture contested must have a template the renderer recognises.

## The matrix

Base lineage is the template ancestor the entry descends from. `renders
through` names the module.

| architecture | base lineage | renders through | point of divergence | how the SPU handles it |
|---|---|---|---|---|
| `qwen2` | ChatML | qwen2 | none - the reference | shared `render_template`, shared role names |
| `qwen3` | ChatML | qwen2 | architecture only; markers byte-identical | entry citing qwen2 (#87, #88 consequence A) |
| `qwen3moe` | ChatML | qwen2 | architecture only, sparse sibling | entry citing qwen2 (#169) |
| `qwen35` | ChatML | qwen2 | markers identical, **state is hybrid** | own `flush: ReestablishAndReprefill` (#168, corrected #172) |
| `qwen35moe` | ChatML | qwen2 | as qwen35, sparse | own flush (#168, #172) |
| `nemotron_h_moe` | ChatML | qwen2 | different vendor, same markup, **hybrid** | own flush; entry cites qwen2 (#173) |
| `llama` (1) | Llama 3 headers | llama | prefix opens `<\|begin_of_text\|>` once | own `render_identity` preamble (dead until #171) |
| `llama` (2) | ChatML | qwen2 | **same architecture, different markup** - SmolLM2 | second entry; the architecture is contested and the artifact's rendering selects (#178) |
| `gpt-oss` | Harmony | gpt_oss | the call-close marker **also ends a turn** | own `recover` reading a routing header, not JSON |
| `gemma4` | Gemma 4 | gemma4 | assistant is called `model`; `<bos>` once; channel pair rendered into the opener; asymmetric markers | own role map, own preamble, own opener (#171) |
| `mistral3` | Mistral v7 | mistral3 | **no role word at all**; empty generation opener; `<s>` once | own `render_delta` building each turn by shape (#173) |
| `phi3` (1) | Phi tags | phi | **the role substitutes into the marker itself**, `<|{role}|>` | own role match; template builds the tag (#184) |
| `phi3` (2) | ChatML | phi | separator token where ChatML puts a newline; **same architecture as (1), disjoint markers** | second contested pair; the artifact's rendering selects (#184) |

## What the matrix shows

**The field starts from a small number of ancestors and every vendor edits its
inheritance for its own use case.** Eight of thirteen entries descend from
ChatML, from five vendors who never coordinated - Phi-4 14B editing it with a
separator token where the others keep the newline. That is why the
abstraction is a registry
of declarations citing shared modules rather than one module per entry: the
common case is a new architecture string over old markup, and it should cost
one table row.

The edits fall on five axes, which is the argument for the abstraction being a
small surface rather than one `template` field.

**The role's name.** ChatML, Llama 3 and Harmony substitute a role word into a
fixed shape, so they share `common_role_name` and the single-pass
`render_template`. Gemma 4 renders the assistant as `model` - its own template
performs exactly that substitution - so it writes its own match and no other
family learns the word. Mistral has no role word at all: `[INST]` wraps the
user and the model's turn is bare, so the *shape* carries the speaker and there
is nothing for a placeholder to fill.

**What opens a prefix.** Three families emit a token once, before the first
turn, that no later turn repeats: llama's `<|begin_of_text|>`, gemma4's
`<bos>`, mistral3's `<s>`. This is what makes `render_identity` and
`render_delta` a real distinction rather than two names for one loop. Gemma 4
and mistral3 reach the same need from opposite declarations - gemma4's artifact
sets `add_bos_token: false` and emits it in the template, mistral3's sets
`true` and relies on the tokenizer - and since this crate tokenizes with
`AddBos::Never`, both must render it.

**What closes a delta.** The generation opener leaves the assistant's turn open
so the model completes it rather than electing a speaker. Mistral's opener is
empty, and correctly so: `[/INST]` is emitted by the *user's* turn and already
opens the model's. Gemma 4's carries a pre-closed empty thought channel,
because without it the model dangles and loops the channel marker to the token
cap.

**How state rolls back.** Not a template fact at all, and the axis that proves
the granularity. `qwen2` and `qwen3moe` truncate; `qwen35`, `qwen35moe` and
`nemotron_h_moe` cannot, because their architectures carry recurrent layers
whose state is a running summary rather than a per-position cache. All render
through the qwen2 module. **One module, one trait object, three flush
mechanisms** - which is why the flush is declared per row and read from the row.

**Whether the architecture identifies the format at all.** Two pairs carry
the case now: `llama`, where a Llama 3 artifact and SmolLM2 declare one
architecture and render different markup, and `phi3`, where a single vendor's
own current line does the same - the minis render role tags and Phi-4 14B
renders the separator form, with marker sets so disjoint that each artifact's
vocabulary lacks the other's markers entirely. One architecture, two entries,
and the artifact's own template breaks the tie.

## Shared mechanism, and what was irreducible

Shared, no per-family branch: `render_template`, a single-pass substitution so
a message containing `{role}` cannot be substituted into; `common_role_name`
and `text_content`, called by the families whose wire vocabulary is the
canonical one; `render_each`, which holds no marker, role or preamble; `scan`,
the parse kernel, which owns no marker - every string it matches arrives from
the family; `promote_stop_conditions`, which turns declared stop strings into
token ids against the artifact's vocabulary.

Irreducible, requiring the family's own path: the role map, where the wire name
is not the canonical one (gemma4) or does not exist (mistral3); the prefix
preamble (llama, gemma4, mistral3); and `recover`, the extraction of a call's
name, which differs four ways - qwen2 parses a JSON object, gpt-oss reads a
`to=functions.` routing header because its call-close marker also ends turns,
gemma4 reads `call:<name>{`, mistral3 reads `<name>[ARGS]`.

## The selection mechanism, and what it does not close

**The renderer used for selection is a detector, not an evaluator**, and Spec
section 5 is emphatic about it. `llama_chat_apply_template` does not run the
artifact's template. It substring-matches the template text against a table of
template families it carries, first match winning, and emits *its own*
rendering of whichever it settled on; upstream says it is not a jinja parser.
Three consequences a reader needs:

- **It answers the case anyway, measured.** Phi-4-mini's template builds its
  markers by concatenation and its source carries only two of the four, yet all
  four arrive through this path, because the detector recognised the family and
  knows what that family emits.
- **It fails closed, and one carried family is such a case.** Gemma 4's
  template returns an error rather than a rendering. This is why the render is
  reached *only* where an architecture is contested: an unconditional render
  would refuse a family the registry already serves.
- **The detector's table belongs to the pinned revision**, `llama-cpp-rs` at
  `277e4100`. A bump can move a marker set this crate never edited, which
  reopens the readings rather than inheriting them.

**The residual risk is mis-detection, narrowed rather than closed.** A
first-match heuristic can settle on the wrong family and this crate cannot
audit the judgment. Because selection only ever chooses among entries that
already agree with the artifact about its architecture, a wrong detection
cannot reach a family the artifact never declared. Within a declared
architecture it can still pick the wrong sibling, and that outcome is a silent
substitution. The Spec names this as the act's residual rather than claiming it
closed.

Two smaller ones, both reported rather than corrected at runtime: llama's
`<|eom_id|>` does not promote against the 3.0-era tokenizer this workshop can
reach, so it is named in `StopSet::unpromoted` and excluded rather than
silently carried; and gemma4's model may emit channel blocks the archived tree
found must be stripped before re-feeding, named in the module as not owed until
the harness's delta shape settles.

## Returned to the handoff, not settled here

- **The charter says "one module per family" and the code has eleven entries
  over five modules.** `weaver-spu-PRD` section 14 uses "family" to mean a
  vendor lineage - "Qwen or Gemma or the Harmony speakers". The registry now
  keys on an architecture *and*, where contested, a marker set. Neither the
  charter nor the PRD names the registry entry as a concept distinct from a
  family, and the granularities are now three deep: lineage, architecture,
  entry.
- **The charter has a family declare "what template identity it renders".**
  `Declaration::selecting_markers` is now the closest thing to that identity,
  while `Declaration::template` is a string that since #171 renders nothing and
  is carried for the measurement record. Which of the two the charter's phrase
  means is unsettled, and #129 bears on it.
- **`permits_truncation` is listed in Spec section 5's surface and is no longer
  on the trait.** #172 moved it to `Declaration`, where it can be keyed per
  entry. Argued in that PR; no document act followed.
