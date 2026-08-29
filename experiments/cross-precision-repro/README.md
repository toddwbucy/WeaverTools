# Cross-precision reproducibility cells

The scripted form of the lab confirmation recorded at
`docs/technical/weaver-agents/reproducibility.md`, extended across
precision and, when a second box runs it, across architecture. The
manual reissue was the one weakness the first result named, so this
harness drives the whole protocol: serve a short and a longer turn,
unload fully, reload, read the request texts back from the record's
own `message.user` events, reissue them byte-exact at the gate socket,
compare field by field, and deposit the runs beside the report.

The pinning discipline is the point. Same commit, same declaration
apart from the artifact path, same declared seed, same turn texts
(constants in the script, never edited per box). A box that needs its
own build records that build's flags as an arm of the experiment
rather than as a nuisance. The report carries per cell: artifact
sha256, precision, build flags, commit, the toolchain in force at the
repository, the serving device read from the worker's own load for both
the source and the replay half, the sha256 of the worker, SPU, and gate
binaries, and the sha256 of the engine libraries the SPU links.

**What `build_flags` must carry, and why the list grew.** A commit does
not determine a runtime, and every field below was found the hard way -
each one silently decided a result while both boxes recorded their
builds in good faith:

- **The `worker-binary` value, which is what selects the loop.** Its
  basename is `pyworker` or `worker`, and that chooses `py_loop` against
  `dev_loop`, which seats a compiled system prompt the other does not.
  Two boxes at one commit ran different prompts for a week on this.

  **The cargo feature is not the selector and recording it is not enough.**
  `weaver-harness/pyworker` gates whether `pyworker` compiles, and a
  feature-on build ships **both** binaries. Which one an agent reaches is
  the operator's provisioning through the `worker-binary` key, read at
  `weaver-admin/src/main.rs:838` and stated at `pyworker/main.rs:5`. A box
  that builds with the feature, records it, and leaves `worker-binary`
  naming `worker` runs `dev_loop` while its deposit reads as a pyworker
  box, which is this defect reached by way of its own fix.

  **The report already carries it.** `weaver_binaries["worker-binary"]`
  has held the path and sha256 since #375, in every deposit from 08-27.
  So this wants comparing rather than writing down, and the gap that cost
  a week was that nothing compared the two records.
- **The cccl version, the kernel it selects, and the architecture list -
  three fields, not one string.** `argsort.cu` and `top-k.cu` guard on
  `CCCL_MINOR_VERSION`, so the host's cccl decides CUB against fallback
  kernels. All three are readable from the versioned namespace in
  `argsort.cu.o`, for example
  `cub::_V_300304_SM_750_800_860_890_1200_1210`, **and they must be
  compared under different rules**, so recording them joined is a record
  a comparator cannot use:

  | field | example | rule |
  |---|---|---|
  | cccl version | `300304` | **must match** across boxes |
  | kernel selection | CUB argsort, CUB top-k | **must match** across boxes |
  | architecture list | `750 800 860 890 1200 1210` | **may differ**, and does |

  The architecture list is the only one of the three a cross-box
  comparison may see differ. Joining it to the other two produces a
  string that differs whenever the hardware does, which reads as a cccl
  mismatch and is not one.
- **`NVCC_CCBIN`.** The host compiler moves the emitted symbol set,
  measured here as nine symbols between GCC 15.3.0 and 16.2.1.
- **Which directory the engine libraries were installed from.**
  `out/lib` and `out/build/bin` hold the same sizes and different bytes -
  the latter keeps a build-directory `RUNPATH`.
- **Whether the state leg stands**, which is decided by whether
  `weaver-state` sits beside `worker-binary` and by no configuration key.
  A standing leg gives a replay a continuity line the source never had.

**The architecture list is expected to differ between boxes and is not a
defect.** A Blackwell box compiles `sm_120a` and an Ada box does not, so
`libggml-cuda.so` cannot be byte-identical across them. What must match
is the source pin, the cccl version, and the kernel selection read from
the object - never the bytes.

**Compare the rendered prompt before trusting a cross-box cell.** It is
in the record, on both sides, so it is a measurement rather than an
inference. Two boxes agreeing on commit, declaration, seed and artifact
still disagreed on turn one.

The device and the binaries are the fields a cross-box comparison rests
on. `nvidia-smi` names the machine rather than the run, so a box holding
more than one card reported every device and named none, and a commit
cannot say whether two boxes built the same bytes. Any field that could
not be read carries `unreadable` and its reason rather than an empty
value.

Run:

```
python3 confirm_cells.py --config thinkpad.json --outdir <deposit>
```

Requires: the box's sudoers fragment for the three admin verbs (the
script drives unload and load), the artifacts named by the config in
place, and an agent whose declaration the config names. The script
backs the declaration up and restores it on exit, and leaves the agent
unloaded when done.
