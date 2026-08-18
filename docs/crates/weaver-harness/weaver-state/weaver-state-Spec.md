# weaver-state - Spec

**Status:** MERGED. In `main` and the source of truth.

**Date filed:** 2026-08-18
**Document ID:** `weaver-state-Spec`
**Parent:** `weaver-state-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

How the custodian is represented: its process, its store, its territory, and
the shapes the ingest half takes. Written from the merged corpus alone. The
charter carries every why, and where reasoning appears here it restates a
charter clause and cites it. The serve half's representation is deliberately
absent throughout, per the charter's cell: its first asker shapes it.

## 1. The crate

A binary crate, one process per session member, spawned at load and retired at
unload while its holdings stand, per `weaver-state-PRD` section 3. It links
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

**Durability yields to speed, and the charter is the license.** The
derivative is rebuildable from the record and the session never depends on
it, per the loss clause, so the store runs with synchronization relaxed and
the journal in memory, the crash cost being a rebuild or an empty stand
rather than a lost account. The exact pragmas are the code act's, under this
election.

## 4. The ingest

The seam's traffic is the contract's `election` opener and its `distillate`
stream, and this crate's half is mechanical: parse, insert, index nothing
per event, answer nothing. **A distillate lands whole or not at all**: the
parse completes before any write, and the event row and its field rows go
in as one transaction that rolls back entire on any failure, because a
distillate held in part would be an attributable envelope over missing
pairs, a corruption custody cannot detect later. A distillate that does not
parse is dropped whole, per the contract's malformed-row clause, and the
defect waits for the serve direction to give its surfacing a voice. Inserts ride the sequence order the harness owes, and
a gap in sequence is not this crate's to notice: the record is the account
of what happened, and custody keeps what arrives.

**Transformation is chartered and none ships yet.** The charter licenses
derived shapes as custody's work, and the first derivations land when the
serve act names what the loop consumes, for the reserved-slot reason: a
derivation nothing reads is a data-shaped empty joint.

## 5. What is enforced, and by which instrument

The ingest's conformance is the contract's section 8, landing with the code
act: the election round trip, real events to attributable rows, and the
dead-peer clause watched by killing the member mid-run. The territory's
mode is checkable by the same walk the trace's custody was checked by, the
agent's uid asked to read the file and refused.

## 6. Open elections

- **The serve surface, whole.** Shape, vocabulary, and the store's
  query-side representation, elected in the context-injection loop's act
  against real asks, per the charter's cell.
- **The transformation vocabulary.** Which derivations custody performs,
  elected with the serve surface, because a derivation is named by what
  reads it.
- **The retirement mechanics.** The session's close retires the holdings,
  and the act that gives sessions a close in practice elects how the file
  is removed, sessions today outliving every run this workshop has
  produced.
- **The member's account name and the territory's exact key.** Deployment
  facts, elected where the spawn path lands, the way every path in the
  admin configuration is.
- **The seam's encoding.** JSON as loop zero carries it, provisionally, per
  the same election `weaver-types-Spec` section 4.3 records for the decode
  seam: reconsidered when real traffic is measurable, and a build that has
  produced that traffic and not reconsidered has an open election reading
  as settled.
