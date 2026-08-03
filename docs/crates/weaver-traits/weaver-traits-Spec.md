# weaver-traits - Spec

**Status:** DRAFT. The first Spec of phase one's Spec pass, cut 2026-08-01 against
the merged charter. No code is written against it until phase three is ratified,
per Working Process section 6.

**Date filed:** 2026-08-01
**Revised:** 2026-08-01, on the review seat's return. The message model's licensed
combinations are stated and its refusing party named, the enum representation is
elected, the absence pins reclassify to compile-fail tests with the instrument
named, the tool block cites the charter rather than the untracked working list,
the `Send` bound joins the inherited constraints, and two citations and two
wordings are corrected. Revised again the same day, on the second return: the
tagging election becomes a stated mechanical test shared with
`weaver-types-Spec`, `Role` drops its tag as a fieldless enum, and two
instrument overclaims in section 7 are corrected.
**Revised:** 2026-08-03, the assertion pass, second of the seven and the second
of the two floor crates, both taken by hand rather than in the fan-out. Twenty-one
assertion records land at the clauses that argue them, nine from section 7's
enforcement sorting and twelve from the elections outside it. Section 0's
boilerplate records what this document now sources and names its one exception,
the shared tagging test whose node and both edges live at `weaver-types-Spec`
section 4.3, which is the mirroring clause that Spec settled from its own side.
Section 7 states where the records sit and which of its bullets another crate
declares.
**Document ID:** `weaver-traits-Spec`
**Parent:** `weaver-traits-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

Build instructions for `weaver-traits`: the module layout, the item signatures, the
dependency set, and the elections a builder would otherwise have to invent. It is
derived from `weaver-traits-PRD` and from every contract that draws this crate,
which today is `weaver-harness-trace-contract` drawing the message model and
nothing else.

Level discipline, stated once. The charter says what the crate holds and why. This
document says how it is represented, and it is the only kind of document in this
corpus permitted to name a Rust item. Where this document and the charter disagree
the charter yields nothing and this document is corrected.

**This document declares its crate's assertion records, less one edge stated
elsewhere,** per Document Format sections 3 and 4 as of the notation of
2026-08-03. The charter stays the source of this
crate's node and its four vocabulary definitions, and a Spec that restated them
would give the mapper two sources for one record, per Document Format section 1.
The exception is the tagging test this crate shares with `weaver-types`: one
claim is one node with an `asserts` edge per bound crate, the node lives at the
shared statement, and `weaver-types-Spec` section 4.3 declares both edges
including this crate's. Declaring it again here would be the duplicate that
format forbids, and dropping it silently would leave part of this crate's
assertion set with nothing recording where it went.

**What this Spec can settle is bounded by what is chartered, and the bound is not
a gap.** Two of the four definitions are fully specifiable today because the
harness and the trace pair demand them now. The other two are named, placed, and
deliberately left unshaped: `tool-trait` is blocked by the charter's own section
3.1, and `provider-trait` carries the decode seam's shapes, which
`weaver-spu-PRD` section 8 defers to the token workflow. Specifying either would
be the anticipatory contract charter section 4 forbids, written in Rust rather
than in prose.

## 1. The crate

**Layout.** One module per definition, re-exported at the root, so a consumer
writes `weaver_traits::Message` rather than tracking module paths that exist for
the author's convenience.

    src/lib.rs          re-exports, and nothing else
    src/message.rs      the message model, section 3
    src/permission.rs   the permission modes, section 4
    src/tool.rs         the tool contract, section 5, blocked
    src/provider.rs     the provider contract, section 6, deferred

**Edition and toolchain.** Edition 2024 on the workspace's pinned nightly, per the
workspace `rust-toolchain.toml`. This crate uses no nightly feature and would
compile on stable, which is worth keeping true: the floor is the one crate every
other links, and a nightly feature here is a nightly requirement everywhere.

```graph
node: traits-compiles-on-stable
kind: assertion
tag: review

edge: asserts
from: weaver-traits
to: traits-compiles-on-stable
```


**The dependency set is one crate and it is argued.** `serde` with the `derive`
feature, because the message model crosses the trace seam as payload content and
something must render it. `serde` alone, with no format crate: the recorder
renders an admitted event to canonical bytes once, per `weaver-trace-PRD` section
4.2, and the payloads carrying this model are opaque octets to that recorder, per
that charter's section 3, so the format is decided where the rendering happens
rather than where the shape is defined. A format crate here would move that
decision onto the floor. Nothing else is admitted, and specifically **no async
runtime, no `async-trait`, no `futures`**, per section 5's boxing election and the
charter's prohibition on anything that does work.

```graph
node: traits-no-format-crate
kind: assertion
tag: manifest

edge: asserts
from: weaver-traits
to: traits-no-format-crate

node: traits-no-async-runtime
kind: assertion
tag: manifest

edge: asserts
from: weaver-traits
to: traits-no-async-runtime
```


**No internal dependency, and the manifest is the check.** The crate's
`Cargo.toml` names no `weaver-*` dependency, which gate H2 reads against the
graph: this crate declares no `floor-link` and no `seam`, so any Cargo edge at all
is a defect.

```graph
node: traits-no-internal-dependency
kind: assertion
tag: manifest

edge: asserts
from: weaver-traits
to: traits-no-internal-dependency
```


## 2. Representation posture, stated once

**Every type here is data, derives what data derives, and owns no invariant it
cannot state in its own shape or name a party to enforce.** Concretely: `Debug`,
`Clone`, `PartialEq`, and `serde::Serialize` with `Deserialize` on the message
model and the permission modes, and no `Default` anywhere. A default is a decision
about what an absent value means, and `weaver-types-PRD` section 5 rules that
absence is never read as a default unless a charter says so and says what it
means. A `Default` impl on a floor type is that ruling defeated by a derive.

```graph
node: traits-data-derive-set
kind: assertion
tag: review

edge: asserts
from: weaver-traits
to: traits-data-derive-set

node: traits-no-default
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-traits
to: traits-no-default
```


**Enumerations are non-exhaustive where the charter says the set grows, and
exhaustive where the charter says it is closed.** The permission modes are a
closed operator-facing set, so their enum is exhaustive and a consumer's match is
checked. Message roles and content blocks grow with the conversation model, so
those enums carry `#[non_exhaustive]` and consumers keep a wildcard arm. The
attribute is elected per type against that test rather than applied as a house
style, because it buys forward compatibility at the cost of exactly the
compile-time loudness that makes a closed set worth closing.

```graph
node: traits-non-exhaustive-per-charter
kind: assertion
tag: review

edge: asserts
from: weaver-traits
to: traits-non-exhaustive-per-charter
```


## 3. The message model

Provider-agnostic conversation content, per charter section 3.3. It is the one
definition in this crate a merged contract draws today, and its shape is fixed by
what two consumers need: the harness assembles prompts from it, and the trace
records `message.user`, `message.assistant`, and `message.tool_result` payloads
carrying it, opaque to the recorder per `weaver-trace-PRD` section 3.

```rust
pub struct Message {
    pub role: Role,
    pub content: Vec<ContentBlock>,
}

#[non_exhaustive]
pub enum Role {
    User,
    Assistant,
    ToolResult,
}

#[non_exhaustive]
pub enum ContentBlock {
    Text { text: String },
    ToolCall(ToolCall),
    ToolResult(ToolResultBlock),
}
```

**The role set is three because the closed event-kind set is three.**
`weaver-trace-PRD` section 3.1 carries `message.user`, `message.assistant`, and
`message.tool_result`, and the harness authors one kind per message, so a role
mapping to no kind or to two would put a judgment on the authoring path that the
kind set has already made. The mapping is one to one and total.

```graph
node: traits-role-set-three
kind: assertion
tag: review

edge: asserts
from: weaver-traits
to: traits-role-set-three
```


**The licensed combinations are stated, because the shape cannot state them.** A
`User` message carries `Text` blocks. An `Assistant` message carries `Text` and
`ToolCall` blocks, in the order the model emitted them. A `ToolResult` message
carries `ToolResult` blocks. Every other pairing is unlicensed: an `Assistant`
message holding a `ToolResult` block, a `User` message holding a `ToolCall`, a
`ToolResult` message holding prose.

**The harness refuses an unlicensed message and the recorder does not.** The
harness is the sole author and the only party that reads a message as a message,
per `weaver-harness-trace-contract`'s vocabulary clause, and a malformed
conversation message is a defect it catches before submitting, per
`weaver-trace-PRD` section 3. The recorder judges the envelope binding and the
octet well-formedness and never the interior, so a rule it cannot see is a rule it
must not be given. This is a behavior rather than a type property, so it takes the
perturbation-verified test of section 7 rather than a pin.

**Content is a sequence of blocks rather than a string, because an assistant turn
is not one.** An assistant turn carries prose and tool calls in one emission, and
a string type would force the harness to re-parse its own model's output to find
the call, which is the shape the previous tree's emitters got wrong. The block
enum is where a turn's parts stay distinguishable from authoring through to the
trace.

```graph
node: traits-content-is-block-sequence
kind: assertion
tag: review

edge: asserts
from: weaver-traits
to: traits-content-is-block-sequence
```

**`ToolCall` and `ToolResultBlock` are named here and shaped with the tool
workflow.** They are the conversation's view of a tool interaction, which is the
message model's business, but their field lists follow the tool protocol that
section 5 holds blocked. This Spec fixes that they exist as blocks and defers what
they carry, which is the half-chartered discipline the SPU and gate charters
already run on.

**The tagging election follows one mechanical test, stated here and applied
identically in `weaver-types-Spec` so the two floor Specs cannot drift.** A
fieldless enum serializes as a plain renamed string. An enum whose every variant
is struct-shaped or wraps a struct is internally tagged. An enum with any variant
wrapping a primitive, a sequence, or another tagged enum is adjacently tagged,
because internal tagging cannot represent those shapes and fails at
serialization time rather than at compile time.

`Role` is fieldless, so it takes `#[serde(rename_all = "snake_case")]` and no tag,
serializing as `"user"`. Tagging it would yield `{"type": "user"}`, a nesting
level that buys nothing and costs a level in every recorded message.

```graph
node: traits-role-plain-string
kind: assertion
tag: review

edge: asserts
from: weaver-traits
to: traits-role-plain-string
```

`ContentBlock` passes the second case, `Text` being struct-shaped and the other
two wrapping structs, so it takes `#[serde(tag = "type", rename_all =
"snake_case")]` and a block reads as `{"type": "text", "text": "..."}` rather
than as `{"Text": "..."}`. The election is about the reader: these payloads reach
the operator's tooling through the stream, where a non-Rust consumer keys on a
stable member name, and serde's default external tagging makes the variant name a
key, which is the shape that breaks silently when a variant is renamed.

**`ContentBlock::Text` takes a struct variant rather than a newtype** for the
mechanical reason the test states: internal tagging cannot represent a newtype
variant wrapping a primitive. No variant of either enum wraps a bare primitive,
which is what keeps the second case available here.

```graph
node: traits-content-block-internally-tagged
kind: assertion
tag: review

edge: asserts
from: weaver-traits
to: traits-content-block-internally-tagged
```


**A tool result appears in two positions and the redundancy is only apparent.**
`Role::ToolResult` says which of the three event kinds the message becomes, and
`ContentBlock::ToolResult` carries the result's content. A reader seeing
`tool_result` in both places is reading a kind and a block tag, and the licensed
combinations above are what make the pairing checkable rather than coincidental.

## 4. Permission modes

The operator-facing policy vocabulary of charter section 3.4, closed and
exhaustive.

```rust
pub enum PermissionMode {
    Ask,
    Allow,
    Deny,
}
```

**Three modes and no fourth, because the question has three answers.** Ask the
operator before a class of action, permit it without asking, refuse it. A fourth
mode would be a policy this program does not hold, and the enum is exhaustive so
a fourth cannot arrive without every consumer's match failing to compile, which is
the loudness charter section 5 asks for.

```graph
node: traits-permission-mode-exhaustive
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-traits
to: traits-permission-mode-exhaustive
```


**The type carries no method that adjudicates anything:** no inherent method and
no trait implementation beyond the data derives of section 2. No `is_allowed`, no
`check`, no predicate of any kind. The mode is what the operator declared and the
harness reads it, and a helper here would be the first step toward a floor type
that decides, which charter section 4 forbids. The absence is enforced by the
compile-fail test of section 7, because an absence is what a runtime test
structurally cannot demonstrate.

**The relationship to the agent config runs one way.** `weaver-types-PRD` section
2.1 defines a `permission-mode` field that elects one of these, and that charter
declares the `elects` edge. This crate defines the vocabulary and learns nothing
about which mode any agent chose.

## 5. The tool contract, blocked

`tool-trait` is blocked rather than open, per `weaver-traits-PRD` section 3.1 and
the G4 terms it states: tool dispatch is harness-internal, no seam crosses it, so
no vocabulary clause draws the definition and it cannot fail before phase close.
This Spec obeys that. `src/tool.rs` exists as a placement and this section states
only what the eventual shape must satisfy, so that the workflow settling it
inherits the constraints rather than rediscovering them.

**Four constraints the tool workflow inherits.** The trait is dyn-compatible,
because `weaver-traits-PRD` section 3.1 has the engine dispatching tools it does
not know the identity of and that is object dispatch by definition. Its async
methods return an explicitly boxed future rather than using `async fn` in trait,
because an `async fn` in a trait is not dyn-compatible and the alternative,
`async-trait`, is a proc-macro dependency on the floor that the boxing writes out
by hand in three lines. **The boxed future carries `+ Send`**, because the harness
drives tool calls on a runtime this crate does not name and a future that cannot
cross a thread would bound the executor the composition root may choose, which is
a transport decision leaking onto the floor. And the trait carries **no safety
classification of any kind**, per charter section 3.1 and apex section 3 step 7,
which the tool workflow may not weaken.

```graph
node: traits-tool-dyn-compatible
kind: assertion
tag: review

edge: asserts
from: weaver-traits
to: traits-tool-dyn-compatible

node: traits-tool-boxed-future-send
kind: assertion
tag: review

edge: asserts
from: weaver-traits
to: traits-tool-boxed-future-send

node: traits-tool-no-safety-classification
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-traits
to: traits-tool-no-safety-classification
```


**The tool taxonomy of `weaver-tools-vision` section 6 reaches this trait when the
workflow charters it.** An internal tool is an organ function the harness
dispatches inside the body and an external tool is world engagement crossing the
gate, and whether one trait serves both classes or the classes take separate
shapes is that workflow's question.

## 6. The provider contract, deferred

`Provider` is the abstraction that keeps the engine transport-agnostic, per
charter section 3.2, and its vocabulary is the request, the streamed response, and
the error. Every one of those three is decode-seam material, which
`weaver-spu-PRD` section 8 defers to the token workflow by name, so this Spec
fixes the trait's role and defers its signature.

**Three constraints the token workflow inherits.** The trait is dyn-compatible for
the same reason the tool trait is: the concrete transport is constructed at the
worker composition root and injected, per charter section 3.2, and injection
against a trait object is what lets the composition root be the only place a wire
format is named. Its futures carry `+ Send`, for the reason section 5 gives. And
the streamed response is expressed without a `futures` dependency on the floor,
the candidate being a boxed future yielding a receiver the caller drains, which
keeps the floor free of an async ecosystem crate that everything above would then
carry. If the token workflow finds that shape costs measurable latency on the
decode path, the dependency is a decision that pass makes with the measurement in
hand, which is the only ground on which the floor takes a second dependency.

```graph
node: traits-provider-dyn-compatible
kind: assertion
tag: review

edge: asserts
from: weaver-traits
to: traits-provider-dyn-compatible

node: traits-provider-no-futures-dep
kind: assertion
tag: review

edge: asserts
from: weaver-traits
to: traits-provider-no-futures-dep
```


## 7. What is enforced, and by which instrument

Per apex section 11, an invariant that is a type property takes a compile-time pin
and an invariant that is a behavior takes a perturbation-verified test. Naming the
instrument matters as much as naming the property, because the compiler enforces
what it errors on and it errors on nothing for a thing that was never written.

**Enforced by the compiler, by construction.**

- The permission-mode enum is exhaustive, so a fourth mode breaks every consumer's
  match at compile time rather than at run time.
- The message model's serde representation comes from derives rather than
  hand-written implementations, so a field added to a struct cannot be silently
  dropped from the wire.

**Enforced by compile-fail tests, because the property is an absence.** Each is a
`compile_fail` doctest asserting that the offending code does not build, which is
how an absence becomes mechanical rather than remembered.

- No `Default` on any type here: a doctest requiring `Message: Default` fails to
  compile. The day someone adds the derive that code starts compiling and the
  `compile_fail` test starts failing, which is the instrument firing.
- No adjudicating method on `PermissionMode`: doctests naming the tempting
  candidates, `is_allowed` and `check`, fail to compile. A finite set of doctests
  pins the named methods and not the open set of all possible methods, so the
  general prohibition stays review's, which is the same honest split this section
  makes for the manifest rules below.
- No safety classification on the eventual tool trait, when that trait exists.

**Enforced by the manifest, with the instrument named rather than assumed.** The
no-internal-dependency rule is gate H2, which reads Cargo edges against the graph.
The no-async-runtime rule is **not** H2, which checks `weaver-*` edges only: it
takes a build-time assertion over the resolved external tree, a `cargo tree` check
in the workspace's own automation, and naming it here is what keeps it from being
a claim nothing runs.

**Where the records sit, and the one claim another crate declares.** The
assertion records are at the clauses that argue the claims, across sections 1
through 6, rather than gathered here, per Document Format section 6. Three
claims are argued only in this section and carry their records at the end of
it. **The licensed combinations of section 3 are declared by `weaver-harness`,**
whose Spec section 8 carries the test and has discharged the owing, because an
assertion belongs where its test lives. **The tagging test shared with
`weaver-types` is declared there,** node and both edges, per that Spec's section
4.3 and the pilot's rule: one claim is one node, and this crate's edge is
already stated beside the shared statement rather than restated here.

**Requiring a perturbation-verified test.**

- The licensed combinations of section 3: the harness refuses an `Assistant`
  message carrying a `ToolResult` block, confirmed by watching the message reach
  the recorder when the check is removed. This test lives in the harness's suite,
  because the harness is the party the rule binds, and it is named here because
  the rule is stated here.
- A message round-trips through serialization unchanged, block sequence and roles
  intact, confirmed by watching the test fail when a block variant is dropped.
- An unknown tag on deserialization refuses rather than defaulting, confirmed by
  feeding a role this crate does not define.

**This crate has no threat walk of its own, and the absence is stated rather than
left blank.** The security mechanisms this program relies on live where processes
and descriptors do, which is not here. The rule that every security mechanism's
Spec names its adversary and derives its test from the attack applies to the
crates that hold them, and a floor of definitions has no adversary beyond a
consumer that ignores the vocabulary, which the compiler handles.

```graph
node: traits-serde-from-derives
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-traits
to: traits-serde-from-derives

node: traits-message-round-trip
kind: assertion
tag: perturbation

edge: asserts
from: weaver-traits
to: traits-message-round-trip

node: traits-unknown-tag-refuses
kind: assertion
tag: perturbation

edge: asserts
from: weaver-traits
to: traits-unknown-tag-refuses

node: traits-no-adjudicating-method
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-traits
to: traits-no-adjudicating-method
```

## 8. Open elections

Each names what settles it, and none is this Spec's to settle alone.

- **The tool trait's shape.** Blocked, per section 5. Settled by the tool
  workflow's charter pass, against the taxonomy the vision holds.
- **The provider trait's signature, and whether the floor takes a second
  dependency.** Deferred, per section 6. Settled by the token workflow with a
  latency measurement on the decode path.
- **The `ToolCall` and `ToolResultBlock` field lists.** Deferred with the tool
  protocol, per section 3.
