# weaver-gate - Spec

**Status:** DRAFT. Cut 2026-08-02, sixth of the Spec pass, specced to the same
boundary its charter is chartered to: the lifecycle half, with the traffic
arriving via the token workflow. No code is written against it until phase
three is ratified, per Working Process section 6.

**Date filed:** 2026-08-02
**Document ID:** `weaver-gate-Spec`
**Parent:** `weaver-gate-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

Build instructions for `weaver-gate`: the binary's layout, the seam end's
adoption, the hook's mechanics, the predicate at accept, and the elections a
builder would otherwise invent. It is derived from `weaver-gate-PRD` and from
the two contracts this crate is party to, `weaver-harness-gate-contract` and
`weaver-gate-world-contract`, together with `weaver-organ-channel`, the drawn
material the first of them draws.

Level discipline. The charter says what the crate needs and why. This document
says how it is represented, and per gate G2 it elects against grounds the
charter and the contracts state rather than developing grounds of its own.
Where this document and the charter disagree the charter yields nothing.

**This document declares its crate's assertion records,** per Document Format
sections 3 and 4 as of the notation of 2026-08-03. The charter stays the source
of this crate's node, its parent edge, its one floor link, and its one declared
seam, and a Spec that restated them would give the mapper two sources for one
record, per Document Format section 1. What this document sources is the claims
code must conform to, declared at the clauses that argue them rather than
gathered in one place, per that format's section 6, and `asserts` runs from the
crate rather than from this document, which is why the document needs no node of
its own. **What this document leans on and does not argue is declared by the
crate that argues it,** and carries no record here. The descriptor's number and
the fork discipline that leaves this process one inherited descriptor are
`weaver-harness-Spec`'s, per its sections 2.2 and 8. The floor's three
exhaustive wire enums, `PeerIdentity`'s missing `Deserialize`, the socket-type
election with the boundary and truncation tests it owes the pair-creating
crates, and the one policy function with denial before permission are
`weaver-types-Spec`'s, per its sections 3, 4.2, and 4.3. A node declared twice
is the one-name-two-nodes defect that format forbids, and a reliance stays prose
because a citation is what it is.

**It is written from the merged corpus alone,** per the ruling of 2026-08-01
that keeps the old tree's Specs out of the Spec pass.

**The bound is the charter's own half-chartered line.** What is fully
specifiable is the raise and the lower, the boundary predicate, and the
process facts. The turn exchanges, the relay's interior, streaming,
backpressure, cancellation, drain, and concurrent clients defer with the
token workflow, per charter section 8, each waiting rather than missing.

## 1. The crate

**One binary.** The gate is its own executable, forked and exec'd by the
harness during the enter fan-out, per apex section 12, and nothing links it.

```graph
node: gate-one-binary
kind: assertion
tag: manifest

edge: asserts
from: weaver-gate
to: gate-one-binary

edge: grounds
from: gate-one-binary
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**Layout.** One module per obligation, with one placement.

    src/main.rs     entry, the two hygiene sets, and wiring, and nothing else
    src/channel.rs  the seam end and the exchange service, section 2
    src/hook.rs     the instruction's resolution, the bind, the predicate, section 3
    src/relay.rs    the pass-through, deferred, section 4

Four files, `relay.rs` standing as a placement the way the harness Spec
places its deferred modules.

**Edition and toolchain.** Edition 2024 on the pinned nightly, no nightly
feature used.

**The dependency set is one internal crate and two external ones.**
`weaver-types` is the charter's one floor link, taken **without its `config`
feature**: this crate reads no configuration file, per charter section 3, the
gate instruction arriving over the seam instead, so no parser enters a
process whose whole argument is that it holds little, which is the thinness
the feature gate exists for, per `weaver-types-Spec` section 1. No direct
`weaver-traits` line exists, matching the charter's floor-link set.
`serde_json` encodes and decodes the seam's envelopes and touches no client
byte, because the client's line is octets this crate must not read, per the
opacity rule. `nix` is the OS surface, on the grounds
`weaver-harness-Spec` section 2.4 argued: the needed calls are `bind`,
`listen`, `accept`, `getsockopt` for the peer credential, `fcntl`, and the
two `prctl` sets of section 2.

```graph
node: gate-floor-link-types-without-config
kind: assertion
tag: manifest

edge: asserts
from: weaver-gate
to: gate-floor-link-types-without-config

edge: grounds
from: gate-floor-link-types-without-config
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**No async runtime, no logging crate, nothing else.** The lifecycle traffic
is two exchanges and the client traffic is deferred, so nothing here needs an
executor, and this crate writes no account of anything, per charter section
1: a logging crate would be a second author's first step. The absences are
checked by the build-time `cargo tree` assertion the floor Specs share.

```graph
node: gate-no-runtime-no-logging-no-yaml
kind: assertion
tag: manifest

edge: asserts
from: weaver-gate
to: gate-no-runtime-no-logging-no-yaml
```

## 2. The seam end, and the process facts

**The channel end arrives at descriptor 3, inherited rather than
re-decided.** `weaver-harness-Spec` section 2.2 elects the number and owes it
to this document, and this document takes it: the exec'd binary finds its
one inherited descriptor at 3, the first after the standard streams, and
wraps it as an owned handle at entry. It is the only descriptor this process
begins with beyond the standard streams, per the fork discipline of
`weaver-harness-gate-contract` section 1, which makes a build in which this
crate holds a trace, coordination, or residency handle broken whether or not
it uses one. That property is the harness's to enforce at the fork and this
crate's to rely on, and the reliance is stated rather than re-tested here,
the enforcing test living in the harness Spec's section 8. **The wrap is not
a formality.** Descriptors are owned types end to end in this crate, the
listener and the accepted connection of section 3 included, so no raw number
outlives the thing it names and no close happens twice.

```graph
node: gate-descriptors-owned-types
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-gate
to: gate-descriptors-owned-types
```

**Entry performs two hygiene sets and one election, before the first read.**
The dumpable flag is cleared and the channel end is set close-on-exec, both
sets and never checks, per charter section 7 and the set-not-check rule of
`weaver-organ-channel` section 2: this crate spawns nothing, so the flag is
defense against a compromise's exec rather than a planned fork, and it costs
one call. The election is the parent-death signal, which the charter leaves
to this Spec: taken, one `prctl` naming `SIGTERM`, with its guarantee stated
at the width the kernel gives it and no wider. The signal fires on the
termination of the thread that forked this process, not of the harness
process, verified by both seats against a live kernel with the forking
thread exited and the process fully alive. So the backing is real exactly
while the gate is forked from a harness thread whose lifetime is the
worker's, an obligation owed to `weaver-harness-Spec` the way descriptor 3
was owed in the other direction, filed on the working list: that Spec's
everything-on-the-caller's-thread posture already implies it, and the
sentence makes it a stated constraint a later threading change must answer.
It backs the closure observation rather than replacing it, the requirement
standing on closure alone per charter section 4, and it covers the window
where the gate is blocked anywhere other than the channel read, for as long
as that constraint holds. **The backing is review's by election and not by
impossibility,** the third walk's test reaching the closure half alone. A
harness double that forks the gate from a thread of its own and then ends
that thread while its process lives and holds its channel end open produces
the condition the signal keys on, which is the case both seats compiled on
2026-08-02 to narrow the guarantee in the first place, so the reach exists
and this suite simply does not buy it. The signal's presence, the width of
its guarantee, and the thread-lifetime constraint the backing rests on are
review's on that ground, while the closure requirement they back is the
walk's, which is the same split the bind-site absence takes in section 3.
**The channel end's close-on-exec is review's on that ground and not a
weaker one,** an `fcntl` reading it as cheaply as a `prctl` reads the flag
set beside it: the two hygiene sets differ in the instrument bought, walk 2
taking the flag, and not in the instrument available.

```graph
node: gate-dumpable-flag-cleared
kind: assertion
tag: perturbation

edge: asserts
from: weaver-gate
to: gate-dumpable-flag-cleared

edge: grounds
from: gate-dumpable-flag-cleared
to: axiom-floor-is-vocabulary-behavior-is-socket

node: gate-channel-end-close-on-exec
kind: assertion
tag: review

edge: asserts
from: weaver-gate
to: gate-channel-end-close-on-exec

edge: grounds
from: gate-channel-end-close-on-exec
to: axiom-floor-is-vocabulary-behavior-is-socket

node: gate-parent-death-signal-thread-scoped
kind: assertion
tag: review

edge: asserts
from: weaver-gate
to: gate-parent-death-signal-thread-scoped
```

**The exchange service is a serial loop over the channel.** Directives
arrive as `OrganEnvelope` JSON, one message per envelope, on the
`SOCK_SEQPACKET` end the harness created, and this crate carries the
election's receive obligation as every receiving crate does, per
`weaver-types-Spec` section 4: the buffer is sized to the 64 kibibyte
envelope bound, a read returning with `MSG_TRUNC` set is a channel fault and
never a message, and the same bound is asserted on this crate's sends. A
directive out of order for the channel's state answers `OutOfOrder`, per
`weaver-harness-gate-contract` section 3, and the state has three positions,
before-raise, raised, and lowered, the last terminal.

```graph
node: gate-truncation-is-a-fault
kind: assertion
tag: perturbation

edge: asserts
from: weaver-gate
to: gate-truncation-is-a-fault

edge: grounds
from: gate-truncation-is-a-fault
to: axiom-floor-is-vocabulary-behavior-is-socket

node: gate-out-of-order-refused
kind: assertion
tag: perturbation

edge: asserts
from: weaver-gate
to: gate-out-of-order-refused

edge: grounds
from: gate-out-of-order-refused
to: axiom-contract-is-a-complete-interface

edge: grounds
from: gate-out-of-order-refused
to: axiom-harness-integrates-by-the-loop

node: gate-channel-state-three-positions
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-gate
to: gate-channel-state-three-positions

edge: grounds
from: gate-channel-state-three-positions
to: axiom-contract-is-a-complete-interface
```

**Closure is death, and the response is the charter's.** A read that returns
closure means the interior is gone: this crate closes its listener if one
stands and exits, per charter section 4, never treating closure as an
answer, per the drawn material.

```graph
node: gate-closure-is-death
kind: assertion
tag: perturbation

edge: asserts
from: weaver-gate
to: gate-closure-is-death
```

## 3. The hook

**The instruction is resolved, never interpreted beyond its fields.** The
raise directive carries the `gate-instruction` the operator declared and
admin validated, uninterpreted by the harness, and this crate consumes
exactly two things from it: the socket path to bind and the access rule the
predicate judges against. The field list is the floor's satellite, per
section 6, and the demand stated here is what that satellite must carry.

```graph
node: gate-instruction-two-fields-consumed
kind: assertion
tag: review

edge: asserts
from: weaver-gate
to: gate-instruction-two-fields-consumed
```

**The client socket is `SOCK_STREAM`, elected on the same ground as the
operator surface.** `weaver-gate-world-contract` section 2 fixes
newline-delimited JSON, one request per line, so the newline is the framing
and a boundary-preserving type would carry a second framing under the
first. A stream is also what a local client's ordinary tooling dials, which
is the audience that contract names. The opposite election from the organ
channels, principled the same way twice now, per `weaver-admin-Spec`
section 2.

```graph
node: gate-client-socket-stream
kind: assertion
tag: review

edge: asserts
from: weaver-gate
to: gate-client-socket-stream

edge: grounds
from: gate-client-socket-stream
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**The bind takes the path as given and refuses what it finds in the way.**
The socket is created with the close-on-exec flag in the creating call and
bound to the instruction's path. A path already occupied refuses the raise
with `BindFailed` and the reason carried, and this crate unlinks nothing:
the path is the operator's artifact, a stale socket left by an unclean
death is the operator's to clear, and a gate that deleted filesystem
entries to make room for itself would hold an authority its charter never
grants. Ready is answered only after both the bind and the listen have
returned, which is what makes ready a fact about the listener, per
`weaver-harness-gate-contract` section 2. **`hook.rs` exposes the crate's one
bind site,** taking the instruction's path, and a listener built anywhere
else in this crate is out of bounds. The two named shapes are pinned by the
compile-fail doctests of section 6, because an absence is what a runtime test
structurally cannot demonstrate, **and the general prohibition stays
review's,** a pair of doctests reaching the shapes they name and not the open
set of every way a path becomes a listener. The pinning and the prohibition
are two records for that reason, per section 6.

```graph
node: gate-ready-follows-bind
kind: assertion
tag: perturbation

edge: asserts
from: weaver-gate
to: gate-ready-follows-bind

edge: grounds
from: gate-ready-follows-bind
to: axiom-contract-is-a-complete-interface

node: gate-unlinks-nothing
kind: assertion
tag: review

edge: asserts
from: weaver-gate
to: gate-unlinks-nothing

node: gate-one-bind-site
kind: assertion
tag: review

edge: asserts
from: weaver-gate
to: gate-one-bind-site

edge: grounds
from: gate-one-bind-site
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**Every connection is authenticated at accept, before any byte is read.**
The accepting call sets close-on-exec on the connection, the peer's
credential is read with `SO_PEERCRED`, and the identity is judged by the
floor's one predicate against the instruction's rule. Verified on the
admin Spec's pass and relied on here: the credential on an accepted
connection reports the connecting peer's own uid, gid, and pid.

```graph
node: gate-authenticated-at-accept
kind: assertion
tag: perturbation

edge: asserts
from: weaver-gate
to: gate-authenticated-at-accept

edge: grounds
from: gate-authenticated-at-accept
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**The agent uid is denied by construction, not by configuration.** This
process runs as the agent uid, so it knows the one uid the boundary exists
to exclude: its own. The deny set the predicate judges is the instruction's
rule with this process's own uid added at raise, unconditionally, so no
operator mistake in the rule can readmit the agent, denial winning over
permission per `weaver-types-Spec` section 3. A peer that fails is refused
by closure before any content is read, per `weaver-gate-world-contract`
section 5, and nothing is written to it, an admitted-looking answer to a
refused peer being a conversation the boundary already declined.

```graph
node: gate-agent-uid-denied-by-construction
kind: assertion
tag: perturbation

edge: asserts
from: weaver-gate
to: gate-agent-uid-denied-by-construction
```

**The lower closes the listener first and confirms after.** Stopped is
answered only after the close has returned, per the contract, so nothing
new can arrive anywhere in the interior once the harness proceeds. In this
pass no traffic exists, so the close is the whole of it, and what happens
to an in-flight connection at lower is drain, deferred with the token
workflow.

```graph
node: gate-stopped-follows-close
kind: assertion
tag: perturbation

edge: asserts
from: weaver-gate
to: gate-stopped-follows-close

edge: grounds
from: gate-stopped-follows-close
to: axiom-contract-is-a-complete-interface
```

**A refusal leaves nothing held.** A failed bind holds no listener and no
half-bound socket, so the aggregate's rollback has nothing of this crate's
to unwind, per charter section 5, and the refusal is answered rather than
exited on, a party that exited replacing a typed reason with an observed
death.

```graph
node: gate-refused-raise-holds-nothing
kind: assertion
tag: perturbation

edge: asserts
from: weaver-gate
to: gate-refused-raise-holds-nothing

edge: grounds
from: gate-refused-raise-holds-nothing
to: axiom-harness-integrates-by-the-loop
```

## 4. The relay, deferred

`src/relay.rs` is the placement for the pass-through: inbound octets to the
harness, outbound octets to the client, order preserved, nothing retained,
per charter section 2. Everything it needs is the token workflow's, the
turn exchanges toward the harness above all, per charter section 8, and
nothing is shaped here ahead of that charter, per apex section 9. What this
Spec fixes about it is only what the merged contracts already fix: the
relay reads no content, and a line that does not parse is refused by the
harness with the refusal returning by the path the line took, the gate
carrying both directions unread. The claim binds the suite as much as the
build, which is why section 6 has no test that parses a client line, and it
is review's at both ends: no instrument distinguishes octets forwarded from
octets read.

```graph
node: gate-client-content-unread
kind: assertion
tag: review

edge: asserts
from: weaver-gate
to: gate-client-content-unread

edge: grounds
from: gate-client-content-unread
to: axiom-harness-integrates-by-the-loop
```

## 5. The failure vocabulary

```rust
pub enum ChannelFault {
    Truncated { bound: usize },
    Undecodable,
    Closed,
}
```

**Three faults, the harness Spec's split adopted.** A refusal is a typed
answer on an exchange, every refusal this crate issues is a
`lifecycle-refusal` drawn from the floor, `BindFailed` above all, and a
channel fault is a failure below the exchange layer. `Closed` is the one
this crate does not survive: the exit path of section 2 is its handling,
and the other two are answered as faults of the exchange in flight.
**The split is what keeps the contract's refusal set closed,** which is the
grounding the block below carries: a refusal is drawn from the floor's
enumeration and named by the contract, so a failure that is not one of
those is classified below the exchange layer rather than admitted as a new
refusal the other party never agreed to read.

```graph
node: gate-fault-below-the-exchange-layer
kind: assertion
tag: review

edge: asserts
from: weaver-gate
to: gate-fault-below-the-exchange-layer

edge: grounds
from: gate-fault-below-the-exchange-layer
to: axiom-contract-is-a-complete-interface
```

## 6. What is enforced, and by which instrument

Per apex section 11, the walks first.

**The reference walk: an elected tool dials the agent's own mouth.** The
adversary is the agent's tool surface running as the agent uid, the attack
a dial of the named socket the instruction declares, the prompt-yourself
loop the charter names. The mechanism is the predicate at accept with the
agent's own uid denied by construction, per section 3. The test is the one
`weaver-types-Spec` section 3 owed this document, landed: a connection
from the agent uid is refused at accept, before any content is read,
confirmed by watching content reach the harness when the predicate is
weakened.

**The second walk: a same-uid process attaches to the gate.** The adversary
is the same tool surface, the attack a `ptrace` or `/proc/[pid]/fd` reach
into the one process of the worker's family that faces outward. The
mechanism is the dumpable flag cleared at entry, a set and not a check.
The test confirms the flag is clear after entry, watched to fail when the
set is removed.

**The third walk: the boundary outlives its interior.** The adversary is
timing, a client conversing with a gate whose harness is gone. The
mechanisms are the closure observation, exit on a closed channel, and the
elected parent-death signal backing it. The test closes the harness end of
a standing pair, the double staying alive, and confirms the gate exits and
the listener is gone, watched to fail when the closure handling is removed.
**The double has to live for the watch to mean anything.** Killing it as a
process ends the forking thread too, and the signal then exits the gate
whether or not the closure handling stands, which is a perturbation that can
never be seen to fail. **The test reaches the closure half and no further,**
the signal's backing being reachable by a double that forks from a thread
and exits it while its process lives, and simply not bought here. So this
walk's perturbation record is the closure mechanism and the signal's backing
is review's by election, at the section 2 clause that elects it, two records
rather than one for the reason the bind-site absence divides below.

**Enforced by the compiler.**

- The floor's three wire enums are exhaustive, so every directive, answer,
  and refusal case reaches this crate's matches loudly.
- Descriptors are owned types end to end.
- The channel state's three positions are a type, so a directive against a
  lowered hook is refused by a match arm rather than a flag check.

**Enforced by compile-fail tests.** One absence is this crate's own to pin:
`hook.rs` exposes one bind site taking the instruction's path, and a
doctest constructing a listener from a bare `&str` or `PathBuf` anywhere
else in the crate fails to compile, the two named shapes with the general
prohibition staying review's, per the floor Specs' split. **The split is two
assertions rather than one,** the pinned shapes here and the prohibition
itself at the section 3 clause that argues it: a single record tagged for the
mechanical half would claim the doctests for the whole, which is the
overclaim this corpus refuses in prose and has no reason to admit in a graph.
The load-bearing absence this crate relies on, `PeerIdentity` deriving no
`Deserialize`, is the floor's pin, per `weaver-types-Spec` section 3.

**Enforced by the manifest.** The internal dependency is exactly
`weaver-types` without the `config` feature, read against the graph's one
floor-link under gate H2. No async runtime, no logging crate, and no YAML
implementation in the resolved tree, by the build-time `cargo tree`
assertion the floor Specs share.

**Which invariant each claim serves, and why seven serve none.** Seventeen `grounds`
edges run from sixteen of the twenty-three, nine to
`axiom-floor-is-vocabulary-behavior-is-socket`, five to
`axiom-contract-is-a-complete-interface`, and three to
`axiom-harness-integrates-by-the-loop`, with one claim carrying two edges because two
invariants each give it a reason. **The test applied is whether the axiom is the reason
the claim exists, or the claim a precondition of the axiom's own stated reason.** Remove
the socket invariant and this crate has no reason to authenticate at accept, no reason
to elect a socket type for the world seam, no reason to carry a truncation obligation,
and no reason to confine socket creation to one site, so those ground in it. Remove it
and the descriptors are still owned types and the parent-death signal is still elected,
so those ground in nothing. **Two claims this act first refused are grounded on the
operator's rulings of this date.** The dumpable flag holds a premise the socket
invariant argues from rather than a rule it states, which is the second grounding
relation Document Format section 4 now names. The channel state grounds in the contract
invariant because
`weaver-harness-gate-contract` section 3 states the ordering outright, that
lower is last and terminal and that turn exchanges are valid only between a
completed raise and a lower, so the claim was never the internal
representation that invariant excludes. **Eight claims grounding
in no invariant is the expected result and not a gap**, per Document Format
section 4: most of what this document elects is a representation, a
placement, or a hygiene set the charter's walks argue, and representation is
what the invariants are not about.

**The other two axioms take nothing from this crate.** The join key binds a
request belonging to an existing turn, and every exchange this document
specifies is a lifecycle directive belonging to none and carrying none, per
apex section 5.2. The turn exchanges that will carry a key are the token
workflow's and defer with the relay, so that edge lands when they are specced
rather than now. The organ test is a classification the charter passes, and
the one claim it would reach here, that this process holds a channel with the
harness and no other organ's handle, is `weaver-harness-Spec`'s to enforce at
the fork and carries no record in this document, per section 0.

**Three groups among the sixteen are worth stating rather than leaving to be
read.** The two manifest claims ground in the socket invariant because both
are the linkage facts that invariant defines: the gate is a separate
executable nothing links because its behavior is reached over a socket, and
its one internal dependency is a floor link because the floor is the two
crates every domain draws from and no domain contains. The bind site's two
records ground there for a reason both halves share, the pinned shapes and
the general prohibition being halves of a divided claim: a listener built
anywhere but the one site is a socket the accept predicate never guards,
which is the exception that invariant states it does not admit. Four of the
five contract edges are the ordering half and the error half of a complete
interface, ready and stopped being answers the harness builds against
without asking what this crate does, out of order being an ordering
guarantee refused rather than queued, and the fault split being what keeps
the refusal vocabulary drawn from the floor rather than invented here.

**The three loop edges are the places this crate hands work back to the
integrator rather than settling it here.** Apex section 5.5 makes the harness
answerable for correctness and for timing across domains and leaves each organ
answerable inside its own, so a claim grounds in it when the claim is this crate
declining what the loop is answerable for. A directive out of order arrived in an
order this crate did not choose, and a gate that queued it would be reconciling a
timing failure across domains, which that invariant assigns to the loop by
construction. **That record carries two edges and both are real.** Apex section
5.3 requires the contract to state the ordering it relies on, which is why the
refusal is specifiable at all, and apex section 5.5 makes the loop answerable for
that ordering still holding when this crate's ordering meets the SPU's in one
fan-out. A refused raise holding nothing is what lets the fan-out's rollback stay
the loop's work, a half-bound listener left behind being state the harness cannot
see and would have to be told about, which is the organ-to-organ arrangement that
invariant exists to prevent. The relay carrying both directions unread draws the
same line at the world seam: a gate that read a client's line would be deciding
something about the turn, which is the harness's domain, and the refusal of an
unparseable line returning from the harness by the path the line took is that
decision staying where it belongs.

**What the loop invariant did not reach, and why the line falls there.** Ready
follows the bind and stopped follows the close are this crate's own sequencing
inside its own domain, presented at its edge as guarantees the contract states,
and apex section 5.5 says nothing about what happens inside a domain. The channel
state's three positions are the representation that makes the refusal mechanical
rather than the refusal itself, and grounding a representation in an invariant
about what crosses would read the scope limit backwards. The one claim that
invariant's presents-nothing-to-any-peer clause would reach here, that this
process holds a channel with the harness and no other organ's handle, is the same
claim the organ axiom would reach and is `weaver-harness-Spec`'s to enforce at the
fork, carrying no record in this document, per section 0.

**Two calls the labelling first refused, and why.** The agent uid denied by
construction is this crate's centerpiece and it grounds in nothing. The
socket invariant protects the second party knowing who the first is, which
the credential at accept discharges in full, and what the boundary then does
with that knowledge is the charter's reference walk rather than the
invariant's. **The two hygiene sets do not go the same way, and the sentence this act
first wrote here answered for the wrong seam.** This process holds two seams of
different kinds. The world socket is named and its identity property does stand
or fall with the credential. The channel end at descriptor 3 is an unnamed pair
with no credential at all, authenticated by possession, and the socket invariant
rests that case on a premise rather than a rule: no third party can reach a
socket that has no address. A same-uid process reaches it through this process's
own descriptor table unless the flag is cleared, so clearing it holds the premise
true and grounds on the second relation. The channel end's close-on-exec is review's by
election, per section 6, and grounds on the same footing, an exec'd image holding the
end being a third party in possession of a socket with no address.

**Where the assertion records sit, and which of these bullets another crate
declares.** The records are at the clauses that argue the claims, across
sections 1 through 5, rather than gathered here, per Document Format section
6: this section sorts by instrument and the arguments are elsewhere, so a
block here would sit apart from the prose that earns it. One record is the
exception and sits at the end of this section, the doctest pinning of the two
bind-site shapes, whose argument is nowhere else and whose general
prohibition is section 3's. Twenty-three records in all, seventeen from this
section's sorting with the walks counted in and six from the elections
outside it, a split's two halves both counting as this section's because
neither was elected and one was divided out, per Document Format section 3.
**Two of the bullets above are claims another crate argues,** and carry no
record here: the floor's three exhaustive wire enums and `PeerIdentity`'s
missing `Deserialize`, both `weaver-types-Spec`'s. An assertion belongs where
its argument and its instrument live, and a node declared twice is the defect
that format forbids.

**Requiring a perturbation-verified test, beyond the walks.**

- Ready follows the bind: the answer is sent only after bind and listen
  return, confirmed by watching a client's dial succeed against an
  unconfirmed raise when the ordering is reversed.
- Stopped follows the close: confirmed by watching a dial succeed after a
  stopped answer when the ordering is reversed.
- A refused raise holds nothing: after a `BindFailed`, no listener exists
  and no socket file was created by this crate, confirmed by watching a
  leaked listener appear when the cleanup-on-refusal is removed.
- Truncation is a fault: an over-bound envelope produces `Truncated` and
  no directive, confirmed by watching a silently shortened directive
  decode when the `MSG_TRUNC` check is removed.
- A directive out of order is refused and not queued: a lower arriving
  before any raise answers `OutOfOrder` with no listener standing after
  it, and a directive of any kind arriving after a lower answers the
  same, the lowered position being terminal. The compiler bullet above
  pins that the refusal reaches a match arm rather than a flag check,
  and this pins what the arm then does, an arm being free to queue or to
  answer the wrong refusal while compiling exactly as well. Confirmed
  twice, once per case, by watching an early lower drive the channel to
  the terminal position when the before-raise arm stops refusing it,
  after which a legitimate raise is refused and no listener ever stood,
  and by watching a lowered channel accept a second raise when the
  terminal arm is collapsed into the raised one. One watch would leave
  whichever case it misses claimed and unenforced, which is the reading
  this bullet exists to refuse. The refusal is owed to each organ by
  `weaver-types-Spec` section 5, which states it and enforces it nowhere,
  and this discharges the gate's side of that owing alone.
- The unparseable-line refusal path stays whole-cloth deferred: no test
  here parses a client line, and a test that did would be the opacity rule
  breached by the suite, which review checks for.

```graph
node: gate-bind-shapes-pinned-by-doctest
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-gate
to: gate-bind-shapes-pinned-by-doctest

edge: grounds
from: gate-bind-shapes-pinned-by-doctest
to: axiom-floor-is-vocabulary-behavior-is-socket
```

## 7. Open elections

Each names what settles it, and none is this Spec's to settle alone.

- **The `gate-instruction` field list.** A satellite of
  `weaver-types-Spec`, with this document's demand stated in section 3:
  the socket path and the access rule. The demand is recorded here so the
  satellite is shaped against a consumer rather than invented.
- **Everything the token workflow charters.** The turn exchanges and their
  shapes, the relay's interior, streaming, backpressure, cancellation,
  drain on stop, and concurrent clients against the one-turn loop, per
  charter section 8.
- **The tool-uid ruling.** Charter section 7's pending candidate, settled
  by the architecture seat's ratification or the tool workflow's threat
  measurement, and nothing here builds against the separate-uid arm.
- **The satellite types.** `ChannelFault`'s spelling against the harness
  Spec's identical enum, one shared shape in two crates being tolerable
  where a shared crate would be a dependency taken for a name, and the
  channel-state enum's name. Choices with no cross-crate consequence,
  listed so what this Spec leaves to a builder is complete.
