# Load/Unload Path

**Status:** MERGED v0.3, 2026-08-05. Loop 0's composition, opened by the load event,
closed by the unload event, with every loop nesting inside its bracket. It composes
merged charters and contracts, binds no crate, and decides no seam. Where it and a
merged document disagree, the charter stands and this document is corrected, because
the charters are the decision record and this is their composition read back as one
motion.
**Revised:** 2026-08-05, first this date, on the operator's ruling that loop 0 leaves
the loop taxonomy's shelf. Loop 0 is the running agent service, the harness's own
mechanism initiated through admin, per the recut `weaver-harness-PRD` section 2
carries, and the loop taxonomy reaches loop 1 and above and never loop 0. A mechanism
is not a chapter in the operator's booklet, so this document leaves the `Loops/`
container, renames from `load-unload-loop` to `load-unload-path`, and files at the
project level outside the set, beside the other compositions that bind nothing. The
v0.2 header read this document as the outermost loop filed beside
`basic-inference-loop` under the 2026-08-01 filing ruling. That ruling stands for
loops and no longer reaches this document, which is not one. `Loops/` holds the basic
inference loop alone, and that loop running end to end is the proof-of-concept
milestone.
**Parent:** `weaver-harness-PRD`
**Editorial:** Per the Working Rules.

## 0. What this document is for

The lifecycle path runs through five crates, and a crate charter is the wrong
place to hold a path that runs through five of them. The verbs live in
`weaver-admin-PRD` section 4, the exchanges in `weaver-admin-harness-contract`
section 3, admission and release in `weaver-spu-PRD` section 4, the raise and the
lower in `weaver-harness-gate-contract` section 2, and the failure paths in every
one of those documents' own section 5. This document holds the composition: one
complete cycle, load to unload, with each refusal and both kinds of close walked
in order.

**The filing followed the type, and the type was corrected.** The v0.2 filing under
`Loops/` rested on the 2026-08-01 ruling that loops belong to the harness unless
specific to one domain and no other. That ruling stands and governs the booklet, and
this document left its reach when the operator ruled loop 0 out of the loop taxonomy:
what is composed here is the harness's own mechanism, driven through admin, not a
circuit an operator wires. The holder question it answered remains true of the
mechanism, since the harness authors both bracket events at both scopes and
provenance picks no organ, per `basic-inference-loop` section 2. The Document
Format's container entry carries the correction with this document's move.

## 1. Loop 0

The run is loop 0's bracket. `load` opens it, `unload` closes it, and the trace
grammar of `weaver-trace-PRD` section 2.1 is its record: session over run over
turn, the run bracket enclosing every turn and every turn enclosing its interior
events. The inference loop of `basic-inference-loop` runs inside that bracket, and
later loops nest the same way. The name keeps its zero because everything else
happens within it, and the taxonomy of loops the operator wires reaches loop 1 and
above and never loop 0 itself: the bracket is the mechanism the loops run inside,
not a circuit among them, and a loop that has not been opened by a `load` has
nowhere to run.

Whether loop 0 takes a type or a trait is the Spec pass's question and is
deliberately not reserved here. `weaver-traits` is demand-derived, and the engine
whose shape would demand a loop abstraction is not yet written.

## 2. The approach, which is not the loop

Six acts precede the loop, all the load invocation's, per `weaver-admin-PRD`
section 4.1: authorize the name, validate the configuration file, verify the
boundary, resolve the session and open the sink under root, ask the init system to
start the worker under the agent's `User=`, and dial the socket the worker binds
as its first act, retrying within a bound because the dial may arrive before the
bind. They run before the harness is serving, so they sit outside the trace by
construction, per `weaver-trace-PRD` section 3.1, and their record is admin's own
log, per `weaver-admin-PRD` section 2. Loop 0 opens where the record opens, at the
`load` event. A document that filed these steps inside
the loop would have the harness managing acts that predate it, which is the one
place the loop framing would strain, so the boundary is stated rather than
implied.

## 3. Opening the loop

Admin directs enter across its one seam, on the connection it dialed, supplying
the session identity, the run ordinal, the trace descriptor, the model binding,
and the gate instruction, per `weaver-admin-harness-contract` section 3. The
harness fans out along its own seams, in its own order:

1. **Stand up the empty working structure.** Nothing is projected, per the cut of
   2026-08-01, and what continuity a later run may hold is the enter cell
   `weaver-admin-PRD` section 10 names.
2. **Author the `load` event**, the record of admin's contact, the origin of the
   run's monotonic clock, and the opening of loop 0.
3. **Ask the SPU to admit the model binding**, carried uninterpreted. Admission
   is the one check on the device, per `weaver-spu-PRD` section 2, and it refuses
   rather than evicts.
4. **Start Gate last**, so no work arrives before the interior can serve it,
   ready sent only after the bind returns, per `weaver-harness-gate-contract`
   section 2.

The answer is the aggregate: ready when every arm has confirmed, or a refusal
naming where the fan-out stopped. Admin publishes loaded and idle on ready and on
nothing less, a partial load never published as loaded, per apex section 6.

## 4. Refusal, and the unwind

Every arm refuses with a typed reason the aggregate carries unchanged, per the
refusing-organ case of `weaver-admin-harness-contract` section 6. Where the
refusal lands against step 2 decides what the stream shows.

Before the `load` event is authored, nothing entered the stream and the run was
never opened. A refused admit leaves the device empty, per
`weaver-harness-spu-contract` section 5, and a refused bind leaves nothing held,
per `weaver-harness-gate-contract` section 5.

After the `load` event, the stream shows a bracket with no `unload`, which is a
truthful account of a load that did not complete rather than corruption to
repair, per `weaver-admin-PRD` section 5. A device conflict discovered at
admission is this case, arriving after the `load` in the fan-out's own order, per
ruling C.

The unwind is the invocation's reap plus one directive: direct leave where a run
was entered, stop the unit, close the sink it opened, and nothing durable of the
program's exists to remove. A rollback that cannot complete reports what it could
not undo and publishes no state. The reap runs inside the same invocation that
built, which is what makes it a reap rather than a later verb's cleanup.

## 5. Inside the loop

Two states, loaded-and-idle against active, per apex section 6, and the inference
loop is what runs in the second. Work enters only through the gate. The stop
exchange aborts the turn in flight and returns the agent to loaded and idle, the
turn closing with the stop reason marked and the run staying open, per
`weaver-admin-harness-contract` section 3. Stop never unloads. Loop 0 continues
until leave closes it, however many turns run and however many are stopped.

A fault the worker survives is a `fault` event, authored by the harness into the
stream like every other event, per the fault-carrier ruling of 2026-08-01. It
crosses no seam of its own: the operator's tooling keys on it from the stream and
comes back by running a verb, per `weaver-admin-operator-contract` section 6, and
the case set behind the kind is
deferred to the token workflow with the organs' charters.

## 6. Closing the loop

Admin directs leave. The harness unwinds in reverse: it stops Gate first, so
nothing new arrives and a gate process never outlives the interior it protects,
refuses while a turn is in flight rather than racing one, authors the `unload`
event, closing loop 0's bracket, drains the writer's queue to the stream, and
releases the SPU, residency ending and the device freed. The answer carries left,
meaning everything admitted reached the stream, per `weaver-admin-harness-contract`
section 4. The stream ends where the run did, finalized by nothing. Admin stops
the unit and publishes provisioned and unloaded, which is a different state from
absent, absent being reached by an operator act rather than a verb.

Closing the session is not closing the loop. One session may hold many passes of
loop 0, each its own run under admin's ordinal, and `session.closed` is content,
authored by the harness inside a load, stop, unload window, per `weaver-admin-PRD`
section 4.4, its cue the session-close cell that charter's section 10 holds.

## 7. The abrupt close

Process death ends loop 0 without closing it. The stream shows a `load` with no
`unload`, the writer's queue's tail is forfeited, per `weaver-trace-PRD` section
4.2, the device is reclaimed with the process, and the coordination socket dies
with the listener that bound it, so the next verb's dial finds nothing and the
death is observed as a refused connection rather than as a closure mid-exchange.
An invocation in flight when the worker dies observes the closure directly. Admin
repairs nothing on either. What a consumer makes of the
open bracket is that consumer's reading over the operator's storage. Timing picks
the report: before the enter aggregate is answered a death is a refusal on the
enter exchange naming the dead arm, and after it the death is the loss the
coordination seam observes, per `weaver-harness-spu-contract` section 5 and
`weaver-harness-gate-contract` section 5.

## 8. Open cells

Each cell names what settles it, and none is this document's to settle.

- **The fault case set.** The `fault` event's cases, per section 5. Awaits the
  token workflow's pass over the organs' charters, the fault-carrier ruling of
  2026-08-01 having settled the carrier and the kind.
- **What enter becomes without a record.** Rides the cell `weaver-admin-PRD`
  section 10 holds, including where the run ordinal survives across invocations.
- **The loop abstraction.** Whether loop 0 takes a type or a trait. Awaits the
  Spec pass, demand-derived rather than reserved, per section 1.
