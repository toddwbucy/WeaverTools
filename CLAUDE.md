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
in either direction, ever. The quarry is a **parts source you read and never edit** — no
commits, no branches, no fixes there, however tempting. Its last commit is
`d9366d5` (2026-07-28).

The old tree's `CLAUDE.md` and `docs/CLAUDE.md` load automatically when you work inside
`WeaverTools-archived/`. They are accurate about *that* tree and stale about the program's
direction — they describe a 12-crate workspace with a memory leg and a conformance graph,
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
anywhere in either tree — ask the operator rather than inferring it from the empty
`WeaverTools/` repo existing.

## The mission and the carry rule

Deliverable: **a deployable proto-stateful agent that emits a clean, turn-bracketed,
correctly-custodied trace.** The trace is the primary artifact, not a diagnostic — that
reframing is what promotes quarry issues #340/#343/#344/#363 from debt to blockers.

**Proto-stateful, not stateless.** The human's ruling of 2026-08-01 retired "stateless" as
an overstatement, and `WeaverTools-PRD` section 2 is the authority. The agent holds real
state *within* a session and none *across* sessions. **Two things hold state across turns
inside one session, both deliberate and not two things of a kind:** the working structure,
the run's trace events held in RAM in the canonical form the stream carries, volatile by
construction; and the hot KV cache, an optimization whose owner, flush trigger, and
forbidden touchers are named in `weaver-spu-PRD`. Lose the first and turn two has nothing
to be about. Lose the second and the agent is slow rather than absent. If you meet
"stateless" anywhere in this workspace outside a record of the rename, it is stale.

Anything crossing from quarry to new tree goes through exactly one of two doors, and you
**state which door and why at the moment you carry it**:

1. **Live code** — a proto-stateful agent provably needs it, meaning you can name the path a
   single completed turn takes through it.
2. **Stub** — a named joint the memory leg will bolt onto, *and* a written memory-leg
   design already names it. No document, no crossing. Without that constraint door two
   becomes the baggage door and every individual stub still looks principled.

Everything else stays in the quarry. **Nothing crosses because the old tree has it.**

In scope: `weaver-spu`, `weaver-harness`, `weaver-gate`, `weaver-admin`, plus
`weaver-trace`/`weaver-types`/`weaver-traits`. None come over verbatim.
`weaver-traits` and `weaver-types` are **demand-derived** — built from what the SPU and
harness turn out to need, never carried and pruned. `weaver-trace` is the exception:
**designed** against what the memory leg will later read, because demand-derivation
under-builds a deliverable.

Out entirely: the memory leg in any form, `weaver-memory`, the quarry's
`weaver-analysis`, `weaver-train`, `weaver-frontend`, and `weaver-interface`.
**That name was reused on 2026-08-24** and the two are unrelated: the
exclusion above names the previous program's crate, and `weaver-analysis-PRD`
charters a new one, the diagnostic consumer outside the agent boundary that
preloads a replay and reads what it produced. Nothing of the quarry's crossed,
and the reuse is recorded here rather than left for a reader to trip over,
because one spelling of two things is the defect Document Format section 2
names with the halves swapped. The composition root is
deliberately **new code** — it is where the session boundary gets enforced, and where quarry
issue #350 (the agent worker implements no task executor) gets solved rather than
migrated.

Order of work: SPU → trace → harness + new composition root → admin and gate → deployable
proto-stateful agent → autonomic calculator tool → then memory.

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
3. **Phase three, coding.** Open, gates H1-H5 in force per Working Process
   section 6. The floor (`weaver-traits`, `weaver-types`) and the recorder
   (`weaver-trace`) are the first acts; every source file carries a
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

**An assertion record is the middle term of apex section 11's `code → assertion → doc`
chain.** Each names a claim a Spec makes and tags the instrument that holds it:
`compile-pin`, `compile-fail`, `perturbation`, `manifest`, or `review`. Two rules earned
the hard way and worth knowing before you touch one: **a tag follows the mechanism the
clause names, not the heading it sits under**, and **`review` must mean an instrument was
not bought, never that none exists** — the inverse overclaim forecloses tests the corpus
may later want.

**Gates G1–G7 run on every act** (mechanical, level discipline, graph facts, vocabulary,
duplication authority, extraction completeness, rulings landed). H1–H5 are phase three
candidates and are not in force.

**A ruling is a claim about the whole corpus.** A review finding names one sighting of its
violation, so an act that lands a ruling ends with a corpus-wide sweep for every wording
the ruling retires — and the sweep must be whitespace-normalized, because prose wraps at 88
columns and any phrase can straddle a break. This file is the standing proof of what
happens otherwise: the 2026-08-01 rename swept the corpus clean and left `CLAUDE.md`
behind, because `CLAUDE.md` was not in the tree. **It entered the tree 2026-08-24**, at
the repo root with the workspace copy a symlink into it, precisely so that corpus-wide
sweeps reach this file too.

## This machine is not the deployment box

The quarry's own `CLAUDE.md` documents runtime paths (`/opt/weavertools` source,
`/opt/weaver` installed runtime) that **do not exist here**. Consequences:

- The quarry is cloned to a home directory. Per-agent OS users cannot traverse a 0700
  home, so nothing agent-facing can actually run from this checkout — it is a reading and
  planning workspace.
- `.hades/` is gitignored and absent, so `gate-check.py` cannot run here. The quarry's
  mandatory merge-gate sequence is not executable from this machine.
- `nvidia-smi` is present but there is no `nvcc` on PATH; `--features cuda` will not
  compile here.
- The pinned toolchain (`nightly-2026-02-13`, rustc `47611e160`) is installed and matches
  `rust-toolchain.toml`.

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
fork pin (`a67e208`, exposing the ggml scheduler eval callback — the only route to
per-layer activations from a GGUF model) was the stated precondition for cutting the
extraction, and it **is** in the quarry's `main`.

`crates/weaver-frontend` is excluded from the workspace and needs X11/Wayland/GL dev
libs; build it from inside its own directory if at all.

## Orienting in the quarry

Sizes matter here — the carry rule is a subtraction discipline and roughly 90k lines
are in scope for consideration; `wc -l` over `crates/<name>/src` gives the current
figures when you need them.

Reading order for architecture: `docs/weavertools-HAH-v41.md` (the hypothesis this whole
apparatus tests), `docs/weavertools-primary-PRD.md` (the apparatus apex),
`docs/crate-topology-Spec.md` (the doc↔crate map). Per-crate PRDs and Specs are at
`docs/architecture/crates/<crate>/`, mirroring `crates/<crate>/` positionally.
`docs/project/handoffs/` and the dated `HANDOFF-*.md` files at `docs/project/` are the
narrative of how each subsystem reached its frozen state.

Design patterns worth carrying forward conceptually (they are the quarry's real
contribution, independent of its code): per-invocation tool safety classification
(`Tool::invocation_properties(input)` inspects the *actual* command — `ls` reads,
`rm -rf` destroys — which drives parallel-vs-serial batching), events-as-rendering-API
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
   because the signal worth learning is `code → assertion`, and the quarry is no bootstrap:
   25 files carry a conformance header and they cite 7 distinct spec node ids. GraphSAGE is
   the right family because it is inductive, so code nodes added at merge time get
   embeddings with no retrain.

The quarry's own graph (`weavertools_v2`, ArangoDB) is the cautionary case, not the
counterargument. At freeze its bilateral-contract certificate had 0 edges and its axiom
basis covered 7 of 71 claims — the two things a graph uniquely provides had never been
delivered. **The structure was right and the edges were never drawn.** The new program's
guard against repeating that is a rule the operator settled before any labelling began:
an assertion that grounds in no invariant is **representation, not an omission**, and the
coverage number is a fact to read rather than a target to reach. Writing that down first
is what stops a low number from being argued away once someone sees it.

**During authoring, enforcement rests on four devices and no graph.** These do not retire
when the graph lands — the graph indexes them, it does not replace them:

1. Conformance trace headers in source carrying `code -> assertion -> doc`.
2. **Compile-time pins** for invariants that are type properties. A runtime test
   structurally cannot pin the *absence* of a trait impl.
3. **Perturbation-verified tests** for invariants that are behaviours. Always confirm the
   test fails when the property is removed — a test that passes either way converts
   "unenforced" into "documented as enforced", which is worse than no test.
4. Human and CodeRabbit review. Read the review **body**, not the thread count: CR posts
   findings outside the diff range that create no thread and are absent from the
   "actionable comments" total.

Every real defect found in the quarry's final week came from items 2–4, while
`gate-check.py` returned 0 findings on four consecutive PRs and the graph returned zero
code defects while accumulating 53 dangling edges of its own. A clean automated gate is
evidence the gate did not fire, not evidence of correctness.

## Conventions carried from the quarry

- **Editorial: ASCII only, no em-dashes** (use ` - `) in docs and handoffs.
- **Dates are absolute** (`2026-07-28`), and docs carry a dated reconciliation banner.
- **Forbidden vocabulary:** no Id/Ego/SuperEgo/Freudian framing in prose or code. Canonical
  terms are `trace` / `reflection` / `substrate-state`.
- **`latency is the enemy of agency`.** Prefer the shorter abstraction; internal traffic
  uses Unix sockets, never the network stack. Default to subprocess CLI over MCP — the
  JSON-RPC and stdio buffering cost compounds across hundreds of tool calls per session.
- **OPSEC / publish boundary.** The open-core plan extracts the SPU as a separate public
  crate, so the guard is the *publish* boundary: no commercial, GTM, or strategy material
  and no single-operator-vs-multi-tenant distinction in anything destined to be published.
  **Check visibility, never assume it.** On 2026-08-03 this file asserted both repos were
  private while `toddwbucy/WeaverTools` had been public since its creation on 2026-07-28.
  By 2026-08-24 both `WeaverTools` and `Weaver-Web` were PRIVATE again — the state has
  now changed twice, which is the rule's whole point: a dated assertion in this file is
  a record, never a current fact. One command settles it:
  `gh repo view toddwbucy/WeaverTools --json visibility`.
