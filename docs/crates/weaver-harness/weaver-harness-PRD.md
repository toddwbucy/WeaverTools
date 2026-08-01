# weaver-harness - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth for now. The crate PRD set is
written together and merged together, and no Spec is written against any member
before the whole set is merged. Ratification is the mapping of the whole document
set into the graph, and it belongs to the set rather than to this document.

**Date filed:** 2026-07-28
**Revised:** 2026-07-31. Section 4 is restated against the revised apex 5.1, records
that the harness opens exchanges on the coordination seam and holds the alert, and
drops the stale claim that the admin contract is an unwritten stub. Corrected on
review from three seams to four, the trace link seam having been uncounted since the
charter was written. Revised again the same day: section 1's argued drop ordering is
retired with the drop itself, and section 3's orchestration paragraph is split
between the transition, which is admin's, and the fan-out inside the directives,
which is the harness's. Revised a third time the same day: section 2 cites the
interrupt's channel. Per the rulings carried by
`basic-inference-loop`. Revised a fourth time the same day: section 4's seam-record
paragraph resolves the decode seam through `weaver-harness-spu-contract`, landing
the edit registered in `weaver-spu-PRD` section 11, whose entry leaves in the same
act, and section 5's custody paragraph reads the record by the service through
custody and the operator through the shared group, replay and audit being the
operator's acts. Revised a fifth time the same day, the subtraction batch. The live
view is retired under ruling A, its node, its exit, and its receive-side obligation
leaving sections 2, 4, and 5, and the ruling that introduced it overturned. The
integrity witness is retired under ruling B, the answered-here paragraph and the
O_APPEND argument leaving section 5, close-on-exec standing on its own grounds, and
the derivations counting two.
**Revised:** 2026-08-01, a sixth entry. Section 4's seam-record paragraph resolves
turn ingress through `weaver-harness-gate-contract`, the gate pair merged per the
human's ruling of this date.
**Revised:** 2026-08-01, a seventh entry, the durable-record cut. Section 5's
custody prose restates from the admin-owned file to the operator's sink, per the
ruling at `weaver-admin-operator-contract` section 3: the disk paragraphs become
stream paragraphs, the manifest citation leaves the verbosity clause, and the
descriptor discipline stands unchanged as the half that never depended on who
persists the record.
**Document ID:** `weaver-harness-PRD`
**Parent:** `WeaverTools-PRD`
**Editorial:** Per the Working Rules.

---

## 1. What the harness is

**One agent's interior coordinator.** One harness, one agent, one process, one uid.
The harness is a constituent organ of an agent rather than a resident service that
agents are loaded into and unloaded from. An agent whose harness outlives it is not
an agent, it is a session.

```graph
node: weaver-harness
kind: crate

edge: parent
from: weaver-harness
to: WeaverTools
```

This is load-bearing rather than pedantic. The apex rests the entire regulation
model on the agent being an operating-system user, and that boundary only means
something if the user owns the whole assembly rather than a slot in something
shared. A harness serving many agents would put the regulated behavior of several
principals inside one principal, which is the arrangement the architecture exists
to avoid.

The harness holds control. It holds no substrate storage, binds no listening
socket, and implements no model of any kind. Model weights are resident in the SPU and
reached over the decode socket behind a provider interface, so the harness drives
generation without hosting it.

**There is no privilege window.** The harness runs inside the worker process, and the
worker runs as the agent uid for the whole of its life. `weaver-admin-PRD` section 7
has admin holding no capability of its own and asking the init system to start the
worker as a transient unit carrying the agent's `User=`, so the process begins at
`weaver-<name>` and never holds anything above it. Everything the harness does happens
under that identity, and the descriptors arrive into it.

**Custody rests on possession and not on the receiving identity.** An earlier reading
of this section had the worker begin under the admin principal, receive its
descriptors, and drop, and it argued the ordering of that drop. The drop is retired:
under the delegation above there is no interval in which worker code runs as anything
but the agent, so the ordering has no subject. What survives, and what this paragraph
is kept to say, is the reason the ordering never mattered. A descriptor passed over a
Unix socket is a capability rather than a permission, the kernel runs no permission
check at the receiving end, and possession is what custody rests on, so the worker
appends to a file its own uid could not open. A charter that says only that the
harness runs as the agent leaves the mechanism protecting the trace unexplained,
which is why it is stated rather than assumed.

## 2. What the harness owns

**The agentic engine.** The query loop, tool dispatch, and batch partitioning. The
loop runs until the model returns a final answer or the operator interrupts it, the
interrupt arriving as the stop exchange of `weaver-admin-harness-contract` section 3.

**The tool system.** The registry, the execution context, and the permission modes.
Permission modes are operator policy and not a safety boundary. What a tool can
reach is bounded by the kernel, through the constrained user the tool executes as,
and the mode setting governs only whether the operator wants to be consulted before
a class of action. Stating this plainly is the point. A permission mode that reads
as a security control when the kernel is the actual control is a thing that gets
trusted wrongly later.

**Prompt assembly.** The harness composes what the decoder sees: the identity
prefix, which is the system prompt together with the agent's fixed identity
material, then the session's message sequence read from the working structure, then
the tool schemas. Assembly is **per-model and does not transfer**. Phrasing that
reliably elicits a tool call from one decoder does not from another, so this layer
is re-erected for each model integrated into the SPU rather than written once and
reused. No model may become a build or run dependency of it. The deterministic
assembly floor stands alone, always.

**Decode against a resident session.** The harness issues decode requests over the
decode socket against a resident KV session rather than resending the conversation
each turn. The identity prefix is rendered once per session and held resident. Each
turn appends only its delta. The harness owns the flush decision while the SPU owns
the cache, so "flushed on the harness's terms" is an obligation of the decode seam
rather than a local policy. Permanence of the identity prefix is an invariant of
that same seam, honored by the SPU, not a guarantee this crate can hold. The
harness does not touch the cache and so cannot protect any region of it.

The alternative this rules out is resending the full message history every turn.
The previous tree ran production on that path and measured a single exploration
turn climbing from 5,988 to 24,932 prompt tokens, with a concurrent request timing
out because the accumulated context made every call slow. **Stateless in this
program means the agent begins each session with no accumulated experience. It does
not mean the decoder is driven statelessly.** The two readings produce opposite
architectures, and the previous tree used the word for the second one.

Assembly is also where the model's view of the trace is settled. The harness
reasons over an in-RAM working structure that holds the whole trace, and it renders
from that structure only the message sequence. The measurement events, the
lifecycle events, and the custody records stay in the harness's working state and
never enter a prompt. This is a discipline of the engine and not a property of the
model, and it is the seam a later recall feature would breach by rendering the
record back into context. Trace content reaches the model as the ordinary
conversation and in no other form.

The message model is provider-agnostic. The concrete transport is constructed at
the worker composition root and injected, so this crate names no wire format.

**Trace authorship.** The harness is the sole writer of the trace and the sole
broker of access to it. This is a first-order responsibility of the crate rather
than instrumentation attached to one. Section 5 governs it.

**Activity control.** Starting, stopping, cancelling, and interrupting a run inside
a loaded agent. Every one of these returns the agent to loaded and idle. None of
them unloads it. Activity is the only lifecycle layer the harness owns.

## 3. What the harness does not own

**The lifecycle transition goes to `weaver-admin`, and the fan-out inside it comes
back.** Authorizing a load or unload, opening the record, starting and stopping the
worker unit, rolling back what its own acts built, and supervising worker and gate
lifetimes are `weaver-admin`'s, which is long-lived where the harness is mortal. The
harness is one of the things a load assembles, not the thing that assembles it, and
it cannot drive the early steps of its own creation, because the worker spawn and the
descriptor handoff run before the harness is running as the harness at all. What the
harness does own is the interior of the enter and leave directives: admin holds one
seam and no channel to the SPU or the gate, per `weaver-admin-PRD` section 6, so the
harness fans admin's directive out along its own seams, collects each organ's
confirmation, and returns one aggregate. Sequencing the organs is the harness's
because the seams are, and the previous tree carried roughly four and a half thousand
lines of multi-agent coordination inside the opposite reading.

**Network ingress goes to `weaver-gate`.** The harness binds no listening socket
and has no first-contact surface. Work arrives already authenticated.

**Boundary verification and lifecycle supervision go to `weaver-admin`.** Provisioning
is the operator's rather than admin's, per `weaver-admin-PRD` section 1, and there is
no privileged startup window to assign. The harness reads the configuration file the
operator produces and never writes it.

```graph
edge: reads
from: weaver-harness
to: agent-state-file
```

**Encoding and decoding both go to `weaver-spu`, and so does everything that
serves them.** Model residency, GPU arbitration, decode compute, embedding compute,
and the KV cache are that crate's. The encoder is not a harness component that
happens to live near the decoder. It is the other half of the semantic processing
unit and is held to the same discipline: the same residency accounting, the same
GPU arbitration, the same lifecycle confirmation.

**The harness routes.** It sends work to the SPU and receives tokens or embeddings
back, and which of the two it asked for changes nothing about the seam. It holds no
weights, performs no forward pass, and has no in-process model of either kind. A
harness that hosts an embedder for latency reasons has taken half the SPU into the
interior and given up the arbitration that made residency accountable.

The harness decides when a KV flush happens. It does not hold the cache and does
not reach into it. Eviction granularity is constrained by the append-only session
protocol and is settled in that crate's PRD and the contract, not here.

**The trace primitive goes to `weaver-trace`.** The span type, the event schema, the
stream mechanics, the working-structure projection, and the export
formatters are that crate's. The harness authors events. `weaver-trace` defines
what an event is and what it costs to commit one.

**The trait contracts go to `weaver-traits`, and the state file format to
`weaver-types`.**

**Safety adjudication of tool input goes to the kernel and is not a harness
function at all.** The harness does not inspect a command to decide whether it is
dangerous. It executes the tool as the agent's constrained user and the kernel
decides what that user can touch. There is no classifier here and none is coming,
because a classifier would be a heuristic standing where an enforced boundary
already stands.

**Memory in every form is out of scope.** No recall, no consolidation, no
surfacing, no substrate. When memory returns it returns as a socket peer under its
own contract, per apex 5.1, and this crate gains a seam rather than a dependency.

## 4. The seams

The harness is party to four seams, and every one of them is governed by a named
contract, per apex 5.1. Three cross a process line and are Unix sockets that
authenticate their peer, by credential where the channel has a name and by possession
where it has none. The coordination seam is a connected pair with no name and
authenticates by possession, per `weaver-admin-harness-contract` section 2. Which case
the other two fall under follows from process topology, which no document in the set
states yet.

The fourth is the seam to `weaver-trace`. It crosses no process line, so it is a
library boundary tagged `link` rather than `socket`, and it authenticates nothing
because there is no second process to identify. It is a seam under a contract all the
same, which is why the count here is four and the table below is three.

Every request carrying work across these seams carries the turn context and returns
it, per apex 5.2. Lifecycle directives on the coordination seam carry no turn context
because they belong to no turn, and the scope of 5.2 is owed a restatement that says
so, filed in `weaver-admin-PRD` section 11.

The three sockets:

| Seam | Peer | The harness's role |
|---|---|---|
| Turn ingress | `weaver-gate` | Receives authenticated work. Gate never reaches past it. |
| Decode | `weaver-spu` | Opens the resident session, appends each turn's delta, and issues the flush. Requests carry `turn_key` and `session_key`. Consumes the response and its measurement payload. |
| Coordination | `weaver-admin` | Receives lifecycle sequencing, the trace descriptors of section 5, and the operator's intent to stop. Reports readiness, confirmation, and the turn's fate on a stop. Opens its own exchanges to raise `harness-alert`, which is the direction that makes the seam duplex. |

**The harness raises alerts and does not assume where they land.** An alert is its
own exchange on the coordination channel, opened by the harness, and the emit point is
not designed on the assumption that the record is its only sink, so a push transport
on the deployment track is a future seam rather than a future rewrite of this one. An
alert that cannot be written because the channel is full or closed is dropped and the
harness continues, per `weaver-admin-harness-contract` section 6, and the drop is
noted in the record so that a run with no alerts and a run whose alerts were lost stay
distinguishable afterward.

The harness links `weaver-traits` and `weaver-types` as floor vocabulary, and links
`weaver-trace` as a member of its own domain under a contract. It links no other
internal crate. The three are one dependency surface and two classifications, which is
what the block below projects: two `floor-link` records and one `seam`. Calling all
three floor vocabulary would use the word `WeaverTools-PRD` section 5.1 reserves for
what every domain draws from and no domain contains. That is the whole
dependency surface, and it is checkable against this list.

```graph
edge: seam
from: weaver-harness
to: weaver-trace
via: weaver-harness-trace-contract
tag: link

edge: floor-link
from: weaver-harness
to: weaver-traits

edge: floor-link
from: weaver-harness
to: weaver-types
```

The socket seams above carry no records here. A seam without the contract that
governs it fails G3 rather than passing incompletely, and none is in that state now:
the turn ingress seam resolves through `weaver-harness-gate-contract` as of the gate
pair's merge on 2026-08-01, and the decode seam through
`weaver-harness-spu-contract`, each declared from the organ's side per the organ
rule of Document Format section 4, with no record declared here on either crate's
behalf.

The coordination seam is no longer in that state. `weaver-admin-harness-contract` is
written and declares the seam from admin's side, per the Document Format rule that on
an organ channel the organ declares. That contract is the `via` this seam
resolves through, and no record is declared here on admin's behalf.

## 5. Trace authorship and custody

The trace is the primary artifact and this crate authors it. Three rules govern
that, and they are separable.

These rules defend against one adversary, the model reaching for its own trace
through legitimate operation, whether an elected tool call or a recall path that
renders the record back into context. Against that adversary the walls below are
complete, because the model receives only what the harness hands it. They do not
defend against a compromised harness. Owning the deterministic engine is owning the
agent, and no trace custody changes that. That risk is bounded where multi-agent
isolation is bounded, at the operating system, by uid separation and least
privilege. Custody stops what the model can reach through normal operation. The
kernel stops what a compromise would reach. Neither asks the model to be
trustworthy.

**The harness authors, components report.** No other component writes to the trace.
The SPU returns its generation and measurement payload tagged with the `turn_key`
it was given, and the harness writes `model.request`, `model.output`, and
`model.measurement`. The harness writes `turn.started` and `turn.closed` around
every turn, the message events, and the tool-call events. It records the
`load` event that opens each run, which for `run0` is both the session's first
entry and the record of `weaver-admin`'s initial contact. One emission per event,
authored against the durable event schema, feeding the outbound stream and the
working structure together.

**Sole writer means sole enforcer of verbosity.** `weaver-trace-PRD` section 5 defines
a floor that is always recorded and a ceiling elected per agent, and the recorder holds
no policy, so nothing but the harness can decide that a ceiling event is not emitted.
The harness reads the election from the agent state file at every load and applies it
for that run, and it authors the run's level into the run's own events so the stream
states verbosity per run and elected brevity and silent loss stay distinguishable. A
later run finding the file changed adopts the new value as its own load condition,
which is the mechanism working rather than a conflict to refuse.

```graph
edge: writes
from: weaver-harness
to: session-record
```

**One emission, two derivations.** The outbound stream and the working structure
receive the same authored emission, and neither is a second author. This settles
schema authority rather than inheriting it: the durable event schema is the author,
the projection is a view, and a change to what a view wants cannot reach back and
alter what was recorded. There is no third derivation and no view this program takes
responsibility for: a consumer who wants a front end builds one on the output, on the
consumer's own compute.

**Nothing on the turn path waits on the sink.** The harness reads its own trace from
the working structure in RAM. Even a fast sink is too much latency to block a turn
on, so the stream write runs off the hot path and a slow or failing sink never slows
the interior read. The two are failure-isolated. Under pressure the stream side
reports its growing queue rather than absorbing it, `weaver-trace-PRD` section 4.2
leaves it no cadence to elect and no window to tune, and what may happen next is the
marked election of `weaver-admin-operator-contract` section 3. It may never shed
silently, because the trace is measurement data and a silently partial account
renders a plausible and wrong story of the turn.

**Custody is structural, not policy.** The stream's sink is opened by
`weaver-admin` under its own principal, per `weaver-admin-operator-contract`
section 3, and the record the stream accumulates lives on the operator's side of
it. So the record is readable there with no harness involved, replay and audit are
the operator's acts over the operator's storage, per `weaver-admin-PRD` sections 7
and 8, and the agent that produced it holds nothing of its own.

**The agent holds no route to the sink, twice over.** The first lock is the
operator's provisioning, where the sink is a file: ownership, mode, and the
directory's search bit deny the agent uid the path, verified by admin at load, so a
tool the agent elects, `bash` above all, cannot traverse to it. The kernel refuses
the path lookup before anything else is consulted. The second is the descriptor
discipline. **The harness never resolves a trace path.** Descriptors are opened by
a `weaver-admin` principal and passed to the worker over the coordination socket
using `SCM_RIGHTS`, so the harness receives handles, never paths, and writes
through them. The agent is never told the name and could not act on it if it were.
Neither lock is redundant, because they answer different adversaries. The
provisioning stops the agent's tool surface from reaching the sink. The descriptor
discipline stops the harness from being the leak, since a component that never
resolves a path holds no path to disclose and cannot be induced to open one. The
write capability rides the descriptor rather than the mode, so custody takes
nothing the agent needs for its own home directory and work.

**Trace descriptors are close-on-exec, and the crate is broken without it.**

Without `O_CLOEXEC`, every subprocess the harness spawns for a tool call inherits a
writable handle to the trace, and the agent edits its own audit record through a
descriptor requiring no path and passing no check.

The flag rides the descriptor and does not cross with a passed handle, so the
receiving call is the only place it can be supplied and supplying it is this crate's
obligation rather than the sender's. `weaver-admin-harness-contract` section 5 holds
that obligation.

It is invisible at runtime until exploited, and a test that opens a trace normally
will never see it. **It is not reachable by a compile-time pin.** A behavior on the
receive path is not a type property. What a pin does reach is the shape of the
receive: one receive site, taking no flag argument, returning a handle the rest of
the crate cannot construct another way. That pin is real and it is a different claim.
The flag itself takes the perturbation-verified test of apex section 11, and the test
counts only when it has been watched to fail with the flag removed.

**The harness reads, the model does not.** The harness reads the trace continuously,
because the working structure is its projection and every prompt assembly is a read
of it. That is harness-internal work on the harness's own state. Sole broker names
the harness holding the only handle, not a service it offers the model. There is no
model-facing read path, and the pin in section 2 is what keeps that true: trace
content reaches the model as the rendered message sequence and in no other form.
This is what apex section 4 means by writing the trace where the agent cannot reach
it, stated precisely. The tool surface has no path to the file, and the model has no
path to the content.

## 6. Children

Specs to be written against this charter once the PRD set is ratified. Named here
so the set is bounded, not drafted here.

- The engine and the query loop.
- Prompt assembly, including the per-model scaffolding this layer is re-erected
  from for each decoder integrated into the SPU.
- The tool system, the execution context, and permission modes as policy.
- Activity control, covering stop, cancel, and interrupt.
- Trace authorship, covering the event set this crate writes and the descriptor
  discipline of section 5.

Contracts this crate is party to are written with the PRDs of their other parties,
one per seam in section 4, and are not children of this document.
