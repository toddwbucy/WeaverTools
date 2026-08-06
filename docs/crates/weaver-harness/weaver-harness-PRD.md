# weaver-harness - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth. The crate PRD set is
written together and merged together, and no Spec is written against any member
before the whole set is merged. Ratification is the mapping of the whole document
set into the graph, and it belongs to the set rather than to this document.

**Date filed:** 2026-07-28
**Revised:** 2026-08-05, second this date, the socket inversion and the admin recut.
Per the operator: any socket connecting to the harness is an internal connection, so
this crate binds the coordination socket inside the agent's sandbox and listens,
where it adopted a handed end before, and refuses every dialing peer that is not
root. Admin is a role and a crate the operator runs with root rather than a service
account, so the long-lived party in an agent's lifetime is the init system. Section 2
carries the bind, section 4 carries the seam's authentication case, and the Spec's
section 2.3 carries the mechanism.
**Revised:** 2026-08-05, loop 0 named as the running agent service. Per the operator:
loop 0 is not a document set and not a milestone but the object itself, the sealed
agent that boots under its unit and holds its sockets, which is why it is seated in
this crate directly rather than in the loop container, and why a builder's drop-in
reaches loop 1 and above and never loop 0.
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

**Custody rests on possession of the passed descriptor and not on the receiving
identity.** An earlier reading of this section had the worker begin under the admin
principal, receive its descriptors, and drop, and it argued the ordering of that
drop. The drop is retired: the init system starts the unit at the agent identity, so
there is no interval in which worker code runs as anything else and the ordering has
no subject. What survives, and what this paragraph is kept to say, is the reason the
ordering never mattered. A descriptor passed over a Unix socket is a capability
rather than a permission, the kernel runs no permission check at the receiving end,
and possession is what custody of the sink rests on, so the worker appends to a file
its own uid could not open. This is the sink's mechanism and not the coordination
channel's, which authenticates by credential since the inversion. A charter that
says only that the
harness runs as the agent leaves the mechanism protecting the trace unexplained,
which is why it is stated rather than assumed.

## 2. What the harness owns

**Loop 0 is the running agent service, per the operator's ruling of 2026-08-05.**
It is not a document set, not a milestone to reach and pass, and not a loop a
builder supplies. It is the object itself: the thing that boots under its unit,
comes up as the statically provisioned agent identity, binds the coordination
socket inside its own sandbox and listens on it, creates the unnamed pairs its
organs are reached over, and sits there being one sealed agent. **It binds
exactly one name and listens on exactly one socket**, per the inversion ruling
of 2026-08-05: any socket connecting to the harness is an internal connection,
so the coordination socket lives inside the sandbox and this crate is the party
that creates it. It is dialable by construction and the credential check is what
refuses, root or refused, per `weaver-admin-harness-contract` section 2, where
the earlier form's unreachability did the refusing and could not tell an elected
tool from the worker. Nothing else here is dialable: the organ pairs have no
names. **That is why it is seated in this crate
directly and not in the loop container.** The container under `Loops/` holds the
agent's internal logic, which the running loop 0 service executes, and a loop 0
filed beside those documents would read as one more supplied loop rather than as
the service that executes them. **Both builder-facing surfaces are limited to
loop 1 and above and exclude loop 0**, not one of the two: the extension seam of
section 6, composed against and compiled into the worker binary, and the socket
binding the working list holds open, dropped in beside a running agent. Loop 0
is the service that runs a builder's loop under either surface, and is not
itself supplied through either.

**Loop 0, and the machinery loop 1 composes against.** The harness owns the
lifecycle interior of loop 0, the load and the unload, the same for every agent,
together with the engine machinery a loop is built from: tool dispatch, batch
partitioning, and the surfaces the rest of this section names. The loop itself,
loop 1, is the builder's, per the composability ruling of 2026-08-02: written at
the worker composition root against what this crate exposes, which is the
extension seam the children list of section 6 names, compiled into the worker
binary, and immutable there, so which loop an agent runs is which binary its
unit starts, a provisioning fact rather than a configuration field. What holds
for every loop 1 alike is loop 0's bracket discipline: the operator's interrupt
arrives as the stop exchange of `weaver-admin-harness-contract` section 3,
aborts whatever is in flight, and returns the agent to loaded and idle,
whatever the loop's own semantics. The basic loops this program ships are
demonstrations built by the same path, native in the same way, and they run
until the model returns a final answer or that interrupt arrives, which
describes the demonstrations rather than constraining a builder's loop, a
judgment loop or a fixed-turn loop being licensed by the same ruling.

**The tool system.** The registry, the execution context, and the permission modes.
Permission modes are operator policy and not a safety boundary. What a tool can
reach is bounded by the kernel, through the constrained user the tool executes as,
and the mode setting governs only whether the operator wants to be consulted before
a class of action. Stating this plainly is the point. A permission mode that reads
as a security control when the kernel is the actual control is a thing that gets
trusted wrongly later.

**Prompt assembly's deterministic floor, with the family render across the
seam.** The harness composes the canonical conversation: the identity prefix,
which is the system prompt together with the agent's fixed identity material,
then the session's message sequence read from the working structure, then the
tool schemas, in that order, always. What moved on the framing ruling of
2026-08-02, ratified with the token workflow's act, is the per-model half:
the family template's application, the phrasing layer that reliably elicits a
tool call from one decoder and not another, seats in the SPU's family library
per `weaver-spu-PRD` sections 13.4 and 14, because family knowledge lives in
one home and the harness links nothing of any model's. The harness sends
canonical messages, the family library renders, and the rendered reality
returns on the report path, template identity, token identifiers, and block
partition, so the harness authors what the model saw without having rendered
it. No model is a build or run dependency of this crate, now stronger than a
discipline: the per-model knowledge lives across a seam entirely. The
deterministic assembly floor stands alone, always.

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
out because the accumulated context made every call slow. **Proto-stateful in
this program means the agent begins each session with no accumulated experience,
holding real state only within one, per apex section 2. It does not mean the
decoder is driven statelessly.** The previous tree used the old word stateless
for cold full-history resend, the opposite architecture, and the rename of
2026-08-01 retires that collision along with the overstatement.

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
lifetimes are `weaver-admin`'s, which the operator runs with root, one invocation per
verb, per that charter's section 1 as recut on 2026-08-05. The party that is
long-lived where the harness is mortal is the init system, which holds the unit and
outlives every invocation that drives it. The harness is one of the things a load
assembles, not the thing that assembles it, and it cannot drive the early steps of
its own creation, because the worker spawn runs before the harness is running as the
harness at all. What the
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
to: agent-config
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
stream mechanics, the working structure, and the export
formatters are that crate's. The harness authors events. `weaver-trace` defines
what an event is and what it costs to commit one.

**The trait contracts go to `weaver-traits`, and the config file format to
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
where it has none. The coordination seam is a named socket this crate binds and
authenticates by credential, root or refused, per `weaver-admin-harness-contract`
section 2 as of the inversion of 2026-08-05. The other two are unnamed pairs this
crate creates at its organ forks and authenticate by possession.

The fourth is the seam to `weaver-trace`. It crosses no process line, so it is a
library boundary tagged `link` rather than `socket`, and it authenticates nothing
because there is no second process to identify. It is a seam under a contract all the
same, which is why the count here is four and the table below is three.

Every request carrying work across these seams carries the turn context and returns
it, per apex 5.2. Lifecycle directives on the coordination seam carry no turn context
because they belong to no turn, per 5.2 as scoped by the re-authoring of
2026-08-01.

The three sockets:

| Seam | Peer | The harness's role |
|---|---|---|
| Turn ingress | `weaver-gate` | Receives authenticated work. Gate never reaches past it. |
| Decode | `weaver-spu` | Opens the resident session, appends each turn's delta, and issues the flush. Requests carry `turn_key` and `session_key`. Consumes the response and its measurement payload. |
| Coordination | `weaver-admin` | Receives lifecycle sequencing, the trace descriptor of section 5, and the operator's intent to stop. Reports readiness, confirmation, and the turn's fate on a stop. Opens no exchange of its own, the fault travelling as a `fault` event on the stream per the fault-carrier ruling of 2026-08-01. |

**A fault the worker survives is a `fault` event, and the stream is its carrier.**
The fault-carrier ruling of 2026-08-01 retired the alert exchange: with one
outbound path carrying every event in order to the operator's sink, a second
carrier for the same fact earned nothing. The harness authors the fault like every
other event, no run blocks on anything downstream of the emission, and the
operator's tooling keys on the fault fields and comes back by running a verb, per
`weaver-admin-operator-contract` section 6.

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

**Sole writer means the harness authors whatever occurs, and filters nothing.**
`weaver-trace-PRD` section 5 carries no recording level since the ruling of
2026-08-02, so there is no class of event the harness declines to emit and no
policy for it to apply on the recorder's behalf. What an operator elects at load
governs what the agent produces, the residual readout being the reference case, and
the harness authors what production yields. A later run finding the file changed
adopts the new value as its own load condition, which is the mechanism working
rather than a conflict to refuse.

```graph
edge: writes
from: weaver-harness
to: session-record
```

**One emission, one rendering, two holders.** The outbound stream and the working
structure hold the same canonical rendering of the same authored emission, per the
ruling of 2026-08-01 that retired the relational projection: nothing is derived, so
nothing can diverge and no reconciliation is owed. The durable event schema is the
only schema, and there is no second representation and no view this program takes
responsibility for: a consumer who wants rows, an index, or a front end builds one
on the output, on the consumer's own compute.

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
`weaver-admin` under root, the role's principal, per
`weaver-admin-operator-contract` section 3, and the record the stream accumulates
lives on the operator's side of it. So the record is readable there with no
harness involved, replay and audit are
the operator's acts over the operator's storage, per `weaver-admin-PRD` sections 7
and 8, and the agent that produced it holds nothing of its own.

**The agent holds no route to the sink, twice over.** The first lock is the
operator's provisioning, where the sink is a file: ownership, mode, and the
directory's search bit deny the agent uid the path, verified by admin at load, so a
tool the agent elects, `bash` above all, cannot traverse to it. The kernel refuses
the path lookup before anything else is consulted. The second is the descriptor
discipline. **The harness never resolves a trace path.** The descriptor is opened by
a `weaver-admin` principal and passed to the worker over the coordination socket
using `SCM_RIGHTS`, so the harness receives a handle, never a path, and writes
through it. The agent is never told the name and could not act on it if it were.
Neither lock is redundant, because they answer different adversaries. The
provisioning stops the agent's tool surface from reaching the sink. The descriptor
discipline stops the harness from being the leak, since a component that never
resolves a path holds no path to disclose and cannot be induced to open one. The
write capability rides the descriptor rather than the mode, so custody takes
nothing the agent needs for its own home directory and work.

**The trace descriptor is close-on-exec, and the crate is broken without it.**

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

**What this crate itself raises, which the organs' enumerations cannot
cover.** The harness is an organ too, per apex section 5.4, the one whose
domain is coordination, so the `fault` event's case set is incomplete without
its sources and they are named here with the organs': the recorder surfacing
commit pressure while the sink stays writable, a stream write failing against
a live process, and an organ's death observed as channel closure after the
enter aggregate was answered. The third is why this enumeration cannot be
derived from the organs' own: a dead party is exactly the one that cannot
report, so the SPU's cases at `weaver-spu-PRD` section 13.10 and the gate's
at `weaver-gate-PRD` section 13.4 are silent about their own deaths by
construction, and the harness's observation is the only account there is.
With these three named the corpus-wide case set closes across all three
organs, and the payload's shape is the trace act's to elect against it.

**The harness reads, the model does not.** The harness reads the trace continuously,
because the working structure is its own copy of the record and every prompt
assembly is a read
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

- The extension seam where loop 1 composes, and the machinery it composes
  against.
- Prompt assembly, including the per-model scaffolding this layer is re-erected
  from for each decoder integrated into the SPU.
- The tool system, the execution context, and permission modes as policy.
- Activity control, covering stop, cancel, and interrupt.
- Trace authorship, covering the event set this crate writes and the descriptor
  discipline of section 5.

Contracts this crate is party to are written with the PRDs of their other parties,
one per seam in section 4, and are not children of this document.
