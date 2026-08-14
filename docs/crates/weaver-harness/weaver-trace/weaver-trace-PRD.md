# weaver-trace - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth. The crate PRD set is
written together and merged together, and no Spec is written against any member
before the whole set is merged. Ratification is the mapping of the whole document
set into the graph, and it belongs to the set rather than to this document.

**Date filed:** 2026-07-29
**Revised:** 2026-08-12, the request is the turn's contribution, per the
operator's ruling of this date closing issue 124. Section 3.2's model.request
narrows from the prompt as the model received it to the turn's rendered
delta, the full effective context being what the record determines by
accumulation, the measurement's token identifiers carrying the exact
tokenization. Two conditions ride it: a flush becoming reachable adds its
event to the kind set or reopens the ruling, and replay's identity prefix
leans on the open identifiability question. The state is a distillation of
the record, never stored back into it.
**Revised:** 2026-08-14, the run identifies itself. Session-wide order is the pair of
admin's run reference and the sequence, the reference having replaced an
ordinal that no per-invocation party could supply.
**Document ID:** `weaver-trace-PRD`
**Parent:** `weaver-harness-PRD`
**Companion contract:** `weaver-harness-trace-contract`, written with this document
**Editorial:** Per the Working Rules.

---

## 1. What this crate is

`weaver-trace` defines the primary artifact. It depends on no other Weaver crate,
binds no socket, and holds no cognition.

**It is not a floor crate.** The floor is `weaver-traits` and `weaver-types`, which
every domain draws from and none contains. This crate has one caller. The harness
authors every event and no other crate submits one, and a reader of a finished record is
downstream of a file rather than a party to this crate. An earlier reading grouped it
with the floor, and that grouping is residue from before sole-writer collapsed the
five-party trace seam to two. Depending on nothing is what this crate has in common with
the floor. Being drawn by everything is what it does not. The no-dependency claim
survives contact with the message kinds only because their payloads are opaque here,
which section 3 states rather than assumes. It defines what an event is, renders each
one to canonical form once, holds that rendering in RAM as the working structure,
and hands the same rendering to the outbound stream.

```graph
node: weaver-trace
kind: crate

edge: parent
from: weaver-trace
to: weaver-harness
```

**It does not produce the trace.** The harness authors. This crate is the mechanism the
harness authors through, and the distinction is the whole of the charter. `weaver-trace`
decides nothing about what is worth recording, when a session begins, or what a turn
is. Every one of those is policy and every one is the harness's. This crate
guarantees that what it is handed is recorded faithfully, ordered correctly, and
readable afterward.

The previous tree had no document governing production at all. It had a schema
contract, an access contract, and a draft custody contract, and nothing binding the
sole producer on how it produces. Every defect that corpus records is on the
producer side, which is why this charter is organized around production and why it
is written together with the contract that binds it.

## 2. The artifact

### 2.1 Three nested units

**A turn** is one request through to its final answer, bounded by `turn.started`
and `turn.closed`.

**A run** is one residency working on a session. `run0` opens the session. A run
ends at unload or at process death, and the identifier is an ordinal within the
session, so which run produced an event is answerable from the event alone.

**A session** is the identity the runs share, and the unit the agent's conversation
belongs to. It is the boundary the proto-stateful definition of apex section 2 is
drawn against: a new session begins with nothing. What continuity a later run of
the same session holds is the enter cell `weaver-admin-PRD` section 10 names, the
program promising none since the ruling of 2026-08-01, and the stream's account of
the session accumulating on the operator's side rather than in anything the
program keeps.

### 2.2 The session has two materializations

The **working structure** is the session in RAM: the run's admitted events in the
same canonical NDJSON form the stream carries, held in order, per the ruling of
2026-08-01 that retired the relational projection. It is what the harness reasons
over, which makes it state rather than a report about state, and the loop's reads
are sequential by design, the message sequence and the tool events in the order
they landed. It lives for one run, and it runs ahead of the outbound stream by the
depth of the writer's queue, per section 4.2.

**The working structure is append-only by construction.** It offers no update
surface, no delete surface, and no mechanism a caller could reach to alter an event
after it has landed. This is a structural guarantee rather than a behavioral one.
The agent runs as its own UID with access to bash, and a mutable in-process store
reachable through any path that UID can touch is a store the agent could alter.
A misconfigured sudoers entry, a future dependency, or a bug in a calling crate
cannot produce a mutation because the structure offers no mutation. The audit trust
property this crate exists to protect requires that guarantee to be architectural,
not a promise that everyone agreed to keep.

The **stream** is the session's outbound account: the same admitted events in the
same canonical NDJSON form, one event per line, in order, written to the sink the
operator declares and `weaver-admin` connects at load. Durability is the operator's,
per `weaver-admin-operator-contract` section 3, and the program holds nothing of the
stream once it has left the writer.

**The session record is what the stream accumulates, and the operator owns it.**
The name survives the ruling of 2026-08-01 because the thing it names survives: an
append-only, sequence-faithful, canonical account of the session, outliving process
and run both. What changed is custody. The record lives on the operator's side of
the sink, in whatever storage that operator's tooling keeps, and the program neither
reads it back nor vouches for what stands behind the descriptor. The agent has no
path to the sink, per `weaver-harness-PRD` section 5, so the custody that matters,
the agent's exclusion from its own account, does not depend on who persists it.

```graph
node: session-record
kind: artifact
```

### 2.3 One rendering, held and handed, and nothing to reconcile

The working structure and the stream carry the same canonical rendering, one held
and one handed, so no reconciliation between them can be owed and none exists.
An earlier version of this section guaranteed a deterministic projection from
events to rows, and the guarantee dissolved with the rows: with one representation
there is nothing a projection version could govern and nothing a divergence could
open between. A consumer of the stream that wants rows, an index, or any other
shape builds a derived view on its own compute from the same bytes, and a derived
view names its source range and is never a durable home, per section 8.

An earlier version of this section also made recreation the session-resume
mechanism, `run1` opening the record, projecting it, and appending. That mechanism
dissolved with the program-owned record under the ruling of 2026-08-01. Every run
begins with an empty working structure, the program promises no resume, and what
continuity across runs becomes, against operator-held storage or the memory round,
is the enter cell `weaver-admin-PRD` section 10 holds.

### 2.4 Spans

Spans are neither materialization and they are not stored. A span is a view over a range
of events, derived on demand for interoperability with tools that speak spans. It
has no durable home, no residency, and no authority. The previous tree wrote closed
spans into its durable log as events, which made spans a durable primitive, which
inverted its own ruling that a derived span is never the durable home, and which
left a separate span export that was a projection of data already stored in
projected form. Its own crate Spec could not resolve the resulting redundancy. This
charter resolves it by honoring the original rule: events are durable, spans are
derived, and nothing is stored twice.

### 2.5 Schema authority

The single emission is authored against the durable event schema, and that schema
is the only schema. The working structure holds the same canonical form rather
than a second representation, so no projection version exists to govern anything,
per the ruling of 2026-08-01. A change to the durable event schema is the breaking
change, and it is the one version every consumer keys on.

## 3. Events are typed, and that is a safety property

An event carries an envelope and a payload. The envelope identifies the session, the
run, the turn, the sequence, the kind, the producing subsystem, the causal parent,
and **two timestamps**: a session-scoped wall-clock stamp at
millisecond resolution for the calendar question, and a run-scoped monotonic reading
at nanosecond source with a microsecond floor for interval measurement. The two are
not interchangeable and neither answers the other's question, which is why there is no
single occurrence time. `weaver-harness-trace-contract` section 3 states which party
stamps them and why the scopes differ. The payload shape is determined by the kind,
and the kind vocabulary is closed.

**Three payload shapes are opaque to this crate.** `message.user`,
`message.assistant`, and `message.tool_result` carry conversation messages in the
shape `weaver-traits` defines, and this crate neither defines that shape nor decodes
it. It records the octets, sequences them, and carries them as opaque
content. Decoding is the harness's, which links `weaver-traits` and is the only party
that reads a message as a message.

This is the demand rule rather than a convenience. Section 6 guarantees canonical
byte form, a gapless run-scoped sequence, an interrogable committed boundary, whole
events to the sink, one rendering held and handed, and typed refusal, and not one
of them requires knowing what a message says. A crate that depends on no other
linking a definition it does not need gives up the property for nothing, so the
alternative reading, where
`weaver-trace` floor-links `weaver-traits` to decode three kinds, is refused on the
same grounds section 4 refuses everything else it does not demand.

**What refusal on submit means for these three.** The `payload does not decode for
its kind` case of `weaver-harness-trace-contract` section 5 reaches the envelope
binding and the octet well-formedness, not the message's interior. A malformed
conversation message is a defect the harness catches before submitting, because the
harness is where a message is a message.

```graph
node: event-envelope
kind: vocabulary

node: event-kind-set
kind: vocabulary

node: payload-shapes
kind: vocabulary

node: commit-boundary-states
kind: vocabulary

node: working-structure
kind: vocabulary

node: failure-vocabulary
kind: vocabulary

edge: defines
from: weaver-trace
to: event-envelope

edge: defines
from: weaver-trace
to: event-kind-set

edge: defines
from: weaver-trace
to: payload-shapes

edge: defines
from: weaver-trace
to: commit-boundary-states

edge: defines
from: weaver-trace
to: working-structure

edge: defines
from: weaver-trace
to: failure-vocabulary
```

These six are what `weaver-harness-trace-contract` draws from this crate, so the
union check of G4 runs against this list rather than against a reading.

**One field is recorder-assigned, the rest harness-supplied.** The sequence is
computed by the recorder on admission, because ordering is a property of the account
rather than a fact the harness holds. The hashes an earlier version assigned beside
it left under ruling B and the cut of 2026-08-01. Everything else is a fact only the
harness holds and the recorder could not derive.

**The recorder assigns properties of the account. It never authors content.** That
is the line. A sequence is a fact about where an event sits rather than a statement
about what happened. An event is content, which is why the harness authors every one
of them and why section 3.1 refuses a commit-checkpoint kind. Assigning a field to
an event the harness authored does not make the recorder an author. Emitting an
event the harness did not author would.

### 3.1 The closed kind set

The kind vocabulary is closed, and this is where it lives. A set declared closed and
held nowhere cannot be checked against, and the compile-time pin above is argued from
its closure.

**The set is flat, and every kind is recorded when it occurs**, per section 5:

| Kind | Meaning |
|---|---|
| `load` | opens a run |
| `unload` | closes a run |
| `session.closed` | the session will not be resumed |
| `turn.started` / `turn.closed` | the turn bracket |
| `message.user` | operator input |
| `message.assistant` | model output as conversation |
| `message.tool_result` | a tool result entering the conversation |
| `tool.call.started` / `tool.call.completed` | the tool bracket |
| `fault` | a fault the worker survived, reported by an organ and authored by the harness |
| `model.request` | the decode boundary, request side |
| `model.output` | the decode boundary, response side |
| `model.measurement` | input and output token identifiers, entropies and surprisals, the decode timings, model identity with its weights hash, the prompt-block partition, residual reductions |

Fourteen kinds. Adding one is an edit to this charter and to every contract whose
vocabulary clause names the set, because consumers key on the closure. `fault` is
the fourteenth, added under that rule by the fault-carrier ruling of 2026-08-01:
the stream is the program's one fault carrier, so a fault the worker survives is an
event like every other, recorded whenever it occurs because a fault absent from an
account would be the silently partial record this charter forbids. The
operator's tooling keys on it from the stream and comes back through the operator
surface with a verb. Its payload names the raising organ and carries what that
organ reported, and the case set behind it is deferred to the token workflow with
the organs' own charters, per `weaver-spu-PRD` section 10.

**Apex section 8's deterministic re-feed rests on five inputs, and since the
trace act of 2026-08-02 they span two kinds of one turn rather than one
payload.** Replay feeds recorded tokens back through a forward pass rather
than re-sampling, which needs the input token identifiers, the output token
identifiers, the model identity and its weights hash, the prompt-block
partition, and the sampling parameters, five items displayed as five because
the apex splits the identifiers and joins the identity to its hash. **The
first four are `model.measurement`'s and the fifth is `model.request`'s,**
the sampling values having moved to the request in that act because an input
to a decode belongs with what was asked rather than with what was read off
it. The join is the turn: both kinds carry the same
turn in the envelope, so a replay driver reads the pair rather than one
payload, and this sentence exists because a driver built against the earlier
single-payload reading would find four of five and conclude the record does
not support the arrangement every record supports. The prompt-block partition
and the explicit identity were
absent from an earlier version of this row. A payload carrying a weights hash and no
identity says which bytes were loaded without saying what was loaded, which is enough
to detect that two runs differ and not enough to reconstitute either, and a payload
without the partition cannot tell a replay where one block ended and the next began.

**Replay is universal since the levels retired on 2026-08-02, and the apex's
requirement is claim-relative since the correction of the same date.** Every run
carries its measurement payload, so the deterministic re-feed arrangement's
inputs are in every record and every record names the model that served it.
What the apex forbids, per its section 8 as corrected, is a record missing an
input of an arrangement it claims. Which further arrangements a record
supports, stochastic re-entry above all, follows from what the deployment
declared and produced, elections at the source governing production and
production governing the record, which is this charter's own section 5
instrumentation rule read forward.

**There is no `session.started`.** A session begins when `run0` begins, so a separate
kind would fire at the same moment as `run0`'s `load` and mean the same thing. The
same argument retires the description of `weaver-admin`'s initial contact as a
distinct first entry: **the `load` event of `run0` is that contact's record.** Admin
authorizes the transition and hands it across, the harness authors its `load` on
entering the run, and the monotonic origin is captured at that event. There is no
room between those for an earlier entry, and the worker spawn and descriptor handoff
precede the harness existing at all, so they sit outside the trace by construction.

**There is no commit-checkpoint kind.** The committed boundary is interrogable from
the recorder's boundary report, per section 4.2, and an event marking it would be
the recorder authoring into an account the harness is the sole writer of.

**This enumeration appears in the contract as well, deliberately.** A Spec is derived
from its crate's PRD plus every contract that crate is party to, so the harness Spec
writer reads the contract and never opens this charter. The contract must therefore
carry the field set rather than point at it. The two copies are not redundant, because
this one says what an event **is** and the contract's says what the harness
**supplies**, but they assert a property of the same thing and can diverge. **If they
ever disagree, this charter is authoritative** for the envelope, because this crate
defines the artifact and consumers outside that contract read the record. A divergence
is a defect to file rather than one a reader resolves by choosing.

**The emission path is typed values, not instrumentation macros.** This is a design
requirement rather than an implementation preference, and the reason is a defect
the previous tree documented against itself. Its span layer took field names as
literal tokens at the emit site, so nothing connected an emitter to the declared
attribute vocabulary. The result was that four of the most operationally valuable
measurements in the system, the decode timing fields, appear nowhere in the
vocabulary at all, and a span kind outside the closed set was emitted at three
sites. The Spec's own conclusion was that the vocabulary could not be read as the
schema.

A typed event with a closed kind and a typed payload cannot carry an invented
field, because the compiler refuses it. The apex asks for compile-time pins where a
runtime test structurally cannot reach. This is one, obtained by construction
rather than by adding a check that someone must remember to run.

Adding an event kind is a change to this charter and to the contract. It is not a
free extension, because consumers key on the closed set.

**`load` and `unload` are event kinds, and they are the run bracket.** Entering a
run writes its `load` event and leaving it writes its `unload` event, both authored
by the harness. The minimal run is those two events with nothing between, so **a run
cannot be empty**, and that is what makes run numbers verifiable by whoever reads
the account rather than merely declared. The program runs no such check since the
cut of 2026-08-01, the reading being the consumer's, and the property holds either
way because it is a property of what is written.

The bracket is defined here beside the run field deliberately. A run label that
could be stamped without the events existing to verify it against would be an
assertion with nothing behind it, which is the shape of every defect in the previous
tree's attribute vocabulary.

**A run edge is authored, not inferred.** Its boundaries are known at the moment
each bracket event is written, because nothing else writes a boundary and the
envelope's run field is the only place one is recorded. No consumer reconstructs a
run by range analysis.

**A `turn.closed` payload states which kind of close it was.** A clean close carries
the turn's completion. A stopped close carries the stop reason in place of one,
authored by the harness when the abort of `weaver-admin-harness-contract` section 3's
stop exchange lands. Every open has a close and the close says which kind it was, so
a reader never infers an abort from an absence. This adds a field to one payload
shape and no kind to the set, and it edits this charter and the contract in the same
act, per the rule of this section.

### 3.2 The payload shapes the token workflow settles

Six of the fourteen kinds carried no chartered shape, and this subsection
settles four of them on the token workflow's acts of 2026-08-02: `fault` and
the three `model.*` kinds. The tool bracket's two are the remaining pair and
stay deferred with the tool workflow, per section 9. What follows is content
and its obligations, which is this document's level. How any of it is
represented is the Spec's.

**`fault` carries the floor's report and this crate defines no second shape.**
`fault-report` is `weaver-types`' definition as of the gate act, one shape for
every reporting seam because what an organ hands the harness is the same fact
whichever socket carried it. The case set behind it closed the same day across
all three organs, `weaver-spu-PRD` section 13.10, `weaver-gate-PRD` section
13.4, and `weaver-harness-PRD` section 5, so this payload is elected against a
closed enumeration rather than a guess. Which subsystem raised it is the
envelope's field and is not repeated in the payload. A fault the worker does
not survive is not this kind, per the contracts, because the party that would
report it is the one that died.

**`model.request` carries what the model was asked this turn, the turn's
rendered contribution, per the operator's ruling of 2026-08-12.** The turn's
delta as the family library rendered it, the identity of the template that
produced it, and the sampling values in effect for the turn, effective
values whichever side set them, per the disposition rule of `weaver-spu-PRD`
section 13.8. The seam is append-only, so what the model received grows each
turn while only the delta crosses, and the full effective context is what
the record determines rather than what any one event stores: the
accumulation of the recorded contributions under their recorded template
identities, from the identity prefix the configuration carries. Recording
the full effective prompt per turn would store a derived quantity the record
already determines, one fact in two places, and would grow the account by
the square of the conversation, the resend pathology the append-only seam
exists to kill returning as record bloat. The state is a distillation of the
record and is never stored back into it. The rendered form is here rather
than nowhere because of the end-to-end requirement of 2026-08-02: each model
carries its own template, so the turn's contribution exists twice, as the
canonical messages this crate's message kinds hold and as the rendered piece
the model was given, and a record holding only the first forces a replay to
re-render through a template that may have changed since, which observes a
forward pass that never happened. Recording both, with the measurement's
token identifiers carrying the exact tokenization, is what makes apex
section 8's rule that tokenization is reproducible from what is recorded
true rather than hopeful. Two conditions ride the ruling, named rather than
slid past. The accumulation reading holds while every operation that moves
the resident context is recorded. The flush is chartered on the decode seam
and served by the SPU, and no granted surface exposes it today, so no loop
can reach it: the act that exposes the flush adds its event to the kind set
through the front door or reopens this ruling. And a
replay's identity prefix is the configuration's, so the record's sufficiency
for replay leans on the open question of what the load event carries of the
admitted identity, which this ruling names and does not decide.

**`model.output` carries the emission verbatim, before any parse.** The
model's own family-styled output, the reasoning blocks and channel markers
and tool-call markers exactly as emitted, and how the generation ended,
completed or stopped. **The record holds both layers and never flattens
one into the other:** this payload is the verbatim reality and the message
kinds hold the canonical parse of it, with the family parsers as the recorded
bridge between them, which is the outbound half of the same end-to-end
requirement. A record that kept only the parse could not answer what the
model said, and one that kept only the verbatim would make every consumer a
parser of every family.

**`model.measurement` carries the instrument readings, and its field list is
section 3.1's row as this act corrects it.** The row loses the sampler
parameters, which move to the request above because an input to a decode is
part of what was asked, and gains the surprisals and the decode timings, which
were readings the row had never named. What this subsection adds is the
producing obligations
that make those readings trustworthy, stated as content because a reading
produced wrongly is not the same fact recorded imperfectly. The per-token
signals are computed against the pre-sampler distribution, because a signal
taken after the sampler measures the sampler. They are positionally paired
with the token identifiers, so a consumer joins by position rather than by
inference. And an unproduced reading is absent rather than zero, because an
empty vector and a confident model are different facts and a zero that means
neither is the silently partial account this charter forbids everywhere else.
The residual reductions appear exactly when the residency was admitted with
readout elected, per `weaver-spu-PRD` section 13.7, and a tap that failed
while elected is a fault rather than an absence.

**These four shapes close the deferral this charter carried, and the tool
bracket's two are what remain.** Section 9's staged list keeps them, and
nothing here shapes them, because a payload shaped against an unchartered
protocol is the reserved slot apex section 9 forbids.

## 4. What producing the trace requires

### 4.1 Open

The sink is opened by `weaver-admin` under root, the role's principal, per
`weaver-admin-operator-contract` section 3, and the descriptor is passed to a worker
that has held the agent uid from its first instruction, because the init system
starts the unit under `User=` and there is no drop to order, per
`weaver-admin-harness-contract` section 2. The receiving uid confers nothing either
way: a descriptor crossing a Unix socket is a capability and the kernel rechecks no
permission at receipt. `weaver-admin` resolves which session is being loaded. The
harness never learns a path or a sink's nature.

**Every run begins empty.** The working structure starts with nothing, the run's
first authored event is its `load`, and the stream continues at whatever sink admin
connected. There is no resume path, per the cut of 2026-08-01, no projection of
prior history, and no reconstruction from anything, the enter cell of
`weaver-admin-PRD` section 10 holding what continuity may later become.

**This crate's write surface accepts descriptors, never paths.** That is the API
consequence of the custody model, and stating it here is what makes custody
expressible rather than merely intended. A crate that offers a path-taking write
function offers a way around the boundary, and the previous tree carried exactly
that: a trace-root resolver with zero production callers whose layout described a
path no artifact ever used, while the security invariant three other documents
cited was specified against it.

Every descriptor this crate writes through is close-on-exec, and where the sink is
a file it is append-only as well. The first keeps tool subprocesses from inheriting
a writable handle. The second makes append-only a property of the handle rather
than of the writer behaving well, and it applies where the sink has a seek to
forbid, a pipe or a socket having none.

**The two flags do not arrive by the same route, and only one of them travels.**
Append-only rides the open file description, so a descriptor passed over a Unix
socket carries it and the opener confers it once. Close-on-exec rides the
descriptor, so it does not cross, and the receiving call is the only place it can be
supplied. `weaver-admin` opens a file sink append-only and cannot confer
close-on-exec on a descriptor it hands over. The harness supplies it at the receive,
and `weaver-admin-harness-contract` section 5 holds that split.

**What is pinnable at compile time is the shape and not the flags.** A behavior on
the receive path is not a type property, so no pin reaches either flag. What a pin
does reach is one receive site, taking no flag argument, returning a handle the rest
of the crate cannot construct another way. The flags themselves take the
perturbation-verified test of apex section 11, and the test is confirmed by watching
it fail when the flag is removed.

### 4.2 Update

One emission per event, and one emission reaches two sinks. The order is fixed and
is not an implementation detail, because the failure it prevents is a working
structure holding an event the stream never received:

1. the harness submits an authored event,
2. this crate admits it or refuses it with a typed failure,
3. an admitted event is assigned its sequence and rendered to canonical bytes once,
4. that one rendering fans out, appended to the working structure and handed to
   the stream writer in the same act.

**Refusal sits at admission, ahead of the fan-out.** An event that fails validation
reaches neither sink and an event that passes reaches both, so there is no state in
which one materialization holds what the other refused. What the recorder may judge
is bounded by section 3, reaching the envelope binding and the octet well-formedness
and never the interior of a payload.

**The working structure lands first and is the acknowledgment.** It is in-process
memory and the sink is not, so the append completes and the turn proceeds while
the stream write is still in flight. Reads are served at memory latency and the
stream trails. This is a trade the architecture makes deliberately rather than a gap
in it.

**The stream trails by the depth of the write queue.** Process death forfeits
whatever the writer had not yet handed to the sink. On a fast, quiet sink that tail
is effectively one event. On a saturated, contended, or slow sink the queue is
deeper and so is the loss. **The bound is a property of the deployment, not of this
crate.** This crate offers no policy field to tune it and no flush cadence to elect,
because a periodic flush is an interruption on a path whose purpose is to never have
one. What may happen under sustained pressure is the marked election of
`weaver-admin-operator-contract` section 3, and nothing here widens it.

**What reaches the sink is whole.** The writer hands the sink complete events and
never a partial one, so a consumer's account truncates at an event boundary rather
than holding a corrupted middle.

The three states of the commit boundary describe where an event sits in that
fan-out. **Committed** has been handed to the sink and is what the operator's
account holds. **Pending** has been admitted and appended and is still in the
writer's queue, so it is the loss window, whose depth the deployment sets rather
than an election. **Failed** is a stream write that errored against a live process,
and it is surfaced.

**Silent shed is forbidden while the process lives.** A stream write that fails
under a running process is named and surfaced and never swallowed, because the
process is there to report it. A tail lost to process death is a different failure,
unreportable by the thing that died, and the account must not present the two as
one. A trace is measurement data, and a silently partial account does not produce a
gap a reader can see. It produces a coherent account of a session that did not
happen.

**Nothing on the turn path waits on the sink.** The harness reads the working
structure in RAM. The stream write runs off the hot path, and a slow or failing
sink never slows the interior read. The two are failure-isolated.

**Canonical form is one mechanism, used everywhere.** Integer fields that can exceed
the double-safe range serialize as decimal strings. Nanosecond values exceed it by
roughly two hundred times, so a reader parsing them as doubles gets a silently
different number with no error and no way back. The previous tree got this right in
its record and wrong in its span export, which is what happens when two artifacts
carry the same field under two rules.

**The reason is resolution, not overflow.** Read as overflow avoidance the rule
looks like defensive rounding, and the natural response is to store fewer digits.
That response destroys the measurement. The monotonic clock resolves to nanoseconds
with a microsecond floor because the harness runs orders of magnitude faster than
the decoder, which pays network-class millisecond latency, and local latency is the
reason the model was brought next to the services in the first place. A record that
cannot resolve finer than the fastest thing it measures makes that advantage
invisible. This is systems-architecture timing, not network timing, and the decimal
string exists to carry the digits rather than to dodge an exception.

### 4.3 Drain

A run ends at unload or at process death. At unload the writer's queue is drained,
so a left answer on the coordination seam means everything admitted reached the
sink, per `weaver-admin-harness-contract` section 4. At process death the queue's
tail is forfeited, per section 4.2, and nothing drains it.

That is the whole of maintenance since the cut of 2026-08-01. There is no
finalization, no checksum, and no manifest, because the program owns no artifact to
finalize: what shape the account takes at rest, whether one file or many, rotated or
not, indexed or not, is the operator's tooling's business on the operator's side of
the sink. `session.closed` is content, authored by the harness like every other
event, and the stream simply ends where the session did.

## 5. Instrumentation

**The trace carries no recording level, and the recorder filters nothing.** Per the
human's ruling of 2026-08-02: whatever the agent produces reaches the record, and
what an operator elects at load is what the program *does* rather than what it
writes down. There is no floor and no ceiling here, because those named a filter
between the working structure and the stream, and the stream is a copy of the
working structure rather than a selection from it.

**The ruling follows the one that placed durability with the operator.** The cut of
2026-08-01 ended this program's obligation at the tee and left retention to the
party that owns the storage. A recording level is that same decision taken one
layer earlier, the program choosing what an operator is permitted to keep before
the operator chooses what to keep. Removing it is the durable-record ruling read
consistently rather than a new position.

**Elections live at the source, and the residual readout is the reference case.**
It is elected per agent in the config file, read at load, fixed for the life of the
run, and what it changes is what the SPU computes. When it is on, the reductions
exist and are recorded. When it is off, there are none to record. That is the shape
every instrumentation election takes: it governs production, and production governs
the record.

**What this dissolves is an ambiguity rather than only a mechanism.** An earlier
version of this section had the harness author each run's level into the run's own
events, because elected brevity and silent loss otherwise looked identical to a
consumer. With no level to elect, anything missing from an account is loss, which
is a stronger property than a reader consulting a recorded level to interpret a
gap.

**Two consequences follow, both improvements.** Every run is replayable, because
the token identifiers apex section 8 rests replay on are in every run's
measurement payload rather than in an elected subset, with the sampling
parameters in the same turn's request payload since the trace act of
2026-08-02, per section 3.1. And every record
names the model that served it, along with its weights hash, which closes the cell
`weaver-spu-PRD` section 10 held open on the premise that a floor run named
neither.

**The cost is stated rather than prevented.** Measurement payloads are per-token,
so every run grows the working structure at the rate an elected run grew it before,
and RAM is the one thing an operator cannot reclaim downstream. The lever is the
production election, which is the honest place for a lever: not producing a
measurement is cheaper than producing and discarding it, and discarding it after
the fact is the operator's to do on the operator's own compute.

## 6. What this crate guarantees

- Canonical byte form, one rule, all artifacts.
- A strictly increasing, gapless sequence over admitted events, **scoped to the
  run**. Session-wide order is the pair of admin's run reference and the sequence,
  assembled by the consumer, because the program holds nothing across a residency
  since the cut of 2026-08-01. The word monotonic is reserved in this charter for
  the clock of section 4.2. The sequence is the order and the clock is the
  instrument, and reading either for the other's job is an error the contract names
  explicitly.
- An explicit committed boundary, interrogable while the process lives: what was
  handed to the sink, what the queue still holds, and what failed. It is not a
  promise that nothing is lost, because section 4.2 forfeits the writer's queue to
  process death and bounds that queue by the deployment rather than by this crate.
- Whole events to the sink, never a partial one, so a consumer's account truncates
  at an event boundary.
- One rendering, held and handed: the working structure and the stream carry the
  same canonical bytes, so no reconciliation between them exists to owe.
- Typed failure for every refusal. Nothing fails silently and nothing returns a
  partial result with a success status.

What left this list on 2026-08-01, deliberately rather than by drift: exact
recreation, run integrity on read, mechanical validity on read, and the per-turn
hash. Each presupposed a program-owned record to read back, and the ruling recorded
at `weaver-admin-operator-contract` section 3 places the record with the operator.

## 7. The seam

The production seam is bound by `weaver-harness-trace-contract`: the two exchanges,
what each party supplies and guarantees, the ordering rule that admission precedes
the fan-out, the failure vocabulary, and the prohibitions on both sides. It is read
alongside this charter and neither is complete without the other.

The seam edge is declared once, by the crate that asks, which is the harness. This
charter points at the contract and does not restate the edge.

This section was a list of what the contract had to settle, written before the
contract existed. It is now a third partial copy of a document that says it better,
so it is a pointer instead.

## 8. What does not cross

**The stored span export.** Spans are derived on demand. There is no second
artifact.

**The reserved span kinds.** The previous tree retained sleep and reconciliation
kinds so its enum could keep parsing archived artifacts. There are no archived
artifacts here, so the reason does not transfer and the kinds do not cross. This is
consistent with the apex rule against carrying slots for designs not yet written,
and it is a different reason than that rule gives.

**The attribute-name vocabulary as a convention.** Typed payloads replace a module
of name constants that emitters were free to ignore.

**Post-hoc annotation by consumers.** The previous tree let its analysis crate write
spans into a finished trace after the run, which meant a reader could not assume
every span in an artifact came from the producing process. The harness is the sole
writer, and an analysis artifact is a separate file.

**Memory, in every form.** No consolidation events, no belief vocabulary, no
substrate. The event kinds that serve those exist when their emitters do.

**Redaction, at write time.** The recorder never filters, so it never scrubs. The
harness authors what it authors and the recorder never filters, so a recorder that
dropped or altered content because it judged the payload would have taken policy
the harness holds. There is no third
place on the write path where redaction could occur, so the record is raw by
construction rather than by an unmade decision. Scrubbing, if ever wanted, is a
formatter election on export and downstream of everything here. What an operator
does with a delivered artifact is theirs.

**Embeddings.** Nothing in the proto-stateful turn retrieves by similarity, so a vector
recorded here would have no consumer. A payload field whose only reader is unbuilt
is a reserved slot in data form, and the schema is where that rule is easiest to
break, because adding a field feels smaller than adding an interface. Embeddings
enter the record when something reads them.

## 9. Staged requirements

A staged requirement is recognized work with an entry condition that holds it out of
the current pass. It is written here rather than carried in conversation so that its
disposition is a decision made at ratification rather than a thing remembered. This
document is living, so an item leaves this section by being built rather than by
being rediscovered.

**This section is authoritative for staged work belonging to this crate.** A working
list of open items holds deferred work too, and the two placements need one of them
named. A staged requirement with a crate to belong to lives in that crate's charter,
because it travels with the crate and because a Spec is derived from the charter plus
its contracts and never from a working list. A working list holds what has no owner
yet, and an item moves into a charter the moment it acquires one. That list is never
ratified and shrinks toward empty, so work parked only there is work that evaporates.

**The in-RAM representation.** Whether the working structure holds canonical lines
parsed on read or typed events rendered to their line once, and how the loop's
sequential reads are served, are the Spec's, bounded by section 2.2: one canonical
structure, append-only, no second authoritative form and no mutation surface. The
relational engine this item used to stage, a hand-rolled typed query builder that
existed because `rusqlite` was ruled out on append-only grounds, dissolved under
the ruling of 2026-08-01: nothing chartered reads relationally, the loop's reads
are sequential, and a consumer wanting an index builds a derived view. Recall over
past runs, when the memory round takes it, is similarity through the SPU's encode
side over the NDJSON account rather than a relational query, per the proposed
reading the enter cell of `weaver-admin-PRD` section 10 carries. **Entry
condition:** the Spec pass.

**The weight of the previous durability implementation.** The previous tree's
`event_log.rs` is 2,773 lines carrying canonical byte encoding, payload hashing,
sequence-gap detection, a committed-versus-pending boundary, and commit-pressure
policy. The obligation it would be read against shrank under the cut of 2026-08-01,
so the examination is now whether anything beyond canonical encoding, the boundary,
and pressure surfacing survives at all. **Entry condition:** the Spec pass.

## 10. Open ruling

**The interoperability target.** Spans are derived for tools that speak spans, and
the previous tree's format borrowed OTLP field names without being OTLP, then
carried a claim that a trivial converter could bridge them which turned out to be
untrue and unverified. Whether the derived view targets OTLP specifically, and what
conformance record backs that claim, is not settled here.
