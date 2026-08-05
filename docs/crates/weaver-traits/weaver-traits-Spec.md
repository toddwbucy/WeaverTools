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
of the two floor crates, both taken by hand rather than in the fan-out. Twenty-two
assertion records land at the clauses that argue them, eleven from section 7's
enforcement sorting and eleven from the elections outside it. Section 0's
boilerplate records what this document now sources and names its one exception,
the shared tagging test whose node and both edges live at `weaver-types-Spec`
section 4.3, which is the mirroring clause that Spec settled from its own side.
Section 7 states where the records sit and which of its bullets another crate
declares. On the review seat's return the same day, the adjudicating-method
claim divides into the two records its own bullet always described, the pinned
candidates and the prohibition, the latter moving to the section 4 clause that
argues it. On the second return the same day, that clause's own closing sentence
adopts the split it had been moved beside, having given the whole claim to the
instrument the split took from it.
**Revised:** 2026-08-03, the provenance convention, per the operator's ruling of
this date and Document Format v0.10 section 3: the two halves of a divided claim
both count as the enforcement section's, neither having been elected and one
having been divided out of a bullet that section already carried. No record
moves and no tag changes, only the provenance the counts report.
**Revised:** 2026-08-03, the labelling pass, per Document Format v0.11 sections 3
and 4. Six `grounds` edges land beside the records that carry them, all six to
`axiom-floor-is-vocabulary-behavior-is-socket`, the other three axioms taking
nothing from a floor crate that holds no turn key, is not an organ, and settles
no contract's completeness. Section 7 states the test the pass applied, whether
the axiom is the reason the claim exists, states the four edges a reader would
otherwise reconstruct and the two neighbouring absences that take none, and
records that sixteen claims grounding in nothing is the expected result rather
than a gap. No record moves, no tag changes, and the assertion count is unchanged
at twenty-two.
**Revised:** 2026-08-04, the promotion pass, per the operator's election of this
date. Section 2's two claims divide: the derive set takes a compile-pin doctest
and the non-exhaustive attribute a compile-fail pair, both argued in section 7,
their remainders staying review's per the division rule. The count moves to
twenty-four, and the two new records sit at the end of section 7 with the four
argued only there.
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

edge: grounds
from: traits-no-async-runtime
to: axiom-floor-is-vocabulary-behavior-is-socket
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

edge: grounds
from: traits-no-internal-dependency
to: axiom-floor-is-vocabulary-behavior-is-socket
```


## 2. Representation posture, stated once

**Every type here is data, derives what data derives, and owns no invariant it
cannot state in its own shape or name a party to enforce.** Concretely: `Debug`,
`Clone`, `PartialEq`, and `serde::Serialize` with `Deserialize` on the message
model and the permission modes, and no `Default` anywhere. A default is a decision
about what an absent value means, and `weaver-types-PRD` section 5 rules that
absence is never read as a default unless a charter says so and says what it
means. A `Default` impl on a floor type is that ruling defeated by a derive.
**The named set is pinned by a doctest of section 7, and the posture itself is
review's,** a finite list of bounds being mechanical where a claim about every
invariant a type could own is not. The two are two records for that reason, per
the division rule of Document Format section 3, and the pin's record sits with
the bullet that argues it.

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
**The attribute's presence is pinned by the compile-fail doctests of section 7,
and the election test itself is review's,** a wildcard arm demanded from the
outside being mechanical where the judgment of which charter says a set grows is
not. The two are two records for that reason, per the division rule of Document
Format section 3, and the pin's record sits with the bullet that argues it.

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
that decides, which charter section 4 forbids. **The named candidates are pinned
by the compile-fail doctests of section 7,** because an absence is what a runtime
test structurally cannot demonstrate, **and the prohibition itself is review's,**
a finite set of doctests reaching the methods it names and not the open set of
all of them. The two are two records for that reason, per section 7.

```graph
node: traits-no-adjudicating-method
kind: assertion
tag: review

edge: asserts
from: weaver-traits
to: traits-no-adjudicating-method

edge: grounds
from: traits-no-adjudicating-method
to: axiom-floor-is-vocabulary-behavior-is-socket
```

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

edge: grounds
from: traits-tool-boxed-future-send
to: axiom-floor-is-vocabulary-behavior-is-socket

node: traits-tool-no-safety-classification
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-traits
to: traits-tool-no-safety-classification
```


**The tool taxonomy of `weaver-tools-vision` section 6 reaches this trait when the
workflow charters it, and that section now cuts on two axes rather than one.** An
action is elected when the model emits it and autonomic when the loop fires it on a
condition the loop set, and it is dispatched inward to an organ or outward through the
gate. **This trait serves the elected and outward corner alone**, which is what an
earlier reading called an external tool. The inward corners are function loops the
harness runs beside its control loops and reach no trait here. Which shape the trait
takes is still the workflow's question, and the classification no longer is.

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

edge: grounds
from: traits-provider-no-futures-dep
to: axiom-floor-is-vocabulary-behavior-is-socket
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
- The data derives of section 2 are read by a doctest: a function bounded on
  `Debug`, `Clone`, `PartialEq`, `Serialize`, and `Deserialize` accepts every
  message-model type and the mode enum, so a derive dropped later stops the
  build rather than a behavior. The named bounds are the pinned half of section
  2's posture claim, the posture itself staying review's, per the split this
  section already makes for the adjudicator candidates.

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
  makes for the manifest rules below. **The split is two assertions rather than
  one,** the pinned candidates here and the prohibition itself at the section 4
  clause that argues it: a single record tagged for the mechanical half would
  claim the instrument for the whole, which is the overclaim this corpus refuses
  in prose and has no reason to admit in a graph. Any clause naming one
  instrument for a claim's core and another for its periphery divides the same
  way.
- No safety classification on the eventual tool trait, when that trait exists.
- The non-exhaustive elections of section 2, pinned from the outside: doctests
  matching `Role` and `ContentBlock` without a wildcard arm fail to compile,
  which is `#[non_exhaustive]` doing its work where a consumer would feel it.
  The attribute's presence is the mechanical half of section 2's election claim,
  the per-charter test staying review's, per the same split. The exhaustive half
  of that claim needs no record here, being the first bullet of this section.

**Enforced by the manifest, with the instrument named rather than assumed.** The
no-internal-dependency rule is gate H2, which reads Cargo edges against the graph.
The no-async-runtime rule is **not** H2, which checks `weaver-*` edges only: it
takes a build-time assertion over the resolved external tree, a `cargo tree` check
in the workspace's own automation, and naming it here is what keeps it from being
a claim nothing runs.

**Which invariant each claim serves, and why most serve none.** Six of the
twenty-four carry a `grounds` edge and all six run to
`axiom-floor-is-vocabulary-behavior-is-socket`. The other three axioms take
nothing from this crate: it makes no claim about a turn key, it is not an organ,
and the one contract that draws it draws the message model rather than anything
this Spec settles about representing it. **The test applied is whether the axiom
is the reason the claim exists.** Remove the floor invariant and this crate has no
reason to refuse an internal dependency, no reason to keep an async runtime out of
its manifest, no reason to bound a boxed future with `Send` or to hold `futures`
off the floor, and no reason to deny `PermissionMode` a method that decides.
Remove it and the derive set is still the data derives, the tagging test still
yields the same two shapes, the role set is still three, and the enums are still
non-exhaustive where the charter says the set grows, so those ground in nothing.
**Eighteen claims grounding in no invariant is the expected result and not a gap**,
per Document Format section 4: section 2 of this Spec is named for representation,
a floor crate is mostly that, and representation is what the invariants are not
about.

**The four edges a reader would otherwise have to reconstruct.** The `Send` bound
and the refusal of a `futures` dependency read as async-ecosystem preference and
are not. An executor bound written into a floor signature is a transport decision
taken on the floor, and an async crate the floor carries is one everything above
carries, which is behavior arriving by linkage rather than from behind a socket.
The prohibition on an adjudicating method is the same invariant at the type level,
a floor type that decides being what its first half rules out, and the doctest
pinning of the named candidates carries the edge for the same reason: the two are
one claim divided by instrument, per Document Format section 3, and grounding one
half and not the other would make the division carry a difference it does not
hold.

**Two neighbours that look like they belong and do not.** The absence of a safety
classification sits in the same clause as the boxed future's bound and grounds in
nothing, because its reason is apex section 3 step 7, where what a tool reaches is
bounded by the kernel rather than by a judgment, and that reason stands whether or
not the floor holds only vocabulary. The bar on a format crate sits beside the bar
on an async runtime and grounds in nothing for a like reason: a format crate does
no work, and the argument against it is that rendering is decided where the
rendering happens, per `weaver-trace-PRD` section 4.2.

**Where the records sit, and the one claim another crate declares.** The
assertion records are at the clauses that argue the claims, across sections 1
through 6, rather than gathered here, per Document Format section 6. Six
claims are argued only in this section and carry their records at the end of
it, among them the doctest pinning of the adjudicator candidates, whose
prohibition is section 4's, and the two pins divided out of section 2's
posture and election claims, whose remainders are that section's. **The
licensed combinations of section 3 are declared by `weaver-harness`,** whose
Spec section 8 carries the test and has
discharged the owing, because an assertion belongs where its test lives. **The
tagging test shared with `weaver-types` is declared there,** node and both
edges, per that Spec's section 4.3 and the pilot's rule: one claim is one node,
and this crate's edge is already stated beside the shared statement rather than
restated here.

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

node: traits-adjudicators-pinned-by-doctest
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-traits
to: traits-adjudicators-pinned-by-doctest

edge: grounds
from: traits-adjudicators-pinned-by-doctest
to: axiom-floor-is-vocabulary-behavior-is-socket

node: traits-derive-set-pinned-by-doctest
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-traits
to: traits-derive-set-pinned-by-doctest

node: traits-non-exhaustive-pinned-by-doctest
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-traits
to: traits-non-exhaustive-pinned-by-doctest
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
