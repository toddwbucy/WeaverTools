# The Jacobian lens, first light

The protocol for issue #386's read half, run against the column seam PR #398
landed: fit the upstream lens for Qwen2.5-0.5B-Instruct, then read a
diagnostic record's `residual.column` capture through it.

## The pieces

- `fit_lens.py` - fits `J_l` per layer with `anthropics/jacobian-lens`
  (cloned at `/dbpool/experiments/jacobian-lens`, venv `/fastpool/venvs/jlens`)
  on the HF bf16 weights over 200 wikitext-103 articles, deterministic
  selection, and writes the lens beside a manifest carrying the weights
  sha256, the corpus hash, the implementation revision, and the environment -
  the identity discipline the capture-artifact paper formalizes.
**The reading moved into the crate on 2026-09-01.** `read_columns.py` and
`compare_columns.py` proved the shapes this directory's measurements
elected, and `weaver-analysis` now owns both:

    weaver-analysis lens <capture> --lens <artifact> --weights <model>
    weaver-analysis compare <capture> <capture>

with the same discipline papered rather than scripted - the manifest
judged whole before anything loads, the header's names before its data,
the control gating the trajectories, and the comparison exact and never
truncating. The scripts are retired rather than kept beside it, one
implementation of one reading being the point.

## What the first read read

Run 5's archived record (`null-replay-olympus/logs/`, the certified q8
replay with every election on): 455 columns at 24x896. The lens is fitted
on bf16 and the capture is q8_0 - the quantization mismatch is deliberate
for first light and the control quantifies it. An exact-weights pass wants
a bf16 source trace recorded by the current stack: the archived bf16 cell
predates the seated-prefix act, and `weaver-analysis derive` refuses it
naming `identity` - the claim-relative rule doing its job on a pre-member
record.

## Custody

The lens artifact and manifest land at
`/bulk-store/weaver-testing/jacobian-lens-0.5b/`. Vectors are read from the
archived record and nothing new is stored, per the stream-and-discard
design.
