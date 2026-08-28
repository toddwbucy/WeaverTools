#!/usr/bin/env python3
"""The determinism matrix: does the instrument hold across conditions?

The cross-precision harness confirmed one protocol once. This one asks
the prior question, and asks it on the easiest target on purpose: with
the smallest artifact and a single precision, a cell that fails is this
apparatus failing rather than the model being unfaithful. Prove the
testing works here, then scale.

**The matrix is over what could plausibly break a replay**, not over
repetition. Repeating one prompt exercises kernel nondeterminism and the
allocator and nothing else. Three axes vary instead:

  prompt character  a confident factual answer draws far from a tie and
                    an open-ended one draws near it, and a near-tie is
                    where a float wobble flips the draw and diverges the
                    whole emission. The confident end is what the first
                    experiment measured.
  depth             a turn at position 2 meets a nearly empty resident
                    sequence, one at position 32 meets a long accumulated
                    one, and the derived seed carries the turn ordinal, so
                    depth moves the seed as well as the state.
  length            short and long generations cross different kernel
                    tile boundaries.

Each session serves its turns, unloads fully, reloads, reissues every
turn byte-exact from the record, and compares field by field. The
entropy of each turn is carried into the report beside its verdict, so a
failure can be read against how near a tie the turn's draw was rather
than guessed at.

Run:

    python3 determinism_matrix.py --config thinkpad.json \
        --outdir <deposit> --hours 7

Wall-clock bounded: it finishes the session in hand and stops, so an
overnight run ends cleanly rather than mid-cell.
"""

import argparse
import json
import os
import statistics
import sys
import time

sys.path.insert(
    0,
    os.path.join(os.path.dirname(os.path.abspath(__file__)),
                 "..", "cross-precision-repro"),
)
import confirm_cells as base

# **The prompt set spans the draw's confidence, which is the axis that
# matters.** Each carries the character it was chosen for, so a reader
# grading a failure can see what the turn was meant to be rather than
# inferring it from the text.
PROMPTS = [
    ("factual-short", "confident",
     "What is the capital of France? Answer with the city name alone."),
    ("arithmetic", "confident",
     "What is 17 multiplied by 23? Give the number and nothing else."),
    ("definition", "confident",
     "Define the word 'photosynthesis' in one sentence."),
    ("explain-long", "mid",
     "Explain how a hash table works, including collision handling, "
     "in about two hundred words."),
    ("code-long", "mid",
     "Write a Python function that merges two sorted lists, with "
     "comments explaining each step, then describe its complexity."),
    ("creative-short", "near-tie",
     "Invent a name for a small coastal town. Answer with the name alone."),
    ("creative-long", "near-tie",
     "Write the opening paragraph of a story about a lighthouse keeper "
     "who receives an unexpected letter."),
    ("openended", "near-tie",
     "Name three unrelated things that happen to be blue, and say why "
     "each came to mind."),
]

# **Depth is a session shape rather than a per-turn flag.** A session of
# two turns and a session of eight put the same prompt at different
# ordinals over different resident state, which is the comparison.
DEPTHS = [2, 8, 16, 32]

# Filler between the probes in a deep session. Short and confident, so
# the depth it builds costs little wall clock and adds little entropy of
# its own.
FILLER = "In one short sentence, name a colour and nothing else."


def entropies_of(turn):
    e = base.pointer(turn["payload"]["model.measurement"], "/entropies")
    if not isinstance(e, list) or not e:
        return None
    vals = [x for x in e if isinstance(x, (int, float))]
    if not vals:
        return None
    return {
        "count": len(vals),
        "mean": round(statistics.fmean(vals), 6),
        "max": round(max(vals), 6),
        "min": round(min(vals), 6),
    }


def run_session(cfg, probe, depth, iteration):
    """One matrix cell: serve, unload, reload, reissue, compare.

    The agent is left unloaded whichever path this takes, so a cell that
    fails does not hold the device against the next one.
    """
    key, character, text = probe
    rec = {"probe": key, "character": character, "depth": depth,
           "iteration": iteration, "verdict": None, "turns": []}

    # The probe sits last, so its ordinal is the depth and everything
    # before it is the state the depth exists to build.
    texts = [FILLER] * (depth - 1) + [text]

    try:
        base.admin(cfg, "unload")
        if base.admin(cfg, "load").get("kind") != "state":
            rec["verdict"] = "load refused"
            return rec
        if not base.wait_socket(cfg):
            rec["verdict"] = "gate socket never stood"
            return rec

        source_runs = set()
        for t in texts:
            close = base.gate_turn(cfg, t)
            if close.get("kind") != "answered":
                rec["verdict"] = f"source turn not answered: {close.get('kind')}"
                return rec
            source_runs.add(close.get("run"))

        # The closes name the run, so the wait is on that run rather than on
        # whichever is newest: a wait on the newest is satisfied by the
        # previous cell's run, which carries the same turn count at this
        # depth, and the reissue would compare against a stale record.
        if len(source_runs) != 1 or None in source_runs:
            rec["verdict"] = "the source turns did not share one run"
            return rec
        source_run = source_runs.pop()
        source_turns, _ = base.await_turns(cfg["trace"], depth, source_run)
        if len(source_turns) != depth:
            rec["verdict"] = f"expected {depth} source turns, found {len(source_turns)}"
            return rec

        base.admin(cfg, "unload")
        if base.admin(cfg, "load").get("kind") != "state":
            rec["verdict"] = "reload refused"
            return rec
        if not base.wait_socket(cfg):
            rec["verdict"] = "gate socket never stood after reload"
            return rec

        # Reissued from the record rather than from this script's
        # constants, because the record is the artifact under test.
        runs_seen = set()
        for st in source_turns:
            close = base.gate_turn(cfg, st["text"])
            if close.get("kind") != "answered":
                rec["verdict"] = f"reissue {st['turn']} closed {close.get('kind')}"
                return rec
            runs_seen.add(close.get("run"))
        if len(runs_seen) != 1 or None in runs_seen or source_run in runs_seen:
            rec["verdict"] = "reissues did not land in one fresh run"
            return rec
        replay_run = runs_seen.pop()

        replay_all, _ = base.await_turns(
            cfg["trace"], len(source_turns), replay_run)
        if not replay_all:
            rec["verdict"] = f"the closes named run {replay_run}, absent from the trace"
            return rec
        # **A short replay read is the sink, not the model.** It reaches its
        # own verdict rather than DIVERGED, which is the strongest negative
        # this harness emits and means the model did not reproduce. The two
        # must not share a word in an unattended run, and a sink one turn
        # behind is exactly the shape this harness was fixed for.
        if len(replay_all) < len(source_turns):
            rec["verdict"] = (
                f"replay read short: expected {len(source_turns)} turns,"
                f" found {len(replay_all)} - the record is incomplete"
            )
            return rec
        replay_by = {t["turn"]: t for t in replay_all}

        # A replay carrying surplus turns is interleaved traffic and is
        # never a match, however well the turns it shares agree.
        all_match = len(replay_all) == len(source_turns)
        for st in source_turns:
            rt = replay_by.get(st["turn"])
            if rt is None:
                all_match = False
                rec["turns"].append({"turn": st["turn"], "missing": True})
                continue
            checks = base.compare_turn(st, rt)
            matched = all(c["match"] for c in checks)
            all_match = all_match and matched
            rec["turns"].append({
                "turn": st["turn"],
                "is_probe": st["text"] == text,
                "matched": matched,
                "failed_checks": [c["check"] for c in checks if not c["match"]],
                "entropy": entropies_of(st),
                "source_ms": base.whole_ms(st),
                "replay_ms": base.whole_ms(rt),
            })
        rec["verdict"] = "REPRODUCED" if all_match else "DIVERGED"
        return rec
    except Exception as exc:  # an unattended run records rather than dies
        rec["verdict"] = f"error: {type(exc).__name__}: {exc}"
        return rec
    finally:
        base.admin(cfg, "unload")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--config", required=True)
    ap.add_argument("--outdir", required=True)
    ap.add_argument("--hours", type=float, default=7.0)
    ap.add_argument("--artifact", default=None,
                    help="override the declaration's artifact for every cell")
    args = ap.parse_args()

    with open(args.config) as f:
        cfg = json.load(f)
    os.makedirs(args.outdir, exist_ok=True)

    with open(cfg["declaration"]) as f:
        original = f.read()
    if args.artifact:
        import re
        swapped, n = re.subn(r"(artifact:\s*).*", r"\g<1>" + args.artifact,
                             original, count=1)
        if n != 1:
            print("no artifact line in the declaration", file=sys.stderr)
            sys.exit(2)
        with open(cfg["declaration"], "w") as fh:
            fh.write(swapped)

    deadline = time.time() + args.hours * 3600.0
    # Opened before the first load so the journal read at the summary
    # cannot reach back past this run.
    run_started = time.strftime(
        "%Y-%m-%d %H:%M:%S", time.localtime(time.time() - 1))
    results, iteration = [], 0
    # **Bound before the try, because the interrupt is caught rather than
    # fatal.** `except KeyboardInterrupt` below swallows the interrupt so a
    # run cut short still deposits its summary, which means the summary path
    # runs even when the library read never finished. Left unbound, a Ctrl-C
    # during the 142 MiB hash would reach the summary as a `NameError` and
    # lose every session the run had already recorded. The placeholder says
    # why it is empty rather than reading as "nothing to record", per the
    # same rule the reader itself follows.
    libraries = {"unreadable": "the run ended before the libraries were read"}
    binaries = {"unreadable": "the run ended before the binaries were read"}
    tools = {"unreadable": "the run ended before the toolchain was read"}
    # **Whether the at-start read happened at all**, which the placeholders
    # cannot say for themselves: a failed read and an interrupted one both
    # leave an `unreadable` dict, and comparing a placeholder against a good
    # closing read would report the build as having changed mid-run when the
    # at-start reading never happened.
    read_at_start = False
    logpath = os.path.join(args.outdir, "matrix.log")

    def log(msg):
        line = f"[{time.strftime('%H:%M:%S')}] {msg}"
        print(line, flush=True)
        with open(logpath, "a") as fh:
            fh.write(line + "\n")

    log(f"matrix start, deadline in {args.hours}h, "
        f"{len(PROMPTS)} prompts x {len(DEPTHS)} depths")
    try:
        # **Inside the cleanup scope**, because the declaration has already
        # been swapped by this point where `--artifact` was given: `ldd`
        # missing raises, hashing 142 MiB can be interrupted, and either
        # one outside the `try` would leave the operator's declaration
        # holding this run's artifact.
        libraries = base.engine_libraries(cfg)
        binaries = base.weaver_binaries(cfg)
        tools = base.toolchain(cfg)
        read_at_start = True
        log(f"engine libraries: {json.dumps(libraries)}")
        log(f"weaver binaries: {json.dumps(binaries)}")
        log(f"toolchain: {json.dumps(tools)}")

        # Sweeps rather than repeats: every combination is seen once
        # before any is seen twice, so a run cut short by the clock still
        # covers the matrix rather than the front of it.
        while time.time() < deadline:
            iteration += 1
            for depth in DEPTHS:
                for probe in PROMPTS:
                    if time.time() >= deadline:
                        break
                    started = time.time()
                    rec = run_session(cfg, probe, depth, iteration)
                    rec["seconds"] = round(time.time() - started, 1)
                    results.append(rec)
                    ent = ""
                    for t in rec["turns"]:
                        if t.get("is_probe") and t.get("entropy"):
                            ent = f" H_mean={t['entropy']['mean']}"
                    log(f"i{iteration} {probe[0]}/d{depth}: "
                        f"{rec['verdict']} ({rec['seconds']}s){ent}")
                    with open(os.path.join(args.outdir, "matrix.jsonl"), "a") as fh:
                        fh.write(json.dumps(rec) + "\n")
    except KeyboardInterrupt:
        log("interrupted")
    finally:
        # Restored only where this run swapped it: rewriting unconditionally
        # would turn an unrelated edit made during the run into a silent
        # revert of the operator's own declaration.
        if args.artifact:
            with open(cfg["declaration"], "w") as fh:
                fh.write(original)
        base.admin(cfg, "unload")

    total = len(results)
    good = sum(1 for r in results if r["verdict"] == "REPRODUCED")
    diverged = [r for r in results if r["verdict"] == "DIVERGED"]
    errors = [r for r in results if r["verdict"] not in ("REPRODUCED", "DIVERGED")]

    by_character = {}
    for r in results:
        b = by_character.setdefault(r["character"], {"n": 0, "ok": 0})
        b["n"] += 1
        b["ok"] += 1 if r["verdict"] == "REPRODUCED" else 0

    # **The box facts ride the summary rather than a sidecar**, per issue
    # #370's third ask. The olympus deposit of 2026-08-27 carried its
    # serving device and engine libraries in a hand-written `box-facts.txt`
    # beside this file, which works exactly once and only if whoever runs
    # the matrix next remembers. The readers are the confirm driver's, so
    # the two instruments answer this question the same way or not at all.
    #
    # **Every binding in the window is checked rather than the last one**,
    # per finding 6 of the olympus seat. An earlier draft recorded the last
    # load on the reasoning that every session binds the same device, which
    # is the assumption #370 falsified: `run_session` loads and unloads per
    # session, a seven-hour window holds hundreds of loads, and a mid-run
    # session binding differently would be recorded nowhere. The summary
    # says what was seen and says plainly when it was not one thing, which
    # is what a reader needs to know before trusting a rate over the run.
    # Guarded for the same reason: these facts exist to make the deposit
    # worth trusting, so they must never be the reason there is no deposit.
    # A box without `journalctl` on PATH raises here, and losing a
    # seven-hour matrix over a missing box fact would be the wrong trade.
    # **The binaries are read twice and compared**, per finding 7 of the
    # olympus seat. `--hours` lets a run reach seven and `run_session` loads
    # and unloads per session, so a build swapped mid-matrix would be
    # recorded nowhere and one hash asserted over the whole window - the
    # shape #370 falsified and the one this summary already rejects for
    # device bindings. One extra hash makes the claim checkable rather than
    # assumed. The libraries carry the same exposure and are read with them.
    # Guarded like the device read below, and for the same reason: a closing
    # read that raised would lose the run it was added to describe.
    # **`KeyboardInterrupt` is caught by name**, because it is not an
    # `Exception` and `except Exception` let it past. The main loop has
    # already absorbed one Ctrl-C by this point and these reads hash 142 MiB,
    # so a second one landed in the window would have killed `main` before
    # `summary.json` was written - losing every session, which is the loss
    # the placeholders above exist to prevent.
    #
    # **The two reads are wrapped apart.** Together, a library failure
    # discarded a good binary reading and was then recorded under the binary
    # field, so the summary said the binaries could not be re-read when they
    # could, and the library failure was recorded nowhere.
    caught = (Exception, KeyboardInterrupt)
    binaries_at_close = binaries
    try:
        closing = base.weaver_binaries(cfg)
        if not read_at_start:
            # Nothing to compare against, so the closing read is simply the
            # only reading there is.
            binaries_at_close = closing
        elif closing != binaries:
            binaries_at_close = {"varied": {"at_start": binaries,
                                            "at_close": closing}}
    except caught as e:  # noqa: BLE001 - any failure degrades to a note
        binaries_at_close = {"at_start": binaries,
                             "at_close_unreadable": f"weaver_binaries: {e}"}
    try:
        libraries_at_close = base.engine_libraries(cfg)
        if not read_at_start:
            libraries = libraries_at_close
        elif libraries_at_close != libraries:
            libraries = {"varied": {"at_start": libraries,
                                    "at_close": libraries_at_close}}
    except caught as e:  # noqa: BLE001 - any failure degrades to a note
        libraries = {"at_start": libraries,
                     "at_close_unreadable": f"engine_libraries: {e}"}

    try:
        bindings = base.device_bindings(cfg, run_started)
    except caught as e:  # noqa: BLE001 - any failure degrades to a note
        bindings = [{"unreadable": f"the device read failed: {e}"}]
    summary = {
        "serving_device": (bindings[0] if len(bindings) == 1
                           else {"varied": bindings}),
        "engine_libraries": libraries,
        "weaver_binaries": binaries_at_close,
        "toolchain": tools,
        "sessions": total,
        "reproduced": good,
        "diverged": len(diverged),
        "errors": len(errors),
        "by_character": by_character,
        "diverged_detail": diverged[:20],
        "error_detail": [{"probe": r["probe"], "depth": r["depth"],
                          "verdict": r["verdict"]} for r in errors[:20]],
    }
    with open(os.path.join(args.outdir, "summary.json"), "w") as fh:
        json.dump(summary, fh, indent=1)

    log(f"done: {good}/{total} reproduced, {len(diverged)} diverged, "
        f"{len(errors)} errors")
    for ch, b in sorted(by_character.items()):
        log(f"  {ch}: {b['ok']}/{b['n']}")
    sys.exit(0 if total and good == total else 1)


if __name__ == "__main__":
    main()
