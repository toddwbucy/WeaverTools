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
sha256, precision, device name, driver version, build flags, commit.

Run:

```
python3 confirm_cells.py --config thinkpad.json --outdir <deposit>
```

Requires: the box's sudoers fragment for the three admin verbs (the
script drives unload and load), the artifacts named by the config in
place, and an agent whose declaration the config names. The script
backs the declaration up and restores it on exit, and leaves the agent
unloaded when done.
