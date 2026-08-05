# HANDOFF 2026-08-04, the HADES graph build

**Status:** OPEN. Work leaves this workspace for the server and does not return to it.
**Date filed:** 2026-08-04
**Base commit:** `main` at the commit that carries this document as read. The handoff
travels inside the corpus it describes, so the checkout target is wherever this file
was read from, and a pinned SHA would go stale the moment a later act revised this
document's own claims. First filed against `ad9d807`.
**Document ID:** `HANDOFF-2026-08-04-hades-graph-build`
**Parent:** `WeaverTools-Working-Process`, section 5
**Revised:** 2026-08-04, a third entry this date. Section 3 gains the fifth parser
rule, found live by the first mapper build: `tag` carries one vocabulary per record
kind, and a parser modeling it as seam-only reports every assertion in the corpus as
a defect. On the review seat's return before the merge, section 4's second trap
recasts to the past tense G4's closure left it in, and the rule's tally restates as
the relation rather than the count, the promotion pass in flight being about to move
the count.
**Revised:** 2026-08-04, a fourth entry this date, the reconciliation the
promotion pass owes as the second of the pair to land: the two remaining fixed
tallies recast to hold at any census, section 3's edge fraction anchored to its
filing and section 5's rebuild argument freed of its node count.
**Revised:** 2026-08-04, a second entry this date. G4's third half closes, the five
definitions gaining their statements at their definition sites, and section 0 gains
the map-not-build-order framing per the operator's ruling of this date: the graph
carries the roadmap as content, and coding is scoped against the graph after it
exists.
**Revised:** 2026-08-04, four corrections on the operator's review before this document
travels. The edge-source rule was stated as a two-way split and is a five-kind table.
The two external-boundary contracts' single party edge is named as correct rather than
left to look like a defect. The `via` guarantee is stated, which types it as a
reference. And checklist item 4 is connected to the G4 failure it cannot see, with the
authorship asymmetry that explains why.
**Editorial:** Per the Working Rules.

---

## 0. What this is, and why it is neither a batch nor a commission

`WeaverTools-Handoff-Format` governs two shapes. A **batch** carries edits to documents
that already exist and moves over a shared working tree with a stated base. A
**commission** asks the other seat to produce a document that does not exist yet. This
is a third thing: it asks another environment to **build an artifact from documents
that are already merged**, and the artifact is not a document.

Three properties of that format still hold and are honored here. The base commit is
stated, because it is the one mechanical fact that cannot be reconstructed from the
documents. Nothing is closed silently. And the receiving side is asked to check rather
than handed conclusions already applied.

What does not carry over is the shared tree. The graph is built on the server, this
workspace has no working MCP path to it, and that gap is why this document is longer
than a prompt would be: **it has to survive being read without the sender present.**

**The graph is built on the server and not here.** Nothing in this workspace stands one
up, `.hades/` does not exist here, and the corpus is the deliverable this hands over.

**The graph is a map of the documented set, not a build order, per the operator's
ruling of 2026-08-04.** The corpus carries roadmap - the deferred tool workflow, the
residual readout that follows loop 0, the memory leg's return path - and the graph
maps all of it as content. Deferred material is not a defect to trim and not a
commitment the first build answers for. **What the first coding run builds is scoped
against the graph after the graph exists**, which is the order the Working Process
already states and the reason the current task is the graph rather than the code.

---

## 1. The one thing that is not settled, and it blocks the build

**HADES's ingestion contract is undefined in this corpus.** The name appears exactly
twice: `WeaverTools-Working-Process` section 5 says "a HADES database is stood up from
the merged documents," and the vision mentions a Hades-backed retrieval tool. There is
no schema, no collection naming, no edge representation, and no idempotency rule
anywhere in the tree.

**So the first act on the server is to write that contract, not to write a mapper.** A
mapper written against a guessed target is a mapper rewritten. What follows in sections
2 through 4 is everything the corpus side can state without knowing the answer.

---

## 2. What the corpus holds, at `ad9d807`

```
nodes  290    assertion 240 | vocabulary 27 | crate 7 | document 7
              axiom 5 | artifact 3 | system 1

edges  425    asserts 241 | grounds 78 | draws 36 | defines 27 | party 12
              parent 7 | floor-link 7 | holds 6 | seam 5
              reads 2 | writes 2 | elects 2

227 fenced blocks across 22 documents. 290 distinct node identifiers.
```

Three properties were verified at this commit and each is a precondition the mapper may
rely on rather than defend against:

- **Identifiers are globally unique across every kind.** 290 nodes, 290 distinct ids,
  zero collisions. An identifier is a primary key without qualification by kind.
- **No edge endpoint is undeclared, in either direction.** Every `from` and every `to`
  resolves to a node the corpus declares.
- **Keys in use are exactly seven:** `edge`, `from`, `to`, `node`, `kind`, `tag`,
  `via`. `grounds` was retired as a key on 2026-08-03 and now names an edge kind only.
- **Every `via` value resolves to a declared `document` node.** All five do at this
  commit. `via` is a typed reference to a contract rather than a string annotation,
  and that is a schema decision rather than a convenience.

**Two contracts carry one party edge each and that is correct by design.** The party
count runs 2, 2, 2, 2, 2, **1, 1** - `weaver-admin-operator-contract` and
`weaver-gate-world-contract` each bind one crate and one principal outside the program,
and an external principal is not a crate node. Two conformance queries a receiving side
will reasonably write both report false defects against this:

- **every contract has two parties** fails on two of seven, correctly.
- **every contract is the `via` of a seam** finds five of seven, correctly. The two
  boundary contracts govern no crate-to-crate seam because their far side is outside
  the program.

This is the same class as the seam-identity trap in section 4. Both are shapes the
program means, and both look like defects from the query side.

## 3. The notation, and the five rules a parser will get wrong

`WeaverTools-Document-Format` is the authority, sections 3 through 6. A parser that
reads only the fences will still get these five wrong, so they are stated here.

**Records are blank-line separated inside one fence.** A ` ```graph ` block holds one or
more records. A record begins with `node` or with `edge` and never carries both. Unknown
keys are a defect rather than an extension, because the point of the fixed set is that
the mapper never guesses.

**Position is the pointer, and this is the rule with real work behind it.** No block
carries a section key. An assertion node "names and locates a clause," and **the
locating half is entirely the mapper's job** - it must derive each record's section from
where the block sits in the file. Document Format section 6 states this deliberately and
section 7 records that a key to point at a remote argument was retired for it.

**Cross-file edge targets are normal, not exceptional.** A Spec's `grounds` edge points
at an axiom the apex declares. `weaver-traits` carries an `asserts` edge to a node
`weaver-types-Spec` declares. Any resolver that scopes targets to the declaring file
will report roughly eighty false dangling edges.

**Five node kinds source edges, and a two-way rule will mis-model a third of them.**
An earlier draft of this handoff said edges run from the crate rather than from the
document. That is true of `asserts` and `grounds` and false of a third of the edge
population. The full table at this commit:

```
crate       asserts 241 | defines 27 | seam 5 | parent 7
            floor-link 7 | reads 2 | writes 2
document    draws 36 | party 12
assertion   grounds 78
artifact    holds 6
vocabulary  elects 2
```

**The real rule has two halves and they divide by document kind.** A **contract** gets
a `document` node and sources edges from itself, which is why all twelve `party` edges
and all thirty-six `draws` edges leave a document node. A **PRD or Spec** gets no node
and sources edges from the crate it argues, which is Document Format section 1's rule
that only a document which is itself the source of an edge needs a node record, and a
contract is the one kind that qualifies.

A mapper built on the flat form would conclude no document is ever an edge source and
mis-handle every edge a contract sources, 134 of the 425 at this document's filing
census and the same class at any later one.

**`tag` carries one vocabulary per record kind, and the record kind disambiguates.**
On a seam it is `socket` or `link`. On an assertion it is the enforcing instrument,
`compile-pin`, `compile-fail`, `perturbation`, `manifest`, or `review`, per Document
Format section 5, and every assertion in the corpus carries one. This rule earned its
place the way the others did not: the first mapper build modeled `tag` as seam-only
and reported all 240 assertions as defects in one run. The arithmetic that catches it
in advance is one relation rather than a count, the corpus's `tag` keys less its seam
edges equaling its assertion nodes, which any head can verify against itself and no
assertion pass can stale. The instrument tags are what checklist item 5 queries
against, so a parser that
drops them satisfies the structural half of the checklist while silently discarding
the build half.

## 4. What the graph has to answer

`WeaverTools-Working-Process` section 5 carries the closing checklist. **Closing it is
what ratifies the document set**, so these are acceptance criteria and not a wish list.

1. Built from the merged set, no hand edits.
2. Every crate a node, every seam present with its contract name and its
   socket-or-link tag.
3. Floor present as a layer, not as tree edges.
4. Conformance queries: one parent per crate, no lateral edges, every vocabulary name
   resolving to a definition site.
5. **A build question answered, not only a structural one.** Every crate's assertions
   present with their enforcing instrument, and a query naming a crate returning the
   claims that bind it.
6. The set-level record marks the set RATIFIED.
7. Old code removed from the workspace, confirmed gone.

Items 2 through 5 are satisfiable from the counts in section 2 today. **Items 6 and 7
need action beyond the graph, and item 7 is irreversible** - it deletes the archived
tree, which is what G6 exists to make safe.

**One trap for item 4.** Two `seam` edges between the same crate pair is correct, not a
duplicate: `weaver-spu` holds residency and decode under two contracts. **A seam's
identity is its `via` contract, not its endpoints**, per apex 5.5. A query treating a
crate pair as unique will report this program's intended shape as a defect.

**A second trap for item 4, and it is the one section 5's warning is about.** Item 4
asks that every vocabulary name resolve to a definition site. There are 27 vocabulary
nodes and 27 `defines` edges, so **that query returned clean through the whole span in
which G4's third half stood open.** G4's third half asks a different question the
checklist never asks: whether every definition is either drawn by some clause or stated
to be internal. Seven were drawn by no clause and five of those stated no reason, until
the act of 2026-08-04 closed the half, per section 6. **Item 4 passing was never
evidence that G4 held, and a receiving side reporting item 4 green would have said
nothing about the five for as long as they stood.**

**The structural reason is an authorship asymmetry, and it generalizes.** `defines` is
authored by the definer and `draws` is authored by the consumer. A completeness query
over `defines` can only ever measure whether definers did their job. **Orphan detection
lives entirely on the `draws` side, and no query written from the definition end can
see it.** This is the shape that makes dead-code detection hard in a compiler:
reachability is a property of the caller graph rather than of the declaration table. Any
query in this corpus that starts from a declaration and asks whether it is well-formed
will have the same blind spot, so the graph wants at least one query per relation
written from the consuming end.

## 5. Non-negotiables

**The graph is generated and never hand-edited.** If the graph is wrong the document is
wrong, and the fix is a phase one reopening followed by a rebuild. A hand-edited graph
is a second source of truth and drifts from the first, which is the failure phase two
exists to prevent.

**Rebuild rather than upsert.** "Generated from the documents" reads as
drop-and-rebuild. At this corpus's scale that is cheap at any census it will
reach, and fixing it now is cheaper than discovering an incremental path has
accumulated state no document accounts for.

**A clean automated result is evidence the gate did not fire.** The prior program's
graph returned zero code defects while accumulating 53 dangling edges of its own, and
its
`gate-check.py` returned zero findings on four consecutive PRs. Report what was checked,
not only that nothing failed.

## 6. What is owed on this side before the build is legitimate

Phase one closes when every crate PRD, every seam contract, and every Spec is merged
**and G4 and G6 hold across the whole set**. The first three are done. The gates are
not.

- **G4 holds, as of 2026-08-04.** Every drawn name resolves, every vocabulary node
  has a definition site, and every definition no clause draws now states its reason
  at its definition site: `weaver-types-PRD` section 2.1 covers the four config
fields and `weaver-traits-PRD` covers the mode vocabulary, with `tool-trait` already
stated and `provider-trait` gaining its statement in the same act that closed the rest.
- **G6 has not run.** Nothing the graph or code will need still lives only in the old
  tree. The three-lens quarry survey of 2026-08-03 is a substantial down payment.
- **G2 and G5 have not run** as phase-close sweeps.
- **G7 ran** and found one unlanded ruling, since landed.

**And one question is open that the graph cannot answer**: whether the set ratifies over
a document set that deliberately defers the tool workflow. It has been open since
2026-08-02. If the answer is no, the checklist cannot close, and the build is premature
rather than wrong.

## 7. Reading order for whoever picks this up

```
process/WeaverTools-Working-Process.md    section 5, the checklist that ratifies
process/WeaverTools-Document-Format.md    sections 3 to 6, the notation
docs/project/WeaverTools-PRD.md           section 5, the five invariants
docs/project/code-smells.md               section 1.2's note on seam identity
```

The corpus is 33 markdown documents and no code. `docs/project/open-items.md` is
gitignored and untracked by design, so it does not travel with a clone.

---

## 8. Asked of the receiving side

1. **Write the HADES ingestion contract first**, and send it back before a mapper is
   written. This side can then check it against the notation rather than discovering the
   mismatch at build time.
2. **Do not resolve a corpus defect on the server.** A mapper that works around a
   malformed record hides a document defect and breaks the never-hand-edited rule at one
   remove. Send defects back.
3. **Report the checklist item by item**, including what was checked and not only what
   passed.
