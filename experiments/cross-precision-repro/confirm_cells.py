#!/usr/bin/env python3
"""Cross-precision / cross-architecture reproducibility cells.

For each cell (one artifact at one precision): load the agent on that
artifact, serve one short and one longer turn at the gate socket,
unload fully, reload, read the request texts back from the record's
own message.user events, reissue them byte-exact in order, and compare
the two runs field by field. Stdlib only, so any box with the runtime
can run it unchanged.

The pinning discipline is the point: same commit, same declaration
apart from the artifact path, same declared seed, same turn texts. A
box needing its own build records that build as an arm, not a
nuisance.

Usage:  confirm_cells.py --config <box>.json [--outdir DIR]

Config (JSON):
{
  "box":          "thinkpad",
  "agent":        "karl",
  "declaration":  "/home/todd/.weaveragents/karl.yaml",
  "gate_socket":  "/run/weaver-karl/gate.sock",
  "trace":        "/home/todd/.weaveragents/karl/trace.ndjson",
  "admin_bin":    "/opt/weaver/bin/weaver-admin",
  "admin_config": "/etc/weaver/admin",
  "repo":         "/home/todd/Projects/WeaverTools_Project/WeaverTools",
  "build_flags":  "cargo build --release --workspace --features weaver-spu/cuda",
  "cells": [
    {"name": "q8",   "precision": "q8_0", "artifact": "/opt/weaver/models/qwen2.5-0.5b-instruct-q8_0.gguf"},
    {"name": "bf16", "precision": "bf16", "artifact": "/opt/weaver/models/qwen2.5-0.5b-instruct-bf16.gguf"}
  ]
}
"""
import argparse
import hashlib
import json
import os
import re
import shutil
import socket
import subprocess
import sys
import time

# Pinned across every box and every cell. Do not edit per box.
SHORT_TEXT = "Introduce yourself in exactly one short sentence."
LONG_TEXT = (
    "Write a detailed step-by-step explanation of how a binary search "
    "works, then implement it in Python with comments, then walk "
    "through an example run on a list of twenty numbers."
)

CHECKS = [
    ("rendered prompt", "model.request", "/rendered"),
    ("derived generation seed", "model.request", "/sampling/generation_seed"),
    ("effective sampling knobs", "model.request", "/sampling"),
    ("emission bytes", "model.output", "/emission"),
    ("finish kind", "model.output", "/finish"),
    ("resident count", "model.output", "/resident"),
    ("input token ids", "model.measurement", "/input_tokens"),
    ("per-token entropies", "model.measurement", "/entropies"),
]


def sh(args, **kw):
    return subprocess.run(args, capture_output=True, text=True, **kw)


def admin(cfg, verb):
    r = sh(["sudo", "-n", f"WEAVER_ADMIN_CONFIG={cfg['admin_config']}",
            cfg["admin_bin"], verb, cfg["agent"]])
    line = (r.stdout.strip().splitlines() or [""])[-1]
    try:
        return json.loads(line)
    except json.JSONDecodeError:
        return {"kind": "unparsed", "stdout": r.stdout, "stderr": r.stderr,
                "exit": r.returncode}


def wait_socket(cfg, timeout=120):
    end = time.time() + timeout
    while time.time() < end:
        if os.path.exists(cfg["gate_socket"]):
            try:
                s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
                s.connect(cfg["gate_socket"])
                s.close()
                return True
            except OSError:
                pass
        time.sleep(0.5)
    return False


def gate_turn(cfg, text, timeout=600):
    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    s.settimeout(timeout)
    s.connect(cfg["gate_socket"])
    s.sendall((json.dumps({"text": text}) + "\n").encode())
    line = s.makefile().readline()
    s.close()
    return json.loads(line)


def read_runs(trace_path, keep=None):
    """Run -> its events, in first-appearance order.

    **`keep` reads the tail rather than the file**, and a caller wanting the
    most recent runs should pass it. The trace is append-only and grows
    without bound, so a full parse costs the whole history to answer a
    question about its end: measured 2026-08-27 against a 251 MiB trace, the
    full scan took 4.0 s and ran twice per cell, which is the harness
    charging a run for every run before it. With `keep` the cost follows the
    tail instead.

    The scan walks backward in chunks and stops one run past the `keep`th,
    so the runs it returns are whole rather than cut at a chunk edge. A
    partial line at a chunk boundary is held for the next block for the same
    reason. `keep=None` reads everything, which is the original behaviour.
    """
    lines = open(trace_path) if keep is None else _tail_lines(trace_path, keep)
    runs = {}
    order = []
    for line in lines:
        try:
            e = json.loads(line)
        except json.JSONDecodeError:
            continue
        r = e.get("run")
        if r is None:
            continue
        if r not in runs:
            runs[r] = []
            order.append(r)
        runs[r].append(e)
    return order, runs


def _tail_lines(trace_path, keep, chunk=1 << 20):
    """The trailing lines covering the last `keep` runs, in file order."""
    with open(trace_path, "rb") as f:
        f.seek(0, os.SEEK_END)
        pos = f.tell()
        held = b""
        out = []
        seen = []
        done = False
        while pos > 0 and not done:
            step = min(chunk, pos)
            pos -= step
            f.seek(pos)
            block = f.read(step) + held
            parts = block.split(b"\n")
            # The first element may be the tail of a line beginning further
            # back, so it is held for the next block rather than parsed here.
            held = parts[0]
            for raw in reversed(parts[1:]):
                if not raw.strip():
                    continue
                out.append(raw)
                try:
                    r = json.loads(raw).get("run")
                except json.JSONDecodeError:
                    continue
                if r is not None and r not in seen:
                    seen.append(r)
                    # One past `keep`: the `keep`th run is whole only once a
                    # newer boundary has been crossed.
                    if len(seen) > keep:
                        done = True
                        break
        if not done and held.strip():
            out.append(held)
    return [raw.decode("utf-8", "replace") for raw in reversed(out)]


def await_turns(trace_path, want, keep=4, timeout=30.0):
    """The newest run once it carries `want` turns, or once time runs out.

    **The sink writes after the close answers**, so a read taken the instant
    a turn closes can miss the events that turn produced. This was invisible
    while `read_runs` scanned the whole trace, because the scan itself took
    seconds and the sink caught up inside it; the tail read of 2026-08-27
    removed that accidental delay and the race surfaced as a run one turn
    short, every time, the missing turn always the last. Waiting for the
    record is what the harness meant to do, and an accidental sleep is not a
    way to do it.

    Answers `(run, turns)` with whatever stands when the count is reached or
    the timeout expires, and the caller reports the shortfall - a wait that
    raised would turn a slow sink into a failed cell.
    """
    end = time.time() + timeout
    run, turns = None, []
    while True:
        order, runs = read_runs(trace_path, keep=keep)
        if order:
            run = order[-1]
            turns = cut_turns(runs[run])
            if len(turns) >= want:
                return run, turns
        if time.time() >= end:
            return run, turns
        time.sleep(0.05)


def cut_turns(events):
    """Turn -> its events, in first-appearance order. The request text
    is the turn's last message.user event, identity messages preceding
    the request in render order."""
    order, by = [], {}
    for e in events:
        t = e.get("turn")
        if not t:
            continue
        if t not in by:
            by[t] = {}
            order.append(t)
        by[t].setdefault(e["kind"], []).append(e)
    turns = []
    for t in order:
        k = by[t]
        users = k.get("message.user", [])
        text = None
        if users:
            c = users[-1].get("payload", {}).get("content", [])
            if c and c[0].get("type") == "text":
                text = c[0]["text"]
        payload = {kind: (k[kind][0].get("payload") if kind in k else None)
                   for kind in ("model.request", "model.output",
                                "model.measurement")}
        wall = {kind: (k[kind][0].get("wall_ms") if kind in k else None)
                for kind in ("turn.started", "turn.closed")}
        turns.append({"turn": t, "text": text, "payload": payload,
                      "wall": wall})
    return [t for t in turns
            if t["text"] is not None
            and all(t["payload"][x] is not None for x in t["payload"])]


def pointer(value, ptr):
    cur = value
    for part in ptr.strip("/").split("/"):
        if not isinstance(cur, dict) or part not in cur:
            return None
        cur = cur[part]
    return cur


def compare_turn(src, rep):
    out = []
    for name, kind, ptr in CHECKS:
        a = pointer(src["payload"][kind], ptr)
        b = pointer(rep["payload"][kind], ptr)
        out.append({"check": name, "match": a == b})
    return out


def whole_ms(t):
    a, b = t["wall"]["turn.started"], t["wall"]["turn.closed"]
    return (b - a) if a is not None and b is not None else None


def cell_metadata(cfg, cell):
    h = hashlib.sha256()
    with open(cell["artifact"], "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    gpu = sh(["nvidia-smi", "--query-gpu=name,driver_version",
              "--format=csv,noheader"]).stdout.strip()
    commit = sh(["git", "-C", cfg["repo"], "rev-parse", "HEAD"]).stdout.strip()
    rustc = sh(["rustc", "--version"]).stdout.strip()
    return {
        "box": cfg["box"],
        "precision": cell["precision"],
        "artifact": cell["artifact"],
        "artifact_sha256": h.hexdigest(),
        "device": gpu,
        "build_flags": cfg["build_flags"],
        "rustc": rustc,
        "commit": commit,
        "short_text": SHORT_TEXT,
        "long_text": LONG_TEXT,
    }


def run_cell(cfg, cell, outdir):
    name = cell["name"]
    log = lambda m: print(f"[{name}] {m}", flush=True)
    report = {"cell": name, "metadata": cell_metadata(cfg, cell),
              "steps": [], "turns": [], "verdict": None}

    # The declaration with this cell's artifact, everything else as
    # the operator wrote it.
    with open(cfg["declaration"]) as f:
        decl = f.read()
    swapped, n = re.subn(r"(artifact:\s*).*", r"\g<1>" + cell["artifact"],
                         decl, count=1)
    if n != 1:
        report["verdict"] = "no artifact line in the declaration"
        return report
    with open(cfg["declaration"], "w") as f:
        f.write(swapped)

    def step(verb):
        a = admin(cfg, verb)
        report["steps"].append({verb: a})
        log(f"{verb}: {json.dumps(a)}")
        return a

    # The finally below guarantees the cell never leaves an agent
    # holding the device, whichever return path it takes.
    try:
        step("unload")  # whatever held the device before this cell
        if step("load").get("kind") != "state":
            report["verdict"] = "load refused"
            return report
        if not wait_socket(cfg):
            report["verdict"] = "gate socket never stood"
            return report

        log("serving the source turns")
        for text in (SHORT_TEXT, LONG_TEXT):
            close = gate_turn(cfg, text)
            log(f"close {close.get('kind')} turn {close.get('turn')}")
            if close.get("kind") != "answered":
                report["verdict"] = f"source turn not answered: {close}"
                return report

        source_run, source_turns = await_turns(cfg["trace"], 2)
        if source_run is None:
            report["verdict"] = "the trace holds no runs - wrong path or silent sink"
            return report
        if len(source_turns) != 2:
            report["verdict"] = f"expected 2 source turns, found {len(source_turns)}"
            return report

        step("unload")
        if step("load").get("kind") != "state":
            report["verdict"] = "reload refused"
            return report
        if not wait_socket(cfg):
            report["verdict"] = "gate socket never stood after reload"
            return report

        # Reissue byte-exact, from the record rather than from this
        # script's constants: the record is the artifact under test.
        # Every close must be answered and every close must name one
        # fresh run - a refusal or a split across runs ends the cell
        # rather than comparing against the wrong record.
        log("reissuing from the record")
        runs_seen = set()
        for st in source_turns:
            close = gate_turn(cfg, st["text"])
            log(f"reissue {st['turn']} -> {close.get('kind')}")
            if close.get("kind") != "answered":
                report["verdict"] = f"reissue {st['turn']} closed {close.get('kind')}"
                return report
            runs_seen.add(close.get("run"))
        if len(runs_seen) != 1 or None in runs_seen or source_run in runs_seen:
            report["verdict"] = f"reissues did not land in one fresh run: {sorted(map(str, runs_seen))}"
            return report
        replay_run = runs_seen.pop()

        _, replay_probe = await_turns(cfg["trace"], len(source_turns))
        order, runs = read_runs(cfg["trace"], keep=4)
        if replay_run not in runs:
            report["verdict"] = (
                f"the closes named run {replay_run} but the trace does not carry"
                " it - sink lag or a different sink"
            )
            return report
        replay_all = cut_turns(runs[replay_run])
        replay_turns = {t["turn"]: t for t in replay_all}
        if len(replay_all) != len(source_turns):
            log(f"turn count differs: source {len(source_turns)} replay {len(replay_all)}")

        # A replay with surplus turns is interleaved traffic and is
        # never a match, even when every source turn agrees.
        all_match = len(replay_all) == len(source_turns)
        for st in source_turns:
            rt = replay_turns.get(st["turn"])
            checks = (compare_turn(st, rt) if rt else
                      [{"check": c[0], "match": False} for c in CHECKS])
            ok = all(c["match"] for c in checks)
            all_match &= ok
            m = st["payload"]["model.measurement"]
            report["turns"].append({
                "turn": st["turn"],
                "reproduced": ok,
                "checks": checks,
                "tokens_in": len(m.get("input_tokens", [])),
                "tokens_out": len(m.get("entropies", [])),
                "source_ms": whole_ms(st),
                "replay_ms": whole_ms(rt) if rt else None,
            })
            log(f"{st['turn']}: {'MATCH' if ok else 'DIVERGED: ' + ', '.join(c['check'] for c in checks if not c['match'])}")

        report["source_run"] = source_run
        report["replay_run"] = replay_run
        report["verdict"] = "REPRODUCED" if all_match else "NOT REPRODUCED"

        # Deposit the two runs' events beside the report.
        for label, run in (("source", source_run), ("replay", replay_run)):
            with open(os.path.join(outdir, f"cell-{name}-{label}.ndjson"), "w") as f:
                for e in runs[run]:
                    f.write(json.dumps(e) + "\n")
        return report
    finally:
        # Whichever path returned, the cell never leaves its artifact
        # holding the device. After an ordinary finish this answers
        # no_residency, which is harmless and recorded.
        step("unload")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--config", required=True)
    ap.add_argument("--outdir", default=".")
    args = ap.parse_args()
    with open(args.config) as f:
        cfg = json.load(f)
    os.makedirs(args.outdir, exist_ok=True)

    backup = cfg["declaration"] + ".pre-cells"
    shutil.copy2(cfg["declaration"], backup)
    reports = []
    try:
        for cell in cfg["cells"]:
            reports.append(run_cell(cfg, cell, args.outdir))
    finally:
        shutil.copy2(backup, cfg["declaration"])
        os.unlink(backup)
        print("declaration restored", flush=True)

    out = os.path.join(args.outdir, f"report-{cfg['box']}.json")
    with open(out, "w") as f:
        json.dump(reports, f, indent=1)
    print(f"\nreport: {out}")
    for r in reports:
        print(f"  cell {r['cell']}: {r['verdict']}")
    sys.exit(0 if all(r["verdict"] == "REPRODUCED" for r in reports) else 1)


if __name__ == "__main__":
    main()
