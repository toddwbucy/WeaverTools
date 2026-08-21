# WeaverTools - Technical Report

**Status:** DRAFT. Outside the document set, never ratified, and nothing in the
corpus is written against it. Describes `WeaverTools` at `fbcb73e`, 2026-08-21,
per section 0.7. Section 0 is written and sections 1 through 16 are planned in
appendix A rather than drafted.

**Date started:** 2026-08-21
**Document ID:** `weaver-tools-technical-report`
**Editorial:** ASCII, no em-dashes, no semicolons.

Companion to `README.md` and to `WeaverTools-PRD`. The README is the doorway and
is short by design. The apex is the design and is checkable against the code.
This document is the walk between them, written for a reader who has neither the
corpus nor a reason yet to read it.

---

## 0. What this document is, and is not

### 0.1 What it is

A traversal of a settled document set, performed for a reader standing outside
it. At the commit this document describes the corpus is 27,266 lines under
`docs/`, one apex over nine crate charters, ten contracts and the material they
draw, and nine Specs, and it is written for the seats that build the system. An
engineer meeting WeaverTools for the first time will not read it, and should not
have to in order to learn what was built, how it is put together, and what it
does and does not do.

So this document walks the set once, in the order a reader needs rather than the
order the corpus is filed in, and it carries the reader to the corpus at every
step rather than standing in for it. The apex, the charters, and the Specs remain
the authority on every claim made here.

The motion is one the corpus already performs. A workflow document walks a
settled set and authors no edges of its own, per Document Format section 2, and
where it and a charter disagree the charter yields nothing. This document is that
same walk aimed outward instead of inward.

### 0.2 What it is not

It is not a fourth document kind. Working Process section 4 produces three kinds
at three levels and no fourth, and this document is none of them: it charters
nothing, states no contract, and elects no representation. It authors no node and
no edge, it carries no `graph` block, and the mapping does not read it. A reader
who finds a claim here and the same claim in a charter has found one statement
and its restatement, and the charter is the one that governs.

It is not ratified and cannot become so. Ratification is a property of the
document set, and this document is outside the set by construction rather than by
oversight.

It is not a specification, and nothing is built against it. A builder who wants to
implement against WeaverTools reads the contracts, which are written for exactly
that and carry the vocabulary, the errors, and the ordering guarantees each seam
relies on and provides, per apex section 5.3.

### 0.3 The register, and why it is the README's

The vision's section 0 fixes three registers and puts the program's ambition in
exactly one of them. The README stays on engineering principles with one theory
claim conceded and defended. The Spec corpus carries current work alone. The
vision carries the destination. A document released with the project is a fourth
register, and it takes the README's and extends it.

What that admits: the architecture, the mechanisms, the measurements, the
lifecycle, the failure cases, and the one conceded claim, that latency is the
enemy of agency, argued rather than asserted.

What that excludes: agentic-performance claims of every kind, the organism model
of cortex and brainstem and hippocampus, and the hypothesis the program exists to
test. Those are the vision's and the hypothesis document's, and they are excluded
here for a reason that is not modesty. Everything in this report is meant to be
checkable by a reader with the repository in hand. A claim about what an
individuated agent will eventually do is not checkable that way, and one such
claim standing beside forty checkable ones teaches the reader to treat all
forty-one as the same kind of statement.

The exclusion runs one direction only. This document names the hypothesis as the
program's motive, cites where it is stated, and does not restate or defend it.

### 0.4 The sourcing rule

Every claim in this report is one of two kinds, and a claim of neither kind does
not go in.

**A corpus claim** traces to a merged document by section, and the citation is
written rather than implied. Where the corpus states a thing in two places, the
report cites the one the corpus names authoritative, per G5.

**An artifact claim** names something a reader can inspect: a trace taken on a
stated date, a test that a stated instrument buys, a manifest, a fork revision, a
figure from the measurement regime with the conditions that make it comparable.

The rule is written down before the drafting rather than after it, because a
report a program writes about itself is precisely where unfalsifiable claims
enter that program. This corpus already holds the argument in its own terms: a
clean automated gate is evidence the gate did not fire and is not evidence of
correctness, per apex section 11. A report full of confident prose is the same
instrument with the same failure mode.

### 0.5 What this report may not claim

Four refusals the apex makes bind this document, and a report that quietly
dropped them would be selling something the corpus declines to sell.

**No run-again claim.** The loop is stochastic and does not reproduce, in any
arrangement, per apex section 8. A frozen seed narrows variance and buys audit
rather than determinism. The seed's per-generation derivation of 2026-08-21
removes the need to replay every draw preceding a generation in order to reach
its stream, which is a smaller thing than replaying a run, and the program still
claims no run again.

**No fungibility and no service reliability.** One agent on one machine gives up
horizontal scale, failover, and rolling replacement, per apex section 12. Nothing
in the program holds a contract with the world itself, at any depth.

**No end state.** The apex's own judgment, section 13, is that a local
proto-stateful agent is a defensible intermediate and an indefensible end state,
and that measured at stage one alone, giving up redundancy to co-locate such an
agent is the worse engineering choice. That sentence belongs in this report at
full strength.

**No claim on the far side of a tool.** What a tool reaches on its own side is
not this program's to bound, per apex section 3.

### 0.6 The publish boundary

This document is publish-destined, which makes it the one artifact in the
workspace where the boundary binds on every line rather than in principle.

Out: commercial, go-to-market, and strategy material of every kind, and any
distinction drawn between a single operator and a multi-tenant deployment. Organ
names that belong to the vision stay in the vision, and where this report needs
to name what they name, it says memory or state.

Visibility is checked rather than assumed. As of 2026-08-21 `toddwbucy/WeaverTools`
is private, so releasing this report names an act that has not yet happened, and
the boundary binds at that act rather than at this draft.

### 0.7 Currency, and the commit it describes

The system this report describes moves daily. The header names the commit and the
date the description was taken, and every figure in the report carries the same,
because a count with no commit beside it is a number nobody can check and a number
nobody can check is the thing this program spent a year learning to distrust.

A figure that has moved since the header's commit is a defect in the report rather
than in the system. Refreshing them is a pass over this document and not a
ceremony.

### 0.8 Vocabulary

The corpus holds its terms hard, and a report that softened them for an outside
reader would teach that reader a vocabulary the repository does not use. Three
rules follow.

**Where the corpus names a thing, the report uses that name.** Organ, seam,
socket, link, residency, election, disposition, assertion, and the rest carry the
meaning their charters give them, and the report glosses each at first use rather
than substituting a friendlier word.

**Where the README established a phrase the corpus does not carry, the report may
use it and cites the mechanism rather than the phrase.** The content-neutral
switchboard is the standing case. It appears in the README and appears nowhere in
the corpus, and what it names is checkable: the harness holds no weights and
performs no forward pass, it dispatches on payload kind rather than on content,
the instructions it carries to the SPU and the gate cross it uninterpreted, and
the gate relays opaque in both directions with order preserved.

**The forbidden vocabulary of the Working Rules binds here as it binds
everywhere.** No Id, Ego, SuperEgo, or Freudian framing. Trace, reflection, and
substrate-state are the canonical terms.

---

## Appendix A. The plan of the report

Sixteen sections, each named with the claim it makes and the sources it draws on.
This section exists so the shape can be ruled on before prose is written, and it
shrinks as sections land, the way the vision shrinks toward the built system.

**1. The problem, and the diagnostic instrument.** Agent failures are
unobservable because the traffic between the model and the scaffolding is
unobservable, and a conventional deployment smears the levels at which a failure
can sit across a network nobody controls. Warren Weaver's three levels as the
diagnostic. Sources: README, apex sections 1 and 13.

**2. What the system is.** One agent, one machine, three processes, and the organs
behind them. The organ test, both halves of it, and why `weaver-trace` is a link
rather than a socket while `weaver-state` is a member seam rather than an organ
channel. Why the harness is the hub and why that is the one allowance no other
crate holds. The switchboard reading is carried under 0.8's second rule. Sources:
apex sections 3, 5.4, 5.5, 12, `weaver-harness-PRD` 3 through 5,
`weaver-organ-channel`.

**3. The seams, and why not localhost.** Latency, security, and measurement, each
standing on its own, and what the trade gives up. The conceded theory claim,
argued here and nowhere else in the report. Sources: README, apex section 5.1.

**4. One turn, end to end.** The nine-step path from a line arriving at the gate
to a line leaving it. Sources: apex section 3, `basic-inference-loop`.

**5. The trace.** NDJSON with no framing layer, the closed kind set at nineteen,
the flattened envelope, the two clocks that answer different questions, and the
bracketing rule that every open has a close that says which kind it was. Sources:
`weaver-trace-PRD` sections 2 and 3, `weaver-trace-Spec` sections 2 and 3.

**6. Custody.** The write surface takes descriptors and never paths. Who opens the
sink, which flag rides the open file description and which rides the descriptor,
and why close-on-exec is set rather than checked. The threat walk: a tool that has
read `/proc/self/fd` and wants a second handle, and why it fails. Sources:
`weaver-trace-PRD` 4.1, `weaver-trace-Spec` 7 and 10,
`weaver-admin-harness-contract` 2 and 5, `weaver-state-PRD` 4.

**7. What holds state, and for how long.** The working structure in RAM, the
session, and the two things that deliberately outlive a turn. What the state
custodian ingests and the two asks it serves. Why nothing the agent can draw on
survives the session. Sources: `weaver-trace-PRD` 2.2, `weaver-state-PRD`,
`weaver-harness-state-contract`.

**8. The model organ.** Residency as the whole of what it owns, admission as the
one check on the device, the five admit steps and the point in them past which a
refusal stops being free, the two decode engines as peers, and the device judgment
read from the driver rather than from a ledger. Sources: `weaver-spu-PRD` 1
through 4, `weaver-spu-Spec` 1.1, 3, 4.

**9. Observability as an election.** The measurement payload, the residual-stream
readout, and the probability field, each elected per load, named individually in
the record, and refused at admit where the engine cannot honor it. The obligation
that an elected diagnostic changes no token. Why an absence has no shape. Sources:
apex 7.2, `weaver-spu-PRD` 13.6 through 13.11, `weaver-trace-PRD` 3.1.

**10. Replay, and what it is not.** The five inputs a record must carry, why
tokens are recorded rather than a seed, the template problem that makes canonical
messages insufficient, and the per-generation seed derivation. Closing on the
refusal of section 0.5. Sources: apex 8, `weaver-trace-PRD` 3.1 and 3.2,
`weaver-spu-PRD` 13.8.

**11. The lifecycle, and the deployment.** Loop 0 as the framework's own, the four
parties over three sockets, what refuses and what rolls back, loaded-and-idle as a
first-class state, and the shape a real installation takes on one machine.
Sources: apex 6, `load-unload-path`, `weaver-admin-PRD`, and the installed layout.

**12. The boundary.** What crosses the gate and in which direction, the port test
that decides internal from external, the grip as two contracts rather than one,
and the tools this framework does not build. The narrowing of 2026-08-18 belongs
here at its full strength: the gate holds one tool, the shell, as its own outbound
verb rather than as a guest, there is no tool table, a name that is not the
shell's refuses by name and never by nearest match, and the agent's wider roster
is scripts it writes in its own home and reaches through that one verb. Two
mechanisms carry the section: a tool result has exactly one construction site and
no route through serde or conversion, so a loop that would fabricate one has no
door, and there is no safety classifier and none is planned, because a heuristic
standing where a boundary already stands is the weaker of the two. Sources: apex
9, `weaver-gate-PRD`, `weaver-gate-world-contract`,
`weaver-harness-gate-contract`.

**13. The builder's seat.** What a builder writes and what they inherit, the
disposition on every knob, the compiled loop and why immutability is the feature,
and variance held to a range so that what remains is attributable to the thing
under study. Rendered in this report's register, with the motive left in the
vision. Sources: `weaver-harness-PRD`, `weaver-harness-Spec` 6.

**14. How it was built.** Documents first, a graph generated from those documents,
then code citing the assertions it conforms to. The four enforcement devices and
why a runtime test cannot pin the absence of a trait implementation. The
perturbation rule. This section is the report's least conventional and is placed
late deliberately, because it is the part a reader will weigh only after seeing
what it produced. Sources: Working Process, Document Format, apex 11.

**15. What stands today, and what does not.** The demonstration rather than the
count: a trace over its turns, both decode engines serving, the census at the
stated commit, and the open assertions named with what each waits on. The corpus's
own caution is the section's spine, that a crate can report a high conformance
figure while completing no turn and a completed turn is likewise not a count of
claims met, so the reader is pointed at the trace for whether the deliverable
runs. What does not stand belongs here at equal length and is named rather than
omitted: client-facing streaming is deferred and arrives as an extension to the
world contract rather than a replacement, a status ask refuses today because the
init system's three values do not map onto the four agent states and a
translation is where invention would enter, and the memory leg is out entirely
with a named door rather than a reserved slot. Sources: Working Process section 7,
the conformance headers, a captured trace, `weaver-gate-PRD` 8,
`weaver-admin-Spec`.

**16. What the trade costs.** The four refusals restated as engineering
consequences, the limitations a reader should weigh before adopting anything here,
and the question the apparatus exists to make answerable rather than answer.
Sources: apex 12 and 13.
