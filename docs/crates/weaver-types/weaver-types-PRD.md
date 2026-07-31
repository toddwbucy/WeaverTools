# weaver-types - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth for now. The crate PRD set is
written together and merged together, and no Spec is written against any member
before the whole set is merged. Ratification is the mapping of the whole document
set into the graph, and it belongs to the set rather than to this document.

**Date filed:** 2026-07-29
**Revised:** 2026-07-31. Section 2.3 goes from three wire definitions to five, adding
`organ-envelope` and `harness-alert` owed by `weaver-admin-harness-contract` section 8,
and section 4's departure paragraph follows the count. Revised again the same day:
sections 5 and 6 stop resting on admin writing the configuration file, the writer
being the operator and the readers two, and the deferred state-file contract is
recorded as ruled out rather than pending. Revised a third time: section 2.1's edge
sentence, wrong on both halves, now states that no writes edge exists and that the
contract is ruled out.
**Document ID:** `weaver-types-PRD`
**Parent:** `WeaverTools-PRD`
**Depends on:** `weaver-traits`
**Editorial:** Per the Working Rules.

---

## 1. What this crate is

Where `weaver-traits` holds the contracts the engine is written against,
`weaver-types` holds **the declaration that configures an agent and the identity by
which processes recognize each other**. It binds no socket, spawns no process, holds
no handle, and depends on `weaver-traits` and nothing else internal.

That dependency is a floor link and not a lateral edge. Both crates sit on the floor
under the same root, so an untagged edge between them would read as the sibling
dependency G3 forbids. It is declared as what it is.

```graph
node: weaver-types
kind: crate

edge: parent
from: weaver-types
to: WeaverTools

edge: floor-link
from: weaver-types
to: weaver-traits
```

**It is a requirements document, not a catalogue.** Under the apex vocabulary-clause
rule every contract names what it draws from here in its vocabulary clause, and a
contract without that clause is not valid. This charter states what a crate must satisfy
to participate, and the terms are enforced at every seam rather than in one document
nobody opens.

For a data kernel the compiler is the primary enforcement: the struct definitions are
the schema and the serialization derives are the format. This charter carries only
what the compiler cannot, which is semantic invariant, compatibility posture, and the
obligations that run from a producer to a consumer.

## 2. What it holds

Everything here is present because the harness demonstrably needs it. The set grows
when a later crate demands more, by the ritual: a crate PRD added or changed updates
the floor in the same act.

### 2.1 The agent state file

The declarative document that defines an agent. The operator produces it, and both
`weaver-admin` and the harness read it. Neither crate in this program authors it,
because creating an agent is an operator act and the file is its declaration, per
`weaver-admin-PRD` section 1. Admin validates it before a process exists and the
harness consumes the elections it carries.

**The identifier is wrong and its correction is owed rather than taken here.** The file
is configuration, read at load and fixed for the run, and the agent's state is the
trace. Renaming the node reaches this section, its five `holds` edges, and every
citation in the corpus, which is one act on the corrections branch rather than a
change made beside a producer correction.

```graph
node: agent-state-file
kind: artifact

node: model-binding
kind: vocabulary

node: tool-set
kind: vocabulary

node: permission-mode
kind: vocabulary

node: residual-readout-election
kind: vocabulary

node: verbosity-ceiling-election
kind: vocabulary

edge: defines
from: weaver-types
to: model-binding

edge: defines
from: weaver-types
to: tool-set

edge: defines
from: weaver-types
to: permission-mode

edge: defines
from: weaver-types
to: residual-readout-election

edge: defines
from: weaver-types
to: verbosity-ceiling-election

edge: holds
from: agent-state-file
to: model-binding

edge: holds
from: agent-state-file
to: tool-set

edge: holds
from: agent-state-file
to: permission-mode

edge: holds
from: agent-state-file
to: residual-readout-election

edge: holds
from: agent-state-file
to: verbosity-ceiling-election

edge: elects
from: permission-mode
to: permission-mode-vocabulary

edge: elects
from: tool-set
to: tool-trait
```

The last two are the floor's own argument in edge form. `weaver-traits-PRD` section 2
rests the two-crate floor on one fact, that the state file names a permission mode and
a tool set whose definitions therefore cannot sit above the floor. Those are fields
here electing from vocabulary that crate defines, and without an edge the single
argument the floor's structure rests on is prose the graph cannot see.

Each field is a vocabulary node this crate defines and the artifact holds, because a
contract's clause draws a field rather than the whole file and an edge needs
something to point at. The five records above are the five fields this section lists,
so a clause naming a sixth has no target and the mapping says so.

There is no `writes` edge from any crate, because the writer is the operator and the
operator is not a node the graph carries. Both crates that touch the file declare
`reads`, each in its own charter, and section 6 rules the once-deferred contract out
rather than deferring it.

Known fields, from the passes already done: the model binding, the tool set, the
permission mode, the residual-readout election, and the verbosity ceiling election.
The last two arrived from the trace and harness passes rather than from this crate's
own reasoning, which is the demand rule working as intended.

Two of those fields are named in `weaver-harness-trace-contract`'s vocabulary clause,
so the state file is already a declared dependency of a contract this crate is not
party to. That is the normal shape: this crate supplies vocabulary that agreements
between other crates are written in.

**A field takes effect at load and is fixed for the life of the run.** A session
whose configuration changed mid-run is one every consumer afterward has to reason
about, and the lifecycle already provides the place to change it. Unload, edit, load.

**Every field here is run-scoped, and none is sealed.** A run is a load, so this file
is read at every load and the value it carries governs that run and nothing wider. An
operator editing between runs of a live session is using the mechanism the lifecycle
provides rather than working around one. A consumer needing to know how a given run
was configured reads it from that run's record, which is why `weaver-trace-PRD`
section 4.3 records verbosity per run rather than once for the session.

Run scope is what this crate declares. Whether a particular consumer wants a
particular field held steady across the runs of one session is that consumer's
concern, argued in its own charter against its own harm, and it does not become a
property of this file by being wanted.

An earlier version of this section made some fields session-scoped and sealed at
`run0`, naming the verbosity ceiling election as the instance. That scope is
withdrawn. It also left the residual-readout election, which was never sealed, able to
enable readout against a session sealed floor-only, producing reductions no event kind
was licensed to carry. That interaction closes with the scope that created it.

### 2.2 Peer identity, and one bounded policy carve-out

The seams that admit an outside principal and the seams that cross the membrane are
named `SO_PEERCRED` Unix sockets, so a process accepting a connection on one of them
must establish who is on the other end. The identity type lives here because more than
one process kind needs it and none of them may depend on another to get it.

**The claim is scoped to those seams and was once stated of all of them.** The earlier
wording made every seam in the program a credentialed socket, and
`weaver-admin-harness-contract` falsifies it: an inherited unnamed pair reports the
creating process at both ends, distinguishes nothing, and authenticates by possession
instead. That contract draws neither definition below and says so. A universal with a
counterexample is a weaker seat than a scoped claim that holds, and the scoped claim is
what the carve-out rests on.

**Alongside the identity type, this crate carries the authorization predicate, and
that is a deliberate exception to holding only data.** The rule that decides whether a
given peer may reach a given agent is enforced independently by more than one process,
and one shared definition in a floor crate is the only way separate processes provably
enforce the same rule. Three implementations of one rule are three chances to disagree,
and the disagreement is a privilege boundary.

**The carve-out is bounded and the boundary is the important half.** This crate holds
the identity type and the predicate, and nothing else. Where the rule is applied, how
a failure is handled, how an access list is loaded, and what happens on refusal all
live in the consuming crates. **A second policy function arriving here is a charter
violation to be flagged rather than absorbed**, because the argument above justifies
exactly one exception and does not generalize into a licence.

```graph
node: peer-identity
kind: vocabulary

node: authorization-predicate
kind: vocabulary

edge: defines
from: weaver-types
to: peer-identity

edge: defines
from: weaver-types
to: authorization-predicate
```

### 2.3 The wire vocabulary the first socket contract draws

**These five arrived on demand and not in advance.** Section 4 held wire protocol
vocabulary out of this crate until a written contract needed it.
`weaver-admin-harness-contract` section 8 is that contract, and it draws exactly the
five values below. Nothing else enters this subsection until another contract draws
it, and a sixth definition arriving without a contract behind it is the reserved-slot
error apex section 9 forbids, in schema form.

**Four of the five belong to the coordination seam.** `admin-directive` is what the
asking party sends across the coordination channel. `harness-answer` is what the
answering party returns. `lifecycle-refusal` is the typed form of a refusal, carrying
why a lifecycle act could not be performed rather than a free string, so that a caller
branches on a value and an operator reads a reason. `harness-alert` is the fault the
harness raises unasked, which is the direction the duplex channel exists to carry and
the reason that seam is not simplex.

**The fifth belongs to the floor and not to that seam.** `organ-envelope` is the
carrier every organ channel draws, holding the exchange a message belongs to, that
message's position in the exchange, and the type of its payload. It is defined here
because the coordination seam was the first channel to need it, and it is the one
value in this subsection no later contract draws as new, because apex section 5.4
makes a duplex channel with the harness the test of an organ and every such channel
carries this envelope. The mechanics this record serves live in `weaver-organ-channel`
as of the lift of 2026-07-31, and that document declares no records of its own, so this
record stays here with the other four and each organ contract draws it in its own
vocabulary clause.

The meaning of each, its ordering against the others, and the failure modes are the
contract's and are not restated here. This crate holds the representation the two
processes must agree on, which is the whole of what a floor definition is.

```graph
node: organ-envelope
kind: vocabulary

node: admin-directive
kind: vocabulary

node: harness-answer
kind: vocabulary

node: lifecycle-refusal
kind: vocabulary

node: harness-alert
kind: vocabulary

edge: defines
from: weaver-types
to: organ-envelope

edge: defines
from: weaver-types
to: admin-directive

edge: defines
from: weaver-types
to: harness-answer

edge: defines
from: weaver-types
to: lifecycle-refusal

edge: defines
from: weaver-types
to: harness-alert
```

## 3. What it must not hold

**No logic beyond the section 2.2 predicate.** The carve-out is one rule with one
justification, not a category.

**No internal dependency other than `weaver-traits`.** The state file names a
permission mode and a tool set, which is why that edge exists and why the floor is two
crates rather than one.

**No socket, no process, no handle.** The definitions describe what crosses a boundary.
Crossing it is somebody else's crate.

**Nothing one crate needs.** A shape only the harness reads belongs in the harness.

**No anticipatory schema.** A field added because something will probably want it is a
reserved slot in data form, which apex section 9 forbids as plainly as the interface
kind. The residual election and the verbosity ceiling are here because two completed
passes demanded them, not because a configuration file felt incomplete without
them.

## 4. What does not cross

**The task schema.** The previous tree carried a task work-spec with benchmark, chat,
chatroom, and training-loop kinds. This program has no task surface: work enters
through the gate as turns, and the apex is explicit that no prompt, turn, task, or run
enters through the administrative plane. The previous tree's own issue #350 records
that its agent worker implemented no task executor, so the schema described a
capability that was never built.

**Per-agent path constants.** That tree shared root-custodied path constants between
producers and consumers. Under the custody model the harness never resolves a path,
and `weaver-admin` holds the paths it opens, so there is nothing for a shared constant
to coordinate. If the admin pass finds a path two crates must agree on, it enters then.

**Wire protocol vocabulary was held here and has left.** It was absent on demand
rather than excluded on principle, and the demand has fired.
`weaver-admin-harness-contract` is the first socket contract, it names five values
that cross, and their shared representation is now section 2.3. Nothing was guessed:
each of the five arrived because a written agreement needed it. The rule that placed
the paragraph here still governs everything not yet drawn.

## 5. The terms of participation

**Declare what you use.** Every contract names what it draws from here, and the clause
is present even when the answer is nothing. The union of those clauses is this crate's
required surface: a definition no clause names is unused, and a definition a clause
names and this crate lacks is a gap.

**The producer and the consumers are all bound.** The agent state file has one writer,
the operator, who stands outside the program, and two readers inside it, admin
validating before a process exists and the harness consuming the elections it
carries. The format is an obligation on all three. A producer that emits a shape a
consumer does not accept, or a consumer that tolerates a shape the producer never
emits, are the same defect seen from two ends, and two readers that disagree about
one file is that defect a third way.

**Compatibility posture is stated, not assumed.** A field added, removed, or given a
new meaning is a change to every producer and consumer of the file in the same act.
Absence is never read as a default unless the charter says that field is optional and
says what its absence means.

**A change here is loud by design.** Everything above links this crate. The ritual
carries it: a floor change updates every affected PRD and contract in one act, and a
change that cannot be carried in one act has not been thought through.

## 6. The seams

This crate is party to no contract. It defines and does nothing, so there is nothing
to agree to, and it is **named in** contracts rather than signing them.

The agent state file is a real producer-to-consumer agreement, but its producer is the
operator and its parties are not two crates. An earlier form of this paragraph
deferred a contract between `weaver-admin` and the harness to the admin pass, and that
pass ruled the other way, in `weaver-admin-PRD` section 10: authorship moved to the
operator, both crates that touch the file read it, so there is no producer inside the
program and no producer-consumer agreement between two crates to write. Only seams
take contracts, per Working Process section 4. This charter is the named authority on
the format under G5, each reading charter carries its own validation obligations, and
no third document exists.
