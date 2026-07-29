# weaver-types - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth for now. The crate PRD set is
written together and merged together, and no Spec is written against any member
before the whole set is merged. Ratification is the mapping of the whole document
set into the graph, and it belongs to the set rather than to this document.

**Date filed:** 2026-07-29
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

The declarative document that defines an agent. `weaver-admin` produces it, the
harness consumes it, and neither writes the other's half.

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

The `writes` edge from `weaver-admin` is declared in that crate's charter and is not
written here, which is the same deferral section 6 states in prose.

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

Every seam in this program is a `SO_PEERCRED` Unix socket, so every process that
accepts a connection must establish who is on the other end. The identity type lives
here because every process kind needs it and none of them may depend on another to
get it.

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

**Wire protocol vocabulary.** Not excluded on principle, absent on demand. Each socket
seam names what crosses it in its contract, and the shared representation of that
vocabulary lands here when the first such contract is written. No socket contract
exists yet, so nothing is owed. This is the clearest case in the corpus of a thing
that will obviously be needed and is still not written, because writing it now would
mean guessing at agreements that do not exist.

## 5. The terms of participation

**Declare what you use.** Every contract names what it draws from here, and the clause
is present even when the answer is nothing. The union of those clauses is this crate's
required surface: a definition no clause names is unused, and a definition a clause
names and this crate lacks is a gap.

**The producer and the consumer are both bound.** The agent state file has one writer
and one reader, and the format is an obligation on both. A producer that emits a shape
the consumer does not accept, or a consumer that tolerates a shape the producer never
emits, are the same defect seen from two ends.

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

The agent state file is a real producer-to-consumer agreement, but its parties are
`weaver-admin` and `weaver-harness`. That contract is written in the admin pass, when
admin's side can be specified rather than assumed. Writing it now would produce a
document describing one party's obligations against a charter that does not exist,
which is how the previous tree ended up with contracts that documented code instead of
governing it.
