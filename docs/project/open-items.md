# Open items

**Status:** WORKING LIST. Not a frozen document and not in the freeze set. Items
enter when a pass surfaces something it cannot settle, and they leave when a
document settles them. An item that resolves is deleted from here and lives in the
document that resolved it, so this file shrinks as the corpus grows.

**Date started:** 2026-07-28
**Editorial:** ASCII, no em-dashes, no semicolons.

---

## 1. Apex corrections

`WeaverTools-PRD` was ratified before the crate pass could test it, which is the
same "written at different moments" failure the apex diagnoses in its own section
0. Section 0 says the document set is written together and frozen together, and the
apex is a member of that set, so it is not finished until the crate PRDs are.
Section 10 step 1, which ratifies the apex first, is the line that is wrong.

These are collected rather than fixed one at a time, and resolved in **one**
re-authoring after all seven crate PRDs are drafted. Re-ratifying three times
mid-pass costs more and produces a worse document. Seven is the count of charters,
which is what gates the re-authoring. The composition roots in correction 3 take
directories under the doc mirror without charters of their own, because they are
packaging rather than organs and their behavior is specified by the crates they
compose.

1. **Section 6 assigns `weaver-spu` encoder residency** while the turn in section 3
   never embeds. Gated on the encoder ruling in item 2.1.
2. **Section 6 seats lifecycle orchestration on `weaver-harness`.** Ordered load
   and unload, readiness collection, and rollback of a partial transition move to
   `weaver-admin`, which section 6 already seats with lifecycle intent and custody
   of the boundary. The harness cannot drive the early steps of its own creation,
   because the worker spawn and the descriptor handoff run as `weaver-admin` before
   the harness is running as the harness at all, and supervising worker and gate
   lifetimes is long-lived and fleet-wide while the harness is mortal and dies with
   its agent. Activity control, start and stop and cancel and interrupt inside a
   loaded agent, stays on the harness where section 6 and `weaver-harness-PRD` both
   already put it. This is a ratification against the apex, not a sync. **This is
   the freeze blocker.**

   **The table is the least of it.** The apex argues the coordinating-center model
   in prose, and every sentence of that argument inverts. Section 6 has the harness
   sequencing the transaction, collecting every organ's confirmation, and returning
   the aggregate to admin. The paragraph below it makes the harness the coordinating
   center of the agent, has it sequence the load, and routes all coordination
   between components through it. If admin sequences the load, then admin collects
   the confirmations and admin is the center of the transition, and the harness is
   one of the organs reporting up. The table row is a one-cell edit. The prose is a
   rewrite, and this correction names it so the re-authoring does not fix the table
   and leave a paragraph two screens down asserting the opposite.

   **One clause leaves a sentence whose other clauses stay.** The apex welds
   lifecycle coordination to trace authorship in a single breath: the harness
   creates the trace, records admin's initial contact as the first entry, sequences
   the load, and writes every component's activity into the trace. Three of those
   four are trace authorship and stay. Only "sequences the load" leaves. This is
   finer work than moving a paragraph and it is easy to over-cut.

   **Who authors the load events once admin runs the load.** The answer already
   exists in `weaver-harness-PRD` section 5 and is not in the apex, so the
   re-authoring will rediscover it the hard way unless it is recorded here.
   Components report and the harness authors, and the trace begins at admin's
   initial contact, so the pre-harness spawn and descriptor handoff sit outside it
   by construction.

   **The residual true statement.** The harness is the coordinating center of the
   turn, `weaver-admin` is the coordinating center of the load, and the harness is
   the sole trace writer across both.

   The word coordinator stays with the harness and is not reassigned. The apex
   already calls it that, at the line reading "the harness coordinator sequences
   the transaction," and coordinating the loop, the model, and tool dispatch is the
   harness's primary function. `weaver-admin` owns lifecycle under its own name.
   Any framing that invents a separate long-lived coordinator as a fifth party is
   wrong, because there is no fifth party, and it steals the harness's word for a
   box that is just `weaver-admin`. `weaver-harness-PRD` section 3 carries that
   alias and it comes out in this pass.

   **Travel together.** `weaver-harness-PRD` section 3 now states that orchestration
   and rollback are `weaver-admin`'s, which is ahead of the apex table. This
   correction and that paragraph ratify in the same act, or the harness PRD names a
   responsibility the apex still seats elsewhere. The alias removals elsewhere in
   that document carry no such dependency and can stand alone.
3. **Section 0 says "exactly seven crate PRDs."** The composition roots become
   crates, and the doc mirror gives them directories under `docs/crates/` whether
   or not they are organs. The count is nine.
4. **Section 4 item 3's custody phrasing.** "Writes that trace where the agent
   cannot reach it" is too blunt. The agent does reach it, through the harness. The
   precise claim is that there is no external path and every internal path is
   mediated.
5. **Section 2 frames the KV cache and the working structure as two things of a
   kind.** They are not. The working structure is the state the agent reasons over.
   The KV cache is an optimization holding the same content precomputed. Drop the
   cache and the agent is slow. Drop the working structure and turn two has nothing
   to be about.
6. **Sections 3 and 9 carry the two-representation split as inherited rather than
   justified.** It now has a reason: the disk artifact is a plain append-only file
   because universal tooling is the point of having an off-process artifact, and
   the in-RAM structure is relational because extension by schema is the point of
   having a queryable one.
7. **"Stateless" needs a disclaimer section and has a documented collision.** The
   previous tree used the same word for cold full-history resend, which is the
   opposite architecture, and section 2 exists only to walk the term back. What the
   program builds is an agent with session-scoped state and no cross-session
   persistence. Renaming would shrink section 2 from a disclaimer to a definition.
   Operator call, and the re-authoring is the cheapest moment it will ever be.
8. **Section 2 closes the working-structure bullet with "it is working memory, not
   a store,"** and one clause earlier rebuilds that structure exactly from the
   durable record without ever saying what the durable record is. The durable record
   is a store. It outlives the process, it outlives the session, and `weaver-admin`
   reads it for audit afterward. Both statements hold only if silently scoped to
   memory the agent can reach, and neither says so. One clause fixes it: scope the
   claim to agent-usable memory and name the durable record for what it is, a
   persistent audit store the agent has no path to. `weaver-tools-vision` section 1
   carries the corrected wording and can be lifted.
9. **Section 10 step 1**, per the preamble above.

## 2. Open rulings

**2.1 Does the encoder cross?** `weaver-harness-PRD` section 7 is written as though
it does not, with the reasoning recorded. The previous tree's justification for
holding encoder weights in process was that a cross-process hop on every memory
read is dead cost, and memory is out of scope. Under apex section 7 the embedder
serves no step of the turn and is not in the closed observability set. But the
previous tree ratified the opposite rule, that an agent without an embedder fails
closed, and retiring a ratified rule is an operator decision rather than one the
criteria make silently. Resolving this also resolves apex correction 1.

**2.2 The raw-trace policy.** The previous tree recorded "traces remain raw by
design," with no redaction. Defensible under the custody model, since the only
direct reader is `weaver-admin` and the agent has no path to the file, but the
record holds prompts, tool payloads, and token data and is secrets-grade. It will
be relitigated the first time someone notices. Needs one line, in
`weaver-trace-PRD` or `weaver-harness-PRD`.

**2.3 The observed instance for residual readout.** Apex section 8 requires the
capability without settling where the observed instance lives. Toggle in place
means unloading the agent, editing its state file, and reloading with capture on,
which takes production down while you diagnose. An observed twin keeps production
up but collides with the lifecycle rule that load never auto-evicts, so it needs
VRAM for two decoder residencies. Hardware consequence, so it should be decided
rather than discovered.

## 3. Deferred to the Spec pass

- **The in-RAM engine.** `rusqlite` in memory puts a C compile in the floor crate.
  Hand-rolling avoids that but must not regress to a fixed list of named reads,
  which is what made the previous tree's working structure a dead end when memory
  needed two more. A typed query builder over indexed tables is the shape that
  keeps extension cheap without a parser. The SPU open-core split is what decides
  whether a C dependency in the floor is acceptable.
- **The working structure must be rich enough** that a future activation network can
  attach and read it without reshaping it.
- **`event_log.rs` is 2,773 lines of hand-rolled durability** in the previous tree:
  canonical byte encoding, payload hashing, sequence-gap detection, a
  committed-versus-pending boundary, and commit-pressure policy. Worth scrutinising
  whether the implementation is heavier than the obligation requires. This is about
  the implementation, not about whether the format is right.
- **`weaver-admin`'s binary layout.** The CLI and the daemon both run as
  `weaver-admin`, so the compile boundary does not separate them and one crate with
  two binaries versus two crates is a code-organization call, not a security one.
  The worker is separate either way, because it runs as `weaver-<name>`. This does
  not gate the apex, which names `weaver-admin` as the lifecycle party without
  committing to its internal crate count, so it leaves the freeze gate entirely. It
  was mislabeled as resolving correction 2, and correction 2 is settled above by
  moving the responsibility, independent of how `weaver-admin` is packaged.
- **Trace descriptor rotation.** Settled in principle: `weaver-admin` opens
  descriptors and passes them to the worker over the coordination socket with
  `SCM_RIGHTS`, so the agent uid never resolves a trace path. The mechanism needs
  specifying, including what happens when a session needs a file that did not exist
  at load.
- **A GID mask at lifecycle boundaries** gives `weaver-admin` read-only access while
  the agent runs and read-write after unload. Clean on a plain file. It is
  incompatible with a WAL database, since a WAL reader must write the shared-memory
  index, which is one more reason the durable artifact stays a file.
- **The completeness contract** from the previous tree's tiered-access PRD survives
  re-grounded: a `trace_export` object carrying status, dropped count, and a drain
  error, with legacy manifests read as unknown and never assumed complete. Belongs
  in `weaver-trace-PRD`.

## 4. Deferred beyond the MVP

- **An admin-side database** as a trace consumer, if `weaver-admin` tooling ever
  wants indexed query over history. Additive by construction, because export is a
  formatter contract over the durable record and never the durable home. Keep it on
  a Unix socket if it happens. A network-attached database on the admin side would
  be the first thing in the architecture arguing against apex section 12.
- **A reconciliation in `weaver-tools-vision`.** The previous tree's autonomic-memory
  PRD proscribes decoder-invoked memory tools outright, arguing that
  retrieval-as-tool spends the decoder's budget on retrieval machinery instead of
  reasoning. Vision section 4 has the Hades-backed retrieval tool as an ordinary
  call out through the tool surface, which is the pattern that PRD rejects. Both are
  defensible and the vision document is the newer thinking, but they disagree and
  the vision document is where it gets settled.
