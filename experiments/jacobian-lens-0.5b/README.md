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
- `read_columns.py` - parses a diagnostic record, pairs each
  `residual.column` with the token its position drew, and prints the layer
  trajectory `unembed(J_l @ h)` at chosen positions. **The control runs
  first**: at the final layer, `unembed(h_final)` with no transport must
  rank the actually-drawn token at or near top-1 across every position,
  which validates the position pairing, the layer convention, and the
  engine-vs-HF numerics in one number before any lens claim is made.
  **The control gates the trajectories**: below `--min-top5` (default 0.9)
  nothing is printed and the exit is nonzero naming the rate, the
  no-reading-from-an-uncertified-replay rule one level down. The reader
  also refuses a lens whose manifest names other weights (sha256 recomputed
  against the model in hand) and a record whose columns pair with no
  measurement, and it scopes every pairing by turn so a multi-bracket
  record cannot alias positions across passes.

## What the first read reads

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
