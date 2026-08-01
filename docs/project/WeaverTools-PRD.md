# WeaverTools - Primary PRD

**Status:** MERGED 2026-07-28. In `main` and the source of truth for now. Nothing
in this corpus is ratified until the whole document set maps into the graph, which
is the Working Process section 2 definition and belongs to the set rather than to
any one document.

**Date filed:** 2026-07-28
**Revised:** 2026-07-31. Section 5 carries a fourth invariant and 5.1 is restated.
Both were taken early as a named exception to Working Process section 7, recorded
in section 5. This is not the apex re-authoring, which still waits on all seven
charters, and no other item owed to that re-authoring moved with these two. The 5.1
example clause was corrected the same day, on review, from a fork mechanism the
governing contract had already retired.
**Revised:** 2026-07-31, second entry. Section 6's verb set restates from four
transition verbs to two plus `validate`, the operator acts leaving the verb set,
per the edit `weaver-admin-PRD` section 11 owes and section 4 states. Taken early
under the same named exception, because a merged verb set contradicted by a
merged charter is a collision in main. The rest of section 6's owed items,
orchestration seating, the linked-vocabulary sentence, and the emission wording,
move with the apex re-authoring and did not move here.
**Revised:** 2026-07-31, third entry. Section 6's GPU-conflict binding relocates
from the load driver to SPU model admission, per ruling C of this date: admin
arbitrates no hardware, the SPU is the one authority on the device, and the
no-auto-evict guarantee moves with the rejection rather than leaving with it.
Taken early under the same named exception, because a merged binding contradicted
by the ruling's charter edits would be a collision in main.
**Revised:** 2026-08-01, fourth entry, the durable-record cut. Durability is the
operator's, per the ruling recorded at `weaver-admin-operator-contract` section 3:
section 2's rebuild-from-record clause dissolves with record-based resume, section
3's single emission feeds the working structure and the outbound stream, section
4's definition of done restates the trace items against the declared sink, and
section 8's replay driver reads the operator-held stream. Taken early under the
same named exception, because merged contracts now contradict the old wording and
a collision in main is the case the exception exists for.
**Revised:** 2026-08-01, fifth entry. The relational projection retires per the
human's ruling of this date: section 2's working structure holds the canonical
form the stream carries and section 3's schema-authority paragraph drops the
projection version, one schema being the only schema, and section 9's
schema-extension clause counts one schema with it. Taken early under the same
named exception, the re-authoring following immediately.
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

A deployable **stateless agent** that completes a turn end to end against a
real local model and emits a clean, turn-bracketed, correctly-custodied trace.

The trace is the primary artifact. It is not a log of what the harness did. It
is the substrate the crates coordinate over: every component reports what it
did, the harness authors those reports into one record, and the per-turn root
span is the frame that joins them. Every later capability - replay under
observation, and eventually the memory leg - is a consumer of finished traces
rather than a peer of the harness loop.

## 2. What "stateless" means, precisely

Stateless means **the agent begins each session with no accumulated
experience**. There is no belief graph, no consolidation, no recall, no sleep
or nap pass, and no memory substrate of any kind.

It does **not** mean the agent is stateless within a session. Two things hold
state across turns inside one session, both deliberate:

- **The KV cache.** It is kept hot and flushed on the harness's terms, not per
  prompt. It is one of the two surfaces holding state the program cannot
  reconstruct, which is what makes it the surface most likely to grow quietly
  into session state if the line is not drawn. Its owner, its flush trigger, and
  who is forbidden to touch it are named in `weaver-spu-PRD` as a rule the code
  can be checked against.
- **The working structure.** The run's trace events held in RAM in the same
  canonical form the stream carries, per the ruling of 2026-08-01 that retired
  the relational projection. It is volatile by construction: lose the process and
  lose the working structure, and the program rebuilds it from nothing rather
  than from a record, the stream's accumulation being the operator's. It is
  working memory, not a store.

Conversation context is not agent memory. The distinction is the whole of what
this program defers.

## 3. One turn, end to end

This is the path the MVP must execute. Every requirement in every crate PRD
states how it serves this path.

1. A client reaches the agent's **Gate** over its network face. Gate is the
   only crate that binds a listening network socket and the only ingress for
   work. Outbound connections made by a tool under step 7 are not ingress and
   do not pass through Gate.
2. Gate authenticates the peer, converts to the internal wire, and forwards to
   the agent's harness worker over a SO_PEERCRED Unix socket.
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
   the trace. If residual readout is enabled in this agent's state file, the
   eval callback reduces per-layer activations in place and the reduction
   returns by the same path.
7. If the generation contains a tool call, the harness executes it as the
   agent's own constrained Linux user. What that tool can reach is bounded by
   the kernel through filesystem permissions, sudoers, and cgroups, rather than
   by any harness judgment about the command it was handed. Bash and CLI access
   is the reference tool and a real MVP capability, safe because the user it
   runs as cannot reach what it should not. The harness emits
   `tool.call.started` and `tool.call.completed`, then returns to step 5.
8. On a final answer the harness emits `turn.closed` and assembles the per-turn
   record as a read of the working structure.
9. The response returns through Gate to the client.

Throughout: each event is emitted **once**, and that single emission feeds both
the volatile working structure and the outbound NDJSON stream, whose durability is
the operator's per `weaver-admin-operator-contract` section 3. Not two writes to
reconcile.

The single emission is authored against the **durable event schema**, and that
schema is the only schema. The working structure holds the same canonical form
the stream carries rather than a projection of it, per the ruling of 2026-08-01,
so no projection version exists and nothing can diverge between the two. A
change to the durable event schema is the breaking change, and it is the one
version every consumer keys on.

## 4. Definition of done

A stateless agent that:

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
   state file alone, with no rebuild.

Autonomic action, in the sense this program reserves the word, is
harness-initiated and out of scope here. Protoautonomic names the mechanic
alone, a model-elected call whose result the harness supplies deterministically,
and it makes no claim about the finished behavior. A tool call that requires
retained state to decide when to fire belongs to the later stateful program.

## 5. The four invariants

These bind every crate PRD, every Spec, and every contract. A document that
violates one of them is wrong, not merely inconsistent.

**Section 5.4 and the restatement inside 5.1 were taken early on 2026-07-31, as a
named exception to Working Process section 7,** which holds the apex re-authoring
until all seven charters exist and calls it the one piece of collected work that
cannot be taken early. The exception was ruled because 5.4 defines what an organ
is, `weaver-spu` is the second organ, and the charter the rule waits on cannot be
written against a definition the rule withholds until that charter exists. The
scope of the exception is those two entries. Every other item in
`weaver-admin-PRD` section 11 marked as filing with the apex re-authoring still
files with it, and this document is still un-re-authored.

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

This invariant is what makes statefulness a feature add rather than a
re-architecture. Memory behind a socket is a new socket, a new contract, and a
schema extension. Memory as a linked crate is surgery on the harness's
dependency graph and on every call site.

### 5.2 The join key travels with the work

**Every request crossing a seam carries the trace context identifying the turn
it belongs to, and every response carries it back.**

The harness is the sole writer of the trace, so a component does not emit its
own spans. It reports, and the harness authors the event. That only works if
the report can be attributed. A component handed work without a `turn_key`
cannot tell the harness which turn its result belongs to, and with more than
one turn in flight the harness cannot recover the association afterward. This
is what turns the trace from a set of per-process logs into a coordination
substrate, and it must be designed into every wire format at the moment the
wire is specified.

### 5.3 A contract is a complete interface

A contract names its parties, and for each party: the types it uses, the errors
it can return, and the ordering guarantees it relies on and provides.

Two mechanical consequences:

- A party that emits **links the crate that defines what it emits**, so the
  party list is checkable against the dependency graph.
- An agent handed one side of a contract can build that side without asking
  what the other side does. This is not an aspiration - it is the property
  that permits crates to be built in parallel once the floor is merged.

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

Section 6 already names organs and the harness as the coordinating center, and
5.1 already carries the floor half of the same three-way distinction. This
harvests what those two imply rather than importing a new frame.

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
| **weaver-admin** | External authorization, provisioning, lifecycle intent, custody of the boundary |
| **weaver-harness** | Ordered load/unload orchestration, readiness, rollback, activity control |
| **weaver-spu** | Model admission, decoder and encoder residency, GPU release |
| **weaver-gate** | Sole work ingress, and the outer membrane, started last and stopped first |

Admin authorizes lifecycle intent. The harness coordinator sequences the
transaction, collects every organ's confirmation, and returns the aggregate to
Admin. Each organ performs its own operation.

`weaver-trace` is not on that list. It is linked vocabulary under 5.1 rather
than a socket peer, so it cannot be a party that confirms a transition. It is
the substrate every organ emits into during the load and for the whole of the
agent's residency thereafter.

The harness is the coordinating center of the agent. Admin authorizes the
intent and hands the transition across, and from that point the harness creates
the trace for the session, records admin's initial contact as its first entry,
sequences the load, and writes every component's activity into that trace. All
coordination between components passes through the harness. It is the sole
writer of the trace and the sole broker of access to it. That seam is where
later organs attach, each behind its own socket and contract, and it is the
structural reason statefulness is an extension rather than a re-architecture.

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

### 7.1 Does it serve one stateless turn end to end?

"Serves" means you can name the step in section 3 that exercises it. Not that
it might be needed, not that the prior tree had it.

### 7.2 Is it observability the operator needs to diagnose a deployed agent?

The named set, closed:

- **Residual-stream readout.** Per-layer activations from the running decoder,
  reduced in place, enabled or disabled per agent by its state file. See
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
run-again claim. Replay does not need one. Because the trace records the
sampler's **actual tokens** rather than a seed, a recorded scenario is replayed
by feeding the recorded token sequence back through the forward pass. Nothing
is re-sampled. The residuals are deterministic given the same weights, within
GPU float tolerance.

This makes residual readout a **production troubleshooting and interpretability
instrument** rather than a research aside. When visibility is needed it is
enabled and the agent runs slower. When it is not, the agent loads without it.
The cost is real and it is the operator's to elect, per load.

For replay to be worth anything the trace must record, exactly: input token
ids, output token ids, model identity and weights hash, sampling parameters,
and the prompt-block partition. Tokenization must be reproducible from what is
recorded. These are requirements on `weaver-trace-PRD`, derived from a
`weaver-spu-PRD` capability, and they are not optional - a replay missing any
of them observes a forward pass that never happened.

Custody places the replay driver outside the agent. The agent must not own or
even read its own trace, so a tool that reads the operator-held stream and drives
the SPU runs as an operator principal, over the operator's own storage. This is
structural, not policy.

## 9. Out of scope, and how it returns

**Out entirely:** the memory leg in every form - belief graph, consolidation,
sleep and nap passes, recall, and any memory substrate. Also out: offline
analysis, training, and the desktop frontend.

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
anticipation of this. Preparation for memory is a property of the schemas being
extensible, not a set of empty joints. A slot reserved today is a guess about a
design that has not been written.

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

What it buys is that the locus of regulation moves up to the agent. Because an
agent is an operating-system user, what it may touch is bounded once, at the
agent, by the kernel. The alternative is to distribute components across a
network and regulate every seam between them, so each component carries its own
policy layer and the composite behavior is whatever those layers happen to add
up to. Regulating the principal is cheaper to reason about and cheaper to audit
than regulating every seam, and it is a discipline with decades of tooling
behind it rather than one invented for the purpose.

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

A stateless agent is precisely the workload a network architecture serves well.
Anything holding no accumulated state is replicable by construction, and
replicable workloads are what horizontal scale, failover, and rolling
replacement exist for. Measured at stage one and nowhere else, giving up
redundancy in order to co-locate a stateless agent is the worse engineering
choice, and no amount of saved latency repays it. A local stateless agent is
not a better version of a networked stateless service. It is a worse one with a
faster interior.

The trade pays only once the agent holds state bound to it. At that point the
thing redundancy would protect is unique by construction and cannot be
replicated regardless of architecture, so the reliability being given up was
never available. The latency being bought stops being a per-request saving and
becomes the budget a continuous coordination loop runs inside.

So this document specifies a floor and not a product. A stateless local agent
is a defensible intermediate and an indefensible end state, and building one
without building past it would be paying the full cost of this architecture for
none of what the cost is for. That is an accounting of where the costs land,
not a claim about what gets built next.

The order, however, is not a staging preference. Statefulness is a property
added to a working turn rather than a substitute for having one, so a stateful
agent cannot be built before there is a stateless agent to make stateful. The
prior attempt is the evidence. It built the memory leg and the turn loop
concurrently, so every change to either had to stay coherent with the other
while both were still moving, and its final phase deleted roughly nine thousand
six hundred lines against four thousand added across five separate retirement
commits. Stage one is therefore not a stepping stone that a better-resourced
program would skip. It is the only order in which either thing gets built.
