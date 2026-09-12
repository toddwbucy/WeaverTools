# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## This workspace holds two separate repositories

`WeaverTools_Project/` is **not** a git repository. It is a container for two independent
clones, and the relationship between them is the single most important fact here:

| Directory | Remote | Role |
|---|---|---|
| `WeaverTools/` | `toddwbucy/WeaverTools` | **The new tree.** The ratified document corpus plus the first phase-three code: `weaver-traits`, `weaver-types`, and `weaver-trace` land first - see "Where the work stands". |
| `WeaverTools-archived/` | `toddwbucy/WeaverTools-archived` | **The quarry.** The full ~150k-line prior program, frozen. 438 PRs of history. Scheduled for deletion once G6 certifies extraction complete (checklist item 7). |

The new tree is a **one-way extraction** from the quarry, not a fork. Nothing merges back
in either direction, ever. The quarry is a **parts source you read and never edit** - no
commits, no branches, no fixes there, however tempting. Its last commit is
`d9366d5` (2026-07-28).

The old tree's `CLAUDE.md` and `docs/CLAUDE.md` load automatically when you work inside
`WeaverTools-archived/`. They are accurate about *that* tree and stale about the program's
direction - they describe a 12-crate workspace with a memory leg and a conformance graph,
all of which the extraction leaves behind. Read them for how the parts work, not for what
to build.

### The founding document is not on disk

The handoff that defines the extraction was committed and then immediately reverted
(`9dd8caf` then `d9366d5`, both 2026-07-28). Retrieve it:

```bash
git -C WeaverTools-archived show \
  9dd8caf:docs/project/HANDOFF-2026-07-28-radical-simplification-PROPOSED.md
```

**Read it before doing anything in this workspace.** It is marked PROPOSED and its own
first instruction is that the session produces a written, operator-ratified inventory
before any code is written. Whether that ratification has happened is not recorded
anywhere in either tree - ask the operator rather than inferring it from the empty
`WeaverTools/` repo existing.

## The mission and the carry rule

Deliverable: **a deployable proto-stateful agent that emits a clean, turn-bracketed,
correctly-custodied trace.** The trace is the primary artifact, not a diagnostic - that
reframing is what promotes quarry issues #340/#343/#344/#363 from debt to blockers.

**Proto-stateful, not stateless.** The human's ruling of 2026-08-01 retired "stateless" as
an overstatement, and `weaver-agents-PRD` section 2 is the authority. The agent holds
real state *within* a session and none *across* sessions. **Two things hold state across turns
inside one session, both deliberate and not two things of a kind.** The first is the working
structure, the run's trace events held in RAM in the canonical form the stream carries,
volatile by construction. The second is the hot KV cache, an optimization whose owner, flush
trigger, and forbidden touchers are named in `weaver-spu-PRD`. Lose the first and turn two
has nothing to be about. Lose the second and the agent is slow rather than absent. If you
meet "stateless" anywhere in this workspace outside a record of the rename, it is stale.

Anything crossing from quarry to new tree goes through exactly one of two doors, and you
**state which door and why at the moment you carry it**:

1. **Live code** - a proto-stateful agent provably needs it, meaning you can name the path a
   single completed turn takes through it.
2. **Stub** - a named joint the memory leg will bolt onto, *and* a written memory-leg
   design already names it. No document, no crossing. Without that constraint door two
   becomes the baggage door and every individual stub still looks principled.

Everything else stays in the quarry. **Nothing crosses because the old tree has it.**

In scope: `weaver-spu`, `weaver-harness`, `weaver-gate`, `weaver-admin`, plus
`weaver-trace`/`weaver-types`/`weaver-traits`. None come over verbatim.
`weaver-traits` and `weaver-types` are **demand-derived** - built from what the SPU and
harness turn out to need, never carried and pruned. `weaver-trace` is the exception:
**designed** against what the memory leg will later read, because demand-derivation
under-builds a deliverable.

Out entirely: the memory leg in any form, `weaver-memory`, `weaver-train`,
`weaver-frontend`, and `weaver-interface`. The composition root is
deliberately **new code** - it is where the session boundary gets enforced, and where quarry
issue #350 (the agent worker implements no task executor) gets solved rather than
migrated.

Order of work: SPU -> trace -> harness + new composition root -> admin and gate ->
deployable proto-stateful agent -> autonomic calculator tool -> then memory.

## Where the work stands, and what governs it

**Four process documents in `WeaverTools/process/` govern everything and outrank this
file.** Read them before acting: `WeaverTools-Working-Process` (phases, seats, gates),
`WeaverTools-Document-Format` (the graph notation), `WeaverTools-Working-Rules`
(editorial), `WeaverTools-Handoff-Format`. They carry versions and change often, so read
the file rather than trusting a version remembered from a summary.

**Three phases, and the program is in the third.**

1. **Phase one, authoring.** PRDs, contracts, Specs. Closed 2026-08-04.
2. **Phase two, graph mapping.** The graph was built on the HADES server as
   `WeaverTools_v3` per `HANDOFF-2026-08-04-hades-graph-build`, and **the set is
   RATIFIED 2026-08-04** per the operator's ruling recorded at Working Process
   section 5 - the set ratifies as the complete document set for the toolless
   inference deliverable, the tool workflow's later arrival a planned re-entry.
   Checklist item 7 (quarry deletion) outlives ratification and waits on G6.
3. **Phase three, coding.** Open, gates H1-H6 in force per Working Process
   section 6. The floor (`weaver-traits`, `weaver-types`) and the recorder
   (`weaver-trace`) are the first acts, and every source file carries a
   `//! conforms: <crate>-<slug>` header per Document Format v0.14, and code
   accrues into the graph as it merges.

As of 2026-08-05 the corpus holds 8 PRDs, 8 contracts, 7 Specs, and **245 assertion
records across the seven Specs**. Survey it rather than guessing:

```bash
cd WeaverTools
find docs -name '*-Spec.md' | while read f; do
  printf '%3d  %s\n' "$(grep -c '^kind: assertion' "$f")" "$(basename "$f")"
done
```

**An assertion record is the middle term of apex section 11's `code -> assertion -> doc`
chain.** Each names a claim a Spec makes and tags the instrument that holds it:
`compile-pin`, `compile-fail`, `perturbation`, `manifest`, or `review`. Two rules earned
the hard way and worth knowing before you touch one: **a tag follows the mechanism the
clause names, not the heading it sits under**, and **`review` must mean an instrument was
not bought, never that none exists** - the inverse overclaim forecloses tests the corpus
may later want.

**Gates G1-G7 run on every act** (mechanical, level discipline, graph facts, vocabulary,
duplication authority, extraction completeness, rulings landed). H1-H6 are phase
three's, in force per Working Process section 6, H6 having joined 2026-09-11.

**A ruling is a claim about the whole corpus.** A review finding names one sighting of its
violation, so an act that lands a ruling ends with a corpus-wide sweep for every wording
the ruling retires - and the sweep must be whitespace-normalized, because prose wraps at 88
columns and any phrase can straddle a break. This file is the standing proof of what
happens otherwise: the 2026-08-01 rename swept the corpus clean and left `CLAUDE.md`
behind, because `CLAUDE.md` was not in the tree. **It entered the tree 2026-08-24**, at
the repo root with the workspace copy a symlink into it, precisely so that corpus-wide
sweeps reach this file too.

## This machine is not the deployment box

The quarry's own `CLAUDE.md` documents runtime paths (`/opt/weavertools` source,
`/opt/weaver` installed runtime) that **do not exist here**. Consequences:

- The quarry is cloned to a home directory. Per-agent OS users cannot traverse a 0700
  home, so nothing agent-facing can actually run from this checkout - it is a reading and
  planning workspace.
- `.hades/` is gitignored and absent, so `gate-check.py` cannot run here. The quarry's
  mandatory merge-gate sequence is not executable from this machine.
- `nvidia-smi` is present but there is no `nvcc` on PATH, so `--features cuda` will not
  compile here.
- The pinned toolchain (`nightly-2026-02-13`, rustc `47611e160`) is installed and matches
  `rust-toolchain.toml`.

## Building the new tree

From `WeaverTools/`. Nightly, edition 2024, twelve packages.

```bash
cargo build --workspace
cargo test --workspace
cargo test -p weaver-harness            # one crate
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

**Every command was run from `WeaverTools/` on 2026-09-06 before being
written here**, which is the difference between a command that works and one
that ought to. `build --workspace` and `fmt --all -- --check` returned clean.
`test --workspace` passed 584 and failed none. `test -p weaver-harness`
passed 105 and failed none. The clippy line returned the backlog section
"Enforcement" describes. A later reader re-runs rather than trusting the
date.

**No `--features` flag belongs on the clippy line here.** `weaver-spu`
declares `default = ["gguf"]`, `gguf`, and `cuda`, so the inference path is
on without one. **The quarry's command below is not this one** and carries a
`weaver-spu/inference` flag that is correct there and errors here, which is
a mistake this file's own reader made on 2026-09-06 before these commands
existed.

`--features weaver-spu/cuda` will not compile on a box with no `nvcc`, which
is why no gate command carries it: the CUDA path is verified where the
hardware is, per issue #397's parking.

## Building the quarry (read-only verification)

Nightly, edition 2024. From `WeaverTools-archived/`:

```bash
cargo build                      # workspace, no GPU
cargo test --workspace           # 12 packages
cargo test -p weaver-harness     # one crate
cargo test <name_fragment>       # one test by substring
cargo clippy --workspace --all-targets --features weaver-spu/inference -- -D warnings
cargo fmt --all -- --check
```

Cold resolution needs network: `weaver-spu` sources `candle-*` and `llama-cpp-2`/
`llama-cpp-sys-2` from `github.com/toddwbucy` forks at pinned revs. The `llama-cpp-rs`
fork pin (`a67e208`, exposing the ggml scheduler eval callback - the only route to
per-layer activations from a GGUF model) was the stated precondition for cutting the
extraction, and it **is** in the quarry's `main`.

`crates/weaver-frontend` is excluded from the workspace and needs X11/Wayland/GL dev
libs, so build it from inside its own directory if at all.

## Orienting in the quarry

Sizes matter here - the carry rule is a subtraction discipline and roughly 90k lines
are in scope for consideration. `wc -l` over `crates/<name>/src` gives the current
figures when you need them.

Reading order for architecture: `docs/weavertools-HAH-v41.md` (the hypothesis this whole
apparatus tests), `docs/weavertools-primary-PRD.md` (the apparatus apex),
`docs/crate-topology-Spec.md` (the doc<->crate map). Per-crate PRDs and Specs are at
`docs/architecture/crates/<crate>/`, mirroring `crates/<crate>/` positionally.
`docs/project/handoffs/` and the dated `HANDOFF-*.md` files at `docs/project/` are the
narrative of how each subsystem reached its frozen state.

Design patterns worth carrying forward conceptually (they are the quarry's real
contribution, independent of its code): per-invocation tool safety classification
(`Tool::invocation_properties(input)` inspects the *actual* command - `ls` reads,
`rm -rf` destroys - which drives parallel-vs-serial batching), events-as-rendering-API
(`QueryEvent` over mpsc, consumed identically by CLI/TUI/tests), provider-agnostic
messages with all wire format isolated at the composition root, and `SO_PEERCRED`-verified
Unix sockets for all internal IPC.

## Enforcement, and when the graph arrives

**Corrected 2026-08-03.** This section previously said the new program builds no graph
until just before the memory leg lands. `WeaverTools-Working-Process` section 5 governs
and says otherwise: **phase two stands up a HADES graph from the merged documents, and
closing its checklist is what ratifies the set.** That happened: the graph stood up
2026-08-04 as `WeaverTools_v3` and the set ratified the same day.

The sequence the operator settled on 2026-08-03:

1. **Graph from the documents.** Phase two. This is ratification, not an audit.
2. **Code**, with code nodes accruing into the graph as work merges. The graph is a code
   generation input and a ledger the operator follows during generation.
3. **A GraphSAGE GNN**, trained only once the graph has seen conforming code. It waits
   because the signal worth learning is `code -> assertion`, and the quarry is no bootstrap:
   25 files carry a conformance header and they cite 7 distinct spec node ids. GraphSAGE is
   the right family because it is inductive, so code nodes added at merge time get
   embeddings with no retrain.

The quarry's own graph (`weavertools_v2`, ArangoDB) is the cautionary case, not the
counterargument. At freeze its bilateral-contract certificate had 0 edges and its axiom
basis covered 7 of 71 claims - the two things a graph uniquely provides had never been
delivered. **The structure was right and the edges were never drawn.** The new program's
guard against repeating that is a rule the operator settled before any labelling began:
an assertion that grounds in no invariant is **representation, not an omission**, and the
coverage number is a fact to read rather than a target to reach. Writing that down first
is what stops a low number from being argued away once someone sees it.

**During authoring, enforcement rests on six devices and no graph.** These do not retire
when the graph lands - the graph indexes them, it does not replace them:

1. Conformance trace headers in source carrying `code -> assertion -> doc`.
2. **Compile-time pins** for invariants that are type properties. A runtime test
   structurally cannot pin the *absence* of a trait impl.
3. **Perturbation-verified tests** for invariants that are behaviours. Always confirm the
   test fails when the property is removed - a test that passes either way converts
   "unenforced" into "documented as enforced", which is worse than no test.
4. Human and CodeRabbit review. Read the review **body**, not the thread count: CR posts
   findings outside the diff range that create no thread and are absent from the
   "actionable comments" total.
5. **Clippy at `-D warnings`, per crate at the point of an act**, on the
   operator's ruling of 2026-09-06. **The gate is the crate you touched, not the
   workspace**: `cargo clippy -p <crate> --all-targets -- -D warnings` passes
   before that crate's act merges. It is the cheapest of the six, and until the
   census joined it the only one a person had to type, which is how it went
   unrun.

   **Stated per crate because the workspace did not pass when the gate
   landed, and a gate nobody can pass is a gate everyone learns to ignore.**
   The backlog clears as each crate is next touched rather than as one act
   nobody owns.

   **Issue #471 is the register and this file keeps no census.** A count
   written here is stale by the next act and then argues with the command:
   the table that stood here through 2026-09-06 named seven failing crates
   and cited issue #475 as its largest item, and by 2026-09-07 #475 was
   closed and the table was naming crates that had since cleared. Measure
   rather than read:

   ```bash
   for c in $(ls crates); do
     if out=$(cargo clippy -p "$c" --all-targets --message-format=short \
                -- -D warnings 2>&1); then
       printf '%-18s %s\n' "$c" 0
     else
       n=$(printf '%s\n' "$out" | grep -cE '^crates/.*: error:') || true
       [ "$n" -eq 0 ] && n=BROKEN
       printf '%-18s %s\n' "$c" "$n"
     fi
   done
   ```

   **`BROKEN` means the run failed for a reason that is not a lint** and the
   crate's gate is unknown rather than passed. It is separated because a
   loop that counts lint lines out of a pipe reports the exit status of
   `grep` and prints a clean zero for a run that never linted, which is how
   issue #471's stale gate command went unnoticed. That command's
   `--features weaver-spu/inference` errors on its first argument, and
   through a counting pipe it reads as clean.
   **The zero a broken run prints is the most expensive line in this
   section**, so it prints a word instead.

   **It counts the source lines the lint names**, so a finding whose path
   clippy prints relative to the crate rather than the tree is not in the
   count. That is not the case in this tree today, and it is where to look
   first if a crate you know is dirty reads zero.

   **The count is per box and this file records none.** Two seats ran that
   loop against `704bb3d` on 2026-09-07 and got different answers, the
   thinkpad seat four findings and the olympus seat five. The one they
   disagree on is `mhdr.msg_controllen as usize` at
   `crates/weaver-harness/src/channel.rs:452`, and the lint that names it
   fires only where that cast is a no-op, which is a property of the target's
   headers rather than of the tree. **Why the two boxes differ is not
   established here** and is #471's to settle. What the disagreement settles
   already is that **a count is a reading taken on a box**, which is the
   second reason it does not live in this file, and #471 carries each reading
   with the seat that took it.

   **The workspace sweep is not the gate and under-reports it.** A crate that
   fails does not compile under deny-warnings, so its dependents are not
   linted at all and `--workspace` answers a smaller question than twelve
   per-crate runs do.

6. **The census**, on the operator's ruling of 2026-09-11.
   `python3 process/gates/census.py`, run from the repository root wherever
   clippy and fmt are run - **before the first review and again after the
   rework**, since a fix is an act and can regress what it is fixing.

   **It counts what the other five devices structurally cannot see.** Each of
   those verifies an artifact against itself: a test against its code, a lint
   against its crate, a compile pin against its types. **None compares a claim
   in a document against a fact in code.**

   ```text
   dangling_citations                      a citation naming no node
   uncited_perturbations                   a tag claiming an instrument
   untagged_assertions                     a node no tag query can reach
   unknown_tags                            a tag outside the format's five
   duplicate_node_ids                      one identifier, two declarations
   malformed_node_ids                      a declaration this gate cannot read
   malformed_citations                     a citation this gate cannot read
   enforcement_table_mismatch              a document against its own table
   documents_without_an_enforcement_table  the table a row is owed in
   sources_without_a_header                phase three's rule, per unit
   ```

   **`WeaverTools-Working-Process` section 6 owns the rule**, as H6, and this
   file carries the invocation and not a second copy of it: two authorities
   for one gate is the duplication G5 refuses. **The rule is section 6's, in
   full, and is not restated here** - the first form of this sentence said so
   and then restated three of its clauses, which is the condition it claimed
   to have removed. `process/gates/test_census.py` is the gate's fixture and
   runs beside it.

   **Issue #558 is the backlog** and records how it came about: nineteen of
   the first thirty-three uncited perturbations were born in documents-only
   commits, the Spec authoring an assertion that phase three would code later,
   with nothing holding the receipt.

   **It has been wrong twenty-two times and its docstring lists every one**,
   which is worth reading before trusting a number it prints. The shape
   repeats: a regular expression too strict about where text sits, printing a
   count that is confidently too low. `\w` does not match a hyphen, so
   `compile-pin` and `compile-fail` read as untagged and **fifty four of them
   were published in this file as a defect count**. A graph block declares
   several nodes and the first form read one, calling seventy sound citations
   dangling. Declining to read a crate's `tests/` moved fifteen perturbations
   into the uncited column, **citing and owing a header being different
   questions** that one walk was answering. There are no untagged assertions
   in this corpus.

Every real defect found in the quarry's final week came from items 2-4, while
`gate-check.py` returned 0 findings on four consecutive PRs and the graph returned zero
code defects while accumulating 53 dangling edges of its own. A clean automated gate is
evidence the gate did not fire, not evidence of correctness.

## The pull request path

All pull requests open as drafts. A draft PR goes to the code review seat, a sub-agent
invoked under the review skill. The sub-agent posts its review to the pull request
before the coding session acts on it, and hands the same report to the coding session.
Posting first is required, so the record carries the finding as it stood, whether it
was fixed or argued down.

The coding session answers each finding with a commit. More than four review rounds
with the seat means the diff is not the problem. The pull request is pulled and the
work re-enters authoring.

**The seat reviews twice, and the second pass reviews the rework.** Answering
fifteen findings is itself an act, and on 2026-09-11 it introduced a real defect
in three pull requests out of four: a transaction that was not a snapshot, a
sweep whose claim was measured against one marker of several, and a newtype that
held its kind for the compiler and not for the value. **Each was found by the
pass that came after the fixes**, not by the one that found the original
defects. So a second pass is not optional where the first produced substantive
work; where the first returned nothing actionable, a second is ritual.

**Passing means no finding that changes behaviour or corrects a claim is
unanswered.** A declined finding is answered - with the reason on the pull
request, since the record carries the finding as it stood either way.

**The reviews are cheap in the resource that is scarce.** A pass runs in a
sub-agent, so its own hundred-odd thousand tokens never enter the session's
window and only the findings do. **Do not spend a review pass on what the
census counts**: a reviewer's attention on "is this perturbation cited" is
attention not on "does this fix hold", and the first is deterministic.

The order, then: gates including the census, first review, answer every finding,
gates again, second review, then out of draft.

A pull request leaves draft only when the code review seat passes it. Leaving draft is
what invokes CodeRabbit, which is the final pass and is expected to confirm rather
than to find work. Two exchanges with CodeRabbit is the ceiling. A third means the
draft phase did not finish, so the pull request returns to draft and the seat works it
again before it comes back out.

## Command output is context, and the session pays for it

**On the operator's ruling of 2026-09-11, after a session spent twenty-two
percent of a one-million-token window on the output of its own commands.**
Not on the work, and not on the conversation: on `cat`, on full test runs, on
`psql` dumps, and on re-reading files already in the window. The corpus is
large and a session that reads it carelessly runs out of room to think.

**Verbose network and database output goes to a file, then the file is
queried.** A result held once on disk can be grepped ten times for nothing,
where a result printed to the session is paid for once and then paid for again
in every later turn that carries it. Write to the scratchpad, report the count,
read back only the rows that matter.

**Cargo writes its diagnostics to stderr, so a pipe without `2>&1` discards
what it claims to filter** and prints a clean nothing whether the command
succeeded or failed. The enforcement section above spends a paragraph on this
exact failure and calls the zero it prints the most expensive line in that
section. The first form of this block dropped the redirect from three of its
own examples.

```text
cargo test -p <crate> 2>&1 | grep -E '^test result|FAILED'
cargo build 2>&1          | grep -E '^error' -A4
cargo clippy -p <crate> --all-targets --message-format=short -- -D warnings 2>&1 \
                          | grep -cE '^crates/.*: error:'
git diff                  --stat first; the full diff only for the hunk in hand
a listing                 aggregated - uniq -c, awk totals - never row by row
a file already read       sed -n 'X,Yp', never cat
```

**A count from a pipe is still not the command's verdict**: a crate that fails
to compile emits no `test result` line at all, so the grep prints nothing and
nothing reads like success. Check the exit status where the answer matters.

**Never `git checkout --` a file to undo an experiment.** It restores the
index, and an uncommitted rewrite in that file is gone. Copy the file aside
and copy it back; this session destroyed one that way on 2026-09-11.

**Grep the narrowest thing that answers the question.** `grep -c` where a
count settles it. A path rather than a tree. One section of a Spec rather than
the Spec, which at two and a half thousand lines is most of a percent of the
window each time it is opened.

**This is a discipline and not a tooling gap.** A retrieval index over the
corpus would cut the document half of it, and is wanted for other reasons -
but the command output above is the session's own doing and no index touches
it.

## Conventions carried from the quarry

- **Editorial: ASCII only, no em-dashes** (use ` - `) in docs and handoffs.
- **Dates are absolute** (`2026-07-28`), and docs carry a dated reconciliation banner.
- **Forbidden vocabulary:** no Id/Ego/SuperEgo/Freudian framing in prose or code. Canonical
  terms are `trace` / `reflection` / `substrate-state`.
- **`latency is the enemy of agency`.** Prefer the shorter abstraction. Internal traffic
  uses Unix sockets, never the network stack. Default to subprocess CLI over MCP - the
  JSON-RPC and stdio buffering cost compounds across hundreds of tool calls per session.
- **OPSEC / publish boundary.** The open-core plan extracts the SPU as a separate public
  crate, so the guard is the *publish* boundary: no commercial, GTM, or strategy material
  and no single-operator-vs-multi-tenant distinction in anything destined to be published.
  **Check visibility, never assume it.** On 2026-08-03 this file asserted both repos were
  private while `toddwbucy/WeaverTools` had been public since its creation on 2026-07-28.
  By 2026-08-24 both `WeaverTools` and `Weaver-Web` were PRIVATE again - the state has
  now changed twice, which is the rule's whole point: a dated assertion in this file is
  a record, never a current fact. One command settles it:
  `gh repo view toddwbucy/WeaverTools --json visibility`.
