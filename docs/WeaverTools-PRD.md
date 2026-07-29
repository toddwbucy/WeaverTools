# WeaverTools - Primary PRD

**Status:** RATIFIED 2026-07-28 by operator. Frozen. This document changes only
by being re-authored and re-ratified whole, per section 10.

**Date filed:** 2026-07-28
**Document ID:** `WeaverTools-PRD`
**Editorial:** ASCII, no em-dashes.

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

They are written together, as one act, and frozen together. This is the
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
  prompt. It is the one surface holding state that cannot be reconstructed from
  the durable record, which is what makes it the surface most likely to grow
  quietly into session state if the line is not drawn. Its owner, its flush
  trigger, and who is forbidden to touch it are named in `weaver-spu-PRD` as a
  rule the code can be checked against.
- **The working structure.** The in-RAM relational projection of the session's
  trace events. It is volatile and reconstructible by construction: lose the
  process and lose the working structure, then rebuild it exactly from the
  durable record. It is working memory, not a store.

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
the volatile working structure and the durable record. Not two writes to
reconcile.

The single emission is authored against the **durable event schema**. The
working structure is a projection of committed events rather than a second
author, so its projection version governs only how events become rows and may
be bumped without changing what is emitted. A change to the durable event
schema is the breaking change, and the projection declares which event-schema
versions it can consume.

## 4. Definition of done

A stateless agent that:

1. completes a turn end to end, through the gate, with a real local model,
2. emits a trace that is turn-bracketed, contains only events this system
   authored with nothing leaked in from dependencies, resolves to the correct
   path, and conforms to its own schema,
3. writes that trace where the agent cannot reach it,
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

## 5. The three invariants

These bind every crate PRD, every Spec, and every contract. A document that
violates one of them is wrong, not merely inconsistent.

### 5.1 The floor is vocabulary and every behavior is a socket

`weaver-traits`, `weaver-types`, and `weaver-trace` are linked as Cargo
dependencies because they are shared vocabulary - types, traits, schema. You
cannot send a type definition over a socket.

**Every seam where one crate asks another to do something is a SO_PEERCRED
Unix socket governed by a named contract.** There are no exceptions, including
for crates that arrive later. When the memory leg returns it returns behind a
socket, not as a path dependency.

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
  that permits crates to be built in parallel once the floor is frozen.

## 6. The agent lifecycle

The administrative lifecycle has exactly four state-transition verbs:
`create`, `load`, `unload`, `destroy`. Read-only `list` and `show` are
observations, not transitions.

```
absent
  |   ^
  |   |    create / destroy
  v   |
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

`create` and `destroy` govern provisioned identity. `load` and `unload` govern
the complete residency boundary. **Loaded-and-idle is a first-class state** -
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
- GPU conflict is rejected until the operator explicitly unloads the occupant.
  Load never auto-evicts another agent.
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

### 7.2 Is it an observability capability the operator needs to diagnose a deployed agent?

The named set, closed:

- **Residual-stream readout.** Per-layer activations from the running decoder,
  reduced in place, enabled or disabled per agent by its state file. See
  section 8.
- **Measurement payloads.** Token identifiers and token entropies, emitted to
  the durable record at production time. These are what make replay under
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
even read its own durable trace, so a tool that reads traces and drives the SPU
runs as an operator principal. This is structural, not policy.

## 9. Out of scope, and how it returns

**Out entirely:** the memory leg in every form - belief graph, consolidation,
sleep and nap passes, recall, and any memory substrate. Also out: offline
analysis, training, and the desktop frontend.

Statefulness returns as a **feature add**, not as a retrofit. The mechanism is
fixed now, in three parts:

1. **Schema extension.** The in-RAM relational schema and the durable NDJSON
   event schema are versioned independently and extend additively, under the
   authority stated in section 3. Memory adds tables and event kinds, and it
   does not reshape existing ones.
2. **A new socket and a new contract.** Per invariant 5.1, memory arrives as a
   socket peer with a complete contract, never as a linked crate.
3. **Its own PRDs.** Stateful PRDs are written per crate as required, and
   contracts are amended or added by the freeze discipline in section 10.

No seam, stub, reserved slot, or dormant contract party is carried in
anticipation of this. Preparation for memory is a property of the schemas being
extensible, not a set of empty joints. A slot reserved today is a guess about a
design that has not been written.

## 10. The freeze discipline

The document set is frozen when ratified, and **a frozen document changes only
by being re-authored and re-ratified whole**.

No amendment banners. No supersession notices. No citations into retired
documents. No obligations patched inline because their referent was withdrawn.
If a change touches a contract, every party to that contract re-ratifies in the
same act.

Every one of those devices is a reasonable local decision. Together they are
how a ratified corpus stops being coherent while every individual document
still looks maintained.

The order of work is strict:

```
1. This document, ratified
2. Seven crate PRDs, written together        -> freeze
3. Specs, against frozen PRDs                -> freeze
4. Contracts, complete per 5.3, party-checked -> freeze
5. Floor code: traits, types, trace           -> freeze
6. spu | harness | admin | gate, in parallel
7. Composition root, integration
```

Nothing at step N is written before step N-1 is frozen. The parallelism at step
6 is earned by the completeness of step 4 and the freezing of step 5, and by
nothing else. Contracts that are complete cannot be built against in parallel
while the floor beneath them is still moving.

## 11. Enforcement

No conformance graph is built during this program. The PRD to Spec to contract
to code chain is still enforced strictly, and it does not require a database. A
graph measures whether code matches settled intent. This program deliberately
unsettles intent, so the graph is early rather than slow. It is the instrument
for the phase after this one.

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
