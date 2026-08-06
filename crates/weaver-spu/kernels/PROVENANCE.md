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
| A6000 pair, Ampere | CUDA 13.0 | yes, `sm_86` | yes, 14 tests | yes, three device tests |
| RTX PRO Blackwell laptop | CUDA 13.3 | yes, all four lines | not run | not run |

On the Blackwell box the linked archive was read back with `cuobjdump`: both
members of `libweaver_cuda_kernels.a` carry native SASS for `sm_86`, `sm_89`,
and `sm_120`, plus the `compute_86` PTX fallback. That is a stronger fact than
a clean nvcc exit, because it says the code for each target is present in the
artifact that links rather than merely that the compiler accepted the flags.
The Ada line has no machine behind it and rides on that evidence alone.

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
