"""The 8B's identity by content, shared by the scripts that read it.

The revision is validated rather than copied: these are the five shards'
digests at the revision the GGUF rung was converted from, taken from the
calibration fit of 2026-09-03, and `verify` hashes what is on disk and
refuses anything else, so no script can record this revision for weights
that are not it.
"""
import glob, hashlib, os

MODEL_REVISION = "b968826d9c46"
PINNED_SHARDS = {
    "model-00001-of-00005.safetensors": "31d6a825ae35f11fb85b195b4c42c146c051e446433125a215336abdf95cbf5f",
    "model-00002-of-00005.safetensors": "5991236cea6fe21f3d43cab0f0e84448734fbbe0789816202989f2ddc9d18282",
    "model-00003-of-00005.safetensors": "c5185c4794be2d8a9784d5753c9922db38df478ce11f9ed0b415b7304d896836",
    "model-00004-of-00005.safetensors": "b5ee7de71fbf17db3d5704e0c8f2bc7d005ca9e1d7ca2aeb19827b0cfcaa917a",
    "model-00005-of-00005.safetensors": "20c2d6366ab85c90786ccdd829cd2b9e7d30ef3b2ebbb998280e7e4014b542ff",
}


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def verify(model_dir):
    """The shard digests under `model_dir`, refusing an empty, partial, or
    different set. Answers the dict a manifest records."""
    shards = {os.path.basename(p): sha256(p) for p in sorted(glob.glob(f"{model_dir}/*.safetensors"))}
    if not shards:
        raise SystemExit(f"no safetensors shards under {model_dir}: nothing to identify")
    if shards != PINNED_SHARDS:
        differing = sorted(k for k in set(shards) | set(PINNED_SHARDS) if shards.get(k) != PINNED_SHARDS.get(k))
        raise SystemExit(f"the shards under {model_dir} are not revision {MODEL_REVISION}: {differing}")
    return shards
