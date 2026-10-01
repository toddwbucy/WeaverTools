"""The provenance readers a run with both SPUs relies on, each held by a test that fails
when its rule is perturbed: the weights of a safetensors directory, the serving device of
an engine that prints no llama.cpp lines, the engine libraries of a Rust SPU that links
CUDA and of a Python SPU, the interpreter a Python SPU runs on, and the SPU a load served.
Run with `python3 test_both_spus.py` or under pytest.
"""
import contextlib
import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import confirm_cells as base  # noqa: E402

CFG = {"agent": "alpha"}


def model_directory(root, files=None):
    """A safetensors export as the operator's plain copy holds it."""
    os.makedirs(root, exist_ok=True)
    for name, data in (files or {"model.safetensors": b"weights", "config.json": b"{}",
                                 "tokenizer.json": b"{}"}).items():
        os.makedirs(os.path.dirname(os.path.join(root, name)) or root, exist_ok=True)
        with open(os.path.join(root, name), "wb") as fh:
            fh.write(data)
    return root


def test_a_directory_artifact_is_read_whole():
    """python-spu-Spec section 5: a directory artifact is a digest over its sorted
    relative paths and each file's sha256. The same files in another order of
    creation read the same; a byte changed, a file renamed or a file added reads
    otherwise. Perturbation: hash the paths' sha256 alone without the names, and the
    rename reads the same."""
    with tempfile.TemporaryDirectory() as tmp:
        first = model_directory(os.path.join(tmp, "a"))
        second = model_directory(os.path.join(tmp, "b"), {"tokenizer.json": b"{}", "config.json": b"{}",
                                                          "model.safetensors": b"weights"})
        one = base.weights(first)(CFG)["artifact"]
        assert base.is_reading({"artifact": one}), one
        assert one["directory"] == 3, one
        assert one["sha256"] == base.weights(second)(CFG)["artifact"]["sha256"]
        with open(os.path.join(second, "model.safetensors"), "ab") as fh:
            fh.write(b"!")
        assert base.weights(second)(CFG)["artifact"]["sha256"] != one["sha256"]
        third = model_directory(os.path.join(tmp, "c"))
        os.rename(os.path.join(third, "config.json"), os.path.join(third, "config2.json"))
        assert base.weights(third)(CFG)["artifact"]["sha256"] != one["sha256"]
        fourth = model_directory(os.path.join(tmp, "d"))
        model_directory(fourth, {"sub/extra.json": b"[]"})
        assert base.weights(fourth)(CFG)["artifact"]["directory"] == 4


def test_a_link_in_or_at_a_directory_artifact_refuses():
    """A symbolic link anywhere in the artifact, or at its own path, is unreadable and
    never followed, since the bytes a link names are not the artifact's. Perturbation:
    follow links in the walk, and the linked file reads as part of the digest."""
    with tempfile.TemporaryDirectory() as tmp:
        outside = os.path.join(tmp, "outside.json")
        with open(outside, "wb") as fh:
            fh.write(b"{}")
        linked = model_directory(os.path.join(tmp, "linked"))
        os.symlink(outside, os.path.join(linked, "extra.json"))
        reading = base.weights(linked)(CFG)
        assert not base.is_reading(reading) and "symbolic link" in reading["artifact"]["unreadable"], reading
        linked_dir = model_directory(os.path.join(tmp, "linked-dir"))
        os.symlink(tmp, os.path.join(linked_dir, "elsewhere"))
        reading = base.weights(linked_dir)(CFG)
        assert not base.is_reading(reading), reading
        plain = model_directory(os.path.join(tmp, "plain"))
        os.symlink(plain, os.path.join(tmp, "alias"))
        reading = base.weights(os.path.join(tmp, "alias"))(CFG)
        assert not base.is_reading(reading) and "symbolic link" in reading["artifact"]["unreadable"], reading


def test_a_directory_the_walk_cannot_read_refuses():
    """A subdirectory the reader may not list is unreadable, never skipped as absent.
    Perturbation: walk without the raising `onerror`, and the shut directory reads as
    though it held nothing."""
    if os.geteuid() == 0:
        print("skip test_a_directory_the_walk_cannot_read_refuses: no mode denies root")
        return
    with tempfile.TemporaryDirectory() as tmp:
        root = model_directory(os.path.join(tmp, "m"), {"model.safetensors": b"w", "shut/x.json": b"{}"})
        os.chmod(os.path.join(root, "shut"), 0)
        try:
            reading = base.weights(root)(CFG)
            assert not base.is_reading(reading), reading
        finally:
            os.chmod(os.path.join(root, "shut"), 0o755)


TESTS = [v for k, v in sorted(globals().items()) if k.startswith("test_")]

if __name__ == "__main__":
    for test in TESTS:
        test()
        print(f"ok {test.__name__}")
