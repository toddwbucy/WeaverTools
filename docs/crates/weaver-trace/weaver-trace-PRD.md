# weaver-trace - PRD (crate charter)

**Status:** DRAFT. Not ratified. The crate PRD set is written together and frozen
together, and no Spec or code is written against any member before the whole set
is ratified.

**Date filed:** 2026-07-29
**Document ID:** `weaver-trace-PRD`
**Parent:** `WeaverTools-PRD`
**Companion contract:** `weaver-harness-trace-contract`, written with this document
**Editorial:** ASCII, no em-dashes, no semicolons.

---

## 1. What this crate is

`weaver-trace` is the definition floor for the primary artifact. It depends on no
other Weaver crate, binds no socket, and holds no cognition. It defines what an
event is, records events durably, projects them into a queryable present, and
validates a record on read.

**It does not produce the trace.** The harness authors. This crate is the
mechanism the harness authors through, and the distinction is the whole of the
charter. `weaver-trace` decides nothing about what is worth recording, when a
session begins, what a turn is, or when to flush. Every one of those is policy and
every one is the harness's. This crate guarantees that what it is handed is
recorded faithfully, ordered correctly, and readable afterward.

The previous tree had no document governing production at all. It had a schema
contract, an access contract, and a draft custody contract, and nothing binding the
sole producer on how it produces. Every defect that corpus records is on the
producer side, which is why this charter is organized around production and why it
is written together with the contract that binds it.

## 2. The artifact

### 2.1 Three nested units

**A turn** is one request through to its final answer, bounded by `turn.started`
and `turn.closed`.

**A run** is one residency working on a session. `run0` creates the session.
`run1` and after resume it. A run ends at unload or at process death, and the
identifier is an ordinal within the session, so which run produced an event is
answerable from the event alone.

**A session** is the continuity itself, spanning one or more runs. It is the unit
the agent's conversation belongs to, and it is the boundary statelessness is
defined against: a new session begins with nothing, and a session that has been
running for a week is still one session.

### 2.2 The session has two materializations

The **working structure** is the session in RAM: the volatile relational present,
a deterministic projection of committed events. It is what the harness reasons
over, which makes it state rather than a report about state. It lives for one run.

The **durable record** is the session on disk: append-only, sequence-faithful,
canonical, and outliving both the process and the run. It is a persistent audit
store that `weaver-admin` reads directly and the agent has no path to.

One session is one record. A run appends to it, so the record accumulates across
every run of that session and the working structure is rebuilt from it at the start
of each.

**A record is not a file.** Today it is exactly one, because there is no rotation,
but the two nouns are kept apart deliberately. If a record ever spans more than one
file its parts are **segments**, and nothing has to be renamed on the day that
happens. Defining the word costs a sentence. Discovering that half the corpus used
`record` to mean `file` would cost considerably more.

### 2.3 Recreation is how a session resumes, not only how it is audited

The durable record recreates the working structure exactly. That property is
usually stated as an audit guarantee, and it is one, but it is first a **live
mechanism**: rebuilding the working structure from the record is what lets a
session continue after a restart. `run1` opens the session's record, projects it,
and appends. Session resume is recreation, and there is no second path for it.

This is why recreation accepts any well-formed record including an operator-provided
copy, and why the projection must be deterministic rather than approximately
faithful. A resume that produced slightly different rows would be an agent that
remembers its own conversation slightly wrong.

### 2.4 Spans

Spans are neither materialization and they are not stored. A span is a view over a range
of events, derived on demand for interoperability with tools that speak spans. It
has no durable home, no residency, and no authority. The previous tree wrote closed
spans into its durable log as events, which made spans a durable primitive, which
inverted its own ruling that a derived span is never the durable home, and which
left a separate span export that was a projection of data already stored in
projected form. Its own crate Spec could not resolve the resulting redundancy. This
charter resolves it by honoring the original rule: events are durable, spans are
derived, and nothing is stored twice.

### 2.5 Schema authority The single emission is authored against the durable event
schema. The working structure is a projection rather than a second author, so its
projection version governs only how events become rows and may be bumped without
changing what is emitted. A change to the durable event schema is the breaking
change, and the projection declares which event-schema versions it consumes.

## 3. Events are typed, and that is a safety property

An event carries an envelope and a payload. The envelope identifies the session, the
run, the turn, the sequence, the kind, the producing subsystem, the causal parent,
and **two timestamps**: a session-scoped wall-clock stamp at millisecond resolution
for the calendar question, and a run-scoped monotonic reading at nanosecond source
with a microsecond floor for interval measurement. The two are not
interchangeable and neither answers the other's question, which is why there is no
single occurrence time. `weaver-harness-trace-contract` section 3 states which party
stamps them and why the scopes differ. The payload shape is determined by the kind,
and the kind vocabulary is closed.

**This enumeration appears in the contract as well, deliberately.** A Spec is
derived from its crate's PRD plus every contract that crate is party to, so the
harness Spec writer reads the contract and never opens this charter. The contract
must therefore carry the field set rather than point at it. The two copies are not
redundant, because this one says what an event **is** and the contract's says what
the harness **supplies**, but they assert a property of the same thing and can
diverge. **If they ever disagree, this charter is authoritative** for the envelope,
because this crate is the definition floor for the artifact and consumers outside
that contract read the record. A divergence is a defect to file rather than one a
reader resolves by choosing.

**The emission path is typed values, not instrumentation macros.** This is a design
requirement rather than an implementation preference, and the reason is a defect
the previous tree documented against itself. Its span layer took field names as
literal tokens at the emit site, so nothing connected an emitter to the declared
attribute vocabulary. The result was that four of the most operationally valuable
measurements in the system, the decode timing fields, appear nowhere in the
vocabulary at all, and a span kind outside the closed set was emitted at three
sites. The Spec's own conclusion was that the vocabulary could not be read as the
schema.

A typed event with a closed kind and a typed payload cannot carry an invented
field, because the compiler refuses it. The apex asks for compile-time pins where a
runtime test structurally cannot reach. This is one, obtained by construction
rather than by adding a check that someone must remember to run.

Adding an event kind is a change to this charter and to the contract. It is not a
free extension, because consumers key on the closed set.

**`load` and `unload` are event kinds, and they are the run bracket.** Entering a
run writes its `load` event and leaving it writes its `unload` event, both authored
by the harness. The minimal run is those two events with nothing between, so **a run
cannot be empty**, and that is what makes run numbers verifiable rather than merely
declared. A missing run number is a load episode whose bracket events vanished,
which is corruption of the same kind as a sequence gap.

The bracket is defined here beside the run field deliberately. A run label that
could be stamped without the events existing to verify it against would be an
assertion with nothing behind it, which is the shape of every defect in the previous
tree's attribute vocabulary.

**A run edge is authored, not inferred.** Its boundaries are known at the moment
each bracket event is written, because resume writes nothing and the envelope's run
field is the only place a boundary is recorded. No consumer reconstructs a run by
range analysis.

## 4. What producing the trace requires

### 4.1 Create, or resume

A record is opened by `weaver-admin` while the worker still holds that principal,
and the descriptor is passed to the worker before it drops to the agent uid.
`weaver-admin` resolves which session is being loaded. The harness never learns the
path either way.

**`run0` creates.** The record is new and empty, and the working structure begins
empty with it.

**Every later run resumes.** The record already exists, and before the first event
of the new run is authored it is read, validated, and projected into a working
structure. Only then does the run append. A resume that cannot validate its record
is a failed load rather than a degraded start, because an agent that begins against
a partial history is one that has silently forgotten part of its own conversation.

The two cases differ only in whether the record was empty. There is no separate
resume path, no second projection mechanism, and no reconstruction from anything
other than the record.

**This crate's write surface accepts descriptors, never paths.** That is the API
consequence of the custody model, and stating it here is what makes custody
expressible rather than merely intended. A crate that offers a path-taking write
function offers a way around the boundary, and the previous tree carried exactly
that: a trace-root resolver with zero production callers whose layout described a
path no artifact ever used, while the security invariant three other documents
cited was specified against it.

Every descriptor is close-on-exec and append-only. The first keeps tool
subprocesses from inheriting a writable handle. The second makes append-only a
property of the handle rather than of the writer behaving well.

### 4.2 Update

One emission per event. The order is fixed and is not an implementation detail,
because the failure it prevents is a working structure holding events the record
refused:

1. the harness submits an authored event,
2. this crate assigns the sequence and produces canonical bytes,
3. the durable writer accepts the event or refuses it with a typed failure,
4. only on acceptance is the event projected into the working structure.

Acceptance means the writer took the event into a state the commit boundary
governs. It does not mean every flush policy has completed.

The commit boundary has three states. **Committed** is visible to recreation and
export. **Pending** is accepted but not durable, and is the loss window the deployer
elected. **Failed** is not committed and is surfaced. A restart reads committed
events only.

**Silent shed is forbidden.** When commit cannot keep pace with emission the
implementation blocks, spools, or fails loudly, and its choice and trigger
conditions are visible. A trace is measurement data, and a silently partial record
does not produce a gap a reader can see. It produces a coherent account of a
session that did not happen.

**Nothing on the turn path touches disk.** The harness reads the working structure
in RAM. The durable commit runs off the hot path, and a slow or failing durable
consumer never slows the interior read. The two are failure-isolated.

**Canonical form is one mechanism, used everywhere.** Integer fields that can exceed
the double-safe range serialize as decimal strings. Nanosecond values exceed it by
roughly two hundred times, so a reader parsing them as doubles gets a silently
different number with no error and no way back. The previous tree got this right in
its record and wrong in its span export, which is what happens when two artifacts
carry the same field under two rules.

**The reason is resolution, not overflow.** Read as overflow avoidance the rule
looks like defensive rounding, and the natural response is to store fewer digits.
That response destroys the measurement. The monotonic clock resolves to nanoseconds
with a microsecond floor because the harness runs orders of magnitude faster than
the decoder, which pays network-class millisecond latency, and local latency is the
reason the model was brought next to the services in the first place. A record that
cannot resolve finer than the fastest thing it measures makes that advantage
invisible. This is systems-architecture timing, not network timing, and the decimal
string exists to carry the digits rather than to dodge an exception.

### 4.3 Maintain

**One session is one record, and there is no rotation.** However many runs a
session spans, they append to the same file. Rotation would make a record a set of
files, which would put an ordering and enumeration problem into recreation, and
recreation is the mechanism session resume depends on. If a session ever runs long
enough that one file is a problem, rotation arrives as a schema and manifest
extension. It is not built in advance of that.

A run ends at unload or at process death. Ending a run is not ending the session,
so the record is flushed and left open to the next run rather than finalized.
Finalization belongs to the session, not the run.

Close is ordered: drain, then checksum, then write the manifest. The manifest is
written last because it describes a finalized artifact, and a manifest that
describes a file still being written describes nothing.

The manifest carries the committed sequence range, the artifact hash, the run count,
whether the ceiling was enabled, and a completeness status. **A producer emits
status and never a conclusion.** Whether an artifact is usable is a reading a
consumer performs, and absence of a completeness block is never read as complete.

## 5. Verbosity

Two levels, **floor** and **ceiling**, and they add rather than exclude.

**The floor is always recorded and cannot be switched off.** It is what the turn
needs to run: the turn brackets, the message sequence, and enough of the tool events
to carry results into the next iteration. It is derived rather than chosen, because
the harness reasons over the working structure and an event the turn depends on is
not elective. Nothing below the floor is a quieter agent. It is a broken one.

**The ceiling is elected per agent in its state file** and takes effect at load. It
adds the measurement payloads with their token identifiers and entropies, the decode
boundary, and the residual reductions when readout is enabled. The cost is real and
the election is the operator's, per load.

Three consequences. **The manifest records whether the ceiling was enabled**, or
elected brevity and silent loss look identical to a reader, which defeats the
completeness status entirely. **Replay requires the ceiling**, because the token
identifiers and sampler parameters it needs live in the measurement payload, so a
floor-only session has to be reproduced rather than diagnosed. And the election is
fixed for the life of a load, because a session with a verbosity discontinuity is one
every consumer then has to reason about.

## 6. What this crate guarantees

- Canonical byte form, one rule, all artifacts.
- A strictly increasing, gapless sequence over committed events, **scoped to the
  session** and therefore continuous across every run that appends to it. The word
  monotonic is reserved in this charter for the clock of section 4.2. The sequence is
  the order and the clock is the instrument, and reading either for the other's job
  is an error the contract names explicitly.
- A crash-safe commit with an explicit committed boundary.
- Deterministic projection: the same record, the same schema version, the same
  projection version, the same rows.
- Exact recreation of the working structure from any well-formed record, including
  an operator-provided copy. Recreation does not privilege the original file.
- Run integrity on read: run 0 present, run numbers contiguous from 0 with no
  repeats and no holes, and every run opening with its `load` event and closing with
  its `unload` event. A broken bracket is corruption even when the number is
  present, so this is stronger than a number check.
- Mechanical validity checking on read, and a typed refusal rather than a partial
  result. Authenticity is not judged here, because the operator owns what is
  submitted.
- Typed failure for every refusal. Nothing fails silently and nothing returns a
  partial result with a success status.

## 7. The seam

The production seam is bound by `weaver-harness-trace-contract`: the two exchanges,
what each party supplies and guarantees, the ordering rule that acceptance precedes
projection, the failure vocabulary, and the prohibitions on both sides. It is read
alongside this charter and neither is complete without the other.

This section was a list of what the contract had to settle, written before the
contract existed. It is now a third partial copy of a document that says it better,
so it is a pointer instead.

## 8. What does not cross

**The stored span export.** Spans are derived on demand. There is no second
artifact.

**The reserved span kinds.** The previous tree retained sleep and reconciliation
kinds so its enum could keep parsing archived artifacts. There are no archived
artifacts here, so the reason does not transfer and the kinds do not cross. This is
consistent with the apex rule against carrying slots for designs not yet written,
and it is a different reason than that rule gives.

**The attribute-name vocabulary as a convention.** Typed payloads replace a module
of name constants that emitters were free to ignore.

**Post-hoc annotation by consumers.** The previous tree let its analysis crate write
spans into a finished trace after the run, which meant a reader could not assume
every span in an artifact came from the producing process. The harness is the sole
writer, and an analysis artifact is a separate file.

**Memory, in every form.** No consolidation events, no belief vocabulary, no
substrate. The event kinds that serve those exist when their emitters do.

**Redaction, at write time.** The recorder never filters, so it never scrubs. The
harness authors what it authors and its only filtering mechanism is the verbosity
election of section 5, and a recorder that dropped or altered content because it
judged the payload would have taken policy the harness holds. There is no third
place on the write path where redaction could occur, so the record is raw by
construction rather than by an unmade decision. Scrubbing, if ever wanted, is a
formatter election on export and downstream of everything here. What an operator
does with a delivered artifact is theirs.

**Embeddings.** Nothing in the stateless turn retrieves by similarity, so a vector
recorded here would have no consumer. A payload field whose only reader is unbuilt
is a reserved slot in data form, and the schema is where that rule is easiest to
break, because adding a field feels smaller than adding an interface. Embeddings
enter the record when something reads them.

## 9. Open ruling

**The interoperability target.** Spans are derived for tools that speak spans, and
the previous tree's format borrowed OTLP field names without being OTLP, then
carried a claim that a trivial converter could bridge them which turned out to be
untrue and unverified. Whether the derived view targets OTLP specifically, and what
conformance record backs that claim, is not settled here.
