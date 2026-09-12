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
from contextlib import redirect_stderr, redirect_stdout

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

node:fix-no-separator
kind: assertion
tag: perturbation

node: fix-kind-no-separator
kind:assertion
tag: review

node: fix-empty-tag
kind: assertion
tag:

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
/// conforms: Fix_Broken.Citation
/// conforms: fix-cited-pin trailing words
/// conforms:fix-cited-pin
/// the header form reads `conforms: <crate>-<slug>` and is not a citation
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
        # **Each restore is registered as its global is mutated.** A failure
        # in the git calls below would otherwise skip tearDown and leave
        # `census.ROOT` pointed at a temp directory for the rest of the suite,
        # where every later test reads an empty corpus and reports zeros.
        self.addCleanup(shutil.rmtree, self.dir, True)
        self.was = census.ROOT
        self.addCleanup(setattr, census, "ROOT", census.ROOT)
        census.ROOT = self.dir
        # **The fixture is a repository**, because the gate reads the tracked
        # set rather than walking, which is the format's own rule and the one
        # thing a walk cannot honour.
        import subprocess
        for cmd in (["init", "-q"], ["add", "-A"]):
            subprocess.run(["git", "-C", self.dir] + cmd, check=True,
                           capture_output=True)
        self.baseline = census.BASELINE
        self.addCleanup(setattr, census, "BASELINE", census.BASELINE)
        census.BASELINE = os.path.join(self.dir, "baseline.json")

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

        # **A broken header and a broken declaration are two things.** One key
        # reporting both leaves a reader unable to tell which they have.
        # Three broken headers and one piece of prose that is not a header:
        # a bad identifier, a trailing word after a sound one, and a missing
        # separator. The prose mentions the form and must not be a finding.
        bad = " ".join(reading["malformed_citations"])
        self.assertEqual(len(reading["malformed_citations"]), 3, bad)
        self.assertIn("Fix_Broken.Citation", bad)
        self.assertIn("trailing words", bad)
        self.assertIn("conforms:fix-cited-pin", bad)
        self.assertNotIn("<crate>", bad, "prose quoting the form is not a citation")
        self.assertNotIn(
            "Fix_Broken.Citation", " ".join(reading["malformed_node_ids"])
        )
        self.assertEqual(reading["uncited_perturbations"], ["fix-uncited-perturbation"])

        # **A block declares several nodes**, so a reader taking the first
        # per block sees one of six and calls the rest uncited.
        self.assertEqual(len(reading["duplicate_node_ids"]), 1)
        self.assertIn("fix-cited-pin", reading["duplicate_node_ids"][0])

        # A malformed id is named rather than dropped into silence, where it
        # would make every citation to it read as dangling.
        # A bad identifier, and a record with no space after its key - the
        # second is a grammar the format does not admit even though the
        # identifier in it is sound.
        ids = " ".join(reading["malformed_node_ids"])
        self.assertEqual(len(reading["malformed_node_ids"]), 4, ids)
        self.assertIn("fix-Malformed-Id", ids)
        self.assertIn("no space after the key", ids)
        # A field written `kind:assertion`, and one written `tag:` with nothing
        # after it - both shapes the grammar does not admit, and neither is an
        # untagged assertion, which is a legitimate thing to be.
        self.assertIn("fix-kind-no-separator", ids)
        self.assertIn("fix-empty-tag", ids)

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
        with open(census.BASELINE, encoding="utf-8") as fh:
            before = json.load(fh)["dangling_citations"]
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

    def test_update_names_a_recurrence_under_a_name_it_already_holds(self):
        """**The reporting path compares as the gating path does.** A set
        comparison calls the second occurrence familiar and moves the baseline
        in silence, which leaves the act's commit message nothing to say."""
        write(self.dir, "docs/demo-Spec.md", CORPUS + CORPUS)
        self.assertEqual(run("--update"), 0)
        write(self.dir, "docs/demo-Spec.md", CORPUS + CORPUS + CORPUS)
        code, out = run_out("--update")
        self.assertEqual(code, 0)
        self.assertIn("duplicate_node_ids", out, f"the move is named: {out}")

    def test_a_document_with_two_enforcement_tables_counts_both(self):
        """Stopping at the first counts one section's rows against the whole
        document's assertions - a loud false mismatch beside a silent omission
        of the rows that do exist."""
        # **Adjacent, with no blank line.** Separated by one, a row loop that
        # runs past its own table still stops at the blank; written back to
        # back it counts the next header and separator as claims and then
        # counts that table's rows again. Two tables of one row read as five.
        two = CORPUS.rstrip("\n") + "\n| claim | instrument |\n|---|---|\n| a second | review |\n"
        write(self.dir, "docs/demo-Spec.md", two)
        reading = census.take()
        # Four assertions against two tables of one row each: the mismatch
        # names two rows. Stopping at the first would name one.
        self.assertEqual(
            reading["enforcement_table_mismatch"], ["docs/demo-Spec.md (4 nodes, 2 rows)"]
        )

    def test_a_new_file_not_yet_staged_still_owes_a_header(self):
        """**The gate is run mid-act**, which is when a source file is written
        and not yet added. Reading the index alone prints a clean pass on the
        very work the gate was added to check."""
        path = os.path.join(self.dir, "crates/demo/src/fresh.rs")
        with open(path, "w", encoding="utf-8") as fh:
            fh.write("// no header at all\n")
        reading = census.take()
        self.assertIn("crates/demo/src/fresh.rs", reading["sources_without_a_header"])

    def test_a_tracked_file_deleted_and_unstaged_does_not_crash_the_gate(self):
        """The other half of the same mid-act state."""
        os.remove(os.path.join(self.dir, "crates/demo/src/bare.rs"))
        reading = census.take()
        self.assertNotIn("crates/demo/src/bare.rs", reading["sources_without_a_header"])

    def test_a_second_identical_dangling_citation_is_a_second_defect(self):
        """**The set is upstream of the multiset.** Folding citations into a
        set removes a repeat before `main` can compare by identity - the same
        set-for-multiset mistake the gating and update paths each carried."""
        self.assertEqual(run("--update"), 0)
        write(self.dir, "crates/demo/src/second.rs",
              "//! conforms: fix-nonexistent\n")
        code, out = run_out()
        self.assertEqual(code, 1, out)
        self.assertIn("+ fix-nonexistent", out)

    def test_an_unrecognised_argument_refuses(self):
        self.assertEqual(run("--updat"), 2)

    def test_a_member_that_is_not_there_refuses_rather_than_counting_zero(self):
        write(self.dir, "Cargo.toml", 'members = [\n  "crates/gone",\n]\n')
        with self.assertRaises(SystemExit):
            census.take()


def run_out(*args):
    """The gate, with both streams captured.

    **Stderr too.** Uncaptured, the gate's own "Something is new" banner
    prints into an all-green run and a passing suite reads as a failing one,
    which is the state that teaches people to stop reading test output.
    """
    out, err = io.StringIO(), io.StringIO()
    was = os.sys.argv
    os.sys.argv = ["census"] + list(args)
    try:
        with redirect_stdout(out), redirect_stderr(err):
            code = census.main()
    finally:
        os.sys.argv = was
    return code, out.getvalue() + err.getvalue()


def run(*args):
    return run_out(*args)[0]


def write(root, rel, text):
    with open(os.path.join(root, rel), "w", encoding="utf-8") as fh:
        fh.write(text)
    import subprocess
    if os.path.isdir(os.path.join(root, ".git")):
        subprocess.run(["git", "-C", root, "add", "-A"], check=True, capture_output=True)


if __name__ == "__main__":
    unittest.main(verbosity=2)
