# WeaverTools - Technical Report

**Status:** DRAFT. Describes `WeaverTools` at `fbcb73e`, 2026-08-21. Sections 5
through 14 are planned in appendix A rather than drafted.

**Date started:** 2026-08-21
**Document ID:** `weaver-tools-technical-report`
**Editorial:** ASCII, no em-dashes, no semicolons.

This document describes a built system: what it is made of, how a turn moves
through it, what it records, and what it will not do. It describes the system at
one commit, named above, because the system moves and a description with no
commit beside it cannot be checked against anything.

It is not a specification. The corpus under `docs/` is the authority on every
claim made here, and a builder implementing against WeaverTools reads the
contracts, which carry the vocabulary that crosses each seam, the errors it can
return, and the ordering it relies on and provides.

---

## 1. The problem

WeaverTools was built to answer a practical frustration. Our agents do not behave
the way we need them to, and when they fail we cannot see why. The failure is not
the model's alone and it is not the scaffolding's alone. It happens somewhere in
the traffic between them, and that traffic is exactly what a conventional setup
leaves unobservable.

Diagnosing it requires saying what kind of failure it was, and for that this
program borrows an instrument. Warren Weaver, introducing Shannon's *Mathematical
Theory of Communication* in 1949, distinguished three levels at which
communication can fail. **Level A, the technical problem:** how accurately are the
symbols transmitted. **Level B, the semantic problem:** how precisely do the
transmitted symbols convey the intended meaning. **Level C, the effectiveness
problem:** how effectively does the received meaning change conduct.

The three levels are Weaver's. **The reading this program puts on them is the
program's own**, and the distinction is worth keeping, because Weaver held the
levels to overlap and held the Level A theory to apply to the levels above it,
where this program reads them as a dependency hierarchy in which each level's
ceiling is set by the one beneath. Under that reading you cannot convey more
meaning than the channel permits and cannot produce more effective action than
the conveyed meaning supports, which is the pattern engineers already know from
network stacks and from operating system layering. Its consequence is what makes
the reading worth having: a single fault travels the whole stack while changing
appearance at each layer, surfacing as an effectiveness problem at Level C when
its ceiling was set at Level A.

A conventional agent deployment smears the three levels across a network nobody
controls. The model is behind an API, the scaffolding is somewhere else, retries
and timeouts and serialization sit between them, and when the assembly fails the
levels cannot be separated well enough to ask which one gave way. The useful
question is which level failed, and that question is what a smeared deployment
cannot ask.

WeaverTools separates them by construction. It pins Level A: one agent, one
machine, kernel-enforced identity, and boundaries preserved on every seam, so
transmission stops being a variable rather than being measured as one. It
captures Level B: every symbol that crossed every seam, in order, in one
canonical form, which is the trace. What is left over is Level C, whether the
agent behaves the way its operator needs, and that is the question the apparatus
exists to make answerable rather than guessable.

Tooling of this class exists inside the large labs and does not leave them.
Outside, the same work gets improvised one script at a time.

## 2. What the system is

An agent is **three processes on one machine**, plus a fourth program that stands
outside every agent and does not run while one is serving.

    worker      the composition root: the harness, the recorder, and the floor,
                linked into one binary, binding the coordination socket
    weaver-spu  the model organ, holding weights on the device
    weaver-gate the boundary, crossed in both directions: work in, response
                out, and the tool calls that reach the world
    weaver-admin root-run, one invocation per verb, then exit

Nine crates make those four programs. Two are the floor, shared vocabulary every
domain draws from and no domain contains, linked rather than dialed because a
type definition cannot be sent over a socket.

    weaver-types      1,323    the floor: config, identity, wire shapes
    weaver-traits       314    the floor: messages, roles, permissions, tools
    weaver-trace      1,443    the recorder and the in-RAM working structure
    weaver-state      1,178    the session custodian, sqlite behind a socket
    weaver-harness    7,050    the switchboard, the loops, trace authorship
    weaver-spu       12,061    residency, two decode engines, measurement
    weaver-gate       2,280    the boundary, and the shell as its own verb
    weaver-admin      3,093    lifecycle authorization and custody of the sink
    weaver-internal     297    functions the loop dispatches inward

Figures are lines of Rust under `src/`, 29,039 in total, with a further 7,300 in
integration tests.

**The apex governs exactly seven crate charters** and names them: admin, harness,
spu, gate, trace, traits, and types. `weaver-state` and `weaver-internal` were
chartered on 2026-08-18, after the set ratified, and the apex's enumeration has
not been amended to name them. A reader holding both documents should read that
list as the ratified set rather than as the current tree.

**An organ is a crate that governs a domain and holds a two-initiator channel
with the harness.** Both properties, and neither alone. The harness is the organ
whose domain is coordination, which is why it is the hub every other organ holds
its channel with rather than a spoke. Three crates pass the test against it:
`weaver-spu`, `weaver-gate`, and `weaver-admin`.

**A submodule falls under an organ's domain with that organ as its consumer and
holds no channel with the harness.** `weaver-trace` and `weaver-state` are both
submodules of the harness's domain, and their parent edges say so. They differ in
transport rather than in kind. The recorder is linked into the worker binary and
crosses no process line, so it authenticates nothing, there being no second
process to identify. The custodian sits behind a Unix socket, and that socket is
a member seam rather than an organ channel: one party asks and the other answers,
where an organ channel has either party able to open an exchange.

`weaver-internal` is neither organ nor submodule. It parents to the system, and
it is linked rather than dialed, holding no channel with the harness at all. It
is the operator's promotion space, where a capability that could stand as a
script outside the agent is mounted inside instead, because a control loop wants
an answer at a latency and a determinism the shell round trip cannot give.

The hub holds an allowance no other crate holds. Every organ presents its
contracts to the harness and nothing to any other organ, so a conflict between
two organs is settled in the contracts each holds with the harness rather than
between them. There are no lateral edges. Adding an organ is a socket and a
contract onto the hub rather than surgery on everything already standing.

The harness is content-neutral, and that is checkable rather than a slogan. It
holds no weights and performs no forward pass. It dispatches on the payload kind
of a message rather than on anything inside it. The instruction that configures
the SPU and the one that configures the gate both cross the harness
uninterpreted, resolved by their consumers. The gate relays in both directions
opaque, with order preserved, and does not parse the line it carries.

## 3. Every seam is a Unix socket

Where one crate asks another **process** to do something, the seam is a Unix
domain socket governed by a named contract, and it authenticates its peer. There
is no listening network socket in the program, at any depth.

The first question an engineer asks is why not localhost, and the answer has
three parts that stand separately.

**Latency.** An agent routes through the harness on every exchange, so any
per-hop cost compounds directly into the loop, per token, at batch one. Loopback
still pays the TCP stack, the kernel network path, and serialization on each of
those hops. A Unix socket collapses that to nearly nothing while keeping the same
topology. This is the program's one conceded theory claim, that latency is the
enemy of agency.

**It is reasoned rather than measured, and it is the one claim in this report
that is.** No per-hop figure for loopback against a Unix socket, at the message
sizes these seams carry, has been taken in this repository, and the measurement
regime that would take one stands outside it. What the corpus carries instead is
a measurement of the consequence rather than of the transport. The previous tree
ran production on full-history resend and measured a single exploration turn
climbing from 5,988 to 24,932 prompt tokens, with a concurrent request timing out
because the accumulated context had made every call slow. That is evidence that
latency compounds into how an agent behaves. It does not price the hop, and a
reader should not take the one for the other.

**Security.** A Unix socket inherits the operating system's trust model, which
means `SO_PEERCRED`, filesystem permissions, and kernel-enforced identity.
Localhost inherits the network's, which means authentication built above a
surface that is reachable as a network port by construction. Going OS-level is
what lets trust belong to the kernel and leaves no organ with a network surface
to defend.

**Measurement.** When the object of study is a stochastic engine, every component
between the observer and it is noise in the instrument. Removing network
variability is not a convenience here. It is what makes a reading attributable.

What this gives up is real and is not hedged: no fungibility across machines, no
distributed uptime, no failover, and no direct network applicability of the
deployed whole. The topology is network-shaped on purpose, a hub and spoke of
content-neutral duplex channels being the same architecture a network uses on a
quieter substrate, so a seam that runs over a socket today can run over a wire
tomorrow. Nothing was coded to the substrate, only to the topology.

**Authentication takes two forms and they are one property.** Where a channel has
a name, the peer is authenticated by credential. Where it has none, the channel
is a connected pair and possession of the descriptor is the authentication.

The named cases are worth stating concretely, because each was shaped by
something the kernel does rather than by preference:

- **The coordination socket** is bound by the worker inside the agent's sandbox
  as its first act, and `weaver-admin` dials in, one connection per verb. The
  harness reads `SO_PEERCRED` at every accept, before any byte, and refuses every
  peer that is not root. The direction was inverted in August 2026 for exactly
  this reason: the earlier design expected the agent's own uid, which every tool
  the agent can reach also satisfies, so an elected shell could have presented a
  credential the check would have accepted. Now it dials and is refused.
- **The gate's world socket** is authenticated at accept against a floor
  predicate that admits front-end principals. The gate runs as the agent uid, so
  it knows the one uid the boundary exists to exclude, its own, and adds it to
  the deny set at raise unconditionally, with denial winning over permission, so
  no operator mistake readmits the agent to its own front door.
- **The organ pairs** between the harness and the SPU and gate are unnamed
  `socketpair`s created before each fork. `SO_PEERCRED` on such a pair reports
  the creating process for both ends and therefore distinguishes nothing. The
  unnamed pair is chosen because it removes the second party, not because it
  identifies one.

**The two kinds of seam elect different transports, and framing is the reason.**
The world socket is `SOCK_STREAM`, elected because the newline its contract
already uses is the framing. The interior seams are `SOCK_SEQPACKET`, which
frames for them, so a message boundary is the kernel's to keep rather than a
reader's to find. A reader meeting one NDJSON line at the front door and framed
messages inside has met two transports chosen for what each already had.

One asymmetry is recorded rather than papered over. Where the gate dials rather
than accepts, `SO_PEERCRED` reports the credentials captured for the listening
socket rather than those of the process that accepted, verified against a live
kernel: a dialing client's read returned the pid that called `listen`, not the
one that called `accept`. The kernel does not support symmetric identification
here, and that is carried as a demand on the tool contract still to be written.

## 4. One turn, end to end

A turn is one request through to its final answer. Nine steps carry it, and the
trace events named at each are the record the next section is about.

1. A client dials the gate's world socket and sends **one NDJSON line**, UTF-8,
   one request per line, the newline being the framing.
2. The gate authenticates the peer by credential and relays the line inward
   without reading it. A request forwarded is a request gone: the gate keeps no
   work state.
3. The harness assigns the turn its key and authors `turn.started`. Every event
   from here carries that key at every seam it crosses, and every response
   carries it back.
4. The harness assembles the prompt.
5. It sends a decode request to the SPU over the decode channel, and authors
   `model.request`.
6. The SPU generates against the resident session and returns the generation
   together with its measurement payload, tagged with the turn key. The harness
   authors `model.output` and `model.measurement`.
7. If the emission carries a tool call, the family parse recovers it and the
   harness opens **one execution exchange per recovered call**, serially and in
   the order recovered, on its seam with the **gate**. Tool traffic crosses the
   gate like any other world traffic, per the egress ruling of 2026-08-07, which
   reversed an earlier reading in which outbound tool connections bypassed it.
   What happens on the far side depends on which tool it is, and today the gate
   holds exactly one. The shell is the gate's own outbound verb rather than a
   guest it hosts: the gate forks it into its own process group, supervises it
   against the caller's clock, kills the group past that clock because a shell
   leaves descendants holding the pipes open, and drains both pipes bounded and
   concurrently, since a pipe left unread turns a chatty command into a false
   kill. The answer is one of four contents, and a result is one of them.
   `tool.call.started` and `tool.call.completed` bracket the call, and control
   returns to step 5 with the result in the next prompt.
8. The harness authors `turn.closed`, whose payload states the close kind rather
   than leaving it to be inferred from an absence.
9. The response leaves through the gate as **one NDJSON line out**.

**The gate's second socket is chartered and unreached.** It is opened by the
agent rather than by the world, it admits registered applications that bind a
listening port, and no exchange of the harness-gate seam reaches it in this pass.
A reader should take it as the shape an external tool will arrive by rather than
as a path anything travels today.

Two properties of that path are load-bearing. **Each event is emitted once**,
with no second write to reconcile. And the turn is already closed at step 8
before the answer leaves at step 9, which is the rule that the crossing delivers
and does not clock. A gate that dies mid-delivery loses the delivery rather than
the turn.

---

## Appendix A. Sections not yet drafted

**5. The trace.** NDJSON with no framing layer, one line per event, the closed
kind set at nineteen, the flattened envelope, and two clocks that answer
different questions. Why every integer that can exceed the double-safe range
serializes as a decimal string. The bracketing grammar: session over run over
turn, strictly nested, interior events adding depth to a turn and never adding
turns.

**6. Custody.** The recorder's write surface takes descriptors and never paths,
not as a convenience and not behind a feature. Who opens the sink, which flag
rides the open file description and which rides the descriptor, and why
close-on-exec is set at the receive rather than checked. The threat walk: a tool
that has read `/proc/self/fd` and wants a second handle, and why it finds no call
that takes what it learned.

**7. What holds state, and for how long.** The working structure in RAM, the hot
KV cache, and the session custodian behind its socket. The two asks it serves.
Why every run begins empty and nothing the agent can draw on survives the
session.

**8. The model organ.** Residency as the whole of what it owns. Admission as the
one check on the device, its five steps, and the point past which a refusal stops
being free. Two decode engines as peers rather than a legacy and a target, with
the backend decided by the artifact rather than by configuration. Why the device
judgment reads the driver rather than the crate's own ledger.

**9. Observability as an election.** The measurement payload, the residual-stream
readout, and the probability field, each elected per load, named individually in
the record so a record declares the posture it was written in, and refused at
admit where the engine cannot honor it. The obligation that an elected diagnostic
changes no token, and why an absence has no shape.

**10. Replay, and what it is not.** The five inputs a record must carry, why
tokens are recorded rather than a seed, the template problem that makes canonical
messages insufficient, and the per-generation seed derivation of 2026-08-21.
Ending on the refusal: the program makes no run-again claim, in any arrangement.

**11. The lifecycle.** Loop 0 as the framework's own, four parties over three
sockets, what refuses a load and what a rollback guarantees, loaded-and-idle as a
first-class state, and gate last up and first down.

**12. The boundary.** What crosses the gate and in which direction. The port test
that decides internal from external. One tool, the shell, as the gate's own
outbound verb rather than a guest, with no tool table and a refusal by name
rather than a nearest match. Why a tool result has exactly one construction site,
and why no safety classifier exists or is planned.

**13. The builder's seat.** What a builder writes and what they inherit. A
disposition on every knob, frozen or operator-tunable, with the effective values
recorded whichever side set them. Why the loop is compiled rather than
configured, and how that holds variance to a range so what remains is
attributable to the thing under study.

**14. What stands, and what does not.** The demonstration rather than the count,
since a crate can report a high conformance figure while completing no turn and a
completed turn is not a count of claims met. Both engines serving, the trace over
its turns, and the assertions still open named with what each waits on. Against
that, what is deferred: client-facing streaming, the status ask, and the memory
leg, each with a named door rather than a reserved slot. Closing on what the
trade costs, including the program's own judgment that a local proto-stateful
agent is a defensible intermediate and an indefensible end state.
