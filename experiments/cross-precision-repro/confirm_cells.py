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
  "spu_bin":      "optional; overrides the spu-binary named in admin_config",
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


DEVICE_LINE = re.compile(
    r"using device CUDA(\d+) \(([^)]*)\) \(([0-9a-fA-F:.]+)\)")
# One per load, printed ahead of the device block, so it marks the boundary
# that grepping the journal would otherwise destroy.
LOAD_BOUNDARY = re.compile(r"ggml_cuda_init: found \d+ CUDA device")


def serving_device(cfg, since):
    """The devices that actually answered, read from the worker's own load.

    `nvidia-smi` reports the machine, not the run. On a box holding more
    than one card its output names every device and the one that served
    appears nowhere, which is how the first olympus Ada arm ran to
    completion on an A6000 and reported REPRODUCED - the error was caught
    by reading the journal and by nothing in the record.

    **The engine logs one line per device it bound, not one per load.** At
    the pinned rev `ecce255`, `llama.cpp:1081` is
    `for (const auto & dev : model->devices)` around the `using device`
    line, and `ResidentModel::load` sets `LlamaSplitMode::Layer` with
    `with_devices` whenever admission binds more than one GPU. So a paired
    binding emits two lines in one load and a single scalar answer would
    name a device that never served alone. Every line of the most recent
    load is kept, and the most recent load is the last contiguous run of
    them: the engine emits the block from one loop with nothing
    interleaved.

    **An unreadable journal is not an absent device.** `journalctl` exits 0
    with empty output when the invoking user is in neither `systemd-journal`
    nor `adm`, and this script runs it unprivileged while running admin
    under `sudo -n`. Reporting that as "no CUDA device" would hide the
    wrong-device defect this reader exists to catch, on exactly the boxes
    whose provisioning is least careful. The unit logged copiously during a
    load that succeeded, so zero lines of any kind means the read failed,
    and that is recorded as its own answer.
    """
    groups = _device_groups(cfg, since)
    if "unreadable" in groups:
        return groups
    found = groups["groups"]
    if found:
        return {"devices": found[-1]}
    return {"devices": [], "note": "the load named no CUDA device"}


def device_bindings(cfg, since):
    """Every distinct binding seen in the window, in first-seen order.

    `serving_device` answers for one load. A run that loads repeatedly needs
    to know whether the answer held, which is the assumption issue #370
    falsified, so this reports the set rather than a representative.
    """
    groups = _device_groups(cfg, since)
    if "unreadable" in groups:
        return [groups]
    seen = []
    for g in groups["groups"]:
        if g not in seen:
            seen.append(g)
    return seen


def _device_groups(cfg, since):
    """The window's `using device` blocks, one list of devices per load.

    **The load boundary is read explicitly and not inferred from adjacency.**
    An earlier draft grouped on contiguity, which is correct against the raw
    journal and wrong the moment the read is grepped: `-g` drops every
    non-matching line, so four single-device loads arrive as four adjacent
    lines and read as one four-device binding. The engine prints
    `ggml_cuda_init: found N CUDA devices` once per load ahead of the block,
    so that line is matched too and starts a new group.

    **Exit 1 is "no matches" and not a failure.** `journalctl -g` exits 1
    when its pattern matches nothing, which is the ordinary answer for a
    window holding no load.
    """
    unit = f"weaver-worker@{cfg['agent']}.service"
    base = ["journalctl", "-u", unit, "--since", since, "--no-pager", "-o", "cat"]
    # Grepped in the journal rather than in this process: an unfiltered read
    # spans every load in the window and llama.cpp is verbose.
    r = sh(base + ["-g", "ggml_cuda_init: found|using device CUDA"])
    if r.returncode not in (0, 1):
        return {"unreadable": f"journalctl exit {r.returncode}: "
                              f"{r.stderr.strip()[:200]}"}
    groups, current = [], None
    for line in r.stdout.splitlines():
        if LOAD_BOUNDARY.search(line):
            if current is not None:
                groups.append(current)
            current = []
            continue
        m = DEVICE_LINE.search(line)
        if m and current is not None:
            current.append({"ordinal": int(m.group(1)),
                            "name": m.group(2),
                            "pci_bus_id": m.group(3)})
    if current is not None:
        groups.append(current)
    groups = [g for g in groups if g]
    if groups:
        return {"groups": groups}
    # No match. Distinguish a journal this user cannot read from a load that
    # genuinely bound no CUDA device, by asking whether the unit logged
    # anything at all. Paid only in the empty case.
    probe = sh(base + ["-n", "1"])
    if probe.returncode != 0 or not probe.stdout.strip():
        return {"unreadable": "the unit's journal read back empty; this user "
                              "is likely in neither systemd-journal nor adm"}
    return {"groups": []}


def spu_binary(cfg):
    """The SPU binary path, from the authority that already holds it.

    `weaver-admin` reads `spu-binary` from its config directory, required
    rather than defaulted, on the stated ground that a missing one refuses
    and names itself rather than being searched for
    (`weaver-admin/src/main.rs`, per Spec section 9). Guessing it beside
    `admin_bin` would re-introduce the search that rule exists to forbid,
    and the crate's own default is `/usr/libexec/weaver-spu` while the
    deploy material uses `/usr/local/libexec/weaver/`, so the two are not
    reliably co-located. The config is read first, an explicit `spu_bin`
    overrides it, and the sibling guess is the last resort rather than the
    first.
    """
    if cfg.get("spu_bin"):
        return cfg["spu_bin"]
    stated = os.path.join(cfg.get("admin_config", ""), "spu-binary")
    try:
        with open(stated) as f:
            named = f.read().strip()
        if named:
            return named
    except OSError:
        pass
    return os.path.join(os.path.dirname(cfg["admin_bin"]), "weaver-spu")


def engine_libraries(cfg):
    """sha256 of the libraries the serving binary actually links.

    Read through `ldd` rather than from a configured list, so the answer is
    what the loader resolves rather than what an operator believed. The
    decode math lives in `libggml-cuda` and `libllama`, and issue #370
    established that a Blackwell figure cannot be attributed while these
    are unrecorded: a cross-box divergence is silicon, libraries, or both,
    and a report that omits them cannot say which.

    **Every failure says which failure it was.** A bare empty result would
    read as "recorded, nothing to record", which is the same silent absence
    the `device` retirement exists to end.
    """
    spu = spu_binary(cfg)
    if not os.path.exists(spu):
        return {"unreadable": f"no SPU binary at {spu}"}
    r = sh(["ldd", spu])
    if r.returncode != 0:
        return {"unreadable": f"ldd exit {r.returncode} on {spu}: "
                              f"{r.stderr.strip()[:200]}"}
    out = {}
    for line in r.stdout.splitlines():
        m = re.search(r"(lib(?:ggml[\w-]*|llama)\.so[\w.]*)\s+=>\s+(\S+)", line)
        if not m:
            continue
        name, path = m.group(1), m.group(2)
        # `ldd` prints `=> not found` for an unresolved library, whose
        # second field is the bare word `not`. Recorded as unresolved
        # rather than hashed as a path.
        if not path.startswith("/"):
            out[name] = {"path": None, "sha256": None, "unresolved": True}
            continue
        try:
            h = hashlib.sha256()
            with open(path, "rb") as f:
                for chunk in iter(lambda: f.read(1 << 20), b""):
                    h.update(chunk)
            out[name] = {"path": path, "sha256": h.hexdigest()}
        except OSError as e:
            out[name] = {"path": path, "sha256": None, "error": str(e)}
    if not out:
        return {"unreadable": f"{spu} links no ggml or llama library"}
    return out


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

    The scan walks backward in chunks and stops one run past the `keep`th, so
    the `keep` newest runs are whole rather than cut at a chunk edge. A
    partial line at a chunk boundary is held for the next block for the same
    reason. `keep=None` reads everything, which is the original behaviour.

    **The oldest run returned is a boundary fragment and carries one event.**
    The stop fires on the first line of the `keep`-plus-first run met going
    backward, so `keep=k` answers k+1 runs and `order[0]` is a stub. Every
    caller here takes `order[-1]` or indexes by a run it already names, so
    the stub is inert, and it is described rather than trimmed because a
    caller that iterated `runs` would otherwise meet it unwarned.

    **A run's events are assumed contiguous**, which holds by construction:
    a run is one load-to-unload cycle against a sequentially served agent, so
    no second run interleaves it. Were they interleaved the backward stop
    could fall inside a run and truncate it, and the interleaving checks
    elsewhere in this harness guard the reissue rather than this scan.
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


def await_turns(trace_path, want, run_id, keep=4, timeout=None):
    """`run_id` once it carries `want` turns, or once time runs out.

    **The run is named by the caller rather than taken as the newest**, per
    the seat's finding of 2026-08-27. A wait on whichever run is newest is
    satisfied by the wrong run when the sink is dead rather than slow: the
    cell before this one left a run of the same length, `want` is met on the
    first read, and the caller reissues against a stale record and reports a
    match that never happened. The close already carries the run it opened,
    so the identity is in hand and is passed.

    **The sink writes after the close answers**, so a read taken the instant
    a turn closes can miss the events that turn produced. This was invisible
    while `read_runs` scanned the whole trace, because the scan itself took
    seconds and the sink caught up inside it. The tail read of 2026-08-27
    removed that accidental delay and the race surfaced as a run one turn
    short, every time, the missing turn always the last. Waiting for the
    record is what the harness meant to do, and an accidental sleep is not a
    way to do it.

    Answers `(turns, events)` with whatever stands when the count is reached
    or the timeout expires, and the caller reports the shortfall - a wait
    that raised would turn a slow sink into a failed cell. **The events come
    back beside the turns** so a caller depositing the run needs no second
    read and holds no `runs` dict of its own.
    """
    # **The bound follows the work.** A deep session flushes many times the
    # events of a shallow one behind a sink that has just spent a minute
    # generating, so a fixed bound is generous for one and tight for the
    # other.
    if timeout is None:
        timeout = 30.0 + 0.5 * want
    end = time.time() + timeout
    turns, events = [], []
    delay = 0.02
    while True:
        _, runs = read_runs(trace_path, keep=keep)
        if run_id in runs:
            events = runs[run_id]
            turns = cut_turns(events)
            if len(turns) >= want:
                return turns, events
        if time.time() >= end:
            return turns, events
        time.sleep(delay)
        # Backing off rather than polling flat: the scan has a one mebibyte
        # floor, so a wait that runs to its bound reads on the order of a
        # gigabyte to learn nothing.
        delay = min(delay * 1.5, 1.0)


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


def cell_metadata(cfg, cell, libraries):
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
        # **`device` is retired rather than redefined**, per issue #370. It
        # held this whole-machine listing, so every GPU present appeared in
        # every report and the one that answered appeared nowhere. Reusing
        # the key for the serving device would leave old and new reports
        # disagreeing in meaning under one name, which is worse than either
        # meaning: the reader cannot tell which they hold. The listing keeps
        # a name that says what it is, and `serving_device` is filled in
        # after the load by the party that knows.
        "machine_gpus": gpu,
        # Filled in after each load by the party that knows. `source` and
        # `replay` are recorded apart because they are two loads and may
        # not bind the same devices, per finding 3 of the olympus seat.
        "serving_device": {"source": None, "replay": None},
        # Hoisted: the libraries cannot change during a run and
        # `libggml-cuda` built for four architectures is 142 MiB to hash.
        "engine_libraries": libraries,
        "build_flags": cfg["build_flags"],
        "rustc": rustc,
        "commit": commit,
        "short_text": SHORT_TEXT,
        "long_text": LONG_TEXT,
    }


def run_cell(cfg, cell, outdir, libraries):
    name = cell["name"]
    log = lambda m: print(f"[{name}] {m}", flush=True)
    report = {"cell": name, "metadata": cell_metadata(cfg, cell, libraries),
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
        # The window opens before the load so the journal read below cannot
        # reach back to a previous cell's load, and a second of slack
        # absorbs the clock skew between this process and the journal's
        # own timestamps.
        since = time.strftime(
            "%Y-%m-%d %H:%M:%S", time.localtime(time.time() - 1))
        if step("load").get("kind") != "state":
            report["verdict"] = "load refused"
            return report
        if not wait_socket(cfg):
            report["verdict"] = "gate socket never stood"
            return report

        # **What served is read from the worker and not from the machine**,
        # per issue #370's third ask. Read after the socket stands, so the
        # load has reached the point of binding a device rather than merely
        # having been asked to.
        report["metadata"]["serving_device"]["source"] = serving_device(cfg, since)
        log(f"source devices: "
            f"{json.dumps(report['metadata']['serving_device']['source'])}")

        log("serving the source turns")
        source_runs = set()
        for text in (SHORT_TEXT, LONG_TEXT):
            close = gate_turn(cfg, text)
            log(f"close {close.get('kind')} turn {close.get('turn')}")
            if close.get("kind") != "answered":
                report["verdict"] = f"source turn not answered: {close}"
                return report
            source_runs.add(close.get("run"))

        # The closes name the run, so the wait is on that run and not on
        # whichever is newest, per the seat's finding of 2026-08-27.
        if len(source_runs) != 1 or None in source_runs:
            report["verdict"] = (
                "the source turns did not share one run: "
                f"{sorted(map(str, source_runs))}"
            )
            return report
        source_run = source_runs.pop()
        source_turns, source_events = await_turns(cfg["trace"], 2, source_run)
        if len(source_turns) != 2:
            report["verdict"] = f"expected 2 source turns, found {len(source_turns)}"
            return report

        step("unload")
        replay_since = time.strftime(
            "%Y-%m-%d %H:%M:%S", time.localtime(time.time() - 1))
        if step("load").get("kind") != "state":
            report["verdict"] = "reload refused"
            return report
        if not wait_socket(cfg):
            report["verdict"] = "gate socket never stood after reload"
            return report

        # **The reissue half binds its own devices and they are read too.**
        # A cell that compared a source on one card against a replay on
        # another and called it REPRODUCED would be the olympus A6000 error
        # relocated to the second half, per finding 3 of the olympus seat.
        # A disagreement fails the cell rather than being recorded and
        # passed over: the comparison the cell exists to make is not
        # between these two runs.
        report["metadata"]["serving_device"]["replay"] = serving_device(
            cfg, replay_since)
        src = report["metadata"]["serving_device"]["source"]
        rep = report["metadata"]["serving_device"]["replay"]
        log(f"replay devices: {json.dumps(rep)}")
        if src != rep:
            report["verdict"] = (
                "source and replay did not bind the same devices: "
                f"{json.dumps(src)} against {json.dumps(rep)}")
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

        replay_all, replay_events = await_turns(
            cfg["trace"], len(source_turns), replay_run)
        if not replay_all:
            report["verdict"] = (
                f"the closes named run {replay_run} but the trace does not carry"
                " it - sink lag or a different sink"
            )
            return report
        replay_turns = {t["turn"]: t for t in replay_all}
        if len(replay_all) != len(source_turns):
            log(f"turn count differs: source {len(source_turns)} replay {len(replay_all)}")

        # A replay with surplus turns is interleaved traffic and is
        # never a match, even when every source turn agrees.
        # A short replay read is the sink rather than the model, and it
        # reaches its own verdict rather than being folded into a mismatch.
        if len(replay_all) < len(source_turns):
            report["verdict"] = (
                f"replay read short: expected {len(source_turns)} turns,"
                f" found {len(replay_all)} - the record is incomplete"
            )
            return report
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

        # **Deposited from a fresh read rather than from the snapshots the
        # comparison used.** Those were taken the moment `cut_turns` was
        # satisfied, which on the source side is before its unload, so a
        # deposit made from them holds a run with no closing event and a
        # consumer cannot tell from the file that the run ended cleanly. The
        # comparison is unaffected either way and this costs one read a cell.
        _, whole = read_runs(cfg["trace"], keep=6)
        for label, run in (("source", source_run), ("replay", replay_run)):
            run_events = whole.get(run) or (
                source_events if label == "source" else replay_events)
            with open(os.path.join(outdir, f"cell-{name}-{label}.ndjson"), "w") as f:
                for e in run_events:
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
    # Read once for the run: the libraries cannot change under it, and
    # `libggml-cuda` built for four architectures is 142 MiB to hash.
    libraries = engine_libraries(cfg)
    print(f"engine libraries: {json.dumps(libraries)}", flush=True)
    reports = []
    try:
        for cell in cfg["cells"]:
            reports.append(run_cell(cfg, cell, args.outdir, libraries))
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
