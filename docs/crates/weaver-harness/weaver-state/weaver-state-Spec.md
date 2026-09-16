# weaver-state - Spec

**Status:** MERGED. In `main` and the source of truth.

**Date filed:** 2026-08-18
**Document ID:** `weaver-state-Spec`
**Parent:** `weaver-state-PRD`
**Editorial:** Per the Working Rules.
**Landing PR:** #626

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

**The manifest declares two internal crates and they stand differently.**
`weaver-trace` carries the canonical event vocabulary the ingest parses, and
the shipped ingest parses that vocabulary without naming the crate: the only
units reaching it are this crate's own tests, where the opener and the
distillate are built through `weaver_trace::opener` and `weaver_trace::distill`
rather than by hand. `weaver-types` is declared and **no unit here consumes it
at all**, this crate's tests included. Charter section 5 carries a cell for
each and this clause restates them rather than answering either: what the
`weaver-trace` dependency is, the graph carrying no edge for it, and which
member of `weaver-types` this crate consumes. **The floor link itself is not
open.** It is declared at charter section 1 and kept on the operator's ruling
of 2026-09-14, a link held for work not yet done being an election rather than
a leftover, and what the cell asks is what it will be drawn for.

**Dependencies, external.** One per engine, each behind a feature named for
its engine so a build compiles the integrations it deploys and no other, per
the ruling of 2026-09-04 that the store is a port: `rusqlite` with its bundled
engine for the embedded store, so that store's version is the build's fact
rather than the host's, and `postgres`, the synchronous client, for the service
store, so no async runtime enters with it. Both pinned by the lock file like
every dependency. `serde_json` for the canonical event JSON the ingest reads.
`nix` for the preload door's credential check and for the descriptor handling
both doors require. Nothing else: no async runtime, no logging
crate, no HTTP, per the corpus's standing refusals.

## 2. The process and its territory

The member runs under its own account, owning one subdirectory in the
operator-side territory where the session record lives, per the charter's
custody ruling. Under the embedded engine the store opens by path in that
subdirectory and keeps sibling files, so custody is by ownership, mode-locked
against the agent's uid, and nothing else writes there. Under the service
engine the subdirectory holds the preload door's name and nothing of the
store, the store being reached by a connection the member alone holds: it
dials the store's unix socket under its own account, the store's peer
authentication maps that account to the role the binding declares, and the
agent's uid maps to no role, which is the second gate of the charter's section
4 standing where the first did not already refuse. The engine and, for the
service engine, the socket, the database, and the role arrive on the member's
vector from the binding, per `weaver-admin-Spec` section 6.

The first door's end arrives with the process, per the operator's ruling of 2026-08-26:
admin creates the pair at the spawn and this member inherits its end, so the peer is
authenticated by possession and no credential is judged on this door, the one party that
can hold the other end being the one the enter handed it to. The end's number is the
code act's to elect, a fixed convention between this crate and admin rather than a value
the vector carries, so the vector's positionals are the territory and the preload path
alone, the engine's flags standing ahead of them, per `weaver-admin-Spec` section 6.
**The number is probed before it is adopted**: the member reads the number's socket type
and refuses, with a named fault, a number holding no stream socket, because a hand-run
process holds whatever its shell left there and an adoption would read it as seam
traffic and close it on exit. The probe borrows and owns nothing, so the refusal closes
nothing that is not this process's own. The choreography election below is narrowed once
already by section 4 and now again by the ruling: what remains that act's is the number
and the probe's mechanics. The preload door's name arrives on the vector under a
diagnostic binding or a serving load that elects a restore, per issue #432, this member
binding whatever name it is given and none it is not, and binds under this member's own
territory, the credential judgment of section 4 unchanged on it.

## 3. The store

**The store is a port and the engine is elected, per the operator's ruling of
2026-09-04.** `src/store.rs` declares `Store`, the port: open under the
binding's election, retire a session's prior holdings, land a distillate
whole, build the elected indexes, and answer the three asks. Every engine
implements it whole and the ingest and serve of section 4 speak to the port
and never to an engine, so the seam's traffic is the same whatever engine
answers. Two engines stand, one module each behind its feature: `Sqlite`, the
embedded engine of the 2026-08-18 ruling, its file `state.sql` in the member's
territory, opened or created at load, reopened by later loads of the same
session, and retired with the session, and `Postgres`, the service engine, one
database per agent named by the binding, reached over the store's unix socket
under the member's account. The port is the one place a query language is
spelled, each engine spelling its own dialect of the same two-table shape
below, and a third engine is an implementation of the port in its own act.

```graph
node: state-store-is-a-port
kind: assertion
tag: review

edge: asserts
from: weaver-state
to: state-store-is-a-port
```

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

**An elected index is named from its key path, and a name that cannot be made
is a refusal.** Two elected paths never share a name, so a later load's
differing election never falls silently under `CREATE INDEX IF NOT EXISTS`
beneath an earlier load's, which is the whole reason the name is derived from
the path rather than from the key's position in the election. Where an engine's
identifier limit cannot hold a path's name the load refuses the election with a
named fault, because building the subset that fits is the same silent loss read
from the other end. The encoding is the code act's under this election, the way
the pragmas below are. **The service engine does not answer this election
today and the code is owed**, per issue #618: its names truncate at the store's
sixty-three byte identifier limit, so two elected paths sharing a long enough
prefix collide to one name and the second index is a no-op nothing reports. The
embedded engine is unaffected, sqlite setting no such limit.

**Durability yields to speed, and the charter is the license.** The
derivative is rebuildable from the record and the session never depends on
it, per the loss clause, so the embedded engine runs with synchronization
relaxed and the journal in memory, the crash cost being a rebuild or an empty
stand rather than a lost account. The exact pragmas are the code act's, under
this election. The service engine's durability is the store's own and this
crate asks nothing of it, its rows being the same derivative under the same
license.

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
twenty-eight earlier runs it never had. **A store holding more than one
session is the normal case rather than the broken one**, sessions outliving
runs and the store outliving sessions under either engine, so the restriction
is what makes `weaver-state-PRD` section 4's within-a-session boundary a
property of the answers instead of an assumption about the store. What
becomes of an earlier session's rows on disk is section 6's open question and
deliberately not
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

The `shape` ask runs one grouped count over the event table, and the landing order the
contract's first-seen clause asks for is the `id` column's, custody's own order key: the
run groups are ordered by the least `id` each holds, each carrying its kinds and their
counts as the envelope spelled them, rendered as the contract's answer frame and written
back on the channel as one answer frame, the frame's byte shape riding the encoding
election of section 6. The `recall` ask reads the event rows of the four message kinds
with their field pairs, ordered by the `id` column like every landing-order answer, and
where `last-turns` bounds it the bound resolves as the distinct session, run, and turn
triples of the most recent turns by id, the rows outside them left unread, a turn label
recurring across runs naming two different turns. The answer serves each event as the
distillate's own shape, envelope and pairs, because custody serves what it kept in the
form it kept it. The `grants` ask reads no event row: it reads the engine's own
boundary, the catalog's lines for the connected role under the service engine and the
file's owner, group, and mode under the embedded one, and answers them in the engine's
order as `{"answer":{"grants":{"surface":[...]}}}`, each line a string, per the
contract's fourth ask of 2026-09-04. The `identity` ask reads the event rows of kind
`message.system` whose turn is absent and whose run is the run of the newest such row,
ordered by the `id` column, with their field pairs, and answers them as
`{"answer":{"identity":{"messages":[...]}}}`, each the distillate's own shape, an empty
list where the session holds none, per the contract's fifth ask of 2026-09-04. A
malformed ask is dropped whole the way a malformed distillate is, and the resulting
silence is the harness's bound to convert into a missing answer.

**Three protocol bounds are this crate's elections, each named with what its
breach means, per the audit of 2026-08-26.** The answer ceiling is one
mebibyte: an answer past it is not sent at all, silence the asking side's
bound converts into the missing answer per `weaver-harness-state-contract`
section 3, because custody never invents an answer shape for a fault - and
the ceiling is reachable on an ordinary session through `replay`, which
serves every held event, so a loop reading a missing answer there is told
the truth about the seam and nothing about the holdings. The inbound frame
cap is eight mebibytes: one frame larger is not the seam's traffic and the
door that carried it ends as closure, the peer holding a credential and not
a license to exhaust this process. The answer write deadline is two
seconds: a peer that takes nothing for that long has stopped reading, and a
custodian wedged on its behalf would cost the session its custody, so the
seam retires with the holdings standing. The values are this act's and a
measurement may move them, the meanings being the elections.

**The preload door lands its distillates through the same path the first door does, and
that is the mechanism of the contract's indistinguishability claim.** A distillate
arriving on the preload channel parses, transacts, and lands exactly as one arriving
from the tee, one code path and one store, so nothing marks how a holding arrived and
the serve restriction binds to the preload opener's session the way it binds to the
harness opener's. **The one act the preload path adds is the opener's retirement**:
receiving the preload election deletes the declared session's event and field rows in
the same transaction that records the opener, before any distillate lands, per the
contract's section 2. The path is thereby idempotent at the preload grain - re-running
it replaces the session's holdings rather than appending to them - and a dead driver's
prefix needs no cleanup act, the next opener being the cleanup. The first door's path
performs no retirement and gains no branch: the delete hangs on the preload opener
alone. What is new is the door's standing and its judgment, and both are conditioned
facts: the member binds the preload name only where the party that stands it names one,
and that party names it under a diagnostic binding or a serving load that elects a
restore, per issue #432, holding the resolved kind from the inventory per
`weaver-admin-Spec` section 4. **That party is `weaver-admin` and the name rides the
vector**, per that Spec's section 6 as amended 2026-08-25, no exchange this member holds
carrying a path. **Section 2's election is narrowed rather than closed**: the descriptor
choreography it leaves to the code act is still that act's, and what is settled here is
only that a name arrives on the vector and not on a descriptor. The credential judgment
is this member's one, the first door authenticating by possession per the operator's
ruling of 2026-08-26: the accept on the preload name admits the operator principal and
refuses every other peer before any byte is read, the agent's among them and no longer
knowable by number, the vector having dropped the agent's uid with the first door's
judgment.

**The seal is a per-standing fact, held apart from the transport, and the
replay ask reads it alone.** The member holds, for its own standing's life,
whether a preload has sealed, per that contract's section 2, and the fact
is not the preload channel's openness: it is false before any dial, false
mid-stream, false after a sealless close, and true from the seal frame on.
Where the member stands with the preload door, a `replay` ask parks until
the fact is true, surviving the preload channel's close, answered at the
seal against the sealed holdings in one frame stream like any answer. **The
`identity` and `recall` asks park on the same fact**, as of 2026-09-06 per
the contract's section 2 on issue #432: a session
standing from a preloaded record asks for its prefix and its conversation at
the enter, before the driver has sealed, so the two park where the door
stands and no seal has landed and answer at the seal in arrival order,
against the sealed holdings, the replay ask's replacement rule reaching the
replay ask alone. Where no door stands they answer immediately as before.
The member cannot tell a restoring load from a diagnostic one and need not:
the door's standing is the fact, and a diagnostic load's `identity` ask
answered at the seal is the record's own prefix, which is what
`weaver-analysis-Spec` section 3 says the preloaded store answers. **The
door itself survives the channel too**: on any close of the preload
channel the member unlinks and rebinds the name and the per-channel opener
state resets, per the contract's retry mechanism, while the seal fact
stands apart and is never reset, so a retry's opener retires the dead
prefix and a parked ask still answers only at a seal. A re-stand whose bind
fails logs the fault and the member serves on doorless, the derivative
degrading rather than the standing ending, where the same failure at the
initial stand is fatal because nothing has been served yet that a death
would interrupt. The
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

**The preload name's mode is this member's election.**
`UnixListener::bind` sets none, so the name would land at `0777 & ~umask` and
the door's permissions would be whatever umask this process inherited. The
bind happens under a umask denying every bit outside the owner, so the name
lands at `0700`.

**`0700` here where the gate's door is `0770`**, per `weaver-gate-Spec`
section 3. The accept below admits `uid() == 0` and no other, so no group
reaches this door and a mode granting one would describe access it does not
offer. The credential check is the lock that decides and this is the lock that
keeps a stranger from arriving at it, which is the two-locks reasoning
`weaver-harness-PRD` section 5 applies to the trace descriptor.

**Elected in the creating call and not on the path afterwards**, for the
reason the gate states: a mode set after the bind leaves the name live at the
inherited mode in between, races a path an unprivileged process may be able to
swap, and on failure leaves a file behind. The umask is process-global, so the
guard serializes on it.

```graph
node: state-preload-door-states-its-mode
kind: assertion
tag: perturbation

edge: asserts
from: weaver-state
to: state-preload-door-states-its-mode
```

**`state-preload-door-stands-only-diagnostic`, below, is one half of a two-sided
claim.** This crate's half is that the member binds no name it is not given. The other
half is `weaver-admin`'s, `admin-preload-name-follows-the-kind` at `weaver-admin-Spec`
section 6, which holds the vector in **both** directions: a serving inventory carries no
name, and a diagnostic one, or a serving one whose declaration elects a restore, carries
one, per issue #432. **The two records do not divide the fact evenly.** This crate's
covers what the member does with what it is given, and the vector is entirely the other
side's, because a member given a name binds it and a member given none binds none, which
is this record holding rather than failing whichever way the name was wrong. The claim
is recorded twice because the two crates' behaviours are two facts, and the seam between
them is the other record's alone.

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

**This crate holds no `tests/` target**, every test it has standing in an
in-file `#[cfg(test)]` suite beside the unit it watches, so a citation at a
test here is written at an item inside such a suite and never at a path under
`tests/`, which is the scope Document Format section 5's review rule reads.

**Requiring a perturbation-verified test.** Five claims, each watched where
the behaviour sits.

- The serve restricts to the opener's session, watched by dropping any of the
  three `WHERE session` predicates the reads carry, which returns an earlier
  session's runs to a shape answer and an earlier session's rows to a recall.
- The replay answers at the seal, watched by parking the replay alone the way
  the law did before the enter asks joined it, and by making the park ignore
  the seal, either of which answers over a prefix.
- The preload name states its mode, watched by dropping the owner-only umask
  from the bind, which leaves the name at whatever mode this process inherited.
- The member binds no name it is not given, watched by giving the name a
  default path where the vector carries none, which stands a door on a serving
  load.
- The preload door refuses every peer but the operator, watched by dropping
  the root arm from the accept, which admits the agent's own uid.

**The first of the five is watched for one engine.** The predicates are the
embedded engine's and nothing in this tree builds the service engine under
test, so the session predicate could leave that engine's reads and every device
would answer green. The claim is cited at the port and at the engine that holds
the instrument, and the service engine cites it when it holds one of its own.
The same reading covers the whole of that engine: it is reachable only from the
binary's own election and no unit constructs it, so its arm of every claim
below rests on the reading and not on a run.

**Enforced by review, and each clause names what would buy it.** Review here
means the instrument was not bought and never that none exists, per Document
Format section 5.

- **Custody without policy** is read off the crate's surface for any door a
  judgment could enter by. The claim is an absence spread across a surface
  rather than a shape, so no single compile-fail pin names it and a test can
  only watch the doors that exist. What would buy it is a check over the
  crate's exported items refusing any that ranks, judges or initiates, which
  is a shape this corpus has no instance of, and naming it is what keeps the
  claim from reading as unbuyable.
- **The store is a port** and the nearest thing to an instrument is the
  signature: the ingest and the serve take the port behind a reference and can
  name no engine, so a call reaching past it would not compile as this code
  stands. Nothing holds that signature in place, so a later act widening one of
  them to a concrete engine compiles and this claim goes quiet. What would buy
  it is a compile-fail pin over an ingest path that names an engine.
- **A distillate lands whole** and the embedded suite watches a good one
  landing and surviving a reopen, which is the persistence half. The half that
  carries the claim - a landing that fails between the event row and its pairs
  leaving neither behind - has no instrument. What would buy it is an insert
  forced to fail inside the transaction with the holdings counted after.
- **The indexes are built at load** and the embedded suite watches the naming:
  two differing elections build two indexes rather than the second falling
  under the first's name. What is not watched is the build happening at the
  load rather than at the query that wants it, nor the envelope's standing
  indexes, and the service engine contradicts the claim outright until the
  election of section 3 is answered, per issue #618.

**The walks the seam's conformance asks for are not in this tree.** The
contract's section 8 names them and says both directions land with the acts
that open the seam and shape the surface, which have landed: the election round
trip and real events to attributable rows against the living producer, the
dead-peer clause watched by killing the member mid-run and by asking with the
member gone, the replay ask observed waiting in all three unsealed states and
its retry sequence, and the answered-against clause read in time with asks
interleaved among distillates. The suites here reach most of those properties
through the port or through the unit that holds them rather than across the
seam, which is the cheaper instrument and not the one the contract names, and
**the dead-peer clause is reached by nothing here in either direction**. The
territory's mode is owed the same kind of walk, the agent's uid asked to read
the file and refused, and the service engine's second gate the same at the
store, the agent's uid asked to connect as any role and refused by the store's
own authentication. The `grants` ask
reports the file's owner and mode and asserts nothing about either, so it is a
surface for that walk rather than the walk.

**Where the records sit.** The assertion records are at the clauses that argue
the claims, across sections 1 through 4 rather than gathered here, per Document
Format section 6.

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
  and the act that gives sessions a close in practice elects how the
  embedded engine's file is removed and how the service engine's rows are
  dropped, sessions today outliving every run this workshop has
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
