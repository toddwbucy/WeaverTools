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

**Revised:** 2026-09-06, against the review of PR #468. The `sessions` table
and the six templates the first draft left unclassified are classified.
`web/admin.rs`'s routes are marked as inner and the `/admin` mount named.
**And the claim that the store has no rows that matter is withdrawn**: it
was a claim about every deployment made from one box's `systemctl`, and
what stands in its place is the three questions someone with reach must
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

**Every module cites a document that does not exist.** `store.rs` cites
"Spec section 12", `repro.rs` "section 17", `queue.rs` "section 8",
`wire.rs` "section 16", `Cargo.toml` "the Spec's section 15". The rewritten
Spec has ten sections. `migrations/0001_init.sql` cites
`docs/SPEC.md section 4`, **a path that exists nowhere in the tree**, so the
schema's stated authority cannot be read at all.

**And the crate's own layout is not the one the Spec draws.** Section 1
names `store/`, `ingest/`, `authoring/`, `queue/`, `surfaces/`, `seams/` and
`link/`. The crate holds `adapters/`, `web/`, `channel.rs`, `router.rs`,
`traceview.rs`, `repro.rs`, `wire.rs`, `registry.rs`, `lifecycle.rs`,
`config.rs`, `store.rs` and `queue.rs`.

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

1. **Whether the retiring 882 lines come out in one act or stay until their
   surfaces are built.** Deleting them is cheap and makes the crate read as
   what it is. Leaving them means the crate builds and serves something
   while the instrument is written beside it.
2. **Whether the schema is migrated or replaced.** The five tables share
   nothing with the seven, so a migration between them would be a drop and a
   create rather than an alteration, and there is no column in the old set
   that a new table wants. **What this register cannot tell you is what
   would be discarded.** No store stands on the box this reading was taken
   from, `postgresql` being inactive here, and this crate runs on one machine
   while the agents run on another, so a deployed store elsewhere is exactly
   the thing this seat cannot see. **Before any drop, someone with reach
   answers three questions**: which boxes hold a store, what its tables carry
   by row count, and whether anything in `channel_events` is wanted as a
   record rather than as a chat log. This register asserts none of that and
   an earlier draft did, saying the store has no rows that matter, which was
   a claim about every deployment made from one box's `systemctl`.
3. **What the four ruling-wanted modules become**, one by one, and in what
   order against the surfaces.
4. **Whether the crate's edition alignment rides this work.** `Cargo.toml`
   pins 2021 against the workspace's 2024 and calls the migration real work,
   citing a Spec section that no longer exists.

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
  descriptive.** It names seven directories, of which the crate has one.
- **Nothing anywhere records that the implementation predates its charter.**
  A reader of the rewritten documents would take the crate for empty, and a
  reader of the crate would take the charter for unwritten.
