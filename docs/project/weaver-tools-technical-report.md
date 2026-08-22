# WeaverTools - Technical Report

**Status:** LIVING. In `main` and outside the document set. **This document is
subject to change as development continues**, it is never ratified, and nothing
in the corpus is written against it. Describes `WeaverTools` at `fbcb73e`,
2026-08-21. Sections 5 through 15 are planned in appendix A rather than drafted,
appendix B carries what is not built, not proven, or not measured, and appendix C
states the formulation as design intent rather than as description.

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
attributable to the thing under study. The section states the clock's mechanics
and does not argue them: there is one clock, the load boundary is an event on it
rather than a second tempo, and what that buys for attribution. The argument, and
the alternatives it rules out, belong to the reasoning-loop formalism, which sits
on an open pull request rather than in the tree, so that citation is owed.

**14. The apparatus in use.** The measurement regime stands outside this
repository, registers each test with its method before it runs, and carries with
every result the conditions that make it comparable: the commit, the build
profile, and the identity of the binaries measured rather than the profile's bare
claim. **Standing outside the repository makes this the one section a reader
holding the tree cannot check**, which is said here rather than left to be
noticed, and appendix B carries it as owed.

The result the section rests on is an A/A test. Two agents identical in model
artifact, declaration identity, loop file, and calculator, differing only in
which device they held and which seeds they drew, each ran one hundred rounds of
a four-turn task scored against fixed numeric answer keys, so the grader is a key
comparison rather than a second stochastic instrument sitting inside the test.
Alpha scored 2.890 of 4 with a standard deviation of 0.859, bravo 2.730 with
0.847, a difference of 0.160 against a standard error of 0.121, t = 1.33.

**That is a failure to detect a difference and not a demonstration that none
exists**, and the distinction is the section's to make, because an underpowered
A/A test passes trivially and a reader who knows the statistics will say so. What
the numbers support is a bound rather than a null. At this sample size and these
standard deviations the test would have caught a difference of about 0.34 points
with eighty percent power, and the ninety percent confidence interval on the
observed difference runs from -0.04 to +0.36. So the rig's own contribution to
the score is bounded above by roughly 0.36 points on a four-point scale rather
than shown to be zero, and a treatment expected to move the score by less than
that is not yet separable from the machine. The unit of observation is the round
and not the turn, one hundred per arm, because turns within a round are strongly
dependent: every one of the fifty-seven rounds that answered turn two went on to
convert turn four.

Two further results show instruments earning their place, and a third shows the
discipline working. The measurement payload is not decoration, the model's own
per-token surprisal correlating with its score at r = -0.300 over two hundred
rounds, which is about nine percent of the variance and is a signal rather than a
predictor. Forty recorded runaways re-run against a raised generation ceiling
with the same declared seeds came back byte-identical to the original until the
old ceiling and differed only after it, **which demonstrates the third of apex
section 8's three unranked arrangements, stochastic re-entry from a frozen
starting field, and not the deterministic re-feed section 10 argues for.** Those
generations were re-sampled and matched because the seed and the sampling surface
were the same, where a re-feed pushes recorded tokens back through the forward
pass and re-samples nothing. The re-feed is owed its own demonstration. And the
negative results are kept beside the positive ones, two hypotheses built on the
entropy signal having died on the evidence, there being no branch point and no
in-flight predictor worth the name at 59.5 percent against a 51 percent base
rate.

**15. What stands, and what does not.** The demonstration rather than the count,
since a crate can report a high conformance figure while completing no turn and a
completed turn is not a count of claims met. Both engines serving, the trace over
its turns, the series section 14 reports rather than a second telling of it, and
the assertions still open named with what each waits on. Against
that, what is deferred: client-facing streaming, the status ask, and the memory
leg, each with a named door rather than a reserved slot. Closing on what the
trade costs, including the program's own judgment that a local proto-stateful
agent is a defensible intermediate and an indefensible end state.

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
  Two assertions wait on this, `spu-two-taps-one-shape` above all.
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

**Owed by this report rather than by the code.**

- **Section 14 cannot be checked from the tree.** The measurement regime's
  registrations and results stand outside this repository. Until they travel with
  the release or move into it, section 14 is the one section whose sources a
  reader holding the tree cannot reach, which breaks the standard the rest of the
  report holds to.
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
- **Appendix C is not checkable from the tree.** It states what an agent is taken
  to be rather than what this code does, so a build neither confirms nor falsifies
  it, and its own status line says so. The argument behind the notation sits in the
  agent paper, which is not in this repository, so that citation resolves to
  nothing a reader holding the tree can open.
- **The loop numbering is unsettled between the two documents.** The formulation
  numbers the primary reasoning loop `L_0` where this report calls the lifecycle
  loop zero and the inference loop loop one. Appendix C states the collision and
  names a candidate resolution it does not adopt. The ruling is the apex's.

---

## Appendix C. The formulation

**Status: descriptive of design intent. Not checkable against the tree, and not
ratified.** Every other claim in this report can be verified by a reader holding
the commit named at the top. This appendix cannot, because it states what an
agent is taken to be rather than what this code does, and a definition verifies
against argument rather than against a build. It is placed here so that the
mapping in the last part of the appendix is available to a reader who wants it
and skippable by one who does not. The argument for the notation is not made
here. It is made in the agent paper (Bucy, 2026), and this appendix states the
result of that argument and the reading of it.

The notation exists in this document for one reason beyond description. Work that
comes later will need a fixed object to cite, and a formulation carried in a
released technical report at a named commit is a firmer citation than one carried
in a working draft.

### C.1 The statement

$$A = B\Big[\, (H,\, M) \,+\, \{\,T,\, V,\, \ldots\,\} ,\quad
\big\langle\, L_0,\, L_1,\, \ldots,\, L_n \,\big\rangle \,\Big]$$

Read left to right, the three kinds of bracket are three different claims and the
difference between them is the whole content of the statement.

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
a part standing at the bound. The bound is the uid.** A reader who takes the gate
for B would have this report contradicting itself on its second page, since B is
defined as not one of the agent's parts and the gate is enumerated as one. The
gate is where crossings are authorized. It is not what makes a crossing a
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
  2, and planned section 8.
- **C, the context.** The prompt assembled at step 4 of section 4 and sent at
  step 5.
- **T, tools.** One occupant today, the shell, which section 4 describes as the
  gate's own outbound verb rather than a guest it hosts. Planned section 12.
- **V, validators.** **An unoccupied position.** Nothing in the tree fills it.
  The braces being an optional inventory is what permits that, and naming the
  position empty is more useful than pretending the slot is not there.
- **The array.** Section 13's compiled-rather-than-configured loop is the built
  form of the claim that ordering is the lever. What a builder inherits is an
  array they did not choose at runtime, which is what holds variance to a range.
- **B, the bound.** Section 3. Agent uid, socket permissions, `SO_PEERCRED`.

### C.5 Two defects this appendix carries

**The loop subscripts collide with the report's.** The formulation numbers the
primary reasoning loop `L_0`. This report calls the lifecycle loop zero and the
inference loop loop one. The same subscript therefore names two different loops
across the two documents, and a reader moving between them will be misled. The
recurrence above is written with a bare `L` to avoid asserting a numbering that
is not settled.

There is a candidate resolution and it is not adopted here. The lifecycle loop
may not be a member of the array at all. Loading and unloading is the drawing and
erasing of B rather than a loop the bounded thing runs, and section 13 already
holds that there is one clock and the load boundary is an event on it rather than
a second tempo. Under that reading the array holds inference-side loops only, the
collision dissolves, and the formulation's `L_0` and the report's loop one are
the same object. **That is a ruling the apex has not made**, and this appendix
states the collision rather than settling it.

**The formulation is a definition and the rest of this report is a description.**
Nothing here is falsified by a build failing, and nothing here is confirmed by
one passing. A reader should hold this appendix to the standard a definition is
held to, which is whether it draws its distinctions where the joints are, and not
to the standard the fifteen sections above hold themselves to.
