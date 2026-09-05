# depth-series

One long single-topic session that fills the window, so the per-token
series the diagnostic leg emits can be read across depth. Consecutive
refined pages of one OCR'd book are the input, three pages a turn, the ask
identical every turn, with the surprisal and readout elections standing at
serving so `weaver-analysis signals` reads the session's own trace slice.

    python3 depth_session.py --config olympus-depth.json \
        --book /bulk-store/books/books/text_output/I_robot_automated \
        --first-page 20 --pages-per-turn 3 --turns 15 \
        --task continuation --max-tokens 640 --outdir <deposit>

Three runs on 2026-09-03 and what each taught, recorded in the driver's
comments and in the testing hub's reading of that date: a summary task
converges on a house style and measures the model predicting itself, the
family's `/no_think` switch leaves the prose inside an unclosed think block
under this stack's template, and a continuation with no stated length loses
to the declaration's own identity, which asks for brevity. The finding that
held on three sessions of three kinds: the body of the series narrows with
depth and the far tail does not.

The deposit is the run's own slice of the trace beside a report carrying
resident per turn, the load event with its elections, both declaration
hashes, and whether the declaration was restored, which it must be.

The config's `loop_sha256` declares the loop by digest, `alpha_loop.py` as
the olympus declaration names it, and the driver refuses the session before
its first turn where the load event's composer records another digest or
none, depositing `"loop_refused": {"declared": ..., "recorded": ...}` and
exiting nonzero rather than a short session. The digest and not the name is
the identity, a config with no key is unchecked, and `sha256sum` on the
deployed loop file gives the value. The cross-precision README is the
authority on the key, per issue #426.
