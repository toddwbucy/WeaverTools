# weaver-harness / weaver-trace - contract

**Status:** DRAFT. Not ratified. Written with `weaver-trace-PRD` as one act, and
ratified with the document set rather than separately.

**Date filed:** 2026-07-29
**Document ID:** `weaver-harness-trace-contract`
**Parent:** `WeaverTools-PRD`, invariant 5.3
**Editorial:** ASCII, no em-dashes, no semicolons.

**This is a contract, not a Spec.** It states the protocol two parties agree to. It
names no Rust type, no module, and no function, because how either party implements
its side is that crate's Spec and no Spec is written until every PRD and every
contract is done.

---

## Parties

- **`weaver-harness`, the author.** Sole producer of trace content. Decides what is
  worth recording, when a session begins and ends, what a turn is, and when the
  record is flushed. Holds every descriptor.
- **`weaver-trace`, the recorder.** Assigns ordering, produces canonical form,
  commits durably, projects into the working structure, validates on read. Holds no
  policy and decides nothing about content.

No third party writes. Components elsewhere in the system report to the harness and
the harness authors. A crate that emits directly into the trace is outside this
contract and is a defect.

**This seam is a library boundary, not a wire.** `weaver-trace` is floor vocabulary
and the harness links it. A contract is not the same thing as a socket, and this one
governs an API surface rather than a protocol on a transport.

## Vocabulary

Every contract names the vocabulary it depends on, grouped by the crate that
defines it. A contract without this clause is not a valid contract, and a group is
stated even when empty, because an explicit nothing is an assertion someone checked
and an absent group is silence.

**From `weaver-traits`.** The message model. The `message.user`,
`message.assistant`, and `message.tool_result` payloads carry conversation messages
in whatever shape that crate defines, and this contract does not redefine them.

**From `weaver-types`.** Two fields of the agent state file, read at load and fixed
for the life of the run: the verbosity ceiling election of section 3, and the commit
policy that governs the boundary in section 4. Neither is interpreted here beyond
being carried.

**From `weaver-trace`.** The event envelope and its field set, the closed event-kind
vocabulary, the per-kind payload shapes, the three commit-boundary states, the
working structure as a projection, and the failure vocabulary of section 5. These
are the party's own definitions, named because they cross the seam and a reader of
this document should not have to infer where they come from.

**Nothing from any other crate.** This seam is a library boundary between two crates
and touches no third.

---

**The envelope field set appears here and in `weaver-trace-PRD` section 3,
deliberately.** A Spec is derived from its crate's PRD plus every contract that crate
is party to, so the harness Spec writer reads this document and never opens the trace
charter. Pointing there instead of enumerating would break that derivation for the
crate on the other side of the seam. The two copies are not redundant, because the
charter says what an event **is** and this document says what the harness
**supplies**, but they describe the same field set and can diverge. **If they ever
disagree the charter is authoritative**, and the divergence is a defect to file
rather than one a reader resolves by choosing.

## 1. What this contract governs

The production seam: how an authored event becomes a durable record and a queryable
present, what each party guarantees to the other, and what happens when either
fails.

It does **not** govern the content of events, which is the harness's, nor the
internal storage mechanism, which is `weaver-trace`'s, nor how any consumer reads a
finished artifact, which is a separate agreement.

## 2. Two exchanges

The seam carries two, and the second is easy to overlook because it happens once
per run rather than once per event.

### 2.1 Resume

Before the first event of a run is authored, the harness asks the recorder to
project the session's existing record into a working structure. The recorder reads,
validates, and projects, or refuses with a named failure and produces nothing.

For `run0` the record is empty and the projection is empty with it. For every later
run this is how the session continues, so **a refusal here fails the load** rather
than starting the run against a partial history. There is no degraded resume. An
agent that begins against an incomplete projection is one that has silently
forgotten part of its own conversation, and it has no way to discover that it did.

### 2.2 Emit

One authored event moves through four steps, in this order, always.

1. **Submit.** The harness offers an authored event carrying its kind, its payload,
   its session, run, and turn identity, its producing subsystem, its causal parent
   when it has one, and both timestamps of section 3. It does not carry a sequence
   number.
2. **Order and canonicalize.** `weaver-trace` assigns the sequence, produces the
   canonical bytes, and computes the payload hash over them.
3. **Accept or refuse.** The durable writer takes the event into a state the commit
   boundary governs, and returns the assigned sequence. Or it refuses, with a named
   failure and no partial effect.
4. **Project.** Only on acceptance does the event enter the working structure.

**The order is the guarantee.** Acceptance precedes projection so the volatile
present can never hold an event the record refused. A working structure containing
what the record does not is a divergence with no detection point, and the harness
would reason over history that will not survive a restart.

Acceptance means the event entered a state the boundary governs. It does not mean
every flush policy has completed.

## 3. What the harness owes

**Authorship.** It submits only events it authored. When another component reports
something, the harness turns that report into an event and submits it. The reporting
component does not submit.

**Identity the recorder cannot derive.** The session, the run, the turn key, the
producing subsystem, and the causal parent when one applies. `weaver-trace` has no
notion of a turn or of a load episode and must not acquire one. A run is bracketed
by the harness authoring its `load` and `unload` events, so the recorder could not
infer a run boundary even if it tried, because resume writes nothing.

**Both timestamps, stamped at authoring.** The recorder's own clock would read
commit or receive time, which is a later and different quantity, so it does not own
these fields.

- **Wall-clock, millisecond resolution, session-scoped.** The calendar question,
  when in the day this happened. Millisecond is deliberate rather than lazy: it is
  all the calendar question needs, and holding the resolution there keeps an NTP
  correction jittering a low digit from ever looking like a real event. Wall-clock
  is never used for interval arithmetic, so a step or a backward correction cannot
  corrupt a latency reading.
- **Monotonic, nanosecond source with a microsecond floor, run-scoped.** The
  measurement instrument. Its origin is the run start, captured at the `load` event,
  because monotonic time counts from an arbitrary origin valid only within one
  process lifetime and a run is one process lifetime. The origin lands on an event
  that already exists for the bracket rule, so one point serves two purposes reached
  from different directions.

**Monotonic readings compare only within a single run.** Across a run boundary
monotonic is undefined, and the fallback is wall-clock at millisecond. The three
fields sort by scope with no overlap: sequence is session-wide and is the order,
wall-clock is session-wide and is the calendar, monotonic is run-scoped and is the
microsecond instrument. No field is ever read for the one job it cannot do.

**Descriptors, never paths.** Every write target is supplied as an already-open
descriptor, obtained from `weaver-admin` before the worker drops privilege. The
harness never asks the recorder to open a path, and the recorder offers no way to.

**Policy.** The verbosity level for the session, the commit policy, and the flush
decision. The recorder applies them and does not choose them.

**Verbosity filtering at authoring time.** The floor is always authored. When the
ceiling is not elected, the harness does not submit the events above it. The
recorder records what it is given and never filters, so a recorder that drops an
event because it judged the level has taken policy the harness holds.

**Handling refusal.** A refused event is not emitted. The harness must not project
it, must not treat it as recorded, and must not retry it under a new sequence as
though it were a new occurrence.

## 4. What the recorder owes

**Ordering.** Sequence numbers are assigned by the recorder, strictly increasing
**within the session**, with no duplicates and no gaps among committed events. The
scope is the session and not the run, because a resuming run appends to the record
its predecessor started and continues its numbering. Monotonicity is guaranteed
against the session rather than against wall-clock time, and reads follow sequence
order.

**Canonical form.** One byte-form rule for every artifact it writes. Integer fields
that can exceed the double-safe range are written as decimal strings, so a consumer
parsing numbers as doubles cannot read a silently different value.

**A commit boundary that can be interrogated.** The last committed sequence, the last
accepted sequence, the commit policy in force, the maximum loss window that policy
permits, and the last commit error when one exists.

**Durability without silent loss.** When commit cannot keep pace, the recorder
blocks, spools, or fails loudly according to policy, and every pressure transition
is itself visible. It never drops an accepted event quietly.

**Deterministic projection.** The same record, schema version, and projection version
yield the same rows. The working structure is rebuildable exactly from committed
events, and rebuilding only reads.

**Validation on read.** Envelope version, known kind, sequence monotonicity, no
duplicate committed sequence, payload decodes for its kind, payload hash matches,
required fields present, coherent committed boundary, and no trailing bytes
masquerading as committed.

**Run integrity, checked at the same time.** Run 0 present, run numbers contiguous
from 0 with no repeats and no holes, and every run opening with its `load` event and
closing with its `unload` event. A run whose bracket is broken is corrupt even when
its number is present, so a number check alone is weaker than the property. This is
the run-level analogue of the gapless-sequence check, and a record showing runs 0, 1,
1, 2, 5, 8 fails on both counts.

A malformed record is refused rather than partially accepted. Authenticity is not
judged, because the operator owns what is submitted.

**Typed failure.** Every refusal is named. Nothing returns a partial result with a
success status.

## 5. Failure vocabulary

Named here so both parties agree on what each means. The representation is each
crate's Spec.

- **Refused on submit.** The event was not accepted and has no effect. Sub-cases:
  unknown kind, payload does not decode for its kind, required envelope field
  absent, sequence conflict.
- **Commit failed.** The event was accepted but the record could not commit it. The
  session is failed rather than continued, unless policy elects spooling.
- **Commit pressure.** Durable commit cannot keep pace. Resolved by the policy in
  force and surfaced as an event while the record remains writable, or as a fatal
  error when it does not.
- **Projection failed after acceptance.** The record holds the event and the working
  structure does not. **The recorder owns the resolution:** rebuild the projection
  from the record, or fail loudly. It must never continue against a silently stale
  present.
- **Record invalid on read.** Recreation refuses. Names the check that failed and the
  sequence range involved when known.
- **Resume refused.** The same validity checks applied at run start rather than at
  audit time. The consequence differs: an audit read that refuses leaves an operator
  to investigate, while a resume that refuses means the agent does not load. Both
  refuse whole and neither returns a partial projection.
- **Write target unusable.** A descriptor is closed, is not writable, or does not
  satisfy the required flags.

## 6. What neither party may do

**The recorder may not hold policy.** It does not decide what is worth recording,
what constitutes a turn or a session, when to flush, or what verbosity means. A
recorder that acquires any of these has taken cognition into the floor.

**The recorder may not offer a path-taking write surface.** Not as a convenience,
not for tests, not behind a feature. A path-taking function is a way around the
custody boundary, and its existence is the defect regardless of who calls it.

**The harness may not reach the artifact directly.** It writes through the recorder
and reads through the working structure. It does not open, seek, truncate, or
inspect the file, and it holds no path with which it could.

**Neither may write a second authoritative representation.** Derived views are
permitted when they name the committed source range they represent and do not claim
completeness beyond it. A derived view is never a durable home.

**Neither may make a span durable.** Spans are views over event ranges, produced on
demand. Writing a span into the record makes it a primitive and reintroduces the
duplication this seam exists to prevent.

## 7. Change protocol

This contract is one agreement. A change to the exchange in section 2, to either
party's obligations, to the ordering guarantee, or to the failure vocabulary is one
edit requiring both parties to re-ratify in the same act, together with any Spec
written against it.

Adding an event kind is a change to `weaver-trace-PRD` and to this contract, because
the kind set is closed and consumers key on it.

## 8. Conformance

How each check is implemented is Spec work. What must be checkable is stated here.

- A refused submission leaves no row in the working structure.
- A resume against a valid record yields a projection matching what the previous run
  held at its last committed event.
- A resume against an invalid record fails the load and produces no partial
  projection.
- Sequence continues across a resume rather than restarting, so a session spanning
  several runs has one unbroken numbering.
- A projection failure after acceptance rebuilds or fails, and never continues stale.
- Committed sequence is monotonic and gapless under concurrent authoring during a
  single run.
- Run 0 is present, run numbers are contiguous from 0, and every run opens with its
  `load` event and closes with its `unload` event.
- A monotonic reading is never compared across a run boundary.
- A truncated tail reads as uncommitted rather than committed.
- The working structure rebuilt from a record matches the live projection over the
  committed range.
- No write surface accepts a path. This is a compile-time property and is pinned as
  one, because a runtime test cannot demonstrate the absence of a function.
- Every descriptor the harness holds is close-on-exec and append-only. Also a
  compile-time property.
- Commit pressure under a hard cap fails loudly when no spool is enabled.
- A malformed payload hash is refused.
- No stored span appears in any record.
