# WeaverTools Document Format

**Version:** v0.6, 2026-07-31. Companion to the Working Process. Project documents
carry a version and a date and no state, per Working Process section 2. The v0.6
change is six entries in one act, the sixth restating the stub paragraph of section
2 to the v0.6 stub ruling of the Working Process, live stubs at `.stub` and consumed
stubs leaving the tree. Four more were ruled on this date and never landed,
found when three merged charters cited rules this document did not state: the
contract-by-suffix sentence with its drawn-material reading and the container
table's gloss, per the filing ruling `weaver-organ-channel` section 5 records, the
section 3 reconciliation of the kinds outside the Working Process's three, and the
section 4 declaration rule scoped to the declaring side with the organ-channel
case, per the rule `weaver-admin-harness-contract` section 0 states. The fifth is
the `Loops/` container entry, per the filing ruling of this date. The v0.5 change
was one entry: the artifact example in section 3 stops naming `weaver-admin` as the
state file's writer and stops citing the deferred state-file contract, both retired
by the admin pass.
**Parent:** WeaverTools Working Process

The Working Process says who is primary and in what order the work moves. This says
what shape the documents take so that the graph can be built from them mechanically.

Ratification is the mapping of the whole document set into the graph. A document that
has to be interpreted before it can be mapped puts a human in the middle of the one
step this project treats as the terminal gate, and an interpreted mapping is a hand
edit wearing a different name. So the nodes and the edges are stated, not implied.

## 0. What this document is for

Every document in the corpus already states edges. It states them in four notations
and none of them is mechanical. `Parent:` headers state one kind. `Depends on:` states
another and carries no tag. `weaver-harness-PRD` section 4 states four seams and two
floor links in prose. Every contract's vocabulary clause states a whole edge set to a
fixed shape by ruling, which makes it the largest body of edge data in the corpus and
still not a form a mapper can read.

This document does not invent the edges. It picks one notation for edges that are
already being written and says where the notation goes.

## 1. Two node layers

The document layer is what the corpus is made of. The subject layer is what the corpus
is about. These are different graphs sharing one file set, and a file can sit in both
without the layers collapsing. A contract is the standing case: it is a document node
with a container, and it is also the source of `party` and `draws` edges that run
between crates. The document it is and the crates it speaks about are different
subjects, and a mapper that treats one as the other builds a graph where a file is its
own dependency.

A document node carries a container and nothing else. The container comes from the
directory, per section 2, so a document node is built by walking the tree rather than
by reading a block, and only a document that is itself the source of an edge needs a
`node` record. A contract is the one kind that qualifies, because `party` and `draws`
run from the contract rather than from either crate. A PRD states its subject crate's
edges and is not their source.

No edge joins the layers. A document's subject is its directory and its filename,
which the walk already has, so a `describes` edge would carry a fact nothing had to
look up. The same argument retired `governs`: an edge no rule can emit, restating
what another record already holds.

State is not a graph property. It answers no query the phase two checklist names, it
lives in the status header where the merge process reads it, and a second copy in a
block would be a G5 duplication with nothing gained. Section 5's key set closes
without a state key deliberately.

## 2. Containers come from the directory

The directory structure supplies the container of every document node and no document
declares its own container.

    process/                            the project documents
    docs/project/                       the apex, and documents outside the set
    docs/crates/weaver-<n>/             the PRD and the Spec for one crate
    docs/crates/weaver-<n>/weaver-<m>/  a member crate of that domain root
    docs/crates/contracts/              contracts, and the material they draw
    docs/crates/weaver-<n>/Loops/       workflow documents of that domain root

A document under `docs/crates/contracts/` is a contract by its `-contract` suffix,
named for its parties. A document there without the suffix is drawn material, stated
once where the contracts that cite it draw from it, and `weaver-organ-channel` is
the first, per the filing ruling its section 5 records. The suffix reading is the
same exclusion-by-naming device this document uses twice already, the absent
`weaver-` prefix and the `.stub` suffix. A document under a `weaver-<n>/` directory
is a PRD or a Spec by its own filename suffix. This is what makes the mirror check
mechanical: every `docs/crates/weaver-*` has a matching `crates/weaver-*` at the
same depth, and `contracts/` excludes itself by the absent prefix rather than by a
written exception.

A document under a `Loops/` directory is a workflow document, the container typing the
kind directly because that directory splits into no further kinds, where `contracts/`
needs the suffix to split contracts from the material they draw. It walks a settled set
and authors no edges of its own, so it carries no graph block, and where it and a
charter disagree the charter yields nothing. `Loops/` excludes itself from the mirror by
the absent `weaver-` prefix, the same exclusion `contracts/` takes. Loops file under the
organ that holds them, and agent loops belong to the harness, so the first such
directory is `docs/crates/weaver-harness/Loops/`.

**A contract is named for its parties, initiator first.** The name records which of the
two nodes initiates the signal the contract governs, so `weaver-admin` asking the
harness to enter and leave a run gives `weaver-admin-harness-contract`, and the harness
asking the SPU to admit and release a model gives `weaver-harness-spu-contract`. This is
the same fact the seam record carries in `from` and `to`, which is why a name that
disagrees with its seam is a defect in one of them rather than a matter of taste.

**The name states flow direction and nothing else.** It does not say which crate
sequences the workflow that signal sits inside, and it does not say that this is the
only traffic between the pair. Behavior between two nodes collapses into direction of
flow, one signal at a time, and a later signal running the other way is a second seam
with its own name rather than a contradiction of this one.

**A live stub carries `.stub` and is not a document.** It is not a `.md` file, so the
walk never reaches it and the mirror never counts it, which is the same exclusion by
naming that `contracts/` gets from its absent prefix. A stub leaves the tree in the
act that cuts its draft, per Working Process section 2 as ruled 2026-07-31, the draft
landing at the `.md` name beside the suffix's tracked deletion, so no consumed stub
survives to be walked and history is the archive.

**The project documents sit outside `docs/` rather than under it.** Working Process
section 2 puts them outside the document set and outside the mapping, so a mapper that
walks `docs/` never reaches them and needs no rule to skip them. Placing them under
`docs/project/` would have required one.

**The mirror is recursive, and the nesting carries domain rather than dependency.** A
**domain root** is a crate that other crates are members of, and a member crate nests
inside its root at both trees. The word is not *composition root*, which this program
already uses for the wiring site inside one binary where the concrete transport is
constructed and injected. That is one place, made of code. This is several, made of
crates. Section 5 forbids two spellings of one name, and one spelling of two things
is the same defect with the halves swapped. `weaver-trace` sits under `weaver-harness`
because the harness is its only caller, not because nothing else may link it. Nesting
is not a visibility rule, a Cargo boundary, or a claim about who may depend on what.
Those are settled by the seam and floor-link records, which do not read the
directory. Nesting says which crate holds the domain, and its purpose is that
someone who knows where a crate lives can find its documentation without a search.

**The floor does not nest.** Which crates are floor is `WeaverTools-PRD` section 5.1's
to say and not this document's, per section 8. What belongs here is where they sit:
a floor crate is placed at the top of the crate tree rather than under a domain root.
Filing one under a root would make every other root reach through that root's subtree
to draw floor vocabulary, which reads as the floor belonging to one domain when the
invariant has it belonging to none. The rule is a directory rule and takes its
classification from the apex, so a change to what the floor consists of changes no
line here.

**Depth is discovered, not designed.** A root gains a subtree when it gains a member,
and a root with no members yet is a directory holding one PRD. Drawing a subtree for
a crate that has not been chartered would guess at its membership, which is the
reserved-slot move applied to directories.

## 3. Node kinds

**system.** One node, `WeaverTools`, declared by the apex. It exists because G3
requires each crate to have exactly one parent and the domain roots have no crate
above them.

**A parent edge names the domain parent.** A member crate parents to its domain root and
a domain root parents to `WeaverTools`, so the parent edges reproduce the directory tree
exactly and the graph answers the question the filesystem answers. This matters because
domain membership is an architectural fact, and a fact that lived only in a directory
listing would be hiding in the one place section 1 says facts are not allowed to hide.
Every crate still has exactly one parent and the crate graph is still a tree, so the
shape G3 checks is unchanged. What changed is what the edge means, and G3's wording is
updated in the same act rather than left to be read against a rule it predates.

**The `Parent:` header names the same thing the edge does.** It carries the charter of
the domain parent, which is the apex for a domain root and the root's charter for a
member. The header is a reader's convenience and the edge governs, so a disagreement
between them is a defect in the header. It is kept rather than dropped because a
charter opened on its own should say what it belongs to without a directory listing to
hand.

**crate.** Every crate, whether a domain root or a member of one. Organ is the
vision document's word and it does not cover the floor, so it is not used here. One
PRD, one Spec, and as many contracts as it has seams.

**artifact.** A durable thing produced or consumed by crates without a call between
them. The agent state file, the session record, the model artifact. Artifacts are
first-class because the corpus's real relationships around them are three-place and
pass through one: `weaver-types` owns the agent state file's format, the operator
writes it from outside the program, and `weaver-admin` and the harness both read it,
with no Cargo edge anywhere in that sentence. Without an artifact node those
relationships are untypeable. An earlier form of this entry had `weaver-admin` as the
writer and cited a deferred state-file contract, and both halves are retired:
authorship is the operator's per `weaver-admin-PRD` section 1, and section 10 of that
charter rules the contract out rather than deferring it, there being no producer
inside the program to bind.

**vocabulary.** A named definition a crate owns and contracts draw: a trait, a type, a
mode, an event kind. The unit G4 resolves against.

**document.** A PRD, a Spec, or a contract. Working Process section 4 produces
three kinds and no fourth, and the two documents this format types that are none of
the three do not breach that: drawn material is contract material stated once,
holding what the organ contracts share and deciding nothing they do not draw, and a
workflow document walks a settled set and authors no edges of its own. The three
kinds author the graph. These two add no authored record to it, which is what the
Working Process's count is a count of.

## 4. Edge kinds

Between crates:

- `parent`, exactly one per crate, declared by the child.
- `floor-link`, a compile-time dependency on a floor crate, declared as a floor link
  and never confused with the parent edge.
- `seam`, a place where one crate asks another to do something. Carries `via`, the
  governing contract, and `tag`, either `socket` or `link`. A seam with no governing
  contract is an incomplete edge and G3 fails on it.

A pair is never both a `floor-link` and a `seam`. Linking a floor crate and asking it
to do something are different relationships and the harness has one of each: it links
`weaver-types` and asks nothing of it, and it links `weaver-trace` and asks it to
record. Where a governing contract exists the pair is a `seam` tagged `link`, because
the seam record carries the contract and the floor-link record cannot. Where none
exists the pair is a `floor-link`. Writing both for one pair produces two edges where
the system has one relationship.

A seam is declared once, from the declaring side. On a seam with one asking party
the asker declares. On an organ channel both parties ask, so the asks rule has no
unique answer there, and the organ declares rather than the harness, per the rule
`weaver-admin-harness-contract` section 0 states: the harness is the hub every organ
is duplex with, and a hub that declared its own edges would carry the whole seam
graph in one crate. The other party's charter points at the contract and does not
restate the edge, which is what keeps the party list checkable rather than doubled.

Nothing points back at a seam. A seam is an edge, an edge cannot be the target of an
edge, and giving seams identifiers so that contracts could address them would buy an
inverse edge that carries no fact the seam's own `via` does not already carry. The
governing relation is stated once, on the seam, from the declaring side. The
contract's `party` records are what make the pair checkable from the other direction.

Between a contract and what it binds:

- `party`, from a contract to each crate it binds.
- `draws`, from a contract to each vocabulary node its clause names. This is the
  vocabulary clause in edge form and it is what makes G4 a query rather than a reading.

Between a crate and what it owns or touches:

- `defines`, from a crate to a vocabulary node. The definition site declares the
  vocabulary node with a `node` record beside the edge. A `defines` edge does not
  introduce its target, because an implied node has nowhere to carry a kind and
  because implied nodes are what this format exists to remove.
- `elects`, from a vocabulary node that is a field to the vocabulary node whose values
  it selects from or is validated against. The floor's two-crate structure rests
  entirely on this relation: the agent state file's permission mode and tool set are
  `weaver-types` fields electing from `weaver-traits` definitions, and a `floor-link`
  is crate-level and carries none of it.
- `holds`, from an artifact to a vocabulary node that is one of its fields. A clause
  that draws a field of an artifact points at the field, which is a vocabulary node
  the owning crate defines, and the artifact holds it. Without this the state file's
  fields are drawable in prose and unaddressable in the graph.
- `writes` and `reads`, from a crate to an artifact.

## 5. The block

One fenced block, info string `graph`, one or more records, records separated by a
blank line, one `key: value` per line. A mapper finds every block by its fence and
needs to know nothing else about markdown.

    ```graph
    edge: seam
    from: weaver-harness
    to: weaver-trace
    via: weaver-harness-trace-contract
    tag: link
    ```

    ```graph
    node: weaver-harness
    kind: crate

    edge: floor-link
    from: weaver-harness
    to: weaver-types
    ```

Identifiers are kebab-case, always, including for vocabulary that names a Rust item.
`Tool` is `tool-trait` and `Provider` is `provider-trait`. Two spellings of one name
is how a graph acquires two nodes for one definition, and a rule that says follow the
source spelling produces exactly that on the day a trait is renamed. Identifiers that
differ by one character are a G1 visual collision as much as a mapping hazard, so
`permission-modes` and `permission-mode` do not both exist.

Keys are `node`, `kind`, `edge`, `from`, `to`, `via`, `tag`, and `grounds`. A record
begins with `node` or with `edge` and no record carries both. Unknown keys are a defect
rather than an extension, because the point of the fixed set is that the mapper never
guesses.

## 6. Placement, and why not a header block

A block sits in the section that argues the edge, directly under the prose that argues
it. Not in a header block, and not in a topology document.

A topology document is the single document binding every crate that the vocabulary
ruling already rejected, wearing a different noun. It separates every claim from its
grounds, it duplicates facts the charters also state, and under G5 it would need an
authority named against each one.

A header block is the same failure at smaller scale. It collects every edge at the top
of the file, two screens from the paragraph that justifies it, so a rewrite of the
paragraph and a stale line in the header cannot be seen together. G3 asks for grounds
per edge, and grounds are prose.

So the grounds are the containing section, and `grounds:` is written only when the
argument lives somewhere else. Position is the pointer. This is the vocabulary clause's
own argument applied one level out: a clause is checked at the seam, by the people
writing that seam, while they are thinking about it.

## 7. Where a block and its prose disagree

The prose yields nothing and the block yields. The prose is the argument and the block
is its projection, so a block that contradicts the paragraph above it is a transcription
defect and is fixed by rewriting the block.

This is a G5 authority statement and it is written here once rather than at every
block, because the relationship is the same at every block. It is the only duplication
in the corpus that needs no local authority line.

## 8. What this does not do

It does not say what any document contains. Level discipline is G2's and the three
document kinds are the Working Process's. A document that carries correct blocks and
wrong material passes this format and fails that gate.

It does not make the graph. Phase two builds the graph from documents written this way
and never by hand.

## 9. When mapping runs

Mapping runs continuously, from the first charter, and ratification remains the mapping
of the whole set.

A format that is mechanical from the first charter can be mapped the day that charter
lands, which surfaces a missing edge while one document is still in hand rather than in
a terminal pass over nine. The alternative concentrates every mapping defect into one
pass at the end of phase one, where each fix is a phase one reopening and the document
that would answer it was written weeks earlier.

Continuous mapping does not move ratification earlier. A mapped document is not a
ratified one, the set ratifies together or not at all, and an intermediate build is a
check rather than a milestone. What continuous mapping buys is that phase two's closing
checklist meets a graph that has already been built many times rather than one being
attempted for the first time.
