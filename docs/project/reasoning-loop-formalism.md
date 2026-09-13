# The Reasoning Loop as a Two-Clock System

**Status:** v0.8, 2026-08-12. Architecture-seat material, outside the document
set, landed by the authoring seat from the LaTeX of the same version, which was
the working artifact until this file. **Ratified by the operator, 2026-08-12, as
a foundations document**, standing between the vision document and the PRD
level: above the PRDs because it commits no crate to a mechanism, below the
vision because it is math. Named as an input to the WeaverTools technical paper
to follow, and as the core of a planned second formalization of the whole
agent, to which this document owes exactly one artifact, the trace. **Where
this document and a merged document disagree, the merged document yields nothing
and this one is corrected**, because the charters and the contracts are the
decision record and this is their shape read back in symbols. It authors no
graph records, so the phase two walk reads nothing from it. Change note against
v0.7: the membership test cites the ratified criterion, the tool line is drawn
as ownership against exposure, state management is described whole with its
channel, and record is reserved for the account.
**Document ID:** `reasoning-loop-formalism`
**Companions:** `reasoning-loop-boundary`, whose section 2 carries the ratified
membership criterion this document cites as its test.
**Editorial:** Per the Working Rules, with equations as indented blocks, exempt
from the width rule as the Format exempts indented and fenced material.

## 0. Abstract

An agent's reasoning loop is formalized as a two-clock system. Within a
residency the fast clock runs: a frozen binding of harness configuration and
model, a loop that drives, an operative state that is the sole basis for
action, and a tee at every clerking event that fans one event into two typed
legs, the complete account and the action-relevant selection. Across
residencies the slow clock runs: the operator revises the binding, and an
extraction reads the dead record to seed the next initial state. The account is
complete, append-only, and consulted by nothing within the residency. The
coordination seam carries lifecycle authority that acts on the clock, never on
what any transition computes. The document closes with an objective conditioned
on the moving initial state and one falsifiable prediction about a baseline
specimen.

## 1. What is being formalized

The subject is the reasoning loop, not the agent. Generally, when agents are
formalized, the formalization is A = H(M): the agent is the model, and the
harness is a function applied to it. That habit gives configuration two letters
and state none, and it obscures the object this document describes. The
reasoning loop is not the model with a wrapper. It is a coordinated set of
organs, and membership is decided by one test, the criterion ratified at the
reasoning-loop boundary: does the thing directly enhance the processing of
meaning. Read structurally, the criterion is coverage of a domain of reasoning,
since a domain of reasoning is a part of the processing itself and covering a
part is enhancing the whole. Three organs pass today. Semantic processing, the
SPU, covers the production of the action. State management covers the
preservation of what has just happened, holding what has passed between the
agent and the world as the basis reasoning is over. Coordination, the harness,
covers the routing: it takes what comes in, routes it internally to be
processed, notes it on the record, and assembles what goes out. Remove any one
and there is no reasoning loop. The test is generative rather than a label for
the current roster: a visual or an auditory processing unit would cover a
domain that extends the reach of reasoning, would pass the same test, and would
join the loop when built.

Two further organs attach to the loop without being of it. The gate and admin
are conduits, and the ratified criterion excludes them without special
pleading: neither enhances the processing of meaning, each supplies material
the processing works over, and neither holds a domain of reasoning. Each
carries one specific traffic across the loop's edge, the gate carrying turn
traffic and admin carrying lifecycle authority inward and trace custody
outward. Section 2 draws both. The harness's constant traffic with them is not
a foot in their class: coordination is the domain that consists of interacting
with every other organ, the way a brainstem touches everything it regulates,
and reaching the conduits is that domain exercised, not membership in it.

Two mistakes follow from misplacing the loop's boundary, and they are one
category error committed from opposite sides. Collapsing state into the harness
denies a constitutive organ its standing: state is not an internal activity of
the harness. It is the basis reasoning is over, and there is no reasoning about
state when state is not present. Pulling tools inside the loop promotes a
non-constituent: a tool covers no reasoning domain. It supplies specific
context to the loop on demand, and what belongs to the loop is the call that
requested the context and the context that returned, never the tool itself. The
first mistake also breeds a build error, because filing the record and the
basis in one organ tempts an implementation where they share a substrate
unforked, and the tee of Section 3 exists to forbid exactly that.

One boundary larger than the loop's is named and then left alone. The reasoning
loop lives inside an agent. The agent is individuated by its uid, authored by
the operator before any process exists, and bounded at the operating system.
That wall has two faces, ownership and exposure: a tool reached within what the
kernel scopes to the uid is owned and lives inside the agent's edge, and a tool
reached across the port namespace is exposed and lives outside it, so the loop
is held at a distance from exposure by construction. The formalization of that
wall, together with the gate Spec's open tool-uid ruling, is a second
document's subject and is not attempted here. This document owes that subject
exactly one thing: the loop it formalizes, whose trace the second formalization
builds on.

At the center of the loop's mechanics is one mechanism. The record and the
basis are different roles separated by a fork at the clerking event. Sections 3
through 8 formalize the fork and what it protects.

## 2. Objects, domains, and the driver

**The binding** c = (H, M): harness configuration and model, fixed together at
load, constant within a residency.

**The loop**: the harness's logic loop, the driver of the fast clock. Drive is
sequencing and sole authorship of the clerking, not initiation: the loop is the
only actor that does anything with what lands, its own calls expect returns,
and three kinds of landing arrive unbidden, typed in Section 3, the frame, the
directive, and the report.

**The operative state** S_t: the loop's basis for its next action. Set at seed
time, updated within the run, dead at unload. The model refers to the state and
to nothing else.

**The constitutive organs**: the harness, the SPU, and state management, one
per reasoning domain, drawn below. Each is a chartered surface of its domain,
none is a peer of the loop, and callability is the SPU's property, not the
class's.

**The conduits**: the gate and admin, drawn below.

**The trace** Tr_t: the complete account. Append-only, carrying everything the
loop clerked including production-time measurement, durable to the extent the
operator elects a sink.

The organs sort into three reasoning domains, and the sorting is architecture,
not taxonomy for its own sake.

**Coordination** is the harness. The loop lives there. It reads the gate, calls
the SPU, clerks every event, and holds the descriptor through which the account
reaches the operator. Coordination is the only domain that moves.

**Semantic processing** is the SPU, where the model sits. The SPU provides
specific semantic functionality on call: given a basis, it produces the action.
The action's producer is the SPU domain, not the model as a floating organ, and
the SPU's output is a return to a caller, not an organ speaking. The SPU also
originates exactly one message that answers no call, the fault report, and
Section 3 types it.

**State management** is the organ whose domain is temporal context storage and
retrieval. It is the tee's state-leg consumer, and it holds a duplex channel
with the harness: the harness hands it the selection, and it hands back the
basis the loop reads next. The composition of the state is open cell 1, and the
realization of the channel is open cell 5, held under the pointwise property of
Section 8.

**The conduits.** The gate carries the turn: content, prompts, and returns from
the world land there as frames, and the deliverable leaves there. Admin relays
both ways, lifecycle authority inward and trace custody outward, holds nothing
during the run, and Section 5 gives it its own treatment. The conduits are the
two places the loop faces outward, so the agent's boundary is named at both:
the gate is the loop's edge, not the agent's. The agent's edge is the operating
system's, drawn around the uid with ownership and exposure as its two faces,
and the fork of Section 1 decides which tools live inside it. Tools appear
nowhere in the organ inventory because they are not organs. The boundary is
categorical and admits no exception: every tool is on the far side of the gate,
and a tool return lands as gate-relayed evidence. Removing tools from the
inventory simplifies the evidence type rather than complicating it. Evidence is
whatever the gate relays, frames and tool returns alike, one source, one type.

## 3. The tee

The loop calls a function, the function returns, and the loop clerks the
return. That is the common case, and the direction of expectation types the
rest. The loop's calls expect returns. Three kinds of landing arrive unbidden.
The frame is world: it lands at the gate, it is owed an answer, and it grants
the seat. The directive is seam traffic: it lands from admin, it is owed an
answer, and it acts on the clock, never on what any transition computes. The
report is interior: it is owed nothing beyond its accounting, and the SPU's
fault report is the one SPU-originated message in the set. This trichotomy
comports with the merged dispatch law, whose entered-state wait spans exactly
these three channels. Comportment is the claim's whole strength: the corpus
tests the same trichotomy the formalism asserts, so a perturbation that breaks
the dispatch law breaks this section with it. That is a shared exposure, not
evidence, and the document's one evidential surface remains Section 10.

The clerked event set is the landings together with the clerk's own marks. A
turn bracket, a load event, a stop-marked close land from nowhere: the loop
authors them in the act of clerking its own conduct. They are clerked events,
and they are not landings. Let o_j be the jth clerked event, indexed by
clerking, so that a step contains however many clerkings it holds, the request,
the output, the frames, the reports, the directives, and the clerk's marks:

    delta_j = W_c(o_j, m_j)      (trace leg: the complete increment)
    sigma_j = sel_c(o_j)         (state leg: the action-relevant selection)

and the two consumers advance together:

    Tr_{j+1} = Tr_j || delta_j
    S_{j+1}  = T_c(S_j, sigma_j)

m_j is production-time measurement: timings, per-token signals, custody and
lifecycle content. The trace leg carries it. The selection drops it. sel_c is
fixed by the binding, mechanical, and lossy on purpose: it excludes fields that
are present and true because they are not action-relevant, and that deliberate
exclusion is the mechanism, not an accident to be minimized. Every clerked
event passes through the same fork, and the selections sort by kind. A return
may select into the state. A frame that parses does: a line the harness cannot
parse answers as a refused turn, and what any of it contributes to the basis is
part of the parse question the corpus defers with the gate's turn half. A
directive within the residency is clerked with empty selection, which is
Invariant 2 restated as a tee fact. A report's selection is empty by the
projection discipline, which is Section 9's narrowed question. The clerk's
marks select nothing, being lifecycle and measurement content, exactly what the
selection exists to drop. One landing sits outside the tee altogether: the
enter directive's seed arrives before the fast clock exists, and Section 5's
invariant names it the one legal boundary write.

**Invariant 1.** The selection is a pointwise, load-fixed function of the
clerked events, never of the structure as a whole. The two legs of the tee
share a source and neither reads the other. There is no arrow from Tr to S on
the fast clock, not because the account is sealed away but because a pointwise
selection cannot see what it does not select. The corpus's own sentence is the
right form: a filter at the read site, not a judgment applied after a full
read. Both legs are functions of the same clerked events, which is why the
account cannot lie about the basis: they cannot diverge about what happened,
only about how much of it they carry.

The pointwise form is the load-bearing property. When the selection is
pointwise and fixed at load, it does not matter whether it is applied at write
time to an event in flight or at read time to an event at rest in the admitted
structure. What it computes cannot depend on the structure as a whole, and that
property is what lets Section 8 settle the realization question by theorem
rather than by inspection.

The signature check follows. The state leg's inputs are drawn entirely from the
action and the evidence through sel_c, so state evolution takes exactly state,
action, and evidence, and an input from outside that set means something is
entering that should not.

Three violations:

1. **Leakage.** sel_c admitting measurement or administrative content into the
   state leg as though it were conversation.
2. **The recall breach.** The account rendered back into the state as record,
   the model consulting its own excretion as an account of itself.
3. **A whole-structure read.** Any path by which the selection becomes a
   function of more than the event being rendered, summarizing, reordering,
   deduplicating, or selecting over the live account as a whole.

An implementation with any of the three is not a variant. It is a different
agent. Shared origin is the design. A bad fork is the violation.

**The engine, an election rather than physics.** An engine offers the tempting
analogy, and it is not exact. Drive torque and exhaust both come off the same
combustion event, and in an engine without feedback trim the exhaust does no
work on the next stroke because the geometry routes it out while the torque
routes forward. But real engines run closed-loop on their own exhaust: the
lambda sensor trims the next stroke's fueling from exhaust content, exhaust gas
recirculation routes it into the intake, a turbocharger does work on the next
stroke with it. Feedback on the account is available, ubiquitous, and mature
engineering elsewhere. This architecture elects against it, with grounds,
measurement, liability, and observability, the same three that carry the load
boundary. The tee is an election, not physics, and the formalism is stronger
for saying so, because an election can be defended where physics can only be
asserted.

## 4. The fast clock, driven

    a ~ pi_c( . | S)
    e ~ Env(a)

with the transition given by the tee's state leg. The policy reads the state
and nothing else. A turn opens at the gate, world speaks first, and the opening
frame is clerked like every landing: if it parses, its selection enters the
state before any action is drawn. From the frame onward the loop drives. It
calls the SPU when it needs the next action, commits that action through its
calls, clerks what comes back, and the results of those calls are returns to a
caller. Evidence is what the gate relays. Action is what the loop commits.
Nothing else is either.

**Termination is the loop's own.** The loop halts a turn when its halt
predicate fires:

    j* = min { j : h_c(what the loop holds at j) = 1 }

h_c is fixed by the binding, owned by the loop, and evaluated over what the
loop already holds. It introduces no new input class, so the signature check is
untouched: nothing enters, the loop decides. No called function can force a
halt by returning a value, because returns are clerked, not obeyed. A turn
ended by the halt predicate closes with whatever was already clerked, the
partial standing in the record via the trace leg, and the state holding
whatever selections had landed. The terminal action of a turn is the
deliverable, the emission owed to the flow, its delivery is the gate's relay,
and its transition takes the degenerate form

    S_final = T_c(S_{j*}, sel_c(o_{j*}))

A toolless turn is exactly one such pass.

## 5. Admin, the coordination seam, and the lifecycle

Admin is the second conduit and the second place the loop faces outward, so the
agent's individuating fact is named here as it was at the gate. The uid is
authored by the operator before any process exists, and admin is the crossing
mechanism: the instantiation of the agent across that boundary is admin's work,
exercised through the verbs below, and the identity material the enter
directive carries is how the authored fact becomes a resident one.

**Per-invocation, holding nothing.** Admin holds nothing during the run. Each
operator verb dials the coordination seam anew, and the connection dies when
the verb answers. A loaded agent with no admin attached is the ordinary resting
state. What exists at runtime is one listener the harness binds and one sink
descriptor the harness holds. There is no standing admin socket, and there are
not two of anything.

**Two verb sets, and admin as translator.** The operator speaks three verbs to
admin: load, unload, and validate. What reaches the harness's seam is a
different set: enter, leave, and stop. The harness never hears the word load.
Admin's job is the translation, one authority relayed across two vocabularies.

**The stop is chartered.** The stop is the seam's fifth exchange, ruled and
merged in the admin-harness contract, with its mid-stream mechanics chartered
in the harness Spec and a perturbation assertion waiting on the wiring act.
Nothing is left to wire. The framework's command surface is not a floor the
operator builds on. It is the complete set.

**The contract is closed at run time.** The harness opens no exchange the
contract does not enumerate, and no reserved slot exists for one. The command
set can grow, and the operator holds full authority to grow it, but the growth
is a documents act: the contract is amended, the seam visibly moves, and the
change is ratified through the front door. Section 11 states the discipline as
the kit's own.

**The trace leaves by custody, not by socket.** The account does not flow
through an admin-held socket. Admin opens the sink under root before the run,
the descriptor crosses the seam exactly once inside the enter directive, and
the harness appends to the operator's sink directly for the life of the run.
The operator contract's sentence, that the stream exits at admin, is a custody
statement: admin owns the arrangement, the harness does the writing.

**Invariant 2.** Scoped to the fast clock: within the residency, nothing on the
coordination seam lands in the state. A lifecycle command determines when
transitions cease or begin, never what any transition computes, and no seam
traffic becomes a field the model reads. At the boundary the scope ends,
because there the seam is how state begins: the enter directive carries the
identity material whose decoder instruction becomes the resident prefix, the
content of S_0. That seeding write is the one legal write the seam performs on
the state, and it happens before the fast clock exists to be protected.

Admin decides none of this. Admin relays, both ways: lifecycle authority
inward, trace custody outward. The operator authors the verb, admin translates
it, and the loop obeys its own logic and what reaches it through the seam.
Authority runs operator to loop, and the stop still needs no third input class
beside action and evidence. Loop-issued, it is the driver halting itself,
interior to h_c. Operator-issued, it is chartered seam traffic acting on the
clock. In neither case is it evidence, and in neither case does the signature
check bend.

## 6. The trace as complete account

The account's three properties, stated over the tee:

1. **Append-only and complete.** Every clerked event lands, in full,
   measurement and the clerk's marks included. The account extends and is never
   revised. A turn closed early, by either origin of stop, leaves its partial
   in the account, because the trace leg had already carried everything clerked
   before the clock closed.
2. **A faithful witness.** Because both legs are functions of the same events,
   the boundary-visible trajectory tau, the sequence of actions and evidence,
   is recoverable from the account by construction. Section 10 leans on this.
3. **Written for the operator.** Durability is the operator's election per the
   standing sink-kind ruling. The program tees to the sink and its obligation
   ends there.

The account also extends between turns. A clerked interior fault with no turn
open lands on the trace leg as a report, the landing that answers no call and
is owed nothing back. Whether its selection is empty is Section 9's narrowed
question.

## 7. The slow clock, and the boundary's two crossings

    c_{k+1}    = O(c_k, D_k)
    S_0^(k+1)  = X_k(Tr'_k)

D_k is derived from the emitted record of residency k and drives the operator's
revision of the binding. X_k is the extraction, the operator-held selection
that reads the emitted durable record of a dead residency and sets the initial
state of the next, the prime marking the record at the sink rather than the
in-RAM structure, which died with the process. X and sel are kin: the same job,
selecting the action-relevant and dropping the surplus, on different clocks
with different inputs. sel reads a clerked event within a live residency. X
reads a dead account after the residency that wrote it has ended. The kinship
is why the extraction is safe where a whole-structure read inside the residency
would be a breach: the record it consults cannot be influenced by the state it
produces.

**Invariant 3.** Within a residency the binding is constant and the account is
consulted by nothing. Every path by which past conduct influences future
conduct runs through the operative state on the fast clock, or crosses a load
boundary through X or through O. Both crossings are the boundary's, and the
boundary is the sole point at which the account re-enters causation.

**The moving object.** Because S_0^(k+1) is a function of the prior record, the
initial state moves with k by construction, and the objective of Section 10
conditions on it. The objective is therefore J(c, S_0), jointly, and the
discipline is stated as part of the formalism: hold X fixed across any
comparison of bindings, or score the pair (c, X) jointly and say so. The
recursion side-condition, that the exterior objective be held fixed across k
for any cross-residency claim, stands for the same reason: with two things
moving on the slow clock, every comparison must state what it froze.

**Open cell.** The extraction X has no contract. The corpus has already dug its
hole: the enter cell of `weaver-admin-PRD` section 10 holds what a later run
holds of a session, and this cell anchors there rather than floating. A working
rule bearing on what X's output must satisfy, that backend-neutral inputs
should render to an equivalent logical starting state, stands in the working
list's open items as a standing item, origin named, status marked unratified,
its documents act still owed.

## 8. Realization

The tee specifies a fork at write time. The merged corpus realizes both roles
over shared substance with a projection at read time. The two agree
extensionally exactly when the read-time projection equals the concatenation of
per-event selections, and the merged Spec answers that question affirmatively,
with instruments.

**Closed cell.** The merged assembly floor is pointwise. Harness Spec section 5
states the property in its own words: the harness assembles a prompt by
iterating the working structure in sequence order, selecting on the lifted
kind, and the measurement, lifecycle, and custody events never enter a prompt
because the assembly path cannot see them. The filter is the kind set at the
read site, not a judgment applied after a full read. This is not an
implementation accident. It is a merged assertion,
`harness-assembly-kind-filter-at-read-site`, tagged perturbation, sitting
beside `harness-deterministic-assembly` and `harness-prompt-part-order`. And
the decode seam enforces pointwise on the wire: each turn crosses only its
delta, resending history is the anti-pattern the append-only protocol exists
against, so whole-structure recomputation on the model path is not merely
undisciplined, it is unsendable. The fusion ground, that nothing is derived and
nothing can diverge, is a theorem of the design, carried by three named
instruments and a wire protocol.

The equivalence clause carries three caveats, absorbed here so the claim is
exact. First, the rendered context includes two segments sourced from the
binding rather than from events, the identity prefix and the tool schemas, so
the realization is c-material plus concatenated selections, not concatenated
selections alone. Second, the family template applied on the SPU side is fixed
at load and per-message but position-aware, a generation header knows it
follows the last message, so the property is pointwise up to position. Third,
flush, when any workflow uses it, revises the state outside sel-concatenation.
It is licensed for S, whose composition is open cell 1, and the realization
claim scopes to the render path.

The consequence stands armed as a test: a future implementation that
summarizes, reorders, deduplicates, or selects as a function of more than the
event being rendered is not a variant realization. It is violation three, and
the instruments above are the tests that would catch it.

## 9. The fault cell, narrowed

Mechanically, today, a fault event is clerked to the account and its selection
is empty, because a fault is not conversation and sel_c selects nothing from
it. That is in force by the projection discipline, and world-first survives
whole. The remaining question is whether anyone will ever want sel_c widened so
that interior faults become action-relevant to the agent that suffers them, a
future documents act with a real cost: an agent that reads its own faults is an
agent whose evidence set includes landings that never crossed the gate. The
cell stays open at that width and no wider.

## 10. Objective, variance, and one prediction

For a task distribution Q and an exterior evaluator G,

    J(c, S_0) = E_{g ~ Q} E_{tau ~ P_c( . | g, S_0)} [ G(tau, g) ]

where tau is the boundary-visible trajectory induced by the fast clock from
S_0.

G is a function of tau, and tau survives the residency nowhere except the
account. The evaluator therefore reads the account, as witness. That is the
faithful-witness property doing its work: the account contains what was asked
and what was answered, the trajectory is recovered from its conversation
component, and what G must not read is the state, which is interior, and m,
which is administration.

**Constraint 1.** G is exterior to the binding. No component of c computes,
stores, or revises G.

**Proposition 1.** If G is a function of c, then max over c of J is degenerate:
a binding that rewrites its own scorer attains any score. Exteriority is a
condition for the objective to be an objective.

**Proposition 2.** J(c, S_0) is estimable from emitted records only if c is
recoverable from them. The model half is closed by the standing ruling that
records name the model and weights hash. The harness half is open, and until
the load event carries an admitted identity for H, estimates of J pool over
harness configurations they cannot distinguish.

Variance decomposes by the law of total variance over Q and the within-task
stochasticity of the fast clock, and paired comparison lives in the within-task
term, defined over trajectories read from the witness, never over increments,
whose measurement content diverges trivially between any two runs. Delta is
reserved for effect size wherever a paired design is specified.

**Provenance.** The baseline specimen is an operator-held record: a benchmark
session of a standard retrieval-augmented agent, forty turns on the bench
platform, prior program's material, consumer-side like the platform it ran on.
The governed corpus holds no such record. If it cannot be produced for whoever
runs the comparison, the test degrades without vanishing: it begins at the
first fresh baseline run, which everything below binds identically.

**Prediction.** Paired repeats of the specimen's task, same binding, same
initial state, same task, will show low trajectory divergence together with
high task error. The runs will fail, and they will fail the same way, because
the failure is a property of the binding, not of the draw: the right fact sat
in the basis, was recited, and did not arrive as action-relevant at the
decision point, three times.

**Falsifier.** High trajectory divergence at high error. If paired repeats fail
differently each time, the failure is stochastic, the structural reading is
wrong, and the argument that the remedy lies at the binding and its boundary
loses its specimen. The section is written so that it can.

## 11. IS and IS-NOT

**This formalism IS** a behavioral account of one reasoning loop at its
boundary: a frozen binding, a loop that drives, an operative state, a tee at
every clerking, an account that is complete and read within the residency by
nothing, a coordination seam whose traffic acts on the clock, and a boundary
with exactly two causal crossings.

**It IS** the corrective to A = H(M): the missing letter was state, the missing
mechanism inside state was the fork, and the missing subject was the loop. H is
not a function applied to M. H holds the loop that calls M.

**It IS** a description of a kit, not of a wiring. The framework ships loop
zero, the service, the wait, the clerking, and the granted seat, together with
the chartered command surface of the coordination seam. Loop one is the
builder's, compiled at the worker composition root, which in the kit's own
metaphor makes loop one the wiring. The line is soldered versus wirable: the
board and its soldered service are shipped, the wiring is the builder's, and
this document describes the board and the legal connections, not any one agent
built on them.

**It IS** a kit under amendment discipline. Any future change assumes a change
to the underlying contract, and that is the design working rather than friction
around it. The seams are built to be highly visible precisely so that no change
can happen quietly: a change that touched no contract would be a change that
escaped the boundary the architecture exists to enforce. A moved seam is a
documents act somebody authors and somebody ratifies, and the grounds are the
load boundary's own three, measurement, liability, and observability. A change
visible in the diff of a contract is a change that can be measured, attributed,
and audited. The operator's authority over the command surface is full, and it
is exercised at design time, through amendment, never through a reserved slot
at run time.

**It IS NOT** a formalization of the agent. The agent is the larger thing,
individuated by its uid and bounded at the operating system, holding this loop
together with the tools that live inside its edge. Its formalization, including
the ownership and exposure faces of its wall, is a second document, and it
builds on the trace this one defines.

**It IS NOT** a claim that the account is inert by substance. The roles are
separated by a pointwise selection fixed at load, the merged structure realizes
that separation by theorem, and the instruments that test the theorem are named
in Section 8.

**It IS NOT** an implementation. The composition of the state, the contract of
X, and the width of sel remain named and handed to the seat that holds the
code.

**It IS NOT** confirmed by agreeing with the corpus. Agreement is a consistency
check, and the alignment of Section 3's trichotomy with the merged dispatch law
is a shared exposure, not a confirmation. The document's one exposed surface is
the prediction of Section 10, and it is written to be wrong there in an
informative way if it is wrong.

## 12. Open cells, gathered

1. Composition of the operative state. Awaits: an inventory of what the harness
   holds and renders between steps.
2. The extraction contract for X, anchored to `weaver-admin-PRD` section 10's
   enter cell. Awaits: what X may read, must preserve, and must not invent. The
   seed-state rule, a standing item in the working list, awaits its documents
   act.
3. The width of sel. Awaits: a future documents act, if ever, admitting
   interior faults as action-relevant, with the stated cost.
4. The harness half of identifiability. Awaits: whether the load event carries
   an admitted identity for H.
5. The realization of state management's channel. Awaits: the socket and
   contract by which the domain arrives as its own structure, the corpus rule
   being a schema extension plus a new socket and contract, never a retrofit,
   under the pointwise constraint already in force.
6. Symbols. W, X, sel, h, and Q are this document's own until ingestion. Delta
   is reserved for effect size, leaving delta to the increment. Frame is the
   corpus's word for what lands at the gate, and this document uses it. The
   loop's own clerked acts are the clerk's marks, this document's coinage,
   awaiting a corpus name if one is ever chartered.

(Bucy, 2026)
