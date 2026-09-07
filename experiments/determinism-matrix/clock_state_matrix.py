#!/usr/bin/env python3
"""The clock-state matrix: does the reproduction survive a clock change?

The determinism matrix held bit-exact on this box three times, and every
time under a tight loop that kept the card boosted and thermally steady.
The question this asks is whether any member of the reproducibility tuple
varies inside a single load, and clock state is the one substrate
condition that moves during a run without anyone electing it.

**An idle between turns cannot move the clock on this card**, which is why
this driver locks it instead. Measured 2026-09-06 on the RTX A6000 under
driver 610.57.04: with any CUDA context held and no work running the card
sits at 1800 MHz in P2, and it falls to P8 only about a second after the
context is released. The SPU holds its context for the whole load, so an
idle inside a load moves temperature and leaves the SM clock where it was.
`nvidia-smi -lgc` moves it, and the hardware honors the lock within
0.3 s while the readout lags it by one to two seconds, which is why the
lock below waits for the readout to agree before anything dispatches.

**The lock is drawn once per half, independently.** A session serves its
turns under one clock, unloads, reloads, and reissues them under another,
each drawn from the same set without regard to the other. The comparison
the matrix makes is between the source turn and its reissue, so a
different clock on each side is a clock contrast on exactly that
comparison. A clock drawn per turn would add transitions inside a half
and cost a settle wait at every one of the 928 dispatches in a sweep.

**Every dispatch is still read.** The clock, memory clock, power, and
temperature are sampled at each turn's dispatch and again at its close,
so a divergence, if one appears, is placed against a clock state and not
merely against a turn index. The readout is `nvidia-smi dmon -s pc`, and
the deposit says so, because the `--query-gpu=clocks.sm` path reports
210 MHz at full load on this driver and must not be trusted.

Everything else is the determinism matrix unchanged: the same protocol,
the same prompts, depths, sweep order, comparison, and summary, imported
and run rather than restated. This driver patches three seams in that
harness, the admin `load`, the gate turn, and the session, and touches
nothing else. The card is unlocked on return, on an exception, on
Ctrl-C, and on SIGTERM.

Run:

    python3 clock_state_matrix.py --config <cfg> --outdir <deposit> --hours 7
"""

import argparse
import json
import os
import random
import hashlib
import signal
import subprocess
import sys
import threading
import time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, "..", "cross-precision-repro"))
import confirm_cells as base  # noqa: E402
import determinism_matrix as dm  # noqa: E402

# The P8 floor, the max boost, and six points between. The card's lock
# points run 210 to 2100 in 523 steps, and these are the ones drawn.
CLOCKS = [210, 420, 600, 900, 1200, 1500, 1800, 2100]

READOUT = "nvidia-smi dmon -s pc -c 1"


def smi(args, timeout):
    """`nvidia-smi` under a bound, a hang reported as a failed command.

    Every call here is on the path to the unlock: the sampler thread that
    `gate_turn` joins, the settle loop before a load, and the reset in the
    `finally`. An `nvidia-smi` that never returns would hold all three, so
    each call is bounded and a timeout comes back as a non-zero result
    rather than as an exception the caller did not plan for.
    """
    try:
        return subprocess.run(args, capture_output=True, text=True,
                              timeout=timeout)
    except subprocess.TimeoutExpired:
        return subprocess.CompletedProcess(
            args, 124, "", f"nvidia-smi timed out after {timeout}s")


def dmon(device):
    """One sample of power, temperature, memory clock, and SM clock."""
    r = smi(["nvidia-smi", "dmon", "-i", str(device), "-s", "pc", "-c", "1"],
            timeout=5)
    lines = [l for l in r.stdout.splitlines() if l.strip() and not l.startswith("#")]
    if r.returncode != 0 or not lines:
        return {"unreadable": (r.stderr or r.stdout).strip()[:200]}
    f = lines[-1].split()
    try:
        return {"pwr_w": float(f[1]), "gtemp_c": float(f[2]),
                "mclk_mhz": float(f[4]), "pclk_mhz": float(f[5]),
                "at": time.time()}
    except (IndexError, ValueError):
        return {"unreadable": lines[-1]}


def lock(device, mhz, settle=5.0):
    """Lock the graphics clock and wait for the readout to agree.

    The hardware applies the lock within 0.3 s and the readout follows one
    to two seconds later. Waiting for the readout is what makes the
    recorded clock a reading rather than a request. A readout that never
    agrees is recorded as unsettled, and the run goes on, because the
    request was honored and the record says which it was.
    """
    r = smi(["sudo", "-n", "nvidia-smi", "-i", str(device), "-lgc", str(mhz)],
            timeout=10)
    t0 = time.time()
    if r.returncode != 0:
        return {"requested_mhz": mhz, "applied": False,
                "error": (r.stderr or r.stdout).strip()[:200]}
    end = t0 + settle
    last = None
    while time.time() < end:
        last = dmon(device)
        if last.get("pclk_mhz") == mhz:
            return {"requested_mhz": mhz, "applied": True, "settled": True,
                    "settle_s": round(time.time() - t0, 2), "reading": last}
        time.sleep(0.25)
    return {"requested_mhz": mhz, "applied": True, "settled": False,
            "settle_s": round(time.time() - t0, 2), "reading": last}


def unlock(device):
    r = smi(["sudo", "-n", "nvidia-smi", "-i", str(device), "-rgc"], timeout=10)
    return r.returncode == 0


class Condition:
    """The clock state of the session in hand, and every dispatch under it."""

    def __init__(self, rng, device, log):
        self.rng, self.device, self.log = rng, device, log
        self.reset()

    def reset(self):
        self.halves = []       # one entry per load: the draw and its settle
        self.dispatches = []   # one entry per gate turn, in dispatch order

    def next_half(self):
        mhz = self.rng.choice(CLOCKS)
        got = lock(self.device, mhz)
        got["half"] = len(self.halves)
        self.halves.append(got)
        self.log(f"  half {got['half']}: lock {mhz} MHz, "
                 f"{'settled' if got.get('settled') else 'UNSETTLED'} "
                 f"in {got.get('settle_s')}s")
        return got


def install(cond):
    """Patch the three seams. The originals run unchanged inside."""
    original_admin, original_turn, original_session = (
        base.admin, base.gate_turn, dm.run_session)

    def admin(cfg, verb):
        # The lock goes on before the load so the load, the socket, and
        # every turn of the half run under one clock.
        if verb == "load":
            cond.next_half()
        return original_admin(cfg, verb)

    def gate_turn(cfg, text, timeout=600):
        # **The clock that ran is the condition, not the clock requested.**
        # A 2100 lock under the 250 W cap may not hold, so the card is
        # sampled while the turn runs, and the record carries the range it
        # saw. The readout lags the hardware by one to two seconds, so a
        # turn shorter than that shows the settled pre-turn reading and
        # nothing else, which the sample count makes visible.
        half = len(cond.halves) - 1
        before = dmon(cond.device)
        seen, stop = [], threading.Event()

        def sample():
            while not stop.is_set():
                seen.append(dmon(cond.device))
                stop.wait(0.25)

        th = threading.Thread(target=sample, daemon=True)
        th.start()
        try:
            close = original_turn(cfg, text, timeout)
        finally:
            stop.set()
            th.join()
        after = dmon(cond.device)
        pclk = [x["pclk_mhz"] for x in seen if "pclk_mhz" in x]
        cond.dispatches.append({
            "half": half,
            "ordinal": sum(1 for d in cond.dispatches if d["half"] == half) + 1,
            "requested_mhz": cond.halves[half]["requested_mhz"] if half >= 0 else None,
            "at_dispatch": before,
            "during": {
                "samples": len(pclk),
                "pclk_min_mhz": min(pclk) if pclk else None,
                "pclk_max_mhz": max(pclk) if pclk else None,
                "pwr_max_w": max((x["pwr_w"] for x in seen if "pwr_w" in x), default=None),
                "gtemp_max_c": max((x["gtemp_c"] for x in seen if "gtemp_c" in x), default=None),
            },
            "at_close": after,
        })
        return close

    def run_session(cfg, probe, depth, iteration):
        cond.reset()
        rec = original_session(cfg, probe, depth, iteration)
        rec["clock"] = {"readout": READOUT, "halves": cond.halves,
                        "dispatches": cond.dispatches}
        # The same samples, hung on the turn they belong to. The source
        # half dispatches the texts in order and the replay half reissues
        # the source turns in order, so ordinal is turn position on both
        # sides.
        by = {(d["half"], d["ordinal"]): d for d in cond.dispatches}
        for i, t in enumerate(rec.get("turns", []), start=1):
            s, r = by.get((0, i)), by.get((1, i))
            t["clock"] = {
                "source_mhz": s and s["requested_mhz"],
                "replay_mhz": r and r["requested_mhz"],
                "source_at_dispatch": s and s["at_dispatch"],
                "replay_at_dispatch": r and r["at_dispatch"],
                "source_during": s and s["during"],
                "replay_during": r and r["during"],
            }
        return rec

    base.admin, base.gate_turn, dm.run_session = admin, gate_turn, run_session


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--config", required=True)
    ap.add_argument("--outdir", required=True)
    ap.add_argument("--hours", type=float, default=7.0)
    ap.add_argument("--clock-seed", type=int, default=20260906,
                    help="seed of the draw, so the sequence of clocks is reproducible")
    args = ap.parse_args()

    with open(args.config) as f:
        cfg = json.load(f)
    device = int(cfg.get("device", 0))
    os.makedirs(args.outdir, exist_ok=True)
    logpath = os.path.join(args.outdir, "clock.log")

    def log(msg):
        line = f"[{time.strftime('%H:%M:%S')}] {msg}"
        print(line, flush=True)
        with open(logpath, "a") as fh:
            fh.write(line + "\n")

    # Refuse before touching the agent if the instrument is not there.
    probe = dmon(device)
    if "unreadable" in probe:
        print(f"dmon unreadable on device {device}: {probe}", file=sys.stderr)
        sys.exit(2)
    if not unlock(device):
        print("cannot reset the graphics clock under sudo -n", file=sys.stderr)
        sys.exit(2)

    # **The declaration is named by hash**, because the one on the agent
    # path has moved since the freeze and goes back after this run. A
    # deposit that does not say which declaration ran is one the
    # declaration does not describe.
    with open(cfg["declaration"], "rb") as fh:
        declaration_sha = hashlib.sha256(fh.read()).hexdigest()
    rng = random.Random(args.clock_seed)
    with open(os.path.join(args.outdir, "clock-facts.json"), "w") as fh:
        json.dump({
            "device": device,
            "declaration": {"path": cfg["declaration"], "sha256": declaration_sha},
            "clocks_mhz": CLOCKS,
            "draw": "uniform over clocks_mhz, once per load, seeded",
            "clock_seed": args.clock_seed,
            "readout": READOUT,
            "readout_note": ("nvidia-smi --query-gpu=clocks.sm reports 210 MHz at "
                             "full load on driver 610.57.04 and is not used; "
                             "dmon agrees with achieved throughput"),
            "lock": "sudo nvidia-smi -lgc <mhz> before each load, -rgc at exit",
            "idle_reading_before_run": probe,
        }, fh, indent=1)
    log(f"clock-state matrix on device {device}: clocks {CLOCKS}, seed {args.clock_seed}")
    log(f"declaration {cfg['declaration']} sha256 {declaration_sha}")

    cond = Condition(rng, device, log)
    install(cond)
    # **SIGTERM is turned into an exception so the `finally` below runs.**
    # Python raises nothing into the stack on SIGTERM, so a `kill`, a unit
    # stop, or a shutdown during a seven-hour run would otherwise leave the
    # card at whatever lock the current half drew. Raised as `SystemExit`,
    # it passes through the matrix's own `finally`, which unloads the agent
    # and restores the declaration, before reaching the unlock here. Found
    # by the thinkpad review seat on PR #483.
    def on_term(signum, frame):
        raise SystemExit(128 + signum)

    signal.signal(signal.SIGTERM, on_term)
    sys.argv = [sys.argv[0], "--config", args.config, "--outdir", args.outdir,
                "--hours", str(args.hours)]
    try:
        dm.main()
    finally:
        ok = unlock(device)
        time.sleep(2.5)  # the readout lags the reset like it lags the lock
        log(f"graphics clock reset: {'ok' if ok else 'FAILED'}, "
            f"reading {dmon(device)}")


if __name__ == "__main__":
    main()
