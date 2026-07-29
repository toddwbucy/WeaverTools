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
which is what gates the re-authoring. The composition roots in correction 2 take
directories under the doc mirror without charters of their own, because they are
packaging rather than organs and their behavior is specified by the crates they
compose.

1. **Section 6 seats lifecycle orchestration on `weaver-harness`.** Ordered load
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
2. **Section 0 says "exactly seven crate PRDs."** The composition roots become
   crates, and the doc mirror gives them directories under `docs/crates/` whether
   or not they are organs. The count is nine.
3. **Section 4 item 3's custody phrasing.** "Writes that trace where the agent
   cannot reach it" is too blunt. The agent does reach it, through the harness. The
   precise claim is that there is no external path and every internal path is
   mediated.
4. **Section 2 frames the KV cache and the working structure as two things of a
   kind.** They are not. The working structure is the state the agent reasons over.
   The KV cache is an optimization holding the same content precomputed. Drop the
   cache and the agent is slow. Drop the working structure and turn two has nothing
   to be about.
5. **Sections 3 and 9 carry the two-representation split as inherited rather than
   justified.** It now has a reason: the disk artifact is a plain append-only file
   because universal tooling is the point of having an off-process artifact, and
   the in-RAM structure is relational because extension by schema is the point of
   having a queryable one.
6. **"Stateless" needs a disclaimer section and has a documented collision.** The
   previous tree used the same word for cold full-history resend, which is the
   opposite architecture, and section 2 exists only to walk the term back. What the
   program builds is an agent with session-scoped state and no cross-session
   persistence. Renaming would shrink section 2 from a disclaimer to a definition.
   Operator call, and the re-authoring is the cheapest moment it will ever be.
7. **Section 2 closes the working-structure bullet with "it is working memory, not
   a store,"** and one clause earlier rebuilds that structure exactly from the
   durable record without ever saying what the durable record is. The durable record
   is a store. It outlives the process, it outlives the session, and `weaver-admin`
   reads it for audit afterward. Both statements hold only if silently scoped to
   memory the agent can reach, and neither says so. One clause fixes it: scope the
   claim to agent-usable memory and name the durable record for what it is, a
   persistent audit store the agent has no path to. **`weaver-trace-PRD` section 2
   carries the phrasing to lift**, naming the record a persistent audit store that
   `weaver-admin` reads and the agent has no path to, and stating that it outlives
   the process and the session both. `weaver-tools-vision` section 1 is a second
   corrected copy rather than the source. Both are targets of this fix as much as
   references for it, since a frozen-set member now says the record outlives the
   session while the apex still implies nothing does, and the set is checked by
   reading each document against the others.
8. **Section 10 step 1**, per the preamble above.
9. **Section 10 makes contracts a phase after the PRDs, and orders Specs before
    them besides.** Both wrong. A Spec is build instructions for one crate written
    against its PRD and every contract it is party to, so it cannot precede the
    contracts that define its obligations. The previous tree ran it that way and
    got contracts that smuggled in type declarations, because by the time they
    were written the code existed and the contract was documenting it rather than
    governing it.

    The deeper correction is that contracts are not a phase at all. The harness is
    the hub every crate connects to, so a crate PRD is largely about its seam with
    the harness, and the first question when adding any crate is how it connects
    there. **The contract is written with the PRD, as part of making the crate,
    not after the set is done.** A crate PRD written without its contract has no
    center to attach to and grows one of its own, which is what produced a
    `weaver-admin` carrying three charters and 3,802 lines of documentation in the
    previous tree. The corrected order is PRD-and-contract together per crate,
    then Specs, then code.
10. **Section 9 lists the reserved-slot prohibition in interface terms only:** seam,
    stub, reserved slot, dormant contract party. A reserved slot can be a **data
    field** just as easily. Recording embeddings in the trace when nothing
    retrieves by similarity, or carrying a payload field whose only consumer is
    unbuilt, is the same error in schema form, and the schema is where the rule
    will be tested next because it is the surface the ritual touches on every
    crate addition. The prohibition should name data as well as interfaces.
11. **Section 5.3 says a contract names "the types it uses."** That phrasing
    invites the same leak. A contract names the vocabulary that crosses the seam,
    its meaning, ordering, and failure modes. How a crate represents that
    internally belongs to its Spec.
12. **Section 5.3 does not require a vocabulary clause, and it must.** Every
    contract carries a clause naming the vocabulary it depends on, grouped by the
    crate that defines it, and **a contract without one is not a valid contract.**
    The clause is mandatory even when a group is empty: "this contract draws nothing
    from `weaver-types`" is an assertion someone checked, while a missing group is
    silence, and silence is what let the previous tree's attribute vocabulary drift
    until the declared names were smaller than the emitted ones.

    This is what makes the floor governable without a floor contract. A single
    document binding every crate is one nobody opens. A clause is checked at every
    seam, by the people writing that seam, while they are thinking about it. It also
    yields a mechanical check the previous tree never had: **the floor's required
    surface is the union of every contract's clause.** Anything in the floor named
    by no clause is unused. Anything named by a clause and absent from the floor is
    a gap. That is the party-list-against-dependency-graph check of section 5.3,
    applied to vocabulary rather than to emitters.

    The consequence for `weaver-traits-PRD` and `weaver-types-PRD` is that they are
    not crate catalogues. They are **requirement documents** stating the conditions
    of participation, enforced at every contract rather than admired in one place.
    It is also why the floor is written third rather than last: the harness
    establishes the center, the floor establishes the terms on which anything
    connects to it, and nothing written after can be correct without them.

**Document taxonomy, three kinds not two.** A PRD is what and why, per crate. A
contract is the protocol between parties, binding two or more crates and belonging
to none. A Spec is build instructions for one crate. Contracts are **not** named
with a `-Spec` suffix, because the absence says the document tells you nothing
about how to build anything. The previous tree named every contract
`*-contract-Spec.md` and its contracts duly filled with Rust type declarations.

**Contract naming carries the parties:** `weaver-<a>-<b>-contract.md`, the `weaver-`
prefix carried once, filed under `docs/crates/contracts/`. Naming by party rather than by
subject makes the binding legible at a glance and keeps the party list checkable
against the dependency graph. Under the hub model most contracts are bilateral,
harness plus one, so the harness contracts sort together and the hub is visible in
a directory listing. A contract needing three names is a signal worth examining
rather than a formatting problem. The previous tree's five-party trace contract
existed because every crate emitted into the trace directly, and sole-writer
collapses it to two.

Party naming also implies **one contract per crate pair**, and two contracts between
the same two parties is a smell. The previous tree carried `decode-serving-contract`
and `embedding-serving-contract` as separate agreements, both harness-to-SPU at their
core, split because encode and decode were treated as different kinds of thing. Once
encoding and decoding are one domain and the harness merely routes, they are one
seam and become `weaver-harness-spu-contract`.

A contract is not the same thing as a wire. The floor may hold contracts with no
socket between the parties: `weaver-types` owns the agent state file, which
`weaver-admin` writes and the harness reads, which is a producer-consumer agreement
over an artifact. The previous tree's `model-artifact-contract` is that shape and
says so, noting the two crates share no Cargo edge and the only coupling is the
on-disk artifact.

**`contracts/` sits inside `docs/crates/`, not beside it.** Contracts are the glue
binding the crates, so they live among them rather than one rank above. The absent
`weaver-` prefix is what marks the directory as not-a-crate, which makes the mirror
check mechanical: every `docs/crates/weaver-*` has a matching `crates/weaver-*` and
every crate has a directory, while `contracts/` excludes itself by its own name. The
previous tree filed contracts as a sibling of `crates/` on the grounds that a
contract is not a crate, which is true but separated the glue from what it binds and
still needed the exception written down rather than declared by naming.

## 2. Open rulings

**2.1 The encoder. DECIDED 2026-07-29, carries into `weaver-spu-PRD`.** Two
questions, both settled, and the one I had been asking was neither of them.

**Ownership.** Encoding and decoding are one domain and it is `weaver-spu`'s. The
encoder is not a harness component that happens to sit near the decoder, it is the
other half of the semantic processing unit, held to the same residency accounting,
GPU arbitration, and lifecycle confirmation. The harness routes tokens or embeddings
and holds neither. Stated in `weaver-harness-PRD` section 3, which retires the
previous tree's in-process embedder and the latency argument that justified it.

**Scope.** The encoder is **not in the stateless MVP.** An encoding is only useful
if something retrieves by similarity, and nothing in stage one retrieves, so the
embedder would produce vectors with no consumer. Writing them into the trace anyway
would be reserving a slot in data form. The embedder arrives with drey and a real
memory system, which is when something first needs it.

Ownership is not usage, so apex section 6 assigning `weaver-spu` encoder residency
stays correct. `weaver-spu-PRD` states the domain and states that stage one does not
build it, and adds no affordance: no trait, no variant, no feature flag, no config
field. A charter naming a domain is a decided boundary. An unbuilt interface waiting
to be filled is the thing apex section 9 forbids.

**2.2 Trace verbosity. DECIDED 2026-07-29.** Two levels named **floor** and
**ceiling**, and they are additive rather than exclusive. The floor is always
recorded and cannot be switched off. The ceiling is elected per agent in its state
file and adds to the floor.

The floor is derived rather than chosen: the harness reasons over the working
structure, so the events the turn needs to run are not elective. It is the turn
brackets, the message sequence, and enough of the tool events to carry results into
the next iteration. The ceiling adds the measurement payloads, the decode boundary,
and the residual reductions when readout is on.

Three consequences carry. The manifest records whether the ceiling was enabled, or
elected brevity and silent loss look identical to a reader, which defeats the
completeness status. Replay requires the ceiling, since the token identifiers and
sampler parameters it needs live in the measurement payload, so a floor-only session
has to be reproduced rather than diagnosed. And the setting is fixed for the life of
a load, because a session with a verbosity discontinuity is one every consumer then
has to reason about.

This is the ritual's first live exercise. `weaver-trace-PRD` defines the levels and
the manifest field, `weaver-harness-PRD` gains the enforcement line as sole writer,
and `weaver-types-PRD` holds the state-file field beside the residual-readout
toggle. Three documents, one act.

**Two rulings retired 2026-07-29, recorded because the pattern matters.** A
raw-trace policy and an observed-instance policy were both listed here as open. They
were not rulings. What an operator does with a delivered artifact is theirs, and
whether they take production down to observe an agent is an operational choice the
lifecycle already supports. Framing either as a framework decision put this program
in the position of dictating deployment. The framework promises the capability and
the artifact. It does not decide their disposition. Watch for the same inflation in
later passes, because it makes the open list look like design debt when it is
someone else's call.

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
  was mislabeled as resolving correction 1, and correction 1 is settled above by
  moving the responsibility, independent of how `weaver-admin` is packaged.
- **Descriptor acquisition.** `weaver-admin` opens the session's record and passes
  the descriptor to the worker over the coordination socket with `SCM_RIGHTS`, so
  the agent uid never resolves a trace path. The rotation half of this item is
  resolved and gone: one session is one record, runs append to it, and there is no
  second file to acquire mid-life.
- **A GID mask at lifecycle boundaries** gives `weaver-admin` read-only access while
  the agent runs and read-write after unload. Clean on a plain file. It is
  incompatible with a WAL database, since a WAL reader must write the shared-memory
  index, which is one more reason the durable artifact stays a file.

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
