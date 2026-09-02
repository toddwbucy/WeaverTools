#!/usr/bin/env python3
"""Regression cases for the reader's identity refusals: malformed identity
answers a refusal, never an exception, and nothing heavy loads until the
identity holds.

Torch-free by design - the unit cases sit above the model imports, and the
integration cases assert exactly that by substituting a sentinel loader
that fails the test if it is ever called.

Run: python3 test_read_columns.py
"""
import importlib.util
import io
import json
import os
import sys
import tempfile
from contextlib import redirect_stdout

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location(
    "read_columns", os.path.join(HERE, "read_columns.py")
)
rc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rc)

GOOD = {
    "lens": "jacobian_lens_m-bf16.pt",
    "fitted_for": {"model": "/m", "model_safetensors_sha256": "0" * 64},
    "lens_shape": {"d_model": 896, "source_layers": [0, 1, 2]},
}


def without(member, inner=None):
    m = json.loads(json.dumps(GOOD))
    if inner is None:
        m.pop(member)
    else:
        m[member].pop(inner)
    return m


def with_value(member, inner, value):
    m = json.loads(json.dumps(GOOD))
    m[member][inner] = value
    return m


# --- the schema's own cases, including booleans wearing a number's place
cases = [
    (GOOD, None),
    ("not an object", "not an object"),
    ([], "not an object"),
    (None, "not an object"),
    (without("lens"), "names no lens"),
    ({**GOOD, "lens": 7}, "names no lens"),
    (without("fitted_for"), "no fitted_for"),
    ({**GOOD, "fitted_for": "x"}, "no fitted_for"),
    (without("fitted_for", "model"), "names no model"),
    (without("fitted_for", "model_safetensors_sha256"), "no weights hash"),
    (with_value("fitted_for", "model_safetensors_sha256", 7), "no weights hash"),
    (with_value("fitted_for", "model_safetensors_sha256", ""), "not a sha256"),
    (with_value("fitted_for", "model_safetensors_sha256", "abc"), "not a sha256"),
    (with_value("fitted_for", "model_safetensors_sha256", "f" * 63), "not a sha256"),
    (with_value("fitted_for", "model_safetensors_sha256", "f" * 65), "not a sha256"),
    (with_value("fitted_for", "model_safetensors_sha256", "g" * 64), "not a sha256"),
    # Uppercase is refused rather than folded: `hexdigest` is lowercase, so
    # an uppercase digest could only ever mismatch, and refusing it here
    # names the manifest instead of the weights.
    (with_value("fitted_for", "model_safetensors_sha256", "A" * 64), "not a sha256"),
    (without("lens_shape"), "no lens_shape"),
    ({**GOOD, "lens_shape": []}, "no lens_shape"),
    (without("lens_shape", "d_model"), "d_model"),
    (with_value("lens_shape", "d_model", "896"), "d_model"),
    (with_value("lens_shape", "d_model", True), "d_model"),
    (with_value("lens_shape", "d_model", False), "d_model"),
    (without("lens_shape", "source_layers"), "source_layers is absent"),
    (with_value("lens_shape", "source_layers", []), "source_layers is absent"),
    (with_value("lens_shape", "source_layers", [1, "2"]), "non-integer"),
    (with_value("lens_shape", "source_layers", [0, True, 2]), "non-integer"),
    (with_value("lens_shape", "source_layers", [2, 1]), "not a sorted set"),
    (with_value("lens_shape", "source_layers", [1, 1, 2]), "not a sorted set"),
]
for manifest, want in cases:
    got = rc.manifest_shape_refusal(manifest)
    if want is None:
        assert got is None, f"the good manifest refused: {got}"
    else:
        assert got and want in got["refusal"], f"{manifest} -> {got}, wanted {want!r}"

# A boolean must not reach a later numeric comparison as 1.
assert not rc._is_int(True) and not rc._is_int(False)
assert rc._is_int(0) and rc._is_int(896)

# --- the manifest path derives whole, directory preserved
assert rc.manifest_path_for("/x/y/jacobian_lens_m-bf16.pt") == "/x/y/lens-manifest.json"
assert (
    rc.manifest_path_for("/x/y/jacobian_lens_m-bf16-1000p.pt")
    == "/x/y/lens-manifest-1000p.json"
)

# --- integration: main() refuses malformed identity and loads nothing


def sentinel(*a, **k):
    raise AssertionError("the loader ran against a refused identity")


def run_main(manifest_text, lens_name="jacobian_lens_m-bf16.pt"):
    """main() over a temporary lens directory, with the heavy loader
    replaced by a sentinel: the return is the refusal's exit status and the
    sentinel proves nothing loaded."""
    with tempfile.TemporaryDirectory() as d:
        lens = os.path.join(d, lens_name)
        open(lens, "w").write("")
        open(os.path.join(d, "lens-manifest.json"), "w").write(manifest_text)
        held_loader, held_argv = rc.load_model_and_lens, sys.argv
        rc.load_model_and_lens = sentinel
        sys.argv = ["read_columns.py", "/nonexistent-record.ndjson", "--lens", lens]
        try:
            out = io.StringIO()
            with redirect_stdout(out):
                status = rc.main()
            return status, out.getvalue()
        finally:
            rc.load_model_and_lens, sys.argv = held_loader, held_argv


for manifest_text, want in [
    ("{ this is not json", "does not read"),
    (json.dumps([]), "not an object"),
    (json.dumps(without("lens_shape")), "no lens_shape"),
    (json.dumps(with_value("lens_shape", "d_model", True)), "d_model"),
    (json.dumps({**GOOD, "lens": "another.pt"}), "names a different lens"),
]:
    status, printed = run_main(manifest_text)
    assert status == 1, f"{want}: expected exit 1, got {status}"
    assert want in printed, f"{want}: printed {printed!r}"

print(f"ok: {len(cases) + 5} manifest cases, 2 path cases, 5 integration cases")
