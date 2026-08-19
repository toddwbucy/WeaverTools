# weaver-harness / weaver-spu - decode contract

**Status:** MERGED. Cut with the token-workflow act of `weaver-spu-PRD` sections 13 and
14, per apex section 10, and merged on the human's call like every document, the header
flipping at that call per the Working Process. Written and it governs the token seam:
the second seam between the same parties, on its own socket per the decoder-cut ruling
of 2026-08-02. The residency seam keeps its own contract, `weaver-harness-spu-contract`,
and neither document restates the other.

**Date filed:** 2026-08-02
**Revised:** 2026-08-19, second of this date, the flush names its cut.
Per the operator's ruling: the cleanup line is the loop's, because the
loop knows what it does not want to re-decode, and a fixed outcome made
one policy true for all use cases. The flush directive gains `keep`, the
resident length the session returns to, bounded below by the identity
prefix whose permanence this seam guarantees and above by the resident
count, the confirmation's counts carrying what held in every case. The
SPU still decides nothing: it executes the cut the ask names, per
`weaver-spu-PRD` section 13.9 as amended in this act.
**Revised:** 2026-08-19, the generation reports the session's fullness. The
generate exchange's closing answer carries the session's resident token
count and its capacity beside the emission, per issue #221's arc: the
asking loop must see pressure before the wall, and the overflow refusal
was the only carrier of either number. Plain counts with no judgment,
`weaver-types-Spec` section 4.4 shaping them.
**Revised:** 2026-08-17, the request owns the template identity, per the
ruling on #129. The measurement's enumeration listed it beside the block
partition while the request carried it too, one fact in two boxes of one
turn's pair. It is an input-side fact of the sampling values' kind and it
travels where they travel. The code and `weaver-trace-PRD` section 3 already
did this; the enumeration catches up.
**Revised:** 2026-08-16, the sampling values leave this seam. They travelled
here per turn and the engine builds its sampler once at session open, so they
had no engine to reach, and they now cross in the declaration at admit. The
conformance item widens with them: this seam carries no sampling value at all
and the capture says so without needing to know which were frozen.
**Revised:** 2026-08-12, third of this date, the request is the turn's
contribution. Per the operator's ruling closing issue 124, the request
content the close carries is the turn's delta as rendered, the full
effective context being the accumulation the record determines, and the
SPU's rendering stays final as built.
**Revised:** 2026-08-12, second of this date, the receipt retires. Per the
operator: the SPU is a function, input in and output out, stopping when it
stops unless interrupted, and a receipt it must consume is protocol state
with no consumer behind it. The fault report becomes the seam's one
emission rather than its fifth exchange, owed nothing back at all, the
trace entry being the acknowledgment, and the fault-carrier ruling's razor
is the precedent, a second carrier for one fact earning nothing. Sections
2, 3, 5, 6, and 8 carry the change.
**Revised:** 2026-08-12, the seam's dataflow rule stated, per the operator's
ruling of this date. Section 2 names the direction of expectation its count
already embodied: a harness-opened exchange asks and its answer returns what
the ask produced, and the SPU-opened exchange reports and is owed nothing
back but receipt. The receipt this entry lands is retired by the entry
above, same date, and the exchange it names is an emission since that
ruling. Section 6 carries the rule's edge as a prohibition, the
SPU holding no exchange in which to ask. Stated for the SPU in particular
and generalized to no other organ.
**Revised:** 2026-08-11, the seam streams, per the operator's ruling of this
date. The append-and-generate exchange carries each token as an intermediate
message as it is drawn, the identifier and its rendered piece, none closing
the exchange, and the answer closes as it always did with the generation
whole. The close gains the rendered prompt as the family library produced
it, beside the measurement rather than inside it, one spliced member per
record box, which is the record's request side gaining its wire source.
Section 3 orders the intermediates before the answer, section 4's supplies
follow, and the guarantee lands that the stream and the close never disagree
about what was drawn. The stream changes the rhythm and not the record.
**Document ID:** `weaver-harness-spu-decode-contract`
**Parent:** `WeaverTools-PRD`, invariant 5.3
**Editorial:** Per the Working Rules.

---

## 0. What this document is

The agreement over the token seam: how the harness's turns reach the resident
model and how the generation and its measurement return, what each party may
rely on, and how it fails. It is read alongside `weaver-harness-PRD` and
`weaver-spu-PRD` section 13, and none of the three is complete without the
others.

It carries no representation. The types it names have a definition site and no
field list here, the ordering it fixes is stated as a rule rather than a state
machine, and how any of it is encoded is the Spec's, elected against a
measurement because this is the hot path.

**The name carries the seam's traffic, because two seams between one pair need
two names.** The Document Format names a contract for its parties, initiator
first, and both seams between this pair have the harness initiating, so the
party rule alone cannot distinguish them. The `-decode-` infix is the
distinguisher, naming the governed traffic the way the wire vocabulary is
named for its loop, and the residency contract's name stands unchanged.

**This document does not draw `weaver-organ-channel`.** The token seam is not
an organ channel, per `weaver-spu-PRD` section 13.2: the organ test names one
two-initiator channel and the lifecycle channel is it. The channel mechanics this
seam needs are stated in section 1 as its own, and the organ envelope does
not cross here.

```graph
node: weaver-harness-spu-decode-contract
kind: document

edge: party
from: weaver-harness-spu-decode-contract
to: weaver-harness

edge: party
from: weaver-harness-spu-decode-contract
to: weaver-spu

edge: draws
from: weaver-harness-spu-decode-contract
to: token-directive

edge: draws
from: weaver-harness-spu-decode-contract
to: token-answer

edge: draws
from: weaver-harness-spu-decode-contract
to: token-refusal

edge: draws
from: weaver-harness-spu-decode-contract
to: fault-report

edge: draws
from: weaver-harness-spu-decode-contract
to: message-model
```

**The seam edge is declared by the organ and appears in `weaver-spu-PRD`
section 6.** This document names its parties and does not restate the edge.

## 1. The channel

**An unnamed connected pair, created by the harness before the fork, beside
the lifecycle pair.** The same act creates both, the same fork carries both
ends, and `weaver-spu-PRD` section 7 counts them: the SPU receives its two
channel ends and no other descriptor. Possession authenticates, because a
pair with no name has no second opener, and the channel lives exactly as
long as the process at its far end.

**One write is one message, and the property comes from the socket type.**
The seam's traffic is framed by the channel rather than by a layer above
it, so no contract that touches this seam carries framing. Which socket
type supplies the property is the Spec's election, taken with the hot-path
measurement this seam uniquely owes, since the per-token volume here is
what the loop-0 elections deliberately did not price.

**Close-on-exec on both ends, by the split the corpus already runs.** The
harness's end carries the flag from the pair's creation, so no tool
subprocess the harness forks inherits a handle to the decode seam. The
SPU's end is set after its final exec, a set and not a check, so nothing a
later workflow spawns from the SPU inherits it either.

**Closure is not an answer.** A closed channel with an exchange outstanding
is the far party dead mid-exchange, treated as that exchange's failure and
never its success, and what a death means is section 5's.

## 2. The exchanges

Four exchanges and one emission. The exchanges are opened by the harness.
The emission is the SPU's fault report, the origination the residency
contract deferred, arriving here because the faults it carries are
decode-domain traffic, and it is an emission rather than a fifth exchange
per the receipt's retirement, the second ruling of 2026-08-12.

**The count carries the seam's dataflow rule, stated with the operator's
ruling of 2026-08-12.** An exchange the harness opens is an ask, expected
to set the SPU producing, and its answer returns what the ask produced:
the session opened, the generation drawn, the flush's outcome held. What
the SPU originates is a report, owed nothing back at all. The fact it
carries has one home, the record, whose custodian is the harness, so the
trace entry is the acknowledgment and no answer returns to the SPU.
Nothing that originates on this seam arrives owed work, which is the fact
the harness charter reads when it names what may drive a turn. The rule
is stated for the SPU in particular and generalized to no other organ: a
later organ's contract states its own dataflow rule at its own chartering
rather than inheriting this seam's.

**Open the session.** Opened by the harness, once per residency, after
residency is confirmed on the lifecycle seam. It carries the session's
identity material, the canonical messages the identity prefix is rendered
from, per the framing ruling of `weaver-spu-PRD` section 13.4, ratified by
the operator with this act. The SPU renders through the family library,
establishes the resident session with the prefix resident, and answers
opened, or refuses, typed.

**Append and generate.** Opened by the harness, one per turn, carrying the
turn's context per apex section 5.2 and the turn's delta as canonical
messages under the same framing ruling. **It carries no sampling values.**
The operator-tunable remainder reaches the SPU in the declaration at
admit, per `weaver-spu-Spec` section 8, because the engine builds its
sampler once at session open and a value arriving with a token directive
has no engine to reach. An earlier wording carried them here, which is
the route this act closes. The SPU appends the delta at the
resident end and generates, **and the exchange streams as it does**, per
the operator's ruling of 2026-08-11: each token crosses as it is drawn,
an intermediate message carrying the token's identifier and its rendered
piece, any number of them and possibly none, ordered, and none closing
the exchange. The answer closes the exchange as it always did, carrying
the generation whole, the model.request content beside the measurement
because each splices into its own box of the record, the request being the
turn's delta as rendered with its template and effective sampling per the
ruling of 2026-08-12, the SPU
rendering it whole per the custody act of 2026-08-11, its measurement:
the token identifiers, the per-token signals, the timings, the block
partition, and, where the residency was admitted with readout elected, the
residual reductions. Beside the two boxes, as the generation's own typed
members and never inside the measurement, the session's fullness as the
generation closed: the resident token count and the capacity, added
2026-08-19 so the loop that elects a flush sees the pressure before the
wall rather than in the wall's refusal, shaped at `weaver-types-Spec`
section 4.4 because the harness consumes them. **The template identity
travels in the request and only
there**, per the ruling of 2026-08-17 on #129: it is an input-side fact of
the same kind as the sampling values, which moved to the request for the
same reason, and a member carried by both boxes of one turn's pair would be
one fact in two places with no authority named. **The stream changes the seam's rhythm
and not the record's
shape**: the emission and the measurement arrive whole at the close, so a
consumer that ignores every intermediate message reads the exchange
exactly as the batch seam read. The answer closes the exchange and the
turn's context returns with it.

**Cancel.** Opened by the harness while a generation is in flight,
carrying the turn's context and nothing else. The generation stops at the
next token boundary, the session is left well-framed with the family's
turn terminator resident, and the outstanding append-and-generate answers
with the partial output marked stopped, the tokens already streamed
standing and the close accounting every one of them. A cancel with nothing in flight
answers at rest, a clean close rather than a refusal, the same shape the
stop exchange takes on the coordination seam.

**Flush.** Opened by the harness between turns, never while a generation
is in flight. The directive names its cut, `keep`, added 2026-08-19 on
the operator's ruling that the cleanup line is the loop's: the session
returns to its first `keep` resident tokens, everything beyond them gone,
the outcome fixed against the named cut and the mechanism the Spec's per
family, per `weaver-spu-PRD` section 13.9. The cut is bounded, not
refused: a `keep` below the identity prefix's length holds the prefix
entire, its permanence being this seam's own guarantee, and a `keep` at
or beyond the resident count cuts nothing, the confirmation's counts
carrying what held in either case, so no outcome is silent. A `keep` of
zero is the prefix-only state the flush has always meant. The answer
confirms after the outcome holds, and carries the resident token counts
before and after the truncate: the SPU is the one authority on either
number, and the harness authors the record's `flush` event from exactly
them.

**Report a fault.** Emitted by the SPU, the seam's one SPU-originated
message and not an exchange, carrying a `fault-report` naming a case of
`weaver-spu-PRD` section 13.10 that arose outside any outstanding
exchange, residency degraded above all. A fault arising inside a
generation is that exchange's typed answer instead, so one fact never
travels twice. The harness authors what it is handed as the `fault`
event, per the fault-carrier ruling, and answers nothing. The receipt
retired on the second ruling of 2026-08-12: the SPU could do nothing
with it but discard it, and a confirmation whose one reader discards it
is the alert exchange's error one layer down. The promise of faithful
authorship is this contract's, in section 4, rather than an answer's.
The SPU forgets the fault at the send, produced, reported, gone, and a
send that fails is the channel dead, which is section 5's account of a
death.

**No exchange carries a path, and no exchange carries lifecycle.** Admit
and release live on the residency seam and never here, the two seams
carrying two kinds of traffic being the decoder-cut ruling's whole point.

## 3. Ordering

- Open is first on this seam, happens once, and is valid only after the
  residency it serves is confirmed. An open before residency is refused
  and not queued.
- One generation is in flight at a time. A second append-and-generate
  while one is outstanding is refused and not queued, one turn behind
  one intent, per the grammar.
- Cancel is valid only after the session opens, and at rest it answers at
  rest, the validity window being the session rather than the generation.
- Flush is valid only between turns. A flush while a generation is in
  flight is refused, the cancel existing for exactly that case.
- The fault report is emitted only at rest, no exchange outstanding, any
  time after the session opens, and it takes no answer. A fault arising
  while an exchange is outstanding is that exchange's typed answer, per
  section 5, so a report never interleaves a stream.
- Messages within one exchange are ordered, and the intermediate token
  messages of append-and-generate all precede its answer.
- An answer to append-and-generate arrives only after the session is
  well-framed for the next turn, stopped or complete alike, so the
  harness may rely on the session's coherence without inspecting it.
- An answer to flush arrives only after the kept-prefix state holds.

## 4. What each party supplies and guarantees

Derived from section 2 rather than prose beside it, because every payload
change is a supplies change by construction and a Spec writer reads this
list.

**The harness supplies** the session's identity material at open, each
turn's context and delta, the cancel, and the flush. The sampling values
are not among them and reach the SPU in the declaration instead.

**The harness guarantees** that it opens no session before residency
confirms and no second session ever. That one generation is in flight at
a time. That every ask carries the turn's context and that it supplies no
sampling value on this seam at all, the operator-tunable remainder having
crossed once in the declaration and the frozen knobs being the binary's
and crossing nowhere. That it never touches the
cache, holds no handle to it, and derives nothing from the session but
what answers carry. That what it receives it authors faithfully, the
verbatim emission and the canonical parse both reaching the record, per
the operator's end-to-end requirement.

**The SPU supplies** the opened confirmation, the token stream as each is
drawn, each generation with its measurement and elected reductions, the
stopped partials, the flush confirmation, the typed refusals, and the
fault reports.

**The SPU guarantees** that the session only advances: nothing ever asks
resident state to rewind, and the weakest family sets the rule for all.
That the identity prefix is permanent from open to release, the flush's
outcome included. That an aborted generation leaves the session
well-framed before its answer returns. That the measurement is produced
at production time, positionally paired, absent rather than zeroed, and
that nothing is retained after the answer: produced, reported, gone. That
an overflow refuses, typed, and sheds nothing silently. That a refusal
leaves the session as the directive found it. That it answers a refusal
rather than exiting on one. That every token it streamed appears in the
close's account, so the stream and the answer never disagree about what
was drawn.

## 5. Failure

Refusals are typed and enumerable. The cases:

- the session is not open, or residency is not confirmed, for the ask
- the directive is out of order for the seam's state, a second generation
  or a mid-flight flush above all
- the session cannot take the next delta, the overflow named with the
  session's account of itself
- the delta is malformed for the family, under the framing candidate

**A fault inside a generation is the exchange's typed answer**, naming the
fault the way section 13.10 enumerates, and the turn fails with its
account on the record. **A fault outside any exchange is the SPU's
emitted report.** **A death is neither:** the harness observes closure, the run's
account shows what was authored, and what the harness does with a dead
decoder is the coordination seam's business, the fault travelling to the
record as the death's observation rather than the dead party's report.

**Nothing on this seam retries.** A refused ask returns to the harness,
and a re-sent generation would put two attempts behind one turn.

## 6. Prohibitions

**On the harness.** It opens no exchange this document does not
enumerate. It sends no path. It does not touch, flush by any private
means, or reason about the cache beyond the flush ask this seam carries.
It does not resend history the session already holds, because the resend
is the anti-pattern the append-only protocol exists against. It does not
treat an answer as authorization beyond its exchange.

**On the SPU.** It authors no trace event and holds no descriptor to the
record. It opens no exchange at all, the fault report being an emission,
and it has no exchange in which to ask, per section 2's dataflow rule, so
an SPU that needs the harness to act has a fault to report and never a
request to make. It retains nothing
across answers and nothing across the residency. It evicts and compacts
nothing, cognition being the harness's. It reaches no other crate.

**On both.** Neither carries a fact about the other's interior beyond
what the exchanges state. The harness does not know how the device is
allocated, the SPU does not know what a turn means, and the family
knowledge of `weaver-spu-PRD` section 14 lives on the SPU's side of the
line, which is the framing candidate's ground.

## 7. Vocabulary

Every contract names the vocabulary it depends on, grouped by the crate
that defines it, and a group is stated even when empty.

**Drawn from `weaver-types`:** the token trio, `token-directive`,
`token-answer`, and `token-refusal`, with the cases section 2 enumerates.
**The name extends the naming ruling and the extension is ratified with
this act.** The ruling names wire vocabulary for the loop whose traffic
it carries, and this seam's loop is loop 1, the builder's and variable
since the composability ruling, so a loop name is exactly what this seam
cannot take. The trio is named for the seam's currency instead, the
token, which is the naming ruling's second case, vocabulary for a seam
whose loop varies, ruled by the human at this act's merge and carried
into the floor's act with the definitions. The definitions land in
`weaver-types-PRD` section 2.3 and
are owed by this act, the demand existing now, and the records are
written unfenced deliberately so a mapper reading this document does not
ingest records this document is not the source of:

    node: token-directive
    kind: vocabulary

    node: token-answer
    kind: vocabulary

    node: token-refusal
    kind: vocabulary

    edge: defines
    from: weaver-types
    to: token-directive

    edge: defines
    from: weaver-types
    to: token-answer

    edge: defines
    from: weaver-types
    to: token-refusal

**Also drawn: `fault-report`**, added on the gate act of 2026-08-02, which
found that the gate's fault had no carriage and settled one definition for
every reporting seam. This seam carries it inside the trio's cases rather
than inside an envelope, the decode socket carrying none, which is the same
definition with a second carriage rather than a second definition.

The session and turn identities ride inside the trio's cases rather than
being drawn as vocabulary of their own: the floor holds `SessionId` and
`TurnKey` as satellite types, no vocabulary node exists for either, and
this contract draws none, the turn context of apex section 5.2 crossing
as fields of the cases the trio enumerates.

**Drawn from `weaver-traits`:** the message model, under the framing
ruling of `weaver-spu-PRD` section 13.4, ratified with this act: the
canonical messages cross this seam into the family library's render, so
the name crosses in one direction, the harness supplying and the SPU
consuming.

**Drawn from `weaver-trace`:** nothing. The SPU reports and the harness
authors, so no event kind and no envelope field crosses this seam. What
the answers carry is aligned with the three `model.*` payload shapes the
token workflow's trace act fixes, and alignment is not a draw: the trace
defines what the record holds and this contract defines what the seam
carries, with the harness converting as author.

**No organ envelope.** Stated as a negative deliberately, per section 0:
this is the one contracted socket seam in the program that does not draw it,
and the absence is the classification rather than an oversight. The egress
ruling of 2026-08-07 charters a second seam that will not draw it either, the
gate's agent-opened socket, and this sentence narrows to contracted seams
until the tool workflow authors that contract and the count is read again.

## 8. Conformance

How each check is implemented is Spec work. What must be checkable:

- A session never rewinds: no operation of this seam reduces the resident
  state except the flush, whose outcome is the kept prefix and never less
  than the identity prefix.
- The cut's bounds hold at their edges: a keep of zero, one inside the
  span, one at the resident count, and one beyond it each resolve to the
  clamped length, the confirmation's counts agreeing with the outcome in
  every case and the identity prefix never cut.
- An aborted generation leaves the session accepting the next turn's
  delta cleanly, watched to fail when the terminator step is removed.
- The stop lands within a token boundary of the cancel's arrival.
- The measurement is positionally paired and absent-not-zeroed.
- Readout reductions appear exactly when the residency was admitted with
  the election, and a tap failure while elected is a fault, never a
  silent absence.
- An overflow refuses before any partial append, the session unchanged.
- One generation in flight, enforced by refusal rather than queueing.
- A report is never emitted while an exchange is outstanding, a fault
  arising then being that exchange's typed answer.
- No sampling value crosses **inbound** on this seam, watched by a capture
  that fails when one reaches the SPU here. The check was written against the
  frozen knobs alone, when the tunable remainder travelled here, and it
  widens to the whole set with the remainder's move to the declaration.
  **The answer is excluded and the exclusion is the point.** `Generation`
  carries the request the SPU rendered whole, its template and its effective
  sampling among it, because the record holds what a turn ran with whichever
  side set it, per charter section 13.8. A capture reading both directions
  would fail on that and call the record a leak.

## 9. What this document changes elsewhere

Named here because a document whose reach cannot be read for the reach is
a trap. The authoritative register is `weaver-spu-PRD` section 11.

- `weaver-types-PRD` section 2.3: the token trio, owed by this act, per
  section 7.
- `weaver-harness-PRD` section 2: landed in this same act, the framing
  ruling ratified, per the register.
- `weaver-trace-PRD` section 3.1: the three `model.*` payload shapes, by
  the token workflow's trace act, aligned with section 2's answers.
- `basic-inference-loop`: the turn path's decode tine now has its seam
  and contract, a citation the next revision of that document carries.
