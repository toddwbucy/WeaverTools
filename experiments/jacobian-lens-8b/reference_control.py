"""The reference arm of the 8B control, per issue #386: the safetensors model
itself, teacher-forced on a capture's own token ids, ranked in f32 as the
crate ranks. One model, one revision, one set of drawn tokens, so a GGUF
capture read through the lens's head compares to this and to nothing else.

Run: /fastpool/venvs/jlens/bin/python reference_control.py --capture <ndjson> --out <json> [--device cuda:1]
"""
import argparse, json, sys, time, hashlib
import torch, transformers
ap = argparse.ArgumentParser()
ap.add_argument("--capture", required=True); ap.add_argument("--out", required=True)
ap.add_argument("--model", default="/bulk-store/models/Qwen--Qwen3-8B"); ap.add_argument("--device", default="cuda:1")
args = ap.parse_args()
CAP, M, OUT, DEV = args.capture, args.model, args.out, args.device
inp = out = None
with open(CAP) as f:
    for line in f:
        e = json.loads(line)
        if e["kind"] == "model.measurement":
            inp, out = e["payload"]["input_tokens"], e["payload"]["output_tokens"]; break
assert inp and out, "no measurement in the capture"
t0 = time.time()
tok_ids = torch.tensor([inp + out], device=DEV)
hf = transformers.AutoModelForCausalLM.from_pretrained(M, dtype=torch.bfloat16).to(DEV).eval()
with torch.no_grad():
    logits = hf(tok_ids).logits[0].float()   # [len, vocab], ranked in f32 as the crate ranks
ranks = []
for i, tokn in enumerate(out):
    row = logits[len(inp) + i - 1]
    ranks.append(int((row > row[tokn]).sum().item()))   # tokens outranking the drawn one
top1 = sum(r == 0 for r in ranks); top5 = sum(r < 5 for r in ranks)
res = {"arm": "torch safetensors forward, teacher-forced on the capture's own token ids",
       "model": M, "model_revision": "b968826d9c46", "dtype": "bfloat16, logits ranked in f32",
       "capture": CAP, "capture_sha256_head": hashlib.sha256(open(CAP,"rb").read(1<<20)).hexdigest()[:16],
       "input_tokens": len(inp), "output_tokens": len(out),
       "top1": top1, "top1_rate": round(top1/len(out),4), "top5": top5, "top5_rate": round(top5/len(out),4),
       "ranks": ranks, "seconds": round(time.time()-t0,1),
       "torch": torch.__version__, "transformers": transformers.__version__}
json.dump(res, open(OUT, "w"))
print(json.dumps({k: v for k, v in res.items() if k != "ranks"}))
