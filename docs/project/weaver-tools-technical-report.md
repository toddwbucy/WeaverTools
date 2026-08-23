# WeaverTools - Technical Report

**Status:** LIVING. In `main` and outside the document set. **This document is
subject to change as development continues**, it is never ratified, and nothing
in the corpus is written against it. Describes `WeaverTools` at `da5a503`,
2026-08-22. All fifteen sections are drafted, appendix B carries what is not
built, not proven, or not measured, and appendix C states the formulation as
design intent rather than as description.

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

**It is also not the technical site**, which stands at `docs/technical/` as one
paper per crate over a shared contracts page. The division is by question rather
than by audience. A crate paper answers what one crate is and what it refuses,
reading out that crate's merged documents and restating no contract. This report
answers what the assembly does, which is a question no crate paper is positioned
to take, and it carries the material that belongs to no crate: the level
argument, the seam rule, the turn, and the holes. Where the two disagree the
merged corpus settles it and both are defects.

**The system is under construction, and this report says so wherever it is
true.** Where something described here is chartered but not built, built but not
proven, or claimed but not measured, the sentence describing it says which, and
appendix B collects every such case in one place. A reader looking for the holes
should not have to infer them from careful phrasing, and a report that read as
finished would be the more comfortable document and the less useful one.

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

    weaver-types      1,498    the floor: config, identity, wire shapes
    weaver-traits       314    the floor: messages, roles, permissions, tools
    weaver-trace      1,632    the recorder and the in-RAM working structure
    weaver-state      1,178    the session custodian, sqlite behind a socket
    weaver-harness    8,659    the switchboard, the loops, trace authorship
    weaver-spu       13,185    residency, two decode engines, measurement
    weaver-gate       2,280    the boundary, and the shell as its own verb
    weaver-admin      3,093    lifecycle authorization and custody of the sink
    weaver-internal     297    functions the loop dispatches inward

Figures are lines of Rust under `src/`, 32,136 in total, with a further 8,070 in
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

`weaver-internal` fits neither definition, and this report notes that rather
than resolving it. It holds callables the loop dispatches inward, linked into the
worker rather than reached over a socket, so it holds no channel with the harness
and fails the organ test. Its parent edge points at the system rather than at an
organ, which is what a domain root's edge does, so the submodule definition does
not reach it either. **Whether that is a gap in the test or a misparent is a
question for the apex and not for this document.** Appendix B carries it.

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

That is the invariant, and every seam standing today meets it. One chartered seam
does not yet exist to meet it: the gate's agent-opened socket, whose predicate
admits registered tools but whose contract is unwritten and whose binding end is
undecided. For that seam the sentence above states the rule it will have to
satisfy rather than a mechanism now running, and appendix B carries it.

The first question an engineer asks is why not localhost, and the answer has
three parts that stand separately.

**Latency.** An agent routes through the harness on every exchange, so any
per-hop cost compounds directly into the loop, per token, at batch one. What a
Unix socket removes from that cost is the transport overhead: loopback traverses
the TCP stack and the kernel's network path, and a Unix socket does not, while
keeping the same topology. What it does not remove is serialization. Every seam
here encodes and parses what crosses it, JSON on the interior seams and NDJSON at
the boundary, and that cost is paid on a Unix socket exactly as it would be on
loopback. The saving is the network path rather than the whole of what a hop
costs, and a reader should size the claim to that. This is the program's one
conceded theory claim, that latency is the enemy of agency.

**It is reasoned rather than measured.** No per-hop figure for loopback against
a Unix socket, at the message sizes these seams carry, has been taken in this
repository, and the measurement
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
quieter substrate. **The implementation is not substrate-neutral, and this report
will not imply that it is.** The seams are Unix-specific in code: `UnixListener`
and `UnixStream` at the named sockets, `socketpair` at the unnamed pairs,
`SO_PEERCRED` for credentials, and `SCM_RIGHTS` for descriptor passing. Moving a
seam to a wire would need framing and a peer-authentication mechanism that do not
exist here, and appendix B carries that as future work. What carries upward is
the topology and the contracts, which are written against what crosses a seam
rather than against how it crosses. The apex makes the narrower version of this
point about the gate alone, that the boundary is relocatable because a gate which
parsed content would have to know what produced it, and adds that the property is
a consequence rather than a plan and that the program builds nothing to exploit
it.

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
   guest it hosts, forked into its own process group and supervised against the
   caller's clock, with section 12 carrying the mechanics. The answer is one of
   four contents, and a result is one of them.
   `tool.call.started` and `tool.call.completed` bracket the call, and control
   returns to step 5 with the result in the next prompt.
8. The harness authors `turn.closed`, whose payload states the close kind rather
   than leaving it to be inferred from an absence.
9. The response leaves through the gate as **one NDJSON line out**.

**Where the resident context has to give ground, the edit is recorded.** Between
decodes the loop may return the decode context to its prefix, which is a flush, or
remove an interior span of it, which is an elision. Each crosses the decode seam
as its own exchange and each reaches the record, `flush` carrying the resident
counts either side and `elision` carrying those counts beside the half-open span
the loop named. **Which span to elide is the loop's election and the harness holds
no policy about it**, the seat forwarding the span unjudged, because a port that
judged one would be the switchboard deciding what a context is worth. The span
comes from the ask and the counts from the answer, each party writing what it is
the authority on. The two edits are recorded differently because they are
differently recoverable: a flush leaves a suffix a reader could infer from the
counts alone, and an elision leaves a sequence no count describes.

**Where a seam refuses, two records are written and neither implies the other.**
Each of the three asks that cross an organ channel can come back refused, and the
refusal reaches the record as its own event carrying the refusing party's own
account, the floor's refusal record naming which seam answered and carrying that
seam's case with the values it holds. Where the refusal ends the turn,
`turn.closed` carries `Refused`. **The close says the bracket ended and which kind
of ending it was, the event says what was refused, and neither is recoverable from
the other**, which is the division the fault event already runs on. The recorder
splices that account rather than shaping it, since a refusal is produced by the
party that refused, and typing it in the recorder would make the recorder hold and
version seam vocabularies it otherwise knows nothing about.

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

## 5. The trace

The record is NDJSON. One event is one line, UTF-8, with no interior newline and
no framing layer above the newline itself, because the sink is a file or a pipe an
operator owns and the line is already the frame. There is one schema, the durable
event, and the working structure in RAM holds that same canonical form rather than
a second representation of it. **No projection version exists to govern anything.**
A change to the durable event schema is the breaking change, and it is the one
version every consumer keys on.

**The kind set is closed and stands at twenty-one.** Closure is the property
consumers are allowed to rely on, so adding a kind is an edit to the trace charter
and to every contract whose vocabulary clause names the set. The set is not a
namespace anyone extends at the edges.

**The envelope flattens into the event and the payload does not.** An event renders
as one flat object rather than nesting its envelope under a member of its own, and
the reason is the reader: this line is what an operator's tooling consumes, every
such consumer keys on `kind` first, and a nesting level between the line and its
kind is a level every consumer pays on every event. The payload stays nested
because its shape is the kind's and flattening it would put a different set of
member names at the top level on every line.

**Four payload shapes are opaque to the recorder**, and that is a design property
rather than an omission. The four conversation kinds carry messages in the shape
`weaver-traits` defines, and the recorder neither defines that shape nor decodes
it. It records the octets, sequences them, and carries them through. Everything the
recorder guarantees - canonical byte form, a gapless run-scoped sequence, an
interrogable committed boundary, whole events to the sink, one rendering held and
handed, and typed refusal - it guarantees without knowing what a message says. A
crate that depends on no other crate in this program, linking a definition it does
not need, would give up that independence for nothing.

### Two clocks, because there is no single occurrence time

Every event carries two timestamps and neither answers the other's question. A
**session-scoped wall-clock stamp at millisecond resolution** answers the calendar
question, which is when this happened. A **run-scoped monotonic reading at
nanosecond source with a microsecond floor** answers the interval question, which
is how long that took. They are not interchangeable. A wall clock steps when the
machine's time is corrected and cannot be differenced across such a step, and a
monotonic reading names no date. Carrying one and deriving the other is the
mistake, so the record carries both and the charter states which party stamps each
and why the scopes differ.

**Every integer that can exceed the double-safe range serializes as a decimal
string.** Nanosecond values exceed it by roughly two hundred times, so a consumer
parsing them as doubles gets a silently different number, with no error raised and
no way back to the original.

**The reason is resolution rather than overflow, and the distinction decides the
design.** Read as overflow avoidance the rule looks like defensive rounding, and
the natural response to defensive rounding is to store fewer digits. That response
destroys the measurement. The monotonic clock resolves this finely because the
harness runs orders of magnitude faster than the decoder, which pays
network-class millisecond latency, and local latency is the reason the model was
brought next to the services at all. **A record that cannot resolve finer than the
fastest thing it measures makes the advantage it was built to show invisible.** The
decimal string exists to carry the digits, not to dodge an exception.

### The bracketing grammar

Four kinds open or close a scope and the rest occur inside one. `load` and `unload`
are the run bracket. `turn.started` and `turn.closed` are the turn bracket.
`tool.call.started` and `tool.call.completed` are the tool bracket. `session.closed`
says the session will not be resumed, which is a different claim from a run ending.

The brackets are **strictly nested**: session over run over turn, with interior
events adding depth to a turn and never adding turns. That is what lets a reader
reconstruct the shape of a session by scanning for openings and closings without
interpreting anything between them.

**Not every pair is a bracket.** The two classify kinds are a request and an
answer, not an opening and a closing, and they bracket nothing. Their turn member
is optional, because a classification between turns belongs to no turn, and a death
mid-exchange authors no fabricated answer.

**One kind is recorded per decode position rather than per turn or per exchange.**
`model.field` carries one position's ranked candidates with their probabilities and
the rank the draw landed on. The granularity is forced rather than chosen, since a
per-generation collapse of the field would be a single message of megabytes, past
the decode seam's own bound. It is also the first kind recorded only while its
election stands, and that exception is exactly what the election buys: the volume
is an order of magnitude past everything else the record carries, so a kind always
present would make the record unaffordable rather than complete.

### The record declares the posture it was written in

The `load` event carries every diagnostic election of that load by name. Without it
a record holding no field readings is indistinguishable from a record whose
election stood and produced nothing, **which is a configuration and a fault wearing
the same absence.**

**No election is bundled under a profile name.** A named set drifts as features are
added to it, and every earlier record carrying that name silently becomes a record
of something else. Naming each election individually is what keeps a record's own
account of itself from decaying while the record sits unchanged on disk.

### Pressure, and what the record does when it cannot keep up

The writer runs off the turn path. The harness reads the working structure in RAM,
the stream write happens elsewhere, and **a slow or failing sink never slows the
interior read** - the two are failure-isolated.

That isolation raises the question of what happens when the sink falls behind, and
the answer changed on 2026-08-22 in a way worth recording. Pressure first arrived
as a failure the recorder returned, which the harness could not act on without
treating a recorded event as a lost one, since the variant came back after the
event had already landed. **The depth moved to the recorder's own surface, so the
harness reads pressure after authoring rather than receiving it in place of a
sequence.** It reaches the operator as a `fault` naming `RecorderCommitPressure`,
authored by the harness.

**The reading is per authored event and the fault is not.** A fault for every
submission above the mark would answer a full queue by filling it, which is the one
direction that cannot help. The report is authored when the mark is first crossed
and not again until the depth has fallen back under it, because **a pressure report
is a report about a condition rather than about an event, and a condition that
persists is one condition.**

## 6. Custody

The trace is the agent's account of itself, which makes the agent the one party
that must not be able to edit it. Custody is how that is arranged, and it is
arranged in the kernel rather than in the recorder's good behaviour.

**The agent writes through a descriptor and by no other route.** `weaver-admin`,
running as root, opens the sink the agent's configuration declares, under the
role's principal, and passes the descriptor into the enter directive as
`SCM_RIGHTS` ancillary payload. The worker starts bare of descriptors and receives
this one. **So it writes a stream its own uid could not have opened**, which is the
whole of its access to the record, and which the operator revokes by closing the
descriptor.

Permission is checked once, when the sink is opened, against the opener. A
descriptor installed by `SCM_RIGHTS` rides against that same open file description
with no recheck afterwards. That is the property the design rests on, and it is
also the reason a wire version of this seam would need a different design rather
than a port of this one.

**This is a kernel check and not a concealment.** A process holding a descriptor
can always `readlink` its own `/proc/self/fd` and learn where the sink is, so
hiding the path was never available and was never the plan. What keeps the agent
off the file is the operator's provisioning - ownership, mode, and the search bit
on the directory above it - which admin's boundary check verifies rather than
builds. Nothing there is repaired and nothing there is constructed.

### The write surface takes descriptors and never paths

This is the API consequence of the custody model, and stating it as a property of
the surface is what makes custody expressible rather than merely intended. **A
crate that offers a path-taking write function offers a way around the boundary**,
whatever the surrounding prose says.

The previous tree is the cautionary case and it is a precise one. It carried a
trace-root resolver with zero production callers, whose layout described a path no
artifact ever used, while the security invariant three other documents cited was
specified against that resolver. The mechanism and the claim had drifted apart
without either being wrong on its own terms.

### Two flags, two routes, and only one of them travels

Every descriptor the recorder writes through is close-on-exec, and where the sink
is a file it is append-only as well. The first keeps a tool subprocess from
inheriting a writable handle. The second makes append-only a property of the handle
rather than of the writer behaving well, and it applies where the sink has a seek
to forbid, a pipe or a socket having none.

**The two flags do not arrive by the same route, and the asymmetry decides who
confers each.**

- **Append-only rides the open file description.** It therefore crosses the socket
  with the descriptor, and the opener confers it once. `weaver-admin` opens the
  file sink append-only and the property arrives at the worker already set.
- **Close-on-exec rides the descriptor.** It therefore does not cross, and the
  receiving call is the only place it can be supplied. Admin cannot confer it on a
  descriptor it hands away, so the harness supplies it at its own receive.

**Close-on-exec is set and not checked**, and the distinction is load-bearing
rather than pedantic. A step that finds the flag clear and reports rather than
repairs leaves the descriptor inheritable anyway, so a validating receive would be
that step and would buy nothing. An earlier draft promised exactly that validation,
which would also have required `fcntl` through a dependency the recorder's manifest
does not carry, so the promise and the dependency set contradicted each other
before the doctrine settled which one was wrong.

### What a compile-time pin can and cannot reach here

**What is pinnable is the shape and not the flags.** A behaviour on the receive
path is not a type property, so no pin reaches either flag directly. What a pin
does reach is the receive site: one of them, taking no flag argument, returning a
handle the rest of the crate cannot construct another way. The flags themselves
take the perturbation-verified test, and the test counts only because it was
watched failing when the flag was removed.

### The threat walk

The adversary is not an outside attacker. **It is the agent reaching its own
account through a tool it elected**, which is the one adversary this design is
obliged to survive, since the agent is inside the boundary by construction.

The walk runs like this. A tool subprocess wants to edit the record. It does not
inherit the handle, because the descriptor is close-on-exec and was set so at the
receive. It reads `/proc/self/fd` and learns the sink's path, which it is always
free to do. It then wants a second handle to that path, and it opens nothing,
because the agent uid has no permission on the file. If it could reach a call in
the recorder that took a path it would not need permission of its own, and **there
is no such call**: no path-taking constructor, no path stored anywhere in the
crate, nothing to disclose and nothing to reopen.

The attack fails at the point where it needs the program's own cooperation, and
that is the whole of the mechanism. The compile-fail set is what holds it, since
the absence of a path-taking surface is a type property and a runtime test
structurally cannot pin the absence of one.

## 7. What holds state, and for how long

**The agent is proto-stateful, which means it holds real state within a session
and none across sessions.** It begins each session with no accumulated experience:
no belief graph, no consolidation, no recall of an earlier session, and no memory
substrate of any kind. Conversation context within a session is not agent memory,
and that distinction is the whole of what this program defers.

An earlier vocabulary called this stage stateless, which the ruling of 2026-08-01
corrected as an overstatement. **An agent holding a working structure and a hot
cache is not stateless. It is an agent whose state dies with the session**, and the
weaker word made the design sound like an absence when it is a boundary.

Three things hold state inside a session. They are not three things of a kind, and
what distinguishes them is what their loss costs.

### The working structure, which the agent reasons over

The run's trace events, held in RAM in the same canonical form the stream carries.
There is no second representation and no relational projection, that projection
having been retired by the same ruling of 2026-08-01.

**Lose it and turn two has nothing to be about.** It is the state the reasoning
runs on rather than a cache of anything, which is why it is the one holder whose
loss is fatal to the turn rather than expensive.

It is volatile by construction. The program rebuilds it from nothing rather than
from a record, the stream's accumulation belonging to the operator, and it is
working memory rather than a store. **Every run begins empty**: the structure
starts with nothing, the run's first authored event is its `load`, and there is no
resume path, no projection of prior history, and no reconstruction from anything.

### The hot cache, which is an optimization

The same content precomputed at the decoder. **Lose it and the agent is slow
rather than absent**, which is the whole difference between it and the structure
above.

It is also **the one surface in this program holding state that nothing could
reconstruct even in principle**, which is why the apex singles it out by name and
assigns its ownership rule to the SPU's charter rather than to the crate that
benefits from it. The rule has three parts. **The SPU owns the cache. The harness
owns the flush decision.** And **the harness is forbidden to touch it**, holds no
handle to it, and therefore cannot protect or corrupt any region of it.

That third part is the one worth pausing on. Because losing the cache costs real
compute, it is the surface most likely to grow quietly into session state if the
line is not drawn, and the line is drawn by withholding the handle rather than by
asking the harness to be careful with one.

**The cache ends with the residency and never outlives it.** A release frees the
device, the cache is on the device, and so nothing survives a release to be
reattached to a later admission.

### The session custodian, which is behind a socket

`weaver-state` holds the session's records in sqlite, inside its own process,
reached over a credential-checked Unix socket. One member instance serves one
session. Its process retires at each unload while its holdings stand for the next
run, so **nothing it holds needs an identity it minted itself and nothing survives
the session's close.**

The store is embedded rather than a service, and the election stands on this
workshop's own measurement rather than on preference: an in-process query is a
function call, while a service on loopback pays a round trip per ask.

**The seam is a member seam rather than an organ channel.** The harness asks and
the custodian answers, where an organ channel has either party able to open an
exchange. The ask vocabulary is closed and holds two names.

- **`shape`** carries no members and asks what happened, in what order, in which
  run. The answer carries the session's runs in the order custody first saw them,
  each with its run reference and its held event counts by kind, every name spelled
  as the envelope spelled it. **The counts are organized envelope fact and carry no
  judgment.** What a kind's count means to a turn is the asking loop's business.
- **`recall`** returns the conversation as custody holds it, added 2026-08-19
  against the context-management loop's need. After a flush the decode context is
  empty and the session's knowledge is not, so the loop asks for the material and
  composes its own re-entry. It carries one optional member bounding the answer to
  the most recent turns, absent meaning the session whole. What comes back is the
  four message kinds in landing order with each envelope whole, **the distillate's
  own shape served back, no more recallable than it was distillable.**

### One accounting note this report owes

**The apex says two things hold state across turns inside one session, and the tree
now holds three.** The apex enumerates the working structure and the hot cache.
`weaver-state` was chartered 2026-08-18, after the set ratified, and holds across
runs rather than merely across turns, which is more than either of the two the
apex names. The enumeration has not been amended. This report states both rather
than choosing, on the same footing as the crate roster of section 2, and appendix
B carries it as owed.

## 8. The model organ

**Residency is the whole of what the SPU publishes about itself**, and stating it
that narrowly is what keeps the organ from becoming a second harness. Residency is
the device-side fact that one model's weights are present and ready to serve,
established by an admit and ended by a release, and it is what the two exchanges of
the organ channel move between.

**What the crate owns is wider than what it publishes**, and the two are not one
claim. The hot cache of section 7 is the other holding, owned here and flushed on
the harness's terms. Residency is the published fact. The cache is state this crate
holds and publishes nothing about.

### Admission is the one check on the device

Nothing upstream weighs the device. Admin's concern at load time is the
configuration file and what it points at, so **a device conflict is discovered at
model admission and nowhere earlier**, and the SPU is its one authority.

**Admission refuses and never evicts.** A conflict is rejected until the operator
explicitly unloads the occupant, so no load, at any point in its sequence,
auto-evicts another agent. That is a fleet property rather than a courtesy: an
agent that could be displaced by another agent's load has no residency it can rely
on, and every measurement taken through it inherits the doubt.

### The five steps, and the point past which a refusal stops being free

1. **Resolve the binding to an artifact.** A binding naming a model the crate
   cannot find is refused before the device is touched.
2. **Read what the artifact declares about itself, without loading it.** The header
   and metadata block answer what family this is and what its dimensions are,
   with no tensor data read and no device touched. This converts the most common
   shape of a bad binding, an artifact that is present and wrong, into a refusal
   that costs no device work at all.
3. **Judge the assigned devices and the readout election.** The binding names the
   devices and this crate judges them and selects none. What the artifact's shard
   needs plus the working headroom the residency requires must fit what each
   assigned device has free, which is one inequality read per device rather than a
   new one. Where the set is larger than one, the devices must also be able to
   reach each other, since a sharded forward exchanges activations between them and
   a set that cannot reach is a set that cannot serve. The election is judged here
   too, against what the family's engine declares, and it costs no device work
   because the family is already known from step 2. **An election the engine cannot
   honour therefore refuses before any device is taken.**
4. **Take the assigned devices and load the weights.** Where the set is one this is
   one take and one load. Where it is larger, the takes and loads run in the
   binding's shard order, so a failure partway is a partial take rather than none.
5. **Confirm residency.** The answer confirms and carries nothing else.

**Every step before the fourth is refusable at no cost, and that ordering is the
substance rather than a tidy arrangement.** A refusal reaching the harness before
any device work has happened is one the enter fan-out unwinds cheaply. Step 4 is
where that stops being true, and a load failing after any device is taken is the
case the rollback path exists for, whether one device was taken or several.

### Two engines, as peers

The two decode engines are peers rather than a legacy and a target. **The backend
is decided by the artifact rather than by configuration**, and there is no knob
selecting one.

**How many engines there are is a consequence rather than a policy:** one per
artifact container this program writes an engine for, the container being a
property of the artifact read at step 2. **A further engine arrives with a
container and never with a protocol.**

That last clause is the reason a delegating backend is refused, and the ground is
authority over the device rather than the shape of a dependency. A separate serving
process runs its own admission, holds the device it placed the weights on, and
answers from a residency this crate never judged. **A crate that delegated would be
a client of the device's authority rather than the authority itself**, and the
conflict step 3 exists to refuse would be refused somewhere this crate cannot see.
The weights hash, the family's marker discipline, and the measurement that rides a
generation would all move behind that boundary, each becoming a number this crate
repeats rather than a fact it holds.

### What the device judgment reads, which is not yet ruled

The plan for this section once promised an argument for why the device judgment
reads the driver rather than the crate's own ledger. **The argument exists and the
ruling does not, and the honest form of this section is to say so.**

The argument runs as follows. The previous tree read free memory from the device
driver through a command-line tool and marked that reading diagnostics-only,
holding its own accounting of what it had allocated as the authority for admission
decisions. The two disagree exactly when something outside the program holds
memory, **which is the case admission exists to catch**, so preferring the internal
accounting is preferring the number that cannot see the thing the check is for.
That tree recorded a preference and not a reason.

What is owed is a ruling on which reading admission judges against, taken together
with a measurement of what a driver query costs on the admit path, since shelling
to an external tool on an admission path is the visible cost a reason would have to
name. **Neither the ruling nor the measurement exists, and no driver query stands
in the code.** The working headroom term is a construction parameter at the
worker's composition root for the same reason. Appendix B carries both.

## 9. Observability as an election

The instruments this program can point at a generation are not switches in a
configuration file that happen to be off. **They are elected per load, named
individually in the record, and refused at admit where the engine cannot honour
them.** Three properties follow from that arrangement and none of them survives
the obvious alternative.

### Three elections, each standing alone

The load carries an election for the **residual-stream readout**, one for the
**probability field** at a declared depth, and one for the **per-position
surprisal** vector. The field's election carries a depth because there is
something to size. The other two are flags, because their readings exist or they
do not.

**None of the three is bundled under a name for a set**, and the reason is
provenance rather than tidiness. A named set drifts as members join it, and every
record already carrying that name silently becomes a record of something else.
Naming each election individually is what stops a record's account of itself from
decaying while the record sits unchanged on disk.

### Absent, false, and true are three states, and the shape keeps them three

The field's election is recorded as a depth that is present or absent, because
**absence is the whole of what an unelected field has to say.**

The surprisal's election is recorded as a flag serialized even when it is false,
which is deliberately the opposite choice, and the difference is worth the space
it costs. A record whose operator declined the vector and a record written before
the election existed at all are two different records, and only a flag that
renders when false tells them apart. Collapsing the two would convert a decision
into a silence, which is the same defect the elections exist to prevent.

### The record declares the posture it was written in

Every election of a load is named in the `load` event. Without that, **a record
holding no field readings is indistinguishable from a record whose election stood
and produced nothing, which is a configuration and a fault wearing the same
absence.**

### An elected diagnostic changes no token

This is the obligation the whole arrangement rests on. **A diagnostic that changed
the run it observes would corrupt every use of it, and would do so invisibly.** So
the requirement on the SPU is stated as an equality a test can hold: the same
declaration and the same seed produce the same token sequence with the election on
and with it off. It is carried by a perturbation-verified test rather than by
review, which is the only instrument that can hold a behaviour of this kind.

### The cost is accepted rather than mitigated

A field at useful depth costs decode time at every position and grows the record
by an order of magnitude. **The answer to that is that it is off unless someone
wants it, not that it is made cheap enough to forget.** The same rule is owed to
the residual capture, which is larger by further orders of magnitude and arrives
as its own election under this pattern rather than as a widening of this one.

### An absence has no shape

Where an election did not stand, **no empty vector crosses per position and no
member sits null. The message is not sent.** A consumer counts what was produced
rather than filtering what was not, and why the election did not stand is the load
event's to say rather than the decode seam's.

### What carries no election, and why that is not the same as always

The entropies and the generation's perplexity carry no election. They are produced
wherever the distribution is there to read, **which is not the same as always.** A
backend serving no distribution measures nothing, and the generation still answers,
on the rule that an absent observability is a reported absence rather than a fault.

So the two cases separate cleanly. An unelected reading is absent exactly when
nothing was measured. An elected one is absent when either its election did not
stand or nothing was measured, and what an election adds is a second reason a
member may be missing rather than a different kind of missing.

**The perplexity's accumulator holds no vector.** The mean is carried forward per
position and the terms are discarded as they are used, which is what makes the
unelected posture a smaller computation rather than a withheld reading. Where the
surprisal election also stands, both render, and **a consumer that finds them
disagreeing has found a defect rather than two readings.**

### Where an election refuses

The election is judged at admit, in step 3 of section 8, against what the family's
engine declares. The family is known from the header read at step 2, so the check
costs no device work and **an election the engine cannot honour refuses before any
device is taken.**

That path is live today rather than theoretical. **The GGUF container has no
residual readout tap**, so an elected readout against a GGUF artifact refuses at
admit by name. The native tap stands. Appendix B carries the gap, and it is the
entry there nearest to closing.

## 10. Replay, and what it is not

The trace is not only the record of a session. It is the source from which a
session can be re-run under close observation, and that is the second reason the
record is the primary artifact rather than a diagnostic.

**The loop is stochastic and does not reproduce, and this program makes no
run-again claim, in any arrangement.** Replay does not need one, and the rest of
this section is about what it does need instead.

### Three arrangements, and they are not ranked

**Re-analysis over the frozen record** is always available, because everything
produced is recorded. It claims nothing beyond the record itself.

**Deterministic re-feed** is available when the record holds the token path.
Because the trace records the sampler's actual tokens rather than a seed, a
recorded scenario is replayed by feeding the recorded token sequence back through
the forward pass. **Nothing is re-sampled.** The residuals are deterministic given
the same weights, within GPU float tolerance.

**Stochastic re-entry from the same starting field** is available where the binary
declares one, with the setup surface frozen at the worker's composition root and
seed and sampling parameters baked immutable. **A frozen seed narrows variance and
buys audit rather than determinism**, which is why the run-again claim stays unmade
here too.

On this corpus the first two are always available, every record holding the token
path. The third alone waits on a binary's declared disposition.

### Why tokens rather than a seed

The seed is recorded, and it is recorded so a reader can check that the stream came
out where it should. **This program offers no route that accepts it.** A sampler is
built from its declared inputs and from nothing else, so reading a seed back as a
substitute for the token path would be a different operation than the one on offer,
and it is one nothing here performs.

The distinction matters because a seed is a claim about a mechanism and a token
path is a record of an outcome. A seed replays only if every intervening draw is
also replayed, on the same sampler, in the same order, at the same version. A
token path replays because it is what happened.

### Completeness is claim-relative rather than a fixed bar

Deterministic re-feed requires exactly five inputs, and the list is closed:

1. input token identifiers,
2. output token identifiers,
3. model identity with its weights hash,
4. sampling parameters,
5. the prompt-block partition,

with tokenization reproducible from what is recorded.

**A record claiming a replay arrangement carries everything that arrangement
requires, and a replay missing an input its claim requires observes a forward pass
that never happened** - which is worse than no replay, because it produces a result
that looks like evidence. A deployment claiming only re-analysis owes nothing
beyond the record, because it claims nothing more.

### The template problem, and why canonical messages are insufficient

The harness sends canonical messages and the family library renders them through
the family's template into what the model actually sees. Those are two different
objects, and **a record holding only the first cannot reconstruct the second.**

So the rendered reality returns on the report path: the template's identity, the
token identifiers, and the block partition. The record therefore holds the mapping
from the canonical conversation to what the model saw, rather than the conversation
alone plus an assumption that re-rendering will land in the same place. It will not
reliably, since a template is versioned software and a re-render under a later one
is a different prompt wearing the same messages.

### The seed derives per generation, as of 2026-08-21

An earlier claim of re-enterability was false of its own mechanism, and the
correction is worth recording rather than quietly absorbing. One sampler stood for
the whole residency, every draw advancing a single stream, and a flush reseeded
that stream while clearing the penalty window.

**A generation now draws from a seed derived from the run's seed, the turn, and
which generation of that turn it is, and the sampler holds nothing across
generations.**

What the ruling removes is narrower than it may sound and is worth keeping narrow:
it removes the need to replay every draw that preceded a generation in order to
reach its stream. **That is a smaller thing than replaying a run, and the program
still claims no run again.**

What it buys is a paired comparison. **Two arms of one comparison now meet each
turn at the same stream**, so where they differ, the difference is the treatment's
and the context's rather than the sampler's position - which is what such a
comparison needed and could not have while one stream ran the length of a
residency.

**Both seeds are recorded.** Under derivation the effective value is two values,
the declared and the derived, and neither alone answers what a reader needs.

### The driver stands outside the agent

Custody places the replay driver outside the agent, because the agent must not own
or even read its own trace. A tool that reads the operator-held stream and drives
the SPU therefore runs as an operator principal, over the operator's own storage.
Replay is something done to a record rather than something an agent does.

**One honesty note.** Deterministic re-feed is built and not yet demonstrated. The
demonstration on record is of the third arrangement, stochastic re-entry from a
frozen starting field, which is a different claim. Appendix B carries the
difference.

## 11. The lifecycle

**Loop 0 is the running agent service.** It is not a document set, not a milestone
to reach and pass, and not a loop a builder supplies. It is the object itself: the
thing that boots under its unit, comes up as the statically provisioned agent
identity, binds the coordination socket inside its own sandbox and listens, creates
the unnamed pairs its organs are reached over, and sits there being one sealed
agent.

**The numbering runs from there.** Loop 0 is the framework's own and is not
supplied through any builder-facing surface. **Loop 1 is the builder's**, the
reasoning loop written against the machinery loop 0 provides, and loops above it
are further builder loops. Both builder-facing surfaces - the extension seam
compiled into the worker binary, and the socket the working list holds open -
reach loop 1 and above and exclude loop 0. The harness owns the lifecycle interior
of loop 0, the load and the unload, which are the same for every agent, together
with the engine machinery a loop is built from.

**The harness binds exactly one name and listens on exactly one socket.** Any
socket connecting to the harness is an internal connection, so the coordination
socket lives inside the sandbox and the harness is the party that creates it. It is
dialable by construction and **the credential check is what refuses**, root or
refused. An earlier form had unreachability do the refusing, which could not tell
an elected tool from the worker. Nothing else is dialable, the organ pairs having
no names at all.

### Four parties over three sockets

The load is a fan-out, and the shape of it is what the rest of this section
explains. `weaver-admin` runs as root, one invocation per verb, and holds a
channel to the harness and to neither organ. The harness holds the coordination
socket inward from admin and an unnamed pair outward to each of the two organs
it forks. **So four parties are joined by three sockets, and admin reaches the
organs only through the hub.**

That is why what admin receives is one answer aggregating the fan-out rather than
three answers it must reconcile. Admin asks once and learns ready, or a refusal
naming where it stopped.

### The sequence, and what refuses a load

Admin verifies the boundary rather than building it, resolves which session is
being loaded, and opens the sink under root. It then asks the init system to start
the worker as a transient unit carrying the agent's `User=`. **The worker starts
bare of descriptors**, and its first act is to bind the coordination socket inside
its own sandbox and listen.

Admin then dials that socket, retrying within a stated bound because the bind is
the worker's first act and the dial may arrive first, and sends the enter
directive. Everything after the directive and before the answer is the harness's.
It stands up an empty working structure, authors its `load` event, asks the SPU to
admit the model, and **starts the gate last so no work arrives before the interior
can serve it.**

**Gate last up, gate first down.** The agent's front door is the last thing opened
and the first thing closed, which is what makes the interval where the door is open
and the interior is not serving structurally empty rather than merely short.

A refusal at any point in that ask enters rollback carrying the name of the step
that refused. **A device conflict discovered at model admission is such a
refusal**, named by the SPU inside the aggregate, and admin holds no earlier check
that could have caught it.

### Loaded and idle is a first-class state

The four agent states are `Absent`, `Unloaded`, `Idle`, and `Active`. **`Idle`
means resident and interruptible, ready for the next run**, and it is published
only on a ready aggregate.

**A partial load is never published as loaded**, and the published state is idle
rather than active. That distinction is the point: an agent that is up and holding
its model without work in flight is a state the operator can act on, and a
lifecycle that recognised only running and stopped would have to invent it or
conceal it.

### What the supervisory half does not record

**Everything admin does before the enter directive produces no trace entry, and
neither does the rollback path.** That is a ruling rather than an omission. The
first run's `load` is the record of admin's first contact, and the worker start and
the descriptor handoff necessarily run before the harness exists to author
anything.

The reason it stays that way is the single-writer property. **A second party
writing into the stream would end the property that makes the account evidence at
all**, so the supervisory half of the lifecycle is unrecorded by the trace and its
record is admin's own log.

### The lifecycle refusal reaches the record

Where a lifecycle ask is refused, the refusal is clerked in the same kind every
other seam's refusal is clerked in, per the ruling that one kind serves every seam.
The `refusal` event carries the lifecycle seam's own case beside the ask it
answered, and where the refusal ends a turn the close carries `Refused`.

**The observation exchange is the gap here**, and it is a real one. `show` and
`list` refuse today, because the init system's three unit values do not map onto
these four agent states: the manager's `active` covers both `Idle` and `Active`,
and its `failed` has no agent-state case at all. **A translation is exactly where
invention would enter**, so the refusal stands until an observation exchange
reaches the party that holds the run. Appendix B carries it.

## 12. The boundary

The gate is the agent's edge in both directions. Work arrives through it, the
deliverable leaves through it, and since the egress ruling of 2026-08-07 the
agent's own outbound tool traffic crosses it as well rather than bypassing it.

**It carries two sockets, split by which party opens an exchange.** The world
socket is dialled by the world. The second is opened by the agent, admits
registered applications that bind a listening port, and **no exchange reaches it in
this pass.** A reader should take it as the shape an external tool will arrive by
rather than as a path anything travels today.

**The gate relays and does not read.** It authenticates the peer, preserves order
per connection, and carries the line without parsing it. That is what lets the
boundary be described independently of what the agent happens to be saying.

### The port test

The question of what counts as internal is decided by a mechanical test rather
than by intuition. **A tool that binds a listening port is external.** It is a
registered application the operator provisions, one this program forks none of, so
no uid of its is this program's to choose. Loop code the operator compiles into the
worker holds no separate process and therefore no separate uid, so it is not
external by this test either.

**Two boundaries give two answers about the shell, and they do not contradict each
other.** The shell this crate forks per call is internal to the agent, which is the
port test's answer. Under the reasoning-loop criterion of 2026-08-11 the same shell
is external to the loop, that criterion filing every tool outside the loop without
exception. The two answers name two different boundaries. **It was a refiling
rather than a reversal**: nothing left the agent, and what changed was which
boundary the tool is filed against.

A third thing sits outside both. `weaver-internal` holds inward-dispatched
callables and no process at all, so no uid question reaches it, and the calculator
is dispatched there rather than through the gate.

### One tool, held as a verb rather than as a table

**This crate holds no tool table.** It holds one tool, the shell, and holds it as
its own outbound verb rather than as a member of a roster. The execution exchange
resolves against that one name directly.

**A name that is not the shell's refuses by name, never a nearest match**, and the
refusal to guess is the substance. A nearest match on a tool name is a guess about
intent, executed with the agent's privileges, at the point where the program has
the least context and the most authority.

The agent's wider roster is emergent rather than enumerated: scripts the agent
writes and keeps in its home directory, reached through the shell, owned by the uid
and members of no crate. The program does not know them and does not need to.

### Supervising the shell

The invocation forks the shell into its own process group and supervises it against
the caller's clock. **Past that clock the kill reaches the whole group**, because a
shell leaves descendants holding the pipes open and killing the leader alone leaves
them there.

Both pipes are drained bounded and concurrently, since **a pipe left unread turns a
chatty command into a false kill** - the command blocks writing, the clock expires,
and the record says timeout where the truth was a full buffer.

### A tool result has exactly one construction site

The gate guarantees an answer to every execution opened inside the raised window,
carrying one of four contents. **So the harness's result construction site fires
exactly once per opened call, and no call inside a raised window dangles.**

That is a stronger property than it looks. With one construction site there is no
path on which a call quietly yields two results, and none on which a live gate
leaves the loop waiting on an answer that is not coming. The kill clock is the
caller's own number rather than one the gate chose, and the guarantee is the seam's
rather than a convention both sides observe.

**The guarantee is scoped to a live gate, and the other case is not an answer.** A
gate that dies with an execution open supplies none of the four contents. What the
harness observes instead is closure, and **closure is not an answer** on this seam.
After the enter aggregate, that death is the loss of the agent's reachability,
observed through closure and authored to the stream as a `fault`, which is where
the operator's tooling keys on it. So the exactly-once property holds inside the
window rather than across the gate's own failure - which is the same boundary
section 4 names from the response side when it says a gate dying mid-delivery loses
the delivery rather than the turn.

### No safety classifier exists, and none is planned

The previous tree made per-invocation safety classification its flagship
enforcement type: every tool was obliged to answer whether a given invocation read,
mutated, or destroyed, and the answer drove how calls were batched. **That is gone,
deliberately, and it is not coming back.**

The ground is that both available readings of where containment lives put the
judgment somewhere other than in a method the tool answers. Under the first, a tool
executes as the agent's constrained user, bounded by filesystem permissions,
sudoers, and cgroups. Under the second, it is a registered application the agent
addresses across a socket, whose containment is the application's own and not this
program's to state.

**A trait method asking a tool whether it is dangerous is a heuristic standing
where a boundary already stands**, and its presence would invite the belief that
the answer is load-bearing. The absence is the claim.

## 13. The builder's seat

A builder writes loop 1 and inherits loop 0 whole. That division is the subject of
this section, and most of what looks like a constraint in it is there to make a
later measurement mean something.

**Two surfaces reach the builder and both are limited to loop 1 and above.** The
extension seam is composed against and compiled into the worker binary. The socket
the working list holds open is dropped in beside a running agent. Loop 0 is the
service that runs a builder's loop under either surface and is not itself supplied
through either.

### The seat and its ports

The seat is the surface loop 1 is written against, and it is a set of ports the
harness grants rather than a library the loop links. The decode surface and the
invocation are the original two. The flush, the elision, the classify ask, the
state seam's shape and recall, and the context ports each arrived later, and each
arrived **as a charter and contract edit rather than as an import** - the
capability change entering through the front door, with the Spec clause and the
contract's serve section landing in one act.

The elision port is the clearest case of what the seat is for. It takes a span and
forwards it to the directive unjudged. **Which span to elide is the loop's election
and the harness holds no policy about it**, because a port that judged a span would
be the switchboard deciding what a context is worth. The mechanic is the harness's.
What to keep is the operator's, written in the loop that operator owns.

### A disposition on every knob

**Every sampling knob carries a disposition, and the seed is a knob.** Each is
either frozen at the worker's composition root or left operator-tunable, per knob,
at the builder's election.

**The effective values are recorded whichever side set them, because a disposition
changes who sets a value and never whether the record holds it.** That sentence is
the whole mechanism. A record does not become thinner because a builder froze
something, so a reader never has to know a binary's dispositions in order to read
what a run actually ran with.

The prior program never made its seed configurable at all, so the disposition
mechanism is the seed's first real home.

**Re-entering a generation takes the seed, the surface, and the state.** A reader
supplies the declared seed, the turn, and the generation's index, which fix the
stream, and then the rest of the effective sampling values beside them, **because a
stream is only half of a draw**: the temperature and the truncating filters decide
what the stream selects and the penalty pair decides against what. A re-entry
differing in any of them draws the same luck through a different filter and answers
something else. The same resident tail and the same weights stand behind all of it.

Today every knob but the seed is frozen at the composition root, so a re-entry on
the same binary carries them without asking. The record holds them anyway, and that
is what makes them re-suppliable when a binary changes its mind about which are
frozen.

### Why the loop is compiled rather than configured

The loop is compiled into the binary rather than read from a file at runtime, and
**what a builder inherits is an array they did not choose at runtime.**

The reason is attribution. A binary that cannot change its loop between runs cannot
be the explanation for a difference between two runs of it. Freeze the loop and
freeze the sampling surface, and **what varies is held to a range narrow enough
that what remains is attributable to the thing under study rather than to the rig.**

That is a claim about the apparatus and it is checked rather than asserted. Section
14 reports the A/A test that bounds the rig's own contribution, and the bound it
produces is the honest form of this argument: a number, above which the machine
cannot be hiding a difference, rather than a promise that the machine contributes
nothing.

### The clock, stated rather than argued

**There is one clock.** The load boundary is an event on that clock rather than a
second tempo running beside it. **No in-RAM mutation of behaviour is supported in
any path**, so the load boundary is the only boundary at which behaviour changes.

What that buys is attribution again, from the other side. A run's behaviour is
fixed at its load, so a difference appearing within a run has nowhere to come from
except the content, and a difference appearing across runs is a difference of
loads, with both loads on the record and both declaring their elections by name. A
second tempo would put a third possibility between those two, and every reading
taken through the apparatus would have to rule it out.

**This section states those mechanics and does not argue them.** The argument, and
the alternatives it rules out, belong to the reasoning-loop formalism, which sits
on an open pull request rather than in the tree. **That citation is therefore owed
and resolves to nothing a reader holding this commit can open**, which appendix B
carries as this report's debt rather than the code's.

## 14. The apparatus in use

The measurement regime stands outside this repository. It registers each test with
its method before it runs, and it carries with every result the conditions that
make it comparable: the commit, the build profile, and the identity of the binaries
measured rather than the profile's bare claim.

**Standing outside the repository makes this the one numbered section whose own
sources a reader holding the tree cannot reach**, which is said here rather than
left to be noticed. Section 13 is the narrower case and not a second of these: its
own claims are checkable and one citation it makes is not, the clock argument
sitting on an open pull request. Appendix B carries all three, appendix C being
uncheckable on a third ground.

### The A/A test, and what it does and does not show

The result this section rests on is an A/A test. Two agents identical in model
artifact, declaration identity, loop file, and calculator, differing only in which
device they held and which seeds they drew, each ran one hundred rounds of a
four-turn task scored against fixed numeric answer keys. **The grader is a key
comparison rather than a second stochastic instrument sitting inside the test**,
which is the property that keeps the measurement from needing its own error term.

Alpha scored 2.890 of 4 with a standard deviation of 0.859. Bravo scored 2.730
with 0.847. The difference is 0.160 against a standard error of 0.121, giving
t = 1.33.

**That is a failure to detect a difference and not a demonstration that none
exists**, and the distinction is this section's to make rather than a reader's to
supply. An underpowered A/A test passes trivially, and a reader who knows the
statistics will say so before they trust anything else here.

What the numbers support is a bound rather than a null. At this sample size and
these standard deviations the test would have caught a difference of about 0.34
points with eighty percent power, and the ninety percent confidence interval on the
observed difference runs from -0.04 to +0.36. **So the rig's own contribution to
the score is bounded above by roughly 0.36 points on a four-point scale rather than
shown to be zero**, and a treatment expected to move the score by less than that is
not yet separable from the machine.

**The unit of observation is the round and not the turn**, one hundred per arm,
because turns within a round are strongly dependent: every one of the fifty-seven
rounds that answered turn two went on to convert turn four. Counting turns would
have inflated the sample fourfold while adding almost no independent information,
which is the error this design is arranged to avoid rather than one it happened to
miss.

### Two instruments earning their place

**The measurement payload is not decoration.** The model's own per-token surprisal
correlates with its score at r = -0.300 over two hundred rounds, which is about
nine percent of the variance. That is a signal rather than a predictor, and calling
it the second thing would be the overclaim available here.

**Forty recorded runaways re-run against a raised generation ceiling** with the
same declared seeds came back byte-identical to the original until the old ceiling,
and differed only after it.

That result demonstrates the third of the three unranked replay arrangements,
**stochastic re-entry from a frozen starting field, and not the deterministic
re-feed section 10 argues for.** Those generations were re-sampled and matched
because the seed and the sampling surface were the same, where a re-feed pushes
recorded tokens back through the forward pass and re-samples nothing. **The re-feed
is owed its own demonstration**, and appendix B holds the debt.

### The negative results are kept beside the positive ones

Two hypotheses built on the entropy signal died on the evidence. There is no branch
point, and no in-flight predictor worth the name at 59.5 percent against a 51
percent base rate.

They are reported here for the same reason the bound above is reported instead of a
null: **a regime that publishes only what worked is not a regime**, and the two
dead hypotheses are the evidence that the discipline is running rather than being
described.

### One caveat this section owes about itself

**The equivalence bound above is computed here**, from the reported means, standard
deviations, and sample sizes. It is not a pre-registered power analysis. A series
designed against a target effect would state the bound before running rather than
after, and appendix B carries the difference.

## 15. What stands, and what does not

**The demonstration is the claim, not the count.** A crate can report a high
conformance figure while completing no turn, and a completed turn is not a count of
claims met. So this section leads with what the assembly does and treats the
numbers as context for it.

### What stands

**Both engines serve.** The native decoder and the GGUF decoder are peers, the
backend decided by the artifact rather than by configuration, with the sharded
native pair standing beside the single-device path.

**The trace runs over its turns.** The record brackets session over run over turn,
strictly nested, one line per event across the closed kind set, with the elision
and the refusal both reaching it as of 2026-08-22.

**The series section 14 reports stands**, and this section does not tell it a
second time. What it adds is only the reading: the rig's own contribution is
bounded rather than absent, and the bound is the number a later treatment has to
clear.

### The numbers, as facts to read rather than targets

The corpus holds **285 assertion records across nine Specs**. They divide by
instrument as 130 review, 87 perturbation, 28 compile-pin, 26 manifest, and 14
compile-fail.

**Fifty-nine source files carry a conformance header, and between them they cite
258 of those 285 assertion nodes.** Twenty-seven are declared and not yet cited by
any header, and the open set includes the ones appendix B already names by hand -
`harness-idle-report-authors-without-a-turn`, `spu-one-forward-per-prompt`, and
`spu-field-changes-no-token` among them.

**That figure is a fact to read and not a target to reach.** An assertion that no
code cites yet is a piece of work not done, and driving the number up by loosening
what a citation means would convert a measurement into a decoration. The tags carry
the same discipline from the other side: **`review` must mean an instrument was not
bought, never that none exists**, because the inverse overclaim forecloses tests the
corpus may later want.

The comparison worth drawing is with the previous tree, where 25 files carried a
conformance header and cited seven distinct spec node identifiers between them.
**The structure there was right and the edges were never drawn**, which is the
failure this program's arrangement is built against rather than a fault of the idea.

### What is deferred, each through a named door

**Client-facing streaming** arrives as an extension to the world contract rather
than as a replacement for it. One line in and one line out is the resting shape.

**The status ask** waits on the observation exchange. `show` and `list` refuse
today because the init system's three unit values do not map onto the four agent
states, and a translation is where invention would enter.

**The memory leg** is out entirely and arrives as a socket peer with its own
contract. **No seam, stub, reserved slot, or dormant contract party is carried in
anticipation of it**, which is the discipline that makes the door real rather than
decorative. A reserved slot is a design decision taken early and disguised as
neutrality.

### What the trade costs

The program buys its Level A separation with real losses and does not net them out.
There is no fungibility across machines, no distributed uptime, no failover, and no
direct network applicability of the deployed whole. The seams are Unix-specific in
code, and descriptor passing in particular has no wire analogue, so a wire seam
would need a different custody design rather than a port of this one.

What carries upward is the topology and the contracts, which are written against
what crosses a seam rather than against how it crosses. That is a genuine asset and
it is not the same thing as portability.

**The program's own judgment, stated plainly: a local proto-stateful agent is a
defensible intermediate and an indefensible end state.** It is defensible now
because separating the three levels is what makes a failure attributable, and
nothing available at a distance does that today. It is indefensible as a
destination because an agent whose state dies with the session cannot accumulate
anything, and accumulation is the whole of what the memory leg is for.

**The trace is what connects the two.** It is the primary artifact rather than a
diagnostic precisely because the leg that comes next consumes finished traces
rather than standing beside the loop. Build the record wrong and the memory leg has
nothing to be built on. That is why the recorder was designed against what a later
reader will need rather than derived from what the loop currently demands, and it
is the one place in this program where demand-derivation was deliberately refused.

---

## Appendix A. Sections not yet drafted

**Discharged 2026-08-22, and kept rather than deleted so that appendix B and
appendix C keep their letters.** Every section this appendix planned is drafted
above. What was owed here is now owed by those sections, and what they cannot
stand behind is in appendix B.

---

## Appendix B. What is not built, not proven, or not measured

Every case the report describes and cannot yet stand behind, in one place, so a
reader does not have to find them by reading closely. Each says which of the
three it is. This list shrinks as the work lands and the report is refreshed
against a later commit, and an entry that leaves it does so because something was
built and shown rather than because the wording softened.

**Not built, and chartered.**

- **The gate's agent-opened socket.** Chartered for registered applications that
  bind a listening port. No exchange of the harness-gate seam reaches it, and its
  contract is the tool workflow's to author. Section 4 says so at the point of
  description.
- **The GGUF residual readout tap.** The fork's eval callback is pinned and the
  pin is bought by a compile-fail doctest, but nothing drives a tap through it.
  The native tap stands. An elected GGUF load therefore refuses at admit by name.
  Two assertions wait on this, `spu-two-taps-one-shape` above all. **Two pull
  requests are open against this entry** and it is the one here nearest to
  closing, which is said so that a reader meeting it after they merge knows to
  distrust the date at the top rather than the work.
- **The idle report.** No report authors without a turn because nothing authors
  one at all, which is what `harness-idle-report-authors-without-a-turn` waits on.
- **Client-facing streaming.** Deferred, and it arrives as an extension to the
  world contract rather than as a replacement for it. One line in and one line out
  is the resting shape.
- **The status ask.** `show` and `list` refuse today, the init system's three unit
  values not mapping onto the four agent states, and a translation is where
  invention would enter. The observation exchange retires the refusal when it
  lands.
- **The memory leg.** Out entirely, arriving through apex section 9's door as a
  socket peer with its own contract. No seam, stub, reserved slot, or dormant
  contract party is carried in anticipation of it.
- **Any seam over a wire.** The transports in code are Unix-specific, and a seam
  crossing a machine boundary would need its own framing and a peer-authentication
  mechanism to replace `SO_PEERCRED`. Neither exists. **Descriptor passing is the
  hard case of the three and is named separately for it.** `SCM_RIGHTS` has no
  wire analogue: a descriptor is a capability the kernel hands across a local
  socket and rechecks at no point afterwards, and there is nothing to send over
  TCP that is the same kind of thing. Custody rests on that mechanism, the sink
  reaching the recorder as an already-open descriptor and the recorder offering no
  call that takes a path, so a wire seam would need a different custody design
  rather than a port of this one. The topology would carry, the implementation
  would not.
- **Shard widths beyond two.** A pair is what the salvaged tensor-parallel path
  implements. An N-way forward and its all-reduce are work this program does
  rather than salvage it inherits.

**Built, and not yet proven.**

- **Deterministic re-feed.** Apex section 8's second arrangement is what section
  10 argues for, and the demonstration on record is of the third. Pushing a
  recorded token sequence back through the forward pass with nothing re-sampled
  is owed its own run.
- **`spu-one-forward-per-prompt`.** Watchable under the standing native tap and
  waiting only on its count being taken.

**Claimed, and not measured.**

- **Latency is the enemy of agency.** The program's one conceded theory claim, and
  section 3 marks it. No per-hop figure for loopback against a Unix socket at
  these message sizes has been taken in this repository.
- **The headroom on the admit judgment.** A construction parameter at the worker's
  composition root until a measurement on a real artifact against a real device
  replaces it. Whether it is a constant, a fraction, or derived from the
  artifact's declared shape is unsettled.
- **Which reading admission judges free memory against.** Section 8 states the
  argument for the driver over the crate's own ledger and does not settle it. What
  is owed is a ruling taken with a measurement of what a driver query costs on the
  admit path, and no driver query stands in the code today.

**Owed by this report rather than by the code.**

- **Section 14 cannot be checked from the tree.** The measurement regime's
  registrations and results stand outside this repository. Until they travel with
  the release or move into it, section 14 is the one numbered section whose own
  sources a reader holding the tree cannot reach, which breaks the standard the
  rest of the report holds to. Section 13's owed citation below is the narrower
  case, its own claims being checkable where these sources are not.
- **The equivalence bound in section 14 is computed here**, from the reported
  means, standard deviations, and sample sizes. It is not a pre-registered power
  analysis, and a series designed against a target effect would state the bound
  before running rather than after.
- **The reasoning-loop formalism is not in the tree.** Section 13 cites it for the
  clock argument and it sits on an open pull request, so the citation resolves to
  nothing a reader can open.
- **`weaver-internal` is unclassified.** It fails the organ test and the submodule
  definition does not reach it, while its parent edge makes it a domain root.
  That is an apex question, and the report describes the crate without settling
  what it is.
- **The apex's roster stands at seven against a tree of nine.** Section 2 states
  both rather than choosing, and the reconciliation is the apex's act.
- **The apex counts two state holders and the tree has three.** `weaver-state` was
  chartered 2026-08-18, after the set ratified, and holds across runs rather than
  merely across turns. Section 7 states both rather than choosing, on the same
  footing as the roster above, and the amendment is the apex's act.
- **Appendix C is not checkable from the tree.** It states what an agent is taken
  to be rather than what this code does, so a build neither confirms nor falsifies
  it, and its own status line says so. The argument behind the notation sits in the
  agent paper, which is not in this repository, so that citation resolves to
  nothing a reader holding the tree can open.
- **The loop numbering is unsettled between the two documents.** The formulation
  numbers the primary reasoning loop `L_0` where this report numbers the
  framework's service loop zero and the builder's reasoning loop one, per section
  11. Appendix C states the collision and names a candidate resolution it does not
  adopt. The ruling is the apex's.

---

## Appendix C. The formulation

**Status: descriptive of design intent. Not checkable against the tree, and not
ratified.** The claims in this report are meant to be verifiable by a reader
holding the commit named at the top, and appendix B collects the ones that are
not. This appendix is among them, and on a ground the others do not share: it
states what an agent is taken to be rather than what this code does, and a
definition verifies against argument rather than against a build. It is placed
here so that the mapping in the last part of the appendix is available to a
reader who wants it and skippable by one who does not. The argument for the
notation is not made here. It is made in the agent paper (Bucy, 2026), and this
appendix states the result of that argument and the reading of it.

The notation exists in this document for one reason beyond description. Work that
comes later will need a fixed object to cite, and a formulation carried in a
released technical report at a named commit is a firmer citation than one carried
in a working draft.

### C.1 The statement

$$A = B\Big[\, (H,\, M) \,+\, \{\,T,\, V,\, \ldots\,\} ,\quad
\big\langle\, L_0,\, L_1,\, \ldots,\, L_n \,\big\rangle \,\Big]$$

Read left to right, the outer square brackets are B applied to everything the
agent is, and the three kinds of bracket inside them are three different claims
whose difference is the whole content of the statement.

**Round parentheses hold the required core.** A harness and a model, and neither
can be removed and leave an agent standing. H appears here as a part rather than
as a placeholder for everything that is not the model, which is what it was in
the coarser statements this one replaces.

**Braces hold the optional inventory, unordered.** Tools, validators, and
whatever else a builder supplies. The set is unordered because membership implies
no position, and nothing about being available to the assembly says when or
whether it is reached for.

**Angle brackets hold the loops, ordered.** The ordering is not notational
tidiness. It is the lever the builder holds in fact, and an unordered set would
misstate the one thing that is decided at build time. **Two agents with identical
inventories and different arrays are different agents**, and nothing in the
braces has to change for that to be true.

**B is the bound, and it stands outside all of it**, because the boundary is not
one of the agent's parts. It is the line that decides which parts are the agent's
at all.

### C.2 Why the bound is outside

The reason B is not written inside the bracket is an argument this report
demonstrates in code without stating. Writing the agent as a harness applied to a
model puts the outer bracket in the harness's hands, which says the harness draws
the line around the agent. **A line drawn by the harness moves whenever the
harness is reconfigured, and a boundary that travels with its contents is not a
boundary.** So the bracket comes off the harness and is handed to whatever draws
it in fact, which puts the harness inside the bound beside the model rather than
around it. Both are parts. Neither is the edge.

Section 3 is the built answer. The bound here is the kernel's: an agent uid,
filesystem permissions on the socket paths, and `SO_PEERCRED` at every accept. No
crate in this program can widen it, which is the property the argument asks for
and the reason the security paragraph of section 3 is not a preference among
equals.

**One reading has to be refused where the letter is introduced.** `weaver-gate`
is not B. The gate is a crate, an organ, and one of the nine parts section 2
enumerates, which puts it inside the bracket with everything else. **The gate is
a part standing at the bound. The bound is the kernel's.** A reader who takes the
gate for B would have this report contradicting itself on its second page, since
B is defined as not one of the agent's parts and the gate is enumerated as one.
The gate is where crossings are authorized. It is not what makes a crossing a
crossing.

### C.3 The recurrence

The loops are written as an array rather than described because what a loop does
is a relation, and a relation is written with the state on both sides of it. One
pass of the primary loop, with `y` as what the model produces and `C` as the
context assembled for it:

$$\begin{aligned}
y  &= M(C) \\
C' &= L\big(C,\, y,\, T,\, V,\, \ldots\big)
\end{aligned}$$

`y` leaves as the deliverable and returns as an argument to the next context. A
composition applied once is finished, and this is a recurrence rather than a
composition, so `C'` is not `C` and the assembly that runs a second time is not
narrowing the field it narrowed the first time.

Context is the third place of a three-place relation the core alone cannot state.
There is the part that does the managing, which is the harness. There is what is
managed, which is the model's state, a property of the model and never a term of
its own. And there is what the managing is done with, which is context, assembled
from the inventory:

$$C = H(T,\, V,\, \ldots)$$

That the model's state is not a term is worth one sentence, because this report
has a crate whose name invites the confusion. `weaver-state` is the session
custodian and holds session records. The model's state is the resident field on
the device, a property of the part and not something handed to it, which is why
what crosses the input surface is context rather than state.

### C.4 The terms, and where each is realized

The mapping is a true enumeration and is set out as one. It names design intent
against description, not a proof, and where the tree does not occupy a position
the entry says so rather than finding a nearest match.

- **H, the harness.** The job of assembling context and running the loops. It
  maps to the harness's domain rather than to `weaver-harness` alone, since trace
  authorship and session custody are submodules under that domain. Section 2 for
  the domain, step 4 of section 4 for the assembly.
- **M, the model.** The artifact and its resident state, held by `weaver-spu`.
  The organ is not M. The organ is what holds M resident and admits it. Section
  2, and section 8.
- **C, the context.** The prompt assembled at step 4 of section 4 and sent at
  step 5.
- **T, tools.** One occupant today, the shell, which section 4 describes as the
  gate's own outbound verb rather than a guest it hosts. Section 12.
- **V, validators.** **An unoccupied position.** Nothing in the tree fills it.
  The braces being an optional inventory is what permits that, and naming the
  position empty is more useful than pretending the slot is not there.
- **The array.** Section 13's compiled-rather-than-configured loop is the built
  form of the claim that ordering is the lever. What a builder inherits is an
  array they did not choose at runtime, which is what holds variance to a range.
- **B, the bound.** Section 3. Agent uid, socket permissions, `SO_PEERCRED`.

### C.5 Two defects this appendix carries

**The loop subscripts collide with the report's.** The formulation numbers the
primary reasoning loop `L_0`. **This report numbers the framework's own service
loop zero and the builder's reasoning loop one**, per section 11. Subscript zero
therefore names the sealed service on one side and the reasoning loop on the
other, and a reader moving between the two documents will be misled. The
recurrence above is written with a bare `L` to avoid asserting a numbering that
is not settled.

There is a candidate resolution and it is not adopted here. Loop 0 may not be a
member of the array at all. The lifecycle interior it owns is the drawing and
erasing of B rather than a loop the bounded thing runs, and section 13 states that
there is one clock and that the load boundary is an event on it rather than a
second tempo. Under that reading the array holds builder loops only, the
formulation's `L_0` is this report's loop 1, and the collision dissolves. **That is
a ruling the apex has not made**, and this appendix states the collision rather
than settling it.

**The formulation is a definition and the rest of this report is a description.**
Nothing here is falsified by a build failing, and nothing here is confirmed by
one passing. A reader should hold this appendix to the standard a definition is
held to, which is whether it draws its distinctions where the joints are, and not
to the standard the numbered sections above hold themselves to.
