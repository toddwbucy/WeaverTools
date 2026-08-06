# Provenance of the salvaged kernels and their fixtures

`weaver-spu-Spec` section 10: **the kernels cross verbatim and their tests cross
with them.** The CUDA kernel set and its build script are carried unchanged, per
the operator's ruling of 2026-08-02 and the carry rule's first door, and the
golden fixtures that compare each kernel against a candle reference come with
them, because a salvaged kernel with its comparison left behind is a kernel this
program has not checked.

That claim is tagged `review`, and **it is review's by reach rather than by
election**, which the Spec calls the rare case: what the fixtures assert about a
kernel fires when the kernel drifts, but no suite can watch a comparison that was
left behind, a test being unable to detect its own absence. Whether the
comparisons crossed is therefore a fact about the carry that a reader
establishes and a runner cannot. This file is what the reader reads.

## What crossed, and its hash at the moment of carry

Source tree: `/opt/weavertools/WeaverTools-archived/crates/weaver-spu`, the
quarry, which is read-only as a discipline. Nothing was edited in transit and
the hashes below are the check, not the assurance.

| Carried to | From the quarry at | sha256, first 16 |
|---|---|---|
| `kernels/transformer.cu` | `kernels/transformer.cu` | `80958f743cd7b4e9` |
| `build.rs` | `build.rs` | `da39cd00c9deaaa6` |
| `tests/fixtures/gptoss/rope_tables.txt` | same path | `49a8c355de2f5447` |
| `tests/fixtures/gptoss/sink_attn.txt` | same path | `18d5d7ba14432bb7` |
| `tests/fixtures/gptoss/swa_attn.txt` | same path | `70cbb04de4a8adc7` |
| `tests/fixtures/gptoss/yarn_rope_l0.txt` | same path | `12d4a7d0a1b34daf` |
| `tests/fixtures/gptoss/mxfp4_gate_up_l0e0.txt` | same path | `b40322380629c8fd` |

Verify with `sha256sum` against either tree while the quarry stands. Phase two's
closing checklist removes the quarry, which is why the values are recorded here
rather than left to be recomputed from a tree that will not exist.

## Building the kernels, and the one environment fact

The compile is gated on the `cuda` feature. `build.rs` reads
`CARGO_FEATURE_CUDA` and returns without doing anything when it is absent, so
the no-feature build needs no CUDA toolchain and the suite runs on a machine
with no device.

**The carried script defaults `CUDA_PATH` to `/usr/local/cuda`, and that is
wrong on some machines.** It falls back through `CUDA_PATH`, then `CUDA_ROOT`,
then that literal. A CachyOS box keeps the toolkit at `/opt/cuda`, so the build
needs the variable set:

    CUDA_PATH=/opt/cuda cargo build -p weaver-spu --features cuda

This is recorded rather than fixed because the script crosses verbatim, per the
ruling of 2026-08-02. Changing the default would be an edit to a carried file,
and the fact belongs with the carry either way.

The four `-gencode` lines are `sm_86` (A6000, Ampere), `sm_89` (RTX Ada),
`sm_120` (RTX PRO Blackwell, needing CUDA >= 12.8), and a `compute_86` PTX
fallback that JITs to any architecture at or above 86.

**What has been verified, and on what, 2026-08-06.** Two machines, and the
coverage they give is uneven in a way worth stating rather than averaging.

| Machine | Toolkit | Compiles and links | Suite runs | Device-side |
|---|---|---|---|---|
| A6000 pair, Ampere | CUDA 13.0 | yes, all four lines | yes | yes, three device tests |
| RTX PRO Blackwell laptop | CUDA 13.3 | yes, all four lines | not run | not run |

On the Blackwell box the linked archive was read back with `cuobjdump`: both
members of `libweaver_cuda_kernels.a` carry native SASS for `sm_86`, `sm_89`,
and `sm_120`, plus the `compute_86` PTX fallback. That is a stronger fact than
a clean nvcc exit, because it says the code for each target is present in the
artifact that links rather than merely that the compiler accepted the flags.
The fatbins on the two machines are identical, so the compiles-and-links column
is the same fact on both rows: every build emits all four lines, and which line
a machine can execute is the device-side column's question. The Ada line has no
machine behind it and rides on the `cuobjdump` evidence alone.

**No device-side execution has happened on Blackwell.** The kernels are known to
be present and linkable there and are not known to produce correct numbers
there. That gap closes when the comparison code below crosses, not before.

## What has not crossed yet, named so the gap is not read as completeness

The comparisons themselves live in the quarry at `src/core/gpu/kernels.rs`,
whose test module drives each kernel through its cudarc FFI launcher and
compares the result against the candle reference held in the fixtures above.
**The fixtures crossed in this act and the code that reads them did not.** Until
that module crosses, the fixtures are data no test opens, and the claim this
file exists to let a reader establish is therefore only half established: the
comparisons' inputs are here, the comparisons are not.

That is a stated gap rather than a discovered one. It is named here, and in the
crate's open items, so that a later reader checking the carry finds the answer
recorded rather than inferring completeness from the presence of the fixtures.
The remaining carry is the `src/gpu/` volume of the Spec's layout, which needs
the `cuda` feature, `cudarc`, and a device to run against.

## Known defects in the carried material, inherited verbatim

Review of 2026-08-06, two independent passes, findings verified by reading the
carried sources. The carry rule keeps these unfixed here: the kernels cross
unchanged, so the defects cross with them, and this list is where they are
named so the comparison act that crosses the reader inherits a worklist rather
than a surprise. None of them is reachable today, no launcher having crossed.

- `launch_flash_attention` requests `256 * head_dim + 16384` bytes of dynamic
  shared memory and never calls `cudaFuncSetAttribute`, so any `head_dim`
  above 128 exceeds the 48 KB default and the launch fails, which is exactly
  the case the `FA2_MAX_HALF_DIM` comment claims to support. The register
  array `O_acc` also overruns for `head_dim` above 256.
- Every `launch_*` returns void and never calls `cudaGetLastError`, so a
  launch-configuration failure is invisible to the caller: stale or garbage
  output under a green test, since a later synchronize does not surface
  launch-config errors.
- `attention_output_kernel` assigns one thread per output dimension capped at
  256 with no strided loop, so `head_dim` above 256 leaves the upper
  dimensions silently unwritten on the naive path. The file's own comment
  names Gemma4 global layers at 512.
- `launch_decode_attention` enforces no layout precondition: a `head_dim` not
  a multiple of 32 corrupts shared memory, a `head_dim` above 512 overruns
  two register arrays, and `total_len == 0` divides by zero and writes NaN to
  every output element.
- The rmsnorm tree reduction assumes a power-of-two `blockDim` and the
  launcher passes `hidden_size` directly when it is under 256, so a
  non-power-of-two width under 256 drops elements from the sum. Latent at
  every current call site.
- `--use_fast_math` substitutes approximate `expf`, `tanhf`, and `rsqrtf` and
  relaxes division and square-root precision, while the candle reference the
  fixtures were generated from uses none of those approximations. The
  comparison that crosses later must carry tolerances that absorb that
  offset, and the tolerance belongs beside the comparison when it lands.
- `sink_attn.txt` line 1 reads `4 4 0.5`, and at `head_dim = 4` the derived
  attention scale is exactly `0.5`, so the trailing field cannot be told
  apart from the sink logit without the generator. The fixture crosses
  verbatim regardless: regenerating it needs the quarry's generator, and the
  ambiguity is resolved by that generator's source when the reader crosses.
