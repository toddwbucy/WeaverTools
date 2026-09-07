# The determinism matrix

Asks whether the replay instrument holds across conditions, on the easiest
target available, so that a failure indicts the apparatus rather than the
model. It drives the same protocol `confirm_cells.py` drives - serve,
unload, reload, reissue byte-exact from the record, compare field by field -
over a matrix rather than over one pair of turns, and it imports that
harness rather than restating it.

Three axes, chosen for what could plausibly break a replay: prompt character
across the draw's confidence, conversation depth, and generation length.
Repeating one prompt many times exercises kernel nondeterminism and the
allocator and nothing else, which is why this varies instead.

Run:

```shell
python3 determinism_matrix.py --config ../cross-precision-repro/thinkpad.json \
    --outdir <deposit> --hours 7
```

Wall-clock bounded: it finishes the session in hand and stops, so an
overnight run ends cleanly rather than mid-cell. `--artifact` overrides the
declaration's artifact for every cell, and the declaration is restored on
exit whichever path the run takes. The agent is left unloaded, so a cell
that fails never holds the device.

Sweeps rather than repeats: every combination is seen once before any is
seen twice, so a run cut short by the clock covers the matrix rather than
the front of it. Records append to `matrix.jsonl` per session, so a run that
dies keeps everything before it.

The first result is `RESULT-2026-08-27.md`, and it is the baseline the
protoautonomic calculator experiment measures against.

## The clock-state matrix

`clock_state_matrix.py` runs the same matrix with the graphics clock
locked before every load to a seeded draw between 210 and 2100 MHz,
independently on the source half and the replay half, and samples the
card at every dispatch, during the turn, and at its close. It imports
this driver and patches three seams, the admin load, the gate turn, and
the session record, and touches nothing else. An idle between turns
cannot move the clock, because the card holds boost for as long as the
SPU's context exists, which is why the clock is locked rather than left
to drift. The readout is `nvidia-smi dmon -s pc`, never the query path,
which reports 210 MHz at full load on driver 610.57.04. Needs root for
`nvidia-smi -lgc` under `sudo -n`, and resets the clock at exit.

```shell
python3 clock_state_matrix.py --config <cfg> --outdir <deposit> --hours 7
```

The olympus result of 2026-09-07, 2,616 of 2,616 reproduced with 2,293
probe turns reissued under a clock other than the one that produced
them, is deposited at
`/bulk-store/weaver-testing/determinism-matrix-olympus-2026-09-06-clock/`
with `RESULT-2026-09-07.md` beside it.
