# Pre-registration: does the arithmetic survive a change of operating system and of build site

Filed 2026-11-15, before any run of these cells, for toddwbucy/WeaverTools#28. The
commit that adds this file is the record of when the prediction was made. Nothing in it
changes after the first run. A change found to be needed is a new dated file beside this
one, and the result note reads against both.

## 1. Question

With the model, the declaration, the probe set, the GPU and the driver release held,
does a turn emit the same token ids and per-token entropies when the binaries are built
on a CachyOS host or in an Ubuntu 26.04 guest, and run on the host or in the guest on
the same physical GPU under passthrough?

The guest is Ubuntu 26.04, not 24.04 as first stated. Every file of the host's build
needs a C library of 2.43 or older and a C++ runtime of `GLIBCXX_3.4.32` or older, read
from the build at WeaverAgent `6b91747e` on 2026-11-15. Ubuntu 26.04 meets that and
24.04 does not.

## 2. Prediction and falsifier

**Prediction.** Every cell below emits the same token ids and the same per-token
entropies, to float exactness, on every turn of the probe set.

**Falsifier.** Any difference in an emitted token id or in a per-token entropy on any
turn of any cell, against any other cell.

## 3. The byte sets

- **H**: the stack built on the host from WeaverAgent `6b91747e`, with decision 6's two
  build settings as `deploy/bootstrap-stack.sh` passes them.
- **G**: the stack built in an Ubuntu 26.04 guest from the same commit, by the appliance
  build's steps through the stack step, with the same two settings.
- **G2** (a byte control, run on a card only if H and G differ in output): G built again
  with the cargo and rustup home paths set to the host's strings.

H and G differ in bytes by design. The shipped files embed the cargo and rustup home
paths, and the two sites use different compilers and system headers. The comparison of H
against G is a comparison of two byte sets. The build facts of each are recorded: the
C++ compiler and its version, `nvcc` and its build string, the Rust toolchain, the CCCL
headers and the C library the build saw. Where those differ, H against G compares those
too, and the result note names each difference.

## 4. The cells, in the order they run

One card moves between the host and the guest. Within each comparison the order is host,
guest, host again, so a drift within one side is visible.

| Run | Cell | Where | Byte set | Driver | Guest CPU |
|---|---|---|---|---|---|
| 1 | H on host | host | H | 620 | |
| 2 | G on host | host | G | 620 | |
| 3 | H in guest | guest | H | 620 | host-passthrough |
| 4 | G in guest | guest | G | 620 | host-passthrough |
| 5 | guest CPU | guest | H | 620 | EPYC-Milan (no AVX-512) |
| 6 | driver branch | guest | H | 615 | host-passthrough |
| 7 | G on host, again | host | G | 620 | |
| 8 | H on host, again | host | H | 620 | |

**What each comparison answers.**

- Runs 1, 3 and 8 (H), and runs 2, 4 and 7 (G): one byte set on two operating systems.
- Runs 1 and 2, runs 3 and 4, runs 7 and 8: two builds of one commit, one operating
  system.
- Runs 3 and 5: one guest, one byte set, two virtual CPU models. The guest CPU joins the
  deployment tuple by the operator's ruling of 2026-11-15. The named model must offer
  AVX2, FMA and F16C, which the engine's CPU library is built for. A model without them
  faults, and that is not a result.
- Runs 3 and 6: one byte set, one operating system release, the 615 driver branch
  against the 620 branch. The guest of run 6 is a second clone of the same image.

Two cells of the issue are not run here, because what they need does not exist on
2026-11-15: the sealed appliance image against the host (a version 1 image), and a
respin against its parent (a version 1 image and its respin).

## 5. What is held, and how each is read

- **The model**, by sha256, the same file on both sides: qwen2.5-0.5b-instruct at q6_k,
  sha256 `2f82233630c349ccf6b8daccf48f9a7865713d9f08a2eadfa456cebe9b97c7f5`. A second
  model, Qwen3-8B at bf16, sha256
  `5e416a2020fe63e76ea13c8979be35fc6070aaf3578f7876400c55c2f5c3eb30`, also runs in runs
  1, 2, 3, 4, 7 and 8 if the operator adds it before run 1. Each run's record says which
  models ran. The prediction and the falsifier hold for each model separately.
- **The declaration**, byte for byte apart from `devices` and the trace path, by sha256
  of each side's file. It declares no state store, so no save point is read into a turn
  or written from one.
- **Where the bytes run from.** On each side, each byte set runs from its own directory,
  owned by root, not from an installed stack. Each run's process map shows which files
  it loaded (section 6).
- **The probe set**, the determinism matrix's prompt-by-depth set, by the git tree of
  the harness the runs use.
- **The GPU**, paired by serial and by UUID on each side, not by bus address. That both
  survive passthrough is confirmed at the guest's first run.
- **The driver release**, by its version string on each side. Each kernel module's
  version and file digest are recorded as a condition: a module is built for its kernel,
  so its bytes differ between sides by construction.

## 6. The mapped libraries

Each run reads the mapped files of the running SPU from its own map, not from the
loader's resolution, and records each file's sha256.

**Expected to differ, because they are the operating system:** the C library, the math
library, the loader, the C++ runtime, the GCC runtime (`libgcc_s`) and the OpenMP
runtime (`libgomp`), and any other file the distribution provides that is not on the
list below. The math library is also inside the computation (the SPU and the engine's
samplers call `expf`, `logf` and their kin on the CPU). It is the first suspect, and a
difference in output is read against it before anything else.

**Must be equal across the cells that hold them:** the byte set's own members and engine
libraries (equal to the byte set the cell names), the CUDA runtime libraries the process
maps, and the driver's userspace libraries.

A mapped file outside the first list that differs where it must be equal stops that cell
before any turn, and the run reports the file. A file on the first list that reads equal
is reported, not ignored.

**Read before any run.** On both sides the CUDA runtime is the same release with the
same bytes. The host has Arch's `cuda` 13.4.3-1. The guest image has NVIDIA's
`cuda-toolkit-13-4` 13.4.2-1. Two clones of that image, on 2026-11-12 and 2026-11-13,
read these files byte-equal to the host's: libcudart 13.4.92, libcublas and libcublasLt
13.8.0.4, and libcurand 10.4.4.72. The host's files have not changed since. So every
cell holds the CUDA runtime equal, byte for byte. The guest's files are compared again
before run 3.

At 620.70.06 the driver's userspace is the same release on both sides, from two
packagers: Arch's `nvidia-utils` 620.70.06-2 on the host, and NVIDIA's Ubuntu packages
620.70.06-1ubuntu1 in the guest. The guest's packages come from a local cache, checked
against apt's hashes. Of the libraries in the two compute packages, libcuda and eight
others are byte-equal. Three are not: libnvidia-gpucomp, libnvidia-nvvm70 and
libcudadebugger. They are named here. If the process maps one of them, its two digests
are recorded and the cell does not stop. Any other mapped file of a held release whose
bytes differ stops the cell.

**Run 6 varies the driver**, 615.71.09 against 620.70.06. In run 6 the driver's
userspace and its kernel modules are the varied member: they are recorded, not held. The
CUDA runtime is held in run 6 as in every cell, because its guest is a clone of the same
image.

## 7. Record per run

The byte set and its build facts, the commit, every mapped file's sha256 from the
running process, the kernel, the C library's version, the distribution, the driver
version and each module's digest, the GPU's serial and UUID, the guest CPU model where
there is a guest, and the matrix's own records.

## 8. Report

As counts, not rates: turn slots run, and turn slots identical and different, for each
pair of runs. The result note names every held field that turned out to differ, since
that field was wrongly counted as held.

If identity holds, the note drafts one sentence for the Deployment paper's rung table
and one for the tuple paper: the kernel, the distribution and the build site are
recorded as provenance, and on this evidence they are not determinants. If it fails, the
note names the held field that differed and the cells that separate it.

## 9. Where the evidence goes

A deposit under `weaver-testing/`. This repository carries the result note and the
scripts behind its figures.
