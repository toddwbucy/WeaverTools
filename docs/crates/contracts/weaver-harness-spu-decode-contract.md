# weaver-harness / weaver-spu - decode contract

**Status:** DRAFT. Cut with the token-workflow act of `weaver-spu-PRD` sections
13 and 14, per apex section 10, and merged on the human's call like every
document, the header flipping at that call per the Working Process. Written
and it governs the token seam: the second seam between the same parties, on its
own socket per the decoder-cut ruling of 2026-08-02. The residency seam keeps
its own contract, `weaver-harness-spu-contract`, and neither document restates
the other.

**Date filed:** 2026-08-02
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
duplex channel and the lifecycle channel is it. The channel mechanics this
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

Five. Four are opened by the harness and one by the SPU, which is the
duplex direction the residency contract deferred, arriving here because
the faults it carries are decode-domain traffic.

**Open the session.** Opened by the harness, once per residency, after
residency is confirmed on the lifecycle seam. It carries the session's
identity material, the canonical messages the identity prefix is rendered
from, per the framing ruling of `weaver-spu-PRD` section 13.4, ratified by
the operator with this act. The SPU renders through the family library,
establishes the resident session with the prefix resident, and answers
opened, or refuses, typed.

**Append and generate.** Opened by the harness, one per turn, carrying the
turn's context per apex section 5.2, the turn's delta as canonical
messages under the same framing ruling, and the turn's sampling values
for whatever knobs the binary left operator-tunable, per the dispositions
of `weaver-spu-PRD` section 13.8. The SPU appends the delta at the
resident end, generates, and answers with the generation and its
measurement: the token identifiers, the per-token signals, the timings,
the template identity, the block partition, and, where the residency was
admitted with readout elected, the residual reductions. The answer closes
the exchange and the turn's context returns with it.

**Cancel.** Opened by the harness while a generation is in flight,
carrying the turn's context and nothing else. The generation stops at the
next token boundary, the session is left well-framed with the family's
turn terminator resident, and the outstanding append-and-generate answers
with the partial output marked stopped. A cancel with nothing in flight
answers at rest, a clean close rather than a refusal, the same shape the
stop exchange takes on the coordination seam.

**Flush.** Opened by the harness between turns, never while a generation
is in flight. The session returns to its prefix-only state: the identity
prefix resident, the accumulated turns gone, the outcome fixed and the
mechanism the Spec's per family, per `weaver-spu-PRD` section 13.9. The
answer confirms after the outcome holds.

**Report a fault.** Opened by the SPU, the seam's one SPU-opened
exchange, carrying a fault of `weaver-spu-PRD` section 13.10 that arose
outside any outstanding exchange, residency degraded above all. A fault
arising inside a generation is that exchange's typed answer instead, so
one fact never travels twice. The harness authors what it is handed as
the `fault` event, per the fault-carrier ruling, and answers received,
which closes the exchange and promises authorship rather than any
remedy.

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
- The fault report may open at any time after the session opens.
- Messages within one exchange are ordered.
- An answer to append-and-generate arrives only after the session is
  well-framed for the next turn, stopped or complete alike, so the
  harness may rely on the session's coherence without inspecting it.
- An answer to flush arrives only after the prefix-only state holds.

## 4. What each party supplies and guarantees

Derived from section 2 rather than prose beside it, because every payload
change is a supplies change by construction and a Spec writer reads this
list.

**The harness supplies** the session's identity material at open, each
turn's context and delta, the tunable sampling values, the cancel, and
the flush.

**The harness guarantees** that it opens no session before residency
confirms and no second session ever. That one generation is in flight at
a time. That every ask carries the turn's context and that the values it
supplies for sampling are the operator-tunable remainder only, the frozen
knobs being the binary's and crossing nowhere. That it never touches the
cache, holds no handle to it, and derives nothing from the session but
what answers carry. That what it receives it authors faithfully, the
verbatim emission and the canonical parse both reaching the record, per
the operator's end-to-end requirement.

**The SPU supplies** the opened confirmation, each generation with its
measurement and elected reductions, the stopped partials, the flush
confirmation, the typed refusals, and the fault reports.

**The SPU guarantees** that the session only advances: nothing ever asks
resident state to rewind, and the weakest family sets the rule for all.
That the identity prefix is permanent from open to release, the flush's
outcome included. That an aborted generation leaves the session
well-framed before its answer returns. That the measurement is produced
at production time, positionally paired, absent rather than zeroed, and
that nothing is retained after the answer: produced, reported, gone. That
an overflow refuses, typed, and sheds nothing silently. That a refusal
leaves the session as the directive found it. That it answers a refusal
rather than exiting on one.

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
account on the record. **A fault outside any exchange is the SPU-opened
report.** **A death is neither:** the harness observes closure, the run's
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
record. It opens no exchange but the fault report. It retains nothing
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
this is the one socket seam in the program that does not draw it, and the
absence is the classification rather than an oversight.

## 8. Conformance

How each check is implemented is Spec work. What must be checkable:

- A session never rewinds: no operation of this seam reduces the resident
  state except the flush, whose outcome is the prefix alone.
- An aborted generation leaves the session accepting the next turn's
  delta cleanly, watched to fail when the terminator step is removed.
- The stop lands within a token boundary of the cancel's arrival.
- The measurement is positionally paired and absent-not-zeroed.
- Readout reductions appear exactly when the residency was admitted with
  the election, and a tap failure while elected is a fault, never a
  silent absence.
- An overflow refuses before any partial append, the session unchanged.
- One generation in flight, enforced by refusal rather than queueing.
- The frozen knobs never appear on the wire, watched by a capture that
  fails when a frozen value crosses.

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
