# weaver-admin - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth for now. The rulings that
reshaped this charter are recorded in it rather than pending against it, and section
10 now carries a shorter list than it did. The header moved on the human's ruling of
2026-07-31 with no other edit.

**Date filed:** 2026-07-29
**Revised:** 2026-07-31, twice. First, four entries left the section 11 register as
their edits landed and one entered for the apex 5.2 scoping. Second, sections 4.1 and
4.2 were cut to the one-seam ruling of section 6, the fan-out moving inside the enter
and leave directives, section 5's rollback became admin's own reap plus a directive,
and section 3's count of drawn values went from three to five, correcting a defect
the receiving seat had repaired four times against resends from this copy. Third,
the descriptor cell of section 10 stopped offering admin forking as a peer
alternative and adopted the contract's wording on what an absent interface reopens,
and 4.2's opening line was rewritten for the list it now has.
**Revised:** 2026-07-31, a fourth entry this date. Section 3 splits activity control
across the seam and corrects the network-ingress restatement, and section 8 gains the
stop conveyance. Per the rulings of this date carried by
`basic-inference-loop`.
**Revised:** 2026-07-31, a fifth entry this date. Section 4.1 step 7 orders
projection before the `load` event, per the resume exchange of
`weaver-harness-trace-contract` section 2.1, the section 11 entry for apex 5.2
takes the approved scoping wording, and section 4.4's sequence reads load, stop,
unload, the close conveyance removed, the verb being the stop exchange that already
exists. Per the edit register of this date.
**Revised:** 2026-07-31, a sixth entry this date, hygiene in one act. The operations
log of section 2 stops naming a `destroy` this charter rules admin does not perform,
section 5's rollback clause splits step 7 at the `load` event, section 7 points the
tool-uid cell at the gate charter rather than the consumed stub, section 11 cites
gate G7 as in force and drops the register entry for the spu stub note, whose
substance the merged spu charter carries directly.
**Revised:** 2026-07-31, a seventh entry this date, the subtraction batch. The live
view is retired under ruling A and the integrity witness under ruling B: the
three-tier append-only account reduces to the working structure's structural
sentence, the tamper-evidence limit and the integrity trigger, frequency, and window
leave section 2, the recompute conveyance leaves sections 6 and 8, and the seam table
carries the four exchanges that remain. Leave-time validation, the manifest, and the
drain stand untouched pending the durable-record ruling.
**Revised:** 2026-07-31, an eighth entry this date. Device arbitration leaves this
crate entire, per ruling C: the section 2 fleet-arbitration paragraphs, the load's
arbitration step, the denied-arbitration log instance, and the arbitration Spec
child all come out, the load renumbers to seven steps, a device conflict becomes an
admission refusal inside the enter fan-out landing in section 5's bracket case, and
the rhetorical uses of the word device rename to instrument so the one token means
the GPU.
**Revised:** 2026-08-01, a ninth entry. The operator surface takes its contract,
`weaver-admin-operator-contract`, written as a blocker before any Spec per the
human's ruling of this date: section 8 cites it and section 10's operator-to-service
cell closes.
That contract also records the durable-record ruling of the same date, durability
being the operator's with the program's obligation ending at the NDJSON tee, and the
cut it scopes is registered on the open-items list rather than landed here.
**Revised:** 2026-08-01, a tenth entry, the durable-record cut. The program owns no
record, per the ruling at `weaver-admin-operator-contract` section 3: section 2's
custody restates from the record to the stream's sink and drops the divergence
material, the load opens a sink rather than a record, unload reduces to the drain
with the validation account gone, session close loses the checksum and manifest,
the run-bracket and reader-edge cells leave section 10 with the GID-mask staged
item, and a new cell names what enter becomes without a record, deferred to the
memory-and-state round. The cut handoff's rider lands with it: step 6 of the load
drops the word together, an orphan of ruling A's live-view deletion.
**Revised:** 2026-08-01, an eleventh entry, the fault-carrier ruling. The harness
opens no exchange on the coordination seam: a fault the worker survives travels as
the `fault` event on the stream rather than as an alert to admin, section 6's seam
table and duplex account follow, and the duplex property restates as the
channel's rather than the exchange census's.
**Document ID:** `weaver-admin-PRD`
**Parent:** `WeaverTools-PRD`
**Companion contract:** `weaver-admin-harness-contract`, written with this document
**Editorial:** Per the Working Rules.

---

## 0. What this document is

The charter of the crate that verifies the agent's operating-system boundary and
drives the agent through its lifecycle verbs. It is written together with
`weaver-admin-harness-contract`, which governs the one seam this crate holds during
a transition, and neither is complete without the other.

Level discipline, stated once. This document carries what the crate needs and why,
including the order in which a transition happens, because the order is admin's own
work rather than a wire agreement. What crosses the seam, what it means, and how it
fails is the contract's. How any of it is represented is the Spec's and appears in
neither.

## 1. What this crate is

**The fleet's lifecycle driver, and an unprivileged one.** One admin, many agents,
and it is not a constituent organ of any of them. Where `weaver-harness` is mortal
and dies with its agent, this crate outlives every agent it drives, which is the
first of the two facts that put lifecycle here. The second is that a harness cannot
drive the early steps of its own creation, because the worker spawn runs before the
harness is running as the harness at all.

```graph
node: weaver-admin
kind: crate

edge: parent
from: weaver-admin
to: WeaverTools
```

**It verifies the agent's boundary and authors none of it.** Apex section 12 rests
the whole regulation model on the agent being an operating-system user whose reach
the kernel bounds. That model has a writer and this crate is not it. The agent's uid,
its group memberships, its home directory and the modes on it, and the trace
directory the agent uid cannot search are the operator's to establish before an agent
is created, by whatever means that operator's site already uses to admit a principal.
Admin reads that boundary, checks it against what a load requires, and refuses a load
it cannot confirm.

**Creating an agent is not creating a user, and the separation is the ruling.**
Admitting a principal to a system is administrator authority over the operating
system's own trust model, and a program that took it would be raising a second trust
model above the one it claims to inherit. So provisioning leaves this crate entirely
and leaves the program with it. What remains is an act on an identity that already
exists, which is a different and much smaller kind of work.

**The cost of that is stated rather than glossed.** This charter guarantees the
boundary by checking it and no longer by building it, and those are different
strengths. A boundary that passes verification is the operator's artifact, produced by
a script this corpus does not govern, and the program ships a check rather than a
constructor. That is the same move the corpus made when reading and analysis left in
section 8, and it is right for the same reason, but a later reader must not take the
charter to defend a property it only confirms.

**The agent has no path to its own supervisor.** This is the property the whole
custody model reduces to, and it is structural in two independent ways. The
coordination channel of section 6 has no name in the filesystem, so nothing running
as the agent uid can dial it. And admin's code is never linked into the worker's
address space, so no build exists in which a harness process contains supervisory
code that a bug or an elected tool could reach. The second is the one that stops the
shortcut where someone adds a repair path to the worker because the function was
already compiled in.

The property is cheaper to hold than it was. A supervisor that authored boundaries had
to keep authorship away from the agent, and this one has no authorship to keep.

## 2. What this crate owns

**Authorization of lifecycle intent.** Whether a given operator principal may run a
given verb against a given agent. This is external authorization, it happens before
anything else is touched, and a refusal leaves the system exactly as it found it.

**Verification of the boundary, which is a different thing from its authorship.**
Per section 1, the OS identity, the home directory that is the agent's sandbox, and
the trace directory with its modes and group memberships are the operator's artifacts.
What this crate owns is the check: that the identity resolves, that the home exists
with the ownership and modes a load requires, and that the trace directory is
admin-owned and not searchable by the agent uid. A boundary that fails any of these
refuses the load, and nothing here is repaired.

**Validation of the agent's configuration file.** The operator writes it and both this
crate and the harness read it, per `weaver-types-PRD` section 2.1. It is declarative,
read at load, and fixed for the run, carrying the model binding, the tool set, the
permission mode, and the two elections. Validating it before a process exists is this
crate's, because a file naming a model binding admin cannot satisfy must fail the load
at the cheapest possible moment.

```graph
edge: reads
from: weaver-admin
to: agent-state-file
```

**The node name in that record is wrong and is renamed in a later act.** The artifact
is configuration rather than state, and the agent's state is the trace. The identifier
is `weaver-types`' to change, so the record above keeps the resolving name until the
rename lands in the same act as `weaver-types-PRD` section 2.1, per section 11. What
changes here is the direction, which is this charter's to state.

**Directing a transition, and receiving the aggregate result.** Admin directs the
transition across its one seam and the harness sequences it, each organ performing its
own operation, and the transition publishes only after every organ has confirmed to the
harness and the harness has returned the aggregate to admin. Apex section 6 gives the
harness the word coordinator and the act of sequencing the organs with it, and nothing
here contests it. What admin owns is the directive and the aggregate it gets back, which
is a different act from coordinating a turn, and the two never run at the same time.

**Custody of the stream's sink, and the agent reaches none of it.** The program
owns no record, per the ruling of 2026-08-01 at `weaver-admin-operator-contract`
section 3, and durability is the operator's. What admin owns is the connection: it
opens the sink the agent's configuration declares under its own principal and
passes descriptors into the enter directive, so the worker writes a stream it could
not have opened for itself. Where the sink is a file, the operator's provisioning
keeps the agent uid off it by ownership, mode, and the directory's search bit,
which admin's boundary check verifies rather than builds, the same posture as
section 1, and a kernel check rather than a concealment, since a process holding a
descriptor can always `readlink` its own `/proc/self/fd`.

**The agent writes through a descriptor and by no other route.** Permission is
checked once when the sink is opened, against the opener, and a descriptor passed
by `SCM_RIGHTS` installs against that same open file description with no recheck.
So the worker writes a stream it could not have opened for itself, which is the
whole of its access and is revocable by closing the descriptor.

**The working structure in RAM is append-only by construction, because its API
exposes no update path.** That is the one append-only claim this charter carries, a
structural property of the in-RAM store rather than a witnessed discipline, and
integrity accounting past the emission boundary is the consumer's business on the
consumer's compute.

**A log of admin's own privileged acts.** A completed rollback and a refused load
are privileged acts that leave nothing behind
otherwise. A refusal reaching an operator is a typed `lifecycle-refusal` returned on
the socket of section 8 and needs no log to be legible, but an act that completed with
no operator holding the return has no reader at all. Admin holds a log file for those.
It is admin's own file, admin is its sole author, and it is not the trace and shares no
schema with it. **It records acts of the supervisor and never conduct of the
supervised.** The moment it carries a fact about what an agent did, it is a second
record of the agent with a second author, which is the arrangement the trace's single
writer exists to prevent. Format, retention, and rotation are the Spec's, and are not
settled here on the grounds that a log format decided before a rollback has run is a
format decided from no measurement. The artifact is named in the mapping regardless,
because deferring a format is not a reason to leave the sole record of the privileged
half of the lifecycle invisible to it.

**Custody, stated because every other artifact in this corpus has its access argued.**
The log is owned by `weaver-admin`, grouped to `weaver-admin`, mode 0640, in a
directory owned by `weaver-admin` at mode 0750. It is fleet-scoped rather than
per-agent and it never lands inside an agent home, which is the load-bearing half. The
named adversary is the agent uid, and it is excluded twice over: it is neither owner
nor group, and the directory's missing search bit means it cannot reach the file to
try. An agent that could read this file would read the record of its own supervision,
which is the same class of hole the trace directory's search bit exists to close. The
operator reads through membership in `weaver-admin`, which is the same path by which
the operator reaches a session record.

**The stream's sink and this log are secured against the agent and against nothing
stronger, and the party that concerns is the operator.** Custody is exclusion of
the agent rather than evidence against the holder. This is not a gap the charter
can close, because the operator is the party that admits principals, configures the
delegation of section 7, owns the sink, and holds what accumulates behind it. The
program secures the agent against reaching its own record and does not secure
anything against the operator, who is trusted by construction. The apex states that
trust model once, per section 11, and this paragraph is its local half.

```graph
node: admin-operations-log
kind: artifact

edge: writes
from: weaver-admin
to: admin-operations-log
```

**Admin concludes nothing about the record, because it holds none.** The comparison
an earlier version of this section emitted, the reference direction it argued, and
the repair it forbade all presupposed a program-owned file to compare against the
working structure, and each dissolved with it on 2026-08-01. What survives is the
posture: admin repairs nothing, adjudicates nothing, and calls nothing good, over
any artifact it touches.

## 3. What this crate does not own

**The turn, in any part.** No prompt, turn, task, or run enters through this crate,
per apex section 6. The line is worth stating in its live form rather than its
abstract one, because the operator-facing surface of section 8 is where it will be
tested: reporting an agent's state, listing agents, and driving a verb are in bounds,
and carrying a prompt to a loaded agent is out however convenient a menu makes it.

**Activity control's mechanics.** The abort of a turn belongs to the harness and
returns the agent to loaded and idle. What this crate holds is the conveyance: the
operator's intent to stop crosses as the stop exchange of the contract, one bit and
no work, because the operator holds no other crossing. Admin carries the intent and
never the mechanics, reaches nothing of the turn's content, and unload still waits on
rest rather than racing it.

**Trace authorship.** The harness is the sole writer of the record. Admin authors no
event, holds no event kind, and its own first contact is recorded by the harness as
the `load` event of `run0` rather than by an entry of its own.

**Reading the stream as content.** Admin connects the sink, which is custody, and
custody is not comprehension. Parsing events is the operator's tooling's business
on the other side of the sink, per `weaver-admin-operator-contract` section 6, and
admin interprets no content in either direction, the monitoring being the outside's
job and the verb being admin's, per the basic loop's section 2.

**Provisioning, in every part.** The OS identity, the home directory, the modes and
group memberships on it, and the agent's configuration file are the operator's, per
section 1. Admin creates no principal, chowns nothing, and edits no account. A verb of
this crate that found a boundary missing refuses and does not build one.

**Model admission, residency, and release, and the device entire.** The SPU admits
the model, holds residency, releases, and is the one authority on the device, per
apex section 6 as relocated under ruling C. Admin arbitrates no hardware and reasons
about the device at no point: a conflict is discovered at admission, refused there,
and the refusal travels SPU to harness to admin inside the enter aggregate.

**Network ingress.** There is none. Gate binds no listening network socket, per the
ruling of 2026-07-31 carried on the apex correction list, and no other crate does
either. A localhost Unix socket carrying an operator's verb is not network ingress and
breaches nothing, on two grounds stated here rather than cited. A Unix socket binds no
port and is reachable only by a process already on the host, so it adds no ingress for
the invariant to be about. And a network-attached surface on the admin side would be the
first thing in the architecture arguing against apex section 12, since the regulation
model rests on the kernel bounding reach and a listener admits a principal the kernel
did not place. Section 8 states this where the surface that tests it is named.

**The floor's vocabulary, beyond what it demands.** This crate links `weaver-types`
and does not link `weaver-traits`. It reads a configuration file whose permission mode
and tool set are `weaver-types` fields electing from `weaver-traits` definitions, and
it validates the file's shape rather than the existence of a tool implementation, which
is harness-internal and unreachable from here. A link added to validate something
this crate cannot see would be a dependency taken for nothing.

**The non-link is a declared surface and not a build exclusion.** `weaver-types`
floor-links `weaver-traits`, so those definitions are in this crate's dependency tree
whatever this charter says. What the declaration buys is a checkable statement that
nothing draws them on this crate's behalf, and claiming it as an exclusion would be
claiming a property the manifest does not have.

**A charter carries no vocabulary clause, so this crate draws nothing by its own
words.** The clause `weaver-types-PRD` section 5 asks for is a contract instrument,
present in a contract even when its answer is empty, and the union the G4 check
reads is the union over the contracts a crate is party to. This crate is party to
one, and `weaver-admin-harness-contract` section 8 draws five `weaver-types` values
and nothing from `weaver-traits`. That union is the whole of what this crate draws.

```graph
edge: floor-link
from: weaver-admin
to: weaver-types
```

That contributes one fact and settles nothing. `tool-set` reaches this crate as a field
of the configuration file it reads, which is an artifact this crate validates rather
than a value drawn across a seam, and the distinction matters because a field read out
of a file and a definition drawn by a contract are answerable to different checks.
`tool-trait` is reached neither way. It is held blocked rather than open, because
tool-call protocol depends on a workflow that depends on organs not yet built, and it
cannot fail before phase close by G4's own terms. This charter does not settle it and
does not open it.

## 4. The lifecycle

Three verbs, and two acts that are not verbs. The invariant that orders them is stated
first because every subsection depends on it.

**No verb of this crate writes the boundary.** Admitting and removing a principal are
operator acts on the operating system, performed before an agent exists and after it
stops existing, and they sit outside this crate's verb set entirely. What the operator
leaves behind is a resting state this charter calls provisioned and unloaded, and the
three verbs move an agent within and out of that state rather than into or out of
existence.

**This is a different verb set from the apex's and not only a smaller one.** Apex
section 6 carries four. Two leave, `create` and `destroy`, as operator acts on the
operating system. One arrives, `validate`, which the apex does not carry at all. The
state the departing two bracket survives, so the diagram loses two arrows rather than
a state. Both edits are owed in the same act, per section 11.

**Validation belongs to the crate that takes the verb's first action, and that crate
differs by verb.** On a load the first action is admin's, because admin is handed the
configuration file and nothing moves until admin confirms that file will work, so the
inventory of 4.3 sits in this crate. On an unload the first action that matters is the
comparison of the working structure against the file, and that belongs to whichever
crate holds both of those things, which is not this one. The rule is locality rather
than seniority, and it is why one verb's validation is this crate's own work and the
other's is only directed by it.

**Opening the session record is not a boundary write.** `weaver-trace-PRD` section
4.1 has `run0` creating the record, and this charter keeps that. A record belongs to a
session and a boundary belongs to an agent, so a verb that creates one is not
creating the other, and the invariant above is scoped to the boundary deliberately.

### 4.1 load

Verifies the boundary, opens the record, starts the interior, and publishes. The
order is the substance.

1. **Authorize the intent.** Refuse without touching anything else.
2. **Read and validate the agent's configuration file.** A file that is absent, that
   is missing a required field, or that names a model binding admin cannot satisfy
   fails the load before any process exists.
3. **Verify the boundary the operator wrote.** The OS identity resolves, the home
   directory exists with the expected ownership and modes, and the trace directory is
   admin-owned and not searchable by the agent uid. Any failure refuses. Nothing here
   is repaired, and nothing here is built.
4. **Resolve the session and open the sink.** Admin decides which session is being
   loaded, a decision the harness is structurally unable to make because it never
   learns a path, and opens the sink the configuration declares. The descriptors are
   obtained here, under admin's own principal, which is what lets the worker write a
   stream its uid could not open. Close-on-exec is not admin's to confer on a passed
   descriptor and is the harness's obligation at the receive, per the contract.
5. **Ask the init system to start the worker as a transient unit carrying the agent's
   `User=`.** Admin does not put the worker under the agent uid itself. It asks a
   process that already holds that authority, per section 7, and the unit's cgroup
   arrives with the unit rather than being shaped in advance.
6. **Direct enter, and receive the aggregate.** The directive carries the session
   identity, the run ordinal, the descriptors, the model binding, and the gate
   instruction, per the contract. The descriptors ride inside the directive over the
   coordination channel as `SCM_RIGHTS` ancillary payload, the trace descriptors,
   per `weaver-harness-PRD` section 5, so the worker receives handles and never
   paths and accepts them close-on-exec at its one receive site. What section 10
   holds open is how the channel's own end reaches a unit admin did not fork, and
   not how the descriptors cross a channel that exists. Everything after the
   directive and before the answer is the harness's: it stands up an empty working
   structure, authors its `load` event, which is the record of admin's contact and
   the origin of the run's monotonic clock, asks the SPU to admit the model, and
   starts Gate last so
   no work arrives before the interior can serve it. Admin holds no channel to either
   organ, per section 6, so what admin receives is one answer aggregating the fan-out,
   ready or a refusal naming where it stopped.
7. **Publish loaded and idle.** Only now, and only on a ready aggregate. A partial
   load is never published as loaded, and the published state is idle rather than
   active.

Step 6 is the one ask in the sequence, and a refusal at any point inside it enters
the rollback of section 5 carrying the name of the step that refused. A device
conflict discovered at model admission is such a refusal, named by the SPU inside
the aggregate, and admin holds no earlier check to catch it, per ruling C.

**The worker never holds a principal above the agent's, which removes a window rather
than narrowing one.** Under the delegation of section 7 the init system starts the
process already as `weaver-<n>`, so there is no interval in which worker code runs as
anything else and no drop for the ordering of an earlier draft to get right. The
privilege window that section 10 once carried as an open cell has no subject.

**Steps 1 through 5 produce no trace entry, and neither does the rollback path.** That
is a ruling and not an omission. `weaver-trace-PRD` section 3.1 makes `run0`'s `load`
the record of admin's first contact and places the worker start and the descriptor
handoff outside the trace by construction, because they run before the harness exists
to author anything. The load's one trace entry is written at step 6 by the harness,
and the unload's at its own bracket. Nothing admin does before that moment reaches the
record, and nothing should, because a second party writing into the stream ends the
single-writer property that makes the account evidence at all. The consequence is
stated in section 2 rather than left for a reader to rediscover here: the
supervisory half of the lifecycle is unrecorded by the trace, and its record is
admin's own.

### 4.2 unload

The reverse in effect and not in shape, because what the load built across seven steps
is unwound in three, and one of admin's acts here, the publish to a state that is not
absent, has no counterpart on the way up.

1. **Direct leave, and receive the aggregate.** Everything between the directive and
   the answer is the harness's, in its own order: it stops Gate first, so a Gate
   process never outlives the interior it protects and nothing new arrives, and it
   refuses while a turn is in flight rather than racing one, because a turn
   interrupted mid-decode leaves the SPU holding a session. At rest it authors its
   `unload` event, closing the bracket, drains the writer's queue to the stream,
   then releases the SPU, so residency ends and the device is freed. The answer
   carries left, or a refusal naming where the sequence stopped. Admin holds no
   channel to Gate or the SPU, per section 6, so this directive is the whole of
   admin's part in their unwinding.
2. **Stop the worker unit.** The process exits, descriptors close with it, and the
   unit's cgroup goes with the unit.
3. **Publish provisioned and unloaded.** Which is a different state from absent, and
   absent is reached by an operator act rather than by a verb.

**What leave promises is the drain, and nothing more since 2026-08-01.** A left
answer means everything admitted reached the stream, per the contract's ordering,
and everything after the sink is the operator's, per `weaver-admin-operator-contract`
section 3. Nothing is compared, certified, or adjudicated at unload, the leave-time
validation having dissolved with the program-owned record it compared against, and
the stream ends where the run did, finalized by nothing.

### 4.3 validate

Confirms that an agent's configuration and boundary will work before anything is
started against them. It is the front half of `load` and it is also invocable alone,
which is one code path entered two ways rather than two checks free to drift apart.

What it inventories is what a load depends on and cannot discover later. The
configuration file is present and parses. Its required fields are there. The model it
names and the settings it carries for that model resolve to real things in the format
those things are expected to be in. The stream sink it declares exists, or the flag
that orders one created is set. The boundary checks of load step 3 run against the
operator's provisioning. Every one of these is a question about whether a named
thing exists and is what it claims to be, and admin answers it by looking rather
than by asking another crate, which is why this verb reaches no seam and starts no
process.

Reached as the front half of a load, a clean inventory flows into step 4 and a failure
refuses before any process exists. Invoked alone, it stops at the result and reports,
which is the cheap way to test an operator's provisioning before a load depends on it.
It changes no agent state either way.

**It repairs nothing and concludes nothing.** A check that fails emits what was expected
against what was found, per section 2, and the remedy is the operator's over artifacts
this charter does not govern.

### 4.4 Session close, and removal

`weaver-trace-PRD` section 3.1 carries `session.closed`, meaning the session will not
be resumed. No verb performs it, because a session spans runs and an agent outlives
a session. The harness authors `session.closed` while it is alive, because it is
content and the harness is the sole writer of content. The cost is that closing a
session requires the agent loaded for the authoring, which is load, stop, unload:
the stop exchange ceases token processing and holds everything loaded at rest, and
no close crosses the seam as a verb or a conveyance of its own. What cues the
authoring of `session.closed` inside that window is the session-close cell of
section 10, which this wording does not close. The checksum and the manifest an
earlier version ordered after the drain dissolved with the program-owned record on
2026-08-01, finalization being the operator's over the operator's storage.

**Removing an agent removes no record, because the program holds none.** The
stream's accumulations live with the operator and outlive the process, the session,
and the agent by that operator's own election. Deprovisioning still happens and it
is the operator's, over files and accounts, and this charter states only that no
verb of this crate reaches the operator's storage.

## 5. What a failure partway through leaves behind

Stated by where it failed, because rollback obligations differ and a charter that
says a partial load is never published has not yet said what is left to clear.

**A crate that writes no boundary has no partial boundary to reap**, so rollback is
smaller than it was. What a failed load can leave is a worker process, a connected
sink, and a device the SPU took. Nothing that survives a crash needs a later verb to
recognize and refuse it, because nothing durable of the program's was authored.

A `load` that fails at steps 1 through 4 leaves nothing to reap. No process exists,
and a sink opened at step 4 is closed, nothing having been written through it.

A `load` that fails at step 5, or at step 6 before the `load` event is authored,
leaves a worker that never entered a run. Admin stops the unit, and nothing entered
the stream.

**A device conflict refused at model admission is a refusal inside step 6, and it
arrives after the `load` event in the fan-out's own order,** stand up, author,
admit, gate. So what it leaves is an authored bracket with no `unload` on the
stream, plus a device the SPU's refusal left free, per the residency contract. It
needs no clause of its own, per ruling C, because it is a refusal inside the fan-out
like any other, which is what retiring the pre-flight arbitration was for.

**A `load` that fails inside step 6 after the harness has authored its `load` event
leaves a stream whose last run shows a `load` and no `unload`.** That is a truthful
account of a death rather than corruption the program must repair, resume having
dissolved with the program-owned record on 2026-08-01, and what a consumer makes of
a broken bracket is that consumer's reading over the operator's storage. The
run-bracket fix an earlier version of this section carried, and the cell section 10
held for it, dissolved the same day for want of a validator to enforce them.

**Rollback is admin's own reap plus one directive, because admin built with its own
acts plus one directive.** A refused fan-out is the harness's to unwind along the
same seams it fanned out on, and what returns to admin is the refusal naming where it
stopped. Admin's remaining obligations are its own: direct leave where a run was
entered, and stop the unit. Nothing durable of the program's exists to remove. Each
of those can itself fail, and a rollback that cannot complete reports what it could
not undo and does not publish any state, which is the same rule as a partial load
and not a second one.

## 6. The seam

This crate holds one seam. It is a duplex channel to `weaver-harness`, governed by
`weaver-admin-harness-contract`, and it is the only pathway admin constructs or holds
an end of.

| Seam | Peer | What crosses |
|---|---|---|
| Coordination | `weaver-harness` | Admin directs the run to be entered and left, hands the trace descriptors inside the enter directive, and conveys the operator's intent to stop. The harness confirms, refuses, or answers a stop with the turn's fate. A fault the worker survives crosses nowhere here, travelling as the `fault` event on the stream. |

```graph
edge: seam
from: weaver-admin
to: weaver-harness
via: weaver-admin-harness-contract
tag: socket
```

**Admin reaches no other crate, and the harness carries what admin cannot.** An earlier
draft drew three seams here, adding a channel to `weaver-spu` for model admission and
one to `weaver-gate` for the membrane. Both are struck. Admin owns one constructed
pathway, and when its intent has to reach the SPU or the gate the harness carries it,
because the harness is the coordinator every organ is situated against and admin is not
a peer of the organs the harness sequences. This is apex section 6 read straight: all
coordination passes through the harness, so admin directing a load is admin opening an
exchange on its one channel and the harness fanning that out along its own pathways.
Admin makes no admission decision at all, per ruling C: the device is the SPU's to
judge, and what admin would have asked the SPU it learns from the aggregate instead.

**The seam is duplex at the channel, because admin is an organ.** An organ governs a
domain and holds a duplex channel with the harness, both properties and neither
alone, and admin governs the lifecycle domain. Either party may open an exchange by
the channel's mechanics, and the chartered census is admin's three, two that drive
a transition and one that conveys the operator's intent to stop. The harness opens
none since the fault-carrier ruling of 2026-08-01: a fault the worker survives
travels as the `fault` event on the stream, the operator's tooling keys on it
there, and the duplex property is the channel's rather than the census's, per the
contract's section 0. A worker's death is observed through process exit rather
than reported over the channel, and readiness is a confirmation to a directive
rather than a directive.

**The seam edge is declared by the organ rather than by the harness.** It was declared
by the party that asks, and both parties ask now. The harness is the hub every organ is
duplex with, and a hub that declared its own edges would carry the whole seam graph in
one crate, so the organ declares its seam and the rule generalizes to every organ the
harness gains. Here the organ is `weaver-admin` and the record above is admin's to
state.

**The channel has no name in the filesystem, and that is a requirement rather than an
implementation choice.** A named socket the worker dials is dialable by anything running
as the agent uid, and `bash` is exactly that. A channel with no name cannot be opened by
a second party, so possession of the descriptor is what authenticates the peer and the
channel collapses to one for the whole of the worker's life.

**How an unnamed pair reaches a process this crate did not fork is unsettled, and the
requirement is stated at the level that survives the answer.** An earlier draft said
inherited across the spawn, which was true while admin forked the worker and is not true
under the delegation of section 7. What the seam requires is namelessness and a single
peer, and the two routes to it are named as a cell in section 10. Both preserve the
property this paragraph argues for, so the requirement is written as the property rather
than as the mechanism that used to deliver it.

**Which is why this seam authenticates by possession rather than by credential, and the
difference is stated rather than left to be noticed.** Apex section 5.1 has every seam
crossing a process boundary as a `SO_PEERCRED` Unix socket, and on an inherited pair
`SO_PEERCRED` reports the creating process for both ends, so it can distinguish nothing.
The tag stays `socket` because the seam crosses a process line, which means the tag no
longer implies the credential mechanism and a reader who takes one for the other reads
this seam wrong. Section 11 files the correction, as a restatement of the invariant
rather than as an exception admitted to it.

**The channel descriptor carries close-on-exec, and the trace descriptors it delivers
acquire it at the receive rather than in transit.** Admin sets the flag on its own end
before the channel is handed across, and the worker sets it again after its last exec.
The second is a set and not a check. `execve` can clear the flag, so a step that finds
it clear and reports rather than repairs leaves the channel to the supervisor
inheritable by every tool subprocess, which is the failure this ordering exists to
prevent. A passed descriptor is a different case: close-on-exec is a property of the
descriptor rather than of the open file description, so it does not cross with the
descriptor and the receiving party is the only one that can supply it. Admin cannot
confer it and does not claim to, and the obligation sits in the contract where the party
that can meet it is bound.

## 7. Identity, process boundaries, and privilege posture

**`weaver-admin` is a service account, and the operator is not it.** The name covers
two things: a system account with no login and no human occupant, which holds the
lifecycle grants, and a role an ordinary unprivileged user holds through membership
in the `weaver-admin` group. They share a name because the operating system's own
convention shares it, and prose here says the `weaver-admin` service or the
`weaver-admin` role wherever one sentence could take either reading. The operator
never wears the service identity. He relates to it twice: he reads finished records
as himself through group membership, and he drives the three verbs by asking the
service over the socket of section 8, which authorizes him by peer credential and
group membership.

**This shape is inherited rather than invented, which is the argument for it.** A
service account holding the capabilities and an unprivileged human asking it over a
socket is the ordinary arrangement on a Linux system, and this crate exists to
inherit the operating system's trust model rather than to raise a second one above
it. An earlier reading had one identity do both jobs, holding the lifecycle grants
and also serving the operator's reading and analysis, and the permission arrangement
that reading needed was elaborate in proportion to how far it sat from the ordinary
shape.

**What the split buys is that the capability-holding identity parses almost nothing.**
Reading and analysis of finished records is the code that consumes attacker-influenced
data, because a trace carries whatever the agent handled, and that code now runs as
the operator holding no capability at all. A parse bug there yields the operator's own
uid and has nothing to escalate into. That is a stronger position than partitioning
capabilities inside one privileged identity, because it removes the surface rather
than bounding what the surface can reach once it is taken.

**There is one act left to authorize, and admin holds no capability of its own to
perform it.** An earlier reading of this paragraph argued two authorities that needed
different grants, provisioning against supervision, and weighed capability sets,
sudoers entries, file capabilities, and unit-supplied capabilities as ways to separate
them. Provisioning has left the program, so there is nothing to separate. What remains
is starting and stopping a worker under an existing agent identity, and this crate does
that by asking the init system for a transient unit carrying `User=weaver-<n>` rather
than by exercising any privilege itself. Admin chowns nothing, creates no account, and
carries no capability on its binary.

**Holding no capability is not the same as holding no authority, and the difference is
worth stating.** Launching a unit under another user's identity is a privileged
operation, and an unprivileged process does not get it merely because the target uid
exists. The authority is delegated, either by admin running as a system service the
operator installs with the narrow right to manage its worker units or by a policy rule
scoping exactly that verb to admin's identity. So the accurate claim is that the
authority is bounded, lives in operator configuration rather than in the artifact, and
is enforced by the init system rather than by this crate. The argless requirement below
is what keeps it from widening.

**What a compromise of admin reaches, stated because the delegation invites the
question.** The bounded form of the grant is a fixed unit template and an agent name
validated against an allow-list, so a compromised supervisor reaches the set of agent
identities the operator delegated and nothing above them. It does not reach account
creation, because no such authority is held anywhere in the program. The residual parse
surface is named at the end of this section.

**The grant is argless, and that survives the mechanism being chosen.** A grant
expressed with a wildcard in argument position is not bounded by the path it appears to
name, because a wildcard matches a separator and a rule permitting work under a trace
root also permits the traversal out of it. So the delegated act validates the agent
name and the root against an allow-list and constructs the path itself, and the
authorization names a unit template and no free argument. This was written to hold
under every candidate and it holds under the one taken, which is the check that it was
stated at the right level. It is stated here rather than left to whoever writes the
policy file, because a policy file is where this is gotten wrong and the charter is
where it is caught. Naming a
mechanism as bounding the tool surface is not the same as that mechanism bounding it,
and section 11 files the apex correction that follows.

**Admin's own process boundaries, stated because they are a fact about this crate.**
Apex section 5.1's scoped invariant permits a crate calling a crate inside one binary
and forbids the same reach across a process line, which is a test that reads a process
topology no document states. The general sentence naming which crates compile into one
binary is the apex's to write.
What is this charter's is its own half: admin's code compiles into admin's processes
and into no other, the worker holds the harness and the trace and holds nothing of
admin's, and a boundary between two admin processes is a process line like any other.

**Posture through a load, which is simpler than it was.** Admin runs as itself
throughout and never as an agent. The worker holds the agent uid from its first
instruction, because the init system starts it there, so no ordering of a drop against
a handoff has to be gotten right and no window exists for one to be gotten wrong. The
trace file is owned by `weaver-admin` and grouped to the group the operator shares with
it, the agent uid holds no bit on it and reaches it only through a passed descriptor,
and the trace directory is admin-owned and not searchable by the agent uid.

**Same-uid reach is a live hole and the flag that closes it is a requirement stated
elsewhere.** If an external tool process runs as the agent uid, it can attach to the
worker and drive the coordination channel and the trace descriptors directly, which
defeats descriptor scoping. The worker's dumpable flag is what closes it, and
`weaver-admin-harness-contract` section 2 is authoritative for both the requirement
and its ordering, because the flag is a property admin relies on and cannot verify
from outside the process. This paragraph names the hole and points at the obligation
rather than stating it twice.

**The one parse surface inside the delegated identity.** Admin reads the coordination
channel and the peer at the far end is a worker running agent code, so a small fixed
message vocabulary is parsed by the party holding the delegation. This is what survived
the collapse of the two-authorities argument, and it is narrow rather than gone. It is
bounded by the vocabulary being fixed and closed at the contract, and by the delegation
reaching only agent identities.

**Whether external tool processes run as the agent uid is `weaver-gate`'s to rule,
and this charter names it as an assumption.** If they do, the boundary between the
agent and its own worker is hardening rather than kernel-enforced separation. If they
run under a uid Gate owns, the worker's descriptors are unreachable from tool code by
construction. Admin states no requirement beyond assuming the first case is no worse
than hardening, and the cell is filed against the gate charter so it is inherited as
a constraint rather than rediscovered.

## 8. The operator surface, and the domain that left

**The operator surface is a localhost Unix socket reached by a member of the
`weaver-admin` group.** It reports state, lists agents, drives the three verbs, and
conveys the operator's intent to stop across the contract's stop exchange. It carries
no work, per section 3. It is not network ingress and breaches nothing Gate holds, on
the grounds section 3 states. It is the whole of how the role reaches the service,
and it is a seam by the Working Process test, governed by
`weaver-admin-operator-contract` as of 2026-08-01, which also carries the format of
what crosses and the output stream the durable-record ruling of that date defines.

**Reading and analysis of finished records is not this crate's, and not this
repository's.** An earlier reading named `weaver-admin-tools` a member of this domain,
chartered by name and left unbuilt. The identity structure of section 7 retires that
reading rather than deferring it further. Reading runs as the operator, holds no
grant, shares no process and no identity with the lifecycle service, and reaches the
record through the published trace format and nothing besides. Identity, privilege,
repository, and the wire contract that is the only coupling all fall on the same side
of the same line, and a member of this domain is a thing this charter bounds, which
describes none of it.

**What leaves with it.** Analysis, export, the operator views a graph or a fleet menu
would be built on, and any indexed query over history. Each is a consumer of the
operator-held record on the terms `weaver-trace` publishes, which is what
contract-coupled means here, and none is staged work in section 9, because staged
work is work this crate will later do. Multi-agent management through the operator
surface stays, since it drives verbs rather than reading records.

**Admin reads nothing of the record at all, as of 2026-08-01.** The manifest this
paragraph once held open dissolved with the program-owned record, per the ruling at
`weaver-admin-operator-contract` section 3, so no lifecycle act parses events and
the reader edge section 10 once held has no subject.

## 9. Staged requirements

Recognized work with an entry condition that holds it out of this pass. This section
is authoritative for staged work belonging to this crate, per the rule
`weaver-trace-PRD` section 9 sets.

**An admin-side database is not among them, and its absence is a ruling rather than an
oversight.** Staged work is work this crate will later do, and section 8 has said this
crate will not: indexed query over history left with the reading and analysis tooling.
It is therefore not staged here and not staged anywhere in this program, and a reader
who expects a third item is reading a count this section used to carry.

**The binary layout is settled rather than staged, and it leaves this section.** It was
staged as downstream of the grant mechanism, and the mechanism ruling of section 7
removed the thing a second executable would have separated. One authority remains and
it is delegated, so a split buys nothing and the layout is one binary. A staged item
whose entry condition resolves to no work is closed rather than carried.

Nothing is staged here at present. The GID-mask item this section carried was
written against admin owning the record's file, and it left with the record on
2026-08-01, the write path now ending at a sink the operator declares.

## 10. Open rulings

Each names what settles it. A cell with a proposed reading and a named test is a
handoff rather than a hole. Four cells closed in this pass and are recorded as closed
where the closure is recent enough that a reader would otherwise look for them.

**The grant mechanism is closed.** Delegation to the init system, by a transient unit
carrying the agent's `User=`. It was the fourth of four candidates and the ruling that
moved provisioning out of the program left it the only one with a subject, since the
choice was between ways of separating two authorities and one of the two is gone.
Section 7 carries the consequence.

**The layout is closed with it.** One binary. The choice decided the layout rather than
the other way round, and a delegation that holds no capability leaves a second
executable separating nothing.

**The cgroup is closed, and by neither of the two candidates.** It was posed as a
provisioning artifact that load populates against a residency artifact created at load
and torn down at unload. Under delegation the init system creates a cgroup for the
transient unit and removes it when the unit stops, so the second shape holds and this
program does not shape it. The cell is closed by the mechanism rather than by a ruling
on the cell.

**Drop-first is closed and its subject is gone.** The worker starts as the agent uid,
so there is no privilege window to order a handoff against.

**How the trace descriptors reach a process admin did not fork.** This is the cell the
mechanism ruling opened, and it is the one place section 4.1 is unsettled. Section 6
requires the coordination channel to be an unnamed connected pair, on the grounds that
a named socket is dialable by anything running as the agent uid, and under delegation
admin is not the worker's parent, so inheritance across a fork is not available. The
route this cell holds open is admin passing a descriptor into the transient unit
through the init system's own facility for it, which keeps the channel unnamed and
moves one more thing into operator configuration. Admin forking the worker itself is
not the peer alternative an earlier form of this cell offered, because an init system
supplies no identity to a process it did not start, so admin setting the uid takes a
capability section 7 rules admin does not hold. What an absent interface reopens is
the channel design of `weaver-admin-harness-contract` section 2 and not the grant
mechanism, in that contract's own words. **Settled by:** a check of what the init
system's transient-unit interface will carry, which is a mechanical question with a
factual answer, taken before the Spec. Named here because the ruling that closed the
mechanism opened this, and a charter that recorded only the closure would be
recording the half that reads well.

**Session close.** Section 4.4 puts `session.closed` with the harness, at the cost
of requiring the agent loaded for the authoring. What cues that authoring inside the
load, stop, unload window is the open half. **Settled by:** the human's ruling. The
drain-and-checksum half an earlier form of this cell carried dissolved with the
record on 2026-08-01.

**What enter becomes without a record.** Enter stands up an empty working structure,
per the cut of 2026-08-01, so a later run of a session begins with the session's
identity and none of its conversation, and sessions are single-run at the program's
promise level. Two faces of one question stand open. Whether continuity returns
through operator-held storage handed back at load, through the memory round's own
substrate, or not at all is a design the corpus defers on purpose. And the run
ordinal is admin's to supply with nothing program-side holding the last one across
an admin restart, so what makes the ordinal trustworthy over a fleet of restarts
belongs to the same design. **Settled by:** the memory-and-state round, and
deliberately not by the cut batch that left it, because the batch deletes what
dissolved and a resume redesigned in a deletion batch would be a load-bearing
decision taken in passing.

**The operator-to-service seam is closed.** Section 8's socket is governed by
`weaver-admin-operator-contract`, written on the human's ruling of 2026-08-01 that the
external boundaries are contracted before any Spec. Section 6 still does not declare
it, because the near party is a human role rather than a crate, and the party
category the Document Format lacks is owed to the Format by that contract's own
register, one entry covering this seam and the gate's client boundary both.

**Whether there is a configuration-file contract at all.** `weaver-types-PRD` section 6
defers one to this pass, and the ruling that moved authorship to the operator changed
what the question is. Both crates that touch the file now read it, so there is no
producer inside the program and no producer-consumer agreement between two crates to
write. Working Process section 4 rules that only seams take contracts, and this is an
artifact neither party authors. So `weaver-types` is the named authority on the format
under G5, both charters carry their own validation obligations, and no third document
exists. **Settled by:** the human's ruling. Either way the Document Format's sentence
citing that contract is an owed edit, per section 11.

## 11. Edits owed in the same act

Apex section 10 requires that a change touching a contract merges with every party in
one act, and gate G7 asks that a ruling name the documents it changes. This
section is that register. Nothing below is applied by this document, and the items
already carried in `open-items` are cited rather than restated.

**An entry leaves this register when the edit lands.** The register is the G7 instrument
read from the other end, and it has a symmetric failure: a ruling recorded and not
landed reads as settled to every later reader, and an entry landed and not cleared
reads as outstanding. A reader who checks two entries against the corpus, finds them
already done, and stops trusting the rest has lost the whole register for the cost of
two stale lines. Entries for the wire vocabulary, the section 2.2 repair, the
close-on-exec mechanism, and the drop-first privilege window left this way.

**Four more left on 2026-07-31.** The apex 5.1 restatement and the 5.4 definition
landed together, taken early as a named exception to Working Process section 7 and
recorded as such in `WeaverTools-PRD` section 5. The five wire definitions landed in
`weaver-types-PRD` section 2.3 rather than section 4, which is where this register
said they would go, because 2.3 is where that charter keeps definitions and 4 is where
it keeps the departure argument. Both were edited. The harness clause on opening
exchanges landed in `weaver-harness-PRD` section 4. Nothing else in this register
moved, and the apex re-authoring still waits on all seven charters.

- `WeaverTools-PRD` section 6: the verb set goes from four to three by two departures
  and one arrival. `create` and `destroy` leave as operator acts, the
  provisioned-and-unloaded state they bracket survives, and `validate` arrives, which
  the apex does not carry today. Validate changes no agent state, so the apex's set is
  no longer four state transitions and the sentence scoping it that way is restated as
  the operations admin performs on an agent. The locality rule that decides which crate
  owns a given validation is charter-level and is not owed to the apex. Per section 4,
  and files with the apex re-authoring.
- `WeaverTools-PRD` section 12 and section 3: the program verifies the agent's
  operating-system boundary and does not author it, so the clauses that seat boundary
  authorship inside this program are restated as verification against an
  operator-supplied boundary. Files with the apex re-authoring.
- `WeaverTools-PRD`, the trust model, stated once: this program secures the record
  against the agent and the operator is trusted by construction. Section 2's limit
  paragraph, the between-load interval of section 2, and the emit-only result of the
  same section all point at it, and stating it in three charters instead is the
  duplication G5 exists to catch. Files with the apex re-authoring.
- `weaver-types-PRD` section 2.1: the artifact is renamed from `agent-state-file` to a
  configuration name, its seven `holds` edges follow the node, and the producer becomes
  the operator rather than `weaver-admin`. The graph record in section 2 of this
  charter keeps the old identifier until this lands, because a node renamed on one side
  only is a dangling edge, and the two move in one act.
- `weaver-harness-PRD`: the trace file is owned by `weaver-admin` and not by the agent
  uid, per section 2, and any clause resting on the agent owning the record is restated.
  Owed with the custody ruling.
- `WeaverTools-PRD` section 5.2: scoped to requests that belong to an existing turn,
  the approved wording per the ruling of 2026-07-31. As stated, every
  request crossing a seam carries the trace context identifying the turn it belongs
  to. A lifecycle directive on the coordination seam crosses a seam and belongs to no
  turn, so the seam this pass chartered is a counterexample to the invariant as
  written, and it is the second such counterexample in two rounds after 5.1.
  `weaver-harness-PRD` section 4 states the scoping and names it as owed, and this
  entry is what it is owed to. A charter naming an apex edit that no register holds is
  the G7 failure the register exists to catch. Files with the apex re-authoring.
- `WeaverTools-PRD` component ownership: the reading and analysis of finished records
  leaves this repository, per section 8, so the apex's component list drops it rather
  than carrying it as a member yet to be built. Files with the apex re-authoring.
- `weaver-types-PRD` section 2.2: the peer-credential carve-out gains its motivating
  case. The operator-to-service socket of section 8 is the seam that authenticates by
  credential, against the coordination pair that authenticates by possession, and 2.2
  currently names the second without the first.
- `WeaverTools-PRD` section 3 step 7: naming a policy mechanism as bounding the tool
  surface does not bound it. A grant carrying a wildcard in argument position is
  unbounded by the path it appears to name, so the apex clause takes the argless
  qualification section 7 states. Files with the apex re-authoring.
- `WeaverTools-PRD` process topology: the general sentence naming which crates compile
  into one binary, assigned to the apex by section 7. Admin states its own half there
  and the rest has no owner, so apex section 5.1's scoped invariant is a test that
  reads a topology no document states. Files with the apex re-authoring.
- `WeaverTools-Document-Format.md` section 3: the clause citing the deferred
  state-file contract, per the ruling in section 10.
- `open-items.md` section 4: the binary-layout item is closed rather than moved, per
  section 9, and the GID-mask item moves into section 9 restated. The admin-side
  database entry leaves the list rather than moving, because section 8 removed its
  owner and no other crate takes it.
- `open-items.md` section 1: the custody-limit item is answered by section 2's limit
  paragraph and by the apex trust-model entry above, and its two-branch analysis is
  moot, since the mechanism it branched on is closed and the principal it names is the
  operator.

## 12. Children

Specs to be written against this charter once the PRD set is ratified. Named so the
set is bounded, not drafted here.

- Boundary verification, covering identity, home, and the trace directory.
- Lifecycle sequencing and the rollback of section 5.
- The coordination channel, covering the transient unit and descriptor handoff.
- The operator surface of section 8.

Contracts this crate is party to are written with the PRDs of their other parties,
one per seam in section 6, and are not children of this document.
