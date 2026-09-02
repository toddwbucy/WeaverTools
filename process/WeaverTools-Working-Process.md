# WeaverTools Working Process

**Version:** v0.31, 2026-09-02. Companion to the Working Rules, the Document
Format, and the Handoff Format. The apex says what we are building. The Working
Rules say how we write. The Document Format says what shape a document takes. The
Handoff Format says what shape a batch takes when it moves between seats. This says
who is primary, in what order the work moves, and what must be true before it
moves.

This document is the boot prompt for every fresh session on this project, in either
seat. Read it first. Section 7 says where the work currently sits on the map.

Phase three is ratified, 2026-08-04, gates H1 through H5 in force per section 6.
Code merges against them and against nothing invented at review time.

## 1. Standing rules

These hold in every phase and are not restated inside the protocols.

Whoever holds the authoritative artifact is primary. The artifact that is
authoritative changes as the work moves, and the primary seat moves with it. The other
seat advises in and does not edit in place.

Two seats carry the work, and their assignment changed on the human's ruling of
2026-08-01. The authoring seat is the session that holds the working tree: it
drafts and lands PRDs, contracts, specs, and their edits, because grounding in
the files and the corpus's cross-document state proved decisive through phase
one. The review seat is the remote session: it consults, reviews uploaded
snapshots at a distance, and returns findings in the standard shape below, which
is the defect-finding a fresh context does best. The earlier assignment, the
architecture seat authoring and the implementation seat landing, is history the
changelogs of this date record. The codebase and the graph stay with the seat
that holds the tree. A session in either seat states which one it is in before
it begins.

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

**A header states the state its act lands it in**, per the operator's ruling of
2026-08-04: a branch cut to merge carries MERGED, made true at the merge, rather
than flipping in a follow-up act, and a header never outlives its state. The sweep
of this date flipped the merged set's DRAFT headers, which had been wrong in the
direction this rule closes, and the earlier finding against a branch pre-asserting
MERGED resolves the other way: pre-asserting the post-merge state is the practice,
and the defect would be merging without the human's call, not the header. The
boundary of the flip is doctrinal rather than enumerated: every member header read
MERGED alone at that sweep, ratification being represented only at the set level
under the rule of the time, and the same sweep retired the source-of-truth "for now"
from every member header, ratification having ended the provisionality the phrase
carried. **The set-level-only half of that sentence is retired**, per the ruling of
2026-08-23 recorded below.

**RATIFIED.** The document conforms to the pattern the set-wide act of 2026-08-04
established, and clearing its gates is how it shows that. After ratification a
document does not change, and a change found necessary during implementation is not a
patch. Coding stops and the work re-enters authoring.

**A charter ratifies on its own**, per the operator's ruling of 2026-08-23. A crate
that has been chartered and has cleared its gates is ratified, and **the set is
whatever the charters currently say** rather than a snapshot of one date. A member
header may read RATIFIED.

**The set-wide act of 2026-08-04 was a requirement of its moment rather than a
standing obligation.** Nothing existed then to be consistent with, so consistency had
to be established across the whole corpus at once. That act built the skeleton every
later charter is built on. It is not a ceremony to repeat whenever a charter lands.

**The rule this replaces was scaffolding, and it is recorded as such rather than
quietly dropped.** It read that ratification is a property of the set and never of
one file, that no document ratifies alone, and that a member header tops out at
MERGED. That was correct while the pattern was being established, because a document
ratifying alone before a pattern existed would have been ratifying against nothing.
The pattern is established and the gates carry what the ceremony carried, so the rule
is retired rather than softened.

**Which gates a document clears alone, and which it cannot.** G1, G2, and G3 are
document-scoped and a charter clears them by itself. G4 has two halves: the draw
side, that every name a vocabulary clause draws resolves to a definition that exists,
is checkable per document and is cleared here. **The definition side, that every
definition is named by some clause or stated to be internal, cannot be checked from
one document, and neither can G5 or G6.** To know a definition is unused, or a fact
stated twice on purpose, you must read the set, and nothing the document declares
bounds that search. Those two run at the release inventory, and until they do a
per-charter ratification is a claim about the document rather than about the set's
coherence.

**G7 is not in that class and is grouped apart.** It spans more than one document, but
**a ruling names the documents it changes, so the list to check is declared and
finite.** That is a volume problem rather than a scope one, and it runs with the
ruling rather than at any close. **The graph's set-level mark is unchanged by this and
continues to record
the founding act**, the question of whether a ratified charter carries its own tag
being the Document Format's rather than this document's.

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

**Transport silence is part of this gate as of 2026-08-24.** A contract states what
crosses, what it means, and how it fails. **It does not name the mechanism that
carries it.** A contract naming a filesystem path, a descriptor, a socket type, a
flag, or a latency the substrate happens to provide has taken a Spec's material into
a contract, which is this gate's own case read one level finer.

**The failure mode is vocabulary rather than subject, which is why it needs stating
separately.** A custody obligation genuinely belongs in a contract, so the gate reads
clean on the question it was written to ask and the defect passes. What is at the
wrong level is the wording: **"sets close-on-exec at the fork" names a flag where
"does not permit a child process to inherit the handle" names the obligation.** The
obligation survives a change of substrate. The flag does not.

**The test is mechanical.** Read the clause with the substrate removed. If nothing is
left to require, the clause was representation and relocates. If the requirement
stands and only the noun goes, the noun is rewritten and the clause stays. **This is
what makes an organ relocatable in principle**, and a contract that fails it has
decided the substrate on behalf of every later deployment.

**The rule is the contract's alone, and the other two levels are its opposite.**
Substrate belongs in a PRD, which decides it: a charter ruling that this program
opens no network surface, or that a seam is local, is a decision about substrate and
is that document's to make. **Substrate is what a Spec is about.** A Spec naming no
mechanism would have elected no representation, which is the whole of its job. Only
the contract is silent, and it is silent for one reason: **so that what stands on
either side of a seam can move without the page between them changing.**

**Reading this as a corpus-wide ban is the available mistake and it was made before
the rule was a day old.** A pass proposed stripping mechanisms from thirty-seven PRD
sites, which would have taken decisions out of the documents that make them and left
charters unable to say what they had ruled. The rule names contracts and means
contracts.

**Rationale is developed in the PRD and may be restated in the Spec,** per the
human's ruling of 2026-08-01, which is what the spec clause above means and what
it failed to say. A Spec elects a representation, and an election with no stated
ground cannot be reviewed, so the Spec names the ground it answers to. What it may
not do is develop that ground: a criterion argued first in a Spec has been settled
outside the context that governs it, which is how a PRD and its Spec drift into
saying different things about the same crate. The test is mechanical. Where a
Spec's reasoning traces to a charter clause it is restatement and passes. Where it
does not, the criterion lands in the PRD and the Spec cites it, in the same act
where that is practical and named as owed where it is not. This is the alignment the
colocation rule exists for, one PRD and one Spec in one crate directory. **A Spec
merging ahead of the charter clause it cites names that clause as owed**, which is
the case the register exists for rather than a case the gate refuses.

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
disagree, the document names which side yields and why.

**The gate has two halves and they run on different occasions**, per the per-charter
ruling of 2026-08-23, which removed the phase close that used to carry both. The draw
side, that every name drawn resolves, is checkable against one document and runs when
that document lands, and again over every document drawing from a crate whose
definitions an act moves. **The floor's ritual already requires that second act to
update every affected PRD and contract, so this is the check that the ritual was
carried rather than a separate sweep.** The definition side, that every definition is
drawn or declared internal, is meaningless against a partial set and runs at the
release inventory, over the whole set.

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

**That naming is the standing rule for every change, not only for rulings, as of
2026-08-23.** A change names every document it affects. What lands with it, lands.
What does not is named as owed, in the register that tracks it, and leaves that
register when it lands.

**Carrying a whole change in one act is no longer required, and the requirement is
recorded rather than deleted.** Several documents said a change that could not be
carried in one act had not been thought through. That was right while the set was
being established, when nothing existed to be consistent with and a partial change
would have left documents encoding different understandings of the same system with
no register to catch it. The registers exist now, G7 checks them, and demanding
simultaneity of a corpus this size buys nothing the register does not already buy.
**What is required is that nothing a change touches goes unnamed.**

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
5. **A build question answered, not only a structural one.** Every crate's
   assertions present as nodes with their enforcing instrument, and a query
   naming a crate returning the claims that bind it, per Document Format
   sections 3 and 4. This item exists because the items above it are all
   satisfiable by a graph built from charters and contracts alone, which
   carries nothing from any Spec, so the checklist could close over a graph
   that is complete by its own terms and cannot serve the phase it exists to
   enable. Phase three reads the graph, and a graph that answers only where a
   crate sits tells a coder nothing about what to build.
6. The set-level record marks the document set RATIFIED.
7. Old code removed from the workspace, confirmed gone.

Item 7 is last for a reason. The old tree is still legitimately reachable through
phase one drafting and through fact checks during authoring. It stops being reachable
the moment everything it had to offer has been extracted into artifacts this project
trusts, which is what G6 certifies and what the built graph demonstrates. After item 7
the only sources are the documents, the graph, and the specs. Fresh code has nothing to
copy from, by accident or otherwise. The test is not whether the coding seat intends to
avoid the old tree. The test is whether it can reach it, and the answer must be no.

### The ratification of 2026-08-04

The operator ruled on 2026-08-04 that the set is ratified, and the ruling answers
the question open since 2026-08-02: the set ratifies as the complete document set
for the toolless inference deliverable, and the tool workflow's later arrival is a
planned re-entry to authoring rather than a defect. The half-chartered discipline
anticipated this - crates are chartered workflow by workflow and two charters say
so on their faces - so a re-entry adds a workflow to a settled set rather than
reopening the set's meaning.

The graph was built on the HADES server the same day, per
`HANDOFF-2026-08-04-hades-graph-build`, and the checklist was reported item by
item, with item 1 owing its drop-and-rebuild audit trail. Item 6 is carried by the
apex: section 0's system record bears `tag: ratified`, so the set-level mark is
generated from a document and the never-hand-edited rule reaches the mark itself.
The mark lands in the graph on the next rebuild. Item 7 stands open and is the one
item that outlives ratification: it certifies workspace hygiene rather than the
set's coherence, it waits on G6, and the quarry's deletion being irreversible is
the reason it is not hurried.

For section 6's entry gate, phase two's close reads as items 1 through 6, per the
same ruling, so item 7 blocks neither phase-three entry nor a code merge. What
item 7 protects is held meanwhile by the workspace: the build workspace is a fresh
clone carrying neither the old tree nor the probe's code, so the coding seat has
nothing in reach to copy from while the quarry still stands elsewhere. Item 7
stays owed, and G6 followed by the deletion retire the reachability question
rather than deferring it.

## 6. Phase three, coding

Ratified by the operator, 2026-08-04, all five gates and the three cells below.
The entry gate held until that date: no crate code was written until this section
ratified and phase two closed, because a gate invented while looking at a diff is a
gate shaped by that diff. Both conditions are met, phase two's close reading as
checklist items 1 through 6 with item 7 outliving it per section 5's ratification
record, and the gates are in force.

The gates:

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

The three cells, settled with the ratification. H2 runs as a review query against
the graph for now, a build script being a later mechanization of the same check.
H1's mechanical bar is the Spec's own instruments: the doctests, compile pins, and
perturbation tests land with the code they pin, with clippy and fmt as the floor.

**The bar names instruments and owes the invocation that runs them.** That absence
is the defect of #288 and is closed here. `cargo test --workspace` was **not** the
suite: it compiled `weaver-spu` with no engine, so it ran none of the
model-loading, decode, device, or family-selection tests. Measured 2026-08-23 it
reported 398 passing and did not run 51.

The suite is two invocations and a machine is honest about which it can offer.

    the host suite     cargo test --workspace

    the device suite   cargo test --workspace --all-features

**The host suite is one command as of the gguf default**, and was two while
that gate was off, the second reaching a family surface the first could not
compile. `weaver-spu-Spec` section 1.1 argues the change: the gates' shared
reason, that a build with neither keeps the family surface testable on a
machine with no device, covered the device gate and never covered the other.
The host suite needs no card
and runs everywhere the C++ toolchain builds llama.cpp. The device suite needs
the pair and the fixtures, and it is the one whose green means the suite is
green. **A merge states which it ran**, because a claim of green that does not
say which suite is a claim about an unnamed subset.

**The perturbation obligation reaches the rule, not only the test.** Apex
section 11's third device is the standing rule: always confirm the test fails
when the property is removed, because a test that passes either way converts
unenforced into documented as enforced. That is stated for tests, and two
cases fell outside it in the week of 2026-08-23.

**A judgment can have no test to perturb.** #284 narrowed the readout refusal
from the container to the family's declaration, which is the whole substance
of that act. Restoring the container ground afterwards, which undoes it
entirely, **failed nothing in a hundred and sixty-four tests**. Every negative
case reached its refusal through a family declaring no tap and so refused
under either rule, and the judgment's own unit tests called it directly
without travelling the admit path. The obligation says to confirm that a test
fails when its property is removed, and the property here had no test whose
failure could be confirmed, so the obligation had nothing to bite on. **So an
act that narrows or widens a judgment perturbs the judgment**, restoring what
it retired, and records what caught it. Where nothing does, that is the finding,
and the act owes the watch before it owes anything else.

**A build requirement cannot be perturbed on the machine that has it.** The
`gguf` gate carrying the default on 2026-08-23 left `llama-cpp-2` taking CUDA
unconditionally, so a host build would have demanded a toolchain it has no use
for. No test on this workstation can fail for that, because the machine it
breaks does not exist here, and two documents asserted the opposite while the
suite ran green. **So an act that changes what a build requires names the
machine that can no longer build it**, and that naming is the check. A green
suite is silent on this by construction rather than by oversight.

**The record goes in the act.** A perturbation run and reported in a
conversation is evidence that expires with the session. Named in the commit or
the pull request, it is the one place a later reader can find out whether the
property was ever watched. Apex section 11's closing line is the reason: a
clean automated gate is evidence that the gate did not fire, and it is not
evidence of correctness.

**Coverage is not readable from any one manifest.** Cargo unifies features across a
workspace, so a crate's tests may run because a different crate wanted the feature.
`weaver-types` gains twenty-one config tests under `--workspace` only because
`weaver-admin` takes that feature for its own reasons, and nothing connects the two.
Were admin to stop needing it, those tests would stop running with no edit to
`weaver-types` and nothing reporting the change. So a run states the features it
resolved rather than the features it was asked for.

**The three bars are not equally held today, and saying so is the point of naming
them.** Measured 2026-08-23: the device suite passes, `cargo clippy --workspace
--all-targets` reports six warnings, and `cargo fmt --all --check` reports a hundred
and thirty-nine diffs. A floor stated and unmet reads to a later seat as a floor that
was never meant, so either the tree rises to it or the bar is rewritten to what the
work actually holds. That ruling is the operator's and is not taken here.
A failing H3 case files as a known gap with a named owner rather than blocking
merge, for the loop 0 act only, because the bare-minimum milestone does not wait
on refusal-path coverage - and the gaps are named at filing so the exception does
not become the permanent state. After loop 0, a failing H3 case blocks merge.

## 7. Current position

The set was ratified set-wide on 2026-08-04 per the operator's ruling recorded in
section 5, and **charters have ratified on their own since 2026-08-23**, so the set
is whatever the charters currently say. Phase one closed with the whole set merged:
seven charters, seven Specs, the contract layer, and the assertion records under
their instruments. **Two crates were chartered since**, `weaver-state` and
`weaver-internal`, both on
2026-08-18, both ratified under the per-charter rule, and **both joined the apex's
enumeration on 2026-08-23 when it was corrected to nine.** **The roster reached
ten on 2026-08-24**, when `weaver-diagnostic` was chartered as a consumer
outside the boundary and the operator's later ruling of the same date moved it
inside as the harness's third member, the mechanism the harness authors a
diagnostic-trace through. `weaver-analysis` was chartered beside it in that act
and does not enter the roster, holding the position outside that
`weaver-diagnostic` vacated. Phase two ran
on the HADES server per `HANDOFF-2026-08-04-hades-graph-build`, the graph stood up
from the merged set, and the set-level mark rides the apex's system record.

**The diagnostic leg is delivered end to end, 2026-09-01, and the roster
is twelve directories under `crates/`.** What was owed at the last refresh
is built: `weaver-diagnostic` writes the diagnostic-trace as the harness's
third member, the replay loop runs from the Gateless seat's own criterion,
the null replay certifies against real records, the column seam carries the
residual vectors under the diagnostic binding and no other, and
`weaver-analysis` stands outside the boundary as the driver and the reader
- deriving the declaration from the record, preloading, gating on the
stated outcome, applying the lens, and comparing two captures exactly.
Epic #293 closed on the operator's direction with its one live row, the
capture artifact, carried by issue #386, and that act's papers landed the
artifact criteria from measurement rather than assumption. **The vector
bar is a measured number**: two certified column replays of one source
differenced to 9,784,320 of 9,784,320 values exactly equal, so
certification's vector comparison is exact within a device model and the
float tolerance is the cross-device bar alone.

**What the instrument measured about itself belongs here too, because it
bounds where the next acts point.** On the 0.5b the lens reads concrete and
lexical content and does not read the abstract evocations the source
paper's workspace results turn on - unchanged at five times the fitting
compute, so the bound is the model's scale rather than the fit's thinness.
The families above it are therefore where the instrument earns its keep,
and `weaver-spu`'s per-family `taps_readout` and `taps_column` are what
stand between it and them, each owed its neutrality demonstration on the
engine that would serve it, per issue #212.

**The graph was rebuilt 2026-08-08 from `98c8713`** and stands at 293 nodes and
426 edges in 19 `wt_` collections under the named graph `corpus_graph`. The census
below verified on that build. Two earlier builds preceded it, 2026-08-06 from
`96c40bb` and 2026-08-04 from `0426ef5`, and each was a drop-and-rebuild rather
than an upsert, which is checklist item 1's audit trail as far as it has been
recorded. **A rebuild is owed on document movement even where the census does not
move.** The 2026-08-08 rebuild found the record set almost unchanged across 28
document commits, two assertions retagged and none added or removed, while 138 of
242 assertions pointed at a line the document no longer held. A count check would
have reported that graph healthy, so a matching census is not evidence a rebuild
can be skipped. Document movement since `98c8713` had added three `draws` edges by
2026-08-10, and the state leg's papers of 2026-08-18 and 2026-08-19 have
since added a crate node, its parent and seam edges, a contract with its
parties, draws, and four term definitions, and three assertion records, so
the stated expectation of 293 nodes and 429 edges is withdrawn as stale.
The next rebuild derives its expected census from a fresh pass over the
merged set before it runs, stated as numbers at that pass per this
section's own discipline, and HADES remains down meanwhile, so the rebuild
is owed and not runnable.

The floor probe of 2026-08-04 is the evidence the entry into code rested on: a
commissioned session with no repository access rebuilt both floor crates from the
graph and the corpus text, both compiled, the suite passed whole, and all 41 floor
assertion slugs landed under their instruments. Its three divergences were named
for the first code act rather than as defects in the set. One, the probe inverted
the non-exhaustive election and re-aimed the compile-fail pin at the inverted
claim: the real Specs elect the attribute per type, growing sets carrying it and
closed sets not. Two, the probe elected two permission modes where the floor Spec
enumerates three, Ask, Allow, and Deny. Three, the probe filled the fault report's
deliberately open election with an invented shape, and the deferral is the
corpus's and stands.

Seat assignment follows section 1's rule unchanged: the seat holding the working
tree authors, and review runs through the PR's review seats. Later code acts sit
where the operator points them.

**An eighth crate joined the set's shadow on 2026-08-18 and the roster
below predates it.** The statefulness leg returned through apex section 9's
door: `weaver-state-PRD` chartered the custodian on the operator's rulings,
`weaver-harness-state-contract` and the Spec landed its seam and shapes, and
the code acts stood the ingest (#208) and the serve direction with its
first asker, the context-injection loop (#209, #210), each proven against
the living agent - the loop now injects the session's shape at a run's
opening and the model has answered from it. The crate carries three
conformance headers, whose assertion records land with the position
refresh of 2026-08-19 after the refresh found them cited but undeclared,
each tagged review. A recount over the eight-crate set is owed at the next
counting pass and the table below is the seven-crate figure of 2026-08-16.

**All seven crates of the ratified set are built and merged.** Each file
carries its conformance header per Document Format sections 3 and 4, and no
crate carries a header citing an assertion no Spec declares. Recounted
2026-08-16 over every tracked unit carrying a header, per the Document
Format's counting clause:

    weaver-types      18/18      weaver-harness   55/56
    weaver-traits     24/24      weaver-gate      27/27
    weaver-trace      38/38      weaver-admin     32/32
    weaver-spu        60/62

**The figures moved on the method rather than on the work**, six of the seven
rows changing at that recount and none of them because a crate gained or lost a
citation that day. The count had been taken over `src` alone, which excluded
every assertion whose citation sits in an integration test while including
manifest assertions cited in `lib.rs` whose instrument is the manifest. The
earlier table read `types 17/17`, `trace 39/39`, `harness 48/48`, `gate 23/23`,
`admin 31/31`, and `spu 56/60`, and it is recorded here because a reader
comparing an old batch against this section needs to know the ruler changed and
not the thing measured.

**The roll of open assertions is recounted 2026-09-02, and it is fifteen
rather than three.** The method is stated so a later reader can repeat it
rather than trust it: every `node:` in an assertion record across `docs`,
against every `conforms:` header across `crates`, the difference being
what no code file claims. **355 assertion records are declared and 340 are
cited**, and the tag census runs 137 perturbation, 136 review, 32
manifest, 32 compile-pin, and 18 compile-fail. The earlier roll of three
was taken by hand at a smaller set and did not move as the set grew, which
is the drift a stated method exists to prevent.

    weaver-analysis   analysis-binds-no-port                        review
    weaver-analysis   analysis-writes-no-record                     compile-fail
    weaver-harness    harness-idle-report-authors-without-a-turn     perturbation
    weaver-spu        spu-architecture-and-markers-are-unique        compile-pin
    weaver-spu        spu-elected-readout-changes-no-token           perturbation
    weaver-spu        spu-family-is-architecture-and-template        perturbation
    weaver-spu        spu-field-changes-no-token                     perturbation
    weaver-spu        spu-field-depth-refused-below-the-cutoff       perturbation
    weaver-spu        spu-gguf-is-the-default-gate                   manifest
    weaver-spu        spu-one-forward-per-prompt                     review
    weaver-spu        spu-reduction-renders-its-shape                perturbation
    weaver-spu        spu-room-refusal-carries-capacity              perturbation
    weaver-spu        spu-sampler-holds-nothing-between-generations   perturbation
    weaver-spu        spu-seed-derives-per-generation                perturbation
    weaver-trace      trace-no-version-member                        review

**Uncited is not the same as unbought, and the roll cannot tell them
apart.** Three of the fifteen are review-tagged, where the instrument is a
reading and a header is a courtesy rather than the purchase. The rest name
an instrument, and for those the header's absence is either an unwritten
test or a written one whose act forgot the citation - a distinction only
the reading of each can make, which is the counting pass owed rather than
this refresh's to settle. **What the roll does say is where they cluster**:
twelve of fifteen are the SPU's, the crate that grew fastest under the
readout and decode acts, and the seed derivation, the room refusal, and
the sampler's memory each name behaviour the seam tests exercise daily
without claiming.

`spu-two-taps-one-shape` left this roll on the GGUF tap's landing, and
`harness-idle-report-authors-without-a-turn` remains what it was: the idle
report is unbuilt, so nothing authors one at all, and its sibling
`harness-frame-grants-the-seat` is cited.

**Four came off this roll on 2026-08-16, and each is named with what closed
it**, because a reader who saw one listed learns it closed rather than finding
it absent:

    admin-run-reference-distinguishes                    cited, PR 161
    spu-session-parameters-carry-dispositions            cited, PR 160
    spu-tunables-arrive-in-the-declaration               cited, PR 160
    harness-organ-argv-carries-construction-parameters   cited, PR 162

`admin-run-reference-distinguishes` was satisfied on 2026-08-15 and uncited, the
act that built the three-part reference not adding the header, so its citation
landed separately. The other three were cited by the acts that closed them, the
last of them landing hours after the document that authorized it, which is the
ordinary sequence rather than a delay.

An assertion sitting in a census unexplained is the shape this project has
repeatedly found, a record unable to say what it did not measure, so the roll
carries the reason beside the name in both directions: what an open one waits
on, and what closed one that has gone.

**Both decode engines are written, and the turn completes through either.**
The GGUF engine landed 2026-08-08 and the native engine followed across
issue #158's arc, closed 2026-08-19: `native.rs` stood 2026-08-17 (#196),
the pair
forward and the split loaders opened the dual-GPU grid in both containers
(#200, #201), and a 65 GB sharded artifact too large for any single card
served across the pair (#202). Section 4.1's derivation answers for both
containers to about 90 GiB across the device pair. The direct peer-to-peer
reduction was entered measured at the close and rejected on the evidence,
the hop staying host-staged with the rejection recorded at the function.
Epic #130 completed the first live turn on 2026-08-14, gate to trace,
against a real local model, and turns have run daily since.

**The demonstration is the evidence and it is inspectable.** A trace taken
2026-08-16 carries one run reference over six turns, each running
`turn.started`, `message.user`, `model.request`, `model.output`,
`model.measurement`, `message.assistant`, `turn.closed`, the whole bracketed by
a `load` and an `unload`. So a directive does reach an engine from outside its
process and an answer returns.

**What rides back with it is the measurement payload, and the readout joins
it where elected**, the two being apex section 7.2's two items rather than
one. The payload carries the timings, the per-token entropies and
surprisals, and a block label over the turn's token range. The
residual-stream readout's native tap stood 2026-08-19 with #158's closing
act: an elected qwen2 residency taps every forward at either width, each
layer's norm taken on the device with one scalar crossing, and the
reduction travels in the measurement as `residual_norms`, absent rather
than empty. The refusal turns on the election alone: an elected GGUF load
refuses at admit by name, that tap being unwritten still, while a GGUF
load with `residual_readout_election` false admits and serves turns
exactly as before the tap existed. Where readout is not elected the SPU
emits no `residual_norms` member at all, and the harness and the trace
carry the measurement as opaque JSON either way, so the omission stays an
absence in the record rather than anything converted to empty. The
standing agent declaration elects no readout.

**A completed turn satisfies no conformance assertion, and the SPU's two
open moved on 2026-08-19 without closing.** Both waited on the readout tap
existing, and the native half now does. `spu-two-taps-one-shape` still
waits on the GGUF half, the eval-callback pin holding that seam open with
nothing driving it. `spu-one-forward-per-prompt` is now watchable under
the standing native tap and waits only on its count being taken. A reader
who takes the demonstration as having closed either has read a count into
a behaviour.

An earlier wording of this paragraph said four, which was the figure the table
carried when the count ran over `src` alone. Two of that four were cited all
along in the tests that buy them, and the recount above is where the figure and
its method now sit together.

The caution this paragraph carried still holds and now points both ways. A
crate could always report a high conformance figure while completing no turn,
which is why the apex asks for a demonstration and not a count, and a completed
turn is likewise not a count of claims met. Read the figures below as what they
are, and read the trace for whether the deliverable runs.

**The four defects the live turn surfaced are closed, 2026-08-15.** Run
identity landed first because five registered measurements join a result to a
trace and could not: the session is the operator's and declared in
`agent.yaml`, and the run reference is minted at the load from an instant, the
agent's name, and eight bytes of randomness, so a declaration without a
`session` field is now refused at load. The unload's misreport of a clean
unwind closed against the Spec's own clause. The unclosed run bracket on a
failed load needed no ruling, the charter and the Spec having both already
required the rollback to direct a leave. The gate socket closed by the
operator's reshaping of 2026-08-14: the socket is fixed by the application
rather than named by a declaration, so `GateInstruction` no longer carries a
path and the stale-pathname hazard is unreachable rather than guarded against.

**A measurement regime stands outside this repository and the first baseline is
taken.** Nine tests are registered with their methods before they run, and each
result carries the conditions that make it comparable later, which includes the
commit, the build profile, and the identity of the binaries measured rather
than the profile's bare claim. It is deliberately not a corpus member and
nothing here is written against it. It does not reach the gates, and a reading
it produces is evidence about the code rather than authority over a document.

**Where the work sits as of 2026-09-02.** The seat is using the framework
rather than building it, per the 2026-08-19 shift, and the pulls this week
came from use exactly as that shift predicted: the diagnostic leg's papers
were pulled by a replay that had to run, the artifact criteria by
artifacts that already existed and needed identity, and the vector bar by
a comparison that wanted a number. What stands open, in the order the
seat holds it: the family taps toward the scale the lens needs (#212), the
streaming shape the sink's declaration already permits and no run has
exercised, the counting pass the roll above names, and the graph rebuild
owed since `98c8713` and unrunnable while HADES is down. One operational
finding rides beside them, filed 2026-09-02 as #404: a deployment whose
organ binaries and admin come from different commits dies with a bare
`Undecodable` and reports as `no_residency`, naming neither the binaries
nor the field, so an experiment directory is all-or-nothing until that
reporting is sharpened.

What remains from the phase behind: G6 and then item 7, and the G2 and G5
phase-close sweeps. The graph's expected census is stated as a number rather than
as a delta so a rebuild can detect a change nobody intended: **242 assertion
nodes carried by 243 `asserts` edges at the last statement**, which the state
leg's three review assertions of 2026-08-19 have since outgrown, the fresh
figures landing with the pre-rebuild pass named above. The two figures differ
by one on purpose, and the difference is a finding rather than an error: the assertion
`types-tagging-test` is asserted by both floor crates, so one node takes two
edges. Which figure a check reads therefore matters, and the closing checklist's
item 5 reads nodes. A rebuild returning any other figure has found either an
unlanded edit or an assertion an act changed without recording.

**Code is not ingested into the graph and that is deliberate.** The earlier ground
was that conformance headers cited retired assertions and ingesting would bake
dangling edges into the map. That count reached zero on 2026-08-08. The standing
ground is the operator's, recorded here because a later reader will find the
earlier one discharged and needs the current one: the architecture is not stable
while acts like the 2026-08-05 re-entry still move it, and a conformance graph
built from moving code would record a shape neither the documents nor the code
will keep.

**The seat shifted 2026-08-19, on the operator's direction: from building
the framework to using it.** The apex deliverable stands and is exceeded,
and the suite around it is named in the living vision's section 13,
weaver-web standing up as the first outside consumer against the two
external contracts of 2026-08-01. **It stood in its own tree until 2026-08-23 and
is absorbed into this one by the ruling of that date**, which leaves the contract
coupling untouched and makes both sides of the seam editable in one commit. What
that changes here is who leads: needs discovered in use pull framework acts
through the change protocols, where the roadmap once pushed, and the
predicted pulls are streaming through the gate's world contract, a status
ask on admin's operator contract, and the operator's read on state per that
charter's named cell. Framework work queued on its own account: the Python
connector of issue #134, the payload-key election through the declaration,
and the Role::System floor act.

## 8. What this document does not do

It does not govern the content of any crate. It governs which seat is primary, in what
order the work moves, and what must be true before it moves. A rule that constrains
what a crate does rather than how it comes to exist belongs in the apex or in a
charter.

It does not say what shape a document takes. That is the Document Format's.
