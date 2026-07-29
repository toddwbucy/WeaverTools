# WeaverTools - Vision

**Status:** LIVING. Not in the freeze set. This document changes as each stage of
development teaches what the next one looks like. It is never ratified and nothing
is written against it.

**Date started:** 2026-07-28
**Document ID:** `weaver-tools-vision`
**Editorial:** ASCII, no em-dashes, no semicolons.

Companion to `WeaverTools-PRD`. The PRD is design and is checkable against code.
This document is motive and is not. Where the PRD specifies one stateless turn,
this document holds the arc that turn is the first step of.

---

## 0. What this document is, and is not

The apex PRD builds a stateless Weaver agent and claims nothing beyond it. That
restraint is deliberate and it is correct. A design document earns its authority by
being falsifiable against the code that fulfills it, and a document that also
carried the long-horizon intent would tangle load-bearing near-term claims with
speculation nobody can check yet. That tangle is how the prior corpus lost its
coherence.

This document holds the intent the PRD sheds. It is the picture of where WeaverTools
goes after stage one, written so the near-term work stays clean by having somewhere
else to put the motive. It is living, not frozen. Its later stages are named, not
dated, because the shape of any stage is not knowable until the stage before it is
most of the way built.

One rule governs the boundary between the two documents. This document speaks in
biology, cortex and brainstem and hippocampus, because the organism model is the
clearest way to say what the architecture is for. The PRD never does. Biology
explains why the architecture is shaped the way it is. It never becomes a spec term.

## 1. The staged arc

Stage one is the agent that keeps a complete record and no memory: one turn, end to
end, through the gate, against a real local model, emitting a trace worth trusting.

The distinction is worth drawing carefully, because calling stage one stateless is a
convenience that misleads anyone who takes it literally. The trace is state. It is a
factual, sequence-faithful account of everything that occurred in a session, held in
RAM while the session runs, and the agent reasons over it, so turn two knows what
turn one did because the trace says so. What stage one lacks is not state. It is
everything that turns a record into a memory: selection, so that what matters is
kept and the rest is not, association, so that recall works by relevance rather than
by position, consolidation, so that many episodes settle into one structure, and
persistence, so that anything the agent can draw on survives the session at all. A
record answers what happened, in order. A memory answers what bears on now.

Something does survive, and saying otherwise would be sloppy. The durable record
outlives the process and the session both. It is a persistent audit store, written
for `weaver-admin` and for later analysis, and the agent has no path to it and no
mechanism that would read it back into a new session. So the claim worth making is
narrower and stronger than nothing survives, which a careful reader falsifies by
pointing at the file on disk. Nothing the agent can draw on survives, and that is
why there is nothing to individuate from, and why stage one tests nothing.

It is the apparatus floor, the Level A substrate of privileged low-latency
connectivity, per-agent OS individuation, and a trace worth trusting, on which the
later work becomes possible.

Later stages add the capacity to accumulate and use experience. They are not
specified here and not committed to an order, because the discipline of this project
is to decide architecture from measurement rather than ahead of it. What can be said
now is the shape of the growth, not its schedule: each new capability arrives as an
organ behind its own socket and contract, and the agent grows from a record toward a
memory rather than being rebuilt for it.

The destination is the test the whole program exists to run. WeaverTools is built to
test whether an agent whose boundary is other-produced can nonetheless self-produce
its individuation through accumulated experience (Bucy, 2026). An agent whose record
ends with its session cannot test this. The staged arc is the path from the floor that makes the test
possible to the agent that is its object of measurement.

## 2. The organism model

The agent is one body, and its parts are organs rather than modules. The framing is
not decoration. It carries a specific commitment: capability is grown when the body
needs it and only then, and the thing that coordinates the growth is written to
spend out new organs without cracking the chest open again.

The SPU, the encode and decode tissue, is the cortex. It is where tokens become
thought and thought becomes tokens. The harness is the brainstem. It keeps the
pulse, coordinates the involuntary function, and holds the loop. Every organ reports
to it, and it is the sole writer of the trace and the sole broker of access to the
trace. That centrality is what lets it grow the body outward: a new organ is a new
socket and a new contract onto the brainstem, not surgery on everything already
there.

The organs that arrive later are filtration and judgment. A classifier or a small
activation network that filters prompts or queries is a kidney or a liver, an organ
that decides what passes and what does not. The memory leg, drey together with the
consolidation pass, is the hippocampus, the organ that turns momentary experience
into lasting memory. None of these exist in stage one, because a body with no
bloodstream needs no kidney, and an agent whose record ends with its session has no
accumulation to settle. The architecture's whole claim is that when the body needs
an organ it grows one, rather than having been built around a slot left empty in
anticipation.

The order in which organs arrive is not arbitrary either, and this is the lesson
the prior attempt paid for by not noticing it. Life carried no persistent memory
long before it carried any, though never without working state: even bacterial
chemotaxis keeps a few seconds of methylation memory to compare conditions now
against a moment ago, the working-state floor the trace is the agent's version of.
Reflex arcs preceded learning. The brainstem is older than the cortex, and the
cortex is older than anything that consolidates experience into lasting structure.
Nothing evolved a hippocampus first and then worked out how to metabolize. Memory
is a late organ everywhere it appears, because memory is a property added to a
working loop rather than a substitute for having one.

The prior attempt built the memory leg and the turn loop at the same time and spent
its final months subtracting one from the other. That was not a resourcing failure,
it was an ordering failure, and the same dependency shows up in both places. An
agent that cannot complete a turn has nothing to remember, and an organ that turns
experience into structure needs experience to work on. The record before the memory
is not a compromise forced by scope. It is the sequence.

## 3. The hardware ceiling

An organism does not grow organs it cannot perfuse. This is the constraint that
keeps the growth model honest rather than open-ended (Bucy, 2026). Because the
substrate of a Weaver agent is the device it runs on, the ceiling on how many organs
an agent can carry is hardware you can budget for rationally. VRAM, compute, and
memory bandwidth are the blood supply. An agent does not sprout an activation
network it cannot fit alongside the cortex, and a deployment that wants more organs
buys more substrate or accepts fewer.

This is not a limitation to engineer around. It is the property that makes the
organism model tractable. A growth model with no ceiling is a wish. A growth model
bounded by a resource you can measure and provision is a plan.

## 4. The protoautonomic path

Autonomic behavior is the harness acting without being asked, and the model seeking
that action without being prompted to. Neither half exists in stage one, and neither
can be conjured by architecture alone. The harness can be built to inject, but the
model has to be trained both to accept injected input in the right circumstances and
to emit a signal when it is reaching for that input. That training needs data, and
the data is what stage one quietly produces.

The mechanism is trace shaping. An ordinary tool call, named and structured with
care, leaves a trace of close to the right shape. The calculator is the
deterministic reference: the harness injects a computed result into the stream in
place of a stochastic one, and the trace of that injection is the seed of the
pattern. A Hades-backed retrieval tool, reached as an ordinary call out through the
gate rather than as an internal store, widens the set from arithmetic to retrieval
shapes. Prompt and context engineering coax these calls into a simulacrum of the
autonomic behavior, close enough that the traces can be sanded into real SFT
examples. The simulacrum comes first, and the trained behavior is built from it.

This is why the calculator in the PRD is named protoautonomic rather than autonomic.
It is step one toward the behavior, tied to it by name, and it makes no claim to be
the finished thing.

## 5. Reinjection and the first judgment organ

A cheap-looking win is available the moment the trace lives in RAM at the token
level: pack past the context limit, clear space, and reinject earlier material from
the working structure. Dumb recycling, keep everything you can, is a harness policy
and nothing more. Selective reinjection is a different animal. The moment something
decides what is worth carrying forward and what to drop, that decision is the seed
of consolidation, and it is the first judgment in the system that is about
relevance rather than order.

So reinjection is not a PRD line. It arrives as its own organ. A small activation
network or a classifier, trained on trace data, watches the trace, pre-caches
portions of it ahead of time, and injects on the right tag.

That organ creates no state. It selects over state the trace already holds, which is
why it can stand on the in-RAM working structure before drey is ever provisioned. It
is the first of the four properties from section 1 to arrive, and it arrives alone.
There is still no association, no consolidation, and nothing the agent can draw on
that survives the session. What changes is that the agent stops carrying its whole record and starts
carrying what bears on now, which is the beginning of a memory and not yet one.

Two questions are open here and are left open deliberately, because they are
architecture decisions and there is not yet a measurement to decide them from:

- Whether the reinjection organ is a classifier or a small activation network.
- Whether it lands before or after drey.

## 6. Individuation and safety as one primitive

The strongest property of the OS-level substrate is that agent individuation and
agent safety are the same mechanism seen from two sides. The UID that isolates one
agent from another is the same UID that bounds what that agent can touch. The kernel
that keeps two agents out of each other's state is the kernel that keeps a single
agent's tool calls inside their permitted reach.

This is what makes tool safety a solved problem rather than a novel one. An agent
that runs as a constrained Linux user is bounded by filesystem permissions, sudoers,
PAM, and cgroups, the same instruments a system administrator has used for decades
and can audit without learning anything new. Operationalizing agent safety becomes
connecting the agent's identity to LDAP or Active Directory, not inventing a
classifier that adjudicates danger from the model's disposition. The enforceable
constraint lives in the OS, where it can be checked, rather than in a heuristic,
where it can only be hoped for.

Stated at the level of the program, this is the Level A claim in operational dress:
per-agent OS individuation is not only how agents are told apart, it is how they are
made safe.

## 7. What this document does not commit

The value of this document depends on it not hardening into design. It names organs
that do not exist, an order of stages it does not fix, and questions it does not
answer. That is the point. A vision that committed to the classifier over the
activation network, or fixed drey before the reinjection organ, would be guessing at
designs that have not been written, which is the exact reserved-slot move the PRD
forbids.

So this document grows as the work teaches it. Each stage that completes turns one
of its open questions into a settled decision, and that decision moves into a PRD and
out of here. The vision shrinks toward the built system as the built system grows
toward the vision. When they meet, this document has done its job.
