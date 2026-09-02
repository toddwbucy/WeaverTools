#!/usr/bin/env python3
"""Regression cases for the reader's manifest-schema refusals: malformed
identity answers a refusal, never an exception. Torch-free by design - the
functions under test sit above the model imports.

Run: python3 test_read_columns.py
"""
import importlib.util

spec = importlib.util.spec_from_file_location(
    "read_columns", __file__.replace("test_read_columns.py", "read_columns.py")
)
rc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rc)

GOOD = {"lens_shape": {"d_model": 896, "source_layers": [0, 1, 2]}}

cases = [
    (GOOD, None),
    ({}, "no lens_shape"),
    ({"lens_shape": []}, "no lens_shape"),
    ({"lens_shape": {"source_layers": [0]}}, "d_model"),
    ({"lens_shape": {"d_model": "896", "source_layers": [0]}}, "d_model"),
    ({"lens_shape": {"d_model": 896}}, "source_layers is absent"),
    ({"lens_shape": {"d_model": 896, "source_layers": []}}, "source_layers is absent"),
    ({"lens_shape": {"d_model": 896, "source_layers": [1, "2"]}}, "non-integer"),
    ({"lens_shape": {"d_model": 896, "source_layers": [2, 1]}}, "not a sorted set"),
    ({"lens_shape": {"d_model": 896, "source_layers": [1, 1, 2]}}, "not a sorted set"),
]
for manifest, want in cases:
    got = rc.manifest_shape_refusal(manifest)
    if want is None:
        assert got is None, f"the good manifest refused: {got}"
    else:
        assert got and want in got["refusal"], f"{manifest} -> {got}, wanted {want!r}"

assert rc.manifest_path_for("/x/jacobian_lens_m-bf16.pt").endswith("lens-manifest.json")
assert rc.manifest_path_for("/x/jacobian_lens_m-bf16-1000p.pt").endswith(
    "lens-manifest-1000p.json"
)
print("ok: 12 cases")
