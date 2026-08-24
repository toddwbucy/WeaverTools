# Open items

**Status:** ARCHIVED 2026-08-24. Nine items, verified one by one on that date: four
were done, two had stopped being relevant, and three were live and became issues #309,
#310, and #311. Those issues carry their own evidence rather than citing this file,
because a working list that was never ratified is not an authority to cite.

**It was tracked briefly before it was untracked, and an earlier reading of this
header said it never was.** It entered the tree 2026-07-28 at `594f48a`, took two
edits, and left at `49e9c99` on 2026-07-29. Those four commits are its history, so the
pre-deletion copy is recoverable and citable as provenance where one is wanted. The
untracking was deliberate and its reason is below.

Previously: **Status:** WORKING LIST. Not a member of the document set and never
ratified, and
untracked as of 2026-07-29 for that reason. Items enter when a pass surfaces something
it cannot settle, and they leave when a document settles them. An item that resolves is
deleted from here and lives in the document that resolved it, so this file shrinks as
the corpus grows.

**Sections are branches.** Reorganized 2026-07-30 to match the Handoff Format, which
sends one branch per batch and asks that an item left open name the branch it went to.
Naming it per item would restate on every line what the section already says, so the
section carries it and an item's position is its destination. An item with no branch
lives in the last two sections and says why.

**Item numbers are positions in a shrinking list and are not identifiers.** They
change whenever an item above them is deleted, so nothing outside this file cites one
and no return between seats refers to one. Name the substance instead.

**Date started:** 2026-07-28
**Editorial:** Per the Working Rules.

---
## 1. `docs/first-loop`, the delivery in hand

Review of the corrected batch: the Format re-cut, the coordination contract, the
three charters and the trace contract, and the loop pair at v0.4. All six prior
items resolve, several better than asked. The Format's v0.6 carries all five entries
with a header that accounts for them, the document-kind entry now reconciles both
drawn material and workflow documents with the Working Process's three kinds in one
argument, `query-event-frame` is declared at its definition site, admin opens four
with both seam tables current, the descriptor route is settled with the mechanism
named and the section 10 cell scoped to the channel's own end, the live-view
non-blocking guarantee reached the derived list, the conformance line landed, and
the loop document is G1 clean with its registers citing by substance. What remains
is one filed fact.

1. **The never-landed account and the repository disagree, and the repository is
   reachable.** Fact found, filed once and not argued, because the human has ruled
   and the corpus outcome is identical under either account: v0.6 carries all five
   entries and every consumer resolves. The Format's header and the loop document's
   register now record that the four rulings were made and never landed. `git show
   c1608c9` shows all four present in the Format that merged at that commit, the
   suffix sentence, the container gloss, the drawn-material entry, and the scoped
   declaration rule, and that commit's own message names the last of them. What
   rides on the account is not the corpus but the evidence: if the rulings never
   landed, the base-behind-tree failure of section 4 of this list fired once rather
   than twice, and the two declined Handoff Format clauses lose their second
   witness. Named because the account is checkable by looking, which is this
   corpus's own test, and the ruling is the human's to keep or revisit with the
   fact in hand.


## 2. `docs/corpus-corrections`

**A note dropped in a stub is context for a later draft, not an edit owed now.** The
workflow being chartered runs through crates that have no charter yet, so a pass that
learns something about one of them deposits it in that crate's stub. The stub carries it
into the drafting of that crate's PRD, alongside the old tree's code. Not every note
survives that drafting and it is not meant to. It is context rather than a ruling, and
nothing in the corpus may cite a stub as having decided anything.

So a defect found in a stub is reported when it is seen and worked when that crate comes
up. It does not gate the crate in hand. The same holds for a correction owed to a
document that merged in an earlier pass. **Depositing a note the current pass owes is
different and is not here**, because that is the current pass's own work and its
register tracks it.

This branch will edit documents the crate branches also edit, `weaver-harness-PRD`
above all. That is a conflict waiting rather than a problem now, and the cheap answer is
to land corrections after a crate branch rather than beside it.

1. **The `crates/` mirror does not survive a clone.** The Document Format makes the
   mirror check mechanical on every `docs/crates/weaver-*` having a matching
   `crates/weaver-*` at the same depth. Those directories are empty, git tracks nothing
   under them, and git does not record empty directories. The check passes on the
   machine that made them and fails for anyone who clones, which is the reader the check
   exists for.

   A placeholder file in each is the cheap fix and it commits nothing but the shape the
   apex already names. Accepting that the check holds only once code exists is the other
   answer, and it means the mirror is unenforceable through the whole of phase one and
   two. It sits here rather than against the process because the Document Format is
   right and the repository is what does not match it.

2. **The `tool-trait` block left this list on 2026-08-01,** landing in
   `weaver-traits-PRD` section 3.1 as a ruling of that charter, per the review
   seat's finding that a Spec was citing this untracked list as authority for a
   structural decision. The entry below is struck rather than deleted so a reader
   of an earlier copy can tell a relocated ruling from one withdrawn.

   ~~**`tool-trait` is drawn by no vocabulary clause and is blocked rather than cheap.**
   G4 makes the union of every clause the floor's required surface, and a definition no
   clause names unused. `tool-trait` is the one floor definition with no candidate
   consumer, because tool dispatch is harness-internal and no seam crosses it.~~

   ~~Filed earlier as cheap to settle while the remaining charters were written, and
   reclassified 2026-07-29. Tool-call protocol depends on the workflow and the workflow
   depends on organs that do not exist, so this waits on `weaver-spu` and
   `weaver-admin`. Do not settle it and do not open it. It cannot fail before phase
   close by G4's own terms.~~

3. **The two external-boundary contracts declare no `draws` edges at all.**
   Found by the audit the gate act's review provoked, 2026-08-02.
   `weaver-admin-operator-contract` draws `peer-identity`,
   `authorization-predicate`, and loop 0's directive, answer, and refusal, and
   `weaver-gate-world-contract` draws `peer-identity`,
   `authorization-predicate`, and `gate-instruction`, and both declare only
   their node and party edges. Document Format section 4 makes `draws` the
   vocabulary clause in edge form and the thing that makes G4 a query rather
   than a reading, so both clauses are currently invisible to the gate that
   exists to check them. Neither document is open in the act that found this,
   which is why it is filed rather than swept: the two blocks are eight edges
   between them and belong to a corrections branch. `weaver-organ-channel` is
   not among them and is correct as it stands, declaring no records
   deliberately per its section 0.

## 3. `docs/apex-prd`

The corrections to `weaver-agents-PRD` were collected here and resolved in one
re-authoring after all seven crate PRDs merged, per the apex's own section 0 rule
that the set is written together. The re-authoring landed 2026-08-01 and every
item left with it, the naming and taxonomy notes this section once carried having
already landed in the Document Format. One editorial question was carried out of
the list and resolved on review: the stage's name corrects from stateless to
proto-stateful, per the human's ruling of 2026-08-01, and the rename swept the
corpus in the re-authoring act.

## 4. Held for the Spec pass, no branch cut yet

1. **Security Specs are threat-anchored, per the external review of 2026-08-01.**
   The corpus states the custody mechanisms, close-on-exec at the receive, the
   dumpable flag, `SCM_RIGHTS` passing, possession against credential, and names
   their adversaries across four documents, and a reader assembles the threat
   story alone. The review's accepted residue: every security mechanism's Spec
   names its adversary, walks the attack the mechanism defeats, a rogue elected
   tool reaching for the trace being the reference scenario, and derives its
   perturbation test of apex section 11 from that walk, so the test is the
   scenario made executable. The review's other recommendations were already
   delivered, the fan-out map by `load-unload-loop` and the definition unification
   by the re-authoring and the rename, and its analogies stay out of charters per
   the corpus register, the vision being the one licensed home.

2. **The config's sink field and the never-told-the-name sentence, surfaced by the
   harness Spec pass.** `weaver-types-Spec` section 2 has `trace-sink` as a config
   field naming the sink, and `weaver-harness-PRD` section 5 has the agent never
   told the name. No chartered workflow exercises the harness's config read today,
   so the harness Spec defers the read to the tool workflow and names this as what
   that pass must answer: whether the read drops the field unretained, or the
   never-told sentence rescopes to the descriptor mechanism, the kernel's
   search-bit lock standing between knowledge and reach either way. Recorded at
   `weaver-harness-Spec` section 9, filed here so the tension is visible before
   that pass opens.

## 5. The composability batch's residue, held for the token workflow

The batch of 2026-08-02 merged in five PRs on this date, the vision's four
sections, the replayability correction, the charter rescope, the decoder cut,
and the harness Spec, and its landed items left this list with it. Three items
remain, all waiting on passes not yet open.

1. **The disposition mechanism's spec work.** `Disposition<T>`, `Frozen(value)`
   or `OperatorTunable` per knob, elected at the composition root. The principle
   is recorded at the vision's section 10 and the mechanics are specced in the
   SPU round with the knobs arriving via the token workflow. Two invariants
   pinned regardless: the trace records effective values whichever side set
   them, and a config setting a frozen knob refuses the load loudly.

2. **The loop binding election, per the operator, 2026-08-05.** The loop seat
   has one granted surface and two bindings, and which one runs is the
   operator's choice rather than the program's. **Both bindings are for loop 1
   and above and neither reaches loop 0**, per the ruling of the same date
   recorded at `weaver-harness-PRD` section 2: loop 0 is the running agent
   service, so it is what a dropped-in loop arrives inside of rather than
   something a builder supplies. Compiled in-process is one
   binding, per the charter rescope of 2026-08-02 and harness Spec section 6
   as they stand. The second is a socket binding: a compiled proxy loop whose
   body speaks the granted surface over a Unix socket to an external process,
   so a builder develops loop logic in Python against the socket and ships a
   Rust twin compiled in, or runs the socket binding in production and pays
   the hop. Neither binding is a development stage the program hard-codes -
   the disposition principle above, applied to the loop seat. Four invariants
   pinned regardless, all existing machinery: the blade holds transitively,
   the proxy composing only the granted surface so the far side can reach no
   more; the seam takes a contract before the socket exists, per apex 5.3;
   the socket authenticates by `SO_PEERCRED` against the floor's one
   predicate, a drop-in loop being an admitted principal; and the loop
   registers what it needs, per the builder-extension ruling. Sequenced after
   loop 0, as its own chartered act with its contract. The harness act builds
   the seat as Specced and forecloses nothing here.

3. **What starts the admin service, and the two halves of assuming the role.**
   Filed 2026-08-05 after the role ruling landed, because the ruling named the
   role, the user, and the crate and left unnamed the thing that runs the crate
   as the user across human sessions. The charter's only sentence near it is a
   subordinate clause inside the delegation argument, offering "admin running as
   a system service the operator installs" as one of two ways the worker
   authority could be delegated, which is not a statement about what starts
   admin.

   **The operator assumes `weaver-admin-role` the way a root-level user manages
   any service**, per the operator's framing of this date: membership in
   `weaver-admin-user`'s group, or root. **Root access to start and stop the
   agent is the correct approach rather than a limitation to design around** -
   the ruling of this date, recorded here so the next pass does not read the
   requirement as friction and try to remove it.

   The gap is that assumption has **two halves and the corpus writes down one**.
   Group membership carries the operator surface, whose access rule already
   allows that group and whose `SO_PEERCRED` predicate already checks it, so
   that half needs nothing. Managing the service itself is the other half, and
   group membership alone does not carry it: a system unit's start and stop want
   root or a policy rule scoping those verbs to the group. A reader following
   the charter today provisions the group, finds the service will not start, and
   derives the second half unaided.

   **Two units, not one, and the corpus distinguishes neither.** The admin
   service's own unit and the per-agent worker unit do different jobs for
   different principals, and the conflation is what made the delegation question
   circle on 2026-08-05 before the role vocabulary separated the layers. Whoever
   opens the admin charter next states both.

   Adjacent to section 10's reopened descriptor cell rather than inside it: what
   starts admin and how a descriptor reaches the worker are different questions
   that happen to sit in the same act's neighbourhood.


## 6. No branch cut, filed against the process rather than the corpus

Two suggestions offered against `WeaverTools-Handoff-Format.md` and not taken in v0.3.
The process documents merged at `010240d`, which changes nothing about these: a
correction to a merged document is an edit rather than a ceremony, per Working Process
section 2. The two suggestions are the architecture seat's to decline.

**A batch names its base and nothing says what happens when the base is behind the
tree.** Handoff Format section 2 puts the base commit in the manifest so the receiving
seat can tell which prior state a file is a change against. It fired on 2026-07-30 and
the format had no rule for the result. A batch arrived a second time, byte-identical
prompt and the same `Base 289e92d`, against a tree that had moved twice since: `main`
was at `010240d` for the process merge, and two documents in the batch carried repairs
made in the review of the first send. Rewriting the files from the stated base reverted
both repairs.

**The gap is on both ends and neither is the base rule.** The sending seat has no
instruction to re-read a document before resending it, so a second send from an
unchanged local copy silently discards whatever the review landed. The receiving seat
has no instruction for a base that is behind, so nothing says whether to review the
batch as sent, restore first and review the difference, or refuse it. Recovery is only
cheap here because the losses were two paragraphs a reviewer had written and could
restore from its own record. A batch of twenty edits against a base three commits back
would not be recoverable by reading.

**Two clauses, one per end.** A batch whose base is not the tip states what moved since
and why the difference is safe, which puts the check on the seat that can see its own
local copy. And a resend is a new batch with a new base rather than the same batch sent
twice, because a batch that reverts a landed repair has undone the check it was asking
for.

**The manifest asks the sending seat to report a gate result the receiving seat would
otherwise run.** Section 4's manifest carries "Gates run and their result", and a
reported pass is a claim rather than a check. This program's own position is that a
clean automated gate is evidence the gate did not fire and not evidence of correctness,
which is the argument that retired the conformance graph. Reporting what was run is
worth having, and it reads as delegating the check unless the format says the receiving
seat re-runs rather than reads. One clause.

**The base commit sits in the prose and in the manifest.** Section 4 has paragraph one
naming the base commit and the manifest opening with `Base <commit>`, against that
section's own split where the prose carries the argument and the manifest carries the
accounting. A bare commit hash is accounting. Purely editorial.

