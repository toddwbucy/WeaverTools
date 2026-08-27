# The protoautonomic calculator: an experiment design

**Status:** PROPOSED. Written 2026-08-27 from the thinkpad seat, against
`main` at 42a29e6. Nothing here is built. This document states the property
under test, the fixture that tests it, and what must exist first, so the
code act that follows has a written target rather than an intention.

**What this is not.** It is not a comparison of which dispatch is faster or
which an end user should prefer. That question has a different answer per
deployment and belongs to whoever builds a loop for a purpose. This asks the
framework architect's question instead: **does the mechanic hold, provably.**

---

## 1. The property under test

`weaver-agents-PRD` section 4 item 6 requires the deliverable to fire at
least one protoautonomic tool call, "where the harness injects a
deterministic result into the stream in place of a stochastic one, with the
calculator as the reference case." Section 8 states what a record owes:
deterministic re-feed requires, exactly, the input token ids, the output
token ids, model identity and weights hash, the sampling parameters, and the
prompt-block partition, **with tokenization reproducible from what is
recorded**, and a claim about the state rests on the tee's election beyond
that list.

**The tokenization clause is not incidental here.** The calculator answers a
string, and a string reaches the resident sequence only by being tokenized,
so the requirement injection touches most directly is the one about
tokenization being reproducible from the record.

The theorem this experiment exists to settle:

> **Injection and elision preserve deterministic re-feed.** A session in
> which the harness supplied a computed value where a sampled one would have
> gone, and in which spans were removed from the resident sequence, replays
> from its own record exactly, and a consumer reading that record can
> reconstruct what the model held at every turn.

**The baseline is measured, not assumed.** The run of 2026-08-27 drove 3,010
sessions over four conversation depths and eight prompts spanning mean token
entropy 0.149 to 7.411, and every one replayed bit-exact: 43,620 turns,
348,960 field comparisons, zero divergences. That is the plain-turn case.
This experiment asks whether the property survives the two operations that
edit context, and the baseline is what a result here is compared against.

**The loop is not claimed to reproduce.** Apex section 8 is explicit that
the loop is stochastic and this program makes no run-again claim. The claim
is about the record, and the record's sufficiency is the whole question.

## 2. Why the calculator, and why it is not an arbitrary choice

`weaver-internal::calculator::evaluate` answers the same string for the same
expression forever. That is the entire reason it is the instrument. **A
divergence under a deterministic tool is attributable to the technique**,
because nothing else in the arrangement could have caused it. Under a tool
that reads a clock, a filesystem, or a network, a divergence is
unattributable and the experiment answers nothing.

Arithmetic also supplies a correctness axis that most agent work lacks:
`17 * 23` has one right answer, so an arm that is fast and wrong is visibly
worse rather than merely different. The cue is unambiguous, which lets the
fixture's elections be scripted rather than inferred.

The calculator is built and unreached. It carries three conformance
citations and five passing tests, and no crate depends on `weaver-internal`.
That is the state this experiment acts on rather than a defect it reports.

## 3. The two arms are one initiation and two dispatches

Apex section 4 settles the framing and this experiment adopts it whole:
"Initiation is what these words name, and dispatch is a separate question.
Whether an action was elected or initiated is settled by who moved first,
which the trace records... Where the harness then routes it, inward to the
organ whose domain it belongs to or outward through the gate as ordinary
output, follows from what the action is for and changes nothing about what
it was."

So the arms are not two techniques:

- **Arm INWARD.** The elected call is dispatched to `weaver-internal`,
  evaluated in process, and its result injected. No seam is crossed.
- **Arm OUTWARD.** The same elected call is dispatched through the gate's
  execution exchange as `ToolExecution`, answered `ToolOutcome`, and the
  result re-enters as an ordinary message.

**Both tools are internal by the port discriminator** of 2026-08-07, which
classifies a tool by whether it binds a listening port rather than by where
it runs. Bash binds none and neither does the calculator. The arms differ in
dispatch, not in classification, and this document uses inward and outward
for the route and never external for either.

**What the arms are for here.** Arm OUTWARD is the control. It exercises a
path that already works, so a divergence in it indicts the harness or the
record rather than the injection mechanic. Arm INWARD is the path under
test. Running both over identical problems is what separates a fault in the
new mechanic from a fault in the apparatus around it.

**The arms differ in resident length by construction**, since one returns
the result as a message and the other injects it, and that shape has bitten
this corpus before: `weaver-spu-PRD` section 13.8 records a defect that
"would have eaten a paired comparison silently, the two arms differing in
resident length by construction, crossing the flush threshold at different
turns, and one reseeding where the other did not". **The coupling that
defect rested on was closed by the sampler ruling of 2026-08-19**, and this
experiment elects no flush, so the hazard is named and answered rather than
left for a reader to raise. Section 6's comparison is within an arm in any
case, source against replay, and never across arms.

## 4. The fixture loop is scripted, and that is the design

A loop that decides intelligently when to elide and when to elect the
calculator would introduce a second stochastic element beside the model's
own generation, and a divergence under two stochastic elements is
unattributable. **The fixture's elections are therefore fixed in advance**:
elide this span at this turn, elect the calculator on this expression. The
loop is an instrument that holds the arrangement still, not an agent that
decides well.

This is the answer to how the mechanic is tested before a real loop exists.
It is not tested by switching between arrangements by hand and reading the
outcome, which is an arrangement whose result cannot be attributed either.
It is tested by a fixture whose every election is written down, so the only
thing left free is the model, which is the thing the record exists to
capture.

The problem set is a chain of arithmetic steps long enough to build context
pressure, so the elision has something to remove and the two arms diverge in
context growth for a reason that is stated rather than incidental.

### 4.1 The arrangement is bare, and the exclusions are the design

**Nothing beyond the trace participates.** No consumer reads the record
while the run proceeds, no analysis leg is attached, and the agent queries
nothing. The record is written, the run ends, and only then is it read for
comparison. An experiment whose subject can consult its own history is
testing something else.

**The seat is eight calls**, per `docs/technical/weaver-agents/loop.md`, and
the fixture uses two. That page also records a live discrepancy worth
knowing, the loop file's own header saying seven where the connector exposes
eight, rechecked 2026-08-24. The exclusions are listed because each is a
confound this experiment is built to avoid, and the list is complete against
the eight so that no port is silently neither used nor refused:

| Port | In the fixture | Why |
|---|---|---|
| `turn` | yes | the thing under test |
| `assembled_empty` | yes | the first-turn test a scripted chain reads, carrying no history |
| `elide` | yes | the context edit under test, its span scripted |
| `fullness` | recorded only | a count carried by the last generation, never a decision input |
| `recall` | no | custody's query, and a subject that can query its history is a second variable |
| `session_shape` | no | a state query, for the same reason |
| `classify` | no | a second organ, whose behaviour would ride the result uncredited |
| `flush` | no | a second context edit, and two edits in one session make a divergence unattributable to either |

**The flush exclusion is this document's own and not inherited.** The flush
and the elision are both context edits and both are chartered, so a fixture
electing both would be testing context editing rather than the elision, and
a divergence would name no operation. The elision is tested alone or the arm
establishes nothing specific.

**The bareness is a property of the deployment and not only of the
fixture.** The reference agent carries no state section in its declaration,
so the leg is absent and `recall` and `session_shape` answer the absence the
contract already defines for a leg that is down. The fixture therefore
cannot reach custody even by mistake, which is a stronger guarantee than a
rule the fixture follows.

**One consequence to record before it is forgotten.** Apex section 8 rests a
claim about the state on the tee's election, and the elision edits state. A
record produced with no state leg and one produced with a leg support
different claims, so this experiment's result covers the arrangement it ran
under and is not evidence about the stateful case. Naming it here stops a
clean result being read later as broader than it is.

**Why bareness is what the diagnostic work needs.** The analysis and
diagnostic crates are built against records. A corpus produced while
custody, classification, and a second context edit were all firing would
hand that work a confounded substrate whose properties could not be
attributed. The output of this experiment is meant to be usable as a known
quantity, and that is what the exclusions buy.

## 5. What the record must carry, and the gap this found

Elision is already answerable. `weaver-trace-PRD` section 3.1 carries the
`elision` kind, twentieth, holding the span the loop named beside the
resident counts either side, and the accumulation rule has a consumer replay
the record's edits rather than accumulating contributions naively. A
consumer can therefore reconstruct the resident sequence across an elision.

**Injection is not answerable, and this is the blocking finding.** The word
does not appear in the trace charter. The record has no member that
distinguishes a token the harness supplied from a token the model sampled.
Two consequences, and the second is the serious one:

1. A reader of the record cannot tell which tokens the harness supplied.
   **The election itself is recoverable** and this document earlier said
   otherwise: `weaver-trace-PRD` section 3.1 carries the tool bracket and
   `weaver-types`' `Generation.content` carries every recovered call as a
   `ToolCall` block in emission order, so who moved first is recorded as
   apex section 4 requires. What is unrecoverable is the provenance of the
   **result span**, which is the narrower and still blocking claim.
2. **A claim about the token path may be silently wrong.** Re-feed feeds
   recorded output token ids back through the forward pass and re-samples
   nothing, so an injected token re-feeds like any other and the arrangement
   appears to hold. It holds by accident. Nothing in the record marks the
   position as one the sampler never drew, so a consumer computing anything
   from the sampler's behaviour over that span reads a draw that never
   happened.

**So the trace act precedes the code act.** The record must gain a way to
say that a span was supplied rather than drawn, and what supplied it.
Whether that is a member on the measurement, a payload on the tool bracket,
or a kind of its own is the trace seat's to settle. This document asserts
only that the property in section 1 cannot be claimed until it exists.

**The tool bracket's payloads are deferred rather than absent**, per
`weaver-trace-PRD` section 3.2: "The tool bracket's two are the remaining
pair and stay deferred with the tool workflow." That matters to whoever
takes the act, because it makes this a deferral coming due rather than an
omission to be argued for from nothing.

A second, smaller gap is named here because the experiment will want it:
**the record carries no first-token timestamp.** Every post-request event of
a turn is authored in one batch at completion, verified across 32 of 32
turns sampled, so time to first token is derivable from `prefill_ns` and one
decode step but is not recorded and cannot include queueing. Latency is not
what this experiment claims, so this does not block it. It blocks describing
what the arms cost.

## 6. What falsifies the claim

Stated first, so the result is read against a written bar:

- A replay of an arm's session diverges on any compared field. The
  comparison is the eight-field set the cross-precision harness already
  applies: rendered prompt, derived generation seed, effective sampling
  knobs, emission bytes, finish kind, resident count, input token ids, and
  per-token entropies.
- A consumer replaying the record's edits reconstructs a resident sequence
  that disagrees with the counts the SPU reported at any turn.
- The record cannot say which positions were supplied rather than drawn.
  This falsifies the claim today, per section 5, and is what the trace act
  must clear.
- The calculator answers differently on replay. This would indict the
  instrument rather than the technique, and it is listed so that outcome is
  never read as a fault in injection.

**A clean result is not a claim about efficiency.** It says the mechanic
holds and the record accounts for it. What it costs, and when a deployment
should elect it, is the loop-building work this experiment precedes.

## 7. Order of work

1. **The trace act.** A supplied span is representable and marked. Without
   it section 1's property cannot be claimed, only appear to hold.
2. **The dispatch act.** The harness gains its dependency on
   `weaver-internal` and an inward route, so an elected call reaches the
   organ whose domain it belongs to. Apex section 9 places the injection
   half here.
3. **The fixture.** The scripted loop and the arithmetic chain, with every
   election written down.
4. **The run.** Both arms, replayed and compared against the plain-turn
   baseline of 2026-08-27.

Steps 1 and 2 are `weaver-trace` and `weaver-harness` acts and sit in the
agent domain rather than this seat's lane. The operator assigns them.

## 8. What this experiment does not settle

The collocation claim in its general form. Injecting at the point where a
stochastic token would otherwise go requires being in process at the
sampling boundary, and no seam-crossing arrangement reaches it. This
experiment demonstrates the mechanic on one deterministic tool. It does not
establish what the technique is worth across tools, models, or deployments,
and a result here should not be read as making that larger claim.
