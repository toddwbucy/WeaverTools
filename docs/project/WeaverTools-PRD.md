# WeaverTools - Primary PRD

**Status:** MERGED 2026-07-28. RATIFIED 2026-08-04 as a member of the document set,
per the operator's ruling of that date: the whole set mapped into the graph on the
HADES server and ratifies as the complete document set for the toolless inference
deliverable. Ratification belongs to the set rather than to any one document, per
Working Process section 2, and section 0's system record carries the set-level mark.

**Date filed:** 2026-07-28
**Document ID:** `WeaverTools-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

This is the apex of the WeaverTools document set. Every crate PRD refines it,
every Spec refines a crate PRD, and every contract is derived from the PRDs of
its parties. Coherence across the set is checked by reading each document
against this one rather than by reading twenty-one pairs against each other.

The set it governs is exactly seven crate PRDs:

```
weaver-admin    weaver-harness   weaver-spu      weaver-gate
weaver-trace    weaver-traits    weaver-types
```

```graph
node: WeaverTools
kind: system
tag: ratified
```

Every crate's parent edge points here. This document declares no other edge,
because a child declares its own parent and an apex holding a copy of the whole
crate graph is a topology document under another name.

They are written together, as one act, and merged together. This is the
property that matters. A corpus assembled from documents written weeks apart
encodes a different understanding of the system in each document, and the
contradictions that result are not mistakes anyone made - they are what
happens when documents from different moments are read as a single body.

## 1. The deliverable

A deployable **proto-stateful agent** that completes a turn end to end against a
real local model and emits a clean, turn-bracketed, correctly-custodied trace.

The trace is the primary artifact. It is not a log of what the harness did. It
is the substrate the crates coordinate over: every component reports what it
did, the harness authors those reports into one record, and the per-turn root
span is the frame that joins them. Every later capability - replay under
observation, and eventually the memory leg - is a consumer of finished traces
rather than a peer of the harness loop.

## 2. What proto-stateful means, precisely

Proto-stateful means **the agent holds real state within a session and none
across sessions**. It begins each session with no accumulated experience: no
belief graph, no consolidation, no recall, no sleep or nap pass, and no memory
substrate of any kind. Conversation context within a session is not agent
memory, and the distinction is the whole of what this program defers. The
prefix reads as protoautonomic does in section 4, naming the mechanics alone
and claiming nothing about the finished behavior. An earlier vocabulary called
this stage stateless, which the human's ruling of 2026-08-01 corrected as an
overstatement: an agent holding a working structure and a hot cache is not
stateless, it is an agent whose state dies with the session.

Two things hold state across turns inside one session, both deliberate, and
they are not two things of a kind:

- **The working structure** is the state the agent reasons over: the run's trace
  events held in RAM in the same canonical form the stream carries, per the
  ruling of 2026-08-01 that retired the relational projection. Lose it and turn
  two has nothing to be about. It is volatile by construction, the program
  rebuilds it from nothing rather than from a record, the stream's accumulation
  being the operator's, and it is working memory, not a store.
- **The KV cache** is an optimization holding the same content precomputed at
  the decoder. Lose it and the agent is slow rather than absent. It is kept hot
  and flushed on the harness's terms, not per prompt, and because its loss costs
  real compute it is the surface most likely to grow quietly into session state
  if the line is not drawn. Its owner, its flush trigger, and who is forbidden
  to touch it are named in `weaver-spu-PRD` as a rule the code can be checked
  against.

## 3. One turn, end to end

This is the path the MVP must execute. Every requirement in every crate PRD
states how it serves this path.

1. A client reaches the agent's **Gate** at its local Unix socket hook, the one
   listening socket the agent binds, of any kind, per the demotion ruling of
   2026-07-31 and `weaver-gate-world-contract`. Admin's operator socket stands
   outside every agent, per section 12, and there is no listening network socket
   anywhere in the program. Outbound connections made by a tool under step 7 are
   not ingress and do not pass through Gate.
2. Gate authenticates the connection by peer credential under the boundary
   predicate, which admits front-end principals and excludes the agent uid, and
   relays the request to the harness unread: one NDJSON line in, per the world
   contract, opaque at the gate and interpreted by the harness.
3. The **harness** opens the turn root span, assigns the `turn_key`, and emits
   `turn.started`.
4. The harness assembles the prompt: system prompt, the session's message
   sequence read from the working structure, and the tool schemas.
5. The harness issues a decode request to the **SPU** over the decode socket.
   The request carries `turn_key` and `session_key`, so the SPU can parent its
   own spans to this turn.
6. The SPU decodes against the hot KV cache and returns the generation together
   with its measurement payload, tagged with the `turn_key` it was given. The
   harness writes `model.request`, `model.output`, and `model.measurement` into
   the trace. If residual readout is enabled in this agent's config file, the
   eval callback reduces per-layer activations in place and the reduction
   returns by the same path.
7. If the generation contains a tool call, the harness executes it as the
   agent's own constrained Linux user. What that tool can reach is bounded by
   the kernel through filesystem permissions, sudoers entries whose grants are
   argless, and cgroups, rather than by any harness judgment about the command
   it was handed, a grant carrying a wildcard in argument position being
   unbounded by the path it appears to name, per `weaver-admin-PRD` section 7.
   Bash and CLI access is the reference tool and a real MVP capability, safe
   because the user it runs as cannot reach what it should not. The harness
   emits `tool.call.started` and `tool.call.completed`, then returns to step 5.
8. On a final answer the harness emits `turn.closed`, closing the bracket in
   the stream, the close naming its kind.
9. The response returns through Gate to the client, one NDJSON line out,
   delivering a turn already closed.

Throughout: each event is emitted **once**, and that single emission feeds both
the volatile working structure and the outbound NDJSON stream, whose durability is
the operator's per `weaver-admin-operator-contract` section 3. Not two writes to
reconcile. The stream is also the program's one fault carrier: a fault the worker
survives is a `fault` event on it, per the fault-carrier ruling of 2026-08-01,
and no second alert path exists anywhere in the program.

The single emission is authored against the **durable event schema**, and that
schema is the only schema. The working structure holds the same canonical form
the stream carries rather than a projection of it, per the ruling of 2026-08-01,
so no projection version exists and nothing can diverge between the two. A
change to the durable event schema is the breaking change, and it is the one
version every consumer keys on.

## 4. Definition of done

A proto-stateful agent that:

1. completes a turn end to end, through the gate, with a real local model,
2. emits a trace that is turn-bracketed, contains only events this system
   authored with nothing leaked in from dependencies, reaches the declared sink,
   and conforms to its own schema,
3. hands that trace to a sink the agent cannot reach, durability being the
   operator's per `weaver-admin-operator-contract` section 3,
4. keeps a hot KV cache across turns, flushed on the harness's terms,
5. executes a real tool under kernel-enforced OS constraint, with bash as the
   reference case,
6. fires at least one protoautonomic tool call, where the harness injects a
   deterministic result into the stream in place of a stochastic one, with the
   calculator as the reference case,
7. can be reloaded with residual readout enabled or disabled by a change to its
   config file alone, with no rebuild.

Autonomic action, in the sense this program reserves the word, is
harness-initiated and out of scope here. Protoautonomic names the mechanic
alone, a model-elected call whose result the harness supplies deterministically,
and it makes no claim about the finished behavior. A tool call that requires retained
state to decide when to fire belongs to the later stateful program.

**Initiation is what these words name, and dispatch is a separate question.** Whether
an action was elected or initiated is settled by who moved first, which the trace
records, so it is a fact rather than a reading. Where the harness then routes it,
inward to the organ whose domain it belongs to or outward through the gate as
ordinary output, follows from what the action is for and changes nothing about what
it was. **Neither question is answered by where the capability sits**, and an action
that reaches a network is classified by who moved first exactly as one that never
leaves the process is. `weaver-tools-vision` section 6 works the cross of the two.

## 5. The five invariants

These bind every crate PRD, every Spec, and every contract. A document that
violates one of them is wrong, not merely inconsistent.

**Each is declared here as an `axiom` node, and a Spec's assertion grounds in one
by naming it,** per Document Format sections 3 and 4. The edge runs from the claim
to the invariant, so the chain apex section 11 states reads upward without a break:
code cites an assertion, an assertion grounds in an invariant. The set is closed at
five, and a sixth is an act on this document rather than on the format. It grew from
four on 2026-08-03, when a labelling batch found two Specs disagreeing about a seam and
the corpus had no stated forum for the disagreement.

**A claim that grounds in no invariant is representation, not an omission.** Most of
what a Spec elects is a format, a name shape, a tagging rule, or a bound, and none of
those serves an invariant because the invariants are not about representation. The
coverage number is a fact to read rather than a target to reach, and it is stated
that way here so it cannot later be argued down: the prior program's basis reached
seven of seventy-one claims and the answer was to keep the layer rather than to ask
what the layer was for.

**The named exception to Working Process section 7 ran its course and is
closed.** The 5.1 restatement and 5.4 were taken early on 2026-07-31, further
entries followed as collisions with merged charters demanded, and the
re-authoring of 2026-08-01 absorbed them all. Nothing files against this
document now except by editing it.

### 5.1 The floor is vocabulary and every behavior is a socket

**The floor is exactly `weaver-traits` and `weaver-types`.** They are linked as
Cargo dependencies because they are shared vocabulary, types and traits and
schema, and you cannot send a type definition over a socket. Floor is a linkage
fact rather than a rank: a floor crate is one every domain draws from and no
domain contains. This is the definition the whole corpus classifies against.

`weaver-trace` is not floor. The harness links it as a Cargo dependency too, but
under a contract and as a member of the harness domain, because the harness is
its only caller. Depending on nothing is what it has in common with the floor.
Being drawn by everything is what it does not.

**Every seam where one crate asks another process to do something is a Unix
socket governed by a named contract, and it authenticates its peer.** There are
no exceptions, including for crates that arrive later. When the memory leg
returns it returns behind a socket, not as a path dependency.

**How it authenticates follows from whether the channel has a name.** A channel
reached by a path is reachable by anyone who can resolve that path, so it
authenticates by credential, which is what `SO_PEERCRED` is for. A channel with
no name is a connected pair created by one party and handed to another, and
possession of the descriptor is the authentication, because no third party can
reach a socket that has no address. Which party creates the pair, and how the far
end travels to the process that holds it, belong to the contract governing that
seam and are not the apex's to settle. Two cases, one property.

The property is what the invariant protects: no process in this program talks to
another without the second knowing who the first is. The earlier reading named
the credential mechanism as the invariant itself, which made the coordination
channel of `weaver-admin-harness-contract` section 2 read as an exception rather
than as the second case, and an invariant that admits one exception in its first
round of contact with a real seam is a rule that will admit a second.

A seam that does not cross a process boundary is a library boundary. It is still
a seam and still governed by a named contract, and it is tagged `link` rather
than `socket` so the difference is stated rather than inferred. The
harness-to-trace seam is the one such seam in the base set. What the invariant
forbids is a behavior reached by path dependency across a process line, not a
crate calling a crate inside one binary.

**A crate outside the floor links the floor for the reason this section gives, and
takes from it only what it draws.** The link is licensed here: the floor is shared
vocabulary and a type definition cannot be sent over a socket, so every consumer links
it and no consumer is asking it to do anything. **Which floor crates it links follows
from what it draws**, per section 5.3's mechanical consequence that a party links the
crate defining what it emits, which is what makes the party list checkable against the
dependency graph. So a Spec whose clause argues only that it links the floor carries the
one edge here, and a Spec whose clause argues that a particular floor crate is linked
because a contract draws from it carries that one too.

**Which features a floor link is taken with grounds in nothing.** It is a build
election that would read the same under any invariant, and a crate takes the
configuration feature or leaves it because of what that crate does rather than because
of anything this section or 5.3 requires. Stated because the shape invites the opposite
reading: a feature named for what the crate does not draw looks like the draw rule
speaking, and it is not.

This invariant is what makes statefulness a feature add rather than a
re-architecture. Memory behind a socket is a new socket, a new contract, and a
schema extension. Memory as a linked crate is surgery on the harness's
dependency graph and on every call site.


```graph
node: axiom-floor-is-vocabulary-behavior-is-socket
kind: axiom
```

### 5.2 The join key travels with the work

**Every request that belongs to an existing turn carries, at every seam it
crosses, the trace context identifying that turn, and every response carries it
back.** The scope is deliberate and load-bearing: a lifecycle directive on the
coordination seam and a residency directive on the SPU seam each cross a seam and
belong to no turn, so they carry none, and the earlier universal wording made
every such directive a counterexample to the invariant it was meant to serve.

The harness is the sole writer of the trace, so a component does not emit its
own spans. It reports, and the harness authors the event. That only works if
the report can be attributed. A component handed work without a `turn_key`
cannot tell the harness which turn its result belongs to, and with more than
one turn in flight the harness cannot recover the association afterward. This
is what turns the trace from a set of per-process logs into a coordination
substrate, and it must be designed into every wire format at the moment the
wire is specified.


```graph
node: axiom-join-key-travels-with-the-work
kind: axiom
```

### 5.3 A contract is a complete interface

A contract names its parties, and for each party: the vocabulary that crosses
the seam with its meaning, the errors it can return, and the ordering guarantees
it relies on and provides. How a party represents any of that internally belongs
to its Spec and appears in no contract.

**Every contract carries a vocabulary clause, and a contract without one is not
a valid contract.** The clause names what the contract draws, grouped by the
crate that defines it, and a group is stated even when it is empty, because an
explicit nothing is an assertion someone checked and an absent group is silence.
This is what makes the floor governable without a floor contract: the floor's
required surface is the union of every clause, a definition no clause names is
unused, and a definition a clause names that the floor lacks is a gap.

Two mechanical consequences:

- A party that emits **links the crate that defines what it emits**, so the
  party list is checkable against the dependency graph.
- An agent handed one side of a contract can build that side without asking
  what the other side does. This is not an aspiration - it is the property
  that permits crates to be built in parallel once the floor is merged.


```graph
node: axiom-contract-is-a-complete-interface
kind: axiom
```

### 5.4 Organ and submodule

**An organ is a crate that governs a domain and holds a duplex channel with the
harness.** Both properties, and neither alone. A crate that governs a domain and
reaches the harness some other way is not an organ, and the duplex requirement
does not bend.

**A submodule falls under an organ's domain with that organ as its consumer.** It
holds no channel with the harness, and the shape of its channel with its own
organ is unconstrained and is that organ's business. That a submodule is never a
party to a lifecycle transition follows from its having no channel with the
harness, rather than standing as a rule of its own.

The harness is the organ whose domain is coordination, which is why it is the hub
every other organ is duplex with rather than a spoke. This is written as a test a
candidate passes and not as a list of the organs that exist today, so a crate
chartered later is classified by reading it against the test rather than by
amending an enumeration.

**No crate holds an allowance another crate could not have.** What a Spec elects for
one organ is available to any organ that later finds the need or the capability, and
a reader meeting an election in one document should not read it as that crate's
privilege. The classification above is written as a test for the same reason, and an
election that could only ever be one crate's is a sign the election is really a
charter fact wearing a Spec's clothes.

**The harness is the one exception, and its form and function are not negotiable.**
It is the organ whose domain is coordination, so the hub is what it is rather than a
shape it elected, and no efficacy argument reaches it. Every other organ is a spoke
by construction and cannot trade that away.

**Whether an organ mirrors this shape inside its own domain is section 5.5's, not
this section's.** The topology here is fixed. What an organ does one level down is a
question about integration and is answered where integration is.

Section 6 already names organs and the harness as the coordinating center, and
5.1 already carries the floor half of the same three-way distinction. This
harvests what those two imply rather than importing a new frame.

```graph
node: axiom-organ-and-submodule
kind: axiom
```

### 5.5 The harness integrates, and the loop is the mechanism

**The harness is the integrator.** Section 5.4 makes it the hub every organ is duplex
with, which is a statement about topology. This is the role that topology exists to
serve. The harness is answerable for the whole working, and an organ is answerable for
its own domain and for nothing outside it. A hub that only carried traffic would leave
integration to whichever organ noticed it was missing, which is how an organ starts
reasoning about a domain that is not its own.

**The loop is the integrating mechanism.** Integration is not a property the parts have
when assembled correctly, it is work something does, and the loop running in the harness
is the thing that does it. This is why the composition root is new code rather than
carried code, and why loop 0 is the framework's rather than a builder's: the mechanism
that makes the parts one program cannot itself be a part.

**Each organ presents its contracts to the harness and presents nothing to any other
organ.** A contract is what an organ offers the integrator, being the vocabulary it
speaks, the errors it returns, and the ordering it relies on and provides, per section
5.3. That is the whole of what an organ exposes and the whole of what the loop has to
work from, which is what lets an organ be built against its own contract alone.

**An organ presents one seam per service within its domain, and the plural is the
general case rather than an exception.** A socket serves one service, so an organ with
two kinds of traffic holds two ends under two contracts, which is what `weaver-spu`
already does with residency at one end and decode at the other. The alternative is one
channel carrying two services, where a flush ordering written for one silently becomes
a rule about the other and neither party can say why it holds. **The seam's identity is
the contract governing it and not the pair of crates it runs between**, which is why
two seams between one pair need two names and why a reader counting crate pairs would
miscount this program's shape.

**The loop is answerable for correctness and for timing.** It ensures that point A talks
to point B correctly and at the right time. Correctly means the vocabulary each contract
names, in the direction that contract states. At the right time means the ordering each
contract relies on still holds when two organs' orderings have to be reconciled against
each other, which no organ can do from inside its own domain, because no organ can see
the other's. **Reconciliation across domains is the loop's work by construction and not
by convention.** Reconciliation inside one domain is the organ's own and this section
does not reach it.

**So a conflict between organs is settled in the contracts they hold with the harness.**
There is no organ-to-organ contract to settle it in, by 5.4, and no other party sees
both sides. A seam question two organs answer differently is not a hard case calling for
a ruling, it is an incomplete contract, which 5.3 forbids by name. The obligation runs
toward the contract rather than away from it, and Document Format section 7 states the
order a reader follows.

**This invariant binds what crosses between domains and says nothing about what
happens inside one.** An organ integrates its own parts however its domain demands,
and it may well arrive at this same shape one level down, a submodule reaching its
organ and an interior loop making the organ's parts one thing. That is emulation and
not obligation. An organ that adopts the pattern inherits an argument already made,
and an organ that departs from it answers to its own Spec and says why there.

**Demand is the pressure and the domain is where it acts.** An organ develops as its
domain's demands dictate, the way an organism develops under selection, and the shape
it arrives at is answerable to those demands rather than to this section. What the
program fixes is the environment that development happens in, which is the harness's
form and the contracts at each organ's edge. **The harness's form is not negotiable
for the same reason the environment is not: it is what the organs develop against,
not an organism developing alongside them.** An environment that changed shape under
the same pressure would leave nothing for anything to adapt to, and every organ would
be adapting to every other organ's adaptations, which is the peer coupling 5.4 forbids
arriving by a slower route.

So the constraint is narrow and it is the whole of it. What an organ exposes is its
contract. What crosses between organs is the loop's. **Everything else inside a domain
is that domain's to evolve.**

```graph
node: axiom-harness-integrates-by-the-loop
kind: axiom
```


## 6. The agent lifecycle

The administrative lifecycle has exactly two state-transition verbs, `load` and
`unload`, and one verb that transitions nothing, `validate`, which confirms an
agent's configuration and boundary without starting anything against them.
Read-only `list` and `show` are observations, not transitions. Admitting and
removing a principal are operator acts on the operating system, performed before
an agent exists and after it stops existing. They sit outside the program's verb
set, and what they bracket is the resting state the diagram names provisioned
and unloaded, per `weaver-admin-PRD` section 4.

```
absent
  |   ^
  |   |    operator provisioning / removal
  v   |    (operator acts, not verbs of this program)
provisioned, unloaded
  |   ^
  |   |    load / unload
  v   |
loaded, idle
  |   ^
  |   |    work starts / work ends
  v   |
active
```

Provisioned identity is made and removed by the operator rather than by a verb.
`load` and `unload` govern the complete residency boundary. **Loaded-and-idle
is a first-class state** -
without it the system cannot interrupt one run while leaving the agent ready
for the next.

The chain, in order:

| Component | Owns |
|---|---|
| **weaver-admin** | External authorization, boundary verification, lifecycle direction, custody of the sink, rollback |
| **weaver-harness** | The fan-out inside enter and leave, readiness aggregation, activity control |
| **weaver-spu** | Model admission, decoder and encoder residency, GPU release |
| **weaver-gate** | Sole work ingress, and the outer membrane, raised last and lowered first |

**Admin is the coordinating center of the load, and the harness is the
coordinating center of the turn.** Admin authorizes the intent, verifies the
boundary the operator wrote, opens the sink, starts the worker unit, and directs
the transition across its one seam, rolling back its own acts where a directive
refuses. The harness cannot drive the early steps of its own creation, because
the worker spawn and the descriptor handoff run before the harness exists at
all, and supervising worker lifetimes is long-lived and fleet-wide where the
harness is mortal. What the harness owns is the interior of the directives:
admin holds no channel to the SPU or the gate, so the harness fans the directive
out along its own seams, collects each organ's confirmation, and returns one
aggregate. Sequencing the organs is the harness's because the seams are, and
each organ performs its own operation.

**The harness is the sole writer of the trace across both centers.** It stands
up the working structure, authors the `load` event of the run, which for `run0`
is the record of admin's initial contact, and writes every component's activity
into the stream for the whole of the residency. All coordination between
components passes through the harness, which is what makes it the hub every
later organ attaches to, each behind its own socket and contract, and the
structural reason statefulness is an extension rather than a re-architecture.

`weaver-trace` is not a party to the transition. It is linked by the harness
under a contract as a member of the harness's domain, per 5.1's link case, so it
cannot be a socket peer that confirms anything. It is the mechanism the harness
records through, during the load and for the whole of the agent's residency
thereafter.

Binding rules:

- A lifecycle call succeeds only after every component transition is confirmed.
- A partial load is never published as loaded.
- Load starts Gate last. Unload stops Gate first. A Gate process never outlives
  the worker interior it protects.
- No lifecycle verb auto-chains another.
- A GPU conflict is rejected at model admission, by the SPU, until the operator
  explicitly unloads the occupant. Admission refuses and never evicts, so no load,
  at any point in its sequence, auto-evicts another agent. The SPU is the one
  authority on the device and nothing upstream weighs it.
- No prompt, turn, task, or run enters through Admin. Work enters only through
  Gate.
- Activity stop, cancellation, or interruption returns the agent to
  loaded-and-idle. It does not unload the agent.

The lifecycle is the system's largest behavioral contract - four parties over
three sockets - and it is filed at this level rather than under any one crate,
because it belongs to none of them.

## 7. Scope criteria

Material crosses from the prior tree, or is written fresh, only if it satisfies
one of exactly two criteria. State which one, at the moment you carry it.

### 7.1 Does it serve one turn end to end?

"Serves" means you can name the step in section 3 that exercises it. Not that
it might be needed, not that the prior tree had it.

### 7.2 Is it observability the operator needs to diagnose a deployed agent?

The named set, closed:

- **Residual-stream readout.** Per-layer activations from the running decoder,
  reduced in place, enabled or disabled per agent by its config file. See
  section 8.
- **Measurement payloads.** Token identifiers and token entropies, emitted into
  the stream at production time. These are what make replay under
  observation possible. Without them a replay is approximate, and an
  approximate replay of a forward pass is a false diagnosis rather than a weak
  one.

The set is closed deliberately. A second criterion with an open membership
becomes the door through which everything re-enters, and each individual
admission looks principled.

Anything satisfying neither criterion does not cross. **Nothing crosses because
the prior tree has it.**

## 8. Replay under observation

The trace is not only the record of a session. It is the source from which a
session can be re-run under close observation.

The loop is stochastic and does not reproduce, and this program makes no
run-again claim, in any arrangement below. Replay does not need one.

**What a record supports follows from what the deployment declared and
produced, per the ruling of 2026-08-02, and the arrangements are not ranked.**
Re-analysis over the frozen record is always available, because everything
produced is recorded. Deterministic re-feed is available when the record holds
the token path: because the trace records the sampler's **actual tokens**
rather than a seed, a recorded scenario is replayed by feeding the recorded
token sequence back through the forward pass. Nothing is re-sampled. The
residuals are deterministic given the same weights, within GPU float
tolerance. And stochastic re-entry from the same starting field is available
when the binary declares one, the setup surface frozen at the worker's
composition root, seed and sampling parameters baked immutable. A frozen seed
narrows variance and buys audit rather than determinism, which is why the
run-again claim stays unmade there too. The conditions are stated for the
requirement's shape, and on the corpus this lands into the first two
arrangements are always available, every record holding the token path since
the levels retired, so the third alone waits on a binary's declared
disposition and the claim-relative rule governs arrangements not yet built
rather than forking the present.

This makes residual readout a **production troubleshooting and interpretability
instrument** rather than a research aside. When visibility is needed it is
enabled and the agent runs slower. When it is not, the agent loads without it.
The cost is real and it is the operator's to elect, per load.

**Completeness is claim-relative rather than a fixed bar.** Deterministic
re-feed requires, exactly: input token ids, output token ids, model identity
and weights hash, sampling parameters, and the prompt-block partition, with
tokenization reproducible from what is recorded. These are requirements on
`weaver-trace-PRD`, derived from a `weaver-spu-PRD` capability. A record
claiming a replay arrangement carries everything that arrangement requires,
and a replay missing an input its claim requires observes a forward pass that
never happened. A deployment claiming only re-analysis owes nothing beyond
the record itself, because it claims nothing more.

Custody places the replay driver outside the agent. The agent must not own or
even read its own trace, so a tool that reads the operator-held stream and drives
the SPU runs as an operator principal, over the operator's own storage. This is
structural, not policy.

## 9. Out of scope, and how it returns

**Out entirely:** the memory leg in every form - belief graph, consolidation, sleep
and nap passes, recall, and any memory substrate. Also out: offline analysis,
training, and the desktop frontend.

**External tooling is out entirely as well, and stays out.** This program builds no
tool crate, and the reason differs on each side of the boundary.

**Outward, what it owns is the grip:** the interface a tool is built to be gripped by,
which is `weaver-gate-world-contract`. A hammer is not installed into a carpenter, it
is shaped so a hand can hold it, and the boundary contract is that hand stated as a
specification. A bash tool, a database client, an API caller are all things built to
fit the grip, and none of them is a crate here.

**Inward, what an older reading called an internal tool is a function loop, located
with the control loops in the harness.** A control loop runs the turn and the
lifecycle. A **function loop** is a function used in a predefined way when the correct
signal is given, and the calculator section 4 requires is one: the harness injects a
computed result where a stochastic one would otherwise go. Managing state and deciding
when state enters the decoder's path are the same kind of thing. Routing the right
work to the right part of the agent is what the loop is for, per invariant 5.5, and
**a calculator small enough to be a few lines does not become an organ by being
useful.**

So the rule reaches the far side of the grip and nothing else. A hand grips what is
outside it, and a function loop the harness runs was never on the far side of
anything.


**The grip is not a reserved slot and is the clearest case of the difference.** A
reserved slot is a shape carried for a reader that does not exist. The grip has a
reader today: every client that dials the gate uses it, and the tool case is that
same path with a different thing on the far side. Nothing is carried in
anticipation, because nothing is added at all.

Statefulness returns as a **feature add**, not as a retrofit. The mechanism is
fixed now, in three parts:

1. **Schema extension.** The durable NDJSON event schema is the one schema,
   versioned once and extending additively, under the authority stated in
   section 3. Memory adds event kinds and payload shapes, and it does not
   reshape existing ones.
2. **A new socket and a new contract.** Per invariant 5.1, memory arrives as a
   socket peer with a complete contract, never as a linked crate.
3. **Its own PRDs.** Stateful PRDs are written per crate as required, and
   contracts are amended or added by the order of work in section 10.

No seam, stub, reserved slot, or dormant contract party is carried in
anticipation of this, **and a reserved slot can be a data field as easily as an
interface**: a payload field whose only reader is unbuilt, a vector nothing
retrieves, an event kind with no emitter. The schema is where this rule is
tested most often, because adding a field feels smaller than adding an
interface and is the same error. Preparation for memory is a property of the
schemas being extensible, not a set of empty joints or empty fields. A slot
reserved today is a guess about a design that has not been written.

## 10. The order of work

A merged document changes by being edited. A ratified document does not change at
all, and a change found necessary after ratification returns the work to authoring
rather than being patched in place. The three states and their transitions belong
to the Working Process, section 2, and are not restated here.

No amendment banners. No supersession notices. No citations into retired
documents. No obligations patched inline because their referent was withdrawn.
If a change touches a contract, every party to that contract merges in the same act.

Every one of those devices is a reasonable local decision. Together they are
how a corpus stops being coherent while every individual document still looks
maintained.

The order of work is strict:

```
1. This document and the seven crate PRDs, together
2. Each crate PRD with its contracts, written as one act
3. Specs, against the merged PRD and contract set
4. Graph mapping, which ratifies the set
5. Floor code: traits, types, trace
6. spu | harness | admin | gate, in parallel
7. Composition root, integration
```

Nothing at step N is written before step N-1 is merged. The parallelism at step
6 is earned by the completeness of step 2 and the merging of step 5, and by
nothing else. Contracts that are complete cannot be built against in parallel
while the floor beneath them is still moving.

A contract is not a phase of its own. The harness is the hub every crate connects
to, so a crate PRD is largely about its seam with the harness, and a PRD written
without its contract has no center to attach to and grows one of its own. Specs
come after the contracts rather than before them, because a Spec is build
instructions for one crate written against its PRD and every contract that crate
is party to. A Spec written first documents code instead of governing it.

## 11. Enforcement

No graph measures code against this document set during this program. The PRD to
Spec to contract to code chain is enforced by the four devices below and does not
require a database. A conformance graph measures whether code matches settled
intent, and this program deliberately unsettles intent, so that graph is early
rather than slow. It is the instrument for the phase after this one.

**The mapping graph is a different artifact and this section does not prohibit it.**
It is generated from the documents rather than from the code, it answers questions
about the documents, and completing it is what ratifies the set. Section 0 declares
the root node it is built from. The two artifacts share a word and share nothing
else: one is built from code and asked whether the documents were obeyed, and the
other is built from documents and asked whether they cohere.

What enforces instead:

1. **Conformance trace headers in source**, carrying the code to assertion to
   document chain.
2. **Compile-time pins** for invariants that are type properties. A runtime
   test structurally cannot pin the absence of a trait implementation.
3. **Perturbation-verified tests** for invariants that are behaviors. Always
   confirm the test fails when the property is removed. A test that passes
   whether or not the property holds is worse than no test, because it converts
   "unenforced" into "documented as enforced".
4. **Human and third-party review**, reading the review body rather than the
   thread count.

A clean automated gate is evidence that the gate did not fire. It is not
evidence of correctness.

## 12. Why this architecture

The design rests on one engineering principle: lower latency is always better,
provided you know what you are giving up to get it. Everything above is a
consequence of naming that trade rather than leaving it implicit, and every
decision in this document is justified by it and by nothing else.

What this architecture gives up is fungibility, and with it the service
reliability that fungibility underwrites. The model is not swappable behind a
remote API, and the harness does not run somewhere other than where the agent
runs. Both are bound to the device, and that binding forecloses what a network
architecture gets by construction: horizontal scale, failover to a healthy
replica, and rolling replacement of a degraded node. A deployment that needs
those is building a different system than this one, and should say so rather
than treating the difference as configuration.

**The gate's far side is experience and never the world.** Whatever stands out
there - a client, a tool process, an appendage - meets the environment under its own
contract with that environment, and hands back what the meeting produced. Physics,
protocol, timeout, and failure mode are settled on the far side of the grip and reach
this program already rendered as data. Nothing here holds a contract with the world
itself, at any depth.

**Opacity is what makes that true, and it does three jobs rather than one.** The gate
carries octets it must not read, opaque both ways. Stated as confinement, that is the
familiar reason: a boundary cannot leak what it cannot parse. It is also why a tool
call is indistinguishable from any other output, which is what keeps the gate from
growing a tool vocabulary. And it is why the boundary is relocatable: **a gate that
parsed content would have to know what kind of thing produced it**, and because it
does not, it cannot tell a human client from an appendage. The same crate therefore
sits unchanged wherever the outermost boundary happens to be. That property is a
consequence and not a plan, and this program builds nothing to exploit it.

What it buys is that the locus of regulation moves up to the agent. Because an
agent is an operating-system user, what it may touch is bounded once, at the
agent, by the kernel. The boundary that does the bounding is the operator's
artifact, authored before an agent exists by whatever means that operator's site
admits a principal, and the program verifies it rather than authors it, per
`weaver-admin-PRD` section 1: a program that admitted principals would be
raising a second trust model above the one it claims to inherit. The
alternative is to distribute components across a
network and regulate every seam between them, so each component carries its own
policy layer and the composite behavior is whatever those layers happen to add
up to. Regulating the principal is cheaper to reason about and cheaper to audit
than regulating every seam, and it is a discipline with decades of tooling
behind it rather than one invented for the purpose.

**The trust model, stated once.** The program secures the agent's record against
the agent and against nothing stronger. The operator is trusted by construction:
the operator admits the principal, writes the boundary, declares the sink, and
holds what accumulates behind it, and every custody argument in this corpus is
exclusion of the agent rather than evidence against the holder. A charter that
appears to defend an artifact against its operator is misread.

**The process topology, stated once so 5.1's test has something to read.** An
agent is three processes and one supervisor outside them. The worker is the
composition root: its binary compiles the harness, `weaver-trace` under its
contract, and the floor. The SPU and the gate are each their own binary,
forked by the harness during enter and holding one channel end each from their
first instruction. `weaver-admin` stands outside every agent, compiles into its
own processes, and is never linked into a worker. A crate calling a crate
inside one of those binaries is a link seam. Anything else crosses a process
line and is a socket, per 5.1.

This is systems architecture at the OS level rather than at the network level,
and the difference is not stylistic. At the network level the unit of
composition is a service, the unit of trust is a credential presented across a
wire, and every seam costs a round trip. At the OS level the unit of
composition is a process, the unit of trust is a UID the kernel enforces, and a
seam costs a socket write. Co-location is what makes a low-latency interior
possible, and per-agent OS identity is what makes a co-located interior safe to
have. Neither half works without the other, which is why they are the two
things this document refuses to trade away.

## 13. Where the trade does not yet pay

The trade in section 12 is not evenly justified across the program, and this
document is the wrong place to pretend otherwise.

A proto-stateful agent is precisely the workload a network architecture serves
well. Holding nothing across sessions, it is replicable by construction, and
replicable workloads are what horizontal scale, failover, and rolling
replacement exist for. Measured at stage one and nowhere else, giving up
redundancy in order to co-locate a proto-stateful agent is the worse
engineering choice, and no amount of saved latency repays it. A local
proto-stateful agent is not a better version of a networked service of the
same shape. It is a worse one with a faster interior.

The trade pays only once the agent holds state bound to it. At that point the
thing redundancy would protect is unique by construction and cannot be
replicated regardless of architecture, so the reliability being given up was
never available. The latency being bought stops being a per-request saving and
becomes the budget a continuous coordination loop runs inside.

So this document specifies a floor and not a product. A proto-stateful local
agent is a defensible intermediate and an indefensible end state, and building one
without building past it would be paying the full cost of this architecture for
none of what the cost is for. That is an accounting of where the costs land,
not a claim about what gets built next.

The order, however, is not a staging preference. Statefulness is a property
added to a working turn rather than a substitute for having one, so a stateful
agent cannot be built before there is a proto-stateful agent to make stateful.
The
prior attempt is the evidence. It built the memory leg and the turn loop
concurrently, so every change to either had to stay coherent with the other
while both were still moving, and its final phase deleted roughly nine thousand
six hundred lines against four thousand added across five separate retirement
commits. Stage one is therefore not a stepping stone that a better-resourced
program would skip. It is the only order in which either thing gets built.
