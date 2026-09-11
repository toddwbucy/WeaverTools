#!/usr/bin/env python3
"""A fixture corpus with one of each defect the census claims to count.

**The gate was wrong four times before it had a test**, and each time it
printed a confident number: seventy sound citations called dangling, four
documents called self-inconsistent, fifty four `compile-pin` and
`compile-fail` assertions called untagged, and fifteen perturbations called
uncited because the walk that collects citations declined to read a crate's
tests. A fixture holding one of each would have caught all four, which is
the argument this file is.

**It exercises `main` and not only `take`.** A test that re-implements the
comparison it is checking passes whatever `main` does, which is the watch
that cannot fail wearing the costume of a test.

    python3 process/gates/test_census.py
"""

import importlib.util
import io
import json
import os
import shutil
import tempfile
import unittest
from contextlib import redirect_stdout

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("census", os.path.join(HERE, "census.py"))
census = importlib.util.module_from_spec(spec)
spec.loader.exec_module(census)

CORPUS = """
```graph
node: fix-cited-pin
kind: assertion
tag: compile-pin

node: fix-uncited-perturbation
kind: assertion
tag: perturbation

node: fix-untagged
kind: assertion

node: fix-odd-tag
kind: assertion
tag: socket

node: fix-Malformed-Id
kind: assertion
tag: review

node: fix-cited-pin
kind: assertion
tag: compile-pin
```

| claim | instrument |
|---|---|
| one row, against the four nodes this corpus declares | review |
"""

SOURCE = """//! conforms: fix-cited-pin
//!
//! A file that carries its header.
"""

BARE = """//! A file with no header citation.

/// conforms: fix-nonexistent
fn f() {}
"""


class Census(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.mkdtemp()
        os.makedirs(os.path.join(self.dir, "docs"))
        os.makedirs(os.path.join(self.dir, "crates/demo/src"))
        os.makedirs(os.path.join(self.dir, "crates/demo/tests"))
        write(self.dir, "Cargo.toml", 'members = [\n  "crates/demo",\n]\n')
        write(self.dir, "docs/demo-Spec.md", CORPUS)
        write(self.dir, "crates/demo/src/lib.rs", SOURCE)
        write(self.dir, "crates/demo/src/bare.rs", BARE)
        # A test directory cites and does not owe a header.
        write(self.dir, "crates/demo/tests/it.rs", "// conforms: fix-untagged\n")
        self.was = census.ROOT
        census.ROOT = self.dir
        self.baseline = census.BASELINE
        census.BASELINE = os.path.join(self.dir, "baseline.json")
        self.addCleanup(shutil.rmtree, self.dir, True)

    def tearDown(self):
        census.ROOT = self.was
        census.BASELINE = self.baseline

    def test_each_defect_is_counted_once_and_named(self):
        reading = census.take()

        # **A hyphen is not a word character.** The bug that published fifty
        # four tagged assertions as untagged lives or dies here.
        self.assertEqual(reading["untagged_assertions"], ["fix-untagged"])
        self.assertEqual(reading["unknown_tags"], ["fix-odd-tag (socket)"])

        self.assertEqual(reading["dangling_citations"], ["fix-nonexistent"])
        self.assertEqual(reading["uncited_perturbations"], ["fix-uncited-perturbation"])

        # **A block declares several nodes**, so a reader taking the first
        # per block sees one of six and calls the rest uncited.
        self.assertEqual(len(reading["duplicate_node_ids"]), 1)
        self.assertIn("fix-cited-pin", reading["duplicate_node_ids"][0])

        # A malformed id is named rather than dropped into silence, where it
        # would make every citation to it read as dangling.
        self.assertEqual(len(reading["malformed_node_ids"]), 1)
        self.assertIn("fix-Malformed-Id", reading["malformed_node_ids"][0])

        # The table is found by its own header, and the documents that carry
        # none are the metric, so one losing its table is a new entry rather
        # than a mismatch count that stays quietly at zero.
        self.assertEqual(reading["documents_without_an_enforcement_table"], [])
        self.assertEqual(len(reading["enforcement_table_mismatch"]), 1)
        self.assertIn("(4 nodes, 1 rows)", reading["enforcement_table_mismatch"][0])

        # **The obligation follows the unit, not a directory**, so the test
        # file owes one too and the format's rule is what this pins.
        self.assertEqual(
            reading["sources_without_a_header"],
            ["crates/demo/src/bare.rs", "crates/demo/tests/it.rs"],
        )

    def test_main_fails_on_a_swapped_defect_whose_count_stands_still(self):
        """**Through `main`**, so the exit code and the comparison are what is
        watched rather than a re-implementation of them beside the gate."""
        self.assertEqual(run("--update"), 0)
        before = json.load(open(census.BASELINE))["dangling_citations"]
        self.assertEqual(run(), 0, "the reading it just took")

        write(self.dir, "crates/demo/src/bare.rs", BARE.replace("nonexistent", "absent"))
        code, out = run_out()
        after = census.take()["dangling_citations"]
        self.assertEqual(len(after), len(before), "the count stands still")
        self.assertEqual(code, 1, "and the gate fails anyway")
        self.assertIn("+ fix-absent", out)

    def test_main_fails_when_a_defect_recurs_under_a_name_already_baselined(self):
        """A second occurrence of a baselined string is a new defect, which a
        set comparison calls familiar while the count visibly rises."""
        write(self.dir, "docs/demo-Spec.md", CORPUS + CORPUS)
        self.assertEqual(run("--update"), 0)
        write(self.dir, "docs/demo-Spec.md", CORPUS + CORPUS + CORPUS)
        self.assertEqual(run(), 1, "a third copy is new")

    def test_a_member_that_is_not_there_refuses_rather_than_counting_zero(self):
        write(self.dir, "Cargo.toml", 'members = [\n  "crates/gone",\n]\n')
        with self.assertRaises(SystemExit):
            census.take()


def run(*args):
    out = io.StringIO()
    was = os.sys.argv
    os.sys.argv = ["census"] + list(args)
    try:
        with redirect_stdout(out):
            return census.main()
    finally:
        os.sys.argv = was


def run_out():
    out = io.StringIO()
    was = os.sys.argv
    os.sys.argv = ["census"]
    try:
        with redirect_stdout(out):
            code = census.main()
    finally:
        os.sys.argv = was
    return code, out.getvalue()


def write(root, rel, text):
    with open(os.path.join(root, rel), "w", encoding="utf-8") as fh:
        fh.write(text)


if __name__ == "__main__":
    unittest.main(verbosity=2)
