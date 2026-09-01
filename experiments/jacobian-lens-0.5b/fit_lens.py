#!/usr/bin/env python3
"""Fit the Jacobian lens for Qwen2.5-0.5B-Instruct, per issue #386.

The fit is offline and never touches the agent: HF bf16 weights, a
pretraining-like corpus slice, the upstream reference implementation. The
manifest beside the lens carries everything a later reader needs to know
what this artifact is - the identity discipline the capture-artifact paper
will formalize.

Run: /fastpool/venvs/jlens/bin/python fit_lens.py [--prompts N] [--device cuda:1]
"""
import argparse, hashlib, json, subprocess, sys, time

MODEL_DIR = "/bulk-store/models/Qwen--Qwen2.5-0.5B-Instruct"
CORPUS = "/bulk-store/training-datasets/wikitext/wikitext-103-v1/train"
JLENS_REPO = "/dbpool/experiments/jacobian-lens"
OUT = "/bulk-store/weaver-testing/jacobian-lens-0.5b"


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def corpus_prompts(n, min_chars=400):
    """N wikitext-103 train articles, in dataset order, long enough to fill
    the fit's 128-token window. Deterministic: the first N that qualify."""
    import datasets
    rows = datasets.load_from_disk(CORPUS)
    out = []
    for row in rows:
        text = row["text"].strip()
        if len(text) >= min_chars and not text.startswith("="):
            out.append(text)
            if len(out) == n:
                break
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--prompts", type=int, default=200)
    ap.add_argument("--device", default="cuda:1")
    args = ap.parse_args()

    import torch, transformers, jlens

    started = time.time()
    prompts = corpus_prompts(args.prompts)
    prompt_hash = hashlib.sha256("\n\x00".join(prompts).encode()).hexdigest()
    print(f"corpus: {len(prompts)} prompts, sha256 {prompt_hash[:16]}", flush=True)

    hf = transformers.AutoModelForCausalLM.from_pretrained(
        MODEL_DIR, torch_dtype=torch.bfloat16
    ).to(args.device)
    tok = transformers.AutoTokenizer.from_pretrained(MODEL_DIR)
    model = jlens.from_hf(hf, tok)

    import os
    os.makedirs(OUT, exist_ok=True)
    lens = jlens.fit(
        model,
        prompts=prompts,
        checkpoint_path=f"{OUT}/fit-checkpoint.pt",
    )
    lens.save(f"{OUT}/jacobian_lens_qwen2.5-0.5b-instruct-bf16.pt")

    jlens_rev = subprocess.run(
        ["git", "-C", JLENS_REPO, "rev-parse", "HEAD"],
        capture_output=True, text=True,
    ).stdout.strip()
    manifest = {
        "lens": "jacobian_lens_qwen2.5-0.5b-instruct-bf16.pt",
        "fitted_for": {
            "model": MODEL_DIR,
            "model_safetensors_sha256": sha256(f"{MODEL_DIR}/model.safetensors"),
            "dtype": "bfloat16",
        },
        "corpus": {
            "source": CORPUS,
            "selection": f"first {len(prompts)} articles >=400 chars, non-heading, dataset order",
            "prompts_sha256": prompt_hash,
        },
        "estimator": {
            "implementation": "anthropics/jacobian-lens",
            "revision": jlens_rev,
            "max_seq_len": 128,
            "skip_first": 16,
        },
        "environment": {
            "torch": torch.__version__,
            "transformers": transformers.__version__,
            "device": args.device,
            "python": sys.version.split()[0],
        },
        "fit_seconds": round(time.time() - started, 1),
        "lens_shape": {
            "d_model": lens.d_model,
            "source_layers": lens.source_layers,
            "n_prompts": lens.n_prompts,
        },
    }
    with open(f"{OUT}/lens-manifest.json", "w") as f:
        json.dump(manifest, f, indent=1)
    print(json.dumps({"fitted": True, "seconds": manifest["fit_seconds"],
                      "layers": len(lens.source_layers)}), flush=True)


if __name__ == "__main__":
    main()
