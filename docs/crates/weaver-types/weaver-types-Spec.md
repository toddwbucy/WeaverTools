# weaver-types - Spec

**Status:** MERGED. Cut 2026-08-01 with `weaver-traits-Spec` as the floor pass of phase
one's Spec pass. Code is written against it under the gates of Working Process section
6.

**Date filed:** 2026-08-01
**Revised:** 2026-09-04, fourth of this date, a session stands from a record.
Section 2 gains `restore`, optional, the record the session stands from and the
turn it stops at, and section 4's enter carries it resolved beside `stack`, the
digests of the organ binaries admin started. Per issue #432.
**Revised:** 2026-09-04, second of this date, admin is the sink field's reader.
Section 2 names it beside the discriminant's argument, per `weaver-admin-Spec`
section 5 and issue #311.
**Revised:** 2026-09-04, first of this date, the identity field is the seed.
Section 2 states that
`identity` seeds the session's first load and that the store governs every load
after it where a state member stands, per `weaver-state-PRD` section 4 as revised
this date and issue #422. The shape and the refusals are unchanged.
**Revised:** 2026-09-01, the column registry's refusals are typed.
`TokenRefusal` gains `ColumnPermissionAbsent`, `ColumnReadoutUnelected`,
and `ColumnUndeclared`, the three arms of `weaver-spu-PRD` section 13.7's
open registry in that clause's own order - no permission, no readout
election, no column in the family's declaration - each a unit variant
because the fact refused carries no value the record holds nowhere else,
on the registry-typing pattern the re-feed's arms set. The refused ask is
`Open`, which `TokenAsk` already names, so no ask joins.

**Revised:** 2026-08-31, fifth of this date, the permission members take a
parse the seam survives. The unknown-field mechanism the second entry of
this date elected for `refeed_permission`, and the column entry for its
sibling, is withdrawn as falsified by the build: one type serves the
declaration and the instruction, so a parse that refuses the member refuses
the instruction admin authored at the next seam, and the first admit across
the coded seam died at exactly that parse. Both members take `default`
instead, and the declaration's refusal moves to admin's inventory, the
resolution site, which refuses a declaration granting either member as
`ConfigInvalid` naming the field, per `weaver-admin-Spec` section 7. A
written `false` is inert, overwritten at construction from the resolved
kind and indistinguishable from absence, which is the bound `default` buys
and the reason the refusal is of the grant alone. This date's entries also
move to the banner's head in this act, having accumulated below entries of
2026-08-21, where newest-first does not file them.
**Revised:** 2026-08-31, fourth of this date, the registry's refusals are
typed. `TokenRefusal` gains `RefeedPermissionAbsent` and `RefeedPathEmpty`,
the two arms of `weaver-spu-PRD` section 13.14's registry, each a unit
variant because the fact refused carries no value the record holds nowhere
else. `TokenAsk` gains `ReFeed`, so the clerking rule of 2026-08-22 names
the refused ask like every other.
**Revised:** 2026-08-31, third of this date, the re-feed carries the rendered
form. `ReFeed`'s middle member read `delta: Vec<Message>`, which would route
the replay through the family renderer - the exact re-rendering the ruling
of 2026-08-12 forbids, the record holding the rendered form precisely so a
replay does not re-render through a template that may have changed. The
member is `rendered: String`, the contribution as `model.request` recorded
it, tokenized and appended with no family rendering on the way.
**Revised:** 2026-08-31, second of this date, the re-feed takes its shapes.
Per `weaver-spu-PRD` section 13.14 and the decode contract's sixth exchange.
`TokenDirective` gains `ReFeed`, the turn, the recorded delta, and the
recorded path. `TokenAnswer` gains `ReFed`, carrying the generation shape
under its own arm so a supplied path can never wear a sampled path's
clothes: the payload is shared and the variant is not, which is the whole
of what the type buys. `DecoderInstruction` gains `refeed_permission`,
admin's member beside `column_permission`, named individually per the
no-bundle rule, set from the binding's kind, and refused in a declaration
under the unknown-field rule like its sibling.

**Revised:** 2026-08-31, the column takes its shapes. Per the operator's
ruling of 2026-08-30 and `weaver-spu-PRD` section 13.7 as amended.
`DecoderInstruction` gains `column_permission`, admin's member and never the
declaration's: admin sets it from the binding's kind, and the parse refuses
it in a declaration under the unknown-field rule, so an operator cannot
write what only the binding may derive. `TokenDirective::Open` gains
`column_ask`, the once-at-open cadence that charter clause elects.
`TokenAnswer` gains `Column`, the decode seam's third intermediate, carrying
its position like `Field` and the layers' values layer-major, crossing as
bare JSON under section 4.4's provisional encoding, the efficient framing
staying `weaver-spu-Spec` section 12's open election. Absent where the ask
is absent and never present-and-empty, the field's own rule.

**Revised:** 2026-08-28, sixth of this date, the identity's refusals are
enumerated and a fourth is added. Sections 2 and 5 state that a message
carries at least one `Text` block and that each carries text: an empty string
in a licensed block passed both prior checks and seated the same nothing an
empty list would have. Section 5's entry names each refusal with its own
watch rather than describing two and citing one.

**Revised:** 2026-08-28, fifth of this date, the parse judges the block
beside the role. Sections 2 and 5 state that an identity message carries
`Text` and nothing else, the door refusing both and the role alone having left
half the refusal at runtime.

**Revised:** 2026-08-28, fourth of this date, the review seat's findings land.
The parse's rule stands and the two families that name no system turn fold it
rather than refusing, per the operator's ruling. Section 5's test joins the
perturbation enumeration its peers sit in, and that section's claim counts are
corrected from seventeen and ten to twenty and thirteen, two thirds of the
drift predating this act.

**Revised:** 2026-08-28, third of this date, the identity role's check lands
and the rule stops being owed. Section 2 states the refusal at the parse with
its field name, section 5 moves the entry from enforced-by-nothing to a
perturbation-verified test, and the assertion record withheld two days ago
lands with the instrument it names. The gate dissolved when the operator
ruled a re-baseline on one frozen build, retiring the deposits the old role
was held for. Item 2 of issue #369, code half.

**Revised:** 2026-08-28, second of this date, the identity prefix's role is
judged where declarations are judged. Section 2 states that every identity
message carries `role: system` and names the parse as the place the rule
binds; section 5 records that the instrument is owed rather than in force,
and why it waits on issue #346. Item 2 of issue #369, docs half.

**Revised:** 2026-08-28, the eleventh case is renamed to the condition it
reports. `identity_prefix_unrecorded` rather than a role-specific name, the
role being one of four ways the record fails to account for a seated prefix.
Per the review seat on PR 371.

**Revised:** 2026-08-27, the fault case set widens to eleven. Section 4.2's
arithmetic follows `weaver-harness-PRD` section 5 from four harness cases to
five, the eleventh being the identity door's refused role, per issue #369.

**Revised:** 2026-08-26, the refusal block catches up to its fifth case.
`TokenRefusal` gained `UnremovableSpan` with the wire on 2026-08-22,
carrying the span it refused beside the bounds it was judged against so the
loop learns which edge it crossed, and this section's prose has argued the
case since that date while the block held four - the ruling reaching the
banner and the prose and never the enumeration beside them, the drift the
audit of 2026-08-26 names as the corpus's dominant failure, caught here.

**Revised:** 2026-08-25, this Spec is named the election's shape authority.
`weaver-trace` holds a second `Election` of this shape because it links nothing, and
from this date the record carries that form too, making four representations of one
fact: this declaration, the enter payload, the tee's own, and the record's. Section 2
states that this document is authoritative for the election's declared shape, that what
the term means on the state seam is `weaver-harness-state-contract`'s, and that a
divergence of shape is a defect against section 2 of this document, per G5. **Field
spellings are each renderer's**: this declaration renames for the operator's file and
`weaver-trace` renders for a seam that already parses it, so the record's `all_kinds` is
not a divergence. Unifying the types is not owed. Per issue 347.
**Revised:** 2026-08-24, third of this date, the diagnostic member composes
inside. The entry below is reversed rather than amended and stands so the
reversal is visible: `trace_sink` goes back to required under either kind,
every binding declaring a sink and the kind selecting the mechanism the
harness authors through rather than whether it authors, per
`weaver-agents-PRD` section 6 as ruled this date. The optional count returns
to three by that rule and one by the conditional one. `gate_instruction` is
unaffected and stays conditional.
**Revised:** 2026-08-24, second of this date, the diagnostic binding writes no
record. `trace_sink` becomes an option and joins `gate_instruction` under the
same conditional rule, a serving binding requiring each and a diagnostic
binding excluding each, per `weaver-agents-PRD` section 6 as amended this date.
An absent sink is stated as not a defaulted one, so no reader invents a path.
The optional count was stale before this entry and is corrected in the same
pass, reading two where the paragraph then named three, `binding_kind` having
joined `state_election` and `loop_file` without the count following it. The
entry below gains the ordinal the convention asks for, which it was filed
without.
**Revised:** 2026-08-24, first of this date, the kind takes its shape. `BindingKind`
joins the config as an option whose absence means serving, `gate_instruction` becomes an
option whose presence follows the resolved kind, and `EnterPayload` carries
`EnterBinding`, the kind resolved with the gate instruction riding inside the serving
case. The refusal question the contract left to this round is answered by shape: a
directive disagreeing with its kind cannot be constructed. Per `weaver-agents-PRD`
section 6 as amended this date and the contract act of the same date.

**Revised:** 2026-08-22, second of this date, the refusal record takes its
shape. `RefusalRecord` names the seam and carries that seam's own case, with
the ask beside it where the seam's asks reach no event of their own, per
`weaver-types-PRD` section 2.1's clause of this date. `TokenAsk` and
`LifecycleAsk` arrive with it. Classify carries no ask, its content already
reaching the record under its own kind.
**Revised:** 2026-08-22, the elision takes its wire shape.
`TokenDirective` gains `Elide { from, to }`, a half-open span of resident
positions, and `TokenAnswer` gains `Elided` carrying the resident counts
either side as `Flushed` does, per the decode contract as amended this date.
The span is state and never the record.
**Revised:** 2026-08-21, the decoder instruction carries a third election.
`DecoderInstruction` gains `surprisal_election`, a bare boolean beside the
field's option, per `weaver-types-PRD` section 2.1's clause of this date on
issue #258. A flag rather than an option because a per-position reading has
no size to declare.
**Revised:** 2026-08-21, the field takes its shapes. `DecoderInstruction`
gains `field_election`, optional because the election is, carrying the
depth the operator declared per `weaver-spu-PRD` section 13.11.
`TokenAnswer` gains `Field`, the decode seam's second intermediate, which
carries its own position because a token message crosses per renderable
piece and cannot pair one. `Candidate` is the ranked pair. Absent where
the election is absent and never present-and-empty, the rule the
measurement's unproduced readings already follow.
**Revised:** 2026-08-20, second of this date, the loop joins the
declaration. Per the operator's ruling on issue #243: `AgentConfig`
gains `loop_file`, optional beside `state_election` and by the same
exception, `weaver-harness-PRD` section 2 ruling its absence as the
worker's own default loop. Present, it names the loop file the agent's
worker runs, the loop being a member of the agent's harness and unique
to it. The carriage is `weaver-admin-Spec` section 6's, in the same
act.
**Revised:** 2026-08-20, the segment series takes shape. Per the decode
contract's amendment on issue #236: a frame past the envelope crosses as
a preamble spelling its count and byte length, then raw octet slices
reassembled to the one frame, recognized by the `kind` member the
preamble lacks, bounded in total at eight mebibytes as
`DECODE_MESSAGE_BOUND`. Section 4.4 carries the spelling beside the
encoding it segments.
**Revised:** 2026-08-19, seventh of this date, the classify role joins the
declaration. `SpuInstruction` gains `classify`, optional by presence per
`weaver-spu-PRD` section 15.3, carrying the model binding at the smaller
size: the second role key, arriving with the act that builds it exactly as
section 2's role-not-slot rule promised. The external citation to
`weaver-spu-Spec`'s open elections follows that document's renumbering to
section 12 in the same act.
**Revised:** 2026-08-19, sixth of this date, the label trio takes shape.
Section 4.5 holds the classify contract's trio the way 4.4 holds the
decode contract's: the cases spelled once from the contract's enumeration,
the directive and refusal internally tagged and the answer adjacent
because the fault's account splices, and JSON as a settled election
against the loop 0 criterion, there being no hot path to measure. Arrives
with the floor half of the classifier's owed acts, per the contract's
section 9.
**Revised:** 2026-08-19, fifth of this date, the flush names its cut.
`TokenDirective::Flush` gains `keep`, the resident length the session
returns to, per the operator's ruling that the cleanup line is the
loop's and the decode contract's amended flush exchange. The cut is
bounded rather than refused, below by the identity prefix and above by
the resident count, the confirmation's standing counts carrying what
held, so the answer's shape does not move.
**Revised:** 2026-08-19, fourth of this date, the generation reports the
session's fullness. Section 4.4's `Generation` gains `resident` and
`capacity`, the session's token count and its ceiling as the generation
closed, per issue #221's arc: the loop that manages the context must see
pressure before the wall, and the wall's own refusal was the only carrier
of either number.
**Revised:** 2026-08-19, third of this date, the finish tells the truth.
Section 4.4's `Finish` gains `Length`: the generation ended because the
turn's token limit was reached, a third fact the two-case set flattened
into `Completed`, which issue #218 found from the record's own evidence -
a capped answer reporting itself complete. The trace's mirror and the
close's member move in the same act.
**Revised:** 2026-09-04, the store election reaches the declaration.
`AgentConfig` gains `state_store`, optional, its absence meaning the embedded
engine per `weaver-state-PRD` section 4 as revised this date, its `engine`
closed at three cases, and `database` and `role` required under the service
engine and refused under the other two. `EnterPayload` carries it resolved
beside the state election. Per issue #411.
**Revised:** 2026-08-19, the tee's election reaches the declaration.
`AgentConfig` gains `state_election`, the one optional field this Spec
carries, its absence meaning the ruled default election per
`weaver-state-PRD` section 4, which is the exception the required-field
rule itself names: a charter saying a field is optional and saying what
its absence means. `StateElection` and `ElectedKindConfig` join the
section, `EnterPayload` carries the resolved election to the worker, and
the supplies change lands in `weaver-admin-harness-contract` in the same
act.
**Revised:** 2026-08-17, the `Generation` subsection's restatement of the
measurement's enumeration follows the decode contract's #129 correction: the
template identity is the request's member, and the sentence names itself a
restatement so the authority stays with the contract.
**Revised:** 2026-08-16, second of that date, a held unit name is its own
refusal. `LifecycleRefusal` gains `PriorUnitUnreaped`, because `BindFailed` was
answering for two conditions a state ask already tells apart: a unit that runs
with an unreachable socket, where a bind is what failed, and a unit whose
process exited non-zero, where a held name is what refuses.
**Revised:** 2026-08-16, the declaration carries what a binary left movable.
`DecoderInstruction` gains `tunable_values`, a name-keyed map of numbers that is
the route `Disposition::OperatorTunable` names and had never been built. A map
rather than a field per parameter because which parameters a binary leaves
tunable is that binary's election and moves with a recompile, so a floor type
enumerating them would move with every deployment that changed its mind.
**Revised:** 2026-08-12, fourth of this date, the encoding rides with the
type. One sentence lands with the gate's turn-half act: the frame encoding's
implementation is the floor's, beside the type, one canonical form for every
party, owed by the wiring act.
**Revised:** 2026-08-12, third of this date, the frame's shape closes.
`TurnFrame` carries its octets base64-encoded in one member, elected at
section 4.1 by argument rather than the deferred measurement, per the
operator's ruling of this date: no party validates a line before it crosses,
so a splice would convert a refused turn into a channel fault that ends
service. Section 6's bullet closes, one perturbation assertion lands, and
the code's empty placeholder gains its member with the wiring act.
**Revised:** 2026-08-12, second of this date, the request is the turn's
contribution. Per the operator's ruling closing issue 124, `Generation`'s
`request` member carries the turn's delta as rendered, the full effective
context being the accumulation the record determines. No shape moves, and
the wording follows `weaver-trace-PRD` section 3.2 as narrowed.
**Revised:** 2026-08-12, the receipt retires and `Received` leaves the trio.
Per the decode contract's second ruling of this date, the SPU's fault report
is the seam's one emission and takes no answer, so the `TokenAnswer` case
that closed it as an exchange is a case nothing would construct, the
reserved slot apex section 7 forbids in data. Section 4.4 drops the case
and states the report's standing: an emission owed nothing back, its wire
case arriving with `FaultReport`'s shape, which section 6 holds open.
**Revised:** 2026-08-17, second of that date, the fault report takes its
shape. Section 6's election closes at section 4.2: a typed `case` against the
three charters' closed nine, and an organ-rendered `account` the harness
splices, apex section 5.2's custody rule deciding the split. The token trio
gains the emission's `Fault` case, the wire case the entry below always said
would arrive with the shape, a case of the answer that closes no exchange and
answers nothing. The run bracket's payload question is settled by the same
judgment and recorded with the closed election, its two reopenings named as
owed to the act that builds the load line's mechanism.
**Revised:** 2026-08-11, third of that date, the model events splice.
`Generation`'s `rendered` member becomes `request` and carries the model.request
content whole, the rendered prompt with its template and effective sampling,
because the custody act makes `model.request` and `model.measurement` spliced
payloads the SPU renders and the harness carries opaque. No case set moves, the
trio's shape unchanged but for the member's name and widened content.
**Revised:** 2026-08-11, the seam streams. `TokenAnswer` gains its `Token`
case, the intermediate the decode contract's streaming ruling of this date
enumerates, any number preceding the close and none closing, the identifier
a bare `u32` and the piece the family's rendering of that one token.
`Generation` carries `request` and `measurement` as two spliced members, one
per record box, the request being the model.request content whole and renamed
from the `rendered` of one act ago now the custody act of 2026-08-11 makes the
whole request the SPU's to render. The decoder's
section gains the `identity` field, the open exchange's canonical messages
as configuration rather than history, required with an empty list
legitimate, interior to the section with no vocabulary node moving.
**Revised:** 2026-08-10, second of that date. The shared tagging test of section
4.3 gains its fourth arm, identical in `weaver-traits-Spec` section 3 so the
floor cannot drift: an enum with a variant wrapping a struct that carries a
spliced member is adjacently tagged, the trio's code act having measured
internal tagging failing the round trip at deserialization, an invalid-type
error at the splice, where the rule's rationale named only write-side
failures. Applied at section 4.3: the token directive and refusal internally
tagged, `Finish` fieldless, and `TokenAnswer` adjacent under the new arm.
**Revised:** 2026-08-10. The SPU's configuration becomes a section:
`spu-instruction` holds a `decoder` subsection carrying the model binding and
the readout election together, and the section is what `EnterPayload` and
`Admit` now carry, which closes the route the charter's revision of this date
names, the election having been declared, held, and given no seam to cross. The
gate's instruction is the pattern followed rather than a precedent invented. In
the same act, `Generation` settles at section 4.4 and its bullet leaves section
6: the emission and the finish are shaped here because the harness consumes
them, and the measurement splices because nothing consumes it on the way to the
trace's model events, the satellites staying `weaver-trace`'s. Section 1's
dependency set names `serde_json`'s `raw_value` feature, the splice's gate, on
the review seat's finding that the set was argued at feature granularity and
the new member was absent. No graph record moves, the trio's nodes being the
charter's and the config vocabulary unchanged.
**Revised:** 2026-08-08, second of that date. The trio names what exists. The
messages it carries are `weaver-traits`' `Message`, drawn rather than restated,
where an earlier wording named a `CanonicalMessage` that exists nowhere.
`Generation` is moved to section 6 as an open election rather than left as a
name with no shape: the determination is the decode contract's and is not in
question, and what is open is where the payload lives, seven of the satellites
it implies being `weaver-trace`'s against a crate that links one internal
dependency. Restating them would put one fact in two places with no authority
and drawing them would widen a floor whose argument is thinness, so the deferral
follows the fault report's rather than inventing a shape ahead of its answer.
Section 6's encoding bullet narrows to the encoding, a review finding of this
act: the entry preceding this one argued that the shape and the encoding
separate and did not carry that through to the bullet still claiming both, so
the document represented the trio at section 4.4 and called it unrepresented at
section 6.
**Revised:** 2026-08-08. The token trio is held at the new section 4.4, its cases
being the decode contract's sections 2 and 5 and this crate holding rather than
creating them. An earlier wording of section 4 deferred the trio's shape and its
encoding together, and the two separate: what cases a type carries is determined
by the demand construction puts on it, and only the encoding waits on the
hot-path measurement. The deferral had become circular, that measurement being
taken against traffic that cannot exist until the seam carries the trio. The
section is retitled from the loop 0 wire vocabulary to the organ wire vocabulary,
the heading being the only place in the corpus that phrase appeared and every
citation to this section being by number. **The sorting test is rate of change
rather than loop number,** per the operator of this date: a type an organ
publishes is constitutive and lives here, loop 0 among them because a badly
changed loop 0 is a model that cannot load, and a loop conforms to an organ's
type where it uses one without being obliged to use it. A shelf for types a
particular loop 1 defines is not created here, nothing in this crate being one
today. No graph record moves: the nodes are the charter's and the ordering's
instrument is `weaver-spu-Spec` section 9's.
**Revised:** 2026-08-06, `lifecycle-refusal` gains `StateNotObservable`. The
`weaver-admin` code act found sections 2 and 3 of `weaver-admin-Spec` jointly
unsatisfiable for `show` and `list`: the answer must be one of the floor's two
enums, and the only fitting answer cases carry an `AgentState` the corpus has no
source for. The case is the honest third door, per section 4.2, and it retires
with the observation exchange that closes the gap.
**Revised:** 2026-08-14, the run identifies itself. `AgentConfig` gains `session`, the
grouping the operator declares, and `EnterPayload`'s `run_ordinal` becomes
`run: RunId`. Both are identifiers rather than numbers, which is what lets a
run reference carry a stamp that distinguishes without anything being
remembered between invocations.
**Revised:** 2026-08-15, the gate socket is the program's. `GateInstruction` loses
`socket_path` and `Raise` gains a `socket` beside the instruction, two fields because
two authors. The group survives the loss rather than collapsing to a bare rule, so a
field the gate workflow adds later has somewhere to land.
**Document ID:** `weaver-types-Spec`
**Parent:** `weaver-types-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

Build instructions for `weaver-types`: the module layout, the item signatures, the
file format the operator writes, the wire encoding the organ channels carry, and
the elections a builder would otherwise invent. It is derived from
`weaver-types-PRD` and from every contract that draws this crate, which is six:
the coordination, residency, gate, and trace agreements, and the two external
boundaries, `weaver-admin-operator-contract` drawing the identity pair with the
refusal and `weaver-gate-world-contract` drawing the identity pair with the gate
instruction.

Level discipline. The charter says what the crate holds and why. This document
says how it is represented, which for this crate means two representation
questions the corpus deliberately left here: what shape the agent config takes on
disk, and what octets the organ wire vocabulary becomes on the wire. Where this
document and the charter disagree the charter yields nothing.

**This document declares its crate's assertion records, and one edge that is
another crate's,** per Document Format sections 3 and 4 as of the notation of
2026-08-03. The exception is the shared tagging test of section 4.3: one claim
is one node with an `asserts` edge per crate bound by it, the node lives at the
statement both floor Specs share, and `weaver-traits`' edge is therefore
declared here beside it. **`weaver-traits-Spec` declares its own assertion
records except that edge,** and says so, which is the other half of the same
rule: without both halves the traits act either redeclares an edge already
declared here, which is the duplicate the format forbids, or drops part of its
crate's assertion set with nothing recording where it went. The charter
stays the source of the crate node, the `agent-config` artifact, its six
`holds` edges, and the seventeen vocabulary definitions. What this document
sources is the claims code must conform to, declared at the clauses that argue
them rather than gathered in one place, per that format's section 6, and
`asserts` runs from the crate rather than from this document, which is why the
document needs no node of its own.

## 1. The crate

**Layout.** One module per charter subsection, re-exported at the root.

    src/lib.rs        re-exports, and nothing else
    src/config.rs     the agent config and its six fields, section 2
    src/identity.rs   peer identity and the authorization predicate, section 3
    src/wire.rs       the organ wire vocabulary and the envelope, section 4

**Edition and toolchain.** Edition 2024 on the pinned nightly, no nightly feature
used, for the reason `weaver-traits-Spec` section 1 gives: a nightly requirement
on the floor is a nightly requirement everywhere.

**The dependency set is four crates, one of them optional, and each is argued.**
`weaver-traits`, the one internal dependency, declared as the `floor-link` the
charter carries and required because the config's permission mode and tool set
elect from that crate's vocabulary. `serde` with `derive`, for the config and the
wire types both. `serde_json` for the wire encoding, per section 4.3, with its
`raw_value` feature on, because section 4.4's measurement is spliced JSON and
the splicing type is what that feature gates. A
maintained YAML implementation for the config file, per the election of section
2, **behind a non-default `config` cargo feature**. Nothing else, and specifically
no socket crate, no async runtime, and no logging: this crate defines what crosses
a boundary and crossing it is somebody else's crate, per charter section 3.

```graph
node: types-one-floor-link
kind: assertion
tag: manifest

edge: asserts
from: weaver-types
to: types-one-floor-link

edge: grounds
from: types-one-floor-link
to: axiom-floor-is-vocabulary-behavior-is-socket

node: types-no-socket-no-runtime-no-io
kind: assertion
tag: manifest

edge: asserts
from: weaver-types
to: types-no-socket-no-runtime-no-io

edge: grounds
from: types-no-socket-no-runtime-no-io
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**The feature gate is thinness applied where the floor is widest.** Every crate in
the program links this one, and only admin and the harness parse the config file,
per charter section 2.1. Without the gate the gate crate and the SPU carry a YAML
parser they never call, into processes whose whole argument is that they hold
little. `weaver-traits-PRD` section 5 states the doctrine this follows: thin is
the point, and a floor that accumulates stops meaning anything. The wire types and
the identity pair are unconditional, because more than one crate draws each.

**Two format crates rather than one is a deliberate cost.** The config is written
by a human and the wire is written by a program, and the two audiences want
different things, per the elections below. A single format for both would be one
dependency lighter and would make one audience worse off, which is the wrong trade
on a file an operator hand-edits under load.

## 2. The agent config

The declarative document that defines an agent, per charter section 2.1: written
by the operator, validated by admin before a process exists, read by the harness
for the elections it carries.

**The format is YAML, elected against the charter's criterion, and the
maintenance fact is part of the argument rather than a discovery a builder
makes.** `weaver-types-PRD` section 2.1 states the ground: the file's reader is a
human writing it, so the format answers to a writer rather than a parser, carries
nesting without ceremony, and survives an operator's comments. YAML meets that
criterion, being what a system administrator already reads. TOML was the alternative
and is the Rust-native choice, and it loses on deep nesting for a reader who is
not a Rust programmer. JSON was never a candidate: no comments, and a
trailing-comma error at three in the morning on a file that gates a load is a bad
way to learn about JSON.

```graph
node: types-config-format-yaml
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-config-format-yaml
```

**What this election must survive is that `serde_yaml`
was archived and deprecated by its author in 2024**, so the implementation is a
maintained one and the Spec pass names that as a requirement rather than a
preference. A builder confirms maintenance status at the moment of writing the
manifest, and if no maintained implementation exists then the YAML conclusion
falls with its premise and the TOML comparison re-runs on the writer-audience
grounds above.

**The session is declared and the run is minted.** `session` joins the
declaration because a session spans runs and an agent outlives a session, per
`weaver-admin-PRD` section 4.4, so the grouping is one only the operator can
draw and the same agent serves many of them by editing one field. The run is
the other half and does not appear here: `RunId` is minted at each load by the
party that performs it, and a declared run would be a value the operator had to
change before every load or watch collide. What the floor fixes is that both
are identifiers rather than numbers, which is what lets a run reference carry a
stamp that distinguishes without anything being remembered between invocations,
per the identity ruling of 2026-08-14.

**One file per agent, named for the agent, in a directory the operator owns.**
This Spec fixes neither the directory nor the naming convention, which are
operator provisioning and outside what this program governs, per
`weaver-admin-PRD` section 1. What it fixes is that admin resolves an agent name
to exactly one config file and refuses a load where it resolves to none or to
more than one.

```rust
pub struct AgentConfig {
    pub session: SessionId,
    pub spu_instruction: SpuInstruction,
    pub tool_set: Vec<ToolName>,
    pub permission_mode: weaver_traits::PermissionMode,
    pub binding_kind: Option<BindingKind>,
    pub gate_instruction: Option<GateInstruction>,
    pub trace_sink: TraceSink,
    pub state_election: Option<StateElection>,
    pub state_store: Option<StateStore>,
    pub loop_file: Option<PathBuf>,
    pub restore: Option<Restore>,
}

pub struct Restore {
    pub record: PathBuf,
    pub through: Option<Cut>,
}

pub struct Cut {
    pub run: RunId,
    pub turn: u64,
}

pub struct StateStore {
    pub engine: StoreEngine,
    pub database: Option<String>,
    pub role: Option<String>,
}

pub enum StoreEngine {
    None,
    Sqlite,
    Postgres,
}

pub struct StateElection {
    pub all_kinds: bool,
    pub keys: Vec<ElectedKindConfig>,
}

pub struct ElectedKindConfig {
    pub kind: String,
    pub paths: Vec<String>,
}

pub enum BindingKind {
    Serving,
    Diagnostic,
}

pub struct SpuInstruction {
    pub decoder: DecoderInstruction,
    pub classify: Option<ClassifyInstruction>,
}

pub struct ClassifyInstruction {
    pub model_binding: ModelBinding,
}

pub struct DecoderInstruction {
    pub model_binding: ModelBinding,
    pub residual_readout_election: bool,
    pub field_election: Option<FieldElection>,
    pub surprisal_election: bool,
    pub column_permission: bool,
    pub refeed_permission: bool,
    pub identity: Vec<weaver_traits::Message>,
    pub tunable_values: BTreeMap<String, f64>,
}


**`identity` is the seed and not the session's identity, per the operator's ruling of
2026-09-04 on issue #422.** The field keeps its shape and its refusals: canonical
messages, every one `role: system`, required with an empty list legitimate. What changed
is its authority. Where a state member stands, the session's identity is what the store
holds under the turnless `message.system` events at the session's opening, and this
field is what the first load of a session seats and lands there, the store governing
every later load of the session, per `weaver-state-PRD` section 4. Where no member
stands the field governs alone, which is what it did before the ruling. Divergence
between the two is not a defect, because they answer different questions, the seed and
the session, and G5 names the store authoritative within the session.
pub struct FieldElection {
    pub depth: u32,
}

pub enum TraceSink {
    File { path: PathBuf, create: bool },
    Pipe { path: PathBuf },
    Socket { path: PathBuf },
}

pub fn parse(source: &str) -> Result<AgentConfig, ConfigError>

pub struct ConfigError {
    pub field: Option<FieldName>,
    pub kind: ConfigErrorKind,
}

pub enum ConfigErrorKind {
    Malformed,
    MissingField,
    UnknownField,
    BadValue,
}
```

**`surprisal_election` is a bare `bool` and `field_election` is an
`Option`, which is a difference in what each has to say rather than an
inconsistency.** A field that is elected carries a depth, so the option
holds the value and its absence is the whole of an unelected field. A
surprisal has no size: the vector exists per decode position or it does
not, so the flag is the whole of it either way. **False is not the same as
absent here**, per `weaver-trace-Spec` section 3: a declaration written
before this election existed omits the member, and serde fills the default,
so the two states a reader must keep apart are kept apart in the record
this crate feeds rather than in this struct.


**`tunable_values` is the route `Disposition::OperatorTunable` names.** The SPU
elects per parameter at its composition root whether a value is compiled in or
supplied, and this map is where a supplied one arrives. It is keyed by the
parameter's name and carries a number, which is the shape the SPU's resolve
already takes, and it covers the sampling knobs and the session parameters
alike because the election is the same election in both cases.

**It is a map rather than a field per parameter, and the reason is which side
owns the list.** Which parameters a binary leaves tunable is that binary's
election and changes with a recompile, so a floor type enumerating them would
have to move whenever a deployment changed its mind, and a declaration naming a
parameter this binary froze is a fact with no effect rather than an error. The
map is present always and may be empty, the no-defaulting rule reaching the
field and not its contents: an empty map is a declaration that supplies
nothing, and a binary with a tunable parameter and nothing supplied for it
refuses the load naming the parameter.

**A value is judged against what the parameter is before the session opens.**
The map carries `f64` because a configuration's numbers are, and most of what it
feeds is not a real: `top-k`, the repetition window, the context capacity and
the per-turn ceiling are counts. So a value bound for a count must be integral,
non-negative, and inside the range its type can hold, and a value bound for a
real must be finite. A `NaN` reaching a sampler is a temperature that compares
false against every bound and an infinity is one no filter clamps.

**A bad count is worse than either, and the reason is that the conversion never
fails.** A float cast to an integer in Rust answers for every input: it
truncates toward zero, so `3.7` becomes `3`, it maps `NaN` to zero, and it
saturates at the target's bounds, so `-1.0` becomes `0` and `1e20` becomes
`u32::MAX`. Measured on this workspace's toolchain rather than recalled. So a
fractional count is silently rounded, a negative one silently becomes zero, and
an oversized one silently becomes four billion, and none of the three reaches
the operator as an error. The truncating case and the saturating case are
different failures and the check refuses both, because a capacity of
`u32::MAX` would fail later at allocation with nothing pointing back to the
declaration that asked for it.

**The refusal is the load's, not the turn's.** A bad value is `BadValue` naming
its field, refused where the declaration is read and before any device work,
which is the same shape every other malformed field in this section takes. The
earlier home for this check was the token seam, where the map used to travel and
where the refusal was `MalformedDelta`, and it moved here with the map rather
than being invented.

**A value here never overrides a frozen parameter.** Frozen means compiled in
and not carried on the wire, so a name the binary froze is ignored where it
appears, and the record still reports the effective value whichever side set
it. That is what keeps a deployment's lock a lock rather than a default.

**The SPU's fields arrive as one section, and the gate's already did.** Charter
section 2.1 rules that an organ's fields are named together and cross together,
and `spu-instruction` is that rule's first application: one declaration admin
validates, the harness carries uninterpreted, and the SPU consumes.
**`decoder` names a role rather than a slot, and `classify` is the second
role, arriving 2026-08-19 with the act that builds it.** The organ's
chartered domain is
every semantic operation in the text modality, per `weaver-spu-PRD` section 8,
and the decode role is the one whose seam stood first,
`weaver-harness-spu-decode-contract`, so each key names something built, with a
reader today. The `classify` subsection carries the model binding at the
smaller size and is optional by presence, per `weaver-spu-PRD` section 15.3:
its absence is the operator declaring the agent runs no classifier, a
declaration rather than a default, so the no-defaulting rule below is
untouched by it. An embedder key arrives in the act that builds an embedder, named
here as absent rather than carried empty, which is the near side of apex
section 9. The no-defaulting argument below survives the nesting untouched:
every field of a present section is required, absence of a required field
refuses the load, and the depth
at which a field sits changes nothing about what its absence means.

**The identity material rides the decoder's section**, resolving the open
exchange's source per the harness handoff's first question: the canonical
messages the identity prefix is rendered from are configuration rather than
history, the working structure being empty at enter by construction, so the
operator writes them, admin validates them as part of the whole parse, the
harness carries them uninterpreted, and the SPU consumes them at the
session's open. The field is required like every field and an empty list is
a declaration the operator made, an agent with no identity prefix being a
legitimate agent, where an absent field is a file unfinished. The messages
are `weaver-traits`' `Message`, drawn rather than restated, and the field is
representation interior to the section the way the gate instruction's access
rule is, no vocabulary node moving.

**Every message of the identity carries `role: system`, and this clause states
the rule ahead of the instrument that will hold it.** The identity door of
`weaver-harness-Spec` section 6 writes `message.system` and refuses every
other role, per `harness-identity-door-writes-system-only`, so a declaration
carrying any other role seats a prefix into the decode context that the record
cannot show - the condition `weaver-harness-PRD` section 5's fifth case exists
to report, found in the field and filed as issue #369. Every message carries
at least one `Text` block, each with text in it and nothing else, per
`weaver-traits-Spec` section 3, and the parse judges that beside the role: the
door refuses both, and judging one here would leave the other to be met as a
fault in a running agent. **The rule belongs
here rather than at the door.** The door refuses what the declaration should
never have contained, which makes it the last place the condition can be
caught rather than the first, and a rule enforced only at the last place is a
rule the operator meets as a fault in a running agent instead of as a refusal
to load. Where declarations are judged is the parse, and the judgment is this
crate's.

**The check is the parse's, and it is in force.** The instrument is the shape
`model-binding`'s empty device list already takes: a `ConfigErrorKind::BadValue`
naming `identity.<n>.role`, so a declaration that would seat an unwritable
prefix does not parse and never reaches a door. The index rides the name
because an operator with several messages needs to know which one, as
`tunable-values.<name>` already does.

**It was written as owed for two days and the reason is worth keeping.**
Changing a declaration's identity role changes the prompt - a system role
renders `<|im_start|>system` where a user role renders `<|im_start|>user` - so
the tokens differ and every run made under the old role stops being a
baseline. The cross-precision deposit of 2026-08-25 and the determinism matrix
of 2026-08-27 were both produced under `role: user`, and a refusal landing
before issue #346's cross-architecture arms ran would have voided that
comparison rather than delayed it. Those arms deposited 2026-08-27 and
2026-08-28, and the operator's ruling of 2026-08-28 retires those deposits as
a comparison basis in favour of a re-baseline on one frozen build, which is
what dissolved the gate. **An empty identity still parses**, an agent with no
prefix being a legitimate agent: the rule judges the messages present and does
not require one to be.

```graph
node: types-identity-role-is-system
kind: assertion
tag: perturbation

edge: asserts
from: weaver-types
to: types-identity-role-is-system
```

**Field names are kebab-case on disk and snake_case in Rust, which takes an
explicit election rather than a convention.** `#[serde(rename_all = "kebab-case",
deny_unknown_fields)]` on the struct, so the operator writes `model-binding` and
`trace-sink` as the charter's vocabulary names them and the Rust field names stay
idiomatic. The `deny_unknown_fields` half is the fixed-surface mechanism of section 2
and stands until the spec round
`weaver-types-PRD` section 2.1 names. Without the rename the two spellings diverge,
which is the collision Document Format section 5 rules against for graph identifiers and
which reads the same way on a page.

```graph
node: types-config-names-kebab
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-config-names-kebab
```

**Every field of the declared surface is required and absence is a refusal, with the
temptation named.** The surface is the union of what the organs register, per
`weaver-types-PRD` section 2.1, so what is required is a per-binary fact rather than one
struct's field list, and the refusal is against that surface rather than against a fixed
type. The property below is unchanged by that and is why the rule exists. Charter
section 5 rules that absence is never read as a default unless the charter says a field
is optional and says what its absence means. Three fields are optional by exactly that
rule's own terms, and one more is an option at the parse for a different reason, named
after them. `state_election` may be absent because `weaver-state-PRD` section 4 rules
what absence means, the default election, the envelope of every kind and nothing more,
so a deployment that elects nothing still holds the session's shape by the charter's
sentence rather than by a parser's guess. The resolved spelling of that default is fixed
here so two resolvers cannot disagree: `all_kinds` true and `keys` empty. The empty list
is only the default's spelling, not a constraint on the pair: `keys` stays meaningful
beside `all_kinds` true, each named kind adding payload paths on top of the envelope
every kind already crosses with. `EnterPayload` carries the election resolved, admin
filling that ruled default at inventory, so the worker never re-derives an absence. When
the block is present, both its members are required, the required-field discipline
resuming inside it. `loop_file` may be absent because `weaver-harness-PRD` section 2
rules what absence means, the worker's own default loop, the compiled body or the
installed file, so a declaration written before the member existed still parses and
still means what it meant. Present, it names the loop file the agent's worker runs, the
loop being a member of that agent's harness and unique to it per the same section's
ruling of 2026-08-20, and it reaches the worker in the unit's argument vector per
`weaver-admin-Spec` section 6 rather than in any exchange, because no exchange carries a
path. `state_store` may be absent because `weaver-state-PRD` section 4 rules what
absence means, the embedded engine, so a declaration written before the member existed
still parses and still means what it meant, and the state member stands. Present, its
`engine` names which port the deployment elects, as of 2026-09-04: `none` declares that
no member stands, which is a deployment's real posture and the one the
instrument-validation matrices ran under, `sqlite` the embedded engine, and `postgres`
the service engine, for which `database` and `role` are required and for the other two
are refused if present, the same cross-field rule admin holds for the gate instruction,
judged at inventory before a process exists. **`none` beside a present `state_election`
is refused by the same rule**: the election says what the tee sends to the member, and a
declaration that elects what to send to a member it declined is malformed rather than
surplus, refused `ConfigInvalid` naming the election, the way a granted permission
naming a field is. `none` with the election absent is whole, the ruled default still
written on the load event as the record's posture, per `weaver-trace-PRD` section 3.1,
beside a `state_member` of false. The three are the binding's members: they change only
across the load boundary, they ride the enter directive resolved, and the load event
records them, per `weaver-trace-PRD` section 3.1. The enum is closed at three because
the state charter charters two engines and the absence of a member, and a further engine
is a state act before it is a variant. `binding_kind` may be absent because
`weaver-types-PRD` section 2.1 rules what absence means, a serving binding, so a
declaration written before the member existed still parses and still means what it
meant, on the same footing as `loop_file` above. The enum is closed at two cases because
`weaver-agents-PRD` section 6 names exactly two kinds, and a third kind is an apex act
before it is a variant. `gate_instruction` is an `Option` for a reason the three above
do not share: presence follows the resolved kind rather than standing alone, a serving
binding requiring it and a diagnostic binding excluding it, per the contract's shape
rule of 2026-08-24. The parse cannot see a cross-field rule, checking each field alone,
so it parses as an option and the rule is admin's at inventory, before a process exists,
per `weaver-admin-Spec` section 4. **`trace_sink` is required under either kind**, and
briefly was not: an act of 2026-08-24 made it conditional on the reading that a
diagnostic binding authors nothing, and the operator's ruling of the same date replaced
that reading with a composition, every binding authoring a record and the kind selecting
the mechanism rather than the presence, per `weaver-agents-PRD` section 6. What the
declaration names is a sink, and what the run writes into it follows the kind without
the field moving. Every other field is required. The residual-readout election is what a
builder will reach to default, to off, and it is exactly the one that must not: an
operator who stated no readout has not thereby declined it, and admin refusing the load
is how that operator learns the file is incomplete rather than discovering it in a
record with no reductions in it. This is why `AgentConfig` derives no `Default` and
`parse` returns no partial value. `restore` may be absent because `weaver-state-PRD`
section 4 rules what absence means, the load standing from nothing, which is what every
load did before the ruling of 2026-09-04 on issue #432. Present, it names the record the
session stands from, a path admin reads under its own custody and the harness never
sees, and `through_turn`, the turn the restored holdings stop at, absent meaning the
record whole. A whole record under the declaration's own session name is a resume and a
cut record under a new one is a branch, per that section, and which it is the record
says.

```graph
node: types-required-field-refuses
kind: assertion
tag: perturbation

edge: asserts
from: weaver-types
to: types-required-field-refuses

node: types-no-default-derive
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-no-default-derive
```

**This Spec is authoritative for the tee's election's declared shape, and a second
spelling of it exists on purpose.** What the term means on the state seam is
`weaver-harness-state-contract`'s, which defines it there as that seam's vocabulary, so
the two authorities answer different questions rather than one question twice.
`weaver-trace` holds its own `Election` of the same shape because that crate depends on
nothing, not even this floor, and cannot spell a type it does not link. From 2026-08-25
the record carries that second form on the `load` event, per `weaver-trace-Spec` section
3, which makes four representations of one fact: this declaration, the enter payload,
the tee's own, and the record's. **Field spellings are each renderer's and are not
shape**: this declaration renames for the operator's file and the record renders
`all_kinds`, which `weaver-trace-Spec` section 3 argues cannot be renamed without moving
a live seam. This side renames for a file rather than a seam, per the kebab-case
election above. **A divergence of shape among them is a defect against this section**
rather than a choice a reader makes, per G5, and unifying the types is not owed: it
would cost `weaver-trace` a no-dependency property that is load bearing for its own
reasons in order to buy a name.

**`trace-sink` names a sink and not only a path.** A file, a pipe, or a socket are
all conforming sinks, per `weaver-admin-operator-contract` section 3, so the field
carries a discriminated shape and admin opens by the discriminant. A bare path
would force admin to guess from the filesystem what the operator meant, and the
guess is wrong exactly when the operator meant a named pipe that does not exist
yet, which is the discriminant's whole argument.

**Admin is the field's one reader**, per `weaver-admin-Spec` section 5's assertion that
the sink path dies at its one open site and issue #311: the harness receives the opened
descriptor over the coordination socket and never the path, so the name this field
carries reaches no process but admin's, and the harness charter's sentence that the
agent is never told it is the same fact from the other side.

```graph
node: types-trace-sink-discriminated
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-trace-sink-discriminated

edge: grounds
from: types-trace-sink-discriminated
to: axiom-contract-is-a-complete-interface
```

**`File` and `Pipe` carry a creation flag and `Socket` does not, and the
asymmetry has a reason rather than an oversight.** Admin can create either of the
first two, an empty file or a `mkfifo`, and the charter's validate step reads the
flag to decide whether a missing sink refuses the load or is made. A socket sink
is different in kind: admin connects to it and something on the operator's side
must already be listening, so a creation flag would promise an act admin cannot
perform. A missing socket sink therefore always refuses.

**`ModelBinding` carries the artifact and the devices it is assigned to, and
the devices are a set.**

```rust
pub struct ModelBinding {
    pub artifact: ArtifactRef,
    pub devices: Vec<DeviceOrdinal>,
}
```

Per charter section 2.1 as the ruling of 2026-08-03 states it. The vector is
ordered and the order is the shard order, so a two-device assignment says which
device holds which half rather than leaving a builder to pick, and a set of one
is the ordinary case with no special shape. **An empty set is a parse error
rather than a default**, because a binding assigning no device is a
declaration the operator did not finish, and defaulting it to device zero is
the placement decision this crate exists not to make. Whether the devices
exist, whether they can reach each other, and whether the backend can shard
across that many are all admission's, per `weaver-spu-Spec` section 3, and
this crate answers well-formed and nothing more.

**`GateInstruction` carries the access rule and not the socket, ruled
2026-08-15.** Where a socket sits is the program's and only who may pass it is
the operator's, per `weaver-gate-PRD` section 2, so the instruction the
operator writes carries the rule alone and the raise directive carries the
socket beside it as a second field. **Two fields because two authors**, and a
single one would put the operator's name on a value they do not choose. The
grouping survives the loss: `gate_instruction` stays a named group rather than
collapsing to a bare rule, on the same pattern `spu_instruction` takes, so a
field the gate workflow adds later has somewhere to land.

**`SpuInstruction` and `GateInstruction` are defined here and are the same types the
wire carries, and the feature gate never reaches them.** The `config` feature
gates the parser and the `parse` surface alone, and never a type: the two are
wire types the loop 0 directives carry, so they compile with the feature off,
which the harness's featureless link constructs directives against, per
`weaver-harness-Spec` section 1. A build with the feature off holds every type
of this crate and no YAML dependency, which is section 1's unconditional-wire
sentence stated as the gate's mechanical scope. Both travel two paths, into the
config from the operator and
across a seam inside a directive, and one type for both is what keeps admin from
re-encoding what it validated. `ToolName` is a name today and gains its element
type with the tool workflow, because it elects from `tool-trait`, which
`weaver-traits-PRD` section 3.1 holds blocked.

**Validation is a total parse into a typed value, and admin performs it.** The
crate exposes `parse` and nothing partial: no builder, no field-by-field accessor
over a half-read document. A partially valid config is the shape that lets a load
proceed on half a declaration, and the type system is where that is prevented
rather than in admin remembering to check.

```graph
node: types-config-parse-total
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-types
to: types-config-parse-total
```

What admin adds beyond the parse is
existence, that the model artifact resolves, the sink exists or its creation flag
is set, and the boundary is as the operator wrote it, per `weaver-admin-PRD`
section 4.3. The parse answers well-formed and the checks answer real, and this
crate owns the first only.

**A field no organ registered refuses rather than being ignored.** Unknown is measured
against the declared surface, per `weaver-types-PRD` section 2.1. **The derive this
crate carries denies unknown fields against one fixed type, which is the mechanism for a
fixed surface, and it is what holds the property today.** It stands until the spec round
that charter section names, and what survives that round is the property rather than the
derive. The three sites below that name the derive say the same and are qualified rather
than rewritten, because the representation is that round's to change and not this act's.
An ignored field is an operator's declaration silently discarded, and a typo in
`permission-mode` that parses as an unknown field and vanishes is the failure this
rejection exists to prevent.

```graph
node: types-unknown-key-refuses
kind: assertion
tag: perturbation

edge: asserts
from: weaver-types
to: types-unknown-key-refuses
```

## 3. Peer identity and the authorization predicate

The identity type, the rule it is judged against, and the one policy function, per
charter section 2.2, drawn by the two seams that judge a peer by credential: the
gate's client socket, governed by `weaver-gate-world-contract`, and the
coordination socket the harness binds, governed by
`weaver-admin-harness-contract`. The second was the operator surface until the
recut of 2026-08-05 retired it, and the coordination seam took its place when the
inversion gave that seam a name and a credential to check.

```rust
pub struct PeerIdentity {
    pub uid: u32,
    pub gid: u32,
    pub pid: i32,
}

pub struct AccessRule {
    pub allowed_uids: BTreeSet<u32>,
    pub allowed_gids: BTreeSet<u32>,
    pub denied_uids: BTreeSet<u32>,
}

pub fn authorized(peer: &PeerIdentity, against: &AccessRule) -> bool
```

**The rule is a type here rather than a shape each consumer invents, because that
is the whole of the charter's carve-out argument.** `weaver-types-PRD` section 2.2
rests the exception on one shared definition being the only way separate processes
provably enforce the same rule, and a predicate that took a rule shaped by its
caller would deliver a shared signature over two different rules, which is the
disagreement the carve-out exists to prevent. The three sets are what the two
consumers need and no more: the gate admits front-end principals by uid or group
and excludes the agent uid, and the coordination seam admits root alone, per the
inversion of 2026-08-05, which excludes the agent uid by the same reading. The
operator surface was the second consumer until that date and retired with the
service account it admitted to.

**Denial wins over permission, and the ordering is the security property.**
`authorized` returns false if the peer's uid is in `denied_uids`, whatever the
allow sets say. The gate's predicate must exclude the agent uid even where a
broad group grant would otherwise admit it, per `weaver-gate-PRD` section 2, and a
rule evaluated permission-first would let a group membership readmit the one
principal the boundary exists to keep out.

```graph
node: types-denial-precedes-permission
kind: assertion
tag: perturbation

edge: asserts
from: weaver-types
to: types-denial-precedes-permission
```

**The predicate is a free function over data and reaches nothing.** No file, no
environment, no clock, no global. Where the rule comes from, how a failure is
handled, and what happens on refusal live in the consuming crates, per charter
section 2.2, and a predicate that loaded its own access list would have taken the
first of those back.

```graph
node: types-one-policy-function
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-one-policy-function
```

**`pid` is carried and is never the basis of a decision.** `SO_PEERCRED` reports
it, so the type would be lying by omission to drop it, and it is unsound as an
authorization input because a pid is reused. It exists for the record and for a
diagnostic, and the predicate ignores it. A builder tempted to key on it is the
reason this sentence is here rather than in a comment.

**`PeerIdentity` derives `Serialize` and deliberately not `Deserialize`, and this
is a compile-fail pin.** The type's whole point is that the kernel supplies it and
the peer never asserts it. A derived `Deserialize` is the machinery for
constructing a peer identity from bytes that arrived over a socket, which is one
careless call away from the exact substitution `SO_PEERCRED` exists to prevent.
Serialization stays, because a refusal that names the peer it refused is worth
recording.

```graph
node: types-peer-identity-no-deserialize
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-types
to: types-peer-identity-no-deserialize

edge: grounds
from: types-peer-identity-no-deserialize
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**The threat walk.** This Spec names the adversary each security mechanism
defeats and derives that mechanism's test from the attack, which is how a
perturbation test under apex section 11 becomes a scenario rather than an
assertion. The adversary is a
process on the host that is not a front-end principal and dials one of the two
named sockets: an elected tool running as the agent uid reaching for the agent's
own mouth, or the same tool reaching the coordination socket its harness binds.
The mechanism is
that both sockets are named and therefore dialable by anything that can resolve
the path, so admission cannot rest on reachability and must rest on identity. The
kernel supplies that identity rather than the peer asserting it, which is what
`SO_PEERCRED` is for and what the missing `Deserialize` keeps true, and the
predicate judges it by one shared rule so that two processes enforcing one
boundary cannot disagree.

**The walk yields tests at two levels, and each names the crate that owns it.** In
this crate, over values: `authorized` returns false for a peer whose uid is the
agent's against a rule whose allow sets include the agent's group, confirmed by
watching it return true when denial stops preceding permission. In the gate's
crate, over behavior: a connection from the agent uid is refused at accept, before
any content is read, confirmed by watching content reach the harness when the
predicate is weakened. The second is named here because the rule is defined here
and owed by `weaver-gate-Spec`, which is the G7 shape read forward rather than a
test this crate can pretend to run.

## 4. The organ wire vocabulary

**What this section owns is loop 0's definitions in full, plus the carriage of
anything the envelope carries,** which is a boundary the act of 2026-08-02
made visible and worth stating rather than leaving to be inferred. Loop 0's
four definitions are shaped here entire. `turn-frame` and `fault-report` are
shaped here only as far as the envelope carries them, their payload variants
and, for the frame, the election section 6 holds, because
`organ-envelope`'s representation is this document's. **The token trio is held
at section 4.4 and its encoding is not this document's**, a division the act of
2026-08-08 made after an earlier wording deferred both together. It rides the
decode socket, which is not an organ channel per `weaver-spu-PRD` section 13.2,
so the hot-path measurement elects what it looks like on the wire. What cases it
carries is the decode contract's determination and this crate holds it, because
this crate holds types rather than creating them. A later workflow reads that
boundary and knows which side its vocabulary lands on before it asks.

Four of the charter's nine definitions, then: the envelope every organ channel
carries, and loop 0's trio named for the loop whose traffic it carries rather than
for a sender, per the naming ruling of 2026-08-01.

### 4.1 The envelope

```rust
pub struct OrganEnvelope {
    pub exchange: ExchangeId,
    pub position: Position,
    pub payload: Payload,
}

pub struct ExchangeId {
    pub opener: Opener,
    pub ordinal: u64,
}

pub enum Opener {
    Admin,
    Harness,
    Spu,
    Gate,
}

pub enum Position {
    Open,
    Continue,
    Close,
}

pub enum Payload {
    Directive(LifecycleDirective),
    Answer(LifecycleAnswer),
    Refusal(LifecycleRefusal),
    Frame(TurnFrame),
    Fault(FaultReport),
}

pub enum RefusingOrgan {
    Spu,
    Gate,
}
```

**`Position` is three, per `weaver-organ-channel` section 1**, which defines a
message's position in its exchange as open or continue or close. A two-variant
reading would serve the minimal exchange, which opens and closes at once, and
would have nothing to say about a directive whose answer arrives after an
intermediate message, which the channel's own layer permits.

**`ExchangeId` is the opening party and an ordinal**, per the same section, and
`Opener` enumerates the four parties that hold an organ channel. Naming the party
rather than carrying a bare bit is what lets a capture read as itself, and the
enum grows when an organ does, which is a floor edit in the act that charters the
organ.

**The payload is a typed enum rather than octets, which is what makes the
encoding of 4.3 possible.** An earlier draft of this Spec carried
`payload: Vec<u8>` with a separate kind discriminant, on the reasoning that a
generic envelope would push each seam's vocabulary into the floor's signature.
The reasoning failed against the naming ruling: loop naming means the payload
types live on the floor already, so naming them here adds no dependency and
removes a layer of encoding. It also removed a defect, since octets inside a JSON
envelope render as an array of numbers by default, tripling the size and
destroying exactly the legibility the JSON election rests on.

**`Frame` is that rule's first exercise, from the token workflow's gate act of
2026-08-02.** The gate's turn exchanges cross the same organ channel the
lifecycle directives do, so the frame enters this enum rather than taking a
carrier of its own, and every consumer's match sees the addition, which is
what the rule below is for. **Its shape is elected, per the operator's ruling
of 2026-08-12: the frame carries its octets base64-encoded in one member, and
the measurement the earlier deferral waited on is not needed.**

```rust
pub struct TurnFrame {
    pub octets: String,
}
```

The member is the line's octets encoded base64, both directions, one
definition per the charter. The encoding is RFC 4648 section 4's standard
alphabet, padded, with no line breaks and no interior whitespace, and the
decode refuses input the encode would not produce, so one octet sequence has
exactly one carried form. The encoding's implementation rides with the type,
a floor obligation the wiring act lands, so one implementation holds the
canonical form for every party rather than two crates agreeing by luck. The
deferral offered two honest answers, a
splice of the line as it stands or an encoding that survives arbitrary
octets, and held them for a measurement over real client traffic. The
constraint set decides without one. The gate reads no frame, per its opacity
rule and the `gate-client-content-unread` assertion, so no party validates a
line before it crosses: the world contract names UTF-8 NDJSON as the
client's format and names the harness as the party that refuses a line that
does not parse, which admits arbitrary octets onto the seam by construction.
A splice therefore fails exactly when a client misbehaves, and it fails at
the wrong layer: an unparseable member makes the envelope undecodable, and
an undecodable envelope is a channel fault that ends service, so a hostile
line would convert a refused turn into a dead channel. The encoding that
survives arbitrary octets is the only answer left, and base64 is its boring
form: ASCII inside the JSON envelope, about a third larger than the line
rather than the tripling a numeric array costs, on a channel that carries
one frame per turn rather than the decode seam's per-token stream, so the
hot-path concern the deferral carried does not reach it. Encoding is
carriage rather than reading, a byte-blind transform parsing nothing, so the
gate's opacity holds. The harness decodes the member back to octets before
the parse question the gate's turn half owns, and what an undecodable member
means behaviorally is that act's to state, the representation alone being
this document's.

```graph
node: types-frame-survives-arbitrary-octets
kind: assertion
tag: perturbation

edge: asserts
from: weaver-types
to: types-frame-survives-arbitrary-octets
```

**`Fault` enters on different grounds and the difference is worth stating.**
It is not a loop's vocabulary, so the rule below does not reach it: a fault
report is what any organ hands the harness across whatever channel it holds,
and it enters this enum because the gate's channel is an organ channel and
the gate has no second socket to carry it. The decode socket carries the same
floor definition inside its own trio rather than inside this envelope, which
is what makes `fault-report` one definition with two carriages instead of two
definitions that would drift.

**A later loop's vocabulary enters this enum in the act that charters that loop,**
which is the same loudness the trio's own case sets carry: one owner, contracts
drawing rather than growing, and a floor edit every consumer's match then sees.

### 4.2 The trio

```rust
pub enum LifecycleDirective {
    Enter { payload: EnterPayload },
    Leave,
    Stop,
    Admit { instruction: SpuInstruction },
    Release,
    Raise { instruction: GateInstruction, socket: PathBuf },
    Lower,
    Load { agent: AgentName },
    Unload { agent: AgentName },
    Validate { agent: AgentName },
    List,
    Show { agent: AgentName },
}

pub enum LifecycleAnswer {
    Ready,
    Left,
    TurnAborted { turn: TurnKey },
    AtRest,
    Admitted,
    Released,
    GateReady,
    GateStopped,
    Validated,
    State { state: AgentState },
    Agents { agents: Vec<AgentSummary> },
}

pub enum LifecycleRefusal {
    Unauthorized,
    Malformed,
    NoSuchAgent,
    CarriedWork,
    OutOfOrder,
    DescriptorsUnusable,
    BoundaryUnverified,
    ConfigInvalid { field: Option<FieldName> },
    ArtifactUnresolvable,
    ArtifactUnreadable,
    DeviceCannotAdmit,
    NoResidency,
    BindFailed,
    PriorUnitUnreaped,
    OrganRefused { organ: RefusingOrgan, reason: Box<LifecycleRefusal> },
    ActivityNotAtRest,
    StateNotObservable,
}

pub struct EnterPayload {
    pub session: SessionId,
    pub run: RunId,
    pub spu_instruction: SpuInstruction,
    pub binding: EnterBinding,
    pub state_election: StateElection,
    pub state_store: StateStore,
    pub restore: Option<Restore>,
    pub stack: BTreeMap<String, String>,
}

pub enum EnterBinding {
    Serving { gate_instruction: GateInstruction },
    Diagnostic,
}

pub struct FaultReport {
    pub case: FaultCase,
    pub account: Box<serde_json::value::RawValue>,
}

pub enum FaultCase {
    DeviceFaultDuringGeneration,
    ResidencyDegraded,
    ReadoutFaultWhileElected,
    ListenerLost,
    ClientConnectionFailedMidTurn,
    AdmissionFailingSystematically,
    RecorderCommitPressure,
    StreamWriteFailed,
    OrganDeathObserved,
    MessageRecordUndecodable,
}
```

**`state_store` rides the enter resolved, beside the election, as of
2026-09-04.** The config holds it as an option whose absence means the
embedded engine, per section 2, and the payload holds it as admin resolved it
at inventory, so the harness never re-derives an absence and the load event it
authors names the engine and, under the service engine, the database and role,
per `weaver-trace-PRD` section 3.1. `database` and `role` are present in the
resolved value exactly where the engine is the service one, the cross-field
rule having been judged before any process existed, and the member's own
vector carries the same three, per `weaver-admin-Spec` section 6, so the two
parties that need them read one resolution.

**`restore` and `stack` ride the enter as of 2026-09-04, per issue #432.** `restore` is
the declaration's election carried whole where it made one, so the harness knows the
session stands from a record and where the cut fell, and it names the parent on the load
event without opening the record, which admin preloaded into the member before the enter
per `weaver-admin-Spec` section 6. `stack` is the digests of the organ binaries admin
started, keyed by the binary's name, so the load event names the stack that ran it and a
record is sufficient for its own conditions without a deposit beside it, per
`weaver-trace-PRD` section 3.1. Both are admin's facts and the harness authors them as
it authors the store's.

**`EnterBinding` is the kind resolved, and a directive disagreeing with its
kind is unrepresentable rather than refused.** The config holds the kind as
the operator may state it, an option whose absence means serving, and the
payload holds it as admin resolved it, so the resolution point is visible in
the types: what crosses the seam has already been decided. The gate
instruction rides inside the serving case, which answers the question
`weaver-admin-harness-contract` section 8 left to this round: no refusal
catches a directive whose members disagree with its kind, because the shape
leaves no such directive to construct. A diagnostic enter has no field for
the instruction and a serving enter cannot omit it, so the wrong pairing is
a struct that does not exist, on the same ground as the one-case outcome of
`weaver-harness-Spec` section 3. The grouping is representation rather than
a term of its own, per the standing rule that the draws name the definitions
and not the grouping, and the vocabulary node stays `binding-kind` alone.

```graph
node: types-enter-binding-disagreement-unrepresentable
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-types
to: types-enter-binding-disagreement-unrepresentable
```

**`PriorUnitUnreaped` is added because `BindFailed` was answering for two
conditions.** A unit whose process exited non-zero leaves its name registered
with the manager, and every later start under that name refuses until it is
reaped. That is what refused the load, and it is a different fact from a socket
that would not bind, which is what `BindFailed` names.

**What the case claims is exactly what the state reports and no more.** It says
a prior process exited non-zero and its name is still held. It does not say the
worker never bound, or never started, or died before serving, because `failed`
carries none of that: a unit that bound its socket, served, and exited non-zero
later reads `failed` too. An earlier wording of this clause said the worker was
never there, which is the error `weaver-admin-systemd-contract` section 3 warns
against in the neighbouring value, a program rendering a state as one of the
conditions it covers.

**It is expressible because it is the one thing the boundary says plainly.**
`weaver-admin-systemd-contract` section 3 measures what the init system reports
and finds most of it ambiguous: a duplicate unit name and a malformed property
fail with the same status, and `inactive` covers three conditions. `failed`
covers one, a unit whose process exited non-zero, so a refusal resting on that
value rests on the only reading the boundary gives without inference.

**The case is not derived from the start ask's status**, which the same section
measures as unable to say which failure it was. It comes from the state ask that
follows, which is where a program may ask a narrow question and get a narrow
answer.

**`FaultReport` is two members, and the custody rule of apex section 5.2 is
the whole argument for the split.** The `case` is what the harness itself
consumes: the judgment of what a fault means for the turn and the residency is
the harness's to make, so the fact it judges is typed, exhaustive, and closed.
The `account` is the reporting organ's own rendering of what happened, opaque
here by construction: a raw value crosses verbatim and splices verbatim, so
the floor carries no organ's descriptive vocabulary and a later organ's richer
account grows no shared type. This is the standing exception of apex section
5.2 corrected in the act that corrects it, the fault path now running the same
direction as the generation measurement.

**The case set is the three charters' closed enumeration and no case is this
document's.** Three from `weaver-spu-PRD` section 13.10, three from
`weaver-gate-PRD` section 13.4, five from `weaver-harness-PRD` section 5,
which closed the corpus-wide set across all three organs for the seams that
exist. A twelfth case is a charter act before it is a code change, and the
code act that typed these found the charter one short of its own crate's
standing practice, the assembly fault, which is the further-case rule
exercised in the act that stated it rather than a rule waiting for its first
test. And the
harness's own five ride the same shape although they cross no socket, because
the same shape serves the wire and the `fault` event's payload and electing it
twice would be two shapes for one fact.

**The reporting organ names its own case, and the harness classifies
nothing.** The report arrives whole, per the contracts' wording that an
emission carries a `fault-report` naming a case of its charter's enumeration,
so a case exists before any `fault` event is authored and no raw account ever
reaches the record wrapped without one. A report the harness would have to
diagnose before it could file it would put the judgment on the wrong side of
the seam, the organ being the party that knows.

**No member names the raiser and no member names the turn.** Which subsystem
raised a fault is the trace envelope's field, per `weaver-trace-PRD` section
3.2, and which turn it belongs to travels as the seam's context per apex
section 5.2, so a member for either would be one fact in two places. The
payload carries the what and never the where.

**All three enums are exhaustive, and the absence of `#[non_exhaustive]` is the
loudness this section claims.** The attribute would force every out-of-crate
consumer to carry a wildcard arm, and a case added later would land in that
wildcard silently, which is the opposite of the property the naming ruling bought.
Exhaustive, a new case breaks every consumer's match at compile time, in the same
act that edits the floor, and every contract's drawn subset is re-read by a human
because the compiler made them look.

```graph
node: types-wire-enums-exhaustive
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-types
to: types-wire-enums-exhaustive

edge: grounds
from: types-wire-enums-exhaustive
to: axiom-contract-is-a-complete-interface
```

An earlier draft of this Spec carried the
attribute beside this same argument, which the review seat caught as the
contradiction it was.

**One directive type for loop 0, carrying every case that crosses any of its four
seams, and each contract's vocabulary clause names the subset that crosses its
own:** enter, leave, and stop at coordination, admit and release at residency,
raise and lower at the gate, and the verbs with the observations at the operator
surface. The answer and the refusal follow the same rule.

**Every directive receives exactly one answer, and the mapping from directive to
answering case is stated because the operator contract requires that and a
builder would otherwise invent it.** The case is determined per directive, and
for `Stop` by what it interrupted. `Enter` answers
`Ready`, `Leave` answers `Left`, `Stop` answers `TurnAborted` or `AtRest`,
`Admit` answers `Admitted`, `Release` answers `Released`, `Raise` answers
`GateReady`, `Lower` answers `GateStopped`, `Validate` answers `Validated`,
`Load`, `Unload`, and `Show` answer `State`, and `List` answers `Agents`. Eleven
of the twelve have a single answering case. `Stop` has two, `TurnAborted` or
`AtRest`, selected by whether a turn was in flight, and both are clean closes
rather than a refusal, per `weaver-admin-harness-contract` section 3. Any
directive may answer a `LifecycleRefusal` instead, which is the second half of
what one answer per request means. `Validated` exists because validation reports an
outcome without transitioning anything, per `weaver-admin-PRD` section 4.3, and
answering it with a state would report a transition that did not happen.

**A receiving party matches its own cases and refuses the rest as `OutOfOrder`,
which is a real obligation rather than a formality.** The gate receiving an
`Admit` is not a case the gate implements, and the contracts already rule that a
directive out of order for the channel's state is refused and not queued. The enum
being wide is what makes that refusal expressible in the type the seam already
carries, and the per-seam test of section 5 is what confirms each party wrote the
refusing arms rather than a wildcard.

**`OrganRefused` boxes an inner refusal because the aggregate carries it
unchanged.** `weaver-admin-harness-contract` section 6 requires a refusing organ's
reason to reach admin without translation, so the harness wraps rather than
re-encodes, and the box is what keeps the enum's size from being set by its
deepest case. **It names a `RefusingOrgan` rather than an `Opener`**, because
only the SPU and the gate refuse inside a fan-out: admin does not refuse to
itself and the harness returns the aggregate rather than appearing inside it, so
reusing the four-case `Opener` would let a well-typed aggregate claim that admin
refused as an organ.

**The refusal cases are drawn from the merged contracts and the set is closed at
this crate.** Whether the SPU's admit cases needed a type of their own was the
cell `weaver-spu-PRD` section 10 held, and the naming ruling settled it as
extension: they are loop 0 refusals because they refuse loop 0's directives.

**`StateNotObservable` joins the set on 2026-08-06, and it refuses a question
rather than an act.** Every other case here says an act could not be performed.
This one says an answer cannot be formed: the party that knows an agent's
lifecycle state is the harness, which holds the run, and
`weaver-admin-harness-contract` section 3 charters enter, leave, and stop with
no observation, so nothing asks it. Admin can read residency from the init
system and residency is not lifecycle state, per `weaver-admin-Spec` section 3,
so `show` and `list` refuse with this case rather than construct an `AgentState`
the corpus has no source for.

**It is a marker with a scheduled death, which is why it is a refusal and not a
new answer.** Growing `AgentState` to carry a service manager's vocabulary would
settle this crate's vocabulary from another party's representation, which gate
G2 forbids, and adding an answer case for residency would make permanent a shape
the observation exchange is expected to replace. A refusal states the absence
plainly, reaches an operator as a typed value rather than as an empty result,
and leaves exactly one thing to delete when the exchange lands. **Whoever
charters that exchange retires this case in the same act**, and a corpus still
carrying it afterwards has left a marker for a gap that closed.

### 4.3 The encoding

**Loop 0's traffic is JSON, one envelope to one message, elected against the
charter's criterion.** `weaver-types-PRD` section 2.3 states the ground: this
traffic is low in volume, so compactness buys nothing measurable, and diagnostic
in audience, read from a capture when a load refuses unexpectedly.

```graph
node: types-loop0-encoding-json
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-loop0-encoding-json
```

**The tagging follows the same mechanical test `weaver-traits-Spec` section 3
states, and the two floor Specs share it so they cannot drift.** A fieldless enum
is a plain renamed string. An enum whose every variant is struct-shaped or wraps
a struct is internally tagged. An enum with any variant wrapping a primitive, a
sequence, or another tagged enum is adjacently tagged, because internal tagging
cannot represent those shapes and fails at serialization rather than at compile
time. **The same arm takes an enum with a variant wrapping a struct that
carries a spliced member**, and this clause is the trio's code act finding a
failure the rule's rationale did not name: internal tagging buffers the
content on the read side and the buffer cannot represent pre-serialized JSON,
so the round trip fails at deserialization rather than at serialization,
measured in this workspace as an invalid-type error at the splice. A wire type
one party can write and the other cannot read is not a wire type, so the arm
extends to the read side's failures rather than the write side's alone.

```graph
node: types-tagging-test
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-tagging-test

edge: asserts
from: weaver-traits
to: types-tagging-test
```

Applied here: `Position`, `Opener`, `RefusingOrgan`, and `FaultCase` are
fieldless and serialize as plain renamed strings, the rename being
`snake_case` as everywhere in this vocabulary, so a case crosses as
`residency_degraded` and never as its Rust spelling. **The trio is internally
tagged**, which is why every case carrying a value in 4.2 takes a struct
variant, `Load { agent }`
rather than `Load(AgentName)`. **`Payload` is adjacently tagged**, `#[serde(tag =
"kind", content = "body")]`, because its variants wrap enums that carry a tag of
their own. **The token trio of section 4.4 divides under the same test**:
`TokenDirective` and `TokenRefusal` are internally tagged, their value-carrying
cases struct-shaped, `Finish` is fieldless and a plain renamed string, and
`TokenAnswer` is adjacently tagged under the spliced-member arm, two of its
cases wrapping the vocabulary's two `RawValue` carriers: `Generation`, whose
`request` and `measurement` are both spliced, and `FaultReport`, whose
`account` is.

**The envelope's layout is stated rather than left to a reader.** Nothing is
flattened. `exchange`, `position`, and `payload` are three members of one object,
and the adjacent tagging nests the payload's own tagged object under `body`, so
no two layers can contribute one key:

    {"exchange":{"opener":"admin","ordinal":7},
     "position":"open",
     "payload":{"kind":"directive","body":{"kind":"load","agent":"alpha"}}}

**Both halves of this election were verified against serde 1.x rather than
reasoned from memory, because an earlier draft got both wrong.** That draft
tagged `Payload` and the trio with the same `kind` name, which emits two `kind`
members into one object and fails to deserialize with a duplicate-field error,
and it used newtype variants, which fail at serialization for any case wrapping a
name or a sequence. The wire shape section 0 claims this Spec settles has to
round-trip, and the shape above does, including the sequence-carrying cases.

**This election does not reach the decode seam and must not be read as reaching
it.** Decode traffic is the hot path, its volume is per token rather than per run,
and its encoding is the token workflow's election with a measurement behind it.
Decode does not share this channel, per the decoder-cut ruling of 2026-08-02
recorded at `weaver-spu-PRD` section 10: it takes its own socket, owned by that
crate, so the head-of-line question this sentence once held open is closed.

**The envelope is length-free, because the socket type carries boundaries.**
`weaver-organ-channel` section 2 requires one write to be one read and rules that
the property comes from the socket type rather than from framing above it, leaving
the type to the Spec. The election is `SOCK_SEQPACKET`: it preserves message
boundaries and connection semantics together, where `SOCK_DGRAM` on a Unix socket
would preserve boundaries but invite the datagram reading that document explicitly
disclaims, and `SOCK_STREAM` would push a length prefix into every contract that
draws the channel, which is the layer violation that document exists to prevent.

**The election binds the crates that create the pairs, and it is owed to their
Specs.** This crate opens no socket. `weaver-admin-Spec` creates the coordination
pair and `weaver-harness-Spec` creates the residency and gate pairs, per their
charters, and each carries this election rather than re-deciding it. Naming the
landing sites here is what keeps a decision made in one Spec from reading as
settled everywhere while binding nothing.

**One write is one read only while the reader's buffer covers the message, so the
election arrives with an obligation.** A short read on `SOCK_SEQPACKET` truncates
silently and discards the remainder unless the receiver checks `MSG_TRUNC`. The
receiving crates therefore size their buffer against a **maximum envelope of 64
kibibytes** and treat a truncation flag as a channel fault rather than a message,
because a silently shortened directive is the failure mode the boundary property
was elected to prevent. The bound is generous against loop 0's traffic, whose
largest case is an enter payload of identifiers, and it is stated as a number so
that a builder sizing a buffer has one to use.

```graph
node: types-socket-seqpacket
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-socket-seqpacket

edge: grounds
from: types-socket-seqpacket
to: axiom-floor-is-vocabulary-behavior-is-socket

node: types-envelope-bound-64k
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-envelope-bound-64k

edge: grounds
from: types-envelope-bound-64k
to: axiom-floor-is-vocabulary-behavior-is-socket
```


### 4.4 The token trio

**This crate holds types rather than creating them.** What shape a type takes is
determined by the demand that construction puts on it, and the party under that
demand is the one that determines it. The decode contract's sections 2 and 5 are
that determination for the token trio, and this subsection is where the floor
holds the result. The charter said as much when it declared the trio: the cases
are the decode contract's enumeration, one owner and one drawer today.

**The trio is held here now because the demand exists now.** Section 9 of
`weaver-spu-Spec` requires the decode seam to answer a `token-refusal` and the
crate cannot write one, and apex section 3 steps 5 and 6 put the seam inside the
turn the program is built to complete, so no loop 1 avoids it however simple. An
earlier reading of this section deferred the shape along with the encoding. The
two come apart, and the deferral belongs to the encoding alone.

```rust
pub enum TokenDirective {
    Open { session: SessionId, messages: Vec<Message>, column_ask: bool },
    AppendAndGenerate {
        turn: TurnKey,
        delta: Vec<Message>,
    },
    ReFeed {
        turn: TurnKey,
        rendered: String,
        path: Vec<u32>,
    },
    Cancel { turn: TurnKey },
    Flush { keep: u64 },
    Elide { from: u64, to: u64 },
}

pub enum TokenAnswer {
    Opened,
    Token { token: u32, piece: String },
    Field { position: u64, ranked: Vec<Candidate>, realized: u32 },
    Column { position: u64, layers: Vec<Vec<f32>> },
    Generated(Generation),
    ReFed(Generation),
    AtRest,
    Flushed {
        resident_before: u64,
        resident_after: u64,
    },
    Elided {
        resident_before: u64,
        resident_after: u64,
    },
    Fault(FaultReport),
}

/// What a seam turned away, as the record carries it.
///
/// **The seam is named and the case is the seam's own.** A consumer
/// dispatches on the variant, which is what typing this here buys over a
/// reason field the trace would have to carry as prose.
pub enum RefusalRecord {
    Decode { asked: TokenAsk, refusal: TokenRefusal },
    Lifecycle { asked: LifecycleAsk, refusal: LifecycleRefusal },
    Classify { refusal: LabelRefusal },
}

/// Which ask a decode refusal answered, so the record says what was
/// refused and not only why.
#[serde(tag = "ask", rename_all = "snake_case")]
pub enum TokenAsk {
    Open,
    AppendAndGenerate,
    ReFeed,
    Cancel,
    Flush { keep: u64 },
    Elide { from: u64, to: u64 },
}

/// Which lifecycle ask a refusal answered.
#[serde(tag = "ask", rename_all = "snake_case")]
pub enum LifecycleAsk {
    Enter,
    Leave,
    Stop,
    Admit,
    Release,
}

pub enum TokenRefusal {
    NotOpen,
    OutOfOrder,
    Overflow { resident: u64, requested: u64, capacity: u64 },
    MalformedDelta,
    UnremovableSpan { from: u64, to: u64, prefix: u64, resident: u64 },
    RefeedPermissionAbsent,
    RefeedPathEmpty,
    ColumnPermissionAbsent,
    ColumnReadoutUnelected,
    ColumnUndeclared,
}
```

**`RefusalRecord` names the ask beside the refusal, and carries a value only
where the record holds it nowhere else.** A refusal alone says why a party
said no. What a diagnostic reader works backwards from is what was asked, so
the variant always says which directive was refused. **What the variant does
not do is reproduce the directive**, and the rule deciding that is one rule
rather than a judgment per case: **a value rides the ask when no other event
carries it, and is named and left alone when one does.**

Applied to the decode seam:

```
Open                  the messages are the identity prefix, and
                      message.system carries them since the prefix act
AppendAndGenerate     the delta is authored as the turn's message kinds
                      before the exchange, so a refused append still has
                      its content in the record
Cancel                the turn is the envelope's field on every event
Flush { keep }        carried: no event holds the cut a flush asked for
Elide { from, to }    carried: no event holds a span that was refused,
                      the elision event recording only removals that
                      happened
```

**Completing the three would be duplication rather than fidelity**, one fact
in two places with no authority named, which this corpus files as a defect.
The record is not lossless about the ask and does not claim to be: it is
complete about the ask's identity and about the values that would otherwise
be lost.

**Classify carries no ask at all**, which is the same rule at its limit. Its
directive is the content, `classify.request` holds it under its own kind,
and nothing of the ask would otherwise be lost.

**`LifecycleAsk` follows the rule and has a gap the rule cannot close.** The
directive is named and the declaration is not reproduced, the load event
carrying the run's posture.

**An enter refused before its bracket is established has no run record**,
because the `load` event is what opens the run and a refusal reaching that
far has not authored one. There is no run to author the refusal into and no
later event that could carry it, whether or not a sink descriptor was taken
before the refusal fell. **The refusal reaches the operator's answer and
nothing else**, which is unchanged by this act and is the whole of what such
a refusal leaves behind. **That is a hole this act does not fill**, named
here so a reader does not take the lifecycle arm as covering it.

**The three arms are the seams that produce typed refusals**, per the
inventory of 2026-08-22: the decode seam, the lifecycle seams which share
one vocabulary, and the classify seam. The gate's `RaiseRefusal` is absent
because it crosses no seam, reaching the harness as
`LifecycleRefusal::BindFailed`, which the lifecycle arm already carries.

**`Elide` names a half-open span and `Flush` names a length, which is the
difference between removing an interior and shortening a tail.** The bounds
are resident positions, `from` inclusive and `to` exclusive, and the pair
describes what leaves rather than what stays: the flush's `keep` says what
the session returns to, and a span says what the session loses. **They take
opposite rules at their edges**, per the decode contract as amended
2026-08-22: an over-large `keep` bounds, and a span describing no removable
region refuses, because a smaller true version of the first exists and of
the second does not.

**`Elided` carries the same pair `Flushed` carries** and for the same
reason: the SPU is the one authority on either count and the harness
authors the record's event from exactly them. Two answers of one shape are
two facts rather than one, so they take two cases rather than a shared case
with a discriminant, on this Spec's standing rule that a name earns its own
variant where a reader would otherwise infer which operation ran.

**The span is state.** These positions index the resident sequence the SPU
holds, never the trace, and no directive on this seam removes anything from
a record.

**The tunable map left this directive in the act that routed it to the
declaration**, per `weaver-spu-Spec` section 8, so nothing sampling-related
arrives with a turn and the value discipline that guarded it moved with it to
section 2. What remains here is a turn's delta, and a delta this seam cannot
serve is `MalformedDelta` as before.

**`Message` is `weaver-traits`' and is drawn rather than restated.** The decode
contract's canonical messages are that type, shaped at `weaver-traits-Spec`
section 3 as a role and its content blocks, and this crate already links that
one as its floor-link. An earlier wording of this subsection named a
`CanonicalMessage` that exists nowhere, which is the naming slip a first reading
of the contract's prose invites: the contract describes the messages as
canonical and does not rename the type.

**`Generation` shapes what the harness reads and splices what it forwards.** The
decode contract's section 2 determines the answer as the generation and its
measurement, the measurement being the token identifiers, the per-token signals,
the timings, the block partition, and the residual reductions where the
residency was admitted with readout elected. The template identity is the
request's member and not the measurement's, per the ruling of 2026-08-17
on #129, and this sentence restates the contract's enumeration rather than
owning it, so it moves when the contract moved.

```rust
pub struct Generation {
    pub emission: String,
    pub finish: Finish,
    pub request: Box<serde_json::value::RawValue>,
    pub measurement: Box<serde_json::value::RawValue>,
    pub resident: u64,
    pub capacity: u64,
}

pub struct Candidate {
    pub token: u32,
    pub probability: f32,
}

pub enum Finish {
    Completed,
    Stopped,
    Length,
}
```

**The split is between what the harness consumes and what it carries through, and
`weaver-trace` states the rule it follows:** what is shaped in a crate is what no
other crate defines. The emission and the finish are consumed here, the first
entering the working structure as the assistant's message and the second closing
the turn, so both are shaped. The measurement and the rendered prompt are
consumed by nothing on the way past and are two members rather than one: each
is forwarded whole into its own box, the measurement into the measurement
event and the rendered prompt into the request event's `rendered`, because one
spliced member per box is what keeps the splice a move and never a transform.
A rendered prompt inside the measurement would reach a box whose accepting
shape carries no such member, and the harness would have to open the splice to
remove it, which is the transform the splice exists to refuse. **Seven of its
satellites are `weaver-trace`'s** and one is `weaver-spu`'s, against a crate that
links one internal dependency, so shaping it here would restate seven types that
already exist with no named authority over either copy, which is the duplication
G5 files as a defect rather than resolves by picking.

**`request` is the model.request content whole, not the rendered prompt alone.**
The custody act of 2026-08-11 makes `model.request` a spliced payload the SPU
renders, so this member carries what that event carries, the turn's delta as
rendered with its template identity and the turn's effective sampling values,
per the ruling of 2026-08-12, rendered by
the SPU because the template and the knobs are the SPU's and assembling them in
the harness would be the transform the splice refuses. It was named `rendered`
for one act, when only the prompt crossed, and the name narrows to the truth the
custody act settled.

**Spliced JSON and not octets, which is a distinction this crate enforced once
already.** `RawValue` carries pre-serialized JSON written in place, so the
measurement is a member of the answer's object rather than a string of bytes
inside it. What section 4.1 refused was double encoding, and splicing is what
avoids it, so that refusal is satisfied here rather than worked around.
`weaver-trace` took the same election for the same reason, its message and fault
payloads being spliced because a tag would wrap the bytes in an object of that
crate's making.

**The SPU renders and the harness authors, which is a ruling this restates rather
than a position this invents.** The fault-carrier ruling already has the SPU
handing over a `fault-report` and the harness authoring it as the `fault` event,
per the decode contract's section 2. The measurement travels that path to the
model events, so no crate holds a second copy of a shape `weaver-trace` owns and
the sole-writer rule is untouched: what the SPU produces is data, and the event
is still the harness's to author.

**The flush confirmation carries both resident counts**, because the SPU
is the one authority on either number and the harness authors the
record's `flush` event from exactly them: the count before the truncate
and the count after, the kept length as it held under the cut's bounds.
A confirmation without the counts would leave the event's payload to a
party that cannot know it, and under a bounded cut the counts are also
where the asking loop learns what its `keep` resolved to.

**The fullness rides every generation, added 2026-08-19 by issue #221's
arc.** `resident` is the session's token count as the generation closed,
terminator included, and `capacity` is the ceiling the load resolved, the
same two numbers the overflow refusal carries after the wall is hit,
carried here so the asking loop sees the pressure before it. Plain counts
with no judgment: when a flush is worth its cost is the loop's business,
per `weaver-state-PRD` section 2.

**Three cases, each one fact.** `Completed` is the family's stop condition
reached, the model's own end. `Stopped` is the generation ended from
outside the model's own signal: the operator's stop directive, or the
session's context capacity reached mid-generation, the resident-length
limit named here explicitly as the non-turn limit so the two ceilings
cannot be read as one. `Length` is the turn's token limit and that limit
alone, `max-tokens-per-turn` reached, added 2026-08-19 by
issue #218's evidence: the cap had exited as `Completed`, so a reader of
the record could not distinguish a finished answer from a cut one, which
is an ambiguity in the one artifact whose reason for existing is that it
never lies.

**`Finish` is shaped here and `weaver-trace` shapes its own, with the harness
converting.** That is the arrangement `SessionId` and `SessionRef` already take,
per `weaver-trace-Spec` section 1, that crate linking no internal crate on purpose
and the harness converting at one call site. Two names for one fact with a named
conversion point is the corpus's existing answer for a floor type the trace needs,
and a different answer here would be a second one.

**What the harness must check, because the type cannot.** A spliced member is
opaque to the compiler, so the measurement's conformance to what the trace's model
events accept is the harness's to enforce at the submit call, in the same place the
kind-to-payload pairing is already enforced by admission rather than by serde. The
election buys thinness in the floor and pays for it there, and naming where it is
paid is what keeps the payment from going missing.

**The five refusal cases are the contract's section 5 and this document adds
none.** The session is not open or residency is not confirmed for the ask. The
directive is out of order for the seam's state, a second generation or a
mid-flight flush above all. The session cannot take the next delta, and the
overflow carries the session's own account of itself because the harness decides
what a full context means and cannot decide it without the numbers. The delta is
malformed for the family. The elision's span describes no removable region, and
the refusal carries the span beside the bounds it was judged against so the
loop learns which of the four edges it crossed. **`OutOfOrder` is the same word
loop 0's refusal carries and is not the same case**, because the states it is
judged against are the decode seam's, which is why the trio carries its own
rather than drawing loop 0's.

**A cancel with nothing in flight answers `AtRest` rather than refusing**, per
the contract's section 2, an answer because it is not a failure of the ask.
**The fault report takes no answer at all, and its case is `Fault`**, per the
decode contract's second ruling of 2026-08-12: the report is the seam's one
SPU-originated emission, and the `Received` case an earlier shape of this trio
carried to close it as an exchange left with the receipt, a case nothing
constructs being the reserved slot apex section 7 forbids in data. The
report's own wire case arrived with `FaultReport`'s shape, the election
section 6 records as closed, and it is a case of the answer per the streaming
ruling's own logic, the SPU-to-harness traffic being one enum, with the prose
fact the type cannot carry stated here: the case closes no exchange and
answers nothing, the trace entry being the acknowledgment, per the contract.

**The stream is a case of the answer and never a fourth type**, per the
contract's section 2 as of the streaming ruling of 2026-08-11. `Token` is the
intermediate: any number cross before the close, each carrying one drawn
identifier and its rendered piece, and none closes the exchange, which stays
`Generated`'s alone. The identifier is a bare `u32` because the wire's numbers
are, the backend's own token type staying on the SPU's side of the seam, and
the piece is the family's rendering of that one token, so a consumer
accumulating pieces holds the emission as it grows and a consumer ignoring
every intermediate reads the seam the batch way.

**The encoding stays deferred and the deferral is the encoding's alone.** The
hot-path measurement elects it, per section 4.3's boundary rule and
`weaver-spu-Spec` section 12, and the measurement is taken against real decode
traffic which the first demonstration produces. Until that traffic exists the
seam carries this trio as JSON, the same encoding loop 0 carries, and **that is a
provisional election with a stated trigger rather than an answer**: it is
reconsidered when the first demonstration produces traffic to measure, and a
build that has produced that traffic and not reconsidered has an open election
reading as a settled one. The decode socket carries octets at its boundary
already, so the election changes one function on each side and nothing above it.

**No record is added here.** The trio's nodes are the charter's, declared at
`weaver-types-PRD` section 2.3 with their `defines` edges, and the instrument
that buys the ordering is `weaver-spu-Spec` section 9's, which names the
assertion and the seam it is watched on. A node restated here would give the
mapper two sources for one record.

**The segment series, added 2026-08-20 per the decode contract's section 1
and issue #236.** A frame whose serialized form exceeds
[`MAX_ENVELOPE_BYTES`] crosses as a series: one preamble datagram spelling
`{"segments":N,"bytes":M}`, then exactly N datagrams of raw octet slices in
order, each within the envelope bound, whose concatenation is the M bytes
of the one serialized frame, parsed as though it had crossed whole. The
preamble is recognized by what it lacks, and the absence is a reserved
shape rather than a guess: every trio frame carries `kind`, so a frame
without one is either exactly the preamble - two members, both unsigned
integers, nothing else - or a channel fault, and a kindless frame that is
not the exact preamble refuses as undecodable rather than being read
around. The preamble validates before any slice is read: `bytes` must
exceed the envelope bound, or the sender had no series to send, and must
not exceed the total bound. `segments` must equal exactly the datagrams
the byte length requires at the envelope size, so a count inconsistent
with `bytes` - too many for the length, too few to carry it, or past the
hundred-twenty-eight the total bound admits - refuses before the first
slice, and the slices must total exactly `bytes`, a short, long, or
interrupted series a channel fault, never a partial message. The
total bound is eight mebibytes, `DECODE_MESSAGE_BOUND`, elected against the
close's growth: a four-thousand-token turn's close measures in the
hundreds of kibibytes, the bound covers a sixteenfold cap without
renegotiation. A frame within
the envelope crosses as it always did, so the series costs nothing where
it is not needed, and either end may send one, the closes being the only
frames that grow today.

### 4.5 The label trio

**The demand is the classify contract's**, sections 2 and 5, drawn 2026-08-19
with the classifier's charter, and this subsection holds the result the way
4.4 holds the token trio's: the party under the construction's demand
determines the shape and the floor holds it.

```rust
pub enum LabelDirective {
    Classify {
        turn: Option<TurnKey>,
        content: String,
    },
}

pub enum LabelAnswer {
    Ready,
    Scored {
        turn: Option<TurnKey>,
        labels: Vec<ScoredLabel>,
    },
    Fault(FaultReport),
}

pub struct ScoredLabel {
    pub label: String,
    pub score: f64,
}

pub enum LabelRefusal {
    NotAdmitted { reason: String },
    NotReady,
    Oversized { requested: u64, bound: u64 },
    MalformedContent,
}
```

**The cases are the contract's, spelled once.** `Classify` carries the
content and the turn identity as the floor's satellite type, optional
because apex invariant 5.3 is conditional on an existing turn: the loop
that classifies between turns, composing its re-entry before any turn
opens, belongs to none and carries none, and a classify within a turn
carries the key. The classifier echoes the turn identity back on `Scored`
exactly as it arrived, absence included, and the equality is the decoded
value's: the satellite type compares by its string, the wire spelling
being serde's, which is the level the contract's echo is checked at.
`Ready` is the readiness emission, `NotAdmitted` its typed failure
traveling in the enter aggregate, and `NotReady` refuses a directive that
arrived before either, per the contract's ordering. `Oversized` names the
artifact's own bound the way the decode seam's overflow names the
session's, both counts denominated in the artifact's own tokens, the
tokenizer's count of the content against the bound the artifact resolved,
one unit stated once. `MalformedContent` is a frame the process could
decode as a directive and not serve. `Fault` rides the answer set the way
the token trio carries it, one `fault-report` definition with a further
carriage. Every label of the artifact's head crosses in `Scored` with
none elided and none beyond, per the contract's completeness guarantee,
so the vector is the head and its order is the head's own. A score is a
finite JSON number, one per label, `NaN` and either infinity excluded: the
wire cannot carry what JSON cannot spell, and a scorer producing one has
faulted rather than answered.

**Tagged the way 4.4 tags, and JSON without a deferral.** `LabelDirective`
and `LabelRefusal` are internally tagged, their value-carrying cases plain
data. `LabelAnswer` is adjacently tagged with `kind` and `body`, because
`Fault` carries the floor's `fault-report` whose account is a spliced
member, unquoted pre-serialized JSON under the shared field rules: a
spliced member cannot ride an internally tagged enum, whose deserializer
buffers the content a splice must read raw, which is the same fact that
shaped `TokenAnswer`. The traffic is one ask and one whole answer per the
asking loop's election, low in volume and diagnostic in audience, the
loop 0 criterion rather than the decode seam's, so JSON is a settled
election against a stated criterion, not 4.4's provisional one: there is
no hot-path measurement to wait for, because there is no hot path.

**No record is added here.** The trio's nodes are the charter's, declared
at `weaver-types-PRD` section 2.3 with their `defines` edges, per the same
one-source rule the token trio states above.

## 5. What is enforced, and by which instrument

**Enforced by the compiler.**

- The crate's one internal dependency is `weaver-traits`, checked against the
  graph's single `floor-link` under gate H2.
- The three wire enums are exhaustive, so a case added to loop 0 breaks every
  consumer's match rather than landing in a wildcard.
- The config parse yields a fully typed value or a typed error and exposes no
  partial value, so a half-valid config is unrepresentable rather than merely
  refused.
- `deny_unknown_fields` makes an unrecognized config key a parse error rather than a
  silent discard, against the surface this crate declares as one type today.
- A directive disagreeing with its binding kind is unconstructable, the gate
  instruction riding inside `EnterBinding`'s serving case, per section 4.

**Enforced by a compile-fail test, because the property is an absence.**
`PeerIdentity` implements no `Deserialize`: a doctest attempting to deserialize
one fails to compile, and it starts passing the day someone adds the derive.

**Enforced by review rather than by a mechanism, and said so.** The carve-out
bound of charter section 2.2, that this crate holds one policy function and gains
no second, is not a compile property: a signature cannot prevent a sibling
function being added beside it. It is charter enforcement read at review, and an
earlier draft of this Spec listed it as a compiler pin, which is the overclaim the
apex's enforcement section exists to prevent.

**Enforced by the manifest.** No socket crate, no runtime, no I/O in the
dependency tree, checked as `weaver-traits-Spec` section 7 checks its own, by a
build-time assertion over the resolved external tree rather than by H2.

**Enforced by a perturbation-verified test at the parse.** Section 2's rule
that every identity message carries `role: system` is a `BadValue` naming
`identity.<n>.role`, beside the empty-device-list check. The test declares
each of the three refused roles in turn and watches the parse refuse, and it
is watched to fail when the call is removed from `parse` - which is not a
hypothetical state but the one that produced the cross-precision deposit of
2026-08-25, a run whose prefix named the agent and whose record never said so.

**This entry read "enforced by nothing today" for two days**, carrying its
date and its reason, because the instrument was withheld on an experimental
ground rather than a technical one. It is recorded here rather than quietly
overwritten: an unenforced rule that no section admits is indistinguishable
from an enforced one, and the corpus is better served by a reader being able
to see that the gap was known and named while it stood.

**Which invariant each claim serves, and why most serve none.** Seven of the
twenty carry a `grounds` edge, five to
`axiom-floor-is-vocabulary-behavior-is-socket` and two to
`axiom-contract-is-a-complete-interface`. The other three axioms take nothing from this
crate. The floor states no claim about a turn key, it is not an organ, and it is not
integrated: the fifth invariant binds what crosses between domains and the floor crosses
nothing, being linked rather than reached. **A crate can be drawn by every domain and
still be the subject of none of the invariants about domains**, which is the floor's
whole character stated from the graph's side. **The test applied is whether the axiom is
the reason the claim exists.** Remove the socket invariant and this crate has no reason
to elect a socket type, no reason to bound an envelope, and no reason to withhold a
deserializer from a credential, so those three ground in it. Remove it and the config
format is still YAML, the names are still kebab-case, and the parse is still total, so
those three ground in nothing. **Thirteen claims grounding in no invariant is
the expected result and not a gap**, per Document Format section 4: a floor
crate is mostly representation, and representation is what the invariants are
not about.

The two edges to the contract invariant are the ones worth stating rather than
leaving to be read. The wire enums are exhaustive so that every case a contract
can return reaches a match loudly, which is that invariant's completeness enforced
at the type level rather than asserted in prose. The sink is a discriminated shape
rather than a bare path for the same reason at the vocabulary layer: a bare path
is vocabulary without meaning, and the consumer would have to guess what the
operator meant, which is the guess the discriminant exists to refuse.

**Where the assertion records sit, and which of these this crate declares.**
The records are at the clauses that argue the claims, across sections 1 through
4, rather than gathered here, per Document Format section 6: this section sorts
by instrument and the arguments are elsewhere, so a block here would sit apart
from the prose that earns it. **A claim this section owes to another crate is
declared by that crate,** not here, because the assertion belongs where its test
lives and a node declared twice is the one-name-two-nodes defect the format
forbids for identifiers. Three of the bullets below are such owings and carry no
record in this document: the out-of-order refusal owed to each organ, the
boundary and truncation tests owed to the pair-creating crates, and the
accept-time refusal owed to the gate, which `weaver-gate-Spec` section 6 has
already discharged.

**Requiring a perturbation-verified test, with the owning crate named.**

- In this crate: the denial-precedence test of section 3, confirmed by watching
  the agent uid pass when denial stops preceding permission.
- In this crate: a missing required field refuses the parse, run separately for
  the residual-readout election, since it is the one a builder is most likely to
  make optional.
- In this crate: a misspelled key refuses, confirmed by watching a mistyped
  `permission-mode` vanish when `deny_unknown_fields` is removed. The watch is the
  derive's removal while the surface is one type's, and it moves with the surface.
- In this crate: an identity message refuses the parse unless it is a
  `System` message carrying at least one `Text` block and every block carries
  text. **Four ways of seating nothing or seating the unwritable, and each is
  watched:**
    - a role the door does not write, naming `identity.<n>.role`, confirmed by
      watching all three refused roles parse when `check_identity_roles` is
      removed from `parse` - the state that produced the cross-precision
      deposit of 2026-08-25, a run whose prefix named the agent and whose
      record never said so
    - an empty content list, naming `identity.<n>.content`, confirmed by
      watching it parse when its check is removed
    - a block the licensing rule does not admit, naming
      `identity.<n>.content.<m>`, confirmed by watching a `tool_call` parse
      when the block loop is removed
    - a `Text` block carrying no text, naming
      `identity.<n>.content.<m>.text`, confirmed by watching it parse when
      that arm is removed

  **The door refuses the role and the block both**, so judging only the role
  left half of it at runtime: a `role: system` declaration carrying a
  `tool_call` parsed clean, authored a fault without aborting the load, and
  was refused at the SPU's open. An empty `identity` list stays lawful, that
  being an agent with no prefix. **This entry read "enforced by
  nothing today" from 2026-08-26 to 2026-08-28**, carrying its date and the
  experimental reason the instrument was withheld. It is recorded rather than
  quietly overwritten: an unenforced rule that no section admits is
  indistinguishable from an enforced one.
- In each organ's crate: a directive case belonging to another seam is refused as
  `OutOfOrder` rather than acted on, confirmed per seam by watching a wildcard arm
  swallow it.
- In the pair-creating crates, with a socket dev-dependency that the
  no-socket-crate rule does not reach because a dev-dependency ships in no build:
  one envelope write is one envelope read at the elected socket type, and a
  message exceeding the buffer sets the truncation flag rather than arriving
  short, both confirmed by watching a message split or silently shorten when the
  type is changed to `SOCK_STREAM`.
- In the gate's crate: the accept-time refusal of section 3.

## 6. Open elections

- **`Generation`'s shape settled at section 4.4 and this bullet retires with it.**
  The emission and the finish are shaped in the floor because the harness consumes
  them, and the measurement splices because nothing consumes it on the way to the
  trace's model events. The seven satellites stay `weaver-trace`'s, neither
  restated nor drawn, and the floor's dependency set is unchanged. **What does not
  retire is the block partition's own question**, which party labels a block's
  spans. Splicing decides where that answer is owed rather than what it is: the
  renderer of a shape the trace accepts is the SPU, so the label is written there,
  and the question is answered before the first measurement crosses rather than
  carried as an open election. It is filed against `weaver-spu` rather than here,
  this crate no longer having a stake in it.
- **The config file's directory and naming convention.** Operator provisioning,
  outside what this program governs, per section 2. What this Spec fixes is that
  admin resolves one file or refuses.
- **The YAML implementation.** A maintained one, confirmed at the moment the
  manifest is written, per section 2. If none exists the format election re-runs
  against TOML on the writer-audience grounds.
- **The decode seam's encoding.** The token workflow's, with the hot-path
  measurement, per section 4.3, the channel question having closed with the
  decoder-cut ruling, decode on its own socket. **The trio's representation
  left this bullet when section 4.4 landed it,** the directive and the refusal
  being shaped there in full, and the one open part of the answer,
  `Generation`'s shape, carrying the bullet above. This bullet reads as the
  encoding alone, which is the separation section 4.4 argues and which an
  unnarrowed wording here contradicted.
- **`DeviceOrdinal` and `ArtifactRef`.** Satellites of section 2 with no
  cross-crate consequence beyond being well-formed: the first is an unsigned
  device number, so a negative one is a parse error rather than a check, and
  the second is what an operator writes to name an artifact, whose resolution
  is admin's and whose readability is the SPU's.
- **`FaultReport`'s shape is closed, 2026-08-17, and its case set widened by
  one the same day**, the code act finding the harness charter one case short
  of its crate's standing practice. Elected at section 4.2
  against the closed case set of `weaver-spu-PRD` section 13.10,
  `weaver-gate-PRD` section 13.4, and `weaver-harness-PRD` section 5: a typed
  `case` the harness consumes and an organ-rendered `account` it splices,
  which is apex section 5.2's custody rule deciding the shape, the fault path
  thereby corrected from the standing exception that section named. One shape
  serves the wire and the `fault` event's payload, as this entry always
  required. **The same judgment decides the run bracket's payload question and
  is recorded here so it is not re-argued**: the load line's reconstruction
  fields are admin's account of a load it caused, so when they land they land
  as organ-rendered content the harness splices, no floor type growing for
  them. That act, and not this one, takes the two reopenings it needs by name,
  `weaver-trace-Spec`'s rule that a bracket kind carries no payload member and
  `weaver-trace-PRD` section 3.2's settlement, because a reopening lands only
  with the mechanism it records. It need not take a third: recording the
  measured hash of what ran is reading what the run produced, so
  `weaver-types-PRD` section 2.1's ruling that no mechanism authors run
  conditions stands untouched by that member. A config identity asserted at
  load is a different matter, the mechanism 2.1's ruling names, and whether
  the load line carries one is the mechanism act's judgment to make by
  reopening 2.1 by name or by declining the member, not this election's to
  foreclose either way.
- **`TurnFrame`'s shape is closed, 2026-08-12.** Elected at section 4.1 by
  argument rather than the deferred measurement, per the operator's ruling of
  this date: the gate validates nothing, so arbitrary octets reach the seam
  by construction, a splice would convert a refused turn into a channel
  fault, and the frame carries its octets base64-encoded in one member.
  Recorded as closed rather than deleted, this list naming what settled each
  entry.
- **The `tool-set` field's element shape.** It elects from `tool-trait`, which
  `weaver-traits-PRD` section 3.1 holds blocked, so the field is a list of names
  today and gains its element type with the tool workflow.
- **`SessionId`, `RunId`, `TurnKey`, `AgentName`, and `FieldName`.** Named in the
  signatures above and shaped in this crate, their representations being
  identifier choices with no cross-crate consequence.
- **`AgentState` and `AgentSummary`, whose case sets are not free.** Their Rust
  representations are this crate's, but they ride a `lifecycle-answer` out of
  admin's invocation, so what an operator can be told about an agent is exactly
  what these enumerate. The lifecycle's four states, per apex section 6, are the
  floor of that set, and whether it carries more is settled with the operator
  interface's own design rather than by a builder.
- **`EnterPayload`'s field list**, which follows what admin supplies in the enter
  directive, per `weaver-admin-harness-contract` sections 3 and 5, and moves when
  that contract does.
