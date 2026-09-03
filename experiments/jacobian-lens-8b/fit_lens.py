#!/usr/bin/env python3
"""Fit the Jacobian lens for Qwen3-8B, per issue #386.

The fit is offline and never touches the agent: HF bf16 weights, a
pretraining-like corpus slice, the upstream reference implementation. The
manifest beside the lens carries everything a later reader needs to know
what this artifact is - the identity discipline the capture-artifact paper
will formalize.

Run: /fastpool/venvs/jlens/bin/python fit_lens.py [--prompts N] [--device cuda:1]
     [--checkpoint-every N] [--tag NAME]
"""
import argparse, glob, hashlib, json, os, re, subprocess, sys, time

MODEL_DIR = "/bulk-store/models/Qwen--Qwen3-8B"
# The revision the GGUF rung was converted from, so the two artifacts are
# one model in two representations rather than two snapshots.
MODEL_REVISION = "b968826d9c46"
CORPUS = "/bulk-store/training-datasets/wikitext/wikitext-103-v1/train"
JLENS_REPO = "/dbpool/experiments/jacobian-lens"
OUT = "/bulk-store/weaver-testing/jacobian-lens-8b"


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
    # **The checkpoint cadence is named here rather than defaulted.**
    # `jlens.fit` writes every prompt unless told otherwise, and a
    # checkpoint is `source_layers * d_model**2 * 4` bytes - 74 MiB at the
    # 0.5b and 2.19 GiB here, so the default would write 0.4 TiB over a
    # 200-prompt fit. The cadence bounds what a crash costs instead.
    ap.add_argument("--checkpoint-every", type=int, default=25)
    ap.add_argument("--device", default="cuda:1")
    ap.add_argument("--tag", default="",
                    help="suffix for the lens, checkpoint, and manifest names, "
                         "so fits at different corpus sizes stand side by side")
    args = ap.parse_args()

    import torch, transformers, jlens

    # The implementation's identity is resolved before any fitting work, and
    # the imported package must come from the revision the manifest will
    # record: an empty or mismatched revision reaching a finished fit would
    # be an artifact whose identity lies.
    rev = subprocess.run(
        ["git", "-C", JLENS_REPO, "rev-parse", "HEAD"],
        capture_output=True, text=True,
    )
    jlens_rev = rev.stdout.strip()
    if rev.returncode != 0 or not jlens_rev:
        raise SystemExit(f"the jlens revision does not resolve: {rev.stderr.strip()}")
    if not jlens.__file__.startswith(JLENS_REPO):
        raise SystemExit(
            f"the imported jlens is {jlens.__file__}, not the repo the "
            f"manifest records ({JLENS_REPO})"
        )

    started = time.time()
    prompts = corpus_prompts(args.prompts)
    if len(prompts) != args.prompts:
        raise SystemExit(
            f"the corpus yielded {len(prompts)} of {args.prompts} prompts: "
            "a partial selection would fit a lens the manifest misdescribes"
        )
    prompt_hash = hashlib.sha256("\n\x00".join(prompts).encode()).hexdigest()
    print(f"corpus: {len(prompts)} prompts, sha256 {prompt_hash[:16]}", flush=True)

    hf = transformers.AutoModelForCausalLM.from_pretrained(
        MODEL_DIR, torch_dtype=torch.bfloat16
    ).to(args.device)
    tok = transformers.AutoTokenizer.from_pretrained(MODEL_DIR)
    model = jlens.from_hf(hf, tok)

    os.makedirs(OUT, exist_ok=True)
    if args.tag and not re.fullmatch(r"[A-Za-z0-9._-]+", args.tag):
        raise SystemExit(f"the tag is not filename-safe: {args.tag!r}")
    tag = f"-{args.tag}" if args.tag else ""
    lens = jlens.fit(
        model,
        prompts=prompts,
        checkpoint_path=f"{OUT}/fit-checkpoint{tag}.pt",
        checkpoint_every=args.checkpoint_every,
    )
    lens.save(f"{OUT}/jacobian_lens_qwen3-8b-bf16{tag}.pt")
    # **The artifact the crate reads is safetensors**, per
    # `weaver-analysis-Spec` section 3: one tensor per source layer named by
    # its index, f32. The torch file stays beside it because the reference
    # implementation's own loader wants it, and the manifest names the
    # safetensors as the lens - the artifact of record is the one both
    # sides of the boundary can read.
    from safetensors.torch import save_file
    lens_file = f"jacobian_lens_qwen3-8b-bf16{tag}.safetensors"
    save_file(
        {str(layer): J.contiguous().float().cpu()
         for layer, J in lens.jacobians.items()},
        f"{OUT}/{lens_file}",
    )

    manifest = {
        "lens": lens_file,
        "fitted_for": {
            "model": MODEL_DIR,
            "model_revision": MODEL_REVISION,
            # **Sharded, so the identity is every shard.** The 0.5b had one
            # file and hashed it by name. Naming that file here would raise
            # rather than mislead, but a manifest that recorded only the
            # first shard would be the worse failure, so all five are named.
            "model_safetensors_sha256": {
                os.path.basename(p): sha256(p)
                for p in sorted(glob.glob(f"{MODEL_DIR}/*.safetensors"))
            },
            "dtype": "bfloat16",
        },
        "corpus": {
            "source": CORPUS,
            "selection": f"first {len(prompts)} articles >=400 chars, non-heading, dataset order",
            "prompts_sha256": prompt_hash,
        },
        "checkpoint_every": args.checkpoint_every,
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
    with open(f"{OUT}/lens-manifest{tag}.json", "w") as f:
        json.dump(manifest, f, indent=1)
    print(json.dumps({"fitted": True, "seconds": manifest["fit_seconds"],
                      "layers": len(lens.source_layers)}), flush=True)


if __name__ == "__main__":
    main()
