#!/usr/bin/env python3
"""One long single-topic session that fills the window, per the operator's
ask of 2026-09-03: large inputs and many turns on one thread, with the
surprisal and readout elections standing, so the per-token series can be
read across depth. The inputs are consecutive refined pages of one OCR'd
book, three pages a turn, the ask identical every turn.

Run: python3 depth_session.py --config <box>.json --book <text_output dir>
     [--first-page N] [--pages-per-turn 3] [--turns 16] --outdir DIR
"""
import argparse, glob, hashlib, json, os, re, sys, time
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "cross-precision-repro"))
import confirm_cells as base  # noqa: E402

ASKS = {
    "summary": ("Here are the next pages of the book we are reading together. In about 200 "
                "words, say what happens in them, then list any passages that look like "
                "scanning errors rather than the author's words."),
    # **A continuation has no format to converge on**, per the run of
    # 2026-09-03 whose summaries settled into a house style by turn eight,
    # and it runs past any cap, so every turn ends at the same token count
    # and completion stops being a variable.
    # **The ask names a length the cap always cuts**, because the
    # declaration's identity asks for brevity and a continuation with no
    # length given came back at two to fifty tokens on 2026-09-03. The
    # family's /no_think switch is not used: under this template it left
    # the prose inside an unclosed think block.
    "continuation": ("Here are the next pages of the book. Continue the story from the last "
                     "sentence in the author's own voice, as prose, for at least 700 words, "
                     "with no heading, no commentary, and no summary."),
}
MAX_TOKENS_PER_TURN = 512  # the default; --max-tokens overrides and the report records it

def sha(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for b in iter(lambda: f.read(1 << 20), b""): h.update(b)
    return h.hexdigest()

def patched(original, cfg):
    text = original
    for pattern, value in ((r"(artifact:\s*).*", cfg["artifact"]),
                           (r"(context-capacity:\s*).*", str(cfg["context_capacity"])),
                           (r"(max-tokens-per-turn:\s*).*", str(MAX_TOKENS_PER_TURN)),
                           (r"(residual-readout-election:\s*).*", "true")):
        text, n = re.subn(pattern, r"\g<1>" + value, text, count=1)
        if n != 1: raise SystemExit(f"declaration line not found: {pattern}")
    # **The surprisal election is inserted, not replaced**: the baseline
    # declaration carries no such line, and a regex over an absent line
    # would have patched nothing and run the session without it.
    if "surprisal-election:" in text: raise SystemExit("the declaration already elects surprisal; refusing to guess")
    text, n = re.subn(r"^(\s*)(residual-readout-election: true)$", r"\1\2\n\1surprisal-election: true", text, count=1, flags=re.M)
    if n != 1: raise SystemExit("could not place the surprisal election")
    return text

def main():
    global MAX_TOKENS_PER_TURN
    ap = argparse.ArgumentParser()
    ap.add_argument("--config", required=True); ap.add_argument("--book", required=True)
    ap.add_argument("--first-page", type=int, default=20); ap.add_argument("--pages-per-turn", type=int, default=3)
    ap.add_argument("--turns", type=int, default=16); ap.add_argument("--outdir", required=True)
    ap.add_argument("--task", choices=sorted(ASKS), default="summary")
    ap.add_argument("--max-tokens", type=int, default=MAX_TOKENS_PER_TURN)
    ap.add_argument("--no-think", action="store_true",
                    help="append the family's soft switch so the budget is the reply's, recorded in the report")
    a = ap.parse_args()
    cfg = json.load(open(a.config)); os.makedirs(a.outdir, exist_ok=True)
    pages = sorted(glob.glob(f"{a.book}/pages/*.txt"))
    chosen = pages[a.first_page - 1 : a.first_page - 1 + a.pages_per_turn * a.turns]
    if len(chosen) < a.pages_per_turn * a.turns: raise SystemExit("the book is shorter than the run")
    turns = [chosen[i:i + a.pages_per_turn] for i in range(0, len(chosen), a.pages_per_turn)]
    decl = cfg["declaration"]; original = open(decl).read(); original_sha = sha(decl)
    backup = decl + ".depth-backup"; open(backup, "w").write(original)
    trace = cfg["trace"]; start_bytes = os.path.getsize(trace) if os.path.exists(trace) else 0
    MAX_TOKENS_PER_TURN = a.max_tokens
    ASK = ASKS[a.task] + (" /no_think" if a.no_think else "")
    report = {"ask": ASK, "task": a.task, "no_think": a.no_think, "book": a.book, "first_page": a.first_page, "pages_per_turn": a.pages_per_turn,
              "declaration_sha256_original": original_sha, "artifact": cfg["artifact"],
              "context_capacity": cfg["context_capacity"], "max_tokens_per_turn": MAX_TOKENS_PER_TURN, "turns": []}
    answered = {}  # turn key -> ordinal, from the gate's own answers
    run_id = None
    failed = None
    try:
        open(decl, "w").write(patched(original, cfg)); report["declaration_sha256_as_run"] = sha(decl)
        # **Unload first, and judge the load by its answer.** A load over a
        # standing unit is refused while the old worker and its socket
        # remain, and a socket that accepts a connection is not evidence
        # that this declaration is the one serving.
        before = base.newest_load(trace)[0]
        base.admin(cfg, "unload")
        load = base.admin(cfg, "load"); print("load:", load, flush=True)
        if load.get("kind") != "state": raise SystemExit(f"the load did not reach a state: {load}")
        if not base.wait_socket(cfg): raise SystemExit("the gate socket never stood")
        # **The loop that composed this load is checked before a turn is
        # served through it**, per issue #426: the config's `loop_sha256`
        # against the load event's composer digest, the shared helper doing
        # the comparison. A refusal serves no turn, lands typed in the
        # report, and exits nonzero, so the deposit is a refusal and not a
        # short session.
        refused = base.assert_loop(cfg, trace, before)
        if refused:
            # The refused run is named in `loop_refused` and nowhere else:
            # `run_id` stays unset so the deposit below writes no session
            # for it, per the README's word that a refusal deposits no run.
            report["loop_refused"] = refused
            failed = {"turn": 0, "why": "loop refused: declared "
                      f"{refused['declared']}, recorded {refused['recorded']}"}
            report["failed"] = failed
            print("LOOP REFUSED", json.dumps(report["loop_refused"]), flush=True)
            turns = []
        for i, group in enumerate(turns, 1):
            body = "\n\n".join(open(p).read() for p in group)
            t0 = time.time(); answer = base.gate_turn(cfg, f"{ASK}\n\n{body}", timeout=900); dt = time.time() - t0
            entry = {"turn": i, "pages": [os.path.basename(p) for p in group], "input_words": len(body.split()),
                     "seconds": round(dt, 1), "answer_kind": answer.get("kind"), "turn_key": answer.get("turn"), "run": answer.get("run")}
            report["turns"].append(entry); print(json.dumps(entry), flush=True)
            # **The run is judged on every answer, failures included**: an
            # answer naming no run cannot be deposited against one, and an
            # answer naming a different run than the first is another
            # residency's and ends this one.
            this_run = answer.get("run")
            if not this_run:
                failed = {"turn": i, "answer": answer, "why": "no run named"}
                report["failed"] = failed
                print("FAILED turn", i, "no run named", flush=True)
                break
            if run_id is None:
                run_id = this_run
                report["run"] = run_id
            elif this_run != run_id:
                failed = {"turn": i, "answer": answer, "why": f"answered from run {this_run}, not {run_id}"}
                report["failed"] = failed
                print("FAILED turn", i, failed["why"], flush=True)
                break
            # **An unanswered turn ends the run**, recorded as such: a report
            # with fewer valid turns that read as complete would be the
            # defect the deposit exists to prevent.
            if answer.get("kind") != "answered":
                failed = {"turn": i, "answer": answer, "why": "not answered"}
                report["failed"] = failed
                print("FAILED turn", i, str(answer)[:300], flush=True)
                break
            answered[answer.get("turn")] = i
    finally:
        print("unload:", base.admin(cfg, "unload"), flush=True)
        open(decl, "w").write(original)
        report["declaration_restored"] = sha(decl) == original_sha
        if report["declaration_restored"]: os.remove(backup)
    # **The deposit is this run's and these turns'.** Run-level events of the
    # run the gate named are kept, turn-level events only for the turns the
    # gate answered here, so another dialer's turn in the same residency
    # cannot ride into the record or into `resident`.
    if "loop_refused" in report:
        # **A refused load deposits the refusal and no session.** The report
        # carries both digests and the refused run's reference; the trace
        # slice, the load event, and `session.ndjson` are not written, so
        # nothing under this deposit can be read as a run of the campaign.
        json.dump(report, open(f"{a.outdir}/report.json", "w"), indent=1)
        print(json.dumps({"loop_refused": report["loop_refused"], "restored": report["declaration_restored"]}), flush=True)
        sys.exit(1)
    with open(trace, "rb") as f:
        f.seek(start_bytes); data = f.read()
    kept, dropped, kinds = [], 0, {}
    for line in data.decode(errors="replace").splitlines():
        try: e = json.loads(line)
        except Exception: continue
        if run_id and e.get("run") != run_id: dropped += 1; continue
        if e.get("turn") and e.get("turn") not in answered: dropped += 1; continue
        kept.append(line); kinds[e.get("kind")] = kinds.get(e.get("kind"), 0) + 1
        if e.get("kind") == "model.output": report.setdefault("resident", []).append(e["payload"].get("resident"))
        if e.get("kind") == "load": report["load_event"] = e["payload"]
    open(f"{a.outdir}/session.ndjson", "w").write("\n".join(kept) + ("\n" if kept else ""))
    report["run"] = run_id; report["record_kinds"] = kinds; report["events_dropped_as_not_this_runs"] = dropped
    json.dump(report, open(f"{a.outdir}/report.json", "w"), indent=1)
    print(json.dumps({"turns": len(answered), "resident": report.get("resident"), "restored": report["declaration_restored"], "failed": failed is not None}), flush=True)
    if failed: sys.exit(1)

if __name__ == "__main__":
    main()
