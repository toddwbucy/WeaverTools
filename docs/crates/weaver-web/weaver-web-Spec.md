# weaver-web - Spec

**Status:** MERGED. In `main` and the source of truth. **Merged is not
ratified.** This text inherits no ratification from the Spec it replaces,
and takes whatever ratification the rewritten `weaver-web-PRD` is granted,
which is the operator's to say.

**A rewrite of this crate's Spec, not a new one.** Authored 2026-09-04 from
the thinkpad seat beside the rewritten `weaver-web-PRD` of the same date.
The prior text is replaced whole rather than amended, and git is its
archive.

**Revised:** 2026-09-06, one artifact has two identities and the join holds.
Section 2.3's row carries the record's identity beside the per-file map it keys on,
computed at import by `weaver-spu-Spec` section 3's rule over the files it registers
and unique across rows, and the record's identity determines the key and not the
reverse, so the join section 2.2 needs is a lookup on it. Section 2.2's artifact
identity is that value, filled from the summary the analysis seam carries per
`weaver-analysis-web-contract` section 2.2. Section 9 gains the row and section
10's election closes. Per issue #465.

**Revised:** 2026-09-05, fourteenth of this date, section 2.3 stops claiming
a join section 10 says cannot be made. The table asserted that a run's
artifact identity resolves to it while the open election two sections down
states that the catalog's identity and the record's are two rules with
nothing to join on. The claim becomes owed and the shape that does hold, a
lookup resolving to the complete file set or to nothing, is stated
separately from it. The three-crate settlement is issue #465 and not this
document's alone. Per the review of PR #464.

**Revised:** 2026-09-05, thirteenth of this date, the version the edit rule
reads is a member the rows carry. Section 3.2 required an edit to carry the
version it read and no table in section 2 held one, so the rule named a
member that did not exist. Sections 2.3, 2.4 and 2.5 carry it, an accepted
write advances it by one, a stale one refuses naming both, and registration
freezes the row at the version it holds without advancing it, an edit
against a registered row refusing on the state instead. The declaration's
version is distinguished from its corpus commit: one answers whether the
field shape is current, the other whether the row has moved since it was
read. Per the review of PR #464.

**Revised:** 2026-09-05, twelfth of this date, against the review of
PR #464. Section 3.2 stated two rules for one write, last-write-wins beside
a version check, and keeps the second. Section 3 counted two writers where
the document names three, the read path writing section 2.6's row, which is
why 2.6 belongs to neither half. Section 2.4 said admin holds a store of
declarations and 3.2 that the operator installs one through admin: admin
stores none and takes none in, its verbs reading a configuration file the
operator placed. Section 9's absence row buys its absence with compile-fail
rather than a pin, per the corpus's own rule that a pin buys a shape that
exists. Section 10 names the identity G5 the review found, two rules for one
artifact identity with the join unable to join.

**Revised:** 2026-09-05, eleventh of this date, the store's authored half is
written down. Section 2 had five tables and every one of them recorded:
three surfaces have listed declarations, staged experiments and artifacts
since the rewrite, and only the artifact had a row, which nothing could
write into because section 3 was the ingest alone and section 6 called it
the only writer. Sections 2.4 and 2.5 give the declaration and the staged
experiment their rows, section 3 splits into the ingest and the authoring
path, and section 6's rule narrows to what it meant: no surface writes a
position or a run. Three conformance rows follow. The recorded query moves
to 2.6 and the indexes to 2.7, with their citations. Per issue #454.

**Revised:** 2026-09-05, tenth of this date, the watch on the stored
position is one that can fail. Section 9's row had read that a second reader
disagrees where the position is derived at the read, which two readers
deriving identically would satisfy while the property went unenforced. It
now alters the summary's counts after ingest and rereads, which only a
stored value survives. Section 3 says the points of an unaddressable
generation do not land while its summary entry does. Per the review of
PR #459.

**Revised:** 2026-09-05, ninth of this date, a position that was never
established is not addressed. Section 3 states what the write path does
where a generation's closing count is absent, no `model.output` having
reported one: the address section 2.1 requires cannot be formed, so the rows
do not land and the run reads partial rather than short. This is
absent-not-empty at the write path, the rule that forbids drawing a missing
surprisal as zero forbidding an invented key for the same reason. Per the
review of PR #463, whose emitter carries the count as absent rather than
derived.

**Revised:** 2026-09-05, eighth of this date, the position is derived at
ingest. Section 3's write path carries the rule, `(R - O - 1) + j` over the
generation's closing resident count, its output token count and the ordinal,
measured on 2026-09-05 against three records and nine generations rather
than reasoned, with the alternative that fails on the first generation
stated so no later act rediscovers it. Section 7.3 no longer states the
contract's ask as landed: the two counts it converts from are owed at
issue #461, and until they cross this seam carries the ordinal alone and
this crate cannot key its store from it. Section 9 gains the two rows the
contract's conformance clause named. Per the review of PR #459.

**Revised:** 2026-09-05, seventh of this date. Section 7.3 cites
`weaver-analysis-web-contract`, landed this date per issue #418, and names
the two clauses of it that reach back here: the position rides beside the
ordinal, converted once at the drain, and an uncertified diagnostic record
is not drawn. Section 7.2's account of `validate` is bounded against the
custody ruling of this date, issue #456: the verb reaches what admin holds
custody of, and a clean answer is acceptance for filing rather than approval
to load.

**Revised:** 2026-09-05, sixth of this date, the second review pass. Four
sentences lost a word to the semicolon sweep and are restored. **A sharded
GGUF carries no index**, so section 2.3 enumerates it by the
`-NNNNN-of-NNNNN` pattern `weaver-spu-Spec` section 3's pin collects on, the
earlier wording having named a split manifest that does not exist for the
format this catalog most holds. The identity cites the manifest member the
owning Spec declares rather than a Rust item it does not. Section 6 no
longer says section 7 names a queue seam, which it does not. The semicolon
count in the second entry was low by two. Entries now run newest first.

**Revised:** 2026-09-05, fifth of this date. Section 2.3's artifact identity
becomes the shape `weaver-analysis-Spec` section 3 gives
`model_safetensors_sha256` rather than a composite this document defined,
equality being set equality with no order imposed. Section 2.2 states that
the parent run and branch position are lineage outside tuple equality, so
two rows differing only in them remain comparable, which is what makes a
branch reproducible against its parent. Section 2.4 carries the lens
artifact a reading was taken through and the weights it was fitted to. Per
the review of PR #453.

**Revised:** 2026-09-05, fourth of this date, the run's row carries the
field election's depth, which sets the length of every position's
alternative list and which the charter's section 4 compares. Per the review
of PR #453.

**Revised:** 2026-09-05, third of this date, the Models surface and the
review of PR #455. Section 2.3 gains the artifact catalog with one derived
identity rule, the complete set the model's own index names. The join
resolves to that set or to nothing, so a lens relation and a reference cell
relation each name one unambiguous artifact. Presence carries its reporter
and its date and gates nothing. The lens row cites `weaver-analysis-PRD`
section 3 rather than restating it, correcting an earlier draft of this act.
Two assertion rows added. Semicolons swept under G1.

**Revised:** 2026-09-05, second of this date, against two reviews on
PR #453. Section 2.3 gains the recorded query, which section 4 required and
section 2 gave nowhere to put, the indexes moving down and their citations
following. The artifact catalog of the act below later took 2.3 and the
recorded query became 2.4. The engine's identity enters the run's row, and
**the deposit regime behind it is cited to `weaver-analysis-PRD` section 3**
rather than asserted, an uncited regime in a Spec being testimony. The
scorer leaves the run's tuple for the verdict. Semicolons swept from the
whole document under G1, eight of them older than this act.

**Revised:** 2026-09-05, first of this date, against the acts of that date.
The observation exchange landed, issue #435 at PR #440, and the per-position
read landed, issue #436 at PR #444. The component catalogue, issue #437, was
answered with a finding rather than a catalogue. Two facts the record spells
corrected this document where it had guessed: the address carries the turn,
and a position is a resident length rather than an ordinal.

**Date filed:** 2026-09-04
**Document ID:** `weaver-web-Spec`
**Editorial:** Per the Working Rules. ASCII, absolute dates.

## 0. What this document is

The charter says what this crate is for. This document says what it is made
of, in the order a reader needs it: the store first, because every surface
reads from it, then the paths into and out of that store, then the seams,
then what holds each claim.

**Where this document elects something the charter left open, it says so.**
Where it leaves something open, section 10 names it rather than letting
silence settle it.

## 1. The crate

```text
src/
  store/        the registry: schema, migrations, the three reads
  ingest/       section 3.1, a consumer of the analysis stream
  authoring/    section 3.2, the writes a surface makes to its own table
  queue/        staged experiments and their states
  surfaces/     one module per surface of charter section 3
  seams/        gate client, admin verbs, analysis stream reader
  link/         the connector and server halves
```

**This crate links no crate of the agent workspace** and reaches the agent
exactly as an outside consumer does: a socket dialed by path, a binary run
by the operator's verb, and a record read where the operator keeps it. That
property is load-bearing and is not spent by the seam of section 7.3, which
is a stream this crate reads rather than a crate it links.

## 2. The store

The grain is the grain the interface clicks at.

**The store has two halves and they are written by different paths.** What
the instrument recorded is sections 2.1 and 2.2, the position and the run,
landed by the ingest of section 3.1 and never by a surface. What the
engineer authored is sections 2.3 through 2.5, the artifact, the
declaration and the staged experiment, landed by the authoring path of
section 3.2 and never by the ingest. Section 2.6's recorded query is the
read's own trace and belongs to neither.

**The halves differ in what a rewrite means.** A recorded row is a fact
about a run that happened, so a second write of it is a replay and must be
idempotent. An authored row is a thing a person is still making, so a
second write of it is an edit and must be ordered. Section 3 states each.

### 2.1 The position

**The address is the run, the turn, and the position**, and the composite of
the three is the primary key.

**This corrects the two-part key this document first carried.** The record's
own spelling is what settles it: turn keys repeat across a serving record's
runs, so an address without the run answers one line per run, which is why
`weaver-analysis field` takes `<turn>:<position>` and an optional `--run`.

**And `position` is the resident length at the draw, not an ordinal within
the turn.** The first generated token of a sixty-token prompt is position
sixty. Verified 2026-09-05 against a field-bearing record on the pool, whose
first `model.field` event carries `position: 60`. A surface that treats the
two as one word will address the wrong token, and section 6 states the
conversion.

Each row carries:

- the emitted token, by identifier and by surface text
- the surprisal and the entropy at that position
- the ranked alternatives with their probability mass
- the raw residual where the capture holds one

**`realized` is a rank and not a token.** It names which rank the draw
landed on, so a row carries `realized` as the record spells it and the drawn
token resolved beside it, and neither is presented as the other.

**The alternative count is the declaration's field election, not this
crate's parameter.** `model.field` carries the ranked candidates under
`field-election: { depth }`, and the records on hand carry forty and fifty.
The depth is the operator's ruling, open at the charter's section 9, and
this crate stores what the election kept rather than choosing a number.

**Raw residual rides alongside rather than a projected readout**, so a lens
refitted later can read a run captured earlier. This is the standing
raw-residual ruling and this document does not disturb it.

### 2.2 The run

Everything identifying the conditions lives in the run's own row:

- artifact identity **at a grain fine enough to catch a quantization
  difference**, which is the record's own, the weights hash the SPU computed
  at admit per `weaver-spu-Spec` section 3, taken from the summary the
  analysis seam carries per `weaver-analysis-web-contract` section 2.2 and
  never from the declaration, a declaration naming a path and being editable
  after the run
- seed, and the full sampler configuration
- device, and precision
- **the engine's identity at build grain**: the libraries the decode ran
  through, each by name and digest, with the build that produced them
- the batching election
- **the field election's depth**, because it sets the length of every
  position's alternative list and the charter's section 4 compares them
- the task, by source and identity
- the declared boundary set
- the parent run reference and branch position, where the run is a branch,
  **which are lineage and stand outside tuple equality**
- whether a token was forced, and which

**The engine is in the compound, so the row holds it.** The tuple is a model
on a device under a kernel at a precision, and a divergence between two rows
differing in both silicon and library revision names neither cause unless
both are recorded. **The regime that deposits these identifiers is
`weaver-analysis-PRD` section 3's**, which carries code identity beside the
device model, the commit by hash, the toolchain and driver by pinned
version, and the binaries and engine libraries by the driver's sha256. This
row carries the same fact so a reading drawn from the store needs no
deposit beside it to be read.

**The row holds more than the tuple, and the difference is lineage.** The
parent run reference and the branch position say where a run came from
rather than what it ran under, per the charter's section 4, so **two rows
differing only in them hold the same tuple** and remain comparable. Nothing
else in this list is outside the compound. Were lineage inside it, a branch
could never be reproduced against its parent, which is the comparison the
staging surface exists to make.

**A reading without its tuple is a reading of an unnamed compound.** The
task is in this list for the same reason the artifact is: two runs of the
same benchmark item under different suite versions are not the same task,
and nothing else in the row would say so.

### 2.3 The artifact

The catalog the charter's section 3.6 surfaces. **Keyed by the artifact's
weights identity rather than by a path**, because a path is where a file
sits and an identity is what it is, and the same weights under two paths on
two boxes are one artifact.

**The identity is the corpus's existing one and this document defines no
second**, because a catalog that dedupes on one rule while a lens refuses
on another is two rules for one subject. **Its identity is the set of
per-file content digests keyed by file name**, which is the shape
`weaver-analysis-Spec` section 3 gives `fitted_for`'s
`model_safetensors_sha256`: one digest for a model kept in one file, and a
digest per shard keyed by shard name for a sharded one.

**What the complete set is depends on the container, and both cases are
named.** A safetensors model enumerates its shards in its own index,
`model.safetensors.index.json`, which the analysis reader follows. **A
sharded GGUF carries no index.** Its set is the siblings matching the
`-NNNNN-of-NNNNN` pattern, which is how `weaver-spu-Spec` section 3's pin
collects them, and a single-file GGUF is the one-file case unchanged. This
catalog holds GGUFs, so the rule that names only an index would leave the
format the box actually runs without one.

**Equality is set equality and no order is imposed**, the map being keyed
rather than sequenced. **A set missing a file the index names is unequal to
the complete set** and therefore joins to nothing, which is the whole of
what the incompleteness rule needs and asks for no rolled-up digest of this
crate's own devising. The catalog dedupes on that set and on nothing else,
and it is the grain section 2.2's tuple means by artifact identity, fine
enough to separate two quantizations because their files differ.

**The provenance chain is recorded and is not an identity.** A conversion
or a quantization records its source artifact with the converter and the
pin, so the chain says what an artifact was made from. It does not version
a lens: `weaver-analysis-PRD` section 3 owns that and versions by the
weights content hash, which a conversion changes. The chain is what makes
an elected reuse across a conversion legible after the fact, per the
charter's section 5, and this crate records the election rather than
deriving it.

Each row carries:

- the artifact identity, derived as above
- **the record's identity**, the weights hash `weaver-spu-Spec` section 3
  states, computed at import over the files the row registers and unique
  across rows, a renamed split GGUF excepted per section 10, and where the
  same weights are imported again from a directory differing in a file the
  index never names the new value joins this row rather than opening one.
  **The sentinel is never a record identity here**: an import that cannot
  compute the value registers nothing, per the charter's section 3.6, and
  a run carrying the empty string joins to nothing and is named as one
  whose identity the SPU could not compute, which is a different fact from
  a run no row carries
- the identity **per file**, following the model's own index, each shard
  under its own name and verified against it
- the provenance: a repository and revision, or a source artifact with the
  converter and pin that produced it
- where it is present, by box, as an observation rather than a fact
- the lens artifacts fitted to these weights, each versioned as
  `weaver-analysis-PRD` section 3 versions them, by the weights content
  hash
- the reference cells taken against it
- **the row's version**, per section 3.2

**This table is what a run's artifact identity resolves to**, section 2.2
holding that identity in the tuple so a reading names its conditions. **The
join is a lookup on the record's identity**, as of 2026-09-06 per
issue #465: the two identities are two rules at two grains, the record's over
everything the binding named and this table's key over the weights alone,
and the record's determines the key because a directory's hash fixes its
shards, so one record identity names at most one row, a renamed split GGUF
being the one case where it names two, which the lookup reports as
ambiguous rather than picking, per section 10, and the row carries
every record identity its weights have been admitted under. A run whose
record identity no row carries joins to nothing and registers nothing, a
record being a fact and not an import, until an import on that box
registers the files.

**What does hold is the catalog's own shape.** A lookup by this table's key
resolves to the complete file set or to nothing, so a lens relation and a
reference cell relation each name one unambiguous artifact and never a
shard of one, and an elected lens reuse is legible here with the fitted
weights and the read weights both resolving by the same rule.

**Presence is a dated observation by a named reporter, and it is
advisory.** Each entry carries the box, the reporter, and when it was last
confirmed. A box that has not reported is unknown rather than empty,
because this crate runs on one machine and the agents on another, and
silence from a box that is merely offline is not evidence about its disk.

**No load consults it.** The load resolves the artifact on the box it runs
on and is refused there under that box's own rules. Presence here is an
index for an operator choosing where to place a run, never the thing
deciding whether the run may proceed: a gate built on a stale observation
refuses a box that holds the artifact and admits one that lost it.

**Nothing here fetches.** An entry says an artifact was seen, not that it
can be obtained. Whether a missing one is fetchable is the provenance's
question, answered by the repository and revision the row already carries.

### 2.4 The declaration

The saved configurations the charter's section 3.6 lists under Agents,
**loadable at any time**, which is what makes them a table rather than a
draft buffer. Each row carries:

- the declaration as authored, whole, in the shape `weaver-types-Spec`
  section 2 defines
- **the corpus commit its field shape was written against**, which is this
  row's staleness rule per section 7.2: when the floor moves, the pin says
  so and `validate` refuses in a way the surface can name
- the parent declaration where this one is derived, and **the one thing
  that moved**, which is what the charter's section 3.6 draws
- the last answer `validate` gave, with when it was given
- **the row's version**, per section 3.2, which is not the corpus commit
  above: that one answers whether the field shape is current, this one
  whether the row has moved since it was read

**The last answer is a reading and not a verdict.** It is what the verb said
at a moment against a corpus commit, so a surface presents it with its date
and re-asks rather than treating it as current. **A declaration this crate
holds is not a declaration a box will load.** Admin stores none and takes
none in: its verbs read a configuration file the operator placed, and what a
box will load is that file, judged by `validate` and then by the SPU at
admission, per `weaver-admin-PRD` section 4.3. This table is where an
engineer keeps what they are composing, and the two agree only where the
operator has placed one.

### 2.5 The staged experiment

The claims the charter's section 3.5 registers and section 3.6 lists, in the
five states of section 5.1. Each row carries:

- the state, and when it last changed
- the parent run reference and the branch position
- the forced alternative token where one is forced
- the parent declaration and its diff, split by when the diff takes effect
- **the question the engineer meant to ask**, which nothing upstream knows
  and nothing else in this store holds
- the runs it produced, where it ran
- **the row's version**, per section 3.2, frozen with the rest at
  registration

**Registration freezes the row and not the table.** A registered experiment
is immutable per section 9's pin, and the rows around it go on being
edited, which is why the state is a member rather than a table each.

**An experiment that never ran keeps its row.** That is pre-registration
falling out of the interface rather than being imposed on it, per the
charter's section 3.5, and it is the one thing here that would be lost by
storing only what returned.

### 2.6 The recorded query

Section 4 admits an open query surface on the condition that the query is
recorded beside its result, and that condition needs somewhere to land.
Each recorded query is its own row under its own identifier, carrying:

- the query text as issued, verbatim
- the runs it addressed, each by identity
- the reader that served it, by name and version
- **the lens artifact it read through, where it read one, and the weights
  that lens was fitted to**
- when it was taken

**The result is not stored beside it.** A stored result would be a second
truth about positions section 2.1 already holds, and it would go stale the
moment a later ingest completed the run. The row stores what reruns the
query, and the rerun is what produces the result again - which is the whole
of what a quotable reading claims.

**An elected reuse is recorded here and is visible in the reading.** Where
the weights a lens was fitted to are not the run's own artifact, the
operator elected the reuse rather than the tool inferring it, per the
charter's section 5, and both identities stand in the row so a reader
downstream sees an election rather than a fit. The catalog resolves each of
them, which is what makes the election legible after the fact. **A reading
whose lens was fitted to other weights and does not say so is the one thing
this row exists to prevent.**

**A query that cannot name every run it addressed is not recorded and not
quotable.** Section 4's condition is that a second person can rerun it, and
a reader that cannot say what it read cannot be rerun by anyone.

### 2.7 The indexes

```text
primary       (run, turn, position)
secondary     (run, surprisal)
```

The secondary index exists so the largest spikes in a run are reachable
without pulling the run down. **Nothing is computed at read time.** A value
that must be derived is derived once at ingest and stored, because a value
computed in the interface is a value nobody else can reproduce.

## 3. The write path

**Three writers, and each owns its tables.** Section 3.1's ingest lands what
the instrument recorded. Section 3.2's authoring path lands what the
engineer authored. **The read path writes too**, one row and only one:
section 4 admits an open query on the condition that the query is recorded,
so the read that serves it writes section 2.6's row and nothing else. That
is why 2.6 belongs to neither half. No writer touches another's tables, and
**no surface writes through 3.1**, which is what section 6's rule means and
all it means.

### 3.1 The ingest

**The ingest is a consumer rather than a step in the loop.** The
analysis emission leaves over its own socket, a process on this side reads
it and lands it in the store, and **the decoder never waits on the store.**

- Writes are **bulk per turn or per window**, never per token.
- Writes are **idempotent on the run, turn and position key**, so a replayed
  or partially failed ingest cannot produce two truths about one position.
- **The run row is written first and closed last**, carrying a status a
  surface can read, so a partially ingested run is visibly partial rather
  than quietly short.

**The position is derived here and nowhere later.** The stream is addressed
by the ordinal within a generation and section 2.1's key is the position,
the resident length at the draw. Where `R` is the generation's resident
count at close, `O` its output token count, and `j` the ordinal, the
position is `(R - O - 1) + j`, the subtracted one being the turn terminator
the SPU makes resident before the answer returns.

**The rule is measured rather than reasoned**, checked on 2026-09-05 against
three records across nine generations, two precisions and two finish kinds,
each derivation compared to the position `model.field` reports directly. The
alternative that suggests itself, the previous generation's resident count
plus the turn's input delta, is exact from the second generation and wrong
on the first by the session prefix, which the first turn's delta does not
carry. **This crate takes the first rule and states the second's failure so
no later act rediscovers it.**

**A generation whose closing count the record does not carry has no
position, and its points do not land, though its summary entry does.** The
count is absent rather than derived where no `model.output` reported one, so
the address section 2.1 requires cannot be formed, and this crate stores
nothing it cannot address rather than storing rows under an invented key.
The run's row carries the status, so a run short of a generation is visibly
partial by the rule above rather than quietly short. **This is
absent-not-empty at the write path**: the same discipline that forbids
drawing a missing surprisal as zero forbids addressing a position that was
never established.

Deriving here rather than at the read is section 2.7's rule and not a
preference: a value derived once at ingest and stored is one a second reader
reproduces, and one computed in a view is one nobody can.

### 3.2 The authoring path

**A surface that authors writes the row it authors and nothing else.**
Compose writes a declaration, Stage writes a staged experiment, and Models
writes an artifact row on import. Each writes its own table of section 2 and
no other, and none of them may write a position or a run, which is what
keeps a recorded fact a recorded fact.

- **Every authored row carries a version, and it is the store's own
  counter rather than anything the author supplies.** Sections 2.3, 2.4 and
  2.5 each carry it. It has nothing to do with the declaration's corpus
  commit, which pins the floor's field shape and answers staleness against
  `weaver-types-Spec`: **one says whether the shape is current, the other
  says whether this row has moved since you read it.**
- **Writes are ordered on the row rather than idempotent, and the order is
  the version's.** An authored row is a thing a person is still making, so a
  second write is an edit. **An edit carries the version it read.** Where
  that version is the stored one the write is accepted and **the stored
  version advances by one**. Where the stored version has moved on the write
  refuses, naming both versions, and the surface says so rather than
  merging. The recorded half's idempotence answers a replay and there is no
  replay here, and a last-write-wins rule would lose the case this one is
  for, two engineers on one declaration.
- **Registration freezes the row at the version it holds and advances
  nothing.** It is a state change rather than a write of its own: the row
  moves from draft to registered and becomes immutable per section 9's pin,
  and what made it immutable is a member the row already carried. An edit
  arriving against a registered row refuses on the state and not on the
  version, which is a different refusal and says so.
- **Import registers what is present and fetches nothing.** An artifact row
  names weights already on a box, its identity computed by the rule of
  section 2.3 rather than accepted from the operator, and whether this crate
  may ever fetch is open at the charter's section 9.

**Nothing here reaches an agent.** The authoring path writes this crate's
own store, and a declaration becomes something a box will load only where
the operator places it as that box's configuration file, which is an act
outside this crate and outside admin's verbs. **Writing a declaration here
changes no agent**, which is what makes composing safe and what makes the
last `validate` answer a reading rather than a promise.

## 4. The read path

Three queries, and the schema of section 2 exists to make each an index hit.

1. **One position's alternatives**, by run, turn and position. This is the
   click, and it is `weaver-analysis field` served from the store rather
   than re-derived.
2. **A contiguous range of positions** carrying the emitted token, surprisal
   and entropy. This is the timeline and the transcript.
3. **The run's tuple.** This is the label on every reading taken from it.

A surface that needs a fourth query is a surface this document has not
described, and it returns here before it is built.

**An open query surface is admissible on one condition: the query is
recorded beside its result.** A reading is a thing a second person reruns,
so a result whose query text was not stored is not quotable and this crate
does not present it as one. **Section 2.6 is where it lands.** That keeps
section 2.7's rule rather than spending it: the derivation is recorded
rather than absent, and what section 2.7 forbids is a derivation nobody can
find.

## 5. Staged experiments

**This crate has no model behind it and forks nothing.** A click authors a
staged experiment holding the parent run reference, the branch position, the
forced alternative token where one is forced, and the parent declaration
with its diff. A runner drains the queue, and each result returns as a run
in the schema of section 2 carrying its parent reference and branch
position, so the comparison needs no reconstruction.

### 5.1 The five states

```text
draft       editable in every field
registered  frozen, the claim on the record, eligible for a batch
queued      handed to a runner, waiting
running     a runner holds it
returned    results in the store
```

**Registration is the freeze**, not the launch. A draft is a draft and
editing one is what drafts are for, because a draft has no result its author
could have seen. Registering puts the claim on the record whether or not it
ever runs, and queueing is a separate act. **What was registered and never
run stays in the record**, which is what makes pre-registration a property
of the interface rather than a discipline imposed on it.

### 5.2 The diff is split by when it takes effect

Both kinds reach the agent through a reload, since the load boundary is the
only change boundary. They differ in **what the result licenses**:

- **Per generation** - seed, temperature, top-p, top-k, repetition,
  maximum tokens. Same weights, same window. The recorded prefix re-feeds to
  the state the parent had, so divergence below the branch position is
  attributable to the one value moved.
- **Load-time** - artifact, precision, devices, context capacity, and the
  elections. The prefix re-feeds **under different weights or a different
  window**, so the parent's internal state is not reproduced. The text
  upstream matches and the state does not, and **the comparison is
  structural rather than byte-exact.** A load-time move also derives a
  declaration, which stands in Agents beside its parent.

The interface states both consequences where the change is made.

### 5.3 Validation at authoring

An experiment is refused when it is authored, not when it is drained.

```text
the parent record is readable, and holds the branch position
the context capacity holds the prefix
the forced token is present in the capture at that position
every artifact the diff names resolves
```

**A runner cannot ask.** A refusal discovered at three in the morning costs
a batch window, and the same refusal at authoring costs nothing.

## 6. The surfaces

One module each, the Experiment view and Models included: each is a surface
with its own destination rather than a mode of a list beside it. Their
destinations are the charter's section 3 and are not restated.

**A surface that renders what is kept reads the store and nothing else** -
Open a trace, Record, Experiments, and the returned half of Stage. That is
what makes section 4's three reads sufficient for them.

**Three surfaces author, and each writes one table.** Compose writes a
declaration, Stage writes a staged experiment, and Models writes an
artifact row on import, each through section 3.2 and each into the table
section 2 gives it. **Models is the only one of the three that is not on
the path**, which is why the group of section 3.6 is named for how it is
reached rather than for what it does.

**A surface that authors or exchanges also holds a seam.** Compose writes
its draft and asks `validate`. Live carries a turn to the gate and reads the
measurement that comes back. Agents drives the lifecycle verbs and reads the
observation exchange. Section 7 names those three. **Stage submits a
registered experiment to the queue, and section 7 names no queue seam**, the
queue being the harness's per the charter's section 3.5 and this crate's
part in it a write the runner drains rather than an exchange it holds open.
**None of them writes the recorded half** - section 3.1's ingest is the only
writer of a position or a run - and each writes only the authored table it
owns, through section 3.2. None reads the agent except through a seam that
is named.

**The state a surface holds is a query, never a location.** A filter chip is
a clause, and clearing it widens the list in place. A card carries the
operator into a list with a chip already set, and no surface has a variant
that differs only by a pre-applied filter.

**A timeline is drawn on ordinals and a click addresses a position, and the
surface converts between them.** The per-generation series is indexed by the
ordinal within its generation, and the field is addressed by the resident
length at the draw. They are different coordinates and the record spells
both, so the conversion is the surface's to make and to make once. A view
that passes an ordinal where a position is expected reads a different token
and says
nothing about it.

**Absence renders as absence.** Entropy rides every generation
unconditionally, and surprisal rides only where its election stands. A
surface that plots an absent surprisal as zero is lying about the election,
so where the election did not stand the surface says so rather than drawing
a floor.

**A reading is produced only where the record's own bracket permits it.** A
serving record carries no gate. A diagnostic record carries one, and a
reading from an uncertified replay is a picture of an unknown run. This
crate honors that gate rather than re-deciding it.

## 7. The seams

### 7.1 The gate

A turn crosses at the gate as any client's does. The gate does not stream,
so a whole-turn answer is presented whole and an in-flight state is clear
rather than simulated. Closes render by kind, and an unnamed close is this
crate's own defect and surfaces as an application error rather than as an
agent's words.

### 7.2 The admin verbs

`validate`, `load`, `unload`, and as of 2026-09-04 the observation exchange:
`show` answers one agent's load facts and `list` answers one summary per
admitted agent in a single ask. Load state is therefore **the harness's own
word rather than an inference from a socket's existence**, and no surface
labels it as inferred.

**The observation answers from any position, a running turn included**, as
of 2026-09-05. It is served from inside the turn between tokens, touching no
bracket and cancelling nothing, and the one bound is the single token whose
decode is in progress. So **a roster may read while an agent is answering**,
and a surface has no reason to withhold a status read during activity or to
present a stale one as current. A view built against the earlier posture
would have designed around a refusal that no longer stands.

No verb chains another. This crate offers each as a separate act and nothing
composite.

**`validate` is also the composition oracle, and what it can answer is
bounded.** It transitions nothing, refuses an incoherent declaration naming
the field, and reaches **the box facts admin holds custody of**, per
`weaver-admin-PRD` section 4.3 as ruled 2026-09-05 on issue #456: admin
adjudicates what it provisions, asks the owner where one can be asked
before a process exists, and leaves to the organ what only the organ can
judge. So the Compose surface writes its draft and asks rather than
judging, **and a clean `validate` is acceptance for filing rather than
approval to load.** Whether the artifact resolves, whether the family
exposes the taps the declaration elects, and whether these weights load at
this precision are answered at admission under the agent's identity and
not here. **This crate therefore carries no second copy of the
rules**, only a copy of the declaration's field shape written against
`weaver-types-Spec` section 2 **at a named corpus commit**, which is that
copy's staleness rule: when the floor moves, the pin says so and `validate`
refuses in a way the surface can name.

### 7.3 The analysis stream

The emission this crate ingests. **Its shape is
`weaver-analysis-web-contract`**, landed 2026-09-05 per issue #418, and this
document names the seam and restates none of it. Two of that contract's
clauses reach back into this document and are worth naming where a reader
of this section stands.

**The series is addressed by the ordinal and this crate's store by the
position**, per section 2.1, and the conversion is this crate's to make at
ingest. **What it converts from is owed and does not cross yet**,
issue #461: the contract's section 3 asks the emitter for the resident count
at a generation's close and its output token count, on the summary stream,
and until that lands this seam carries the ordinal alone. **A reader in that
state cannot key its store** and stores what crosses without converting,
rather than deriving an address from a figure that does not answer.

**A diagnostic record whose bracket did not close certified is not drawn**,
the refusal naming the outcome. A serving record has no gate and none is
owed.

## 8. Placement and the link

Two processes joined by one dialed link: a connector holding the box-bound
reaches, a server holding the presentation stack. Colocated by default and
separated by changing one address.

**Nothing in the read or write path is box-bound to the agents.** The reader
is a store client, the runner is a queue consumer, and the front end with
its store runs on one machine while the agents run on another. That crossing
is a declared boundary under the charter's section 5 rule and appears in the
cell record like any other.

## 9. What is enforced, and by which instrument

| claim | instrument |
|---|---|
| a position is addressed by run, turn and position | compile-pin on the key type |
| ingest is idempotent on that key | perturbation: replay one window twice |
| nothing is computed at read time | review, over the three reads |
| a recorded query names every run it addressed | perturbation: drop one, the row refuses |
| an incomplete shard set joins to nothing | perturbation: drop one file the index names, the join returns none |
| presence never gates a load | review, over the load path: this crate's catalog is not read there |
| a registered experiment is immutable | compile-pin: no mutating path off the frozen type |
| a forced run is marked in the record | perturbation: strip the mark, the read refuses |
| an absent surprisal renders as absent | perturbation: zero-fill, the view is wrong |
| the position is stored at ingest | perturbation: after ingest, alter the summary's counts and reread, the stored position is unchanged |
| an uncertified diagnostic record is not drawn | perturbation: drop the outcome check, an unknown run renders |
| an undeclared boundary refuses the load | perturbation, at the admit path |
| no surface writes a position or a run | compile-fail: a doctest constructing a recorded-table writer from an authoring path does not compile |
| an authored edit against a stale version refuses | perturbation: drop the version check, the second edit silently wins |
| a registered experiment's question is immutable with the rest of it | compile-pin: no mutating path off the frozen type reaches it |
| import computes the identity rather than accepting one | perturbation: take the operator's digest, two boxes disagree about one artifact |
| a record identity names at most one catalog row for a file or a directory artifact, a renamed split excepted per sections 2.3 and 10 | perturbation, at the schema: drop the unique index that holds for every shape but a split, a second import of the same file or directory opens a second row and a lookup answers two where it owes one |
| the sentinel joins to nothing | perturbation: register the empty string as an identity, a run whose hash failed joins to an artifact it never named |

**A watch that cannot fail is not a test.** For each perturbation above, the
act that lands it states what removal makes it fail and confirms it does.

## 10. Open elections

- **The store engine and its migrations.** Postgres is the charter's
  election, and the schema's expression is this document's and lands with
  the first code act.
- **The field election's depth**, which is the charter's open cell and not
  this crate's to set.
- **Whether surprisal and entropy draw as one timeline or two.**
- **Whether the preset ladder is a picker or a wizard.**
- **Who reports an artifact's presence on a box**, which the charter's
  section 9 holds open. Section 2.3 stores the entry with its reporter and
  its date whichever answer lands, and no read of it gates anything, so
  this document is not blocked on the choice.
- **Which identity the catalog and the record share, closed 2026-09-06.**
  Two rules stand at two grains and the row carries both: section 2.3 keys
  on the per-file map the lens refuses on and carries the record's weights
  hash beside it, computed at import by the rule `weaver-spu-Spec` section 3
  now states, and the run row fills its member from the summary the analysis
  seam carries as of the same date. Neither rule was dropped, because the
  lens's refusal rests on one and every record on the other, and the
  derivation between them runs one way. Per issue #465.
- **The split GGUF's names, which the record's identity does not cover.**
  `weaver-spu-Spec` section 3's hash of a split covers the shards' bytes in
  split order and not their names, so two split sets of identical bytes
  under different stems are one record identity and two rows of section
  2.3, and the lookup reports both rather than picking. Two answers stand:
  the SPU's split manifest carries the names, which changes the identity
  every later admit of a split records and is that Spec's to rule, or this
  catalog keys a GGUF by the record's identity alone, a GGUF's shard names
  being the operator's and not the artifact's where a safetensors index is
  the artifact's own. No split GGUF stands in a record or a model directory
  this seat can reach, so the case is named before it is met.
- **Whether this crate scores a correctness verdict**, which the charter's
  section 9 holds open. The reproduction verdict is not open and is not a
  score: it is the projected comparison of two rows this crate holds, on
  the fields the charter's section 4 names, and section 2.2 carries the
  tuple that decides whether equality is claimed or a divergence is
  reported. What stays open is whether a correctness column exists beside
  it. Where one does, **the scorer is named on the verdict and never in the
  run's tuple**, so a second scorer adds a verdict rather than changing what
  the run was.
- **The word "cell"**, which carries a second sense elsewhere in the corpus
  and must be settled once rather than twice.
