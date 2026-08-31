#!/usr/bin/env python3
"""The 8B trace-generation night, per the sketch of 2026-08-31.

The traces are the product. Three runs, each under the shared
serve-unload-reload-reissue protocol so each session produces its own
fidelity check as a byproduct, and no measurement beyond that is taken:

  Run A  the length-instruction pair - one story prompt asked long, then
         asked short, one session each. The finding is the position of
         first divergence between the two emissions.
  Run B  long single-turn determinism - the long ask issued twice with a
         full unload between. Byte-equal emissions extend the
         within-machine claim into depth. A demonstration, not new
         evidence for the claim the minimal-session runs already made.
  Run C  the escalating multi-turn session - eight turns, requested
         length stepping up, the first time an agent stays loaded across
         many turns. The assertion is byte equality across the replay.

Sweep before repeat: the whole set runs once, then again, so a clock-cut
night covers the set rather than its front.

Usage:  generate_traces.py --config <box>.json --outdir DIR

The config is the box's confirm-cells config plus four keys:

  "artifact"            the bf16 rung, staged and hash-verified
  "schedule"            "full-32k" (olympus) or "fit-16k" (thinkpad)
  "context_capacity"    32768 or as found on the box (cell 7.3)
  "sweeps"              3 unless the pricing says otherwise (cell 7.2)

Vocabulary, per the sketch's section 2 ruling: a session is what happens
between a load and an unload, and turns live inside it. Every figure this
driver deposits counts sessions under that ruling.
"""

import argparse
import hashlib
import json
import os
import re
import sys
import time

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                "..", "cross-precision-repro"))
import confirm_cells as base  # noqa: E402

# ------------------------------------------------------------------ prompts
# Pinned across every box and every sweep. Do not edit per box. The story
# subject is deliberately one with no famous single continuation, so the
# model generates rather than recites. Runs A and B share one base sentence
# and differ in exactly one instruction, which is what makes a divergence
# between them attributable to the length ask and nothing else.

STORY = (
    "Tell the story of a lighthouse keeper on a remote northern coast who "
    "has spent thirty years cataloguing every sound the sea makes, and of "
    "the winter she hears one she cannot place. The setting, the "
    "catalogue, and what the sound turns out to be are yours to invent."
)
LONG_ASK = STORY + " Tell it in about 2,000 words."
SHORT_ASK = STORY + " Tell it in about 500 words."

# Run C: confident, low-entropy, factual subject matter - one coherent
# thread through physical geography, so the trace family carries the
# confident line beside the story runs' diffuse one. The {} slot takes the
# schedule's word figure, so the questions are fixed and only the requested
# length moves with the schedule.
RUN_C_QUESTIONS = [
    "In about {} words, explain how rivers form, from first rainfall to a "
    "channel reaching the sea.",
    "In about {} words, explain how glaciers move and how they carve the "
    "valleys they leave behind.",
    "In about {} words, explain how a river delta forms and why deltas "
    "take the shapes they do.",
    "In about {} words, explain what an aquifer is and how groundwater "
    "moves through rock.",
    "In about {} words, explain how mountain ranges rise, and what plate "
    "tectonics contributes to their shape.",
    "In about {} words, explain how volcanoes form and why they cluster "
    "where they do.",
    "In about {} words, explain what drives the ocean's large-scale "
    "currents and how they shape climate.",
    "In about {} words, explain the water cycle, drawing together rivers, "
    "glaciers, groundwater and the ocean from your earlier answers.",
]

# ---------------------------------------------------------------- schedules
# Cell 7.1, settled by window arithmetic. Token targets per turn, with the
# word figures the prompts carry derived at ~1.35 tokens per English word
# for this tokenizer family.
#
# full-32k: 500+750+1000+1500+2000+2500+3000+3750 = 15,000 output tokens.
# Inputs are ~370 more (the identity prefix, eight questions, template
# overhead), so the session ends near 15.4k resident in a 32,768 window -
# elision room to spare, per the sketch. The safety property that sized it:
# a model overshooting every request by half still ends near 23k and fits.
#
# fit-16k: the same shape halved, ending near 7.9k resident in 16,384,
# with the same overshoot-by-half property (~11.8k). The thinkpad's window
# is its own finding (cell 7.3) and its schedule is not olympus's claim.
SCHEDULES = {
    "full-32k": {"tokens": [500, 750, 1000, 1500, 2000, 2500, 3000, 3750],
                 "words": [370, 550, 740, 1100, 1480, 1850, 2200, 2800]},
    "fit-16k": {"tokens": [250, 375, 500, 750, 1000, 1250, 1500, 1875],
                "words": [185, 280, 370, 550, 740, 925, 1100, 1390]},
}

# High enough that every scheduled turn finishes naturally - 6144 is 1.6x
# the largest full-schedule request. A turn that hits it anyway fails the
# session (require_completed below), because a capped turn is a defective
# specimen for source material. Uniform across boxes so the declarations
# differ in capacity alone.
MAX_TOKENS_PER_TURN = 6144

# A 3,750-token turn at the A6000's measured 40.6 tok/s is ~92 seconds;
# this bounds a slower box and a runaway toward the cap, not the plan.
TURN_TIMEOUT = 900


def cccl_provenance():
    """The cccl version read from the header the build includes,
    and the kernel guards evaluated against it.

    Named `_derived` because the guards are computed from the version, not
    read out of a shipped object - the honest form until the loop-identity
    act gives the record a measured field. The version itself is a reading.
    """
    header = "/usr/include/cccl/cuda/std/__cccl/version.h"
    try:
        with open(header) as f:
            m = re.search(r"#define CCCL_VERSION (\d+)", f.read())
        if not m:
            return {"unreadable": f"no CCCL_VERSION in {header}"}
        v = int(m.group(1))
        minor = (v // 1000) % 1000
        return {
            "cccl_version": v,
            "header": header,
            "kernel_selection_derived": {
                "argsort": "CUB" if minor >= 1 else "fallback",
                "top_k": "CUB" if minor >= 2 else "fallback",
            },
        }
    except OSError as e:
        return {"unreadable": base._why(e)}


def emissions_of(deposit_path):
    """Turn -> emission text, read from a deposited source run."""
    out = {}
    try:
        with open(deposit_path) as f:
            for line in f:
                try:
                    e = json.loads(line)
                except json.JSONDecodeError:
                    continue
                if e.get("kind") != "model.output":
                    continue
                em = base.pointer(e.get("payload"), "/emission")
                if isinstance(em, str):
                    out[e.get("turn")] = em
    except OSError:
        pass
    return out


def first_divergence(a, b):
    """Byte position where two emissions part, with a window on each side.

    None means byte-identical. The position is the finding for Run A
    (where does the length instruction start to matter) and the assertion
    for Run B (there must not be one).

    **Measured on the UTF-8 bytes, not on code points.** `len()` over `str`
    counts characters, so a multi-byte character ahead of the divergence
    would shift a code-point index away from the byte the record carries.
    The contexts are decoded with `replace` because the window may open
    mid-character.
    """
    ab, bb = a.encode("utf-8"), b.encode("utf-8")
    if ab == bb:
        return None
    n = min(len(ab), len(bb))
    at = next((i for i in range(n) if ab[i] != bb[i]), n)
    window = lambda raw: raw[max(0, at - 40):at + 40].decode(
        "utf-8", errors="replace")
    return {
        "byte": at,
        "a_context": window(ab),
        "b_context": window(bb),
        "a_len": len(ab),
        "b_len": len(bb),
    }


def read_declaration(cfg):
    """The original and its backup, refusing a backup that already exists.

    **An existing backup means a previous run died patched**, and writing
    over it would replace the operator's real declaration with the patched
    one - every later restore then restores the patch and the true values
    are gone. The refusal names the file and leaves the recovery to the
    operator, who alone knows which of the two is the original.
    """
    backup = cfg["declaration"] + ".pre-tracegen"
    if os.path.exists(backup):
        raise SystemExit(
            f"{backup} already exists - a previous run left the declaration "
            "patched. Restore or remove the backup by hand before running, "
            "because overwriting it would destroy the only true original.")
    with open(cfg["declaration"]) as f:
        original = f.read()
    with open(backup, "w") as f:
        f.write(original)
    return original, backup


def patched_declaration(original, cfg):
    """The patched text, count-asserted so a declaration a regex does not
    match refuses before any load rather than running the night on half a
    patch. The caller writes it inside its restoration scope."""
    patched = original
    for pattern, value in (
        (r"(artifact:\s*).*", cfg["artifact"]),
        (r"(context-capacity:\s*).*", str(cfg["context_capacity"])),
        (r"(max-tokens-per-turn:\s*).*", str(MAX_TOKENS_PER_TURN)),
    ):
        patched, n = re.subn(pattern, r"\g<1>" + value, patched, count=1)
        if n != 1:
            raise SystemExit(f"declaration line not found: {pattern}")
    return patched


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--config", required=True)
    ap.add_argument("--outdir", required=True)
    ap.add_argument("--sessions", default=None,
                    help="comma list to run a subset, e.g. A-short - for "
                         "setup smoke only, never for the night, and the "
                         "deposit says which it was")
    args = ap.parse_args()
    with open(args.config) as f:
        cfg = json.load(f)
    os.makedirs(args.outdir, exist_ok=True)

    schedule = SCHEDULES[cfg["schedule"]]
    sweeps = int(cfg.get("sweeps", 3))
    run_c_texts = [q.format(f"{w:,}")
                   for q, w in zip(RUN_C_QUESTIONS, schedule["words"])]

    sessions = [
        ("A-long", [LONG_ASK]),
        ("A-short", [SHORT_ASK]),
        ("B-a", [LONG_ASK]),
        ("B-b", [LONG_ASK]),
        ("C", run_c_texts),
    ]
    # **Validated before anything on the box is touched.** A typo in
    # --sessions raising after the declaration was patched left the 8B
    # artifact and the raised cap live in karl.yaml with nothing restoring
    # them - the review's finding 2, answered by ordering.
    session_subset = None
    if args.sessions:
        wanted = set(args.sessions.split(","))
        unknown = wanted - {sid for sid, _ in sessions}
        if unknown:
            raise SystemExit(f"unknown sessions: {sorted(unknown)}")
        sessions = [(sid, t) for sid, t in sessions if sid in wanted]
        # A subset is a smoke and its deposit must not read as the night.
        sweeps = 1
        session_subset = sorted(wanted)

    # **The SPU is resolved once and both collectors share it**, per the
    # 2026-08-28 act the shared driver documents: two independent
    # resolutions blipping apart put two builds under one report with
    # nothing marking the disagreement.
    spu = base._resolve_spu(cfg)
    libraries = base.engine_libraries(cfg, spu)
    binaries = base.weaver_binaries(cfg, spu)
    tools = base.toolchain(cfg)
    print(f"engine libraries: {json.dumps(libraries)}", flush=True)
    print(f"weaver binaries: {json.dumps(binaries)}", flush=True)
    artifact_sha = base._sha256(cfg["artifact"])
    print(f"artifact sha256: {artifact_sha}", flush=True)

    original, backup = read_declaration(cfg)
    provenance = {
        "sketch": "8B trace generation, v0.1 2026-08-31",
        "schedule": cfg["schedule"],
        "schedule_tokens": schedule["tokens"],
        "context_capacity": cfg["context_capacity"],
        "max_tokens_per_turn": MAX_TOKENS_PER_TURN,
        "artifact": cfg["artifact"],
        "artifact_sha256": artifact_sha,
        "declaration_sha256_original": hashlib.sha256(
            original.encode()).hexdigest(),
        "cccl": cccl_provenance(),
    }
    if session_subset:
        provenance["session_subset"] = session_subset

    report = {"box": cfg["box"], "provenance": provenance, "sessions": [],
              "analysis": {}}
    report_path = os.path.join(args.outdir, f"report-{cfg['box']}-tracegen.json")

    def deposit_report():
        # Written after every session, so a raise mid-night costs the tail
        # and never the record - defect 2 of #379, answered structurally.
        with open(report_path, "w") as f:
            json.dump(report, f, indent=1)

    cell = {"name": "tracegen", "artifact": cfg["artifact"],
            "precision": cfg.get("precision", "bf16")}
    all_reproduced = True
    try:
        # The live declaration is written only inside the restoration scope,
        # so every exit path from here runs the restore below.
        patched = patched_declaration(original, cfg)
        with open(cfg["declaration"], "w") as f:
            f.write(patched)
        provenance["declaration_sha256_as_run"] = hashlib.sha256(
            patched.encode()).hexdigest()

        for sweep in range(1, sweeps + 1):
            for sid, texts in sessions:
                name = f"{sid}-s{sweep}"
                started = time.time()
                r = base.run_cell(
                    cfg, dict(cell, name=name), args.outdir,
                    libraries, binaries, tools,
                    texts=texts, require_completed=True,
                    turn_timeout=TURN_TIMEOUT)
                r["session"] = name
                r["sweep"] = sweep
                r["seconds"] = round(time.time() - started, 1)
                report["sessions"].append(r)
                all_reproduced &= r.get("verdict") == "REPRODUCED"
                deposit_report()
                print(f"[{name}] {r.get('verdict')} in {r['seconds']}s",
                      flush=True)

        # **Provenance is read again at close.** One opening block stamped
        # into a ninety-minute thirty-load night attributes every trace
        # after a mid-night install to the pre-install hashes. Guarded per
        # the matrix's close rather than the raw comparison #379 documents:
        # a failed closing read deposits as unreadable, never as movement.
        closing_spu = base._resolve_spu(cfg)
        at_close = {
            "engine_libraries": base.engine_libraries(cfg, closing_spu),
            "weaver_binaries": base.weaver_binaries(cfg, closing_spu),
            "toolchain": base.toolchain(cfg),
        }
        opening = {"engine_libraries": libraries,
                   "weaver_binaries": binaries, "toolchain": tools}
        moved = {}
        for field, closing in at_close.items():
            if not base.is_reading(closing):
                moved[field] = {"at_close_unreadable": closing}
            elif not base.is_reading(opening[field]):
                moved[field] = {"at_open_unreadable": opening[field]}
            elif closing != opening[field]:
                moved[field] = {"at_open": opening[field],
                                "at_close": closing}
        provenance["provenance_at_close"] = moved or "unchanged"
        if moved:
            print(f"PROVENANCE MOVED OR UNREADABLE: {sorted(moved)}",
                  flush=True)

        # The cross-session findings, from the deposits rather than from
        # memory of the loop above.
        def em(name, turn="t-1"):
            return emissions_of(os.path.join(
                args.outdir, f"cell-{name}-source.ndjson")).get(turn, "")

        # A pair analysis only exists where both emissions do - a smoke
        # subset must not deposit first_divergence("", "") reading as
        # IDENTICAL for sessions that never ran.
        for sweep in range(1, sweeps + 1):
            a_long, a_short = em(f"A-long-s{sweep}"), em(f"A-short-s{sweep}")
            if a_long and a_short:
                report["analysis"][f"A-divergence-s{sweep}"] = \
                    first_divergence(a_long, a_short)
            b_a, b_b = em(f"B-a-s{sweep}"), em(f"B-b-s{sweep}")
            if b_a and b_b:
                report["analysis"][f"B-identity-s{sweep}"] = \
                    first_divergence(b_a, b_b)
        deposit_report()

        verdicts = [x.get("verdict") for x in report["sessions"]]
        good = sum(v == "REPRODUCED" for v in verdicts)
        print(f"\nsessions: {good}/{len(verdicts)} REPRODUCED", flush=True)
        for sweep in range(1, sweeps + 1):
            notes = []
            if f"A-divergence-s{sweep}" in report["analysis"]:
                a = report["analysis"][f"A-divergence-s{sweep}"]
                notes.append("A first divergence " + (
                    f"byte {a['byte']}" if a
                    else "NONE - identical despite the ask"))
            if f"B-identity-s{sweep}" in report["analysis"]:
                b = report["analysis"][f"B-identity-s{sweep}"]
                notes.append("B " + ("IDENTICAL" if b is None
                                     else f"DIVERGED at byte {b['byte']}"))
            if notes:
                print(f"sweep {sweep}: " + ", ".join(notes), flush=True)
    finally:
        # **Restored from the string in hand, never from the disk.** A
        # re-read of the backup inside a finally can raise, masking the real
        # exception and skipping the unload with the agent still holding the
        # device. The backup is removed only after the original is back, and
        # the unload runs whatever the restore did.
        try:
            with open(cfg["declaration"], "w") as f:
                f.write(original)
            try:
                os.unlink(backup)
            except OSError:
                pass
            print("declaration restored", flush=True)
        finally:
            base.admin(cfg, "unload")

    # **A night that did not reproduce says so in its exit code**, the same
    # gate the confirm driver holds: a wrapper of the shape `... && publish`
    # must not ship defective specimens because the failure was only prose.
    if not report["sessions"] or not all_reproduced:
        sys.exit(1)


if __name__ == "__main__":
    main()
