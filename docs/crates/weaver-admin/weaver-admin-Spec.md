# weaver-admin - Spec

**Status:** MERGED. Cut 2026-08-02, fifth of the Spec pass and the first outside the
agent. Code is written against it under the gates of Working Process section 6.

**Date filed:** 2026-08-02
**Revised:** 2026-08-05, the role ruling. Per the operator: `weaver-admin-role` is
assumed by a human and never by an AI or an automation, a statement of design intent
and not a guarantee about conduct. `weaver-admin-user` is a static service account
rather than a login account and is where the delegation attaches, and the crate is the
peer organ that account runs, whose narrow domain includes custody of where the record
leaves the system. One unit per agent is named as the sandbox pattern rather than a
fleet, and the agent's uid is statically provisioned, a dynamic identity excluded for
two independent reasons. The sandbox's properties are required and its directives stay
the operator's. Section 10's descriptor-route cell reopens with the sudo measurement
attached.
**Revised:** 2026-08-05, second this date, the admin recut and the socket inversion,
one act of three rulings. The crate becomes one invocation per verb run with root:
the operator socket, its accept-time predicate, and the fleet map retire with the
service account that held them, section 2 becoming the invocation's interface and
section 3 reading the init system for state. The coordination channel inverts, so
section 7 dials where it bound, the four acts becoming one connect with a bounded
retry, and the credential check moves to the harness's accept where its record now
lives. Section 6's unit declares no open, its reliance set moving to
`weaver-admin-systemd-contract` and its election's ground restated to what holds,
and a failed dial now consults the unit's state so a refusal names the right
failure.
Section 10's walks and counts are restated against the surviving set.
**Revised:** 2026-08-13, the start ask carries the worker's provisioning. Per the
ruling landed in `weaver-admin-PRD` load step 5: section 6's ask gains an argument
vector taking no second variable, and the reading of bare narrows to descriptors,
which leaves the no-descriptor assertion standing unchanged. Section 9's list of
the operator's installed values gains the two organ binary paths, on the ruling
that one installation's fact does not belong in one agent's declaration, and that
section's claim that the worker's composition root reads the file is corrected to
what happens, the root receiving the values it needs and reading nothing. Section
11 gains the two values the act declined to route.
**Revised:** 2026-08-14, the run identifies itself. Section 7 gains the clause that
mints the run reference and reads the session, and draws the line between that
reference and the exchange ordinal beside it, which stays a counter because a
connection does not outlive the invocation. One assertion is added and section
10's counts are restated to thirty-two.
**Document ID:** `weaver-admin-Spec`
**Parent:** `weaver-admin-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

Build instructions for `weaver-admin`: the binary's layout, the invocation's
interface, the verb sequencing, the sink openings, the transient unit's
invocation, the coordination channel's dial, and the elections a builder
would otherwise invent. It is derived from `weaver-admin-PRD` and from the three
contracts this crate is party to, `weaver-admin-harness-contract`,
`weaver-admin-operator-contract`, the second now bounding the trace's exit alone,
and `weaver-admin-systemd-contract`, cut this date for the boundary the recut made
load-bearing, together with `weaver-organ-channel`, the drawn material the first
of them draws in part.

Level discipline. The charter says what the crate needs and why. This document
says how it is represented, and per gate G2 it elects against grounds the charter
and the contracts state rather than developing grounds of its own. Where this
document and the charter disagree the charter yields nothing.

**This document declares its crate's assertion records and no other record,**
per Document Format sections 3 and 4 as of the notation of 2026-08-03. The
charter stays the source of this crate's node, its parent edge, its one floor
link, its one declared seam, and its artifact edges, and a Spec that restated
any of them would give the mapper two sources for one record, per that
format's section 1. What this document sources is the claims code must
conform to, declared at the clauses that argue them rather than gathered in
one place, per that format's section 6, and `asserts` runs from the crate
rather than from this document, which is why the document needs no node of
its own.

**A claim this Spec cites and another Spec argues carries no record here,**
and there are ten of them, named in section 10 with the crate that declares
each. The pattern behind them is this crate's own: admin authorizes and does
not execute, so a claim about what a run does once the enter directive lands
is argued where the run happens, and a floor definition admin consumes is
argued where it is defined. Copying either would be the duplication gate G5
makes someone adjudicate, with an authority owed against every copy.

**It is written from the merged corpus alone,** per the ruling of 2026-08-01
that keeps the old tree's Specs out of the Spec pass.

**This crate is fully chartered, so this Spec's bound is the charter's own.**
The lifecycle workflow is the whole of admin's job, and nothing here defers to
the token or tool workflows: what stays open is what the charter's section 10
holds open, the session-close cue and the enter question, each already carrying
its settler.

## 1. The crate

**One binary, per the charter's ruled layout.** The crate builds a single
executable, `weaver-admin`, and no library surface is published: nothing links
admin, per the charter's section 7, and a `lib.rs` would be an API for a
consumer the topology forbids. The instrument is review rather than the
manifest, because a manifest carrying no `[lib]` section is indistinguishable
from one whose library target Cargo would find by convention, so nothing
mechanical reads the absence.

```graph
node: admin-no-library-surface
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-no-library-surface

edge: grounds
from: admin-no-library-surface
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**Layout.** One module per obligation.

    src/main.rs       entry, argument parsing, and wiring, and nothing else
    src/surface.rs    the invocation's interface and the OS calls, section 2
    src/verbs.rs      load, unload, validate, stop, and rollback, section 3
    src/inventory.rs  config validation and boundary verification, section 4
    src/sink.rs       sink resolution and opening, section 5
    src/unit.rs       the transient unit, section 6
    src/channel.rs    the coordination channel's dial, section 7
    src/log.rs        the operations log, section 8

**Edition and toolchain.** Edition 2024 on the pinned nightly, no nightly
feature used.

**The dependency set is one internal crate and two external ones, and each is
argued.** `weaver-types` is the charter's one floor link, taken **with its
`config` feature on**, because admin is the crate that parses the operator's
file, per that Spec's section 1, and the parser's whole audience is this
module's inventory. `weaver-traits` is deliberately not a direct dependency, per
charter section 3: it arrives transitively through `weaver-types` and nothing
here draws it by name, which the manifest states by carrying no line for it.
`serde_json` renders the invocation's answers and encodes and decodes the
coordination channel's envelopes. `nix` is the OS surface, on the grounds
`weaver-harness-Spec` section 2.4 argued and this crate inherits rather than
re-argues: descriptor custody is central, the io-safe owned types make it a
compile property, and the needed calls, `socket`, `bind`, `listen`, `accept`,
`getsockopt` for the peer credential, `sendmsg` with control messages, `open`,
`mkfifo`, and `stat`, are all covered. That crate asserts the election where
it argues it, so this one cites it and adds no second record for one
decision.

```graph
node: admin-one-floor-link-types-config
kind: assertion
tag: manifest

edge: asserts
from: weaver-admin
to: admin-one-floor-link-types-config

edge: grounds
from: admin-one-floor-link-types-config
to: axiom-floor-is-vocabulary-behavior-is-socket

node: admin-no-direct-traits-line
kind: assertion
tag: manifest

edge: asserts
from: weaver-admin
to: admin-no-direct-traits-line
```

**What the compiler holds of that custody is the ownership and no more.** A
descriptor is an owned type end to end, so a leak is a move the borrow
checker sees, and that half is a type property. That every creating call sets
the close-on-exec flag atomically is a behaviour rather than a type property,
argued at section 6 and tested by the third walk of section 10, so the two
halves take separate records and neither claims the other's instrument.

```graph
node: admin-descriptors-owned-types
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-admin
to: admin-descriptors-owned-types
```

**No async runtime, no D-Bus crate, no logging crate.** The surface's traffic
is operator-paced and the coordination traffic is per-load, so nothing here
needs an executor, and threads from the standard library carry the concurrency
section 2 needs. The init system is reached by its command-line interface, per
section 6, so no bus library enters the tree. The operations log of section 8
is this crate's own file with this crate's own writer, and a logging framework
would be a second account with its own schema, which is the arrangement charter
section 2 rules out for the trace and this Spec declines for the log. The
absences are checked by the build-time `cargo tree` assertion the floor Specs
share.

```graph
node: admin-no-runtime-no-bus-no-logging
kind: assertion
tag: manifest

edge: asserts
from: weaver-admin
to: admin-no-runtime-no-bus-no-logging
```

## 2. The invocation's interface

The interface of charter section 8: the operator runs the binary with root, one
verb per run. The socket this section carried until 2026-08-05 retired with the
service account, and what replaces it is the process boundary the operating
system already draws around an executed program.

**The verb and its agent arrive as arguments.** One verb per invocation,
`load`, `unload`, `validate`, `stop`, `show`, or `list`, with the agent name as
the one further argument where the verb takes one. Arguments rather than a
parsed request line, because the kernel already delivered them as a vector and
re-encoding them into a wire format would be inventing a wire where no seam
crosses. The service configuration's root is the one environment variable read,
per section 9.

**Authorization is the kernel's, and what this crate checks is the name.** The
invocation runs as root or performs nothing, so no predicate, no allow set, and
no deny set exist here: the earlier form's `authorized` call, its group allow
set, and its agent-uid deny set retired with the socket, the operator being the
party the kernel already admitted. What survives is the allow-list check of
section 4, which is about which agent may be named rather than about who may
name it. The refusal is enacted before any verb touches anything, and the
instrument is a test running the binary as a non-root uid and finding it
refuses, watched to fail when the check is removed.

```graph
node: admin-runs-as-root-or-performs-nothing
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-runs-as-root-or-performs-nothing

edge: grounds
from: admin-runs-as-root-or-performs-nothing
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**The answer is one JSON object on standard output and the exit status agrees
with it.** One `lifecycle-answer` or one `lifecycle-refusal` in the floor's
internally tagged rendering, the discriminant being the tag itself because the
case sets are disjoint at the floor. Zero exits an answer and a non-zero status
exits a refusal, so a shell reads the status and a tool reads the object and the
two never disagree. No organ envelope appears here, because this is not an organ
channel and no contract draws one across it. What the earlier form asserted of
the wire, one answer per request in request order, is structural now: one
invocation carries one verb and emits one object.

```graph
node: admin-answer-and-exit-status-agree
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-answer-and-exit-status-agree

edge: grounds
from: admin-answer-and-exit-status-agree
to: axiom-contract-is-a-complete-interface
```

**Concurrency left this crate with the surface that held it.** One invocation
runs one verb and exits, so no threads, no accept loop, and no cross-connection
synchronization remain. What kept two transitions for one agent from
overlapping was the fleet map's in-flight flag, and section 3 states where that
obligation lands now.

## 3. The verbs, the agent's state, and rollback

**Residency is read from the init system rather than held, per the recut of
2026-08-05.** A per-invocation crate has nowhere to keep a map across verbs, and
the map is not missed for what it truly knew: whether an agent's unit is
running is a question the init system answers authoritatively, through the same
command-line interface section 6 uses, where a map of admin's own would be a
second account of a fact the process manager already holds.

**What that answer is not is the agent's lifecycle state, and conflating them
would be this crate inventing a fact.** A running unit may be one that has not
yet answered enter, one serving a turn, or one unwinding after leave, and apex
section 6's states distinguish exactly those. The unit's presence is a residency
signal and nothing more. Reading it as loaded-and-idle would also contradict the
charter's own rule that the state publishes only on a ready aggregate, since a
unit is running well before any aggregate returns.

**So `show` and `list` refuse, with the residency they could read named in the
refusal's own prose rather than dressed as a state.** They return
`StateNotObservable`, the floor case `weaver-types-Spec` section 4.2 adds for
exactly this, because the answer they would otherwise carry does not exist: the
two fitting cases of `lifecycle-answer` both take an `AgentState`, and this crate
has no source for one. What admin can read is the manager's three values under
the manager's own names, `active`, `failed`, and `inactive`, the last covering
the several cases section 6's measurement records it cannot separate. That is
residency, it is carried rather than translated because a translation is where
the invention would enter, and it is not what these verbs were asked for.

**The refusal is the smallest honest answer available and it is temporary.**
An operator running `show` learns that the state is not observable from here
rather than receiving a value the program guessed, and the refusal is typed, so
tooling keys on it rather than parsing prose. It goes when the observation
exchange lands, and the record below is what a reviewer checks to find it gone.

**Those three values do not map onto `AgentState`, and that is a gap this act
names rather than closes.** The floor enumerates `Absent`, `Unloaded`, `Idle`, and
`Active`, per `weaver-types-Spec` section 6. A manager reading `active` covers both
`Idle` and `Active`, because a running unit is one that may be at rest or serving a
turn and the manager cannot tell which. A manager reading `failed` has no case at
all. So the residency answer is not an `AgentState` and this crate does not
construct one, which leaves `lifecycle-answer`'s `State` case without a producer
for these verbs until the gap is closed. **The edit is owed to `weaver-types`** and
named in the charter's section 11 register, and it is deliberately not made here: a
Spec that grew the floor's enumeration to fit what a manager happens to report
would be settling the vocabulary from the representation, which is the direction
gate G2 forbids.

**What closes it is the observation exchange, which is also what the richer state
needs.** `AgentState`'s distinction between idle and active is reachable only from
the party that holds the run, and `weaver-admin-harness-contract` section 3
charters enter, leave, and stop with no observation. One exchange answers both this
gap and that one, so section 11 files a single open election rather than two.

**The record's instrument is a test rather than review, because the refusal is
checkable.** `show` on a provisioned agent returns `StateNotObservable` and
constructs no `AgentState`, watched to fail when the verb is made to answer with
a state read from the unit, which is the invention this clause forbids. Review
could confirm the absence of a mapping and could not confirm that the verb
refuses rather than returning something plausible.

```graph
node: admin-residency-is-not-lifecycle-state
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-residency-is-not-lifecycle-state

edge: grounds
from: admin-residency-is-not-lifecycle-state
to: axiom-harness-integrates-by-the-loop
```

**Two invocations for one agent are ordered by the init system, and the
consequence is stated rather than glossed.** The in-flight flag that held one
transition per agent went with the map, and what remains is that starting a
transient unit whose name already exists fails at the init system, so two
concurrent loads of one agent cannot both start a worker. Two concurrent
unloads reach a worker that answers leave once and refuses the second by the
channel's own ordering, per the contract's section 4. Neither race is prevented
by a lock of this crate's, and the honest statement is that the ordering is
delegated to the two parties that already serialize: the process manager and
the worker.

```graph
node: admin-publishes-only-on-ready
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-publishes-only-on-ready
```

**`load` runs the charter's seven steps in order and the sequence is code
rather than convention.** Authorize the name, validate through section 4's one
inventory, verify the boundary in the same inventory, resolve the session and
open the sink per section 5, start the unit per section 6, dial the worker's
socket and direct enter per section 7, publish. Seven actions, the charter's
own, the bind-and-listen act the earlier form interleaved here having moved to
the worker with the inversion. Each step's failure returns a typed
`lifecycle-refusal` and enters the rollback below carrying the step's name.

**`validate` is the load's front half, and the pin is one function.** The
inventory of section 4 is a single function that both the verb and the load
call, so the two cannot drift, which is the charter's one-code-path-entered-
two-ways rule made structural and the call graph the compiler checks.

```graph
node: admin-inventory-one-function
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-admin
to: admin-inventory-one-function
```

**What the call graph does not hold is where the verb stops.** Invoked as a
verb the inventory run ends at the report and answers `Validated`, touching
no seam and starting no process, which is a behaviour no signature states and
no count of callers reaches. It is review's and takes its own record, because
a single record tagged for the mechanical half would claim the compiler for
the whole.

```graph
node: admin-validate-starts-no-process
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-validate-starts-no-process
```

**`unload` runs the charter's three steps, and the third waits on the second.**
Direct leave and await the aggregate, stop the unit through section 6's
interface, and answer provisioned-and-unloaded **only once the stop has been
confirmed**. A refusal on leave, `ActivityNotAtRest` above all, returns to the
operator unchanged and answers nothing further.

**A stop that is accepted is not a stop that has happened, and this verb waits
for the difference.** `weaver-admin-systemd-contract` section 4 promises that a
stop ask is answered when the unit has stopped rather than when the stop was
accepted, so the ask itself is the confirmation and this Spec elects no timeout
of its own beside it. What the verb owes is not to run ahead of that answer: a
stop ask that fails, or that returns over a unit a following state ask still
finds `active`, refuses with the failure carried and answers no state, because
an agent reported unloaded while its worker still runs is the one report this
verb must never produce. Where the stop refuses, the run has already left and
the unit stands, which the rollback of this section records as an act it could
not undo, per charter section 5.

```graph
node: admin-unload-answers-after-confirmed-stop
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-unload-answers-after-confirmed-stop
```

**`stop` is a conveyance and its answer is a relay.** The operator runs the
verb, admin dials and opens the stop exchange on the coordination channel, and
the harness's answer, `TurnAborted` or `AtRest`, returns to the operator as
received. Admin holds no opinion about which, per charter section
3. This is the crate's own rule read at a verb: authorizing a stop and
deciding what a stop found are different acts, the second is the harness's,
and a relay that translated the answer would be admin ruling on a run it does
not conduct.

**This record's edge moves to the integration invariant.** The labelling pass
placed it at `axiom-organ-and-submodule`, that being the nearest thing the apex
then held to a statement about domains, and apex section 5.4 settles what an
organ is rather than what an organ is answerable for. This claim turns on the
second question. An organ is answerable for its own domain and for nothing
outside it, per apex section 5.5, and what a stop found is a fact about a run
the harness conducts. A translated answer is the organ starting to reason about
a domain that is not its own, which is the harm that section states as its own
reason for existing.

```graph
node: admin-stop-answer-relayed-unchanged
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-stop-answer-relayed-unchanged

edge: grounds
from: admin-stop-answer-relayed-unchanged
to: axiom-harness-integrates-by-the-loop
```

**Rollback is the reap plus one directive, as data.** What a failed load can
leave is a worker unit, a connected sink, and a device the SPU took, per
charter section 5, and the rollback walks what stands: direct leave where a
run was entered, stop the unit where a unit started, close the sink where one
opened. Each act's failure is logged per section 8, the rollback reports what
it could not undo, and no state is published on any partial outcome, which is
the same rule as the partial load and not a second one.

```graph
node: admin-rollback-logs-its-account
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-rollback-logs-its-account
```

## 4. The inventory

One function, called by `validate` and by `load` step 2 and 3, refusing at
the first failure with the field or check named.

**The parse is the floor's.** `weaver_types::parse` yields a whole
`AgentConfig` or a typed error, per that Spec's section 2, and this crate
adds no partial reader. A parse error maps to `ConfigInvalid` with the field
carried. That the parse is total and exposes no partial value is
`weaver-types-Spec` section 2's claim and asserted there, so what this crate
adds is the mapping and not a second statement of the parse.

**The existence checks are admin's, and each is a look rather than an ask.**
The model binding resolves to a readable artifact. The sink exists, or its
creation flag is set, per the discriminated cases of section 5. The agent's
uid resolves and its home directory exists with the expected ownership and
modes. Any failure refuses with `BoundaryUnverified` or the artifact case
that names it, nothing is repaired, and nothing is built, per charter
section 2. No test below reaches that set, which is stated rather
than left to look like an omission: the checks are a list a reviewer reads
against the charter's boundary, and review is the instrument that holds them.

```graph
node: admin-existence-checks-repair-nothing
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-existence-checks-repair-nothing
```

**One check in that list is the second walk's mechanism and is held
mechanically instead.** The sink path's containing directory is root-owned
and not searchable by the agent uid, whatever the sink's kind: the agent uid
is not the owner, holds no group search bit through any membership, and the
other bits carry no search. Section 10's second walk derives a
perturbation-verified test from that one check, so it takes a record of its
own rather than riding the review the rest of the list takes, a single record
for the whole list having claimed the test for checks no test touches.

```graph
node: admin-boundary-denies-agent-traversal
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-boundary-denies-agent-traversal
```

**The devices the binding assigns are not checked here, and the absence is
stated rather than left to be inferred.** The parse has already answered that
the assignment is present and well-formed, per `weaver-types-Spec` section 2,
which is the whole of what this crate needs to know about it. Whether those
devices exist on the host, whether they have room, and whether they can reach
each other are questions about hardware, and admin reasons about the device at
no point, per ruling C of 2026-07-31, so they belong to the one authority on
the device and are answered at admission. The check would be easy to write
here and that is what makes stating its absence worth the sentence: an admin
that verified the GPU would be the second arbitrator ruling C removed,
reintroduced as a convenience.

**This record grounds in the integration invariant as well, and the two edges
are separate reasons.** That the device has one authority is the domain
partition `axiom-organ-and-submodule` draws, and it is what gives the question
somewhere else to belong. That admin forms no view of it anyway is apex section
5.5's bound, an organ being answerable for its own domain and for nothing
outside it, with what the device can carry reaching a load through the harness
rather than through a second check here.

```graph
node: admin-checks-no-device
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-checks-no-device

edge: grounds
from: admin-checks-no-device
to: axiom-organ-and-submodule

edge: grounds
from: admin-checks-no-device
to: axiom-harness-integrates-by-the-loop
```

**The allow-list is consulted before anything else is touched.** The agent
name is validated against the operator's allow-list and the constructed
identity is `weaver-<name>` from the validated name, never from a
caller-supplied string, which is the name-validation discipline of charter
section 7 landing at the one site that constructs. It is the one site because
the same validated name is what section 6 interpolates into the unit template,
so a name reaching a path or a unit has one origin and review reads that origin
rather than every use.

```graph
node: admin-identity-from-validated-name
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-identity-from-validated-name
```

## 5. The sink

Opened by the discriminant the config carries, under root, the role's
principal, every descriptor close-on-exec in the opening call itself.

**`File { path, create }`.** Opened write-only with `O_APPEND`, `O_CLOEXEC`,
and, when the flag is set, `O_CREAT` at mode 0640, owned by root, which is the
custody of charter section 7.
Append-only rides the open file description, verified: a duplicate of the
descriptor carries the flag, so the worker's copy appends wherever it
writes, which is what `weaver-trace-Spec` section 7 relies on from the far
side. The instrument is review, the verification being a fact about the
kernel read once rather than a property this crate's own suite can perturb.

```graph
node: admin-sink-file-append-only
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-sink-file-append-only
```

**`Pipe { path }`.** Created with `mkfifo` at mode 0640 when the creation
flag is set. Opened write-only with `O_NONBLOCK` and `O_CLOEXEC`, because a
blocking open of a reader-less FIFO hangs the load, and the nonblocking form
fails loudly instead. Verified: the open returns `ENXIO` when nothing holds
the read end, which maps to `DescriptorsUnusable` and refuses the load with
the truth, that the operator's tooling is not listening. On success the
nonblocking flag is cleared, verified clearable, so the worker's writer sees
ordinary blocking semantics.

```graph
node: admin-fifo-open-nonblocking-refuses
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-fifo-open-nonblocking-refuses
```

**`Socket { path }`.** A stream connection to the operator's listener,
close-on-exec at the socket call. There is no creation flag, per
`weaver-types-Spec` section 2: something of the operator's must already be
listening, and a connection refused refuses the load. The discriminated shape
this section opens by, and the asymmetry that leaves this case without a
flag, are that Spec's claims and asserted there.

**One open site, and the path dies at it.** The sink is resolved and opened
in this module and the resulting `OwnedFd` is what travels, so no other
module of this crate holds a sink path, and the worker never sees one at
all, per the descriptor discipline the contracts fix. What this clause
asserts is the site inside this crate. That the worker holds no path is a
pin on the worker's side of the seam and is declared by the crates that would
have held it, per section 10.

```graph
node: admin-sink-path-dies-at-open-site
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-sink-path-dies-at-open-site
```

## 6. The unit

**The init system is asked over its command-line interface, and the election
is argued.** Starting the worker is one invocation per load of the system's
own run tool, with the unit's properties declared on the invocation: the
agent's `User=`, the fixed template's hardening, and the runtime-directory
declaration the coordination socket is bound inside, per
`weaver-admin-systemd-contract` section 2. Stopping it is one invocation of the
stop verb, and the same interface answers the state query of section 3. The
alternative is a bus library, and it loses on the tree, for a handful of
invocations per lifecycle that are neither hot nor latency-bound.

**No descriptor is declared on the invocation, and the negative is the point.**
The trace's sink is opened by this crate under root and crosses inside the enter
directive, per section 5 and section 7, so nothing about the record reaches the
unit's properties. An earlier form of this clause named a standard-output
declaration placing the trace's far end, written while that route was under
consideration and left standing after it was declined. It is struck:
`weaver-admin-systemd-contract` section 0 rejects the route because the unit's
standard output is inherited across fork and exec, which would hand every organ
the harness forks a writable handle to the agent's own record. One record path,
and it is the descriptor.

**What this crate relies on from the init system is the contract's and not this
election's.** `weaver-admin-systemd-contract` section 5 states the reliance set,
the identity holding from the first instruction, the sandbox in force before it,
unit-name uniqueness as the concurrency guard, and the cgroup's arrival and
removal with the unit. This Spec elects only how those asks are carried, so a
builder replacing the command line with a bus library would change this election
and breach nothing in that contract.

**The election's ground is no new dependency, stated exactly because a looser
ground was written first.** The clause above said a bus crate brings an async
runtime, and that is not true as written: `zbus` publishes a blocking API and
`dbus-rs` is synchronous over a C library. What holds is narrower and enough.
`zbus` carries async machinery into the resolved tree whatever its surface API,
which this crate's own manifest assertion forbids, and `dbus-rs` trades that for
a C library dependency in a binary that otherwise links none. The command line
costs neither, at a handful of invocations per lifecycle that are neither hot nor
latency-bound. What it costs instead is failure discrimination, named at the
contract's section 3 and not defended here.

**A failed dial is followed by a state ask, so a refusal names the right thing.**
The contract's section 3 records the measurement: a start ask can succeed over a
unit that never runs, so the dial's bound is what proves liveness and the bound
alone would report an absent residency where the truth is a unit that is not
running. Section 7's refusal therefore consults the unit's state before
returning.

**What that ask yields is a state and never a reason, and what the state
separates is narrower than it looks.** `weaver-admin-systemd-contract` section 3
says outcomes carry status and not the failure's cause, so this clause promises no
diagnostic, and why a unit failed is the manager's journal to answer where this
program does not read, per that contract's section 7. Measured 2026-08-05 against
a live manager, the activity value separates three cases and conflates several:
a unit whose process ran and exited non-zero reads `failed`, a running one reads
`active`, and `inactive` covers a unit that stopped cleanly, one that never
existed, and one whose exec never succeeded because its binary was absent. So the
refusal carries the value and claims nothing beyond it. A dial that timed out
over a unit reading `failed` refuses naming that state, and one over a unit
reading `inactive` refuses saying the worker is not running without asserting
which of the three reasons applies, because the boundary cannot tell them apart
and a refusal that guessed would be inventing a fact.

**The instrument is a test whose watch turns on the state it names.** A unit
started against a binary that exits non-zero reaches `failed`, and the test
watches the refusal carry that state rather than the absent residency, watched to
fail when the state ask is removed. The obvious test, a binary that does not
exist, is the one this clause must not use: that case reads `inactive` and would
pass whether or not the state ask ran, which is the never-failing perturbation
apex section 11 counts as worse than no test.

```graph
node: admin-failed-dial-consults-unit-state
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-failed-dial-consults-unit-state
```

```graph
node: admin-init-system-over-command-line
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-init-system-over-command-line
```

**The subprocess inherits nothing, because every descriptor this crate holds
is close-on-exec atomically at creation,** no descriptor existing for an
instant between its creating call and its flag. This is the behavioural half
of the custody section 1 opens, and section 10's third walk makes it a test,
where section 1's half is the ownership the compiler holds. The two halves
carry separate records because a test cannot demonstrate ownership and the
borrow checker cannot see a flag.

```graph
node: admin-cloexec-atomic-at-creation
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-cloexec-atomic-at-creation
```

**The unit template is fixed and the name is the one variable.** The
template lives in admin's own service configuration, per section 9, and the
only value interpolated is the validated agent name of section 4, so the
delegated authority stays bounded by the allow-list exactly as charter
section 7 requires. **The argument vector the ask carries takes no second
variable.** Its three values are the coordination socket path of section 7,
which this crate already derives from that same validated name, and the two
organ binary paths section 9 holds among the operator's installed values. A
builder who let any of the three be composed from the invocation's own input
would be widening the delegated authority by the route the name check closes,
so the shape to hold is that the vector reads the allow-listed name and the
operator's file and reads nothing else.

**The unit declares no descriptor-bearing open, and the absence is the
assertion.** Under the
inversion the worker starts bare and builds its own coordination socket inside
the sandbox, per `weaver-harness-Spec` section 2.3, so nothing is placed into
the unit at start and no descriptor crosses the init system at all. The sink's
descriptor crosses later and elsewhere, inside the enter directive as ancillary
data over the connection admin dialed, per charter section 4.1 step 6. A
builder adding a socket declaration to this invocation would be reviving the
route the inversion retired, so what this clause asserts is that the start
invocation carries no descriptor-bearing property.

**Bare is a statement about descriptors and the argument vector does not
qualify it.** The distinction is worth holding because the two are easy to
read as one absence: a descriptor is a capability the manager would have to
hold and pass, and an argument is a value the worker reads and then resolves
for itself under its own identity. The worker opens what its arguments name, so
a path in the vector grants nothing the agent uid did not already have, which
is why the vector leaves the reliance set of
`weaver-admin-systemd-contract` section 5 untouched while a descriptor route
would not. The assertion below is unchanged by this act and is the one a
reviewer checks: the invocation carries no descriptor-bearing property, whatever
else it carries.

The earlier form of this clause named a listen-fds route, and the measurement
of 2026-08-05 is kept in the charter's section 10 rather than here: the
manager's own descriptor passing does deliver a caller-held end into a unit,
and the design no longer needs it.

```graph
node: admin-unit-declares-no-open
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-unit-declares-no-open
```

**Worker death is observed, not reported.** The channel's closure is the
observation, per `weaver-organ-channel` section 2, and the unit's status is
consulted through the same command-line interface for the report the log
carries. Admin repairs nothing on either, per the charter.

## 7. The coordination channel

The channel of charter section 6, dialed fresh per verb.

**The socket type is `SOCK_SEQPACKET`, carrying the election of
`weaver-types-Spec` section 4 rather than re-deciding it.** That Spec named
this document a landing site for the boundary-preserving election, and it
lands here on the connection this crate dials: one write is one message, one
message is one JSON envelope, and no framing enters any contract that draws
the channel. A landing site carries an election and does not declare it, so
the election and its envelope bound are both asserted at that Spec's section
4.3 and neither takes a record here. The socket the harness binds carries the
same type, per `weaver-harness-Spec` section 2.3, because a connect against a
listener of another type fails at the kernel and the two sides elect one thing.

**The boundary the type buys is tested where the connection is made, and that
test is this crate's.** The election is that Spec's record and the conduct at
it is this crate's, the division the receive discipline below already takes:
`weaver-types-Spec` section 5 owes the connecting and binding crates a test
with two halves, and the boundary half is that one envelope written on this
channel is one envelope read, arriving neither split nor merged with its
neighbour. Section 10 names it with the substitution its watch turns on,
`SOCK_STREAM` at the creating call leaving every truncation test passing while
the framing every contract that draws this channel rests on is gone.

```graph
node: admin-one-write-is-one-read
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-one-write-is-one-read

edge: grounds
from: admin-one-write-is-one-read
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**The channel is reached in one act, the dial, and the retry bound is the
whole of its subtlety.** The four acts this section carried until 2026-08-05
were admin's bind, listen, accept, and close, and the inversion moved every one
of them to the harness: the socket lives inside the agent's sandbox and the
harness binds it as its first act, per `weaver-admin-harness-contract` section
2. What admin does is connect, per verb, to the per-agent name the operator's
configuration places, with the close-on-exec flag asked for in the socket call
itself and the connection closed when the verb answers.

**The dial retries within a bound because the bind is the worker's first act
and the load's dial may arrive first.** The load starts the unit and then
dials, so the race is real and structural rather than incidental: the elected
bound is one second of attempts at ten millisecond intervals, and a bound
exceeded refuses the load with `NoResidency` rather than waiting without end.
The numbers are this Spec's election and the charter states only that a bound
exists, per its section 4.1 step 6. A retry loop with no ceiling is what this
election exists to refuse, because a worker that never binds would otherwise
hang the operator's terminal rather than answering.

**The connect is nonblocking, and this is a requirement of the bound rather
than a preference.** Measured 2026-08-06: with the listener's backlog full, a
blocking `connect` on an `AF_UNIX` socket was still blocked after three seconds
against a one second ceiling, while the same connect on a nonblocking socket
returned at once with the transient error a retry is for. **A full backlog is
reachable rather than theoretical**, because the harness serves one connection
at a time, so a second verb arriving while one is in flight meets exactly that.
A blocking connect would therefore leave the bound stated here and unheld,
which is the failure the election exists to prevent, reached by a different
road. The flag is cleared once the connection is made, because the enter
directive and the answer it waits for are blocking work and a nonblocking read
would report an empty channel as a fault rather than waiting.

```graph
node: admin-dial-retries-within-a-bound
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-dial-retries-within-a-bound

edge: grounds
from: admin-dial-retries-within-a-bound
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**The credential check is the harness's and takes no record here.** What
refuses a stranger on this channel is the peer credential read at the
harness's accept, root or refused, per the contract's section 2, and the
record for it is declared by the crate that performs it, per
`weaver-harness-Spec` section 2.3. The four records this section carried for
the earlier design, the bind ordering, the directory's mode, the credential
check, and the listener's closure after one accept, retire with the acts they
described. The closure is not merely relocated: a listener that answers one
verb and closes would leave every later verb with nothing to dial, so the
harness's listener lives as long as the worker and the property that replaced
the closure is the check itself.

**The receive discipline is the shared obligation.** The receive buffer is
sized to the 64 kibibyte envelope bound and a read returning with
`MSG_TRUNC` set is a channel fault and never a message, per the election's
own terms, and the same bound is asserted on this crate's sends. The bound
is the floor's number and the discipline is this crate's conduct at it,
which `weaver-types-Spec` section 5 owes to the pair-creating crates by
name, so the record for it lands here and not there.

```graph
node: admin-truncation-is-a-channel-fault
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-truncation-is-a-channel-fault

edge: grounds
from: admin-truncation-is-a-channel-fault
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**The enter directive and its ancillary payload are one message.** The
envelope is rendered to JSON and sent with the sink's descriptor as
`SCM_RIGHTS` control data on the same `sendmsg`, which is what makes the
descriptor cross once, in the enter exchange, with no separate delivery to
order against anything. The exchange identity is the floor's
`ExchangeId { opener: Admin, ordinal }`, ordinals assigned serially by this
crate, per `weaver-organ-channel` section 1.

**The run's identity is minted here and the session's is read.** They are two
different kinds of value and the distinction is worth holding, because an
earlier form of this crate derived one from the other and produced a record
whose runs were indistinguishable. The session is the operator's, declared in
the agent's config and carried uninterpreted, per `weaver-types-Spec` section
2 and the ruling in `weaver-admin-PRD` section 10. The run reference is this
crate's, minted at the load as an RFC 3339 timestamp in UTC at millisecond
resolution, which distinguishes without coordination, sorts in the order the
runs happened, and reads as a date to whoever opens the artifact. **Nothing is
remembered between invocations to produce it**, which is what makes it
answerable by a crate that holds nothing across time, and it is the whole of
what the contract's distinctness guarantee asks for.

**The exchange ordinal beside it is a different thing and stays a counter.**
That ordinal is serial within one connection and is the floor's, per
`weaver-organ-channel` section 1, and a per-invocation crate can hold it
because a connection does not outlive the invocation. A reader who sees both
in one envelope is seeing a counter scoped to a connection and a reference
scoped to a session, and conflating them is how the earlier defect was
written.

```graph
node: admin-run-reference-distinguishes
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-run-reference-distinguishes
```

```graph
node: admin-enter-carries-descriptor-in-one-message
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-enter-carries-descriptor-in-one-message
```

**One exchange in flight per worker, and the serialization is now the
harness's.** The channel's layer permits concurrency, per the drawn material,
and the contract forbids a second transition for the same agent. What held
that was admin's fleet map until 2026-08-05, and a per-invocation crate holds
nothing, so the property lands where the standing party is: the harness serves
one connection at a time and answers a directive arriving out of order with a
refusal, per the contract's section 4 and `weaver-harness-Spec` section 2.3.
This crate's obligation reduces to opening one exchange per invocation and
closing the connection when the verb answers, which one verb per process makes
structural rather than disciplined.

## 8. The operations log

**The format is NDJSON, one act per line, and it shares no schema with the
trace.** The charter fixes the custody, 0640 in a 0750 directory, both owned
by root, fleet-scoped and never inside an agent home. What this Spec adds is
the rendering: one JSON object per line, the same reading tools the stream's
consumers already hold, and a
field set that is this crate's own and deliberately not the event envelope,
because a shared schema is how a second author drifts into the first's
record. The file opens with `O_APPEND` and `O_CLOEXEC` like every descriptor
this crate holds.

```graph
node: admin-log-ndjson-own-schema
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-log-ndjson-own-schema
```

**What is logged is the charter's set.** Transitions directed and their
outcomes, refusals issued, rollbacks with what each act undid or could not,
and units started and stopped. Never a fact about what an agent did, per
charter section 2: the moment a line describes conduct rather than
supervision it is a second record of the agent, and the review that finds
one has found a defect. The instrument is named in that sentence and is the
only one available, no mechanism being able to tell a line about supervision
from a line about conduct.

**The line between supervision and conduct is a domain line, and that is what
this record grounds in.** What an agent did is a fact about the working the
harness is answerable for, and the trace is where that working is recorded. A
line describing conduct would make this crate a second author of an account it
sees only a part of, which is an organ reasoning about a domain that is not its
own, per apex section 5.5. What stays is supervision, which is this crate's own
domain: the transitions it directed, the refusals it issued, and the units it
started and stopped.

```graph
node: admin-log-never-records-agent-conduct
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-log-never-records-agent-conduct

edge: grounds
from: admin-log-never-records-agent-conduct
to: axiom-harness-integrates-by-the-loop
```

**Retention and rotation are deferred with a named settler.** The charter
declines to fix a format's lifecycle before a rollback has run, and this
Spec follows: the file grows until the operator rotates it by ordinary
means, and a rotation policy is elected when there is a measurement of what
accumulates, which is the charter's own grounds read forward.

## 9. The service's own configuration

**Admin has operator-installed configuration of its own, and this Spec names
it rather than leaving it implied.** The coordination socket's per-agent name,
the log directory, the unit template, the agent config directory, the allow-list,
and the two organ binary paths are deployment facts the operator installs. The
operator socket's path left this list with the socket on 2026-08-05, and the
coordination name stayed but changed hands: the operator places it, the harness
binds it, and admin dials it, so one value reaches two crates and the operator's
file is where they agree. They are not the agent config and no seam carries them,
which is why the file takes no contract of its own. **The file and its values part
company at the start ask, and the distinction is worth holding.** This crate is the
only one that reads the file. Three of the values do not stay in it: the
coordination socket's name and the two organ binary paths reach the worker in
section 6's argument vector, over the external boundary
`weaver-admin-systemd-contract` holds rather than over any seam. The shape is
a satellite of section 11: what is fixed here is that these values exist, that
they are the operator's to place, and that none of them is discovered at
runtime by searching.

**The organ binaries are on this list and not in the agent's declaration, and
the placement is the ruling rather than a convenience.** They are one
installation's facts rather than one agent's, identical for every agent the
operator runs, so a declaration carrying them would state one fact in as many
places as there are agents and make a binary's replacement an edit to every one
of them, which is the divergence gate G5 exists to refuse. The charter's own
test settles it from the other side: `weaver-harness-Spec` section 2 has the
organ binaries supplied to the composition root as a deployment fact and names
them not an operator election, and the agent's declaration is exactly the
operator's elections.

**The worker's composition root receives what it needs and reads none of this
file.** An earlier wording of this section had that root reading these values
alongside admin, which no longer describes anything: the values reach it as the
argument vector of section 6, admin being the party that already holds both the
validated name and the operator's file. The correction matters beyond
tidiness, because a worker reading this file would take a dependency on a shape
section 11 holds open and would put a second reader on values only one party
places.

## 10. What is enforced, and by which instrument

Per apex section 11, with the threat walks stated first and each test a walk
made executable.

**The first walk: the agent drives its own lifecycle.** The adversary is an
elected tool running as the agent uid, and the targets are the two ways a
lifecycle verb could be reached. Running the binary is one, and it is refused
because the invocation performs nothing unless it holds root, per section 2.
Dialing the coordination socket is the other, and it is refused at the
harness's accept, whose credential check expects root and holds the record for
it, per `weaver-harness-Spec` section 2.3. The test on this crate's side: the
binary run as a non-root uid refuses before touching any agent, watched to fail
when the root check is removed. The walk lost two mechanisms with the recut,
the operator socket's predicate and the coordination directory's 0700, and it
is stronger for the exchange: both were fences in front of a check that could
not tell the worker from a tool, and what stands now is a check that can.

**The second walk: the agent reaches the sink by path.** The adversary is
the same tool surface, the attack a path traversal to the sink the config
names. The mechanism is the operator's provisioning, verified by the
inventory: the containing directory denies the agent uid the search bit, so
the kernel refuses the lookup before any mode on the file is consulted. The
test: an inventory run against a boundary whose sink directory grants the
agent traversal refuses the load, watched to fail when the check is removed.
The test reaches this one check of section 4's list and none of the others,
which is why that list carries two records rather than one.

**The third walk: admin's own subprocess inherits a descriptor.** The
adversary is whatever the run tool execs becoming an unintended holder of
the sink, a connection, or the log. The mechanism is atomic close-on-exec at
every creating call in this crate, no descriptor existing between creation
and flag. The test: spawn the subprocess, enumerate its descriptors, confirm
none of admin's crossed, watched to fail when any single atomic flag is
downgraded to a later `fcntl`.

**The fourth walk: a stranger speaks on the coordination channel.** The
adversary is a process running as the agent's uid, or as any uid on the host
that is not root, dialing the worker's socket. The mechanism is the harness's
credential check at accept, root or refused, and both the mechanism and its test
belong to the crate that performs them, per `weaver-harness-Spec` section 2.3, so
this crate cites the walk and declares no record for it.

**The check separates root from everything else and does not separate this crate
from other root processes, which is stated rather than implied.** `SO_PEERCRED`
yields a uid, so what the harness can know is that its peer holds root, not that
its peer is `weaver-admin`. Any root process on the host may therefore dial an
agent's coordination socket and direct its lifecycle. **That is not an
additional exposure and the reason is worth naming**: a root process already may
`ptrace` the worker, read and write its memory, replace the binary the unit
starts, or kill it, so a channel that admitted root adds nothing to what root
held before it. The boundary this program draws is between the agent and root,
and it is drawn where the agent cannot cross it. A check that separated admin
from other root processes would need a credential the operating system does not
supply at a socket and would defend against a party the trust model already
trusts, per `weaver-admin-PRD` section 2's statement that the program secures
the agent against reaching its own record and secures nothing against the
operator. What this crate owes the walk is the dial itself
being the only route it takes: no second connection is opened, none is kept
across verbs, and the connection closes when the verb answers, so a descriptor
to a running agent's supervisor exists only for the life of one invocation.

**Enforced by the compiler.**

- The floor's three wire enums are exhaustive, so every directive, answer,
  and refusal case reaches this crate's matches loudly. `weaver-types-Spec`
  section 4.2 argues and asserts that property and this crate consumes it.
- Descriptors are owned types end to end, a leak being a move the borrow
  checker sees. That is the ownership half of the custody section 1 opens,
  and the atomic close-on-exec half is section 6's behaviour and the third
  walk's test, so this bullet claims the compiler for ownership only.
- The inventory is one function with two callers, so `validate` and `load`
  cannot drift, the one-code-path rule as a call graph. Where the verb stops
  is not a call-graph property, so section 3 leaves that half to review.

**Enforced by compile-fail tests, because the property is an absence.** The
floor already pins the load-bearing absence this crate depends on,
`PeerIdentity` deriving no `Deserialize`, so a credential cannot be
constructed from bytes a peer sent, per `weaver-types-Spec` section 3. This
crate adds none of its own: it is the path-holding party by charter, so the
no-path pins live on the worker's side of the seams and are declared by the
crates that hold that side, and a pin invented here would pin nothing the
charter claims.

**Enforced by the manifest.** The internal dependency is exactly
`weaver-types` with the `config` feature, read against the graph's one
floor-link under gate H2, and no direct `weaver-traits` line exists, which
is the charter's declared non-link as a checkable absence. No async runtime,
no bus crate, and no logging crate in the resolved tree, by the build-time
`cargo tree` assertion the floor Specs share.

**Which invariant each claim serves, and why most serve none.** Ten of the
thirty-one carry a `grounds` edge and those ten carry eleven edges, one
record grounding in two invariants. Six run to
`axiom-floor-is-vocabulary-behavior-is-socket`, one to
`axiom-contract-is-a-complete-interface`, one to `axiom-organ-and-submodule`,
and three to `axiom-harness-integrates-by-the-loop`.
**The test applied is whether the axiom is the reason the claim exists, or
whether the claim is a precondition of the axiom's own stated reason,** per
Document Format section 4. Remove the socket invariant and this crate has
no reason to publish no library and no reason to hold one internal dependency,
no reason for the verbs to sit behind a principal check, no envelope has to
arrive whole or a truncation to count as a fault, and the dial's bound has
nothing to bound, so those six ground in it. Remove it and the log is still
NDJSON, the FIFO still opens nonblocking, the inventory still repairs nothing,
and the identity is still built from the validated name, so those ground in
nothing.
**Twenty-two claims grounding in no invariant is the expected result and not a
gap**, per Document Format section 4: most of what this Spec elects is a
rendering, a mode, an ordering, or a route, and representation is what the
invariants are not about.

**`axiom-join-key-travels-with-the-work` takes nothing from this crate,** and
the reason is that invariant's own scope rather than an oversight in this pass.
A lifecycle directive belongs to no turn and carries no turn key, and every
message this crate sends is one, so there is no seam here at which the key
travels. The trace this crate opens a sink for is written by the harness and
admin authors no event in it.

The one edge to the contract invariant is the invocation's answer agreeing with
its exit status, which the recut left standing where the operator surface's two
edges retired with the socket. A contract is a complete interface, so an
interface that could answer one thing and exit another would be two interfaces
disagreeing, and the agreement is what makes the invocation's boundary
readable by a shell and a tool alike. The coordination channel's boundary
election is `weaver-types-Spec`'s record and grounds there. What lands here is
the conduct at it, one write read as one message and a truncation read as a
fault, and both of those ground in the socket invariant instead.

The organ invariant keeps one edge and it is the device check's, the device
having one authority and that authority not being this crate, which is the
domain partition apex section 5.4 draws.

**The three edges to the integration invariant are this crate's charter
position stated from the other side.** Admin authorizes and does not execute,
and the reason it does not is that integrating is the loop's: the stop answer is
relayed unchanged because what a stop found is a fact about a run the harness
conducts, the devices a binding assigns are unchecked because forming a view of
them would be this crate reasoning about a domain it cannot see, and the
operations log stops at supervision because conduct is recorded in the trace the
harness authors. The ten owings below are the same rule at the document level.

**One edge moved and one was added beside an existing one.** Both of the organ
invariant's edges were placed in the labelling pass, before the apex held a
fifth invariant, and apex section 5.4 was the nearest section then saying
anything about domains. That section settles what an organ is and apex section
5.5 settles what an organ is answerable for, and a claim about declining another
domain's question turns on the second. So the stop relay carries the new edge
alone, nothing in the classification of organs being a reason to relay an answer
rather than to read it, and the device check carries both, the single authority
it defers to being 5.4's partition and the declining being 5.5's bound.

One call in the pass is worth a reviewer's eye, and the recut vindicated the
reading behind it. The retired directory mode grounded in nothing, because what
the socket invariant turns on is that a peer is known rather than that a
stranger cannot resolve a name, and the inversion made that reading structural:
the socket is now reachable by design and the check is the whole of the
refusal. The descriptor route grounds in nothing on the apex's own terms, since
which party creates a channel and how a descriptor travels belong to the
contract governing that seam rather than to the apex, so the dial and the
ancillary payload elect a route the invariant left open.

**The sharpest decline against the integration invariant is the published
state.** Waiting on a ready aggregate reads at first like this crate deferring
to the harness's domain, and it takes no edge. The aggregate is a value this
crate's contract delivers, keying on it is what a contract's vocabulary is for,
and the record written is admin's own and sits wholly inside admin's domain.
Apex section 5.5 binds what crosses between domains and does not reach what an
organ does inside one, so an ordering held inside a verb grounds in nothing.
The same reading leaves the load's step ordering, the inventory's one function,
and the residency read from the init system unedged, each of them a sequence this
crate holds or a fact it consults rather than a reconciliation between two
domains.

**Where the assertion records sit, and which of these this crate declares.**
The records are at the clauses that argue the claims, across sections 1
through 8, rather than gathered here, per Document Format section 6: this
section sorts by instrument and the arguments are elsewhere, so a block here
would sit apart from the prose that earns it. Thirty-two records in all,
fifteen tagged for review, twelve for perturbation, three for the manifest,
and two for a compile pin. The residency record moved from review to
perturbation on 2026-08-06, when the code act gave it a test. The elections take nodes
because gate H1 would otherwise leave the largest decisions in this Spec
untraceable, and two review tags come from the sorting rather than from an
election: the verb's stopping short of any seam and the existence checks no
test reaches are the review halves of splits this section's own bullets take,
and a divided half counts with the bullet it divided out of, per Document
Format section 3.

**Ten records retired with the recut, one moved, and six were added, which is
the count's whole movement from thirty-six.** Retired: the operator surface's
six, its stream election, its accept-time refusal, its refusal-by-closure, its
serial answering, its bounded request line, and its bare wire shapes, each
dying with the socket rather than relocating. The coordination channel's bind
ordering, its directory's mode, its listener's closure after one accept, and
the one exchange in flight per agent, the last four retiring with the acts and
the map they described. Moved: the credential check, to
`weaver-harness-Spec` section 2.3, where the accept now happens. Added: the
root check and the answer-and-status agreement of section 2, the dial's bound
of section 7, the residency read from the init system of section 3, the state
ask that follows a failed dial, of section 6, and the unload's wait on a
confirmed stop, of section 3. The unit's declared open inverted
to a declared absence rather than retiring, so it is neither. A rebuild of the
graph reads this movement as the act's assertion
delta.

**A claim this Spec cites and another Spec argues is declared by that Spec,**
not here, because the assertion belongs where its argument and its test live
and a node declared twice is the one-name-two-nodes defect the format forbids
for identifiers. Ten are such owings and carry no record in this document, the
count holding across the recut because one left and one arrived. Two are cited
in this section: the exhaustive wire enums and the missing `Deserialize` on
`PeerIdentity`, both `weaver-types-Spec`'s, the no-path pins on the worker's
side having left this list with the seam clause that cited them. Eight more are
cited where the sections use them. Four are `weaver-types-Spec`'s: the parse's
totality of section 4, the sink's discriminated shape with the socket case's
absent creation flag of section 5, the boundary election of section 7, and the
envelope bound that election carries, which that Spec declares as its own
record beside the election rather than inside it. The denial ordering left the
list with the predicate that applied it. Four are `weaver-harness-Spec`'s: the
OS-surface election of that Spec's section 2.4, cited in section 1, its bind
of the coordination socket and its credential check at accept, both of section
2.3 and cited in sections 6 and 7, and its refusal of a directive arriving out
of order, cited in section 7. The shape behind the list is the crate's own:
admin authorizes and does not execute, so what a run does after the enter
directive is asserted where the run happens.

**Requiring a perturbation-verified test, beyond the walks.**

- The root check: the binary run as a non-root uid refuses before touching any
  agent, confirmed by watching a verb proceed when the check is removed.
- The answer and the exit status agree: a refusal exits non-zero and an answer
  exits zero, confirmed by watching a refusal exit zero when the status is
  taken from the wrong branch.
- The dial's bound: a dial against a name nothing binds refuses within the
  bound rather than waiting, confirmed by watching the invocation hang when the
  ceiling is removed from the retry loop.
- The FIFO refusal: a pipe sink with no reader refuses the load with
  `ENXIO` mapped to its case, confirmed by watching the load hang when the
  nonblocking open is made blocking.
- The rollback's account: a load failed at each step leaves exactly what
  charter section 5 names and the log records what was undone, confirmed by
  watching the account go silent when logging moves off the rollback path.
- Truncation is a fault: an over-bound envelope on the coordination channel
  produces the fault and no directive, confirmed by watching a silently
  shortened answer decode when the `MSG_TRUNC` check is removed.
- One write is one read: two envelopes are written back to back on the
  coordination channel and both writes complete before either read, and two
  reads return exactly one envelope each, confirmed by watching the first
  read return both when the socket is created as `SOCK_STREAM`. **Two
  messages are what make the watch reachable.** The truncation bullet above
  cannot see that substitution at all, `MSG_TRUNC` handling being untouched
  by it, and a single small envelope crosses a stream socket whole, so a
  one-message test would pass under the substitution and pin nothing, which
  is the never-failing perturbation apex section 11 counts as worse than no
  test. This is the boundary half of the pair test `weaver-types-Spec`
  section 5 owes the pair-creating crates, argued at section 7, and it
  discharges this crate's side of that owing alone.

## 11. Open elections

Each names what settles it, and none is this Spec's to settle alone.

- **How an agent's lifecycle state is observed, and what the `State` answer
  carries meanwhile.** Section 3 reports residency in the manager's own three
  values because that is what the init system can answer, and apex section 6's
  four states are the harness's to know. The two halves are one election: the
  manager's `active` covers both `Idle` and `Active` and its `failed` has no
  `AgentState` case, so `lifecycle-answer`'s `State` case has no producer for
  these verbs until an observation reaches the party that holds the run.
  **Settled by:** an observation exchange on `weaver-admin-harness-contract`,
  which charters enter, leave, and stop and no query, together with whatever
  `weaver-types` owes its enumeration once that exchange fixes what can be
  observed. The answer arrives with that contract's next opening rather than
  from a mapping this Spec could invent.
- **The session-close cue and the enter question.** Charter section 10's two
  cells, settled by the human's ruling and the memory-and-state round
  respectively, carried here only so this list is complete.
- **The log's field set and rotation policy.** Satellites of section 8,
  the fields a builder's choice with no cross-crate consequence, the
  rotation elected against a measurement of what accumulates.
- **The service configuration's shape.** Section 9's file: its format and
  field list are a builder's choice bounded by what that section fixes,
  and the natural candidate is the same dialect the agent config elected,
  one syntax for everything the operator writes, per the common-syntax
  direction the composability batch recorded on the working list.
- **`AgentState` and `AgentSummary` field lists.** The floor names the
  types in `lifecycle-answer` and their fields are satellites there,
  consumed here as drawn.
- **The two values the argument vector does not carry.** Section 6's vector
  carries the socket path and the two organ binaries and stops there, and the
  worker's remaining two inputs are named here rather than routed, because
  routing either now would carry a value nothing reads. The assembled prompt's
  identity is one: the agent's declaration holds an identity the SPU makes
  resident as the session's prefix, the assembled prompt holds a second the
  loop reads, and whether those are one thing is the assembly question rather
  than this act's. The tool schemas are the other, the declaration's tool set
  reaching this crate and going no further while nothing dispatches a tool.
  **Settled by:** the assembly and distillation act for the first, and the
  tool workflow that charters dispatch for the second. Until then the worker
  defaults both, which is why a load stands without either.
- **The unit template's hardening set.** Which properties beyond `User=`
  the fixed template carries is the operator's policy surface, named in
  section 9's configuration and deliberately not enumerated by this Spec,
  because a hardening list frozen in a Spec is a security posture that
  cannot track its host. **The properties the sandbox must
  deliver are required and their directives are not**, per the operator's
  ruling of 2026-08-05 and `weaver-admin-PRD` section 7: no privilege
  escalation from inside, no reach into another principal's home, and a bound
  on what the agent may consume. The operator owns the posture the way an
  operator owns a firewall configuration.
- **Whether the unit restricts the agent's address families to `AF_UNIX`.**
  Open with a stated cost rather than required, per the same ruling. It is
  **not** a restatement of the rule that no crate here exposes a network
  surface: that rule binds what these crates link, and this would bind what an
  agent's tools may reach, so an agent whose tools fetch anything would break
  under it. Settled by an operator who knows which tools their agents carry.
