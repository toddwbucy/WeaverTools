# WeaverTools

The suite-level repository of WeaverTools: what belongs to the suite as a whole rather
than to one of its code repositories. WeaverTools is a local-first agent framework whose
primary artifact is the trace, split on 2026-09-30 into repositories along its network
boundaries, one per party a consumer meets across a socket:

| Repository | What it holds |
|---|---|
| [WeaverAgent](https://github.com/toddwbucy/WeaverAgent) | The agent: harness, SPU, gate, admin, state, trace and the crates beneath them, with their charters, Specs and contracts. |
| [WeaverAnalysis](https://github.com/toddwbucy/WeaverAnalysis) | The diagnostic consumer: the tools that read the record the agent's admin hands off, derive and preload a replay, and read what comes back. |
| [WeaverWeb](https://github.com/toddwbucy/WeaverWeb) | The frontend: the human's surface over running agents and over what the analysis emits. |
| WeaverTools, this repository | The suite: experiments, and the documents that bind more than one repository. |

The three code repositories came out of one monorepo, kept as
[WeaverTools-old2](https://github.com/toddwbucy/WeaverTools-old2), whose history is the
readable record of how each subsystem reached the state it left in. Each new repository
opens at its own root commit.

## What is here

**`experiments/`**: the suite's pre-registered measurement acts, each an experiment
directory holding a charter (`README.md`), one directory per arm, and under each arm one
per probe with its Spec, `code/` and `results/`. They live here rather than in
WeaverAgent because their code reads WeaverAnalysis as much as it reads the agent: the
determinism matrix under `deployment-tuple/baseline/` drives an installed agent stack and
checks the declaration WeaverAnalysis derives, and its grammar test reads a WeaverAnalysis
fixture by path.

- `experiments/deployment-tuple/`: the hypothesis that a deployment's output is
  reproducible exactly when six declared fields are held. Arms `baseline/`, `device/`,
  `kernel-stack/`. Its charter is the README a reader opens first.
- `experiments/trace-content/`: the trace-content classifier work, its label split,
  HeroBench agents and classifier evaluation.

A run's evidence lives in its deposit on the shared bulk store, and the repository
carries only the result note and the scripts that produced its figures. A result note
names its deposits and copies no data from them.

**Still to come**, on the operator's word: the cross-repository contracts
(`weaver-gate-world`, `weaver-admin-operator`, `weaver-analysis-state`,
`weaver-analysis-web`), the suite vision, the four process documents, and the paper
material. Until they land, the contracts stand in WeaverAgent under
`docs/crates/contracts/` and the process documents under `process/`.

## Running the experiments' code

Stdlib Python, no dependencies. The harness code expects the suite's repositories as
siblings in one directory, which is how the workshop is laid out:

```text
<workshop>/
  WeaverAgent/
  WeaverAnalysis/
  WeaverWeb/
  WeaverTools/        this repository
```

Where they are not siblings, `WEAVER_ANALYSIS_DIR` names the WeaverAnalysis checkout.
Each probe's `code/README.md` says how to run it on a box, and each harness's
`test_*.py` files are plain scripts that exit non-zero on the first failure:

```sh
cd experiments/deployment-tuple/baseline/determinism-matrix/code
for t in test_*.py; do python3 "$t" || break; done
```

## Editorial

ASCII only; absolute dates; each election's reasoning stated where it is made;
superseded text removed, since git is the archive; commit subjects `code:`, `docs:`,
`process:`. The repository is public: no commercial or strategy material enters it.
