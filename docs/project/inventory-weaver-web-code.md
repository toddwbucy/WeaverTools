# Inventory: the weaver-web code against its rewritten charter

**Status:** INVENTORY, opened 2026-09-06. A working register rather than a
member of the document set, and **nothing here is decided until the operator
rules on it.** It reads the code that stands in `crates/weaver-web` against
the charter and Spec that replaced the ones it was written to, and says of
each part which of three things it is.

**Why it exists.** `weaver-web-PRD` and `weaver-web-Spec` were rewritten
whole on 2026-09-04 for the instrument, replacing a charter written for an
interface to an individuated agent. **The implementation was not read in that
act and has not been read since.** It stands at 4,531 lines of Rust, eleven
templates, and a Postgres schema, all written to the retired text, and no
document in the corpus records the gap.

**Revised:** 2026-09-06, third of this date, against the review of PR #468. The
observation reports that no unit is installed rather than that nothing serves the
database, a process or a hand-started binary not having been checked. **The 37 rows the
mirror argument did not reach are classified**: twenty-one member changes, whose subject
retires with `members`, and sixteen application errors, which no trace under admin's
custody carries because they never reached an agent, both left open rather than answered
by a claim about turns. The `src/` directory comparison says it is one. Entries run
newest first, which the second of this date broke.

**Revised:** 2026-09-06, second of this date, against the review of PR #468 and carrying
the answer that review brought. **The store the register could not see is reported for
one box**, with its five tables' row counts, its two-day span, and the finding that its
turn rows mirror traces held elsewhere, so question 2 is answered for olympus and open
for every other box. **The module citations resolve** against the Spec at `13b8a6a` of
2026-08-25, git being the archive, so a reader checks each verdict against the text its
module was written to rather than finding nothing, and the schema is named as the one
citation that resolves nowhere. `store.rs` cites sections 5 and 14 and not 12, which is
`traceview.rs`'s. **The crate has none of the seven directories**, where this register
twice said one. `0002_roles.sql` joins the sessions paragraph, having made the charter's
section 6 argument in the schema nine days before the rewrite made it in prose. Question
1 gains its third answer, an archive at a named path.

**Revised:** 2026-09-06, against the review of PR #468. The `sessions` table and the six
templates the first draft left unclassified are classified. `web/admin.rs`'s routes are
marked as inner and the `/admin` mount named. **And the claim that the store has no rows
that matter is withdrawn**: it was a claim about every deployment made from one box's
`systemctl`, and what stands in its place is the three questions someone with reach must
answer before any drop.

**Date filed:** 2026-09-06
**Document ID:** `inventory-weaver-web-code`
**Editorial:** Per the Working Rules. ASCII, absolute dates.

## What the reading found

**The charter describes a diagnostic instrument and the code implements a
chat client.** The store's tables are `participants`, `channels`, `members`,
`channel_events` and `sessions`. Among its eleven templates are
`channel.html`, `channels.html` and `sidebar.html`. The rewritten Spec's
section 2 names seven tables and not one of them is among those five.

**Every module cites a document the tree does not hold, and git does.**
`traceview.rs` cites "Spec section 12", `repro.rs` "section 17", `queue.rs`
"section 8", `wire.rs` "section 16", `store.rs` sections 5 and 14,
`Cargo.toml` "the Spec's section 15". The rewritten Spec has ten sections
and every one of those citations resolves against **the Spec at `13b8a6a`,
2026-08-25, the last commit before the rewrite**, where section 12 is the
trace view, 13 the HTTP surface, 14 sessions and roles, 15 the open
elections, 16 the link and 17 the confirm view. Git is the archive and the
tree is not, per the Working Rules, so **a reader checking any verdict below
reads that commit** rather than finding nothing.

The exception is the schema. `migrations/0001_init.sql` cites
`docs/SPEC.md section 4`, **a path that exists nowhere in the tree or in its
history**, so the schema alone has a stated authority that cannot be read.

**And the crate has none of the seven directories the Spec draws.** Section
1 names `store/`, `ingest/`, `authoring/`, `queue/`, `surfaces/`, `seams/`
and `link/`. Section 1's tree is a `src/` layout and this compares against it: the
crate's `src/` directories are `adapters/`, `bin/` and `web/`. Beside `src/`
the crate root holds `assets/`, `deploy/` and `migrations/`, which section 1
does not draw and which this register counts elsewhere. What carries
section 1's names are files rather than modules: `store.rs`,
`queue.rs`, `wire.rs`, `traceview.rs`, `repro.rs`, `router.rs`,
`registry.rs`, `lifecycle.rs`, `channel.rs` and `config.rs`.

## The seam the code separates along

**The retired charter's own two roles are where the code splits**, which is
the finding that makes this cheap rather than expensive. That charter named
a user who converses and an operator who drives lifecycle and reads the
record, and `web/mod.rs` says so in its own words: `user` is the gate
surface, channels and messages, and `admin` is the operator surface,
lifecycle verbs and trace views.

**The conversation half is what the rewrite retired. The operator half is
the instrument's ancestor.** The link that carries both was built against
the framework's public boundary rather than against the conversation model,
which is why it survives nearly whole.

## The register

Three verdicts. **Retires** means the rewrite removed its subject.
**Carries** means the new documents describe the same thing it does.
**Wants a ruling** means the mechanism survives and the subject moved, so
the operator says whether it is reworked or rewritten.

### Retires, 882 lines

| file | lines | why |
|---|---|---|
| `web/user.rs` | 429 | channels, messages, members, the session open. No surface in section 3 is a conversation |
| `router.rs` | 212 | mention parsing and multi-agent invocation routing. The rewrite has no mentions and no volley |
| `channel.rs` | 130 | channel reads and log pages |
| `registry.rs` | 111 | participants and providers reconciled into a participant registry. Section 2.4's table is declarations, which is a different noun |

Five of the eleven templates go with them: `channel.html`, `channels.html`,
`sidebar.html`, `name.html` and `event.html`. **The other six follow their
modules** rather than retiring on their own: `base.html` carries,
`lifecycle.html`, `agent_config.html`, `trace.html`, `trace_event.html` and
`repro.html` want the ruling `web/admin.rs` wants, being that module's
rendering.

Four of the five tables go with them too: `participants`, `channels`,
`members` and `channel_events`. **`sessions` wants a ruling and is the one
table that is not simply retired.** It holds a token, an opened and a closed
timestamp, and a `participant_id` referencing a table that retires, so its
foreign key goes whatever else happens. Whether a browser session survives
at all is the charter's section 6 question rather than this register's:
identity and authentication are deferred there with a named trigger, so the
table's subject is deferred with them.

**The second migration is where that question was already anticipated.**
`0002_roles.sql` adds `role` to `participants` on the operator's ruling of
2026-08-19, its own comment saying the two surfaces separate now so that
identity later attaches to standing roles rather than forcing a
rearchitecture. **That is the charter's section 6 argument, made in the
schema nine days before the rewrite made it in prose**, and it is the one
place in the crate where a ruling is cited by date. The column retires with
`participants` and the reasoning does not.

### Carries, 1,770 lines

| file | lines | what the new documents call it |
|---|---|---|
| `wire.rs` | 813 | section 8's dialed link. Its seven services are `turn`, `verb`, `status`, `declaration`, `trace_runs` and `trace_run`, six asks correlated by id, plus `trace` streaming unasked. **Every one maps to a surface or seam the rewrite keeps** |
| `store.rs` | 344 | the Postgres pool, the migration runner and the single writer task. The infrastructure survives whole and the schema it runs does not |
| `adapters/gate.rs` | 150 | section 7.1, the gate seam, dial-per-turn |
| `config.rs` | 128 | section 8's placement: box facts in the box's config, the roster announced in the link's hello |
| `lifecycle.rs` | 101 | section 7.2, the admin verbs, one JSON object rendered verbatim and failure never swallowed |
| `bin/`, `lib.rs`, `adapters/` | 234 | the two processes section 8 names |

**`wire.rs` is the most valuable thing in the crate.** Link loss marked and
never smoothed, pending asks failing typed, discontinuity marks inserted
into every trace view: that is the absent-not-empty discipline the rewritten
charter argues for, already built.

### Wants a ruling, 1,879 lines

| file | lines | the question |
|---|---|---|
| `web/admin.rs` | 598 | **inner routes**, mounted by `web/mod.rs` under `/admin`: `/lifecycle`, `/lifecycle/{agent}/{verb}`, `/agents/{agent}/config`, `/trace/{agent}`, `/trace/{agent}/stream`, `/repro/{agent}`, so the served paths carry that prefix. Those are Agents, Compose, Open a trace and reproduction, at four of the ten surfaces. Reworked or redrawn |
| `repro.rs` | 369 | pull a run from the record, drive its turns back through the gate on a fresh load, compare field by field. **The rewrite keeps reproduction as measurement**, and its own comment already says a confirm is the operator asking the record a question rather than conversation. The comparison's projection is now section 4's and was not then |
| `traceview.rs` | 327 | the connector tails the NDJSON and the server holds bounded rings. Section 3.4 keeps the surface, and section 7.3's analysis stream is a different seam from a raw tail |
| `queue.rs` | 311 | per-agent single-flight with batch-on-drain. The rewrite has a queue and it holds staged experiments drained by a runner, so the mechanism survives and the subject changes |
| `web/mod.rs` | 274 | the HTTP surface's split into `user` and `admin`. The split itself is the retired charter's two roles and the rewrite has one operator |

## What the operator rules

1. **Whether the retiring 882 lines are deleted, kept, or moved.** Deleting
   them is cheap and makes the crate read as what it is. Keeping them means
   the crate builds and serves something while the instrument is written
   beside it. **Moving them is the third answer**: an archive at a named path
   with a checksum, which is the shape the quarry was frozen under, keeping
   the conversation half reachable to whoever writes the chat interface the
   vision still names. Git holds them either way, so the choice is about
   what a reader of the tree meets rather than about loss.
2. **Whether the schema is migrated or replaced.** The five tables share
   nothing with the seven, so a migration between them would be a drop and a
   create rather than an alteration, and there is no column in the old set
   that a new table wants. **What would be discarded is answered for one box
   and open for the rest**, per the observation below.
3. **What the four ruling-wanted modules become**, one by one, and in what
   order against the surfaces.
4. **Whether the crate's edition alignment rides this work.** `Cargo.toml`
   pins 2021 against the workspace's 2024 and calls the migration real work,
   citing a Spec section that no longer exists.

## What stands in the store, as reported

**A dated observation by a named reporter**, which is the shape this crate's
own section 2.3 requires of presence and is the honest shape for this too.
No store stands on the box this register was written from, `postgresql`
being inactive there, and one stands on olympus.

**Reported by the olympus seat on 2026-09-06**, from a `weaver_web` database
with both migrations applied:

| table | rows | |
|---|---|---|
| `participants` | 5 | one admin, four users |
| `channels` | 11 | |
| `members` | 32 | |
| `channel_events` | 259 | turn-open 87, close 71, message 64, member-change 21, app-error 16 |
| `sessions` | 5 | |

**Every event falls between 2026-08-19 and 2026-08-20**, the two days around
the roles ruling `0002_roles.sql` carries. The turn rows name eighteen runs
of the alpha agent by run label and ten turns by turn label.

**No `weaver-web` unit is installed on that box.** That is the observation.
Whether a process, a container, or a hand-started binary serves the database
was not checked, so this register says a unit is absent and does not say
nothing is serving it.

**What this does and does not settle, by row.** The 158 turn rows, opens and
closes, are **a mirror of turns whose canonical record is the trace under
admin's custody**, so dropping them loses a copy rather than a fact, on the
condition that the eighteen runs' traces still stand, **which is a check for
whoever rules on the drop and not a thing this register asserts**. The 64
message rows are the conversation itself and have no record elsewhere, so
whether they are wanted is the operator's.

**The remaining 37 rows are unclassified and the mirror argument does not
reach them.** Twenty-one member changes and sixteen application errors are
neither a turn nor a message: the first is channel membership, whose subject
retires with `members`, and the second is this crate's own faults, which no
trace under admin's custody carries because they never reached an agent.
**Whether either is wanted, and whether the app-error rows are the only
record of a fault worth reading, is a question this register leaves open**
rather than one the mirror answers.
And **which other boxes hold a store, neither seat can see.** This crate
runs on one machine while the agents run on another, per the charter's
section 5, so a store on a box neither seat reaches is the expected case
rather than an unlikely one, and the answer above is one box's and is not
the deployment's.

## What this does not decide

**It proposes no act.** The register is what a reader needs to see before
choosing one, and the choosing is the operator's.

**And it makes no claim about a deployed store**, per the third question
above, which is the one thing in this register that needs a reach this seat
does not have.

**It is not blocked by the artifact identity of issue #465**, which reaches
the artifact table alone. Nothing above waits on it, and the store's schema
act does, in the artifact table and in the run row's identity member.

## What the corpus should say and does not

Two documents carry statements this reading falsifies, and each wants an act
of its own rather than a line here.

- **`weaver-web-Spec` section 1's layout is aspirational and reads as
  descriptive.** It names seven directories and the crate has none of them.
- **Nothing anywhere records that the implementation predates its charter.**
  A reader of the rewritten documents would take the crate for empty, and a
  reader of the crate would take the charter for unwritten.
