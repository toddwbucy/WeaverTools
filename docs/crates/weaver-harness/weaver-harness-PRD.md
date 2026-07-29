# weaver-harness - PRD (crate charter)

**Status:** DRAFT. Not ratified. The crate PRD set is written together and frozen
together, and no Spec, contract, or code is written against any member before the
whole set is ratified.

**Date filed:** 2026-07-28
**Document ID:** `weaver-harness-PRD`
**Parent:** `WeaverTools-PRD`
**Editorial:** ASCII, no em-dashes, no semicolons.

---

## 1. What the harness is

**One agent's interior coordinator.** One harness, one agent, one process, one uid.
The harness is a constituent organ of an agent rather than a resident service that
agents are loaded into and unloaded from. An agent whose harness outlives it is not
an agent, it is a session.

This is load-bearing rather than pedantic. The apex rests the entire regulation
model on the agent being an operating-system user, and that boundary only means
something if the user owns the whole assembly rather than a slot in something
shared. A harness serving many agents would put the regulated behavior of several
principals inside one principal, which is the arrangement the architecture exists
to avoid.

The harness holds control. It holds no substrate storage, binds no listening
socket, and implements no model. Decoder weights are resident in the SPU and
reached over the decode socket behind a provider interface, so the harness drives
generation without hosting it.

**The privilege window.** The harness runs inside the worker process, and the
worker does not run as the agent uid for the whole of its life. It is spawned by
`weaver-admin`, receives its trace descriptors while it still holds that principal,
and drops to `weaver-<name>`. Everything the harness itself
does happens after that drop. The distinction matters because custody depends on
the descriptors having been obtained before it, and a charter that says only "the
harness runs as the agent" leaves the mechanism that protects the trace
unexplained.

## 2. What the harness owns

**The agentic engine.** The query loop, tool dispatch, batch partitioning, and the
`QueryEvent` stream that is the rendering interface for every consumer. The loop
runs until the model returns a final answer or the operator interrupts it.

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
durable record back into context. Trace content reaches the model as the ordinary
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

**Lifecycle orchestration goes to `weaver-admin`.** Sequencing a load or unload,
collecting each organ's confirmation, rolling back a partial transition, and
supervising worker and gate lifetimes are `weaver-admin`'s, which is long-lived
where the harness is mortal. The harness is one of the things a load assembles, not
the thing that assembles it, and it cannot drive the early steps of its own
creation, because the worker spawn and the descriptor handoff run before the
harness is running as the harness at all. A crate chartered as one agent's interior
cannot also be the component that coordinates all agents, and the previous tree
carried roughly four and a half thousand lines of multi-agent coordination inside
exactly that contradiction.

**Network ingress goes to `weaver-gate`.** The harness binds no listening socket
and has no first-contact surface. Work arrives already authenticated.

**Provisioning, boundary custody, and the privileged startup window go to
`weaver-admin`.** The harness consumes the state file that admin produces and never
writes it.

**Model residency, GPU arbitration, decode compute, and the KV cache itself go to
`weaver-spu`.** The harness decides when a flush happens. It does not hold the
cache, and it does not reach into it. Eviction granularity is constrained by the
append-only session protocol and is settled in that crate's PRD and the decode
contract, not here.

**The trace primitive goes to `weaver-trace`.** The span type, the event schema, the
durable record mechanics, the working-structure projection, and the export
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

Every behavioral seam is a SO_PEERCRED Unix socket under a named contract, per apex
5.1. The harness is party to three, and every request across them carries the turn
context and returns it, per apex 5.2.

| Seam | Peer | The harness's role |
|---|---|---|
| Turn ingress | `weaver-gate` | Receives authenticated work. Gate never reaches past it. |
| Decode | `weaver-spu` | Opens the resident session, appends each turn's delta, and issues the flush. Requests carry `turn_key` and `session_key`. Consumes the response and its measurement payload. |
| Coordination | `weaver-admin` | Receives lifecycle sequencing and the trace descriptors of section 5. Reports readiness and confirmation. |

The harness links `weaver-traits`, `weaver-types`, and `weaver-trace` as vocabulary
and links no other internal crate. That is the whole dependency surface, and it is
checkable against this list.

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
`weaver-admin`'s initial contact as the session's first entry. One emission per event,
authored against the durable event schema, feeding the durable record and the
working structure together.

**One emission, three derivations.** The `QueryEvent` stream is the third, alongside
the durable record and the working structure, and it is not a second event system.
It is a lossy projection of the same authored emission, serving live operator
rendering only. A consumer that falls behind misses events, which is acceptable
because a dropped frame in an operator view is not a hole in the record. It carries
no measurement payload and it is never authoritative. This settles schema authority
rather than inheriting it: the durable event schema is the author, every other form
is a view, and a change to what a renderer wants cannot reach back and alter what
was recorded.

**Nothing on the turn path touches disk.** The harness reads its own trace from the
working structure in RAM. Even fast storage is too much latency to block a turn on,
so the durable commit runs off the hot path and a slow or failing disk consumer
never slows the interior read. The two are failure-isolated. What the durable side
may do under pressure is block, spool, or fail loudly. It may never shed silently,
because the trace is measurement data and a silently partial record renders a
plausible and wrong account of the turn.

**Custody is structural, not policy.** The trace file is owned by the agent uid and
belongs to the `weaver-admin` group. The group reads, the owner does not. So
`weaver-admin` reads the trace directly from the filesystem with no harness
involved, which is what makes operator-side replay and audit possible, while the
agent that produced it holds no read bit of its own.

Ownership alone would not hold this, because an owner may change a file's mode
whenever it can name the file. Two independent locks deny the name. The first is
the directory. The trace lives in a folder owned by `weaver-admin` and not
searchable by the agent uid, so a tool the agent elects, `bash` above all, cannot
traverse to the file to open it fresh or to change its mode. The kernel refuses the
path lookup before the file's own bits are consulted. The second is the descriptor
discipline. **The harness never resolves a trace path.** Descriptors are opened by a
`weaver-admin` principal and passed to the worker over the coordination socket using
`SCM_RIGHTS`, so the harness receives handles, never paths, and writes through them.
The agent is never told the name and could not act on it if it were. Neither lock
is redundant, because they answer different adversaries. The directory stops the
agent's tool surface from reaching the file. The descriptor discipline stops the
harness from being the leak, since a component that never resolves a path holds no
path to disclose and cannot be induced to open one. The write capability rides the
descriptor rather than the mode, so custody takes nothing the agent needs for its
own home directory and work.

**Trace descriptors are close-on-exec and append-only.** Two flags, one pin, and
the crate is broken without either.

Without `O_CLOEXEC`, every subprocess the harness spawns for a tool call inherits a
writable handle to the trace, and the agent edits its own audit record through a
descriptor requiring no path and passing no check.

Without `O_APPEND`, the handle can seek, so a write lands wherever the file
position sits and a holder of the descriptor can overwrite history rather than
extend it. With it, every write is placed atomically at end of file regardless of
position, and the record can be added to but not unwritten through that handle.
This is what makes append-only a property of the descriptor rather than of the
writer behaving well, and it is why a compromised harness can append falsehood but
cannot erase what already happened.

Both are invisible at runtime until exploited, and a test that opens a trace
normally will never see either. Together they are the highest-value compile-time
pin in the crate.

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

Contracts this crate is party to are authored in the contract pass, one per seam in
section 4, and are not children of this document.

## 7. Open ruling

**Does the encoder cross?** Written here as though it does not. The previous tree's
justification for the harness holding encoder weights in process was that a
cross-process hop on every memory read is dead cost, and memory is out of scope, so
that justification leaves with it. Under apex 7 the embedder serves no step of the
turn and is not in the closed observability set, so the criteria exclude it.

The reason this is recorded as open rather than settled is that the previous tree
ratified the opposite rule, that an agent without an embedder fails closed. That
rule was motivated by retrieval. Retiring it is a decision the operator makes rather
than one the criteria make silently.

If the ruling is that the encoder does not cross, apex section 6 needs correcting,
because it assigns `weaver-spu` encoder residency while the turn in apex section 3
never embeds.
