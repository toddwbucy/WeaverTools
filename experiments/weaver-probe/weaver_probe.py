#!/usr/bin/env python3
"""The Weaver probe: one essay, one held tuple, the divergence read twice.

Pre-registered at issue #511 and the vehicle for the arms of issue #485.
One prompt asking for a long essay on Warren Weaver is run repeatedly under
one declared tuple with the declared seed drawn from a schedule, every other
field held. Two readings follow, and they are never folded into one number:

  reading one, the free run     two independent runs compared from the
                                first position they differ, and how far
                                they travel apart after it - the cascade
  reading two, the re-fed run   one run's token path fed back through
                                another arrangement under the diagnostic
                                binding, per-position distributions compared
                                under identical context - the arithmetic

Three verbs:

    weaver_probe.py run    --config ARM.json
    weaver_probe.py refeed --config ARM.json --source RUN_DIR [--as-arm OTHER.json]
    weaver_probe.py read   --deposit DIR

`run` writes one declaration per run, each with its own session and its own
trace sink, so every record holds exactly one run and `derive` and `preload`
read it whole. The agent's standing declaration is restored on exit whichever
path the run takes. `refeed` derives a diagnostic declaration from a source
run's record, points it at the target arm's artifact and device where those
differ, and drives the two-shell diagnostic flow, the load in one and the
preload in the other. `read` computes both readings from the deposit alone.
"""
import argparse
import hashlib
import json
import math
import os
import re
import subprocess
import sys
import time

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                "..", "cross-precision-repro"))
import confirm_cells as base  # noqa: E402

# Pinned. The subject is chosen for having no ground truth to score against
# and enough material that the model will not stall; the length is the
# point, giving a cascade room to show whether an early divergence stays a
# synonym or becomes a different essay. Do not edit per arm.
ESSAY_PROMPT = (
    "Write a long essay of about five thousand words on Warren Weaver: his early "
    "life and mathematical training, his years at the Rockefeller Foundation and "
    "what he chose to fund there, his wartime work on fire control and the Applied "
    "Mathematics Panel, his 1949 memorandum on machine translation and what it "
    "proposed, his collaboration with Claude Shannon and the essay he wrote to "
    "accompany the mathematical theory of communication, his later writing on "
    "science and the public, and an assessment of which of his ideas held up and "
    "which did not. Write it as continuous prose in sections with headings, and do "
    "not stop early."
)


def log(deposit, msg):
    line = f"[{time.strftime('%H:%M:%S')}] {msg}"
    print(line, flush=True)
    with open(os.path.join(deposit, "probe.log"), "a") as fh:
        fh.write(line + "\n")


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 24), b""):
            h.update(chunk)
    return h.hexdigest()


# ---------------------------------------------------------------- declaration

def render_declaration(cfg, seed, session, sink_path):
    """The tuple as one YAML document: every field from the arm's config and
    the two that vary per run, the seed and the record's home."""
    devices = ", ".join(str(d) for d in cfg["devices"])
    identity = json.dumps(cfg.get("identity", "You are Karl, a small local agent. "
                                  "Answer plainly and at length when asked."))
    return (
        f"session: {session}\n"
        "spu-instruction:\n"
        "  decoder:\n"
        "    model-binding:\n"
        f"      artifact: {cfg['artifact']}\n"
        f"      devices: [{devices}]\n"
        "    residual-readout-election: false\n"
        "    surprisal-election: true\n"
        "    field-election:\n"
        f"      depth: {cfg['field_depth']}\n"
        "    identity:\n"
        "      - role: system\n"
        "        content:\n"
        "          - type: text\n"
        f"            text: {identity}\n"
        "    tunable-values:\n"
        f"      seed: {seed}\n"
        f"      context-capacity: {cfg['context_capacity']}\n"
        f"      max-tokens-per-turn: {cfg['max_tokens']}\n"
        f"loop-file: {cfg['loop_file']}\n"
        "tool-set: []\n"
        "permission-mode: ask\n"
        "gate-instruction:\n"
        "  access-rule:\n"
        f"    allowed-uids: [{cfg.get('operator_uid', 1000)}]\n"
        "    allowed-gids: []\n"
        "    denied-uids: []\n"
        "trace-sink:\n"
        "  kind: file\n"
        f"  path: {sink_path}\n"
        "  create: true\n"
    )


def agent_declaration_path(cfg):
    return os.path.join(cfg["agents_dir"], f"{cfg['agent']}.yaml")


def ensure_sink_dir(path):
    """A sink directory the boundary admits: root-owned, the agent's group,
    mode 2750, which is the shape every passing sink on this box has."""
    if os.path.isdir(path):
        return
    subprocess.run(["sudo", "-n", "install", "-d", "-m", "2750", "-o", "root",
                    "-g", os.environ.get("WEAVER_SINK_GROUP", "todd"), path], check=True)


# ---------------------------------------------------------------- the record

def read_privileged(path):
    """A sink file is the admin principal's, mode 0640 to its group, so a
    reader outside that group reads it as the principal. Falls back to a
    plain read where the file is readable."""
    try:
        return open(path).read()
    except PermissionError:
        return subprocess.run(["sudo", "-n", "cat", path], capture_output=True, text=True).stdout


def events_of_run(trace_path, run_id):
    out = []
    with open(trace_path) as fh:
        for line in fh:
            if f'"run":"{run_id}"' not in line:
                continue
            try:
                out.append(json.loads(line))
            except json.JSONDecodeError:
                continue
    return out


def extract_run(events):
    """What a probe run is read by: the emission, its tokens, the per-position
    entropies and surprisals, and the ranked field at each position."""
    rec = {"emission": None, "output_tokens": [], "entropies": [], "surprisals": None,
           "field": {}, "finish": None, "declared_seed": None, "generation_seed": None,
           "timings": None, "weights_hash": None, "model": None}
    for e in events:
        k, p = e.get("kind"), e.get("payload") or {}
        if k == "model.request":
            rec["declared_seed"] = p.get("sampling", {}).get("seed")
            rec["generation_seed"] = p.get("sampling", {}).get("generation_seed")
        elif k == "model.output":
            rec["emission"] = p.get("emission")
            rec["finish"] = p.get("finish")
        elif k == "model.measurement":
            rec["output_tokens"] = p.get("output_tokens", [])
            rec["entropies"] = p.get("entropies", [])
            rec["surprisals"] = p.get("surprisals")
            rec["timings"] = p.get("timings")
            rec["weights_hash"] = p.get("weights_hash")
            rec["model"] = p.get("model")
        elif k == "model.field":
            rec["field"][int(p["position"])] = {"ranked": p["ranked"], "realized": p["realized"]}
    return rec


def wait_for_close(trace_path, run_id, timeout):
    """Until the run's turn closes in the record, or the bound."""
    end = time.time() + timeout
    while time.time() < end:
        if os.path.exists(trace_path):
            with open(trace_path) as fh:
                for line in fh:
                    if f'"run":"{run_id}"' in line and '"kind":"turn.closed"' in line:
                        return True
        time.sleep(1.0)
    return False


# ---------------------------------------------------------------- reading one

def first_divergence(tokens_a, tokens_b):
    """The first position where two token paths differ, or None only where
    the two are identical. A run that stopped early diverges at its own end:
    where one path is a prefix of the other, the divergence is the shorter
    length, because a shorter essay is a different essay."""
    n = min(len(tokens_a), len(tokens_b))
    for i in range(n):
        if tokens_a[i] != tokens_b[i]:
            return i
    return None if len(tokens_a) == len(tokens_b) else n


def first_char_divergence(text_a, text_b):
    n = min(len(text_a), len(text_b))
    for i in range(n):
        if text_a[i] != text_b[i]:
            return i
    return None if len(text_a) == len(text_b) else n


def aligned_agreement_after(tokens_a, tokens_b, start):
    """Of the positions both paths hold past the divergence, the fraction
    holding the same token at the same index: a crude drift, honest about
    what it is. Two essays that re-converge for a stretch score above zero."""
    n = min(len(tokens_a), len(tokens_b))
    if start is None or start >= n:
        return None
    same = sum(1 for i in range(start, n) if tokens_a[i] == tokens_b[i])
    return same / (n - start)


def reading_one(a, b):
    """Two free runs compared: where they part, and how far they travel."""
    ta, tb = a["output_tokens"], b["output_tokens"]
    div = first_divergence(ta, tb)
    return {
        "first_token_divergence": div,
        "first_char_divergence": first_char_divergence(a["emission"] or "", b["emission"] or ""),
        "tokens": [len(ta), len(tb)],
        "aligned_agreement_after": aligned_agreement_after(ta, tb, div),
        "identical": div is None,
        "entropy_mean": [mean(a["entropies"]), mean(b["entropies"])],
        "emission_sha256": [sha256_text(a["emission"]), sha256_text(b["emission"])],
    }


def mean(xs):
    return round(sum(xs) / len(xs), 6) if xs else None


def sha256_text(text):
    return hashlib.sha256((text or "").encode()).hexdigest()


# ---------------------------------------------------------------- reading two

def truncated_kl(ranked_p, ranked_q):
    """KL(p || q) between the two distributions conditioned on the candidates
    both rankings hold, each side renormalized over that shared support, so
    the value is a divergence and never negative. Two truncations meet here
    and are named apart: the field election truncated the vocabulary to a
    depth, whose mass is `ranked_mass_*`, and this metric truncates again to
    the shared support, whose mass is `shared_mass_*`, which is what the
    metric saw. Candidates one side holds and the other does not are excluded
    rather than given a floor, and their mass is `excluded_mass_*`. Where the
    shared support is empty the KL is None rather than zero."""
    p = {c["token"]: c["probability"] for c in ranked_p}
    q = {c["token"]: c["probability"] for c in ranked_q}
    shared = [t for t in p if t in q and p[t] > 0 and q[t] > 0]
    shared_p = sum(p[t] for t in shared)
    shared_q = sum(q[t] for t in shared)
    if not shared:
        kl = None
    else:
        kl = sum((p[t] / shared_p) * math.log((p[t] / shared_p) / (q[t] / shared_q)) for t in shared)
        kl = max(kl, 0.0) / math.log(2)  # the floor absorbs rounding below zero
    return {
        "kl_bits": kl,
        "shared_candidates": len(shared),
        "shared_mass_p": shared_p,
        "shared_mass_q": shared_q,
        "ranked_mass_p": sum(p.values()),
        "ranked_mass_q": sum(q.values()),
        "excluded_mass_p": sum(p[t] for t in p if t not in q),
        "excluded_mass_q": sum(q[t] for t in q if t not in p),
    }


# A position whose shared support carries less than this fraction of the
# ranked mass is counted as thin: a KL conditioned on one or two candidates
# out of two hundred is a divergence over almost none of the distribution,
# and reads as zero by construction where the support is one. The reading
# says how many such positions it holds so a reader taking the maximum alone
# knows how much of itself the metric could see.
THIN_SUPPORT_FRACTION = 0.5


def reading_two(free, refed):
    """The free run against its re-fed replay, position by position under
    identical context: the entropies to the bit, and the ranked field by a
    truncated KL with its coverage stated. **The entropies are compared with
    `==` on purpose**: the claim is bit identity of two computations under
    one context, and a tolerance would retire that claim silently."""
    ea, eb = free["entropies"], refed["entropies"]
    n = min(len(ea), len(eb))
    exact = sum(1 for i in range(n) if ea[i] == eb[i])
    diffs = [abs(ea[i] - eb[i]) for i in range(n)]
    first = next((i for i in range(n) if ea[i] != eb[i]), None)
    # Positions round-trip through JSON as strings, so both sides are read
    # by integer position whatever spelling the store gave them.
    free_field = {int(k): v for k, v in free["field"].items()}
    refed_field = {int(k): v for k, v in refed["field"].items()}
    kls = []
    for pos, fa in free_field.items():
        fb = refed_field.get(pos)
        if fb is None:
            continue
        kls.append((pos, truncated_kl(fa["ranked"], fb["ranked"])))
    return {
        "positions_compared": n,
        "entropy_positions_exact": exact,
        "entropy_first_difference": first,
        "entropy_mean_abs_diff": mean(diffs),
        "entropy_max_abs_diff": max(diffs) if diffs else None,
        "field_positions_compared": len(kls),
        "field_kl_bits_mean": mean([k["kl_bits"] for _, k in kls if k["kl_bits"] is not None]),
        "field_kl_bits_max": max((k["kl_bits"] for _, k in kls if k["kl_bits"] is not None), default=None),
        "field_first_nonzero_kl": next((pos for pos, k in kls if k["kl_bits"] is None or k["kl_bits"] > 0), None),
        "field_positions_no_shared_support": sum(1 for _, k in kls if k["kl_bits"] is None),
        "field_positions_thin_support": sum(
            1 for _, k in kls
            if k["ranked_mass_p"] > 0 and k["shared_mass_p"] / k["ranked_mass_p"] < THIN_SUPPORT_FRACTION),
        "field_thin_support_fraction": THIN_SUPPORT_FRACTION,
        "field_positions_single_shared_candidate": sum(1 for _, k in kls if k["shared_candidates"] == 1),
        "field_shared_mass_mean": mean([k["shared_mass_p"] for _, k in kls]),
        "field_ranked_mass_mean": mean([k["ranked_mass_p"] for _, k in kls]),
        "tokens_equal": free["output_tokens"] == refed["output_tokens"],
    }


# ---------------------------------------------------------------- the verbs

def run_free(cfg):
    deposit = cfg["deposit"]
    os.makedirs(deposit, exist_ok=True)
    sink_dir = os.path.join(deposit, "sink")
    ensure_sink_dir(sink_dir)
    decl_path = agent_declaration_path(cfg)
    original = open(decl_path).read() if os.path.exists(decl_path) else None
    log(deposit, f"probe run, arm {cfg['arm']}, artifact {cfg['artifact']}, devices {cfg['devices']}, "
                 f"seeds {cfg['seeds']} x {cfg['runs_per_seed']}, field depth {cfg['field_depth']}, "
                 f"context {cfg['context_capacity']}, max tokens {cfg['max_tokens']}")
    log(deposit, f"artifact sha256 {sha256_file(cfg['artifact'])}")
    try:
        for n in range(cfg["runs_per_seed"]):
            for seed in cfg["seeds"]:
                name = f"{cfg['arm']}-s{seed}-n{n + 1}"
                run_dir = os.path.join(deposit, "runs", name)
                os.makedirs(run_dir, exist_ok=True)
                session = f"{cfg.get('session_prefix', 'probe')}-{name}"
                sink = os.path.join(sink_dir, f"{name}.ndjson")
                with open(decl_path, "w") as fh:
                    fh.write(render_declaration(cfg, seed, session, sink))
                rec = {"name": name, "arm": cfg["arm"], "seed": seed, "session": session,
                       "trace": sink, "verdict": None}
                started = time.time()
                try:
                    base.admin(cfg, "unload")
                    if base.admin(cfg, "load").get("kind") != "state":
                        rec["verdict"] = "load refused"
                        continue
                    if not base.wait_socket(cfg):
                        rec["verdict"] = "gate socket never stood"
                        continue
                    close = base.gate_turn(cfg, ESSAY_PROMPT, timeout=cfg.get("turn_timeout_s", 3600))
                    if close.get("kind") != "answered":
                        rec["verdict"] = f"turn not answered: {close.get('kind')}"
                        continue
                    rec["run"] = close.get("run")
                    if not wait_for_close(sink, rec["run"], 120):
                        rec["verdict"] = "the turn never closed in the record"
                        continue
                    ex = extract_run(events_of_run(sink, rec["run"]))
                    rec.update({k: ex[k] for k in ("declared_seed", "generation_seed", "finish",
                                                    "timings", "weights_hash", "model")})
                    rec["tokens"] = len(ex["output_tokens"])
                    rec["words"] = len((ex["emission"] or "").split())
                    rec["field_positions"] = len(ex["field"])
                    rec["entropy_mean"] = mean(ex["entropies"])
                    rec["emission_sha256"] = sha256_text(ex["emission"])
                    if ex["declared_seed"] != seed:
                        rec["verdict"] = f"the declared seed did not reach the record: {seed} vs {ex['declared_seed']}"
                        continue
                    with open(os.path.join(run_dir, "run.json"), "w") as fh:
                        json.dump({**rec, **ex}, fh)
                    rec["verdict"] = "RAN"
                except Exception as exc:  # an unattended run records rather than dies
                    rec["verdict"] = f"error: {type(exc).__name__}: {exc}"
                finally:
                    base.admin(cfg, "unload")
                    rec["seconds"] = round(time.time() - started, 1)
                    with open(os.path.join(deposit, "probe.jsonl"), "a") as fh:
                        fh.write(json.dumps(rec) + "\n")
                    log(deposit, f"{name}: {rec['verdict']} ({rec['seconds']}s) tokens={rec.get('tokens')} "
                                 f"words={rec.get('words')} H_mean={rec.get('entropy_mean')} finish={rec.get('finish')}")
    finally:
        if original is not None:
            with open(decl_path, "w") as fh:
                fh.write(original)
        base.admin(cfg, "unload")


def run_refeed(cfg, source_dir, as_arm):
    """One run's token path fed back through an arrangement, under the
    diagnostic binding: the source arm's own, or another arm's artifact and
    device where `--as-arm` names one. The two-shell flow: the load parks
    until the preload seals, so the preload runs beside it."""
    deposit = cfg["deposit"]
    src = json.load(open(os.path.join(source_dir, "run.json")))
    target = as_arm or cfg
    name = f"refeed-{os.path.basename(source_dir)}-under-{target['arm']}"
    out_dir = os.path.join(deposit, "refeeds", name)
    os.makedirs(out_dir, exist_ok=True)
    sink_dir = os.path.join(deposit, "sink")
    ensure_sink_dir(sink_dir)
    diag_sink = os.path.join(sink_dir, f"{name}.ndjson")
    derived = os.path.join(out_dir, "derived.yaml")
    devices = ",".join(str(d) for d in target["devices"])
    r = subprocess.run([cfg["analysis_bin"], "derive", src["trace"], "--devices", devices,
                        "--sink", diag_sink, "--field-depth", str(target["field_depth"]),
                        "--surprisal", "--out", derived], capture_output=True, text=True)
    if r.returncode != 0:
        log(deposit, f"{name}: derive refused: {r.stderr.strip()[:300]}")
        return
    text = open(derived).read()
    if target["artifact"] != src["model"]:
        text, k = re.subn(r"(artifact:\s*).*", r"\g<1>" + target["artifact"], text, count=1)
        assert k == 1
    with open(derived, "w") as fh:
        fh.write(text)
    decl_path = agent_declaration_path(cfg)
    original = open(decl_path).read() if os.path.exists(decl_path) else None
    with open(decl_path, "w") as fh:
        fh.write(text)
    territory = os.path.join(sink_dir, "state")
    door = os.path.join(territory, "preload.sock")
    rec = {"name": name, "source": src["name"], "source_run": src["run"], "target_arm": target["arm"],
           "target_artifact": target["artifact"], "target_devices": target["devices"], "trace": diag_sink}
    started = time.time()
    preload = None
    try:
        base.admin(cfg, "unload")
        # The load parks on the door until the preload seals, so it is
        # started in the background and the preload dialled once the door stands.
        loader = subprocess.Popen(["sudo", "-n", f"WEAVER_ADMIN_CONFIG={cfg['admin_config']}",
                                   cfg["admin_bin"], "load", cfg["agent"]],
                                  stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        end = time.time() + 120
        while time.time() < end and not os.path.exists(door):
            time.sleep(0.5)
        if not os.path.exists(door):
            rec["verdict"] = "the preload door never stood"
            loader.kill()
            return
        # The door is the member's, mode 0700 to the principal that stood it,
        # so the driver dials it as that principal.
        preload = subprocess.run(["sudo", "-n", cfg["analysis_bin"], "preload", src["trace"], door],
                                 capture_output=True, text=True, timeout=600)
        out, _ = loader.communicate(timeout=600)
        answer = (out.strip().splitlines() or ["{}"])[-1]
        rec["load_answer"] = answer
        rec["preload_stderr"] = preload.stderr.strip()[:300]
        # The replay runs under its own run id, so the wait is for the
        # record's `replay.closed` and not for the source's run. The sink is
        # the admin principal's, so it is read as that principal.
        end = time.time() + cfg.get("turn_timeout_s", 3600)
        while time.time() < end:
            if os.path.exists(diag_sink) and '"kind":"replay.closed"' in read_privileged(diag_sink):
                break
            time.sleep(2.0)
        read = subprocess.run(["sudo", "-n", cfg["analysis_bin"], "read", diag_sink], capture_output=True, text=True)
        rec["read"] = (read.stdout.strip().splitlines() or [""])[-1][:600]
        rec["read_stderr"] = read.stderr.strip()[:300]
        # The re-fed record's own measurements, by the replay's run.
        events = [json.loads(l) for l in read_privileged(diag_sink).splitlines() if l.strip()]
        runs = [e["run"] for e in events if e.get("kind") == "replay.closed"]
        rec["replay_run"] = runs[-1] if runs else None
        ex = extract_run([e for e in events if e.get("run") == rec["replay_run"]]) if runs else extract_run(events)
        rec["reading_two"] = reading_two(src, ex)
        with open(os.path.join(out_dir, "refeed.json"), "w") as fh:
            json.dump({**rec, "refed": ex}, fh)
        rec["verdict"] = "RAN"
    except Exception as exc:
        rec["verdict"] = f"error: {type(exc).__name__}: {exc}"
    finally:
        base.admin(cfg, "unload")
        if original is not None:
            with open(decl_path, "w") as fh:
                fh.write(original)
        rec["seconds"] = round(time.time() - started, 1)
        with open(os.path.join(deposit, "refeeds.jsonl"), "a") as fh:
            fh.write(json.dumps({k: v for k, v in rec.items() if k != "refed"}) + "\n")
        log(deposit, f"{name}: {rec.get('verdict')} ({rec['seconds']}s) {json.dumps(rec.get('reading_two'))[:300]}")


def run_read(deposit):
    runs = []
    for d in sorted(os.listdir(os.path.join(deposit, "runs"))) if os.path.isdir(os.path.join(deposit, "runs")) else []:
        p = os.path.join(deposit, "runs", d, "run.json")
        if os.path.exists(p):
            runs.append(json.load(open(p)))
    pairs = []
    for i in range(len(runs)):
        for j in range(i + 1, len(runs)):
            a, b = runs[i], runs[j]
            pairs.append({"a": a["name"], "b": b["name"], "same_seed": a["seed"] == b["seed"],
                          "same_arm": a["arm"] == b["arm"], **reading_one(a, b)})
    report = {"runs": [{k: r.get(k) for k in ("name", "arm", "seed", "tokens", "words", "finish",
                                                "entropy_mean", "emission_sha256", "generation_seed")} for r in runs],
              "reading_one": pairs,
              "same_seed_pairs_identical": [p["identical"] for p in pairs if p["same_seed"] and p["same_arm"]],
              "different_seed_first_divergence": [p["first_token_divergence"] for p in pairs if not p["same_seed"] and p["same_arm"]]}
    rf = os.path.join(deposit, "refeeds.jsonl")
    if os.path.exists(rf):
        report["reading_two"] = [json.loads(l) for l in open(rf)]
    with open(os.path.join(deposit, "report.json"), "w") as fh:
        json.dump(report, fh, indent=1)
    print(json.dumps({k: report[k] for k in ("same_seed_pairs_identical", "different_seed_first_divergence")}))
    for p in pairs:
        print(f"  {p['a']} vs {p['b']}: first token divergence {p['first_token_divergence']}, "
              f"char {p['first_char_divergence']}, aligned agreement after {p['aligned_agreement_after']}, tokens {p['tokens']}")


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="verb", required=True)
    r = sub.add_parser("run"); r.add_argument("--config", required=True)
    f = sub.add_parser("refeed"); f.add_argument("--config", required=True); f.add_argument("--source", required=True); f.add_argument("--as-arm")
    d = sub.add_parser("read"); d.add_argument("--deposit", required=True)
    args = ap.parse_args()
    if args.verb == "run":
        run_free(json.load(open(args.config)))
    elif args.verb == "refeed":
        cfg = json.load(open(args.config))
        run_refeed(cfg, args.source, json.load(open(args.as_arm)) if args.as_arm else None)
    else:
        run_read(args.deposit)


if __name__ == "__main__":
    main()
