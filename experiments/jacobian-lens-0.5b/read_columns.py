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
DEFAULT_LENS = "/bulk-store/weaver-testing/jacobian-lens-0.5b/jacobian_lens_qwen2.5-0.5b-instruct-bf16.pt"


def _is_int(value):
    """A genuine integer. **Booleans are excluded on purpose**: `bool` is a
    subclass of `int`, so a manifest carrying `true` where a width belongs
    would pass an `isinstance` check and then compare equal to 1 in every
    later arithmetic - an identity that reads as a number."""
    return isinstance(value, int) and not isinstance(value, bool)


def manifest_shape_refusal(manifest):
    """The manifest's whole schema judged before any member is read: a
    manifest that is not an object, a missing member, a boolean wearing a
    number's place, or a layer set that will not sort each answer a refusal
    dict, never an exception - malformed identity is a refusal like wrong
    identity."""
    if not isinstance(manifest, dict):
        return {"refusal": "the manifest is not an object"}
    if not isinstance(manifest.get("lens"), str):
        return {"refusal": "the manifest names no lens"}
    fitted = manifest.get("fitted_for")
    if not isinstance(fitted, dict):
        return {"refusal": "the manifest holds no fitted_for"}
    if not isinstance(fitted.get("model"), str):
        return {"refusal": "the manifest's fitted_for names no model"}
    digest = fitted.get("model_safetensors_sha256")
    if not isinstance(digest, str):
        return {"refusal": "the manifest's fitted_for holds no weights hash"}
    # **A digest that cannot be one refuses here, not after the read**: the
    # comparison below hashes the whole weights file, so a malformed value
    # would cost that read to reach a mismatch it could never avoid, and
    # would then refuse naming the weights rather than the manifest. Exactly
    # sixty-four lowercase hex, which is what `hexdigest` produces and the
    # only spelling the comparison can match.
    if len(digest) != 64 or any(c not in "0123456789abcdef" for c in digest):
        return {"refusal": "the manifest's weights hash is not a sha256 digest",
                "model_safetensors_sha256": digest}
    shape = manifest.get("lens_shape")
    if not isinstance(shape, dict):
        return {"refusal": "the manifest holds no lens_shape"}
    if not _is_int(shape.get("d_model")):
        return {"refusal": "the manifest's d_model is absent or not an integer"}
    layers = shape.get("source_layers")
    if not isinstance(layers, list) or not layers:
        return {"refusal": "the manifest's source_layers is absent or empty"}
    if not all(_is_int(l) for l in layers):
        return {"refusal": "the manifest's source_layers holds a non-integer"}
    if layers != sorted(set(layers)):
        return {"refusal": "the manifest's layer set is not a sorted set",
                "source_layers": layers}
    return None


def load_model_and_lens(lens_path, device="cuda"):
    """The heavy loads, behind one name so a caller - or a test - can see
    exactly where the torch stack and the artifact's tensors enter. Nothing
    above this line imports either."""
    import torch, transformers, jlens

    hf = transformers.AutoModelForCausalLM.from_pretrained(
        MODEL_DIR, dtype=torch.bfloat16
    ).to(device)
    tok = transformers.AutoTokenizer.from_pretrained(MODEL_DIR)
    return jlens.from_hf(hf, tok), tok, jlens.JacobianLens.load(lens_path)


def manifest_path_for(lens_path):
    """The manifest that identifies this lens, derived from its name: the
    fit writes `jacobian_lens_...{tag}.pt` beside `lens-manifest{tag}.json`,
    so a tagged lens is checked against its own manifest and never the
    untagged sibling's."""
    import os, re
    base = os.path.basename(lens_path)
    m = re.fullmatch(r"jacobian_lens_.*?bf16(?P<tag>(-[A-Za-z0-9._-]+)?)\.pt", base)
    tag = m.group("tag") if m else ""
    return os.path.join(os.path.dirname(lens_path), f"lens-manifest{tag}.json")


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
    ap.add_argument("--lens", default=DEFAULT_LENS)
    ap.add_argument("--min-top5", type=float, default=0.9,
                    help="control gate: trajectories print only at or above this")
    args = ap.parse_args()
    import hashlib, os

    # **The identity is judged before anything heavy loads**: the schema
    # whole, then the members, so a malformed or mismatched manifest
    # refuses without a torch stack, a model, or an artifact's tensors ever
    # entering the process.
    try:
        manifest = json.load(open(manifest_path_for(args.lens)))
    except (OSError, json.JSONDecodeError) as error:
        print(json.dumps({"refusal": f"the manifest does not read: {error}"}))
        return 1
    refusal = manifest_shape_refusal(manifest)
    if refusal:
        print(json.dumps(refusal))
        return 1
    if manifest["lens"] != os.path.basename(args.lens):
        print(json.dumps({"refusal": "the manifest names a different lens",
                          "manifest": manifest["lens"]}))
        return 1
    if manifest["fitted_for"]["model"] != MODEL_DIR:
        print(json.dumps({"refusal": "the manifest names different weights",
                          "manifest": manifest["fitted_for"]["model"]}))
        return 1
    config = json.load(open(os.path.join(MODEL_DIR, "config.json")))
    if manifest["lens_shape"]["d_model"] != config["hidden_size"]:
        print(json.dumps({"refusal": "the manifest's width is not the model's",
                          "manifest": manifest["lens_shape"]["d_model"],
                          "model": config["hidden_size"]}))
        return 1
    expected_layers = manifest["lens_shape"]["source_layers"]
    h = hashlib.sha256()
    with open(os.path.join(MODEL_DIR, "model.safetensors"), "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    if h.hexdigest() != manifest["fitted_for"]["model_safetensors_sha256"]:
        print(json.dumps({"refusal": "the weights are not the ones the lens was fitted for"}))
        return 1

    # Scoped by turn, not by position alone: positions are unique within
    # one run's session but a record holding several brackets repeats them,
    # so each turn's columns pair against its own measurement and a
    # measurement consumes the positions gathered for exactly its turn.
    columns = {}    # (turn, position) -> [layers][width]
    tokens = {}     # (turn, position) -> drawn token id
    pending = {}    # turn -> positions gathered since the last measurement
    for line in record_lines(args.record):
        e = json.loads(line)
        turn = e.get("turn")
        if e["kind"] == "residual.column":
            p = e["payload"]
            columns[(turn, p["position"])] = p["values"]
            pending.setdefault(turn, []).append(p["position"])
        if e["kind"] == "model.measurement":
            out = e["payload"]["output_tokens"]
            positions = sorted(pending.pop(turn, []))
            # The column at a position feeds the draw whose token fills it,
            # the measurement's output order being the draws' own order -
            # the same pairing the field's realized rank encodes.
            for pos, tok_id in zip(positions, out):
                tokens[(turn, pos)] = tok_id

    if not columns:
        print(json.dumps({"refusal": "the record holds no residual.column"}))
        return 1
    turns = {t for t, _ in columns}
    print(f"record: {len(columns)} columns across {len(turns)} turns", flush=True)

    # **Pre-load inspection of the tensor names themselves is what the
    # safetensors election buys**; the torch serialization in hand cannot
    # offer it, so the loaded lens is held to the manifest exactly, one
    # layer for one, the moment it materializes.
    model, tok, lens = load_model_and_lens(args.lens)
    import torch
    if lens.d_model != manifest["lens_shape"]["d_model"]:
        print(json.dumps({"refusal": "the lens width disagrees with its manifest",
                          "lens": lens.d_model,
                          "manifest": manifest["lens_shape"]["d_model"]}))
        return 1
    if lens.source_layers != expected_layers:
        print(json.dumps({"refusal": "the lens layers do not match the manifest "
                          "one for one",
                          "lens": lens.source_layers,
                          "manifest": expected_layers}))
        return 1

    # **The control**: final-layer residual, no transport, model's own
    # unembedding. Rank of the actually-drawn token per position.
    paired = [k for k in sorted(columns) if k in tokens]
    if not paired:
        print(json.dumps({"refusal": "no column pairs with a drawn token: "
                          "the record holds columns or measurements, not both"}))
        return 1
    ranks = []
    for key in paired:
        h_final = torch.tensor(columns[key][-1])
        logits = model.unembed(h_final.unsqueeze(0))[0]
        rank = int((logits > logits[tokens[key]]).sum())
        ranks.append(rank)
    top1 = sum(1 for r in ranks if r == 0)
    top5 = sum(1 for r in ranks if r < 5)
    top5_rate = top5 / len(ranks)
    print(json.dumps({
        "control": "unembed(h_final) vs the drawn token",
        "positions": len(ranks),
        "top1": top1, "top1_rate": round(top1 / len(ranks), 4),
        "top5": top5, "top5_rate": round(top5_rate, 4),
    }), flush=True)
    # **The control gates the trajectories**: below the bar, the pairing or
    # the numerics are not established and a readout would be a picture of
    # an unknown alignment, so nothing is printed and the exit says why -
    # the no-reading-from-an-uncertified-replay rule, one level down.
    if top5_rate < args.min_top5:
        print(json.dumps({"refusal": "the control is below the bar",
                          "top5_rate": round(top5_rate, 4),
                          "min_top5": args.min_top5}))
        return 1

    layers = [int(l) for l in args.layers.split(",")]
    if args.positions:
        wanted = {int(p) for p in args.positions.split(",")}
        chosen = [k for k in paired if k[1] in wanted]
    else:
        step = max(1, len(paired) // 6)
        chosen = paired[::step][:6]

    for key in chosen:
        turn, pos = key
        drawn = tok.decode([tokens[key]])
        print(f"\n{turn} position {pos}  (drawn: {drawn!r})")
        for layer in layers:
            h = torch.tensor(columns[key][layer]).unsqueeze(0)
            transported = lens.transport(h, layer)
            logits = model.unembed(transported)[0]
            top = logits.topk(args.topk).indices.tolist()
            print(f"  L{layer:>2}: " + "  ".join(repr(tok.decode([t])) for t in top))
        h_final = torch.tensor(columns[key][-1]).unsqueeze(0)
        top = model.unembed(h_final)[0].topk(args.topk).indices.tolist()
        print(f"  L23 (no transport): " + "  ".join(repr(tok.decode([t])) for t in top))
    return 0


if __name__ == "__main__":
    sys.exit(main())
