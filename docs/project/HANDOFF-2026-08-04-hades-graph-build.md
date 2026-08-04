# HANDOFF 2026-08-04, the HADES graph build

**Status:** OPEN. Work leaves this workspace for the server and does not return to it.
**Date filed:** 2026-08-04
**Base commit:** `ad9d807`, `main`
**Document ID:** `HANDOFF-2026-08-04-hades-graph-build`
**Parent:** `WeaverTools-Working-Process`, section 5
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
- **Keys in use are exactly seven:** `edge`, `from`, `to`, `node`, `kind`, `tag`, `via`.
  `grounds` was retired as a key on 2026-08-03 and now names an edge kind only.

## 3. The notation, and the four rules a parser will get wrong

`WeaverTools-Document-Format` is the authority, sections 3 through 6. A parser that
reads only the fences will still get these four wrong, so they are stated here.

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

**Edges run from the crate, not from the document.** `asserts` runs from a crate to an
assertion. `grounds` runs from an assertion to an axiom. This is why a Spec sources
records without needing a node of its own, and why Document Format section 1's
one-document-one-source rule was never amended for Specs.

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

## 5. Non-negotiables

**The graph is generated and never hand-edited.** If the graph is wrong the document is
wrong, and the fix is a phase one reopening followed by a rebuild. A hand-edited graph
is a second source of truth and drifts from the first, which is the failure phase two
exists to prevent.

**Rebuild rather than upsert.** "Generated from the documents" reads as
drop-and-rebuild. At 290 nodes that is cheap, and fixing it now is cheaper than
discovering an incremental path has accumulated state no document accounts for.

**A clean automated result is evidence the gate did not fire.** The prior program's
graph returned zero code defects while accumulating 53 dangling edges of its own, and
its
`gate-check.py` returned zero findings on four consecutive PRs. Report what was checked,
not only that nothing failed.

## 6. What is owed on this side before the build is legitimate

Phase one closes when every crate PRD, every seam contract, and every Spec is merged
**and G4 and G6 hold across the whole set**. The first three are done. The gates are
not.

- **G4 fails today, in its third half.** Every drawn name resolves and every vocabulary
  node has a definition site, both verified. But five definitions are drawn by no clause
  and state no internal reason: `permission-mode`, `permission-mode-vocabulary`,
  `residual-readout-election`, `tool-set`, `trace-sink`. `tool-trait` and
  `provider-trait` do carry statements. Five sentences, not yet written.
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
