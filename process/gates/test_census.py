#!/usr/bin/env python3
"""A fixture corpus with one of each defect the census claims to count.

**The gate was wrong three times before it had a test**, and each time it
printed a confident number: seventy sound citations called dangling, four
documents called self-inconsistent, and fifty four `compile-pin` and
`compile-fail` assertions called untagged. A fixture holding one of each
defect would have caught all three, which is the argument this file is.

    python3 process/gates/test_census.py
"""

import importlib.util
import os
import tempfile
import unittest

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
| only one row, against five nodes | review |
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

    def tearDown(self):
        census.ROOT = self.was

    def test_each_defect_is_counted_once_and_named(self):
        reading, examined = census.take()

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

        # The table is found by its own header, and the count of tables
        # examined is reported so a zero cannot read as assurance.
        self.assertEqual(examined, 1)
        self.assertEqual(len(reading["enforcement_table_mismatch"]), 1)

        # A test file cites and does not owe; `src/bare.rs` owes and does not
        # carry, and `src/lib.rs` carries.
        self.assertEqual(reading["sources_without_a_header"], ["crates/demo/src/bare.rs"])

    def test_a_swapped_defect_is_new_even_though_the_count_stands_still(self):
        reading, _ = census.take()
        was = set(reading["dangling_citations"])
        write(self.dir, "crates/demo/src/bare.rs", BARE.replace("nonexistent", "absent"))
        now = census.take()[0]["dangling_citations"]
        self.assertEqual(len(now), len(was), "the count is unchanged")
        self.assertTrue([x for x in now if x not in was], "and the defect is new")


def write(root, rel, text):
    with open(os.path.join(root, rel), "w", encoding="utf-8") as fh:
        fh.write(text)


if __name__ == "__main__":
    unittest.main(verbosity=2)
