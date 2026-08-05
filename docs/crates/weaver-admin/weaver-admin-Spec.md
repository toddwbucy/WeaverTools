# weaver-admin - Spec

**Status:** MERGED. Cut 2026-08-02, fifth of the Spec pass and the first outside the
agent. Code is written against it under the gates of Working Process section 6, ratified
2026-08-04.

**Date filed:** 2026-08-02
**Document ID:** `weaver-admin-Spec`
**Parent:** `weaver-admin-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

Build instructions for `weaver-admin`: the binary's layout, the operator surface's
mechanics, the verb sequencing, the sink openings, the transient unit's
invocation, the coordination channel's construction, and the elections a builder
would otherwise invent. It is derived from `weaver-admin-PRD` and from the two
contracts this crate is party to, `weaver-admin-harness-contract` and
`weaver-admin-operator-contract`, together with `weaver-organ-channel`, the drawn
material the first of them draws.

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

    src/main.rs       entry and wiring, and nothing else
    src/surface.rs    the operator socket, section 2
    src/verbs.rs      load, unload, validate, stop, and rollback, section 3
    src/inventory.rs  config validation and boundary verification, section 4
    src/sink.rs       sink resolution and opening, section 5
    src/unit.rs       the transient unit and its declared opens, section 6
    src/channel.rs    the coordination channel, section 7
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
`serde_json` encodes and decodes the operator surface's lines and the
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

## 2. The operator surface

The socket of charter section 8, governed by `weaver-admin-operator-contract`.

**The socket is `SOCK_STREAM`, and the election is the contract's own framing
rule read as a type.** The contract fixes newline-delimited JSON, one request
per line, one answer per line, in order, on one connection. The newline is the
framing, so a boundary-preserving socket type would carry a second framing
under the first, and a stream is what the operator's ordinary tooling dials.
This is the opposite election from the organ channels and it is principled the
same way the two envelope layouts were: the organ channel has no in-band
framing and buys boundaries from the type, this surface has in-band framing
from its contract and buys nothing from packet boundaries.

**This record grounds in the contract invariant and not in the socket one.**
What decides the type here is that the contract already fixes the framing
completely, so what this Spec elects is a type that carries that framing and
adds nothing under it. The socket invariant settles that this seam is a socket
and authenticates its peer, and it leaves the choice between the two types
open, which is why the coordination channel's opposite election is argued from
boundaries and this one is not.

```graph
node: admin-operator-socket-stream
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-operator-socket-stream

edge: grounds
from: admin-operator-socket-stream
to: axiom-contract-is-a-complete-interface
```

**The socket lives in an admin-owned directory the operator's group can
search.** The directory is owned by the `weaver-admin` service, group
`weaver-admin`, mode 0750, and the socket file mode 0660, so a group member
can dial and the agent uid cannot traverse to the name. Reachability is not
the authentication, per the contract's section 1: it is one more fence in
front of the check that counts.

**Every connection is authenticated at accept, by peer credential, before any
byte is read.** `accept` runs with the close-on-exec flag set in the accepting
call, the peer's credential is read with `SO_PEERCRED`, and the identity is
judged by the floor's one predicate, `authorized`, against a rule whose allow
set is the `weaver-admin` group and whose deny set is every agent uid the
fleet's allow-list names. Denial wins over permission, per
`weaver-types-Spec` section 3, so an agent uid inside a misconfigured group
grant is still refused. That ordering is argued and asserted at that Spec's
section 3 and applied here, so it carries no second record. Verified against
a live kernel: the credential on an accepted connection reports the
connecting peer's own uid, gid, and pid, which is what makes this check real
where an inherited pair's credential is not. The check itself is this
crate's, and section 10's first walk derives its test from the attack it
defeats.

```graph
node: admin-surface-refuses-at-accept
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-surface-refuses-at-accept

edge: grounds
from: admin-surface-refuses-at-accept
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**A peer that fails the predicate is refused by closure, unanswered, and the
two contract clauses are one behavior.** The contract's section 5 lists the
failed predicate as a refusal and its section 6 forbids answering such a peer.
Both hold: the refusal is enacted at the connection rather than written to it,
the connection closing before any content is read, and no
`lifecycle-refusal` value crosses to a peer the predicate rejected.

```graph
node: admin-refusal-by-closure-unanswered
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-refusal-by-closure-unanswered

edge: grounds
from: admin-refusal-by-closure-unanswered
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**One thread per accepted connection, requests served serially within it.**
Serial-per-connection is what makes one answer per request in request order a
structural property rather than a bookkeeping claim. Across connections the
fleet state of section 3 is the synchronization point, and a transition holds
its agent's entry so that a second directive for the same agent refuses
`OutOfOrder`-shaped rather than queueing, per the contract's section 4, which
section 7 argues as the fleet state's own discipline and records there. The
threads are the standard library's, per section 1.

```graph
node: admin-one-answer-per-request-serially
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-one-answer-per-request-serially
```

**A request line is bounded at 64 kibibytes, the program's one message bound.**
A line longer than the bound is refused as malformed without being accumulated,
because an unbounded line is an unbounded allocation handed to an
unauthenticated-in-content peer. The number is the organ envelope's bound
reused, one limit for the program rather than a second constant to justify, so
the bound itself is `weaver-types-Spec` section 4.3's claim and what this
clause asserts is the refusal, a line over the bound rejected rather than
accumulated toward a message that never arrives.

```graph
node: admin-request-line-refused-unaccumulated
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-request-line-refused-unaccumulated
```

**The wire shapes are the floor's own, bare.** One request line is one
`lifecycle-directive` in the floor's internally tagged rendering, one answer
line is one `lifecycle-answer` or one `lifecycle-refusal`, and the discriminant
between the two is the tag itself: the case sets are disjoint at the floor, so
a consumer keys on the tag and needs no wrapper. No organ envelope appears on
this surface, because this is not an organ channel and the contract draws no
envelope.

```graph
node: admin-surface-carries-no-envelope
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-surface-carries-no-envelope

edge: grounds
from: admin-surface-carries-no-envelope
to: axiom-contract-is-a-complete-interface
```

## 3. The verbs, the fleet state, and rollback

**The fleet state is a locked map, one entry per provisioned agent, holding
the published state and the in-flight flag.** The published states are the
charter's two, provisioned-and-unloaded and loaded-and-idle, rendered to the
floor's `AgentState` for the `State` and `Agents` answers. Loaded-and-idle is
published only on a ready aggregate and never earlier, per charter section
4.1 step 7, and a failed anything publishes nothing, per section 5. The map
is where that holds, one write of the published state per transition and
none on any other path, and no test below reaches it, so the instrument is
review reading the transition sites against this rule.

```graph
node: admin-publishes-only-on-ready
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-publishes-only-on-ready
```

**`load` runs the charter's seven steps in order, with the channel's four
acts interleaved at the split section 7 states, and the merged sequence is
code rather than convention.** Authorize, validate through section 4's one
inventory, verify the boundary in the same inventory, resolve the session and
open the sink per section 5, bind and listen the coordination channel, start
the unit per section 6, accept the worker's connection and direct enter per
section 7, publish. Eight actions, the charter's seven with bind-and-listen
standing as its own act before the unit, because the worker connects at its
start and a name not yet listening is a race. Each step's failure returns a
typed `lifecycle-refusal` to the operator and enters the rollback below
carrying the step's name. The ordering is one claim and carries one record,
at the section 7 clause that argues where the split falls, so this sequence
cites it rather than restating it.

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

**`unload` runs the charter's three steps.** Direct leave and await the
aggregate, stop the unit through section 6's interface, publish
provisioned-and-unloaded. A refusal on leave, `ActivityNotAtRest` above all,
returns to the operator unchanged and publishes nothing.

**`stop` is a conveyance and its answer is a relay.** The operator's stop
crosses the surface, admin opens the stop exchange on the coordination
channel, and the harness's answer, `TurnAborted` or `AtRest`, returns to the
operator as received. Admin holds no opinion about which, per charter section
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
mechanically instead.** The sink path's containing directory is admin-owned
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
name is validated against the fleet's allow-list and the constructed identity
is `weaver-<name>` from the validated name, never from a caller-supplied
string, which is the argless-grant discipline of charter section 7 landing at
the one site that constructs. It is the one site because the same validated
name is what section 6 interpolates into the unit template, so the delegated
authority has one origin and review reads that origin rather than every use.

```graph
node: admin-identity-from-validated-name
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-identity-from-validated-name
```

## 5. The sink

Opened by the discriminant the config carries, under admin's own principal,
every descriptor close-on-exec in the opening call itself.

**`File { path, create }`.** Opened write-only with `O_APPEND`, `O_CLOEXEC`,
and, when the flag is set, `O_CREAT` at mode 0640, owned by the service and
grouped to `weaver-admin`, which is the custody of charter section 7.
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
agent's `User=`, the fixed template's hardening, and the declared open that
connects the coordination socket. Stopping it is one invocation of the stop
verb. The alternative is a bus library, and it loses on the tree: a D-Bus
crate brings an async runtime or its own event loop into a binary that
otherwise needs neither, for two invocations per lifecycle that are neither
hot nor latency-bound.

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
section 7 requires.

**The declared open is the route the coordination end takes, and it
arrives by the listen-fds convention.** The unit's declaration names the
coordination socket's path, the init system connects it at the worker's
start, and the worker receives the connected end at its first instruction
through the listen-fds interface, which is where the composition root takes
the `OwnedFd` it hands `Harness::adopt`, per `weaver-harness-Spec` section
2.3. What the composition root does with that end once it has it is the
harness's claim and asserted there. The sink's descriptor does not travel
this way: it crosses inside the enter directive as ancillary data, per
charter section 4.1 step 6, so the unit declares exactly one open, which is
what this clause asserts and what a builder adding a second declaration would
breach.

```graph
node: admin-unit-declares-one-open
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-unit-declares-one-open
```

**Worker death is observed, not reported.** The channel's closure is the
observation, per `weaver-organ-channel` section 2, and the unit's status is
consulted through the same command-line interface for the report the log
carries. Admin repairs nothing on either, per the charter.

## 7. The coordination channel

The channel of charter section 6, constructed fresh per load.

**The socket type is `SOCK_SEQPACKET`, carrying the election of
`weaver-types-Spec` section 4 rather than re-deciding it.** That Spec named
this document a landing site for the boundary-preserving election, and it
lands here on the bound socket: one write is one message, one message is one
JSON envelope, and no framing enters any contract that draws the channel. A
landing site carries an election and does not declare it, so the election
and its envelope bound are both asserted at that Spec's section 4.3 and
neither takes a record here. The connected pair that Spec speaks of is what
an accept produces, the bind-and-declared-open route having superseded the
forked pair on the charter's fourteenth-entry ruling.

**The boundary the type buys is tested where the pair is made, and that test
is this crate's.** The election is that Spec's record and the conduct at it
is this crate's, the division the receive discipline below already takes:
`weaver-types-Spec` section 5 owes the pair-creating crates a test with two
halves, and the boundary half is that one envelope written on this channel
is one envelope read, arriving neither split nor merged with its neighbour.
Section 10 names it with the substitution its watch turns on, `SOCK_STREAM`
at the creating call leaving every truncation test passing while the
framing every contract that draws this channel rests on is gone. The half
stood unwritten from the pass until this act, filed as issue 35 and closed
across both pair-creating crates in one act because the property and its
watch are the same on either side.

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

**The channel is built in four acts, the acts straddle the unit's start, and
the split is the ordering.** The first two acts run before the unit starts,
because the worker connects at its start through the declared open and a
connect against a name not yet listening is the race the ordering exists to
prevent, and the last two run after it, because there is no peer to accept
until the worker exists. Each of the four acts carries a verified property,
and the first two are these. The socket is created with the close-on-exec
flag in the creating call and bound to a per-agent name inside an
admin-owned directory of mode 0700, the unsearchable home the charter's
section 6 requires, listening before the
unit is asked for. The ordering is the record the load of section 3 cites,
one claim declared once, and the directory's mode is the first walk's first
mechanism, held by review because no test below reaches it.

```graph
node: admin-bind-and-listen-precedes-unit-start
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-bind-and-listen-precedes-unit-start

node: admin-coordination-directory-unsearchable
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-coordination-directory-unsearchable
```

**The last two acts are the accept and the close, and each is tested.** The
listener accepts exactly once, after the unit's start, the accepting call
setting close-on-exec on the connection.
The peer credential is read at that accept and checked against the agent's
expected uid, the check the charter names as available and this Spec elects
because it costs one call: possession remains the authentication, and the
credential confirms the possessor is the worker the unit started rather
than a surprise, refusing the connection on a mismatch. And the listener is
closed after the one accept, verified: a later dial is refused by the
kernel while the accepted connection lives on, so no second opener exists
even for a process that somehow resolved the name, structure doing the work
the search bit already does. Section 10's fourth walk tests the credential
check and its first walk tests the closure, so both are behaviours with a
perturbation behind them rather than elections.

**Both records ground in the socket invariant, and the closure does so for a
reason worth stating.** The credential check is that invariant's named mechanism
at a channel reached by a path. The closure is what leaves possession meaning
something at this channel: an elected tool running as the agent uid passes the
credential check, being the uid the check expects, and what refuses it is that
no listener remains to answer a second dial. The 0700 directory of the
paragraph above is a fence in front of the check rather than the check itself,
so it takes no edge.

```graph
node: admin-coordination-peer-credential-checked
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-coordination-peer-credential-checked

edge: grounds
from: admin-coordination-peer-credential-checked
to: axiom-floor-is-vocabulary-behavior-is-socket

node: admin-listener-closed-after-one-accept
kind: assertion
tag: perturbation

edge: asserts
from: weaver-admin
to: admin-listener-closed-after-one-accept

edge: grounds
from: admin-listener-closed-after-one-accept
to: axiom-floor-is-vocabulary-behavior-is-socket
```

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

```graph
node: admin-enter-carries-descriptor-in-one-message
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-enter-carries-descriptor-in-one-message
```

**One exchange in flight per agent, by the fleet state's lock.** The
channel's layer permits concurrency, per the drawn material, and this
crate's own contract forbids a second transition for the same agent, so the
serialization is admin's discipline at the fleet state rather than a
property claimed of the wire. This is the claim section 2's threading
paragraph cites, one record for one rule, and it is review's because the
lock is read at the fleet state rather than exercised by a test named below.

```graph
node: admin-one-exchange-in-flight-per-agent
kind: assertion
tag: review

edge: asserts
from: weaver-admin
to: admin-one-exchange-in-flight-per-agent
```

## 8. The operations log

**The format is NDJSON, one act per line, and it shares no schema with the
trace.** The charter fixes the custody, 0640 in a 0750 directory, both owned
by the service and grouped to `weaver-admin`, fleet-scoped and never inside
an agent home. What this Spec adds is the rendering: one JSON object per
line, the same reading tools the stream's consumers already hold, and a
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
it rather than leaving it implied.** The operator socket's path, the
coordination directory, the log directory, the unit template, and the fleet
allow-list are deployment facts of the service, installed by the operator
beside the delegation policy of charter section 7. They are not the agent
config, they cross no seam, and no crate but this one reads them. The shape
is a satellite of section 11: what is fixed here is that these values exist,
that they are the operator's to place, and that none of them is discovered
at runtime by searching.

## 10. What is enforced, and by which instrument

Per apex section 11, with the threat walks stated first and each test a walk
made executable.

**The first walk: the agent dials its own supervisor.** The adversary is an
elected tool running as the agent uid, and the target is either named
socket, the operator surface or the coordination listener. The mechanisms
are layered: the coordination directory's 0700 denies the traversal, the
listener is closed after its one accept, and the operator surface's
predicate refuses the agent uid at accept, denial winning over any group
grant. The tests: a connection attempt from a non-member uid is refused
before any content is read, watched to fail when the predicate weakens, and
a post-accept dial of the coordination name is kernel-refused, watched to
fail when the listener is left open. The first test is the operator-surface
twin of the walk `weaver-types-Spec` section 3 named for the gate, landing
in this crate because this crate owns the second consumer.

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

**The fourth walk: a stranger answers on the coordination channel.** The
adversary is any process that is not the started worker holding the
channel's far end. The mechanisms are possession, the declared open placing
the end only in the unit, and the elected credential check at accept
refusing a peer whose uid is not the agent's. The test: a connection from a
wrong uid is refused at accept, watched to fail when the check is removed.

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

**Which invariant each claim serves, and why most serve none.** Thirteen of the
thirty-six carry a `grounds` edge and those thirteen carry fourteen edges, one
record grounding in two invariants. Eight run to
`axiom-floor-is-vocabulary-behavior-is-socket`, two to
`axiom-contract-is-a-complete-interface`, one to `axiom-organ-and-submodule`,
and three to `axiom-harness-integrates-by-the-loop`.
**The test applied is whether the axiom is the reason the claim exists, or
whether the claim is a precondition of the axiom's own stated reason,** per
Document Format section 4. Remove the socket invariant and this crate has
no reason to publish no library and no reason to hold one internal dependency,
no credential is read at either accept, and no envelope has to arrive whole or a
truncation to count as a fault, so those eight ground in it. Remove it and the
log is still NDJSON, the FIFO still opens nonblocking, the inventory still
repairs nothing, and the identity is still built from the validated name, so
those ground in nothing.
**Twenty-three claims grounding in no invariant is the expected result and not a
gap**, per Document Format section 4: most of what this Spec elects is a
rendering, a mode, an ordering, or a route, and representation is what the
invariants are not about.

**`axiom-join-key-travels-with-the-work` takes nothing from this crate,** and
the reason is that invariant's own scope rather than an oversight in this pass.
A lifecycle directive belongs to no turn and carries no turn key, and every
message this crate sends is one, so there is no seam here at which the key
travels. The trace this crate opens a sink for is written by the harness and
admin authors no event in it.

The two edges to the contract invariant are the operator surface's, and they
are where this crate's two seams part company. The surface is `SOCK_STREAM`
because its contract fixes the framing completely and a boundary-preserving type
would carry a second framing under the first, and it carries no envelope because
the contract draws none, an explicit nothing being what that invariant makes an
assertion rather than a silence. The coordination channel's opposite election is
`weaver-types-Spec`'s record and grounds there. What lands here is the conduct
at it, one write read as one message and a truncation read as a fault, and both
of those ground in the socket invariant instead.

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

Two calls in the pass are worth a reviewer's eye. The coordination directory's
0700 grounds in nothing, because what the socket invariant turns on is that a
peer is known rather than that a stranger cannot resolve a name, per the
operator contract's own reading that reachability is not the authentication, and
the listener's closure grounds where the directory does not for the reason
section 7 states at it. The descriptor routes ground in nothing on the apex's
own terms: which party creates a pair and how a far end travels to the process
that holds it belong to the contract governing that seam and are not the apex's
to settle, so the declared open and the ancillary payload elect a route the
invariant left open.

**The sharpest decline against the integration invariant is the published
state.** Waiting on a ready aggregate reads at first like this crate deferring
to the harness's domain, and it takes no edge. The aggregate is a value this
crate's contract delivers, keying on it is what a contract's vocabulary is for,
and the record written is admin's own and sits wholly inside admin's domain.
Apex section 5.5 binds what crosses between domains and does not reach what an
organ does inside one, so an ordering held at the fleet state grounds in
nothing. The same reading leaves the load's act ordering, the inventory's one
function, and the one exchange in flight per agent unedged, each of them a
sequence this crate holds rather than a reconciliation between two domains.

**Where the assertion records sit, and which of these this crate declares.**
The records are at the clauses that argue the claims, across sections 1
through 8, rather than gathered here, per Document Format section 6: this
section sorts by instrument and the arguments are elsewhere, so a block here
would sit apart from the prose that earns it. Thirty-six records in all,
eighteen from this section's sorting and eighteen from the elections outside
it, the elections taking nodes because gate H1 would otherwise leave the
largest decisions in this Spec untraceable. The eighteen elections are all
tagged for review, and two more review tags come from the sorting rather than
from an election: the verb's stopping short of any seam and the existence
checks no test reaches are the review halves of splits this section's own
bullets take, and a divided half counts with the bullet it divided out of,
per Document Format section 3. Every other record drawn from the sorting
carries a mechanical instrument.

**A claim this Spec cites and another Spec argues is declared by that Spec,**
not here, because the assertion belongs where its argument and its test live
and a node declared twice is the one-name-two-nodes defect the format forbids
for identifiers. Ten are such owings and carry no record in this document.
Three are cited in this section: the exhaustive wire enums and the missing
`Deserialize` on `PeerIdentity`, both `weaver-types-Spec`'s, and the no-path
pins on the worker's side of the seams. Seven more are cited where the
sections use them. Five are `weaver-types-Spec`'s: the denial ordering of
section 2, the parse's totality of section 4, the sink's discriminated shape
with the socket case's absent creation flag of section 5, the boundary
election of section 7, and the envelope bound that election carries, which
that Spec declares as its own record beside the election rather than inside
it. Two are `weaver-harness-Spec`'s:
the OS-surface election of that Spec's section 2.4, cited in section 1, and
its section 2.3 adoption of the coordination end, cited in section 6. The
shape behind the list is the crate's own: admin authorizes and does not
execute, so what a run does after the enter directive is asserted where the
run happens.

**Requiring a perturbation-verified test, beyond the walks.**

- One answer per request, in request order, on one connection, confirmed by
  watching interleaving appear when per-connection serialization is removed.
- The refused-before-read property: a failing peer's connection closes with
  no answer written, confirmed by watching a refusal value cross when the
  closure is replaced by a reply.
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
- **The unit template's hardening set.** Which properties beyond `User=`
  the fixed template carries is the operator's policy surface, named in
  section 9's configuration and deliberately not enumerated by this Spec,
  because a hardening list frozen in a Spec is a security posture that
  cannot track its host.
