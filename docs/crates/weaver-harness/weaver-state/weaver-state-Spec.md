# weaver-state - Spec

**Status:** MERGED. In `main` and the source of truth.

**Revised:** 2026-08-25, second of this date, the record's name outlives its
claim. `state-preload-door-stands-only-diagnostic` reads as the whole two-sided
fact and this half asserts the narrower one, that the member binds no name it is
not given. Named as owed rather than renamed here, a rename reaching every
citing document and the conformance header code will carry.
**Revised:** 2026-08-25, first of this date, the door's claim names its other
half. Section 4's
`state-preload-door-stands-only-diagnostic` is one side of a two-sided claim and
said so nowhere: the member binds no name it is not given, and that the name is
given only under a diagnostic binding is `weaver-admin`'s, recorded there as
`admin-preload-name-follows-the-kind` in the act of this date. Both records now
name the other, so a reader of either meets the pair rather than a claim that
looks whole.
**Revised:** 2026-08-24, second of this date, the seal parks the replay
ask. Section 4 gains the seal as a per-standing fact and the `replay` ask's
mechanics: where the member stands with the preload door the ask answers
only at a seal, the fact held apart from the transport so a dead driver's
close answers nothing, and immediate where the door does not stand, the
query the recall's generalized past the four message kinds. One
perturbation assertion lands.
**Revised:** 2026-08-24, first of this date, the preload door takes its
mechanics. Section 4
gains the second door of `weaver-analysis-state-contract`: one landing
path for both doors, which is the indistinguishability claim made
structural, a kind-conditioned standing, and an inverted credential
judgment. Two perturbation assertions land, the door's conditional standing
and the wrong-peer refusal, and the name's route to the member is elected
in the code act per section 2's own pattern. The preload opener's
retirement makes the shared path idempotent at the preload grain, the
delete hanging on that opener alone.

**Revised:** 2026-08-20, custody answers within its session. Section 4
gains the serve restriction: every read bounds to the session the
contract's amended opener carries, which is `weaver-state-PRD` section 4's
within-a-session boundary made a property of the answers rather than an
assumption about the file. Both queries had read the whole table and
answered across every session a store held. One perturbation assertion
lands. Section 6's retirement cell narrows to the disk alone, the operator
having ruled removal a separate act.
**Revised:** 2026-08-19, third of this date, the recall bound keys the
whole turn identity. Review of the arc's code act found the bound as
spelled, distinct turn values alone, recalls an older run's events
wherever a turn label recurs across runs. Section 4 resolves the bound as
the distinct session, run, and turn triples of the most recent turns by
id.
**Revised:** 2026-08-19, second of this date, the assertion records land.
The code act of 2026-08-18 shipped three conformance headers citing
assertions no document declared, which is the defect the counting clause
exists to catch, found by the position refresh of 2026-08-19. The three
records land here under the sections that argue them, each tagged
`review`: their tests demonstrate the property's good half and no
perturbation yet forces the failing half, so a stronger tag would claim
an instrument that does not exist.
**Revised:** 2026-08-19, the serve half is represented. Section 4 gains the
ask handling and the shape query, section 3's provisional store shape
passes its stated trigger and stands with the reasoning recorded, and
section 6 closes the serve surface election, the transformation vocabulary
gaining its first member. Arrives with the context-injection loop's act,
per the contract's change protocol.
**Date filed:** 2026-08-18
**Document ID:** `weaver-state-Spec`
**Parent:** `weaver-state-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

How the custodian is represented: its process, its store, its territory, and
the shapes both halves of the seam take. Written from the merged corpus
alone. The charter carries every why, and where reasoning appears here it
restates a charter clause and cites it. The serve half stood deliberately
absent until its first asker shaped it, per the charter's cell, and arrived
with the context-injection loop's act of 2026-08-19 at section 4.

## 1. The crate

A binary crate, one process per session member, spawned at load and retired at
unload while its holdings stand, per `weaver-state-PRD` section 3. The
charter's one sentence is asserted at the crate: custody without policy,
nothing judged, ranked, or initiated, which review checks by reading the
crate's surface for any door a judgment could enter by.

```graph
node: state-custody-without-policy
kind: assertion
tag: review

edge: asserts
from: weaver-state
to: state-custody-without-policy
```

It links
`weaver-trace` for the canonical event vocabulary its ingest parses and no
other internal crate: the floor's wire trio does not cross this seam, the
distillate being the seam's own vocabulary per the contract, so the floor
links would be dependencies nothing here consumes.

**Dependencies, external.** `rusqlite` with its bundled engine, so the store's
version is the build's fact rather than the host's, pinned by the lock file
like every dependency. `serde_json` for the canonical event JSON the ingest
reads. `nix` for the credential check and descriptor handling the seam
requires. Nothing else: no async runtime, no logging crate, no HTTP, per the
corpus's standing refusals.

## 2. The process and its territory

The member runs under its own account, owning one subdirectory in the
operator-side territory where the session record lives, per the charter's
custody ruling: the store opens by path and keeps sibling files, so custody
is by ownership, mode-locked against the agent's uid. The subdirectory holds
the store's file and whatever siblings the engine keeps beside it, and
nothing else writes there.

The seam's end arrives the way every stood channel's does, at load, and the
peer is judged by credential before any traffic is read. The exact descriptor
choreography follows the standing pattern of the worker's other channels and
is elected in the code act that stands the member up, because a numbering
elected before the spawn path exists would be guessed rather than derived.

## 3. The store

**The engine is sqlite, per the operator's representation ruling of
2026-08-18**, carried in the charter with its measured grounds. The file is
`state.sql` in the member's territory, opened or created at load, reopened by
later loads of the same session, and retired with the session.

**Two tables, and the shape is provisional with a stated trigger.** The
distillate lands as an event row and its elected pairs:

```sql
CREATE TABLE IF NOT EXISTS event (
    id       INTEGER PRIMARY KEY,
    session  TEXT NOT NULL,
    run      TEXT NOT NULL,
    turn     TEXT,
    kind     TEXT NOT NULL,
    sequence INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS field (
    event_id INTEGER NOT NULL REFERENCES event(id),
    key      TEXT NOT NULL,
    value    TEXT NOT NULL
);
```

The envelope's five ride the event row, `turn` nullable because the record
carries turnless events, and every elected pair is a field row holding the
key path and the value as the canonical JSON spelled it. This is the simplest
shape that keeps custody whole and every row attributable, and it is elected
against the ingest alone on purpose: the serve surface is unshaped, so a
shape optimized for queries nobody has asked would be optimizing a guess.
**The trigger is the serve act**: when the first asker's shapes land, this
election is reconsidered against real asks, and a build that has served real
asks without reconsidering has an open election reading as a settled one.
**The trigger fired 2026-08-19 with the shape ask, and the election
stands.** The first real ask groups the event table by run and kind, which
the two-table shape answers in one pass over rows a session keeps in the
low thousands, at a cadence of one ask per run's opening, so a query-side
reshaping would buy nothing measurable and the provisional shape is kept on
that ground. No index is added for it, by the same arithmetic. The next ask
that arrives reopens the question under the same clause.

**The indexes are built at load from the election the seam's opener
carried**, per the contract's ingest clause: the opener arrives before the
first distillate on every standing of the channel, so a restarted member
rebuilds the identical index set before it holds a single new row. Per the
charter: the
envelope's standing indexes on `(run, turn)` and `(kind, sequence)`, and one
index per elected key path on `field (key, value)` filtered to that key,
so extension within a session is rows accumulating under standing indexes
and a new election is a new load's new index set. The engine's automatic
index machinery is not relied on, because an index that appears when a query
happens to want it is a cost landing mid-serve rather than at load.

```graph
node: state-indexes-built-at-load
kind: assertion
tag: review

edge: asserts
from: weaver-state
to: state-indexes-built-at-load
```

**Durability yields to speed, and the charter is the license.** The
derivative is rebuildable from the record and the session never depends on
it, per the loss clause, so the store runs with synchronization relaxed and
the journal in memory, the crash cost being a rebuild or an empty stand
rather than a lost account. The exact pragmas are the code act's, under this
election.

## 4. The ingest and the serve

The seam's traffic is the contract's `election` opener, its `distillate`
stream, and since 2026-08-19 its `ask` and `answer`, and this crate's half
is mechanical: parse, insert, index nothing per event, answer exactly what
was asked. **A distillate lands whole or not at all**: the
parse completes before any write, and the event row and its field rows go
in as one transaction that rolls back entire on any failure, because a
distillate held in part would be an attributable envelope over missing
pairs, a corruption custody cannot detect later. A distillate that does not
parse is dropped whole, per the contract's malformed-row clause, and the
defect waits for the serve direction to give its surfacing a voice. Inserts
ride the sequence order the harness owes, and a gap in sequence is not this
crate's to notice: the record is the account
of what happened, and custody keeps what arrives.

```graph
node: state-distillate-lands-whole
kind: assertion
tag: review

edge: asserts
from: weaver-state
to: state-distillate-lands-whole
```

**The serve half, shaped by its first asker per the charter's cell.** An
`ask` frame arriving on the stream is handled in stream order by the same
loop that lands distillates, which is what delivers the contract's
answered-against clause without a lock or a snapshot: the holdings at the
ask's position are the holdings, because nothing lands between reading the
ask and answering it.

**Every serve query restricts to the opener's session, and the restriction
is the query's rather than the caller's.** The contract's `election` carries
the session the load declared, per its 2026-08-20 amendment, and this crate
holds it for the channel's life and puts it in the `WHERE` of every read
below. It is stated here as a shape rather than left to a reader because
the defect it repairs was invisible: both queries once read the whole table,
answering across every session a store file had ever held, and the answers
looked perfectly well formed - a shape ask reporting a lifetime's runs as
this session's, and a recall reaching a fact the operator believed a session
cut had retired. Nothing surfaced it until a fresh session reported
twenty-eight earlier runs it never had. **A store file holding more than one
session is the normal case rather than the broken one**, sessions outliving
runs and the file outliving sessions, so the restriction is what makes
`weaver-state-PRD` section 4's within-a-session boundary a property of the
answers instead of an assumption about the file. What becomes of an earlier
session's rows on disk is section 6's open question and deliberately not
settled here, per the operator's ruling of the same date: unreachable is
what this act delivers, and removal is its own election.

```graph
node: state-serve-restricts-to-the-session
kind: assertion
tag: perturbation

edge: asserts
from: weaver-state
to: state-serve-restricts-to-the-session
```

The `shape` ask runs one grouped count over the event
table, and the landing order the contract's first-seen clause asks for is
the `id` column's, custody's own order key: the run groups are ordered by
the least `id` each holds, each carrying its kinds and their counts as the
envelope spelled them, rendered as the contract's answer frame and written
back on the channel as one answer frame, the frame's byte shape riding the
encoding election of section 6. The `recall` ask reads the event rows of
the four message kinds with their field pairs, ordered by the `id` column
like every landing-order answer, and where `last-turns` bounds it the
bound resolves as the distinct session, run, and turn triples of the most
recent turns by id, the rows outside them left unread, a turn label
recurring across runs naming two different turns. The answer serves each
event as the distillate's own shape, envelope and pairs, because custody
serves what it kept in the form it kept it. A malformed ask is dropped whole the
way a malformed distillate is, and the resulting silence is the harness's
bound to convert into a missing answer.

**The preload door lands its distillates through the same path the first
door does, and that is the mechanism of the contract's indistinguishability
claim.** A distillate arriving on the preload channel parses, transacts, and
lands exactly as one arriving from the tee, one code path and one store, so
nothing marks how a holding arrived and the serve restriction binds to the
preload opener's session the way it binds to the harness opener's. **The one
act the preload path adds is the opener's retirement**: receiving the
preload election deletes the declared session's event and field rows in the
same transaction that records the opener, before any distillate lands, per
the contract's section 2. The path is thereby idempotent at the preload
grain - re-running it replaces the session's holdings rather than appending
to them - and a dead driver's prefix needs no cleanup act, the next opener
being the cleanup. The first door's path performs no retirement and gains no
branch: the delete hangs on the preload opener alone. What is
new is the door's standing and its judgment, and both are conditioned facts:
the member binds the preload name only where the party that stands it names
one, and that party names it only under a diagnostic binding, holding the
resolved kind from the inventory per `weaver-admin-Spec` section 4. **That
party is `weaver-admin` and the name rides the vector**, per that Spec's
section 6 as amended 2026-08-25, no exchange this member holds carrying a
path. **Section 2's election is narrowed rather than closed**: the descriptor
choreography it leaves to the code act is still that act's, and what is
settled here is only that a name arrives on the vector and not on a
descriptor. The credential judgment inverts the first
door's: the accept on the preload name refuses a peer bearing the agent's
uid before any byte is read, and admits the operator principal.

**The seal is a per-standing fact, held apart from the transport, and the
replay ask reads it alone.** The member holds, for its own standing's life,
whether a preload has sealed, per that contract's section 2, and the fact
is not the preload channel's openness: it is false before any dial, false
mid-stream, false after a sealless close, and true from the seal frame on.
Where the member stands with the preload door, a `replay` ask parks until
the fact is true, surviving the preload channel's close, answered at the
seal against the sealed holdings in one frame stream like any answer. The
parking is the serve
loop's and blocks nothing else: distillates land and the other asks answer
while a replay ask waits, one parked slot per channel sufficing because a
newer replay ask replaces the parked one, per the contract's retry
mechanism, the replaced ask cleared unanswered and the seal answering
whatever the slot holds when it lands. Where the member stands without the
door, the ask
answers
immediately, the query being the recall's generalized past the four
message kinds: every event row of the declared session with its field
pairs, ordered by the `id` column, served as the distillate's own shape.

```graph
node: state-replay-answers-at-the-seal
kind: assertion
tag: perturbation

edge: asserts
from: weaver-state
to: state-replay-answers-at-the-seal
```

**`state-preload-door-stands-only-diagnostic`, below, is one half of a
two-sided claim.** This crate's half is that the member binds no name it is not
given. The other half is
`weaver-admin`'s, `admin-preload-name-follows-the-kind` at
`weaver-admin-Spec` section 6, which holds the vector in **both** directions: a
serving inventory carries no name and a diagnostic one carries one. **The two
records do not divide the fact evenly.** This crate's covers what the member
does with what it is given, and the vector is entirely the other side's, because
a member given a name binds it and a member given none binds none, which is this
record holding rather than failing whichever way the name was wrong. The claim is
recorded twice because the two crates' behaviours are two facts, and the seam
between them is the other record's alone.

**The identifier below still names the pair's claim and this half is narrower
than its name.** `state-preload-door-stands-only-diagnostic` reads as the whole
two-sided fact, and what this record now asserts is that the member binds no
name it is not given, the kind being the other half's to hold. A rename reaches
every document that cites it and the conformance header that will cite it from
code, so it is its own act and is named here as owed rather than taken in an act
about where an assertion sits.

```graph
node: state-preload-door-stands-only-diagnostic
kind: assertion
tag: perturbation

edge: asserts
from: weaver-state
to: state-preload-door-stands-only-diagnostic

node: state-preload-door-refuses-the-agent
kind: assertion
tag: perturbation

edge: asserts
from: weaver-state
to: state-preload-door-refuses-the-agent
```

**Transformation is chartered and the shape aggregate is its first
member.** The grouped count above is custody's derivation under the
charter's license: an organized envelope fact carrying no judgment about
what any count means to a turn. Further derivations land as further asks
name what they consume, for the reserved-slot reason: a derivation nothing
reads is a data-shaped empty joint.

## 5. What is enforced, and by which instrument

The seam's conformance is the contract's section 8, each half landing with
its code act: the election round trip, real events to attributable rows,
and the dead-peer clause watched by killing the member mid-run for the
ingest, and the shape ask answered with exactly what the record shows plus
the ask against a dead member costing the answer and never the turn for
the serve. The territory's mode is checkable by the same walk the trace's
custody was checked by, the agent's uid asked to read the file and refused.

## 6. Open elections

The serve surface's election closed 2026-08-19: its shape and vocabulary
landed in the contract, its query-side representation at section 4, and the
store shape's trigger fired and was answered at section 3, all elected
against the context-injection loop's real ask per the charter's cell.

- **The transformation vocabulary, beyond its first member.** The shape
  aggregate landed with the serve act, and which further derivations
  custody performs stays elected ask by ask, because a derivation is named
  by what reads it.
- **The retirement mechanics.** The session's close retires the holdings,
  and the act that gives sessions a close in practice elects how the file
  is removed, sessions today outliving every run this workshop has
  produced. **Sharpened 2026-08-20 rather than closed**: the serve
  restriction of section 4 makes an earlier session's holdings unreachable,
  so the charter's boundary now holds in the answers, and what remains open
  is the disk - whether a session's close removes its rows, and what an
  operator may recover after it. The operator ruled the two apart in that
  act, so a reader meeting this cell is meeting a narrowed question rather
  than the original one.
- **The member's account name and the territory's exact key.** Deployment
  facts, elected where the spawn path lands, the way every path in the
  admin configuration is.
- **The seam's encoding.** JSON as loop zero carries it, provisionally, per
  the same election `weaver-types-Spec` section 4.3 records for the decode
  seam: reconsidered when real traffic is measurable, and a build that has
  produced that traffic and not reconsidered has an open election reading
  as settled.
