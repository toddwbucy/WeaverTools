# Handoff: building WeaverTools_v4, one crate at a time

**Version:** v0.1, 2026-09-12. Written from the thinkpad seat for the HADES
seat on olympus, against the overnight recovery handoff of the same date.

**Base:** `WeaverTools` at the merge of PR #559. Every number below was
measured against that tree on 2026-09-12 and each says whether it is measured
or estimated.

This is a plan and not a runbook. Two decisions in it are settled and named as
such. One number in it is an estimate doing real work and is the first thing
to replace.

## 1. What is settled

**Rebuild rather than migrate.** `WeaverTools_v3` is a reference copy from a
2026-08-08 dump of the 2026-08-04 build. It holds 242 assertions against the
413 the corpus declares today, it carries the older `wt_*` schema, and it has
no code side at all. Beyond the count, **identifiers have been renamed** -
`web-a-column-frees-at-most-one-member` became
`web-an-arm-frees-at-most-one-member` on 2026-09-11 among others - so a
migration would reconcile adds, deletes and renames across 242 rows to reach
491, with renames the hardest of the three to tell apart. A rebuild costs one
ingest per crate, which the plan does anyway.

**v3 is the template for the schema and not for the rows.** What transfers is
the notation: the seven keys, the edge representation, the idempotency rule.

**One GPU and one window.** GPU 1, the A6000, at a 32,768-token window. No
size-based router, no second device in the path. What does not fit is a named
list of files handled by hand, given in section 4.

**Batch of one.** A 32,768-token sequence peaked at 23,906 MiB at batch one on
the A6000, so `HADES_EMBEDDER_BATCH_SIZE=4` cannot survive the larger window -
four of those is roughly 95 GiB against the card's 48. The corpus is 227 files
and about 1.35 million tokens, this is a one-off build rather than a hot path,
and the operator's ruling of 2026-09-12 is that a slow ingest is acceptable.
**So there is no token-budget batcher and no throughput tuning in this plan.**

**`model_type` says GraphSAGE before the first train.** `hades graph-embed
train` reads it from the schema rather than a flag. The roadmap treats
inductive as a constraint because the graph regenerates per commit, and the
corpus ruled the same on 2026-08-03, that GraphSAGE is right *because* inductive, so
code nodes added at merge time need no retrain. **Crate-by-crate ingest is only
cheap under that choice** - transductive would make per-crate the worst
possible order, twelve full retrains, rather than the best.

## 2. What the corpus holds today

Measured at the base commit.

```
nodes 491     assertion 413 | vocabulary 39 | document 13 | crate 12
              term 5 | axiom 5 | artifact 3 | system 1

edges 665     asserts 414 | grounds 81 | draws 59 | defines 44 | party 23
              seam 12 | parent 12 | holds 8 | floor-link 7
              writes 2 | elects 2 | reads 1

keys 7        node kind tag edge from to via
```

Against the 2026-08-04 build: 290 nodes, 425 edges, 7 node kinds, the same
seven keys.

**The seven-key precondition holds five weeks on**, which is the notation
proving stable and is the strongest argument for templating v3's schema.

**`term` is an eighth node kind v3 never saw.** Five of them. A schema copied
from v3 has no home for them and will drop them silently or wedge them into
`vocabulary`, where they do not belong.

**`tag` appears 426 times against 413 assertions**, so thirteen non-assertion
nodes carry tags. The census validates tags on assertions only, so
`tag: ratified` on a `kind: system` node has never been checked against the
format's five. **Whether the v4 schema admits a tag on a non-assertion is a
decision and not a detail.**

**`reads` fell from two edges to one** between the builds. Small, and an edge
kind losing its only sibling is worth knowing the reason for before a rebuild
absorbs it.

## 3. Chunking

**Structural, with a two-level fallback.** Measured against the three largest
Specs and the three largest sources:

```
docs   ## sections            largest  12,187 tok
rust   top-level items        largest  30,125 tok   (a mod tests)
rust   methods inside that    largest   3,408 tok
```

**No file in this corpus requires an arbitrary split.** Cut at top-level items
for Rust and at `##` for documents. Where one piece still exceeds the window,
cut at its methods. The worst file in the tree is a test module with
twenty-five clean method boundaries inside it.

**The 32k window is for context and the structural boundaries are the cut
points.** These are two different jobs and the plan uses both. A window that
covers most of a Spec in one forward pass means each chunk's vector is
informed by the whole argument - section 9's enforcement table refers to
assertions declared in section 2, and independently embedded sections never
see each other. Late chunking buys that, and structural boundaries buy chunks
that end where a thought ends.

**The overlap seam is placed at a structural boundary**, for the files in
section 4 that exceed one window. The overlapping region is embedded twice
under two different contexts and produces two vectors for the same tokens.
Putting the seam on a section or item boundary means no chunk straddles it and
the question of whose vector wins does not arise.

**A graph fence is atomic.** A ```graph block carries `node`/`kind`/`tag` and
`edge`/`from`/`to` stanzas, and a cut inside one yields a chunk holding half a
declaration. No chunk boundary falls inside a fence.

## 4. The files done by hand

Four at the optimistic token ratio, eight at the pessimistic. The same files
either way, so this is a list rather than a category.

```
always          crates/weaver-harness/src/lifecycle.rs    ~50-67k
                crates/weaver-harness/src/engine.rs       ~39-52k
                docs/crates/weaver-spu/weaver-spu-Spec.md      ~37-47k
                docs/crates/weaver-harness/weaver-harness-Spec.md  ~35-43k

if pessimistic  crates/weaver-spu/src/family/mod.rs       ~38k
                docs/crates/weaver-web/weaver-web-Spec.md      ~37k
                docs/crates/weaver-admin/weaver-admin-Spec.md  ~34k
                docs/crates/weaver-types/weaver-types-Spec.md  ~33k
```

**Four of the eight are Specs whose sections already cut at 12k or less**, so
by hand means one cut at a section boundary rather than a judgement.
`lifecycle.rs` and `engine.rs` are the two needing thought, and
`lifecycle.rs`'s bulk is the 30k `mod tests`.

## 5. The one estimate doing real work

**Every token count above is 3 to 5 characters per token and nothing here has
seen the jina tokenizer.** That assumption is the entire difference between
four files by hand and eight, and between eight files needing the larger window
and thirteen.

**Run the tokenizer over all 227 files once, before anything else.** It is a
short job and it replaces four estimates with one fact. The overnight session's
own lesson is the argument: a token ceiling repeated from a comment that
described a refusal rather than a measurement, caught by running something.

## 6. First ingest: weaver-trace, and why

Every number is known in advance, so the ingest is a test rather than a leap.

```
9 code nodes      61 cites edges      0 files without a header
```

**If the graph reproduces those three, the extractor port, the SCIP reader and
the edge-attachment decision are verified at once**, on a crate small enough to
redo. If it does not, which of the three is wrong is knowable before anything
else is in.

`weaver-trace` is also the crate whose Spec drifted for three weeks - saying
eighteen kinds where the code compiled twenty-one - so the first real query has
a known wrong answer waiting for it.

**weaver-web goes last.** Twenty-seven files, twenty-one with no header,
sixteen citations: the least conformant crate in the tree and the one still
being rewritten. Ingesting it early means ingesting it twice.

## 7. The reconciliation owed before the first ingest

**The SCIP reader reports 156 documents and the tree holds 160 non-archive
`.rs` under the twelve members.** Four files.

Probably `build.rs` and units outside the compile graph, but a silent four-file
gap at twelve crates is a silent forty-file gap at a corpus, and it is one
`comm -13` to settle. **The extractors reproduce three of four census metrics
exactly - `dangling_citations` 0, `sources_without_a_header` 48,
`uncited_perturbations` 33 - and disagree on the fourth**: 478 `cites` edges
against the census's 481 distinct file-and-assertion pairs, out of 516 raw
occurrences. Three edges. Candidates are the `archive/` exclusion, whether
`tests/` counts as citing, and the three `tag: manifest` assertions cited only
from a `Cargo.toml`.

**Both gaps are cheap now and expensive later.** After an ingest they stop
being a difference between two scripts and become a difference between the
graph and the tree, and telling which is wrong gets much harder once the graph
is the thing people ask.

**One question about the extractors that changes what their agreement is
worth:** were they written independently, or calibrated against the census
baseline? If the baseline was the target, matching it confirms the port and not
the number. If they were blind, 33 is the first figure in this corpus two
implementations have reached separately, which is a different and much stronger
thing. The census shipped with twenty-five bugs of its own, so its numbers were
never better than one implementation.

## 8. What ingesting today would write in as fact

The census at the base commit:

```
dangling_citations                      0
untagged_assertions                     0
unknown_tags                            0
duplicate_node_ids                      0
malformed_node_ids                      1
malformed_citations                     0
enforcement_table_mismatch              0
documents_without_an_enforcement_table 12
uncited_perturbations                  33
sources_without_a_header               48
```

**Thirty-three assertions carry `tag: perturbation` and no source file cites
them.** The tag claims an instrument exists. Ingested unflagged, the graph
asserts as fact the thing issue #558 exists to say is not yet true, and a
coverage query over v4 reads thirty-three enforced claims that are not.

**Twelve documents carry no enforcement table**, so for most of the corpus the
rule "cited in the same act, or its enforcement-table row marked owed" has no
table to mark. **One malformed node identifier** and **forty-eight units with
no header** are the remainder.

None of these blocks a build. All of them decide what a build *means*, and the
quarry is the case against ingesting them quietly: `weavertools_v2` froze with
a bilateral-contract certificate at zero edges and an axiom basis covering
seven of seventy-one claims. **The structure was right and the edges were never
drawn.** A graph that encodes today's drift as fact is the same failure with
the edges drawn wrong instead of not at all.

## 9. What this plan does not reach

**The paper corpus.** 300,510 papers, 713,889 abstract-level embeddings, four
docling-converted documents, zero citation edges. Its own numbers are the
argument for treating it as a separate project with its own justification, and
they are the quarry's failure shape stated in a second corpus.

**The memory layer.** `store_observation` fails and `retrieve_memories` has a
signature mismatch, against two client methods that do not exist. API drift
between two files rather than anything deep, and out of scope here.

**The rebrand.** Scheduled separately.
