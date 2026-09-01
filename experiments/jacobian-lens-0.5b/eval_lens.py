#!/usr/bin/env python3
"""Score the fitted lens on the upstream evaluation sets.

The six sets ship with `anthropics/jacobian-lens` as prompts plus target
intermediates; the metric is theirs: pass@k, the mean over items of the
fraction of intermediates whose min-over-layers lens rank is at or under k,
read at the set's one readout position. This tests the fit itself - whether
200 wikitext prompts bought a usable transport - rather than the plumbing
the first-light read tested.

Readout positions per the sets' README: association and typo read at the
final prompt token; multihop, multilingual, and order-ops read at the token
immediately preceding `target`; poetry reads at the last newline token.
Order-ops expands each intermediate to a synonym set and takes the min.

Run: /fastpool/venvs/jlens/bin/python eval_lens.py [--sets a,b,...] [--device cuda:1]
"""
import argparse, json, os, sys

MODEL_DIR = "/bulk-store/models/Qwen--Qwen2.5-0.5B-Instruct"
DEFAULT_LENS = "/bulk-store/weaver-testing/jacobian-lens-0.5b/jacobian_lens_qwen2.5-0.5b-instruct-bf16.pt"
EVAL_DIR = "/dbpool/experiments/jacobian-lens/data/evaluations"
SETS = ["association", "multihop", "multilingual", "order-ops", "poetry", "typo"]


def single_token_ids(tok, word):
    """Every single-token spelling of the word this tokenizer has: bare,
    space-led, capitalised, both. The sets' targets are words, the lens
    ranks tokens, and a word the tokenizer splits is scored by whichever
    single-token form it has, or skipped as unscorable."""
    ids = set()
    for form in (word, " " + word, word.capitalize(), " " + word.capitalize()):
        pieces = tok.encode(form, add_special_tokens=False)
        if len(pieces) == 1:
            ids.add(pieces[0])
    return ids


def readout_position(name, tok, prompt, target):
    if name in ("association", "typo"):
        return -1
    if name == "poetry":
        ids = tok.encode(prompt, add_special_tokens=False)
        newline = tok.encode("\n", add_special_tokens=False)
        want = newline[0] if len(newline) == 1 else None
        last = None
        for i, t in enumerate(ids):
            if want is not None and t == want:
                last = i
            elif "\n" in tok.decode([t]):
                last = i
        return last if last is not None else -1
    # multihop, multilingual, order-ops: the token immediately preceding
    # `target` - the prompt as given ends where the target would begin.
    return -1


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--sets", default=",".join(SETS))
    ap.add_argument("--device", default="cuda:1")
    ap.add_argument("--ks", default="1,5,10")
    ap.add_argument("--lens", default=DEFAULT_LENS)
    ap.add_argument("--out-tag", default="")
    args = ap.parse_args()

    import torch, transformers, jlens

    hf = transformers.AutoModelForCausalLM.from_pretrained(
        MODEL_DIR, dtype=torch.bfloat16
    ).to(args.device)
    tok = transformers.AutoTokenizer.from_pretrained(MODEL_DIR)
    model = jlens.from_hf(hf, tok)
    lens = jlens.JacobianLens.load(args.lens)
    ks = [int(k) for k in args.ks.split(",")]

    results = {}
    for name in args.sets.split(","):
        path = os.path.join(EVAL_DIR, f"lens-eval-{name}.json")
        items = json.load(open(path))["items"]
        per_item = []
        skipped = 0
        for item in items:
            prompt = item["prompt"]
            if not isinstance(prompt, str):
                skipped += 1  # multi-turn prompts are out of this pass's scope
                continue
            position = readout_position(name, tok, prompt, item.get("target"))
            lens_logits, _, _ = lens.apply(model, prompt, positions=[position])
            fractions = {k: 0 for k in ks}
            scorable = 0
            for inter in item["intermediates"]:
                synonyms = inter if isinstance(inter, list) else [inter]
                ids = set()
                for word in synonyms:
                    ids |= single_token_ids(tok, word)
                if not ids:
                    continue
                scorable += 1
                best = None
                for layer, logits in lens_logits.items():
                    row = logits[0]
                    for tid in ids:
                        rank = int((row > row[tid]).sum())
                        if best is None or rank < best:
                            best = rank
                for k in ks:
                    if best is not None and best < k:
                        fractions[k] += 1
            if scorable:
                per_item.append({k: fractions[k] / scorable for k in ks})
            else:
                skipped += 1
        if per_item:
            results[name] = {
                "items": len(per_item),
                "skipped": skipped,
                **{f"pass@{k}": round(sum(p[k] for p in per_item) / len(per_item), 4)
                   for k in ks},
            }
        else:
            results[name] = {"items": 0, "skipped": skipped}
        print(json.dumps({name: results[name]}), flush=True)

    import re
    if args.out_tag and not re.fullmatch(r"[A-Za-z0-9._-]+", args.out_tag):
        print(json.dumps({"refusal": f"the tag is not filename-safe: {args.out_tag!r}"}))
        return
    tag = f"-{args.out_tag}" if args.out_tag else ""
    out = f"/bulk-store/weaver-testing/jacobian-lens-0.5b/lens-eval-results{tag}.json"
    with open(out, "w") as f:
        json.dump(results, f, indent=1)
    print(json.dumps({"written": out}))


if __name__ == "__main__":
    main()
