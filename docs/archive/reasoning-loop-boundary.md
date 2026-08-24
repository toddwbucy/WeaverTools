# Reasoning-loop boundary

**Status:** ARCHIVED 2026-08-23. Its section 2 criterion was ratified by the
operator 2026-08-11 and the two-level model it states is carried by
`weaver-agents-PRD` in the apex's own words. Citations to this document were dropped
on that date. **The loop taxonomy settled 2026-08-23 supersedes the two-level model**
and is
recorded at `sketch-the-loop-taxonomy`, which is worth reading before taking this one
as current.

Previously: v0.3, 2026-08-11. Architecture-seat material, outside the document set,
landed by the authoring seat with the discussion's edits applied. **The criterion of
section 2 is ratified by the operator, 2026-08-11.** The one actionable item for the
harness lane is in section 4 and is owed, not done.

**Document ID:** `reasoning-loop-boundary`
**Companions:** `finding-loop-agent-split`, `reasoning-loop-is-and-is-not`,
`weaver-harness-PRD` section 5, and the figure `reasoning-loop-boundary.svg`.
**Editorial:** Per the Working Rules.

Changed from v0.2: the criterion's second clause is sharpened so connection to the
harness stops reading as the membership marker, the exception-free claim on tooling
names the mechanism as the spec seat's, and the verification-placement citation
resolves to a landed document. Changed from v0.1: v0.1 stated the boundary three
ways, as internal against external context, as hot path against cold, and as
necessary for reasoning. All three are true of the same object, which is why they
kept agreeing, and a document that states a boundary three ways has stated no
boundary, because a reader at the edge cannot tell which framing decides an
admission. This version names one criterion and demotes the other two to
consequences.

## 1. Where we are

Standing today, built and proven: admin, harness, and an SPU with a single decoder.
A model was loaded through the SPU, decoding ran, and output came back. That is the
moment the architecture stopped being a whiteboard and became a thing that runs.

Not built yet: any tooling, and memory. No tool has shipped, so nothing
tooling-shaped lives in the wrong place yet. That is exactly why this is the moment
to draw the line, before the first tool makes it expensive.

## 2. The criterion

What is being bounded is the semantic reasoning loop, Weaver's middle level, where
meaning gets processed. Not the technical level of moving bits and not the
effectiveness level of what output does in the world. The agent is not only its
reasoning loop, and what belongs in the loop and what belongs to the agent are two
separate questions. This document answers the first.

One criterion decides membership. Does the thing directly enhance the processing of
meaning. If it does, it is inside. If it only supplies material that the processing
of meaning then works over, it is outside and it is reached through the gate.

**Membership is decided by the criterion alone, and connection to the harness is a
consequence of membership rather than its marker.** The gate holds a harness channel
too, being an organ of the agent, and it sits outside the loop all the same: the
loop calls through the gate rather than sequencing it. What membership buys is being
sequenced by the loop as part of processing meaning, and every loop member connects
to the harness because the harness is where that sequencing lives.

The harness's own coordination passes the same test rather than riding on its
position. The harness holds the turn's content, assembling the working structure
into what the organs reason over next, so its driving is done with the meaning
rather than around it. A content-blind scheduler, moving opaque payloads on
triggers it never holds as content, fails the criterion however central its
position, which is the same fact the application-orchestrator refusal states at
the system's edge.

The minimum is small. A decoder that reasons over semantic content, a harness to
hold the loop, and enough state to keep a single turn coherent, which at the
minimum is essentially the KV cache. That much reasons. It reasons narrowly, but it
reasons, and it is what stands today.

Everything past the minimum is admitted by the same criterion rather than by a
second one. Memory is the first extension. It provides internalized context that
builds over time, which extends reasoning across turns instead of within one, so it
enhances the processing of meaning directly and sits inside. A visual or auditory
processing unit is a later extension, admitted for the same reason. Perception
feeds meaning, so a perception organ widens what can be reasoned over. It presents
a socket shape nothing else presents, and the harness does not care what the domain
is, only that the contract seats.

Tooling fails the criterion without exception, whatever mechanism the spec seat
elects for the crossing. A tool hands in externalized context. The loop reasons
over what a tool returns, and the tool does no part of that reasoning, so every
tool is on the far side of the gate. A calculator, a local shell, anything the loop
reaches out to for a fact it did not carry itself, is outside.

The transport is Unix sockets on both sides of the line. The line is architectural,
not a change of wire. Crossing it is a statement about what the thing is, not about
how bytes move.

## 3. Two consequences that are not the criterion

These were each proposed as the boundary and neither is. They hold as descriptions
of what the criterion admits, which is why they keep pointing the same direction,
and they must not be used to decide a hard case.

Internalized against externalized context describes what memory and tooling happen
to be. It does not decide, because a perception unit takes in external sensory data
and belongs inside anyway. Sorting on where the context originates would misfile
the next organ.

Hot path against cold path describes where things run. It does not decide either,
because memory is mostly cold and belongs inside. What it does give is the cost
model. The loop is racing the decoder to keep context in front of the model so it
never stalls, which keeps the KV cache hot, and the price of a crossing is why the
criterion is worth enforcing rather than merely stating. The economy runs across
three temperatures. Hot is the KV cache holding the live problem. When the cache is
dumped, state management catches it in a form the loop can return to cheaply, so a
dump is not a loss. When state management fills, it condenses into a memory
structure that reinjects context for similar problems later (Bucy, 2026).

The operative measure of that cost is crossings per decode step times the cost of a
crossing, not organs added. An organ loaded and rarely called is nearly free. An
organ on every hop is paid on every event. That measure sorts memory without
needing the temperature metaphor to rescue it.

Determinism was proposed as the key and dropped, because memory is not
deterministic and belongs inside anyway. Process isolation was proposed as the
basis and it is not the basis, it is the enforcement. Both should stay dropped.

## 4. The one item for the harness lane

Make the boundary a compile-time fact, not a discipline. Audit `weaver-types` and
`weaver-traits` for anything tooling-shaped currently reachable inside the loop,
and find whether the types and traits can be shaped so every tool call is forced
through the gate by construction. If the type of a tool result can only be obtained
from the gate socket, no one can wire a tool into the loop by accident, because
there is no type path that lets them.

This is compatible with the trace's opaque payloads and a reader of both documents
should be told so directly. The record splices content as bytes and the compiler
cannot see inside a payload, which is why vocabulary alignment is enforced at
author time rather than by types. That opacity is interior to a payload. What this
item needs is box granularity, a tool-result type obtainable only from one source,
and the type system holds box granularity fully. Nobody inspects a tool result's
fields to know it came through the gate.

The mechanism is named so the target is checkable rather than aspirational. Only
from one source means a construction door the gate crossing alone opens: no public
constructor, no conversion from another type, and no deserialization path
elsewhere in the floor, which is the absence-pinning the corpus already runs on
the peer identity. The known hurdle is registered with the audit: the conversation
model's own round trip currently deserializes everything it carries, and resolving
that tension is the audit's work, not this document's.

## 5. Building toward, not current

The extensions above are targets and they are one step past where the code stands.
Today only the decoder is real. The rest is the road, held here so a fresh session
orients to the destination without mistaking it for the odometer.

## 6. Parked for the spec seat

These are open and belong to the seat with codebase context, not to this one.

- Gate as general mediation surface. If all externalized context including a local
  shell reaches the loop through the gate, the gate is the single mediation surface
  for everything the loop did not bring itself, which is broader than an opaque
  logged pass-through for external tools. Does the port-as-discriminator rule still
  sort a calculator that binds no port against a shell that is invoked, when both
  are local and both are external to the loop. **Answered by the tool boundary
  ruling of 2026-08-18**: the shell is the gate's own outbound verb and the
  calculator is a `weaver-internal` callable dispatched inward, so the gate
  mediates the world and never the machine's own arithmetic, and the sort is by
  which side of the loop's membrane the answer comes from rather than by port.
- Organ-promotion bar. A cluster of external tools covering a domain may earn
  promotion to an organ when internal management pays for itself. That criterion
  drifts toward aesthetics. What is the measurement that says a cluster earned
  organ status rather than staying loose behind the gate.
- State-management scope. Is the state inside the loop the transient working state
  a single turn needs to stay coherent, or full agent state. Durable
  substrate-resident identity state is written through admin and lives outside. The
  working read is transient, unconfirmed.
- Loop floor crossings. The gate is not a loop organ, it is the boundary the loop
  calls through, and the owed loop-floor measurement has to state whether it
  includes the gate crossing or not. A message arriving from outside pays it, so
  the working read is that it should be included, and the number is not
  reproducible until that is written down.

## 7. Out of scope here

The admin face is an egress, not an instance of this criterion. The criterion sorts
what the loop reaches for. Admin is how the record leaves and how the harness is
started, and it is governed by `weaver-harness-PRD` section 5 rather than by this
document. The figure draws both because both are real. This document defines only
the inbound membership rule.
