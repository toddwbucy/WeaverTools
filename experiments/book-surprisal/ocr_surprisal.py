"""Per-token surprisal of a book's OCR pages under Qwen3-8B, first pass
against refined, and whether the pipeline's flagged artifacts sit on the
spikes. Teacher-forced through the safetensors model, each page alone, per
the run of 2026-09-03 recorded in the testing hub.

Run: /fastpool/venvs/jlens/bin/python ocr_surprisal.py [--book <text_output dir>] [--out DIR] [--device cuda:1]
"""
import argparse
_ap = argparse.ArgumentParser()
_ap.add_argument("--book", default="/bulk-store/books/books/text_output/I_robot_automated")
_ap.add_argument("--out", default="/bulk-store/weaver-testing/ocr-surprisal-2026-09-03")
_ap.add_argument("--device", default="cuda:1")
_a = _ap.parse_args()
import json, re, glob, os, csv, random, time, sys, statistics as st
import torch, transformers
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "jacobian-lens-8b"))
import model_identity  # noqa: E402
B=_a.book; M="/bulk-store/models/Qwen--Qwen3-8B"; OUT=_a.out; dev=_a.device; t0=time.time()
os.makedirs(OUT, exist_ok=True)
shards=model_identity.verify(M)
tok=transformers.AutoTokenizer.from_pretrained(M)
hf=transformers.AutoModelForCausalLM.from_pretrained(M, dtype=torch.bfloat16).to(dev).eval()
def score(text):
    enc=tok(text, return_offsets_mapping=True, return_tensors="pt", truncation=True, max_length=2048)
    ids=enc["input_ids"].to(dev); offs=enc["offset_mapping"][0].tolist()
    with torch.no_grad(): lp=torch.log_softmax(hf(ids).logits[0].float(), dim=-1)
    tgt=ids[0,1:]; s=(-lp[:-1].gather(1,tgt[:,None])[:,0]/0.6931471805599453).tolist()  # bits
    return offs[1:], s   # token i (i>=1): its char span and its surprisal
pages={}
for p in sorted(glob.glob(f"{B}/pages/*.txt")):
    n=os.path.basename(p); q=f"{B}/ocr_pages/{n}"
    if not os.path.exists(q): continue
    fp=open(q).read(); rf=open(p).read()
    if len(fp.split())<40 or len(rf.split())<40: continue
    of,sf=score(fp); orr,sr=score(rf)
    pages[n]={"first_pass":{"text":fp,"offsets":of,"surprisal":sf},"refined":{"text":rf,"offsets":orr,"surprisal":sr}}
def summary(key):
    allv=[v for pg in pages.values() for v in pg[key]["surprisal"]]
    return {"tokens":len(allv),"mean":round(st.mean(allv),3),"median":round(st.median(allv),3),
            "p95":round(sorted(allv)[int(0.95*len(allv))],3),"over_6_bits":round(sum(v>6 for v in allv)/len(allv),4),"max":round(max(allv),2)}
res={"pages":len(pages),"first_pass":summary("first_pass"),"refined":summary("refined")}
# per-page: first pass minus refined, tail mass
delta=[sum(v>6 for v in pg["first_pass"]["surprisal"])/len(pg["first_pass"]["surprisal"])-sum(v>6 for v in pg["refined"]["surprisal"])/len(pg["refined"]["surprisal"]) for pg in pages.values()]
res["pages_where_first_pass_has_more_over_6"]=sum(d>0 for d in delta); res["pages_where_refined_has_more"]=sum(d<0 for d in delta)
# token-level: flagged artifacts vs random words, hit = word's max surprisal at or above the page's 95th percentile
def word_hits(word, text, offs, surs, pct):
    hits=[]
    for m in re.finditer(r"(?<!\w)"+re.escape(word)+r"(?!\w)", text):
        idx=[i for i,(a,b) in enumerate(offs) if a<m.end() and b>m.start()]
        if idx: hits.append(max(surs[i] for i in idx)>=pct)
    return hits
art_hits=[]; arts=0
with open(f"{B}/unknown_words.tsv") as f:
    for row in csv.DictReader(f, delimiter="\t"):
        if row["likely_ocr_artifact"]!="Yes": continue
        for pnum in row["pages"].split(","):
            n=f"image{int(pnum):05d}.txt"
            if n not in pages: continue
            pg=pages[n]["first_pass"]; sv=sorted(pg["surprisal"]); pct=sv[int(0.95*len(sv))]
            h=word_hits(row["word"], pg["text"], pg["offsets"], pg["surprisal"], pct); art_hits+=h; arts+=1 if h else 0
# **The controls score the occurrence that was selected**, not the first
# occurrence of its word on the page, so a repeated word cannot be counted
# twice and every span is scored once.
def span_hit(span, offs, surs, pct):
    idx=[i for i,(a,b) in enumerate(offs) if a<span[1] and b>span[0]]
    return (max(surs[i] for i in idx)>=pct) if idx else None
random.seed(7); rnd_hits=[]; rare_hits=[]
book=" ".join(pg["refined"]["text"] for pg in pages.values())
from collections import Counter
freq=Counter(re.findall(r"[A-Za-z]{4,}", book))
for n,pg in pages.items():
    fp=pg["first_pass"]; sv=sorted(fp["surprisal"]); pct=sv[int(0.95*len(sv))]
    spans=[m.span() for m in re.finditer(r"[A-Za-z]{3,}", fp["text"])]
    for sp in random.sample(spans, min(3,len(spans))):
        h=span_hit(sp, fp["offsets"], fp["surprisal"], pct)
        if h is not None: rnd_hits.append(h)
    # the fairer null: real words occurring exactly once in the whole refined
    # book, legitimate but rare, up to three occurrences a page
    rare=[m.span() for m in re.finditer(r"[A-Za-z]{4,}", fp["text"]) if freq.get(m.group(0))==1][:3]
    for sp in rare:
        h=span_hit(sp, fp["offsets"], fp["surprisal"], pct)
        if h is not None: rare_hits.append(h)
# **An empty control refuses rather than reporting a rate of zero**: the
# comparison is the point of the run, and a null written as 0.0 would read
# as a measurement.
for name, hits in (("artifact", art_hits), ("random", rnd_hits), ("rare real word", rare_hits)):
    if not hits:
        raise SystemExit(f"the {name} control scored no occurrence: nothing to compare")
res["artifact_words_found"] = arts
res["artifact_hit_rate_at_page_p95"] = round(sum(art_hits) / len(art_hits), 3)
res["artifact_occurrences"] = len(art_hits)
res["random_word_hit_rate_at_page_p95"] = round(sum(rnd_hits) / len(rnd_hits), 3)
res["random_occurrences"] = len(rnd_hits)
res["rare_real_word_hit_rate_at_page_p95"] = round(sum(rare_hits) / len(rare_hits), 3)
res["rare_real_word_occurrences"] = len(rare_hits)
res["seconds"]=round(time.time()-t0,1); res["model_revision"]=model_identity.MODEL_REVISION; res["model_safetensors_sha256"]=shards
res["context"]="each page scored alone, no book context, truncation 2048"
json.dump(res, open(f"{OUT}/summary.json","w"), indent=1)
json.dump({n:{k:{"surprisal":[round(v,3) for v in pg[k]["surprisal"]],"offsets":pg[k]["offsets"]} for k in pg} for n,pg in pages.items()}, open(f"{OUT}/per-page.json","w"))
print(json.dumps(res))
