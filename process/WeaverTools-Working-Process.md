# WeaverTools Working Process

**Version:** v0.10, 2026-08-01, third entry this date. The v0.10 change is section
7 once more, the durable-record cut moving from what remains to landed. The v0.9
change was section 7
again: the gate pair merged on the human's ruling, the two external-boundary
contracts arrived as blockers before any Spec, and the durable-record cut entered
what remains. The v0.8 change was section 7's post-merge redraw, the batch of
2026-07-31 moving from the working tree to merged and the gate pair standing as the
draft it then was. The v0.7 change redrew section 7's map to the tree
of 2026-07-31, three passes having merged against a map that still read the admin
pass as next, and corrected this header's pointer at that section from six to seven.
The v0.6 change was section 2's stub ruling and gate G7.
Companion to the Working Rules, the Document Format, and
the Handoff Format. The apex says what we are building. The Working Rules say how we
write. The Document Format says what shape a document takes. The Handoff Format says
what shape a batch takes when it moves between seats. This says who is primary, in what
order the work moves, and what must be true before it moves.

This document is the boot prompt for every fresh session on this project, in either
seat. Read it first. Section 7 says where the work currently sits on the map.

Phase three is unratified. That is deliberate and it is itself a gate. No crate code
is written until phase three is ratified.

## 1. Standing rules

These hold in every phase and are not restated inside the protocols.

Whoever holds the authoritative artifact is primary. The artifact that is
authoritative changes as the work moves, and the primary seat moves with it. The other
seat advises in and does not edit in place.

Two seats carry the work. The architecture seat holds theory, architecture, PRDs,
contracts, and specs. The implementation seat holds the codebase and the graph. A
session in either seat states which one it is in before it begins.

Neither seat settles a disagreement with the other. Both surface it to the human, who
is the adjudicator by definition and who sits in both seats.

Advice moves in one shape, in both directions. A return marks its content by type:
facts found, advice offered, and requests to reopen a closed decision. A return that
arrives with edits already made has skipped the gate rather than passed it.

A gate is a condition, not an intention. Every gate below is written so that it can be
checked by looking, and a gate that cannot be checked by looking is not a gate.

## 2. Document states

Three states, and the transition between each pair is an event with a gate.

**STUB.** A named slot with no content. It exists so a crate or a seam has a home and
a filename before it has a charter, and it is written when the thing it names is
chartered. A stub settles nothing, declares no graph records, and may not be cited by
any document as having decided anything. It is replaced wholesale rather than refined,
which is what separates it from a draft.

**Replaced wholesale means the stub leaves the tree.** The draft is written fresh at
the stub's name and the stub is deleted in the same act, a tracked deletion, so the
commit that cuts the draft is the commit that holds the stub's last state. Ruled
2026-07-31, reversing the preservation rule this section carried through v0.5, which
kept the file beside the draft under a `.stub` suffix. History is the archive and the
tree is not: a note that did not survive the drafting is findable at the commit that
retired it, and a tree that carries consumed stubs accumulates files no gate reads, no
mapper counts, and no pass owns. A stub accumulates what earlier passes learned about
the thing it names, because the work is chartered workflow by workflow and a workflow
runs through crates that have no charter yet, so the drafting reads the stub alongside
the old tree's code. Not every note survives that drafting and none of them binds,
since a stub decides nothing.

**Where a stub and the old tree conflict, the stub wins.** The stub is this program's
more recent statement of intent about that crate and the tree predates the split, so a
conflict is a question the project has already answered once. The precedence fires only
on conflict. Where a stub is silent and the tree speaks, nothing opposes the tree and it
flows into the draft unopposed, which is what step 3 review is for. Precedence handles
collision and review handles absorption, and neither substitutes for the other.

It is a state and not a member of the document set, so ratification does not wait on
it and the mapping does not read it. A stub that acquires a decision has stopped being
a stub and is a draft.

**DRAFT.** Cut from the old tree or newly begun. Carries no decisions and is not built
against.

**MERGED.** In `main` and declared the source of truth for now. Merged means the file
is in a working state that allows the work to move forward, and it means nothing more
than that. It is not a verdict on the contents. Individual files and individual lines
change after merge, and a correction to a merged document is an edit rather than a
ceremony.

**RATIFIED.** The whole document set has been written and mapped into the graph. The
mapping is the ratification. After it, a document does not change, and a change found
necessary during implementation is not a patch. Coding stops and the work re-enters
authoring.

Ratification is a property of the set and never of one file. No document ratifies
alone, so an individual status header tops out at MERGED and a single set-level record
carries ratification. A header that reads RATIFIED before the mapping has run is
asserting something the project has no mechanism to have produced.

**The project documents sit outside this model.** The Working Process, the Working
Rules, the Document Format, and the Handoff Format govern the set rather than belonging
to it, they do not map into the graph, and ratification is defined as the mapping. They
carry a version and a date and no state. A state on a document that cannot reach the
terminal state is a label that never resolves.

**A project document's `Parent:` header carries no edge.** The Document Format defines
that header against a `parent` edge and rules that the edge governs where the two
disagree. These documents do not map, so there is no edge to govern and nothing for the
header to disagree with. It is kept as a reader's convenience, naming which project
document a reader should have in hand first, and every project document parents to the
Working Process because this one is the boot prompt.

The word freeze is not used. It was doing the work of both merged and ratified, which
forced corrections to a merged document to queue behind a ceremony that does not apply
to them.

## 3. The map

Three phases in order. Each has a protocol and a closing gate. Nothing begins until
the phase before it has closed.

    Phase one   Authoring     PRDs, contracts, specs      architecture seat primary
    Phase two   Graph         knowledge graph built       implementation seat primary
    Phase three Coding        crates written and merged   implementation seat primary

## 4. Phase one, authoring

Produces three document types at three levels. The PRD says what a crate needs and
why. The contract says what crosses one seam, what it means, and how it fails. The
spec says how it is represented. No fourth kind, and no leak between levels.

Every document carries its nodes and edges in the notation the Document Format
defines. A document that states an edge only in prose has left work for the mapping,
and the mapping is the terminal gate.

The old tree is live during this phase and only during this phase. It is raw material
and never evidence. It supplies a starting draft and answers questions of fact about
what was built. It does not ratify anything, because this project was split off to
escape that context. A finding in the old tree is a candidate that returns for
ratification.

### Protocol

**Step 1, draft.** The implementation seat cuts a rough draft from the old code,
because that is where the raw material lives. A draft is marked as drawn from the old
tree and carries no decisions.

**Step 2, author.** The architecture seat writes the document. This is the step where
staleness is caught, because everything inherited from the draft is re-derived here or
dropped.

**Step 3, advise.** The document returns to the implementation seat for review against
the code it can see, in the return shape named in section 1.

**Step 4, merge.** The human rules. The document merges on his call and the
implementation seat applies the merge without reopening it. Merge is not ratification
and confers none of its finality.

Specs are written last, after every PRD and contract in the set is merged, because a
spec is a traversal of a settled document set. Contracts are written with their PRDs
and not as a later pass.

### Gates

**G1, mechanical.** Editorial rules hold. ASCII only, no em-dashes, no semicolons,
none of the forbidden words, line lengths in corpus range, no swallowed headings, no
double punctuation, no visual collisions. The forbidden-word list and the line width
live in the Working Rules section 1.

**G2, level.** Nothing in the document belongs to a different level. A PRD carrying
protocol, a contract carrying representation, or a spec carrying rationale fails this
gate, and the material relocates rather than being trimmed.

**G3, graph facts.** The crate has exactly one parent edge, naming its domain parent:
the domain root for a member crate, and `WeaverTools` for a domain root. It carries no
contract and no tag, because nothing is asked across it. It is domain membership rather
than containment, and the two reach the same mechanical shape without being the same
claim, so the word is checked here rather than assumed. Every seam names the contract
that governs it and is tagged socket or link, with the grounds for that tag stated. No
lateral edge to a sibling appears. Floor links are declared as floor links and are not
confused with the parent edge. Checked against the blocks the Document Format defines,
where the containing section is the grounds.

**G4, vocabulary.** Every name a contract's vocabulary clause draws from another crate
resolves to a definition that exists in that crate. Every definition a crate holds is
either named by some clause or stated to be internal. Where a clause and the floor
disagree, the document names which side yields and why. This gate is meaningless
against a partial contract set and is run at phase close, over the whole set.

**G5, duplication authority.** Where the same fact is stated in two documents on
purpose, one is named authoritative, and divergence is a defect to file rather than
something a reader resolves by picking.

**G6, extraction complete.** Nothing the graph or the code will need still lives only
in the old tree. This is the gate that makes the deletion in phase two safe.

**G7, rulings landed.** A ruling names the documents it changes, and this gate checks
that each named document carries the change. It is in force because the first live
ruling in this corpus named four documents and landed in none of them, and nothing
detected that until a re-review opened for other reasons. A ruling recorded in a working
list reads as settled to every later reader, so an unlanded ruling is worse than an open
one. It is checkable by looking, since the ruling names the documents and the documents
either carry the change or do not. Where a ruling is landed in part on purpose, the
documents still owed are named as owed rather than left to be noticed.

Phase one closes when every crate in scope has a merged PRD, every seam has a merged
contract, every spec is merged, and G4 and G6 hold across the whole set. Only seams
take contracts. The other edge kinds are structure and carry none.

## 5. Phase two, graph

Produces the knowledge graph as a standing artifact, and performs ratification. A
HADES database is stood up from the merged documents, which are already structured to
graph cleanly, with the edges and vocabulary present.

The graph is generated from the documents and is never hand-edited. If the graph is
wrong the document is wrong, and the fix is a phase one reopening for that piece
followed by a rebuild. A hand-edited graph is a second source of truth and drifts from
the first, which is the failure this phase exists to prevent.

The graph is what code is checked against in phase three. Prose does not answer a
conformance query and a graph does.

### Closing checklist

Phase two closes on a checklist, each item verifiable by looking. Closing it is what
ratifies the set.

1. Graph built from the merged document set, with no hand edits.
2. Every crate present as a node, every seam present with its contract name and its
   socket-or-link tag.
3. Floor layer present as a layer and not as tree edges.
4. Conformance queries answered: one parent per crate, no lateral edges, every
   vocabulary name resolving to a definition site.
5. The set-level record marks the document set RATIFIED.
6. Old code removed from the workspace, confirmed gone.

Item 6 is last for a reason. The old tree is still legitimately reachable through
phase one drafting and through fact checks during authoring. It stops being reachable
the moment everything it had to offer has been extracted into artifacts this project
trusts, which is what G6 certifies and what the built graph demonstrates. After item 6
the only sources are the documents, the graph, and the specs. Fresh code has nothing to
copy from, by accident or otherwise. The test is not whether the coding seat intends to
avoid the old tree. The test is whether it can reach it, and the answer must be no.

## 6. Phase three, coding

This section is a draft of candidates. Nothing in it is in force.

**Entry gate.** No crate code is written until this section is ratified and phase two
has closed. A gate invented while looking at a diff is a gate shaped by that diff.

Candidates, offered for ratification:

**H1, authorization.** No code without a ratified spec. Behavior in a diff that traces
to no spec clause is out of scope and returns to phase one rather than being argued at
review.

**H2, dependency conformance.** The crate's Cargo dependency list matches its position
in the graph. Every Cargo edge is a declared `floor-link` or a `seam` tagged `link`,
since the Document Format rules that a pair governed by a contract is a seam and never
also a floor link. No dependency on a sibling. The parent edge is domain membership and
appears in no Cargo file, since nesting carries domain rather than dependency. Checked
against the graph, mechanically.

**H3, seam conformance.** The seam is exercised against the contract's failure cases
and not only its success path. A contract that names a refusal and a build that cannot
produce it has not implemented the contract.

**H4, vocabulary conformance.** The types and traits used across a seam are the ones
the contract's vocabulary clause names, at the definition site the clause names, not a
local redefinition of the same shape.

**H5, advisory pass.** The architecture seat reviews the diff against the PRD, the
contract, the spec, and the graph, and returns advice in the standard shape. The
implementation seat holds the merge call and answers the advice in its decision.

Open cells: whether H2 runs as a build script or as review, what the mechanical bar is
under H1 for tests and lints, and whether a failing H3 case blocks merge or files as a
known gap with a named owner.

## 7. Current position

Phase one. Seven charters are merged: `weaver-harness` with `weaver-trace` as its
member, `weaver-traits` and `weaver-types` as the floor, `weaver-admin`,
`weaver-spu`, half chartered by ruling to the end of the lifecycle workflow, and
`weaver-gate`, chartered to the same boundary. Six contracts are merged beside them,
coordination, trace production, residency, the gate seam, and the two external
boundaries, `weaver-admin-operator-contract` and `weaver-gate-world-contract`, with
`weaver-organ-channel` as the drawn material the organ contracts share.

The batch of 2026-07-31 merged to `main` on 2026-08-01: the stop exchange and the
close kind, the subtraction rulings retiring the live view, the integrity witness,
and admin's device arbitration, the apex's conflict binding relocated to SPU
admission, and the `basic-inference-loop` workflow pair under the harness's
`Loops/` directory. The `weaver-gate` pair merged on 2026-08-01 on the human's
ruling that a document on `main` is merged, and the external boundaries took their
contracts the same day, `weaver-admin-operator-contract` and
`weaver-gate-world-contract`, written as blockers before any Spec.

The durable-record cut of 2026-08-01 landed the same day: the program owns no
record, per `weaver-admin-operator-contract` section 3, and the trace pair, the
coordination contract, the admin and harness charters, the apex, and the loop
document all restate against the stream, the enter question left as the cell
`weaver-admin-PRD` section 10 holds.

What remains in this phase: the fault-carrier ruling, which holds the alert
rerouting and the bracket semantic, the apex re-authoring, whose waiting condition,
all seven charters, is now met, and then the Specs, written last against the
settled set.

Nothing ratifies until the whole set maps, so no milestone here is a ratification.
Merged means the work may move forward, which is what merged is for.

## 8. What this document does not do

It does not govern the content of any crate. It governs which seat is primary, in what
order the work moves, and what must be true before it moves. A rule that constrains
what a crate does rather than how it comes to exist belongs in the apex or in a
charter.

It does not say what shape a document takes. That is the Document Format's.
