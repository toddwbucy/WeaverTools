# Sketch: Ablation Matrix Surface

**Status:** SKETCH. Decides nothing. Feeds PRD authoring.

**Version:** v0.1, 2026-09-09. Parent: Sketch Convention. Written from a walk on the
same date against the tree at PR #518. Feeds `weaver-web-PRD` sections 3.5, 3.6 and 4,
and the Spec's section 2 where the authored rows live.

**Not to be confused with** the Diagnostic Replay control loop. That one reruns a
failed run under the diagnostic binding to locate a fork to a position. This one takes
a run that succeeded and reruns it many times under moved conditions to find out what
the success depended on. Same entry point, a trace, and opposite question.

Two mockups accompany this sketch in `mockup-ablation-matrix.html`. They are pictures
of the shape and not a design commitment, and the code session should read them the way
it reads this prose: as what was considered.

## 1. Purpose

To take one run whose outcome is known and ablate the conditions it ran under, so that
the operator can say which members of the tuple the outcome depends on and by how much.
The instrument the crate already has, a run as a cell carrying its declared tuple, is
the unit. What is missing is the surface that generates the cells from a parent, holds
them as a plan before any has run, schedules them as a batch, and reads them back side
by side once they have.

The larger purpose it serves is stated once so that the smaller decisions below can be
checked against it. An agent deployed for a task should carry a bounded failure
envelope: under these conditions it completes the task at this rate, and when it fails
it fails within this region. The envelope is established by ablation before deployment
and is the artifact a third party could register. The runs that establish it are
scaffolding. What has to survive is the registration: which arms contributed to the
bound, under what tuple, and how each was scored. Everything else is discardable, and
the schema should make the discard cheap rather than protect against it.

The same pass produces two further things at no extra cost. Arms that succeeded well
are exemplars, and tagged exemplars are training data for a fine-tune that narrows the
model to the task. A narrowed model does not only succeed more often. Its off-rails
behavior stands out against its own baseline instead of blending into ordinary variance,
so the envelope gets sharper on both sides. The matrix is where the tagging happens
because it is where the arms are already side by side.

## 2. What it is not

It is not a commit graph. The session's runs already form a tree in the store through
the parent reference and the branch position, and the store holds that tree
relationally. Storing objects content-addressed in Postgres was considered and set
aside: git shares whole objects and a branch shares a prefix of a turn, the front end
reads rows and not objects, and objects in the database would make the database the
primary record when the write-once stream is. What survives from that consideration is
the digest on the run row, so a row can be verified against the operator's record, and
the idea of refs, below.

It is not a merge. Two arms under different tuples are exhaustive alternatives and
never rejoin. A merge would assert a reconciliation the experiment exists to deny, so
no run has two parents and the schema never represents one.

It is not a judge of quality. The predicate an arm is scored against is binary over an
objective criterion the task supplies, and where the task is a game the map knows the
answer. Quality of the kind that separates a competent essay from a good one is a
judgment of taste rather than of task success. That is not a claim that the method
cannot reach quality. It is a claim that quality is not enumerable as a pass or fail
when everything else is held, so it cannot be the thing the envelope is stated over.
It waits on its own instrument, and section 6 names the candidate.

It is not a ledger of every run. The columns are the arms of one parent, a handful
side by side. A view with a thousand runs across the top is a spreadsheet and not an
instrument.

## 3. Components

The **Record** list supplies the entry. A run whose trace is on hand gets an action
beside it that opens the matrix pre-staged for that run. Any run can be a parent, and
the ordinary case is one the operator already ran to completion and can reproduce
bit for bit, which is what makes it a baseline.

The **Stage** surface authors the plan. Per the PRD's section 3.5 this crate branches
nothing: a cell the operator schedules becomes a staged experiment carrying the parent
run reference, the branch position where the arm departs from the parent's prefix, and
the parent declaration with its diff. The reload that runs it is the branch. The
matrix is a way of authoring many staged experiments against one parent at once and
of reading them back, and it introduces no second write path.

The **runner** drains the queue, as it does today. Scheduling the batch is a cron job
and needs nothing new from the harness.

The **store** gains two authored objects, the plan and the refs, described in section
5.

**HeroBench**, or whatever task supplies the parent, supplies the predicate and the
denominator. Neither is the framework's. The task is a consumer holding a motive, and
the framework holds the mechanic. Where the task is a generated map, the map's optimal
step count per task is computed when the map is generated and travels as the map's
metadata, so every arm on that map is scored against the same denominator and no
solver version can silently rescore history.

## 4. The sequence

The operator has a run that completed the task, under a tuple held at every member,
and has rerun it and gotten the same bytes. This is the parent. It is the exemplar
before any ablation and the control arm after.

From Record the operator opens the matrix on that run. The matrix arrives pre-worked.
The tuple's members are the rows, and the parent is the first column with every cell
reading held at its value. The remaining columns are candidate arms generated from
the closed tuple space, one field moved per column by default, because a full
factorial over even five fields at three values is two hundred and forty three arms,
which a cron job does not mind and a screen does.

The operator trims and adjusts. A column can be dropped. A cell can be set to a
different held value, or freed. **Freed means Stage draws the field's values at
authoring and registration freezes them with the rest of the row**, per the Spec's
section 5.4, so a freed field is the column's swept member, its value set is the
sweep's, and the record carries what each arm ran under. A column frees at most one
field, because a sweep is one member and its values, and the other cells the operator
moved are the column's diff. **The draw excludes the parent's own value**, because the
Spec's section 5.3 refuses a sweep naming it and 5.4 has the parent as the control, so
a draw that lands it draws again. Setting two or three cells in one column is the
ordinary hypothesis: temperature to 0.7, seed freed, and everything else held. The
hypothesis is the diff between that column and the parent's. No prose field on the
plan carries it, because two tuples and a delta state it mechanically and checkably.

**The staged experiment each column becomes still carries the question of the Spec's
section 2.5**, which the schema holds not null. At scheduling the matrix fills it with
a rendering of the column's diff, the operator may replace that text before
registration freezes it, and the plan holds no copy. So the question stays where 2.5
puts it and nothing else in the store holds it, and a column's hypothesis is stated
twice in two kinds: mechanically as the diff, and in prose as the question the diff
was rendered into.

The operator schedules the batch. Every column becomes a staged experiment. Every
cell in a scheduled column that was neither moved nor freed reads held at the
parent's value, so a column is always fully specified.

The runner drains. As arms complete, the column's status changes and its score
appears: the predicate's verdict, and where the task supplies a denominator, turns
taken over optimal.

The operator reads across. Because the seed was freed, an arm is not one reading but
a distribution, and what the envelope is stated over is that distribution's tail
rather than its mean. Five nines is a claim about the worst arms.

The operator tags exemplars. Among the arms that passed, some passed better, and the
operator's judgment picks them. A tag is a ref, section 5, and a tagged arm is pinned.

## 5. The record shape

The hard part is the store, and it is narrower than it looks. Runs and their lineage
exist. What is new is that the matrix needs the plan as a first-class object, because
a run row can only say what a run did, and the matrix needs a row for a cell that has
not become a run and may never.

**Two new authored objects, and the first is the plan with its cells.** A plan names
its parent run. A cell names its plan, a column, a tuple field, a disposition, and a
value where the disposition needs one. A column, once scheduled, points at the staged
experiment it became, and reaches its runs through that row and never directly: the
staged experiment carries the runs it produced per the Spec's section 2.5, one per
value where the column freed a field, and section 4's fourth read returns them each
with its value, an arm that never ran keeping its place. A column is therefore never
one run row, and a column that freed nothing is a sweep of one value. The pointer is
nullable and that null is the record of a column that was not scheduled.

**A cell has four states and is never blank.** Held at a value. Freed, its value set
drawn at authoring and frozen at registration per section 4, with each arm's value
filled in from the run row once one exists. Queued, meaning the column is scheduled
and the run has not completed. Not exercised, meaning the column was never scheduled.
Not exercised is a state a reader can see, so a column the operator declined is
distinguishable from one that failed to run.

**Declared and achieved are two facts landing at two times.** The cell carries what
was intended. The run row carries what ran, per the PRD's section 4, and where the two
disagree the run row is right and the cell is a plan that did not survive contact.
The matrix renders the cell until a run exists and the arms' run rows after.

**Refs are the second object, and they pin what the envelope cites.** A small table of
named references from a person to a run row. An exemplar tag is one. A citation from a
published bound is another. A run reachable from no ref is scaffolding and is what a
sweep may discard, and the throwaway problem of section 1 reduces to reachability,
which is the one thing worth taking from git's model.

**Joins are free at this scale and token-grain rows are not.** The people this store
serves are a team of developers, not the public, and the join from cell to run row on
a nullable key is the join Postgres is best at. What can get slow regardless of user
count is anything that grows with tokens. The matrix reads the run row's summary and
never the position registry. Every column header is a link, and the interior lives on
the far side of that click, on the surfaces that already exist for it.

**The tuple denormalized on the run row is what makes this cheap.** The PRD's section 4
already puts everything identifying the conditions on the run's own row. The matrix
depends on that holding, so that a column is two reads and no traversal.

## 6. Open cells

**Batch composition is the last field and the only one the architecture was built to
eliminate.** Exercising it means introducing co-resident sequences on purpose. The
cheap path is filler of no particular provenance, since the more arbitrary the company
the better the measurement, and the filler needs no seed because it travels in the
trace as presented material. What it does need is a trace kind for material the agent
was not addressed by and did not author, which does not exist, and a batch shape field
that names count and lengths and arrival order rather than size alone. The measurement
that settles the shape question is whether size alone reproduces the divergence or
whether composition is needed. Parked until the other fields are done.

**Full factorial or one-at-a-time as the generated default.** One-at-a-time answers
most of what an operator asks and fits a screen. The measurement that would move the
default is an interaction effect the one-at-a-time plan cannot see, and the six-field
ledger of 2026-09-07 names precision as an amplifier of the others, so the first
interaction to look for is precision against any one other field.

**A quality instrument.** The candidate is not perplexity, which measures how expected
a text is and ranks fluent mediocrity well. Surprisal and perplexity are one quantity
at two grains, so combining them adds nothing. What may discriminate is the shape of
per-token surprisal rather than its mean, since a plain essay is uniformly unsurprising
and a good one has spikes where a domain term or a subordinate clause lands. The SPU
already hands surprisal per token. The measurement: hold out half the tagged exemplars,
build the metric on the rest, and see whether it ranks the held-out half above the
rejected arms. If it cannot, it is measuring the selection and not the quality.

**Whether the plan is authored on Stage or is its own surface.** Section 3 puts it on
Stage because it introduces no new write path. The measurement is a reader test: hand
the mockup to someone who has used Stage and ask where they expect to find it.

**The digest on the run row.** Verification without dedup. At a few megabytes per run
dedup does not matter, and this sketch treats the digest as sufficient. The measurement
that reopens it is a run size at which the operator's record outgrows the disk the
sink writes to.

## 7. Carried to the PRD act

What the review of PR #522 found and this sketch does not settle, listed here so the
act that authors the PRD inherits it from the tree. Each is a fact about where the
sketch and the merged Spec disagree, or where the sketch draws a thing no document
holds.

**The score has no home.** The matrix reads a verdict and a ratio from the run row,
and the run row holds no verdict: the Spec's revision of 2026-09-05 has the scorer
leave the run's tuple for the verdict. The task supplies the predicate and the
denominator, and the store still has to hold the reading. Section 2.6's recorded
query is the candidate.

**The Reproduced column is a read-time pairwise derivation.** "3 of 3 byte-equal"
compares a run against three others. Spec section 2.7 forbids a value computed at
read unless the query is recorded under 2.6, and section 10's open election has the
reproduction verdict as the projected comparison of two rows. So the column needs a
stored reading or a recorded query per row shown. Same family as the score.

**The branch position has no home in the plan.** PRD section 3.5 and Spec section 5.3
have every staged experiment carry the branch position and validate that the parent
record holds it. The plan names its parent run and the cells name a field, a
disposition and a value, and the mockup shows no position. For a whole-run ablation
the answer is presumably the run's first position, and the act should say so, because
it is the one member the matrix cannot derive from the tuple. A reproduction run is
the same question one level down: the schema has it carry a branch position and the
sketch does not say which.

**The column states map onto section 5.1's five.** The mockup reads editing, not
scheduled, queued, ran and fail, where the Spec has draft, registered, queued, running
and returned, with registration as the freeze and queueing a separate act. The
sketch's Save plan and Schedule batch never name registration, and "not exercised",
defined as never scheduled, cannot tell a column registered and never queued, the
pre-registration case PRD 3.6 prizes, from one that stayed a draft. The act maps
column states onto the five rather than carrying a second set.

**Attribution is parent-relative in the Spec and compound in the mockup.** Spec 5.2
makes divergence below the branch position attributable to the one value moved.
Column B moves precision and temperature and frees the seed, so the comparison that
isolates precision is B against A, which the matrix does not draw, and section 6's
one-at-a-time default is not what the mockup's A, B and E do. The act says whether
the diff row is parent-relative only or a column may name another column as its
baseline.

**The word cell.** Spec section 10 holds open "the word cell, which carries a second
sense elsewhere in the corpus and must be settled once rather than twice." This sketch
uses it in section 5 for a matrix entry and in section 6's title for an open question.
The open election is where it is settled, and this sketch is the pressure on it.

**The plan and the refs are authored rows.** Spec 3.2 gives every authored row an
author and a version. The mockup says who authored the plan and the refs name a
person, and section 5's prose carries neither member for either object. Both land
under 3.2.

**Two citations.** "The six-field ledger of 2026-09-07" resolves to issue #485, which
registers five tuple fields with a sixth run added in its comments, and the hub report
`weavertools-testing/2026-09-07-determinism-matrix-audit`. Neither says amplifier, so
the act sources that reading or drops it. The trace-kind claim of section 6 stands
against `weaver-trace-Spec`'s kind list, which names nothing for presented material.

**The digest on the run row** is decided in section 2 and landed by issue #521's act
at `weaver-web-Spec` section 2.2, beside the session identity, on the operator's
ruling of 2026-09-09.

**"Parent: Sketch Convention"** in the header names a document that is not in the
tree. The two standing sketches cite none.
