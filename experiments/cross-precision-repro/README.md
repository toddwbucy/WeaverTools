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

- **The harness feature set, not only the SPU's.** `weaver-harness/pyworker`
  selects `pyworker` and `py_loop`, where its absence selects `worker`
  and `dev_loop`, which seats a compiled system prompt the other does not.
  Two boxes at one commit ran different prompts for a week on this.
- **The cccl version, and the kernel it selects.** `argsort.cu` and
  `top-k.cu` guard on `CCCL_MINOR_VERSION`, so the host's cccl decides
  CUB against fallback kernels. Record the versioned namespace read from
  `argsort.cu.o`, which carries the CCCL version and the architecture
  list in one string.
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
