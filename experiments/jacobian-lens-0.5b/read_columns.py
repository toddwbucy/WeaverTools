#!/usr/bin/env python3
"""Read a diagnostic record's residual columns through a fitted Jacobian lens.

The lens's first light against real replay capture, per issue #386: the
`residual.column` events are the tap's own copy per layer per sampled
position, crossed under the diagnostic binding and authored by the harness.
This reads them back, transports each layer into the final basis, unembeds
with the model's own head, and prints the layer trajectory at chosen
positions.

**The control comes first**: at the final layer the transport is not
applied, and unembed(h_final) must rank the actually-drawn token at or near
top-1 at every position - that one check validates the position pairing,
the layer convention (l_out-23 is the pre-final-norm residual), and the
engine-vs-HF numerics in a single number before any lens claim is made.

Run: /fastpool/venvs/jlens/bin/python read_columns.py <record.ndjson[.zst]>
     [--positions p1,p2,...] [--layers 2,6,10,14,18,22] [--topk 5]
"""
import argparse, io, json, subprocess, sys

MODEL_DIR = "/bulk-store/models/Qwen--Qwen2.5-0.5B-Instruct"
LENS = "/bulk-store/weaver-testing/jacobian-lens-0.5b/jacobian_lens_qwen2.5-0.5b-instruct-bf16.pt"


def record_lines(path):
    if path.endswith(".zst"):
        out = subprocess.run(["zstd", "-dc", path], capture_output=True, check=True)
        return io.StringIO(out.stdout.decode()).readlines()
    return open(path).readlines()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("record")
    ap.add_argument("--positions", default=None,
                    help="comma-separated absolute positions; default: a spread")
    ap.add_argument("--layers", default="2,6,10,14,18,22")
    ap.add_argument("--topk", type=int, default=5)
    args = ap.parse_args()

    import torch, transformers, jlens

    columns = {}   # position -> [layers][width]
    tokens = {}    # position -> drawn token id (from model.field realized? no:
                   # from output_tokens by order)
    per_turn = {}
    for line in record_lines(args.record):
        e = json.loads(line)
        if e["kind"] == "residual.column":
            p = e["payload"]
            columns[p["position"]] = p["values"]
            per_turn.setdefault(e.get("turn"), []).append(p["position"])
        if e["kind"] == "model.measurement":
            out = e["payload"]["output_tokens"]
            positions = sorted(per_turn.get(e.get("turn"), []))
            # The column at a position feeds the draw whose token fills it,
            # the field's own pairing.
            for pos, tok_id in zip(positions, out):
                tokens[pos] = tok_id

    if not columns:
        print(json.dumps({"refusal": "the record holds no residual.column"}))
        return 1
    print(f"record: {len(columns)} columns across {len(per_turn)} turns", flush=True)

    hf = transformers.AutoModelForCausalLM.from_pretrained(
        MODEL_DIR, dtype=torch.bfloat16
    ).cuda()
    tok = transformers.AutoTokenizer.from_pretrained(MODEL_DIR)
    model = jlens.from_hf(hf, tok)
    lens = jlens.JacobianLens.load(LENS)

    # **The control**: final-layer residual, no transport, model's own
    # unembedding. Rank of the actually-drawn token per position.
    ranks = []
    paired = [p for p in sorted(columns) if p in tokens]
    for pos in paired:
        h_final = torch.tensor(columns[pos][-1])
        logits = model.unembed(h_final.unsqueeze(0))[0]
        rank = int((logits > logits[tokens[pos]]).sum())
        ranks.append(rank)
    top1 = sum(1 for r in ranks if r == 0)
    top5 = sum(1 for r in ranks if r < 5)
    print(json.dumps({
        "control": "unembed(h_final) vs the drawn token",
        "positions": len(ranks),
        "top1": top1, "top1_rate": round(top1 / len(ranks), 4),
        "top5": top5, "top5_rate": round(top5 / len(ranks), 4),
    }), flush=True)

    layers = [int(l) for l in args.layers.split(",")]
    if args.positions:
        chosen = [int(p) for p in args.positions.split(",")]
    else:
        step = max(1, len(paired) // 6)
        chosen = paired[::step][:6]

    for pos in chosen:
        drawn = tok.decode([tokens[pos]]) if pos in tokens else "?"
        print(f"\nposition {pos}  (drawn: {drawn!r})")
        for layer in layers:
            h = torch.tensor(columns[pos][layer]).unsqueeze(0)
            transported = lens.transport(h, layer)
            logits = model.unembed(transported)[0]
            top = logits.topk(args.topk).indices.tolist()
            print(f"  L{layer:>2}: " + "  ".join(repr(tok.decode([t])) for t in top))
        h_final = torch.tensor(columns[pos][-1]).unsqueeze(0)
        top = model.unembed(h_final)[0].topk(args.topk).indices.tolist()
        print(f"  L23 (no transport): " + "  ".join(repr(tok.decode([t])) for t in top))
    return 0


if __name__ == "__main__":
    sys.exit(main())
