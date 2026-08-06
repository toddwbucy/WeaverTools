# weaver-harness - Spec

**Status:** MERGED. Cut 2026-08-02, fourth of the Spec pass and the first above the
floor. Code is written against it under the gates of Working Process section 6.

**Date filed:** 2026-08-02
**Revised:** 2026-08-05, the socket inversion. Per the operator: any socket
connecting to the harness is an internal connection, so this crate binds the
coordination socket inside the agent's sandbox and listens where section 2.3
adopted a handed end, the constructor becoming `Harness::listen`, and every accept
reads the peer credential and refuses what is not root. The credential check arrives
from `weaver-admin-Spec`, where the accept used to happen, and the adopted end's
close-on-exec retires with the end. The listener is not closed after an accept,
admin being per-invocation.
**Document ID:** `weaver-harness-Spec`
**Parent:** `weaver-harness-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

Build instructions for `weaver-harness`: the module layout, the item signatures,
the channel mechanics this crate holds, the descriptor custody as code, and the
elections a builder would otherwise invent. It is derived from
`weaver-harness-PRD` and from the four contracts this crate is party to,
`weaver-admin-harness-contract`, `weaver-harness-trace-contract`,
`weaver-harness-spu-contract`, and `weaver-harness-gate-contract`, together with
`weaver-organ-channel`, the drawn material three of them share.

Level discipline. The charter says what the crate needs and why. This document
says how it is represented, and per gate G2 it elects against grounds the charter
and the contracts state rather than developing grounds of its own. Where this
document and the charter disagree the charter yields nothing.

**This document declares its crate's assertion records and no other record,** per
Document Format sections 3 and 4 as of the notation of 2026-08-03, which retired
the no-records sentence this paragraph replaces. The charter stays the source of
this crate's node, its parent edge, its two floor links, its one declared seam,
and its artifact edges, and a Spec that restated any of them would give the mapper
two sources for one record, per that format's section 1. What this document
sources is the claims code must conform to, declared at the clauses that argue
them rather than gathered in one place, per that format's section 6, and `asserts`
runs from the crate rather than from this document, which is why the document
needs no node of its own.

**A claim this Spec leans on and another Spec argues carries no record here,** and
there are ten of them. Four are `weaver-types-Spec`'s: the `SOCK_SEQPACKET`
election of its section 4, the 64 kibibyte envelope bound that election carries,
the three exhaustive wire enums of its section 4.2, and loop 0's JSON encoding of
its section 4.3. Two are `weaver-traits-Spec`'s: the tool trait's absent safety
classification of its section 5, leaned on in section 6, and the provider trait's
dyn-compatibility of its section 6, leaned on in section 5. Three are
`weaver-trace-Spec`'s: the identity newtypes whose conversion that Spec's section
1 places at this crate's submit call, the envelope index its section 4 holds for
the assembly read, and the recorder reporting pressure rather than authoring it,
its section 6. One is `weaver-gate-Spec`'s, the parent-death signal's
thread-scoped guarantee of its section 2, leaned on in section 1. A node declared
twice is the one-name-two-nodes defect that format forbids, and a reliance stays
prose because a citation is what it is.

**It is written from the merged corpus alone,** per the ruling of 2026-08-01 that
keeps the old tree's Specs out of the Spec pass. Where a question of fact about
the old tree mattered, the fact is cited as a fact and decides nothing.

**What this Spec can settle is bounded by what is chartered, and the bound is not
a gap.** The token workflow and the tool workflow are unchartered, so the decode
exchanges, the turn ingress traffic, the per-model assembly layer, and tool
dispatch defer here with their settlers named, the same half-chartered discipline
the SPU and gate charters run on. What is fully specifiable today is the
lifecycle interior of loop 0, trace authorship, and the descriptor custody that
protects the record, which is the spine of the deliverable rather than its
leftovers.

## 1. The crate

**Layout.** One module per obligation, re-exported at the root.

    src/lib.rs         re-exports, and nothing else
    src/channel.rs     organ-channel I/O and descriptor custody, section 2
    src/lifecycle.rs   the harness type, the run state, the fan-out, section 3
    src/authorship.rs  trace authorship, section 4
    src/assembly.rs    prompt assembly's deterministic floor, section 5
    src/tools.rs       the tool system, blocked, section 6
    src/engine.rs      loop 1's seat, the extension seam, section 6

Seven files, two of them placements, the way `weaver-traits-Spec` section 1
places its blocked and deferred modules.

**Edition and toolchain.** Edition 2024 on the pinned nightly, no nightly
feature used.

**The dependency set is three internal crates and two external ones, and each is
argued.** `weaver-traits` and `weaver-types` are the floor links the charter
declares, the first for the message model the authoring path licenses, the
second for the wire vocabulary of loop 0 and the identity types the envelope
carries. `weaver-types` is taken **without its `config` feature**: this crate
reads no field from the agent config's file, so it links no parser. The model
binding and the gate instruction are config fields this crate does consume,
arriving over the coordination seam inside the enter directive, already
validated, which is the file read staying admin's. The elections the file
carries beyond them, the permission mode, the tool set, and the residual
readout, are consumed by no workflow chartered today, and section 9 holds the
config read as the tool workflow's question. **The featureless link rests on a
placement owed to `weaver-types-Spec`.** `ModelBinding` and `GateInstruction`
are config fields and wire types at once, that Spec defines them in the module
its `config` feature gates while ruling the wire types unconditional, and this
crate can construct a directive with the feature off only if the two types sit
on the unconditional side. The owed edit is filed on the working list rather
than made here, a correction to a merged document landing after this branch
rather than beside it. `weaver-trace` is the seam tagged
`link`, the recorder this crate authors through. `serde_json` encodes and
decodes the loop 0 envelopes, whose JSON election is `weaver-types-Spec`
section 4.3's. `nix` is the OS surface, elected in section 2.4 where the grounds
and the record live. **The set and the feature are two records rather than one.**
Gate H2 reads the internal edges against the graph and reads no feature list, and
the featureless take is a `Cargo.toml` fact of its own, so a single record would
hand one instrument's read to a claim it does not cover. **The set grounds in the
socket invariant and the feature grounds in nothing.** Apex section 5.1 is the
reason the two organs appear nowhere in this list: a crate this one asks to do
something is reached over a socket, so the internal set is the floor plus the one
seam tagged `link`. Which features a floor link is taken with is a build election
that would read the same under any invariant.

```graph
node: harness-internal-dependency-set
kind: assertion
tag: manifest

edge: asserts
from: weaver-harness
to: harness-internal-dependency-set

edge: grounds
from: harness-internal-dependency-set
to: axiom-floor-is-vocabulary-behavior-is-socket

node: harness-types-without-config
kind: assertion
tag: manifest

edge: asserts
from: weaver-harness
to: harness-types-without-config
```

**No async runtime, no logging, no HTTP.** The old tree's harness carried tokio
with every feature, `async-trait`, `tracing` with two subscriber crates, and an
outbound HTTP client, and none of it crosses, per apex section 7: nothing in
loop 0 or the trace seam awaits anything, the lifecycle interior is serial by
charter, and the executor question belongs to the token workflow with a latency
measurement in hand, per `weaver-traits-Spec` section 6. A logging crate would
be a second account beside the one the program exists to produce. The absence
is checked by the same build-time `cargo tree` assertion the floor Specs share.

```graph
node: harness-no-runtime-no-logging-no-http
kind: assertion
tag: manifest

edge: asserts
from: weaver-harness
to: harness-no-runtime-no-logging-no-http
```

**This crate spawns no thread.** The one auxiliary thread the merged set names
is the stream writer's, and it belongs to `weaver-trace`, per that Spec's
section 5. Everything this crate does in this pass runs on the caller's thread,
and the writer's existence is why the fork sites of section 2 state their
safety bound rather than assuming a single-threaded process. **The organ forks
run on a thread whose lifetime is the worker's, and the sentence is a
constraint rather than a description.** The gate's parent-death backing fires
on the forking thread's termination rather than the process's, per
`weaver-gate-Spec` section 2 as verified by both seats, so a later threading
change that moved the enter fan-out onto a short-lived thread would kill the
gate spuriously while the interior it guards is healthy. Today the posture
above satisfies the constraint by construction, and this sentence is what a
change to that posture must answer. **Both are review's by election and not by
impossibility.** A test enumerating this process's threads reaches the first and
a test comparing the forking thread against the serving one reaches the second,
so what holds them is that this suite buys neither rather than that no instrument
exists. The second is the constraint `weaver-gate-Spec` section 2 filed against
this document on the working list, stated here as a record rather than left as an
implication of the posture above. **The constraint grounds in apex section 5.5 and
the posture does not.** The parent-death backing is a timing guarantee the gate
relies on and cannot hold from inside its own domain, because which thread forks it
is a fact only the harness has, so keeping that guarantee true while the interior
runs is the loop's work by that section's division of answerability. That this crate
spawns no thread of its own is an interior posture no other organ's ordering reads.

```graph
node: harness-spawns-no-thread
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-spawns-no-thread

node: harness-organ-forks-on-worker-lifetime-thread
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-organ-forks-on-worker-lifetime-thread

edge: grounds
from: harness-organ-forks-on-worker-lifetime-thread
to: axiom-harness-integrates-by-the-loop
```

## 2. The channels, and custody as code

This crate is party to three socket seams and creates two of them. The
mechanics below are one election carried and the obligations the contracts
land here, each with its contract named.

### 2.1 The socket type, carried

**The pairs this crate creates are `SOCK_SEQPACKET`, carrying the election of
`weaver-types-Spec` section 4 rather than re-deciding it.** That Spec elects the
type for the organ channels and names this document as a landing site: the
boundary property of `weaver-organ-channel` section 2 comes from the socket
type, and this crate creates the residency and gate pairs. The election arrives
with its obligation. **The receive buffer is sized to the maximum envelope of
64 kibibytes, and a read that returns with `MSG_TRUNC` set is a channel fault
and never a message.** Verified against a live kernel rather than reasoned:
a short read on `SOCK_SEQPACKET` returns the truncated prefix with the flag set
and the remainder discarded, so an unchecked flag turns a long directive into a
silently shortened one, which is the failure the boundary property was elected
to prevent. The same bound governs this crate's sends: no envelope this crate
writes may exceed it, asserted at the write site, because a bound only the
receiver holds is a bound the sender discovers in production. **The election and
its bound are `weaver-types-Spec`'s records and the truncation test is this
crate's,** that Spec owing the test to the pair-creating crates and section 8
naming it, so what lands here is the obligation rather than the decision. **The
obligation grounds where the election does,** in apex section 5.1. Remove the rule
that a seam crossing a process line is a socket and this channel is not a bounded
datagram at all, so a flagged short read is nothing to rule on.

```graph
node: harness-truncation-is-a-fault
kind: assertion
tag: perturbation

edge: asserts
from: weaver-harness
to: harness-truncation-is-a-fault

edge: grounds
from: harness-truncation-is-a-fault
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**One write is one message and one message is one envelope.** The wire carries
`OrganEnvelope` as one JSON document per write, per `weaver-types-Spec` section
4.3, and the socket type is what keeps the framing out of this crate, per
`weaver-organ-channel` section 2. **This is the property the type was elected
to buy, and it takes a test of its own.** It is the boundary half of the pair
test `weaver-types-Spec` section 5 owes the pair-creating crates, whose
truncation half stands above: a substitution of `SOCK_STREAM` at the
`socketpair` call leaves every truncation test in this crate passing,
`MSG_TRUNC` handling being untouched by it, while the framing every contract
that draws these channels rests on is gone. So the consequence of the election
was tested here from the pass and the election's own reason was not, filed as
issue 35 and closed in one act across both pair-creating crates because the
property and its watch are the same on either side. Section 8 names the test
with its watch, and the election stays that Spec's record. **It grounds in the
same invariant as the truncation rule above,** framing being what the socket seam
buys and what every contract drawing these channels reads as given.

```graph
node: harness-one-write-is-one-read
kind: assertion
tag: perturbation

edge: asserts
from: weaver-harness
to: harness-one-write-is-one-read

edge: grounds
from: harness-one-write-is-one-read
to: axiom-floor-is-vocabulary-behavior-is-socket
```

### 2.2 Creation, and the atomic flag

**Both ends of every created pair carry close-on-exec from the creating act,
by `SOCK_CLOEXEC` in the `socketpair` call rather than by a later `fcntl`.**
This crate creates three pairs across a run, the residency and decode pairs in
one act before the SPU fork and the gate pair before the gate fork, and the
rule is the same at each.
`weaver-harness-spu-contract` section 1 requires the harness's own end flagged
from the pair's creation. The atomic form is elected because the alternative
has a window: this process forks a subprocess per tool call, a fork between
creation and a separate `fcntl` would inherit an unflagged end, and an
inherited end of the residency seam hands the tool surface a release directive.
Verified: `socketpair` with `SOCK_CLOEXEC` yields both descriptors flagged with
no interval between them. **The atomicity grounds in apex section 5.1's second
authentication case.** A pair with no name is authenticated by possession of the
descriptor and by nothing else, so an end that crosses an exec is a credential
handed to whatever runs next, and the window a later `fcntl` opens is exactly
where it is handed over.

```graph
node: harness-atomic-cloexec-at-creation
kind: assertion
tag: perturbation

edge: asserts
from: weaver-harness
to: harness-atomic-cloexec-at-creation

edge: grounds
from: harness-atomic-cloexec-at-creation
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**The child's ends cross the final exec by `dup2` from descriptor 3 upward
and an unconditional clear of the flag on each, because the duplicate law has
a corner.**
Close-on-exec would otherwise close the child's own end at the exec that
starts the organ binary. A duplicate made by `dup2` is born with the flag
clear, but only when the two descriptors differ: `dup2` onto the same number
is a no-op that returns the descriptor with its flag untouched, so a child
whose end already sat at descriptor 3 would keep the flag, lose the end at
`execve`, and start the organ with no channel, silently, on whatever layout
the deployment happens to produce. Both halves are verified rather than
recalled, the clear-on-copy law for differing descriptors and the
equal-descriptor no-op that defeats it, each run by both seats. So the child
duplicates each end it is given to a descriptor from 3 upward, the first
after the standard streams, clears the flag on each by `fcntl` whether or not
a duplication moved anything, and execs, so the organ binary finds its ends at
3 and, where it has a second, at 4 from its first instruction. **The order is
the channels' own:** the lifecycle channel every organ holds takes 3, and an
organ's further channel takes the next number, so the gate's single end sits
where the SPU's first does. Until the decoder cut of 2026-08-02 an organ held
one end and this paragraph named one number, and the SPU's second channel is
what made the order need stating. This realizes apex section 12's topology,
the numbering being this Spec's own election. **The numbering is owed to
`weaver-spu-Spec` and `weaver-gate-Spec`**, each of which inherits it rather
than re-deciding it, the same owing shape `weaver-types-Spec` section 4 used
to reach this document, and `weaver-spu-Spec` section 2 states the order from
the receiving side. **The placement and the corner's repair take separate
records.** The second walk's test of section 8 forks a child and enumerates its
descriptors, confirming the ends each organ is owed and where they sit, which
reaches the number and the order, and the equal-descriptor corner is not what
that test varies. So the unconditional clear is review's by election and not by
impossibility: a double placing an end at descriptor 3 before the handoff
produces the corner deterministically, which is the shape both seats ran to find
it, and this suite does not buy it. One record for the pair would claim the
walk's test for a half it does not reach, which is the overclaim this corpus
refuses in prose and has no reason to admit in a graph.

```graph
node: harness-organ-ends-from-descriptor-three
kind: assertion
tag: perturbation

edge: asserts
from: weaver-harness
to: harness-organ-ends-from-descriptor-three

node: harness-child-flag-clear-unconditional
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-child-flag-clear-unconditional
```

**Between fork and exec the child performs three calls, `dup2`, `fcntl`, and
`execve`, and nothing else.** All three are async-signal-safe, and the bound
is stated because the worker holds the writer's thread at every fork: a child
of a multithreaded process may safely run only async-signal-safe calls before
its exec, so the enumeration is the safety argument and not a style. An
earlier draft elected two calls and leaned on the duplicate law without its
corner, and the review caught that the tight bound and the silent corner were
one defect, so the middle call is the corner's repair made unconditional. **The
bound is review's by election,** a seccomp filter or a traced exec reaching the
call list between fork and exec, and this suite buying the walks of section 8
instead.

```graph
node: harness-fork-to-exec-three-calls
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-fork-to-exec-three-calls
```

### 2.3 The receive site, and adoption

**The trace descriptor enters this crate at exactly one call, and that call
asks for the flag itself.** The trace descriptor crosses once, as ancillary data on
the enter directive's own message, per `weaver-admin-harness-contract` section
3, so the one `recvmsg` site on the coordination channel carries
`MSG_CMSG_CLOEXEC`. Verified both ways: a descriptor received with the flag
arrives close-on-exec, and one received without it arrives clear, which is the
window `weaver-organ-channel` section 2 describes and the reason the obligation
is the receiver's. The site takes no flag argument and returns owned handles
the rest of the crate cannot construct another way, which is the pinned shape
`weaver-admin-harness-contract` section 5 names. This is the claim
`weaver-trace-Spec` section 10 owes this document, its test standing as the
first walk of section 8 and the owing discharged there. **It grounds in apex
section 5.2 rather than in the socket invariant.** What crosses this receive is
the sink handle and not a channel end, so possession authenticates nothing here.
A tool subprocess holding it writes the record without being the harness, which
is the sole-authorship half of 5.2 defeated by a descriptor, and a second writer
does not merely add events, it adds events belonging to no turn the harness can
attribute them to.

```graph
node: harness-trace-fd-cloexec-at-receive
kind: assertion
tag: perturbation

edge: asserts
from: weaver-harness
to: harness-trace-fd-cloexec-at-receive

edge: grounds
from: harness-trace-fd-cloexec-at-receive
to: axiom-join-key-travels-with-the-work
```

**The coordination socket is this crate's to create, and binding it is the
worker's first act.** Per the inversion ruling of 2026-08-05 and
`weaver-admin-harness-contract` section 2, any socket connecting to the harness
is an internal connection: the composition root creates a `SOCK_SEQPACKET`
socket with close-on-exec in the creating call, binds it to the per-agent name
inside the unit's own runtime directory, and listens, before any directive can
arrive.

**The bind never unlinks, and the runtime directory is why it does not have to.**
A Unix socket's pathname outlives the process that bound it, so a bind against a
name a dead worker left would fail. The directory this socket lives in is created
by the init system at the unit's start and removed with the unit, per
`weaver-admin-systemd-contract` sections 2 and 5, so the name cannot be inherited
from a previous run and there is nothing to clear. **A bind that finds its name
occupied is a fault and never a thing to remove**, because the only ways a name is
occupied are that a live worker holds it, in which case unlinking would strand the
running agent's supervisor, or that the manager did not honor the directory, in
which case the program's assumption is wrong and it should say so rather than
repair. The instrument is review, no test in this crate being able to produce a
manager that misbehaves.

```graph
node: harness-bind-never-unlinks
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-bind-never-unlinks

edge: grounds
from: harness-bind-never-unlinks
to: axiom-floor-is-vocabulary-behavior-is-socket
``` It runs before the serving loop
because an admin invocation dials immediately after starting the unit and a
name not yet bound is the race the ordering exists to prevent, admin's bounded
retry covering what remains. `Harness::adopt` becomes `Harness::listen`, taking
the bound listener rather than a handed end, and the earlier declared-open route
retires with the party that placed it.

```graph
node: harness-binds-coordination-socket-first
kind: assertion
tag: perturbation

edge: asserts
from: weaver-harness
to: harness-binds-coordination-socket-first

edge: grounds
from: harness-binds-coordination-socket-first
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**Every accept reads the peer credential and refuses what is not root, before
any byte.** `accept4` carries close-on-exec in the accepting call, `SO_PEERCRED`
yields the dialing peer's uid, and a uid other than root closes the connection
unanswered. This is the check that makes the inversion worth its churn: the
socket is reachable from inside the sandbox by construction, so an elected tool
at the agent uid can dial it, and what refuses that tool is that it is not root.
The earlier design expected the agent's own uid at this check, which every tool
of the agent's satisfies, and leaned on a listener closed after one accept to do
the refusing. **The listener is not closed after one accept**, because admin is
per-invocation and every later verb dials again, so the closure is retired and
the check carries the property alone. One connection is served at a time, which
is what holds the contract's one-exchange-in-flight rule now that no fleet map
does.

```graph
node: harness-coordination-accept-refuses-non-root
kind: assertion
tag: perturbation

edge: asserts
from: weaver-harness
to: harness-coordination-accept-refuses-non-root

edge: grounds
from: harness-coordination-accept-refuses-non-root
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**The worker's hygiene is performed as sets and not checks, and it survives the
inversion unchanged in substance.** The composition root clears the process's
dumpable flag, the attach defense `weaver-admin-harness-contract` section 2
states, and every descriptor this crate creates or accepts carries close-on-exec
from its creating call. Both are sets because a check that finds the flag wrong
and reports leaves the descriptor inheritable and the process attachable, which
is the set-not-check rule stated at the contract and applied here. What the
inversion removed is the set-again-after-the-last-exec ordering: the listener and
every accepted connection are created after the worker's last exec, so no handed
end exists whose flag an exec could have cleared. **The dumpable flag grounds in
apex section 5.1 and it is the less obvious of the two.** That invariant rests
possession-as-authentication on the claim that no third party can reach a socket
with no address, and `/proc/[pid]/fd` is an address for exactly such a socket, so
clearing the flag is what closes the one route the invariant's own argument
assumes shut, and it reaches the organ pairs whether or not the coordination
channel has a name.

```graph
node: harness-dumpable-flag-cleared
kind: assertion
tag: perturbation

edge: asserts
from: weaver-harness
to: harness-dumpable-flag-cleared

edge: grounds
from: harness-dumpable-flag-cleared
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**No path is taken anywhere in this crate.** There is no call that resolves,
opens, or stores a filesystem path to the trace, per `weaver-harness-PRD`
section 5, and the organ binaries of section 3 are the one exception, supplied
by the composition root as a construction parameter the way `weaver-trace-Spec`
section 6 takes its queue depth: a deployment fact, not an operator election
and not a discovery. **The three named shapes are pinned by the compile-fail
doctests of section 8, and the general prohibition stays review's,** three
doctests reaching the shapes they name and not the open set of every way a path
becomes a call argument. The pinning and the prohibition are two records for
that reason, per section 8, and neither claims the other's instrument.

```graph
node: harness-no-path-taken
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-no-path-taken
```

### 2.4 The OS surface, elected

**The OS crate is `nix`, and the ground is that descriptor custody is this
crate's central obligation.** The mechanics above are `socketpair`, `recvmsg`
with control messages, `fcntl`, `fork`, `dup2`, `execve`, and the dumpable
`prctl`, and `nix` covers that surface over the standard library's owned
descriptor types, so a descriptor's ownership is a compile property and a leak
is a type error rather than an integer left behind. Raw `libc` is the old
tree's answer and puts unsafe integer descriptors at the exact seam whose
custody discipline this crate exists to hold. `rustix` holds the same io-safety
posture and declines to offer process forking as a supported surface, and the
fork is not optional here: the topology of apex section 12 has this crate
forking both organ binaries. One crate covering the whole surface beats two
crates covering it between them, on the floor's own thinness doctrine. **The
election is a manifest property and the record is this crate's,**
`weaver-admin-Spec` section 1, `weaver-gate-Spec` section 1, and
`weaver-spu-Spec` section 1 each inheriting these grounds rather than re-arguing
them: the manifest carries one OS crate and no `libc` or `rustix` line of this
crate's own. The ownership the election buys is the other claim in this
paragraph and it is a type property, so it takes the compiler under section 8's
first sorting rather than the manifest read.

```graph
node: harness-os-surface-nix
kind: assertion
tag: manifest

edge: asserts
from: weaver-harness
to: harness-os-surface-nix

node: harness-descriptors-owned-types
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-harness
to: harness-descriptors-owned-types
```

## 3. The lifecycle interior

The harness type, the run state, and the fan-out of loop 0, per
`weaver-admin-harness-contract` section 3 and the composition `load-unload-path`
reads back.

```rust
pub struct Harness { /* private */ }

pub struct OrganBinaries {
    pub spu: PathBuf,
    pub gate: PathBuf,
}

impl Harness {
    /// The crate's one constructor. Takes the coordination listener this
    /// crate bound and performs the hygiene of section 2.3.
    pub fn listen(coordination: OwnedFd, organs: OrganBinaries)
        -> Result<Self, AdoptionFault>

    /// Serves the coordination channel until leave is answered or closure
    /// is observed, or fails on a fault below the exchange layer.
    pub fn serve(self) -> Result<Outcome, ChannelFault>
}
```

**Construction fails only when a set fails, and a failed set refuses
construction.** A hygiene call that errors leaves the worker attachable or the
end inheritable, so `listen` returns the fault naming the set rather than
proceeding unset, and a `Harness` in hand means the hygiene held. The fault's
shape is a satellite of section 9. **The one constructor is the compile-fail
doctest of section 8 and the refusal is review's by election,** an absence being
what a runtime test structurally cannot demonstrate, and a test failing a hygiene
call on a closed descriptor reaching the refusal that this suite does not buy.

```graph
node: harness-one-constructor
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-harness
to: harness-one-constructor

node: harness-failed-set-refuses-construction
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-failed-set-refuses-construction
```

**The service is a serial loop and the channel state is a type.** One directive
at a time arrives, is judged against the channel's state, and is answered or
refused, per the ordering rules of `weaver-admin-harness-contract` section 4. A
directive out of order for the state answers `OutOfOrder` and is not queued.
The state has three positions, before enter, entered, and left, the last
terminal, and the middle one carries the run. **The positions and the refusal
take two records and two instruments,** the split `weaver-gate-Spec` section 6
takes on the same channel state. The positions are a type and take the
compiler, which pins that a directive out of order for the state reaches a
match arm rather than a flag check. What the arm then does is a behaviour and
takes a perturbation, an arm being free to queue or to answer the wrong refusal
while compiling exactly as well, so the pin alone would leave the refusal
claimed and unenforced. The refusal is owed to each organ by
`weaver-types-Spec` section 5, which states it and enforces it nowhere, and
this discharges this crate's side of that owing alone, section 8 naming the
test with its two watches. **Both halves ground in apex section 5.3.** An
ordering guarantee is one of the three things that invariant requires of a
contract, so the coordination channel's order is a stated guarantee rather than
a convention this crate inherited, and a state type holding it is that guarantee
made structural. The pin reaches the completeness half, every out-of-order case
arriving at an arm, and the perturbation reaches what the arm returns, which is
the contract's own named error. Grounding one and not the other would say the
invariant is a reason for the type and not for the behaviour, which is backwards.

```graph
node: harness-channel-state-three-positions
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-harness
to: harness-channel-state-three-positions

edge: grounds
from: harness-channel-state-three-positions
to: axiom-contract-is-a-complete-interface

node: harness-out-of-order-refused
kind: assertion
tag: perturbation

edge: asserts
from: weaver-harness
to: harness-out-of-order-refused

edge: grounds
from: harness-out-of-order-refused
to: axiom-contract-is-a-complete-interface
```

**The run state is the fan-out's progress held as data, which is what makes the
unwind total.**

```rust
struct Run {
    recorder: Recorder,
    spu: Option<SpuChannels>,
    gate: Option<OrganChannel>,
    turn_in_flight: bool,
}

struct SpuChannels {
    lifecycle: OrganChannel,
    decode: DecodeChannel,
}
```

Each `Option` is an arm of the enter fan-out that has or has not stood up, so a
leave arriving after a refused enter unwinds exactly what stands, stopping the
gate where a gate was raised and releasing the SPU where a model was admitted,
and the compiler's match on the options is what makes a forgotten arm
unrepresentable rather than unlikely. **The SPU's arm is a pair of channels
rather than one, and they are one field because they stand up and fall
together.** The decoder cut of 2026-08-02 gave that organ a second socket, and
the two are created in one act and cross one fork, per
`weaver-harness-spu-decode-contract` section 1, so an option over the pair
keeps the arm's all-or-nothing shape where two options would admit a half-stood
arm the unwind would have to reason about. The decode end takes its own type
rather than `OrganChannel`, because `weaver-spu-PRD` section 13.2 rules that
socket not an organ channel and a shared name would carry the envelope's
assumptions onto a seam that does not take them. This is the mechanical form of
`load-unload-path` section 4's rule that admin's unwind is a reap plus one
directive: the directive works because the harness knows what stands. **The options
ground in apex section 5.5 and the two SPU-shaped records beside them do not.** A
leave has to undo across domains what an enter built across them, and no organ can
see whether the other stood up, so either the loop holds that knowledge or the unwind
is a guess about a domain nobody is looking at. That is the reconciliation 5.5 places
with the loop by construction rather than by convention, and the run state is the
form the knowledge takes. The SPU's arm being one field and the decode end taking its
own type are shapes inside one organ's arm, which that section leaves to the domain.

```graph
node: harness-run-state-options-checked-unwind
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-harness
to: harness-run-state-options-checked-unwind

edge: grounds
from: harness-run-state-options-checked-unwind
to: axiom-harness-integrates-by-the-loop

node: harness-spu-channels-one-field
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-harness
to: harness-spu-channels-one-field

node: harness-decode-end-own-type
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-harness
to: harness-decode-end-own-type
```

**Enter runs four steps in the charter's order, and the answer is the
aggregate.** Receive the sink descriptor from the directive's own message and
construct the `Recorder`, which stands the empty working structure up. The
count is one, per the human's ruling of 2026-08-02: the coordination
contract's plural was residue of the retired live view, corrected to the
singular in the same act as this sentence, and `weaver-trace-Spec` section
5's receive takes the one sink descriptor this Spec builds to. Author
the `load` event, the run's opening and the origin of its monotonic clock.
Create the residency pair and the decode pair in one act, per
`weaver-harness-spu-decode-contract` section 1, fork the SPU binary carrying
both ends, and open the admit exchange on the lifecycle pair carrying the
model binding uninterpreted. The decode socket is created here rather than at
first use because it crosses the same fork, and a socket the child was not
given at its exec cannot be handed to it afterward. Create the gate pair only after the
SPU's answer has confirmed residency, per `weaver-harness-gate-contract`
section 1, then fork the gate binary and open the raise exchange carrying the
gate instruction uninterpreted, the gate last so no work arrives before the
interior serves, per apex section 6. The wait is what the run state's
invariant rests on: `gate` is set only ever after a confirmed `spu`, so the
unwind's reverse order is a property of construction order rather than of
timing. Ready is answered when the last arm confirms. A refusing
arm's reason is wrapped `OrganRefused` and carried into the aggregate
unchanged, per `weaver-admin-harness-contract` section 6, and the scoped
account holds: a refusal before the `load` event leaves the stream clean and
the state at before-enter, and a refusal after it leaves the authored bracket
standing and the run in place for the leave that unwinds it. **Three of this
paragraph's four records are review's by election.** A test counting the
ancillary descriptors on an enter directive reaches the sink's count, a test
exec'ing a probe in the SPU's place reaches the decode pair's crossing of the
same fork, and a test holding a double SPU's answer reaches the gate pair's wait
on confirmed residency, and this suite buys none of the three. The scoped
account is the fourth and section 8 names its test. **Two of the four ground in
an invariant and two do not.** The decode pair's creation before the fork is
apex section 5.1's second authentication case read as a construction rule: a
socket with no address cannot be reached later by resolving one, so the only
moment it can reach a child is before that child exists, and the sentence above
about a socket the child was not given is that invariant and not a POSIX
accident. **The gate pair's wait grounds in apex section 5.5.** One organ's
confirmed readiness gates another organ's construction, which is two organs'
orderings reconciled against each other, and neither can perform that reconciliation
because neither can see the other's domain. The reverse unwind order the paragraph
below opens with is the same fact read backwards and is argued here rather than
there. The sink's count and the scoped account are a count and an ordering of this
crate's interior, which no invariant is about.

```graph
node: harness-one-sink-descriptor
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-one-sink-descriptor

node: harness-decode-pair-created-before-the-fork
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-decode-pair-created-before-the-fork

edge: grounds
from: harness-decode-pair-created-before-the-fork
to: axiom-floor-is-vocabulary-behavior-is-socket

node: harness-gate-pair-waits-on-residency
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-gate-pair-waits-on-residency

edge: grounds
from: harness-gate-pair-waits-on-residency
to: axiom-harness-integrates-by-the-loop

node: harness-scoped-refusal-account
kind: assertion
tag: perturbation

edge: asserts
from: weaver-harness
to: harness-scoped-refusal-account
```

**Leave runs the reverse order and drains before it answers.** Lower the gate
first, refuse `ActivityNotAtRest` while a turn is in flight, author the
`unload` event, drain the writer's queue, and release the SPU. Left is
answered only after the drain returns, which is what makes the answer mean
what `weaver-admin-harness-contract` section 4 says it means, that everything
admitted reached the stream. **The ordering is review's by election,** a double
sink that drains slowly reaching it, which is the shape the gate's
ready-follows-bind test takes for its own ordering, and this suite not buying
one.

```graph
node: harness-left-follows-drain
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-left-follows-drain
```

**Stop answers after the record holds the close.** The stop directive aborts
the turn in flight, the turn's close event is placed with the stop reason, and
only then does the answer carry `TurnAborted`, the announce-after-record
discipline of `weaver-admin-harness-contract` section 3. A stop at rest
answers `AtRest`, a clean close and not a refusal. How the abort lands at the
decoder is deferred with the decode seam, per section 8, and the trace
semantics are settled either way, which is what `basic-inference-loop` section
7 already records.

```graph
node: harness-announce-after-record
kind: assertion
tag: perturbation

edge: asserts
from: weaver-harness
to: harness-announce-after-record
```

**A fault the worker survives is authored, not signalled.** The pressure and
failure reports the recorder surfaces, and the organ deaths observed through
closure after the enter aggregate, reach the operator as the `fault` event on
the stream, per the fault-carrier ruling of 2026-08-01. No run blocks on
anything downstream of the emission. The payload is the floor's
`fault-report`, carried unchanged from the reporting organ and authored
without translation, per `weaver-harness-trace-contract` section 3, and this
crate's own three sources are enumerated at `weaver-harness-PRD` section 5.
The gating an earlier draft of this Spec described lifted when that shape
landed on 2026-08-02. **The two records this paragraph carries are review's by
election,** the never-blocks sentence being section 4's. A test driving a double
organ's fault report to the stream reaches the authoring and the payload alike,
comparing the reported bytes against the authored ones, and this suite buys the
trace side of that pair instead, where `weaver-trace-Spec` section 6 asserts
that the recorder reports pressure and does not author it. **The authoring
grounds in apex section 5.2 and the payload does not.** That invariant's
sole-writer half is this paragraph's rule stated at the apex, a component
reports and the harness authors the event, and a fault an organ signalled for
itself would be a record entry no turn key attributes to anything. What the
event then carries is trace content, and content is where the invariants stop.

```graph
node: harness-faults-authored-as-events
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-faults-authored-as-events

edge: grounds
from: harness-faults-authored-as-events
to: axiom-join-key-travels-with-the-work

node: harness-fault-payload-carried-unchanged
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-fault-payload-carried-unchanged
```

**Loop 0 takes neither a type nor a trait, and the cell closes here.**
`load-unload-path` section 8 holds the question for the Spec pass,
demand-derived rather than reserved. The demand does not exist: the loop is
the interval between two directives, its state is the `Run` struct above, and
its control flow is the serial service, so an abstraction would have no second
implementor and no caller that varies, which is the reserved slot apex section
9 forbids. The inference loop inside it may yet demand one, and that demand
arrives with the token workflow, which may reopen this with the engine's shape
in hand. **The absence is review's by election,** the named-candidate doctest
the floor Specs buy for absences reaching a trait this Spec could name, and the
claim here being that no demand exists rather than that one named item is
missing, which is the open set a doctest does not close.

```graph
node: harness-loop-zero-takes-no-abstraction
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-loop-zero-takes-no-abstraction
```

## 4. Trace authorship

The authoring half of `weaver-harness-trace-contract`, landed as one module
with one submit path.

**Identity converts at the submit call, and nowhere else.** The envelope
identifies the session, run, and turn as `weaver-trace`'s opaque newtypes, and
this crate holds the floor's `SessionId` and `TurnKey`, so the conversion the
no-dependency rule of `weaver-trace-Spec` section 1 forces is a total function
at the one call site that submits, exactly as that Spec places it, which is why
the record for the conversion sits there and not here: one claim is one node, and
that Spec elects the newtypes and places the conversion in the same clause.

**Both timestamps are stamped at authoring, from the standard library's two
clocks.** Wall-clock milliseconds from the system clock, and the monotonic
reading as nanoseconds elapsed since the run's origin, an instant captured
when the `load` event is authored, per `weaver-harness-trace-contract` section
3. No OS crate is needed for either, and the recorder's clock is never
consulted because the contract denies it the fields. **The stamping site is
review's by election,** a double recorder comparing an event's monotonic reading
against the run's origin reaching it, and this suite buying the trace side's
canonical-form tests instead. **The site grounds in apex section 5.2 and the
choice of clocks does not.** The monotonic reading is nanoseconds since the run's
origin, an origin only the author holds, so a component stamping its own report
would carry a reading placeable in no run, which is the attribution that
invariant's sole-writer half exists to buy. Which two clocks supply the numbers
is representation and would read the same under any invariant.

```graph
node: harness-timestamps-stamped-at-authoring
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-timestamps-stamped-at-authoring

edge: grounds
from: harness-timestamps-stamped-at-authoring
to: axiom-join-key-travels-with-the-work
```

**The licensed combinations are enforced here, before submit.** A message is
judged against the licensing rule of `weaver-traits-Spec` section 3, a `User`
message carrying only `Text`, an `Assistant` message carrying `Text` and
`ToolCall`, a `ToolResult` message carrying only `ToolResult` blocks, and an
unlicensed message is refused by this crate and never submitted. The recorder
cannot hold this rule, per that Spec, so the harness is the party the
perturbation test of `weaver-traits-Spec` section 7 binds, and the test lands
in this document's section 8 set. That Spec names this document as the declaring
side, and the record below discharges the owing.

```graph
node: harness-licensed-combinations-refused-before-submit
kind: assertion
tag: perturbation

edge: asserts
from: weaver-harness
to: harness-licensed-combinations-refused-before-submit
```

**A refused submission is handled as the contract orders.** It is not treated
as recorded, not projected, and not retried under a new sequence, per
`weaver-harness-trace-contract` section 3. A refusal on the authoring path is
a defect in the author, and it surfaces as a fault rather than a retry. **The
handling is review's by election,** a double recorder refusing one submission
reaching it and the trace side's gapless-sequence test watching the same
property from the recorder's end, which is the test this program buys.

```graph
node: harness-refused-submission-not-retried
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-refused-submission-not-retried
```

**Pressure becomes an event, authored by this crate.** When the recorder
surfaces `CommitPressure`, the harness authors the `fault` event in response,
per `weaver-trace-Spec` section 6, carrying the floor's `fault-report` as
section 3 states.
Nothing on any turn path waits on the sink, per `weaver-harness-PRD` section
5, and the working structure's return is the acknowledgment the interior
proceeds on. The authoring itself is section 3's record and is not restated
here. **The never-waits property is review's by election,** a double sink that
never drains reaching it while a turn proceeds, and this suite buying the
back-pressure measurement section 9 defers instead.

```graph
node: harness-nothing-waits-on-the-sink
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-nothing-waits-on-the-sink
```

## 5. Prompt assembly's deterministic floor

**Assembly reads the message kinds and nothing else, and the discipline is a
kind filter at one site.** The harness assembles a prompt by iterating the
working structure in sequence order, selecting on the lifted `kind` the index
of `weaver-trace-Spec` section 4 holds for exactly this read, taking the three
message kinds, and decoding their payloads. The measurement, lifecycle, and
custody events never enter a prompt because the assembly path cannot see them:
the filter is the kind set at the read site, not a judgment applied after a
full read. This is
the seam `weaver-harness-PRD` section 2 names as what a later recall feature
would breach, held today by one match a reviewer can find and by the fourth
walk's test of section 8.

```graph
node: harness-assembly-kind-filter-at-read-site
kind: assertion
tag: perturbation

edge: asserts
from: weaver-harness
to: harness-assembly-kind-filter-at-read-site
```

**The per-model layer is deferred with the token workflow, and the floor
beneath it is fixed.** What phrasing elicits a tool call from a given decoder
is re-erected per model, per the charter, and no model is a dependency of this
crate. What does not vary and is fixed now is the order of parts, the identity
prefix, then the message sequence, then the tool schemas, per apex section 3
step 4, and the property that assembly is deterministic over the working
structure's contents: the same records assemble the same prompt, byte for
byte, which is what makes a replayed run's prompts comparable at all. **The two
claims take two instruments.** Determinism is section 8's perturbation bullet,
and the order of parts is review's by election, a test reading the three parts
out of an assembled prompt reaching it and this suite buying the byte-for-byte
comparison that holds whatever order landed.

```graph
node: harness-prompt-part-order
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-prompt-part-order

node: harness-deterministic-assembly
kind: assertion
tag: perturbation

edge: asserts
from: weaver-harness
to: harness-deterministic-assembly
```

**The provider is injected, and this crate names no wire format.** Decode
requests leave through `provider-trait`, constructed at the worker composition
root, per `weaver-traits-Spec` section 6, and every deferred decode shape in
this document defers to the same place that trait's signature does, the token
workflow. **The absence of a wire format here is review's by election,** a
manifest read reaching a named HTTP or provider client and section 8's own
manifest bullet buying exactly that, while what this clause claims beyond it is
that no format is named in source either, which the manifest does not see and a
grep at review does.

```graph
node: harness-provider-injected-no-wire-format
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-provider-injected-no-wire-format
```

## 6. The tool system, blocked, and loop 1's seat

**Tool dispatch is blocked, and this Spec obeys the block.** `tool-trait` is
held by `weaver-traits-PRD` section 3.1 until the tool workflow, so
`src/tools.rs` is a placement, and what is stated now is only what the charter
already fixes: permission modes are consultation policy and not a boundary,
the kernel bounds what a tool reaches through the uid it runs as, and no
safety classification exists here or is coming, per `weaver-harness-PRD`
section 3. The tool subprocess inherits no descriptor this program holds,
which section 2 delivers by construction rather than by a per-tool argument and
asserts there rather than again here. The absent safety classification is
`weaver-traits-Spec` section 5's record, that trait being where the absence is
pinned, and this clause obeys it rather than restating it.

**`src/engine.rs` is loop 1's seat, and the seam it composes across is this
crate's public surface.** The loop itself is the builder's, per the charter's
rescope of 2026-08-02: written at the worker composition root, compiled into
the worker binary, and immutable there, which binary the unit starts being a
provisioning fact. What this crate holds is the seat and the granted surface
the loop composes against, which is the whole of sections 2 through 5, the
channels and their custody, the run state, trace authorship, and assembly's
deterministic floor. **The extension seam is crossed at loaded-and-idle
itself,** the name being the charter's per its sections 2 and 6: loop 0
hands a standing interior to whatever loop 1 the binary carries, and takes it
back at the stop and at the leave, the bracket discipline being loop 0's for
every loop alike. A loop that composes what this surface offers costs nothing
anywhere else, and a loop that needs a port this surface does not offer is a
capability change entering through the front door as a charter and contract
edit. The compiled form enforces that blade structurally: there is no call by
which a loop mints a port, because the ports are types these crates own. The
decode surface a real loop needs, the exchanges, sessions, sampling, and the
flush call, arrives with the token workflow per `weaver-spu-PRD` section 8,
the basic loops this program ships land in their binaries by the same path as
any builder's, and the executor election stays deferred with that workflow,
per section 1. **The blade takes the compiler and the seam's position takes
review.** A port is a type this crate owns with a constructor no consumer can
reach, so a loop composes against the granted surface or does not compile, which
is the same type property section 8's sorting holds for descriptor ownership,
elected here rather than listed there. Where the seam falls
is an election a test could read only by asserting what a loop is handed, which
this suite does not buy. **The blade grounds in two invariants and the seam's
position grounds in neither.** Apex section 5.1 admits no exception for crates
arriving later, and a loop 1 that could mint a port would be a later arrival reaching
another process by a route no contract governs. The blade is that clause held at
the type level against code the builder writes rather than against a crate this
program ships. **It grounds in apex section 5.5 as well, and for the second reason
rather than a restatement of the first.** That section makes the loop the mechanism
that integrates and holds that the mechanism cannot itself be a part, so a builder's
loop minting its own port would be a part doing its own integrating, reaching a
domain by a route the loop never granted and settling a crossing in code no contract
sees. The blade is what leaves loop 1 composing the granted surface, which is where
5.5 puts the crossing. Where in the lifecycle the seam falls is a charter election
that would read the same under any invariant.

```graph
node: harness-loop-mints-no-port
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-harness
to: harness-loop-mints-no-port

edge: grounds
from: harness-loop-mints-no-port
to: axiom-floor-is-vocabulary-behavior-is-socket

edge: grounds
from: harness-loop-mints-no-port
to: axiom-harness-integrates-by-the-loop

node: harness-extension-seam-at-loaded-and-idle
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-extension-seam-at-loaded-and-idle
```

## 7. The failure vocabulary

```rust
pub enum ChannelFault {
    Truncated { bound: usize },
    Undecodable,
    Closed,
}

pub enum Outcome {
    Left,
    ChannelClosed,
}
```

**Three faults, and the split from refusals is principled.** A refusal is a
typed answer on an exchange, and every refusal this crate issues or carries is
a `lifecycle-refusal`, the floor's type, drawn and never twinned. A channel
fault is a failure below the exchange layer: a truncated read, octets that do
not decode to an envelope and so cannot be attributed to any exchange for a
refusal to answer, and closure, which `weaver-organ-channel` section 2 rules
is observed as death and never synthesized into an answer. `Outcome` names the
two ways service ends, the leave answered or the far process gone, so the
composition root branches on a value rather than a guess. **The split is
review's by election and the outcome enum is a type property.** A test can watch
a refusal answer where a fault should have ended the service, which is the reach
this suite declines while `weaver-gate-Spec` section 5 and `weaver-spu-Spec`
section 9 adopt the same split from their own sides, and the enum's two cases
are exhaustive so a third reaches every caller loudly. **The split grounds in
apex section 5.3 and the outcome enum does not.** A contract naming the errors
it can return is what makes its refusal set closed, so a failure below the
exchange layer sits outside that set by construction rather than by taste, and
dressing one as a refusal would put a case on the wire the vocabulary clause
never drew. `Outcome` crosses no seam and is drawn by no contract, and its
exhaustiveness serves this crate's own caller.

```graph
node: harness-fault-below-the-exchange-layer
kind: assertion
tag: review

edge: asserts
from: weaver-harness
to: harness-fault-below-the-exchange-layer

edge: grounds
from: harness-fault-below-the-exchange-layer
to: axiom-contract-is-a-complete-interface

node: harness-outcome-two-cases
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-harness
to: harness-outcome-two-cases
```

## 8. What is enforced, and by which instrument

Per apex section 11. The threat walks are stated with their mechanisms, per
the threat-anchoring rule of the external review of 2026-08-01, and each test
below is a walk made executable.

**The reference walk: a rogue elected tool reaches for the trace.** The
adversary is the agent's own tool surface, a subprocess this crate forks per
call, running as the agent uid with `bash` as the reference case. The attack
is inheritance: without close-on-exec every tool subprocess holds a writable
handle to the trace and edits its own account through a descriptor requiring
no path and passing no check. The mechanism is the one receive site of
section 2.3 asking for `MSG_CMSG_CLOEXEC` in the receive itself. The test
forks a child and confirms it holds no trace handle, watched to fail when the
flag is removed from the receive, which is the test `weaver-trace-Spec`
section 10 owed this document, landed.

**The second walk: the same tool inherits a channel end.** The attack is an
inherited end handing the tool surface a capability that varies by which end
it got: a residency or gate end carries a lifecycle directive, a release
above all, and a decode end carries a resident session's traffic, which is a
different capability rather than a smaller one. The mechanism is the atomic
`SOCK_CLOEXEC` of section 2.2, no window between creation and flag. The test
forks a child and enumerates its descriptors, confirming that each organ fork
carries exactly the ends that organ is owed, two at the SPU's and one at the
gate's, and that nothing crosses a tool fork, watched to fail when the atomic
flag is dropped to a later `fcntl`.

**The third walk: a same-uid process attaches.** The attack reads or drives
the worker's descriptors through `ptrace` or `/proc/[pid]/fd`, no tool fork
required. The mechanism is the dumpable flag cleared at adoption, per
`weaver-admin-harness-contract` section 2, which reparents the proc entries
to root and refuses the attach. The test confirms the flag is clear after
`adopt` and clear again in a forked child's parent, watched to fail when the
clearing is removed.

**The fourth walk: the model reaches its own record through the prompt.** The
adversary is the model eliciting its measurement and custody events into
context, the recall-shaped breach `weaver-harness-PRD` section 2 names. The
mechanism is section 5's kind filter at the read site. The test assembles a
prompt from a structure holding measurement and lifecycle events and confirms
none of their content appears, watched to fail when the filter widens.

**Enforced by the compiler.**

- The run state's options make the partial fan-out representable and the
  unwind a checked match, so a forgotten arm is a compile error in the leave
  path rather than a leaked residency.
- The floor's three wire enums are exhaustive, so every directive, answer, and
  refusal case added later reaches this crate's matches loudly.
- Descriptors are owned types end to end, so a handle that escapes its owner
  is a move the borrow checker sees, not an integer copied silently.

**Enforced by compile-fail tests, because the property is an absence.**

- One constructor: code constructing a `Harness` other than through `adopt`
  fails to compile, the fields being private and no second path existing.
- No path-taking surface: doctests handing `channel.rs` a `&str`, a `String`,
  and a `PathBuf` where it takes owned descriptors each fail to compile,
  three named shapes with the general prohibition staying review's, per the
  split the floor Specs make. **The split is two assertions rather than one,**
  the pinned shapes here and the prohibition itself at the section 2.3 clause
  that argues it: a single record tagged for the mechanical half would claim
  the doctests for the whole, which is the overclaim this corpus refuses in
  prose and has no reason to admit in a graph.

**Enforced by the manifest.** The internal dependency set is exactly the two
floor links and the trace seam, read against the graph under gate H2. No async
runtime, no logging crate, and no HTTP client in the resolved external tree,
by the build-time `cargo tree` assertion the floor Specs share.

**Which invariant each claim serves, and why most serve none.** Nineteen of the
forty-eight carry a `grounds` edge and one of the nineteen carries two, so the edges
number twenty: ten to `axiom-floor-is-vocabulary-behavior-is-socket`, four to
`axiom-harness-integrates-by-the-loop`, three to
`axiom-join-key-travels-with-the-work`, and three to
`axiom-contract-is-a-complete-interface`. **`axiom-organ-and-submodule` takes
nothing from this crate,** which reads oddly for the hub that invariant names and
is right: the hub property is topology, which crate holds a duplex channel with
which, and this document sources no topological record. The charter sources this
crate's node, its parent edge, its floor links, and its one declared seam, per
section 0, and everything below that line is this crate's interior or its
channels' mechanics. A crate is an organ or is not by a test read against the
apex, and no claim here is that reading. **That 5.4 still takes nothing while 5.5
takes four is the two sections' own division working rather than an inconsistency.**
5.4 fixes the topology and the charter is where the topology is stated, so a Spec has
nothing to add to it. 5.5 is the work the topology exists for, and a Spec saying how
the loop stands the organs up and takes them down is arguing exactly that work.

**The test applied is whether the axiom is the reason the claim exists, or whether
the claim is a precondition of the axiom's own stated reason,** the second relation
per Document Format section 4. Remove the socket invariant and this crate has
no reason to bound a receive, to demand that one write arrive as one read, to flag a
descriptor before an exec, to create a pair before a fork, or to keep a loop from
minting a port, so those ground in it. Remove it and `nix` is still the OS crate,
descriptors are still owned types, the child's ends still land at 3 and 4, and the
fork still runs three calls, so those ground in nothing. **Twenty-nine claims
grounding in no invariant is the expected result and not a gap**, per Document
Format section 4: most of what a Spec elects is a format, a name shape, a count, or
an ordering of its own interior, and representation is what the invariants are not
about.

**Four calls are worth stating rather than left to be read.** The descriptor
numbering is ungrounded on the apex's own words, 5.1 leaving how a far end
travels to the contract governing that seam, so the number and the order are
contract material while the flag on the end is the invariant's. The dumpable
flag is grounded where a process-hardening set might have been passed over,
because 5.1 rests possession-as-authentication on no third party reaching an
unaddressed socket and `/proc/[pid]/fd` is that address. The authorship claims
of sections 2.3, 3, and 4 ground in 5.2 while their neighbours do not, who
stamps and who authors being the sole-writer half and what an event carries
being content. And conformance to one contract clause draws no edge, or every
citation in this document would draw one: what 5.3 grounds is completeness, so
the channel state's totality carries an edge while left-follows-drain and
announce-after-record do not.

**The fifth invariant reaches four claims here, and the scope limit is what keeps
it to four.** 5.5 binds what crosses between domains and leaves what happens inside
one to the organ, so a claim reaches it only where the reason cannot be stated
without naming a second organ's ordering. The gate pair's wait on confirmed
residency is the plainest, one organ's readiness gating another's construction. The
run state's options are the same fact held as data, the loop knowing across domains
what it must later undo across them. The thread the organ forks run on is a timing
property the gate relies on and cannot hold from where it sits. The port blade is
the fourth and takes a second edge rather than a moved one. **Three near misses are
worth naming, because the crate this invariant names is where a wide reading would
do the most damage.** Left-follows-drain reconciles the drain against admin's
answer and the drain is this crate's own domain, so one organ is named and not two,
and the reverse unwind order its paragraph opens with is argued at the gate pair's
wait instead. Announce-after-record is that same shape at the stop. The descriptor
numbering stays declined for the reason the labelling pass gave, 5.1 delegating how
a far end travels to the contract governing the seam, and 5.5 does not take it back:
where an organ finds its end is contract material the organ presents rather than an
ordering the loop reconciles.

**Where the assertion records sit, and which of these bullets another crate
declares.** The records are at the clauses that argue the claims, across
sections 1 through 7, rather than gathered here, per Document Format section 6:
this section sorts by instrument and the arguments are elsewhere, so a block
here would sit apart from the prose that earns it. One record is the exception
and sits at the end of this section, the doctest pinning of the three
path-taking shapes, whose argument is nowhere else and whose general
prohibition is section 2.3's. Forty-eight records in all as of the inversion of
2026-08-05, which retired the adopted end's close-on-exec, that end no longer
existing, and added the coordination bind, the accept's credential check, and
the bind's refusal to unlink, all three of section 2.3. Twenty come from this
section's sorting with the four walks counted in and twenty-eight from the
elections outside it, the elections taking nodes because gate H1 would
otherwise leave the largest decisions in this Spec untraceable. Two of the
twenty carry a review tag rather than a mechanical one, the path-taking
prohibition and the child handoff's unconditional flag clear, each being the
half a split divided out of a bullet this section already carried, and a
divided half counts with the bullet it came from, per Document Format section
3. **One bullet above is a claim another crate argues,** and carries no record
here: the floor's three exhaustive wire enums, which are `weaver-types-Spec`
section 4.2's. **Three of the claims sorted here discharge owings that Specs
other than `weaver-types-Spec` filed against this document,** the three that
Spec files being the paragraph below's, and each record sits at the clause
that argues it rather than at the sorting: the first walk's close-on-exec test
that `weaver-trace-Spec` section 10 owes, at section 2.3, the licensed
combinations that `weaver-traits-Spec` section 7 owes, at section 4, and the
second walk's descriptor placement with the fork discipline that
`weaver-gate-Spec` section 0 cites as this document's, at section 2.2.

**`weaver-types-Spec` section 5 files three owings against this document, and
all three now carry a record.** The truncation half of the pair test has been
this crate's since the Spec was cut, argued and recorded at section 2.1. The
boundary half of that same test, one envelope write arriving as one envelope
read, was unnamed here until this act, filed as issue 35 and landed at the
section 2.1 clause that argues it with its perturbation named below. The
out-of-order refusal is the third: it was stated at section 3 and sorted by no
instrument, filed as issue 32, and it lands at that clause on the shape
`weaver-gate-Spec` section 6 settled for the gate, the compile pin holding that
the refusal reaches a match arm and the perturbation holding what the arm then
does. Each record is this crate's side of an owing and no other organ's, so
issue 35 closes on the pair-creating crates together while issue 32 stays open
for the organs whose side is unwritten.

**Requiring a perturbation-verified test, beyond the four walks.**

- Truncation is a fault: an envelope over the 64 kibibyte bound produces
  `Truncated` and no directive, confirmed by watching a silently shortened
  directive decode when the `MSG_TRUNC` check is removed.
- One write is one read: two envelopes are written back to back on a created
  pair and both writes complete before either read, and two reads return
  exactly one envelope each, confirmed by watching the first read return both
  when `SOCK_SEQPACKET` is changed to `SOCK_STREAM` at the `socketpair` call.
  **Two messages are what make the watch reachable.** A single small envelope
  crosses a stream socket whole, so a one-message test would pass under the
  substitution and pin nothing, which is the never-failing perturbation apex
  section 11 counts as worse than no test. The truncation bullet above cannot
  see the substitution at all, which is why the boundary the type was elected
  to buy needs a watch of its own, per section 2.1.
- A directive out of order is refused and not queued: a leave arriving before
  any enter answers `OutOfOrder` and reaches no unwind, and a directive of any
  kind arriving after a leave answers the same, the left position being
  terminal. The compile pin of section 3 holds that the refusal reaches a match
  arm rather than a flag check, and this holds what the arm then does, an arm
  being free to queue or to answer the wrong refusal while compiling exactly as
  well. Confirmed twice, by watching the early leave reach the unwind path when
  the before-enter arm stops refusing it, and by watching a left channel accept
  an enter when the terminal arm is collapsed into the entered one. The refusal
  is owed to each organ by `weaver-types-Spec` section 5, which enforces it
  nowhere, and this discharges this crate's side of that owing alone.
- Announce-after-record: a stop's answer follows the close event's placement,
  confirmed by watching the answer precede the record when the two are
  reordered.
- The scoped refusal account: a refusal before the `load` event leaves the
  stream empty, and one after it leaves a bracket with no `unload`, each
  confirmed by watching the account degrade when the authoring point moves.
- The licensed combinations: an `Assistant` message carrying a `ToolResult`
  block is refused before submit, confirmed by watching it reach the recorder
  when the check is removed, per `weaver-traits-Spec` section 7.
- Deterministic assembly: one working structure assembles one prompt,
  byte-identical across runs, confirmed by watching the comparison fail when
  iteration order stops being sequence order.

```graph
node: harness-path-shapes-pinned-by-doctest
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-harness
to: harness-path-shapes-pinned-by-doctest
```

## 9. Open elections

Each names what settles it, and none is this Spec's to settle alone.

- **The executor.** The engine's shape closed with the token workflow's
  acts, loop 1's seat and the decode surface it composes against both being
  chartered, and what remains open is whether this crate takes a runtime at
  all, deferred with the latency measurement on the decode path per
  `weaver-traits-Spec` section 6, which may overturn section 1's no-runtime
  rule.
- **Stop mechanics at the decoder is closed.** The stop lands at the token
  boundary, ratified at the token workflow's charter act of 2026-08-02 and
  carried at `weaver-spu-PRD` section 13.5, with the family's turn terminator
  made resident before the answer returns. Recorded as closed rather than
  deleted, this list naming what settled each entry.
- **The tee back-pressure election.** Blocking, shedding marked, or detaching
  marked, per `weaver-admin-operator-contract` section 3. A measurement
  against a real consumer at a real rate, taken with the queue's high-water
  mark of `weaver-trace-Spec` section 11, the two settling together.
- **The fault payload's shape is closed.** The case set closed across all
  three organs on 2026-08-02 and the shape landed at `weaver-trace-PRD`
  section 3.2 as the floor's `fault-report`, so section 3's fault path is
  shaped and ungated.
- **The config read, and the sink field's custody question.** The charter's
  `reads` edge to `agent-config` is exercised by no chartered workflow, so
  the read arrives with the tool workflow, which consumes the tool set and
  the permission mode. That pass must also answer what section 5 of the
  charter makes awkward to leave implicit: the config's `trace-sink` field
  names the sink, the charter's custody prose has the agent never told the
  name, and the kernel's search-bit lock is what stands between knowledge
  and reach. Whether the read drops the field unretained, or the never-told
  sentence rescopes to the descriptor mechanism, is that pass's to elect
  with the charter in hand.
- **Tool dispatch, the execution context, and the permission-mode
  enforcement point.** Blocked with `tool-trait`, per `weaver-traits-PRD`
  section 3.1.
- **The satellite types.** `AdoptionFault`'s case set, `OrganChannel`'s
  exchange-surface spelling, `DecodeChannel`'s and `SpuChannels`' names, the
  licensing error's shape, and the
  channel-state enum's name. Identifier and shape choices with no
  cross-crate consequence, listed so what this Spec leaves to a builder is
  complete rather than implied.
