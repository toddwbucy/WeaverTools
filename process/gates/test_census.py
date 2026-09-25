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
import subprocess
import tempfile
import unittest
from contextlib import redirect_stderr, redirect_stdout

HERE = os.path.dirname(os.path.abspath(__file__))
# The repository, captured before any test repoints `census.ROOT` at a fixture.
REPO = os.path.dirname(os.path.dirname(HERE))
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

node: fix-cited-by-the-kernel
kind: assertion
tag: perturbation

node: fix-cited-by-the-loop
kind: assertion
tag: perturbation

node: fix-cited-by-the-manifest
kind: assertion
tag: perturbation

node: fix-cited-below-the-head
kind: assertion
tag: perturbation

node: fix-cited-in-the-body
kind: assertion
tag: perturbation

node: fix-cited-only-in-a-string
kind: assertion
tag: perturbation

node: fix-cited-beside-a-string
kind: assertion
tag: perturbation

node: fix-cited-trailing-a-statement
kind: assertion
tag: perturbation

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


KERNEL = """//! conforms: fix-cited-pin
//! conforms: fix-cited-by-the-kernel
//
// A CUDA unit owes a header and heads a file the way Rust does, `//!` being a
// comment in both languages. **Its second citation is reachable through the
// CUDA kind and no other**, so dropping `*.cu` from the selector moves that
// perturbation into the uncited column and the suite says which kind left.

__global__ void k() {}
"""

BARE_KERNEL = """// A CUDA unit with no header citation, which is the finding.
//
// **The headed kernel above cannot hold the obligation by itself.** It is
// absent from `sources_without_a_header` when the walk reads it and finds a
// header and equally absent when the walk never reaches it, so the assertion
// about it passes under the widening and under its absence alike. This file
// is what tells the two apart.

__global__ void k() {}
"""

LOOP = """# conforms: fix-cited-pin
# conforms: fix-cited-by-the-loop
#
# A Python unit owes a header too, and its file-level leader is `#`, there
# being no `//!` a Python file could legally carry.


def drive(seat, text):
    pass
"""

BARE_LOOP = """# A Python unit with no header citation, which is the finding.


def drive(seat, text):
    pass
"""

INLINE_LOOP = """import os


def drive(seat, text):
    # conforms: fix-cited-in-the-body
    return os.getcwd()
"""

# **Column zero and still not the header**, which is the half `INLINE_LOOP`
# cannot hold. An indented citation is out of the opening block by the block's
# own rule, so a reader matching `#` over the whole text would still miss it
# and the position anchor would read as held while doing nothing. This one
# differs from a header in position alone.
BELOW_HEAD_LOOP = """import os


# conforms: fix-cited-below-the-head
def drive(seat, text):
    return os.getcwd()
"""

# **One unit holding both directions, because neither holds alone.** A fixture
# carrying only the string case passes identically if Python citations stop
# being read at all, which is the watch that cannot fail. So the comment
# citation below must resolve in the same run the two inside the docstring do
# not. `fix-never-declared-in-a-string` is declared nowhere, so reading the
# string puts it in `dangling_citations` as well.
STRING_LOOP = '''"""A module docstring that quotes the header form.

# conforms: fix-cited-only-in-a-string
# conforms: fix-never-declared-in-a-string
"""

# conforms: fix-cited-beside-a-string
def drive(seat, text):
    return 1  # conforms: fix-cited-trailing-a-statement
'''

# **A unit the tokenizer refuses**, the triple quote never closing, so every
# line below it is inside a string that has no end. It reads as no citations
# and is named, which is the branch that must not pass at a quiet zero.
BROKEN_LOOP = '''x = """

# conforms: fix-cited-pin
'''

MANIFEST = """[package]
name = "demo"
# conforms: fix-cited-by-the-manifest
"""

# **A leader the reader would match, so the exclusion is what is being
# watched and not the comment syntax.** Were `.sql` ever walked, this line
# resolves to nothing and the migration lands in `dangling_citations`, which
# the assertions below pin as empty of it.
MIGRATION = """-- a migration
# conforms: fix-sql-is-never-read
CREATE TABLE demo (id INTEGER PRIMARY KEY);
"""


class Census(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.mkdtemp()
        os.makedirs(os.path.join(self.dir, "docs"))
        os.makedirs(os.path.join(self.dir, "crates/demo/src"))
        os.makedirs(os.path.join(self.dir, "crates/demo/tests"))
        os.makedirs(os.path.join(self.dir, "crates/demo/kernels"))
        os.makedirs(os.path.join(self.dir, "crates/demo/dev_python"))
        os.makedirs(os.path.join(self.dir, "crates/demo/migrations"))
        write(self.dir, "Cargo.toml", 'members = [\n  "crates/demo",\n]\n')
        write(self.dir, "docs/demo-Spec.md", CORPUS)
        write(self.dir, "crates/demo/src/lib.rs", SOURCE)
        write(self.dir, "crates/demo/src/bare.rs", BARE)
        # A test directory cites and does not owe a header.
        write(self.dir, "crates/demo/tests/it.rs", "// conforms: fix-untagged\n")
        # **One unit of every supported kind, and one of a kind excluded
        # outright**, per the ruling of 2026-09-17. The four dispositions the
        # regime produces are each held by a file here, so a later narrowing
        # of either set fails rather than printing a smaller number.
        write(self.dir, "crates/demo/kernels/demo.cu", KERNEL)
        write(self.dir, "crates/demo/kernels/bare_kernel.cu", BARE_KERNEL)
        write(self.dir, "crates/demo/dev_python/loop.py", LOOP)
        write(self.dir, "crates/demo/dev_python/bare_loop.py", BARE_LOOP)
        write(self.dir, "crates/demo/dev_python/inline_loop.py", INLINE_LOOP)
        write(self.dir, "crates/demo/dev_python/below_head_loop.py", BELOW_HEAD_LOOP)
        write(self.dir, "crates/demo/dev_python/string_loop.py", STRING_LOOP)
        write(self.dir, "crates/demo/dev_python/broken.py", BROKEN_LOOP)
        write(self.dir, "crates/demo/Cargo.toml", MANIFEST)
        write(self.dir, "crates/demo/migrations/0001_demo.sql", MIGRATION)
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
        self.assertEqual(len(reading["unknown_tags"]), 1)
        odd = reading["unknown_tags"][0]
        # **The entry names its document and its kind**, there being two
        # vocabularies now: without the kind a reader cannot tell whether the
        # tag is wrong or the record typed itself wrong.
        self.assertIn("docs/demo-Spec.md", odd)
        self.assertIn("fix-odd-tag", odd)
        self.assertIn("socket", odd)

        self.assertEqual(reading["dangling_citations"], ["fix-nonexistent"])

        # **A broken header and a broken declaration are two things.** One key
        # reporting both leaves a reader unable to tell which they have.
        # Three broken headers and one piece of prose that is not a header:
        # a bad identifier, a trailing word after a sound one, and a missing
        # separator. The prose mentions the form and must not be a finding.
        bad = " ".join(reading["malformed_citations"])
        # **Four: three broken headers and one unit this reader cannot open.**
        # A `.py` file the tokenizer refuses reads as no citations, which is
        # the safe direction, and is named here so the run fails rather than
        # passing at a quiet zero on the files nobody can check by eye.
        self.assertEqual(len(reading["malformed_citations"]), 4, bad)
        self.assertIn("broken.py: will not tokenize", bad)
        self.assertIn("Fix_Broken.Citation", bad)
        self.assertIn("trailing words", bad)
        self.assertIn("conforms:fix-cited-pin", bad)
        self.assertNotIn("<crate>", bad, "prose quoting the form is not a citation")
        self.assertNotIn(
            "Fix_Broken.Citation", " ".join(reading["malformed_node_ids"])
        )
        # **`fix-cited-only-in-a-string` is cited nowhere a reader would call a
        # citation**, its one sighting sitting inside a docstring, so it stands
        # in the backlog. Reading the raw text takes it out and clears a
        # backlog entry no instrument bought.
        self.assertEqual(
            reading["uncited_perturbations"],
            [
                "fix-cited-only-in-a-string",
                "fix-cited-trailing-a-statement",
                "fix-uncited-perturbation",
            ],
        )

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
        # **A node this gate cannot name is still a node the document
        # declares.** The reject used to return before the assertion
        # accounting, so a malformed identifier shrank its own document's
        # mismatch and the count read closer to its table than the document
        # was. The fixture declares thirteen assertions and its table holds one.
        self.assertIn("(13 nodes, 1 rows)", reading["enforcement_table_mismatch"][0])

        # **The obligation follows the unit, not a directory**, so the test
        # file owes one too and the format's rule is what this pins.
        self.assertEqual(
            reading["sources_without_a_header"],
            [
                "crates/demo/dev_python/bare_loop.py",
                "crates/demo/dev_python/below_head_loop.py",
                "crates/demo/dev_python/broken.py",
                "crates/demo/dev_python/inline_loop.py",
                "crates/demo/dev_python/string_loop.py",
                "crates/demo/kernels/bare_kernel.cu",
                "crates/demo/src/bare.rs",
                "crates/demo/tests/it.rs",
            ],
        )

    def test_the_count_asks_a_header_of_every_supported_unit(self):
        """The ruling of 2026-09-17, held as the four dispositions it produces.

        **Reading a citation and owing a header are two sets and they widen
        differently**, so each is held by an assertion of its own rather than
        one standing for the other. The manifest is the one supported kind
        that reads and does not owe. A kind the ingest never reads is counted
        by neither, because a header exists so the graph can carry
        `code -> assertion -> doc` and a unit with no edge has nothing for a
        header to carry.

        **A headed unit alone holds nothing.** It is absent from the
        headerless list when the walk reads it and finds a header and equally
        absent when the walk never reaches it, which is a watch that cannot
        fail. So every supported kind carries a bare unit for the owing half
        and a perturbation cited from it and nowhere else for the read half,
        and a narrowing of either set fails on the kind it dropped.
        """
        reading = census.take()
        bare = reading["sources_without_a_header"]
        # **The read half is held by a perturbation reachable through one kind
        # and no other**, because an absence from the headerless list is the
        # same absence whether the unit was read and headed or never walked.
        # Each is `perturbation`, so a kind leaving the selector moves its own
        # identifier into `uncited_perturbations` and names itself.
        cited = reading["uncited_perturbations"]

        # Owes, and carries one. The CUDA unit heads with `//!` and the Python
        # unit with `#`, each its own language's file-level leader.
        self.assertNotIn("crates/demo/kernels/demo.cu", bare)
        self.assertNotIn("crates/demo/dev_python/loop.py", bare)
        self.assertNotIn("fix-cited-by-the-kernel", cited)
        self.assertNotIn("fix-cited-by-the-loop", cited)
        # Owes, and carries none. These are the findings the widening buys,
        # and the bare kernel is what holds the CUDA half in both directions.
        self.assertIn("crates/demo/kernels/bare_kernel.cu", bare)
        self.assertIn("crates/demo/dev_python/bare_loop.py", bare)
        # **Owes, cites, and is headerless all the same.** `#` opens every
        # Python comment, so a citation below the unit's opening block is an
        # item's and heads nothing. The citation still resolves, which is the
        # distinction the Format draws for `///` and `//` and which a reader
        # matching `#` anywhere would erase for Python alone.
        self.assertIn("crates/demo/dev_python/inline_loop.py", bare)
        self.assertIn("crates/demo/dev_python/below_head_loop.py", bare)
        self.assertNotIn("fix-cited-in-the-body", cited)
        self.assertNotIn("fix-cited-below-the-head", cited)
        # **A `#` line inside a string is the string's and not a citation**,
        # and the comment in the same unit must resolve in the same run, so
        # neither direction passes on its own.
        self.assertNotIn("fix-cited-beside-a-string", cited)
        self.assertIn("fix-cited-only-in-a-string", cited)
        # **And a marker trailing other code carries none**, per Document
        # Format section 4, which the tokenized path has to carry across
        # because a `COMMENT` token has no idea what precedes it.
        self.assertIn("fix-cited-trailing-a-statement", cited)
        self.assertNotIn(
            "fix-never-declared-in-a-string", reading["dangling_citations"]
        )
        # Reads a citation and owes nothing, having no module to head.
        self.assertNotIn("crates/demo/Cargo.toml", bare)
        self.assertNotIn("fix-cited-by-the-manifest", cited)
        # Counted by neither, and the citation inside it proves the read half:
        # were `.sql` walked, this identifier would stand in the dangling list.
        self.assertNotIn("crates/demo/migrations/0001_demo.sql", bare)
        self.assertNotIn("fix-sql-is-never-read", reading["dangling_citations"])
        self.assertNotIn(
            "fix-sql-is-never-read", " ".join(reading["malformed_citations"])
        )

    def test_the_hadesignore_and_the_census_draw_one_boundary(self):
        """The ingest half and the reading half answer about the same set.

        **`.hadesignore` excludes SQL outright and the census counts it
        never**, which is one ruling in two instruments, so a later act
        lifting one without the other is what this fails on. The supported
        kinds must reach neither exclusion: a `.cu` or `.py` unit the ingest
        skipped would owe a header for an edge that could not exist.
        """
        ignored = ignore_probe(self.dir, "boundaryprobe")
        self.assertTrue(ignored("crates/weaver-web/migrations/0001.sql", False),
                        "SQL is not excluded from the ingest")
        for rel in ("crates/weaver-spu/kernels/transformer.cu",
                    "crates/weaver-harness/src/bin/pyworker/dev_python/a.py"):
            self.assertFalse(ignored(rel, False),
                             f"{rel} owes a header and is excluded from the "
                             "graph, so the edge it owes cannot be drawn")
        # And the reading agrees, on the repository's own tree.
        self.assertEqual(
            [p for p in census.READS_CITATIONS if p.endswith(".sql")], [])
        self.assertNotIn(".sql", census.OWES_A_HEADER)

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

    # **The system record, whose two halves are one exemption.** Document
    # Format section 3 declares one node outside the kebab rule and section 5
    # gives it a vocabulary of its own. Each case below fails if its edit is
    # reverted, which is what the third enforcement device asks of a test: the
    # gate shipped both paths once with no case here and the suite stayed
    # green either way.

    def test_the_system_node_is_not_a_malformed_identifier(self):
        """The one name section 3 declares outside the kebab rule."""
        write(self.dir, "docs/apex.md", SYSTEM_RECORD)
        reading = census.take()

        hit = [e for e in reading["malformed_node_ids"] if "WeaverTools" in e]
        self.assertEqual(hit, [])

    def test_the_system_record_carries_its_own_tag_vocabulary(self):
        """`ratified` is the system record's, per section 5."""
        write(self.dir, "docs/apex.md", SYSTEM_RECORD)
        reading = census.take()

        self.assertEqual([e for e in reading["unknown_tags"] if "WeaverTools" in e], [])

    def test_the_system_vocabulary_does_not_reach_another_kind(self):
        """`ratified` is the system record's and no other record's.

        Without the kind scoping this reads clean.
        """
        write(self.dir, "docs/apex.md",
              SYSTEM_RECORD.replace("kind: system", "kind: crate"))
        reading = census.take()

        hit = [e for e in reading["unknown_tags"] if "WeaverTools" in e]
        self.assertEqual(len(hit), 1)
        self.assertIn("ratified", hit[0])

    def test_the_assertion_vocabulary_does_not_reach_the_system_record(self):
        """The converse, which a single shared set would admit."""
        write(self.dir, "docs/apex.md",
              SYSTEM_RECORD.replace("tag: ratified", "tag: review"))
        reading = census.take()

        hit = [e for e in reading["unknown_tags"] if "WeaverTools" in e]
        self.assertEqual(len(hit), 1)
        self.assertIn("review", hit[0])

    def test_the_name_alone_does_not_buy_the_exemption(self):
        """The pair is the exemption, and the name alone is not enough.

        On the name alone this admits an assertion identifier no conformance
        header could legally cite, NODE_OK still governing the citation side.
        """
        write(self.dir, "docs/apex.md",
              SYSTEM_RECORD.replace("kind: system", "kind: assertion")
                           .replace("tag: ratified", "tag: perturbation"))
        reading = census.take()

        hit = [e for e in reading["malformed_node_ids"] if "WeaverTools" in e]
        self.assertEqual(len(hit), 1)

    def test_a_second_system_record_is_a_finding(self):
        """Section 3 declares one system node, so a second one is reported.

        The name is kebab-valid, so it never needed the exemption and the
        pairing does not reach it. Without the kind check it reads entirely
        clean and carries a vocabulary the format gives to one node.
        """
        write(self.dir, "docs/apex.md",
              SYSTEM_RECORD.replace("node: WeaverTools", "node: not-the-system"))
        reading = census.take()

        name = "not-the-system"
        bad = [e for e in reading["malformed_node_ids"] if name in e]
        self.assertEqual(len(bad), 1)
        self.assertIn("kind: system", bad[0])

    def test_a_system_record_that_drops_its_tag_is_a_finding(self):
        """A kind that owes a tag and carries none is reported.

        Section 5 makes `ratified` what generates the set-level mark, so its
        absence ungrounds the mark where a wrong value would be caught.
        """
        write(self.dir, "docs/apex.md",
              SYSTEM_RECORD.replace("tag: ratified\n", ""))
        reading = census.take()

        hit = [e for e in reading["unknown_tags"] if "WeaverTools" in e]
        self.assertEqual(len(hit), 1)
        self.assertIn("no tag", hit[0])

    def test_the_kind_alone_does_not_buy_the_exemption(self):
        """The fourth quadrant: malformed and typed `system` at once.

        The other fixtures pair a bad name with `kind: crate`, which tests the
        vocabulary side, and a kebab-valid name with `kind: system`, which
        NODE_OK matches whether the exemption fires or not. Without this case
        `exempt = kind == "system"` passes the whole suite.
        """
        write(self.dir, "docs/apex.md",
              SYSTEM_RECORD.replace("node: WeaverTools", "node: NotKebab"))
        reading = census.take()

        bad = [e for e in reading["malformed_node_ids"] if "NotKebab" in e]
        self.assertEqual(len(bad), 1)

    def test_a_malformed_identifier_does_not_hide_the_rest_of_its_record(self):
        """A name this gate cannot read is one finding, not a stop.

        The reject used to return before the kind and the tag were read, so a
        record this gate could not name took its tag, its duplicate and its
        enforcement row out of every other reading in silence.
        """
        write(self.dir, "docs/apex.md",
              SYSTEM_RECORD.replace("node: WeaverTools", "node: NotKebab")
                           .replace("kind: system", "kind: crate"))
        reading = census.take()

        bad = [e for e in reading["malformed_node_ids"] if "NotKebab" in e]
        self.assertEqual(len(bad), 1)
        hit = [e for e in reading["unknown_tags"] if "NotKebab" in e]
        self.assertEqual(len(hit), 1)
        self.assertIn("ratified", hit[0])

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
            reading["enforcement_table_mismatch"], ["docs/demo-Spec.md (13 nodes, 2 rows)"]
        )

    def test_an_experiment_is_read_like_a_crate_and_its_results_are_not(self):
        """The container entry of 2026-09-25: a probe's Spec declares, its
        `code/` cites and owes, its `results/` is read by neither walk.

        **Held from both sides.** A perturbation declared in the experiment's
        Spec and cited only from its code is absent from uncited, which fails
        if either walk stops short of the experiment. A bare unit under `code/`
        owes a header, which fails if the unit walk does not reach it. And a
        citation under `results/` naming no node is absent from dangling, which
        fails if the walk reaches into the records.
        """
        probe = "experiments/demo/arm/probe"
        for part in ["code", "results"]:
            os.makedirs(os.path.join(self.dir, probe, part))
        write(self.dir, "experiments/demo/README.md", "# demo\n")
        write(self.dir, probe + "/probe-Spec.md",
              "```graph\nnode: exp-only-here\nkind: assertion\ntag: perturbation\n\n"
              "edge: asserts\nfrom: probe\nto: exp-only-here\n```\n")
        write(self.dir, probe + "/code/tool.py",
              "#!/usr/bin/env python3\n# conforms: exp-only-here\n")
        write(self.dir, probe + "/code/bare.py", "print('no header')\n")
        write(self.dir, probe + "/results/note.py",
              "# conforms: fix-nonexistent-result\n")
        write(self.dir, probe + "/results/report.md",
              "```graph\nnode: exp-only-here\nkind: assertion\ntag: perturbation\n```\n")
        reading = census.take()
        self.assertNotIn("exp-only-here", reading["uncited_perturbations"])
        self.assertIn(probe + "/code/bare.py", reading["sources_without_a_header"])
        self.assertNotIn(probe + "/code/tool.py", reading["sources_without_a_header"])
        self.assertFalse([d for d in reading["dangling_citations"] if "fix-nonexistent-result" in d])
        # The report under results/ redeclares the id: pruned, so no duplicate.
        self.assertNotIn("exp-only-here", reading["duplicate_node_ids"])

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

    def test_a_file_under_an_archive_directory_is_counted_like_any_other(self):
        """No exemption for `archive/`, per the ruling of 2026-09-13.

        Every archive directory was deleted at PR #563 and none may stand in
        the tree, so the gate's old exclusion protected nothing that exists
        and silently exempted anything a later act might name `archive/`.
        **This is the regression test against it coming back.** Both
        assertions fail if the exemption returns, which is what the two
        archive bugs in the gate's docstring cost when nothing watched them.
        """
        os.makedirs(os.path.join(self.dir, "crates/demo/archive"))
        write(self.dir, "crates/demo/archive/old.rs",
              "/// conforms: fix-uncited-perturbation\nfn gone() {}\n")
        reading = census.take()

        self.assertIn("crates/demo/archive/old.rs",
                      reading["sources_without_a_header"])
        # Its citation counts, so the perturbation an archived file cites
        # leaves the backlog. The one whose only sighting sits inside a
        # docstring stays, no archive reaching it.
        self.assertEqual(reading["uncited_perturbations"],
                         ["fix-cited-only-in-a-string",
                          "fix-cited-trailing-a-statement"])

    def test_the_archive_assertions_are_not_vacuous(self):
        """The watch on the test above.

        Both of its assertions name a path spelling, and a spelling the
        reading never uses makes `assertIn` fail loudly but would make an
        `assertNotIn` pass forever. This pins the spelling against a file the
        gate certainly reports, so a change to how paths are rendered breaks
        here rather than quietly disarming the archive test.
        """
        reading = census.take()

        self.assertIn("crates/demo/src/bare.rs",
                      reading["sources_without_a_header"])
        self.assertEqual(reading["uncited_perturbations"],
                         ["fix-cited-only-in-a-string",
                          "fix-cited-trailing-a-statement",
                          "fix-uncited-perturbation"])

    def test_a_deleted_archive_stops_being_reported_before_it_is_staged(self):
        """The mid-act state, which the first form of this gate failed.

        `archives()` read the index and never the disk, so an act that obeyed
        the rule by deleting an archive failed the gate until the deletion was
        staged. `sources()` has carried the guard for this since the bug list's
        entry on the same subject, forty lines away. Found by the olympus
        review of 2026-09-13.
        """
        os.makedirs(os.path.join(self.dir, "docs/archive"))
        write(self.dir, "docs/archive/old.md", "# gone\n")
        self.assertEqual(census.take()["archive_directories"], ["docs/archive/"])

        # Deleted on disk, still in the index - the state a run is in between
        # `rm` and `git add`.
        shutil.rmtree(os.path.join(self.dir, "docs/archive"))
        self.assertEqual(census.take()["archive_directories"], [])

    def test_the_spellings_a_rule_naming_one_instance_would_miss(self):
        """Section 1's own sentence, applied to this reading.

        A rule naming today's instance is the same mistake as deleting today's
        instance, and the first form matched `archive` exactly.
        """
        for name in ("Archive", "ARCHIVE", "archives", "archived",
                     "_archive", "archive-2026"):
            os.makedirs(os.path.join(self.dir, "docs", name))
            write(self.dir, f"docs/{name}/x.md", "# x\n")
            self.assertEqual(census.take()["archive_directories"],
                             [f"docs/{name}/"], name)
            shutil.rmtree(os.path.join(self.dir, "docs", name))
            run_git(self.dir, "add", "-A")

        # **`archiver` is a word.** The family stops at a separator so a crate
        # or module whose name merely begins with the stem is not a finding.
        os.makedirs(os.path.join(self.dir, "docs/archiver"))
        write(self.dir, "docs/archiver/x.md", "# x\n")
        self.assertEqual(census.take()["archive_directories"], [])

    def test_an_untracked_archive_is_reported_before_it_is_added(self):
        """The branch `write()` cannot reach.

        Every `write()` in this file runs `git add -A`, so both of the first
        tests exercised the tracked path only and dropping `--others` from
        `archives()` left all fourteen green. `sources()` has the equivalent
        watch and bypasses `write()` the same way.
        """
        os.makedirs(os.path.join(self.dir, "docs/archive"))
        with open(os.path.join(self.dir, "docs/archive/new.md"), "w") as fh:
            fh.write("# not added\n")
        self.assertEqual(census.take()["archive_directories"], ["docs/archive/"])

    def test_a_nested_archive_is_one_entry(self):
        """One directory, one row.

        The comparison is by identity, so a nested pair reported twice moves
        two baseline rows when one directory is removed.
        """
        os.makedirs(os.path.join(self.dir, "docs/archive/sub/archive"))
        write(self.dir, "docs/archive/sub/archive/a.md", "# a\n")
        self.assertEqual(census.take()["archive_directories"], ["docs/archive/"])

    def test_main_fails_and_names_the_archive_it_found(self):
        """Through `main`, which is what this file's docstring asks for.

        The metric exists to fail the gate. Both of the first tests called
        `take()`, so nothing pinned that an appearing archive returns 1 and
        prints the path.
        """
        self.assertEqual(run("--update"), 0)
        os.makedirs(os.path.join(self.dir, "docs/archive"))
        write(self.dir, "docs/archive/old.md", "# gone\n")

        code, out = run_out()
        self.assertEqual(code, 1)
        self.assertIn("docs/archive/", out)

    def test_the_two_halves_of_the_archive_rule_agree(self):
        """The reading and the ignore file match the same names.

        gitignore syntax has no case-insensitivity flag and `ARCHIVE_DIR` uses
        `re.I`, so `ARCHIVE/` was flagged by the census and ingested by HADES -
        the two halves of one rule disagreeing, found by CodeRabbit on PR #566.

        **Congruence in both directions.** A name the reading flags must be
        excluded, or the ingest takes a frozen copy the gate is about to
        refuse. A name the reading passes must not be excluded, or the ingest
        silently drops a directory nobody said to drop - `archiver` being the
        case that decides it.
        """
        import fnmatch
        import itertools

        with open(os.path.join(REPO, ".hadesignore"), encoding="utf-8") as fh:
            patterns = [ln.strip() for ln in fh
                        if ln.strip() and not ln.startswith("#")]
        # The directory-name half of each `**/NAME/` pattern.
        names = [p[3:-1] for p in patterns if p.startswith("**/") and p.endswith("/")]

        def excluded(name):
            return any(fnmatch.fnmatchcase(name, n) for n in names)

        # **Generated and not listed.** The first form of this test checked
        # twelve names somebody wrote out, passed, and left the two halves 93
        # names apart - every one of them a leading underscore combined with a
        # suffix, which is exactly the combination a hand list does not think
        # of. CodeRabbit found it on PR #566.
        disagree = []
        for prefix, stem, plural, suffix in itertools.product(
            ["", "_", "__", "___"],
            ["archive", "Archive", "ARCHIVE", "aRcHiVe"],
            ["", "s", "d", "S", "D"],
            ["", "-2026", "_old", "-x", "_Y"],
        ):
            name = prefix + stem + plural + suffix
            reading = census.ARCHIVE_DIR.match(name) is not None
            if reading != excluded(name):
                disagree.append(f"{name}: reading={reading} ignore={not reading}")
        self.assertEqual(disagree, [], "the two halves match different names")

        # The stem is not a prefix match: these are words, and neither half
        # may claim them.
        for name in ("archiver", "archiving", "arch", "architecture",
                     "archivist", "archival"):
            self.assertIsNone(census.ARCHIVE_DIR.match(name),
                              f"the reading flags {name}")
            self.assertFalse(excluded(name), f".hadesignore excludes {name}")

        # And the plain case is covered by both, so an empty pattern file
        # cannot pass this test by making both halves match nothing.
        self.assertIsNotNone(census.ARCHIVE_DIR.match("archive"))
        self.assertTrue(excluded("archive"))

    def test_update_refuses_to_baseline_an_archive(self):
        """The rule is zero, so the escape hatch is closed in both directions.

        Every other metric may be carried in the baseline, no defect being new.
        This one's rule is that it is zero, and the generic `--update` path
        defeated it: create an archive, run `--update`, and every later run
        passes at one. Found by CodeRabbit on PR #566.
        """
        os.makedirs(os.path.join(self.dir, "docs/archive"))
        write(self.dir, "docs/archive/old.md", "# frozen\n")

        code, out = run_out("--update")
        self.assertEqual(code, 2)
        self.assertIn("docs/archive/", out)
        # **And it wrote nothing.** A refusal that still moved the baseline
        # would be the hole with a message on it.
        self.assertFalse(os.path.exists(census.BASELINE))

    def test_a_baseline_holding_an_archive_is_refused(self):
        """The other direction: a baseline written before that guard existed."""
        self.assertEqual(run("--update"), 0)
        with open(census.BASELINE, encoding="utf-8") as fh:
            base = json.load(fh)
        base["archive_directories"] = ["docs/archive/"]
        with open(census.BASELINE, "w", encoding="utf-8") as fh:
            json.dump(base, fh)

        code, out = run_out()
        self.assertEqual(code, 2)
        self.assertIn("docs/archive/", out)

    def test_the_hadesignore_holds_the_process_negations_and_the_fixture_rule(self):
        """Two lines with no watch until 2026-09-14, and the review named both.

        `process/*` with `!process/gates/` and `!process/ingest/` is the rule
        that keeps the census in the graph and the four documents out. Retype
        one negation and both gates stayed green while the census left the
        graph again - the state f8c8618 fixed. And `**/tests/fixtures/` had no
        watch at all: delete it and nothing moved.

        **Everything here is matched with git's own engine**, which is
        `ignore_probe`'s doing and argued there rather than twice. Both forms
        this test carried before - `fnmatch` for `*/tests/fixtures/` three
        segments deep, and a membership check over lines - passed on manifests
        that exclude the wrong things. Both were found by CodeRabbit on PR
        #575, the second refuting this test's own earlier claim that the
        negations had no behaviour to match against. **A gate whose answer
        depends on the box is the hazard `CLAUDE.md` records for the clippy
        count**, and the same shape the olympus seat found in
        `--exclude-standard` on 2026-09-13.
        """
        ignored = ignore_probe(self.dir, "ignoreprobe")

        # **The process rule, as git reads it.** `process/*` excludes the four
        # documents and the two negations name the instruments back in. Order
        # decides it: move a negation above `process/*` and the line is still
        # present while the gate it names leaves the graph.
        self.assertTrue(ignored("process/WeaverTools-Working-Process.md", False),
                        "a process document is not excluded")
        self.assertFalse(ignored("process/gates/census.py", False),
                         "census.py is excluded - H6 is not in the graph")
        self.assertFalse(ignored("process/ingest/chunk_plan.py", False),
                         "chunk_plan.py is excluded")

        # The fixture rule, the same way.
        self.assertTrue(ignored("crates/weaver-spu/tests/fixtures", True),
                        "a crate's tests/fixtures/ is not excluded")
        self.assertTrue(
            ignored("crates/weaver-analysis/tests/fixtures/a.ndjson", False),
            "a file under tests/fixtures/ is not excluded")
        # And it must not reach a test file or a source directory.
        self.assertFalse(ignored("crates/weaver-spu/tests/native.rs", False))
        self.assertFalse(ignored("crates/weaver-spu/src", True))
        # **The trailing slash is load-bearing and this is what holds it.**
        # `**/tests/fixtures` without it matches a regular file of that name
        # too, and every assertion above passes either way. The rule excludes
        # a directory of data, not a file that happens to share its name.
        self.assertFalse(ignored("crates/weaver-gate/tests/fixtures", False),
                         "the rule reaches a regular file, so it is not "
                         "directory-only")

    def test_the_hadesignore_carries_the_load_bearing_patterns(self):
        """The exclusion half, which had no watch at all.

        Drop `**/archive/` from `.hadesignore` and nothing moved: no gate, no
        test, no reading. HADES honours the file - both ingest walkers call
        `add_custom_ignore_filename` - so the line is live rather than inert,
        and what was missing was anything that notices its removal.

        **The repository's own file and not a fixture**, which is the point.
        """
        path = os.path.join(REPO, ".hadesignore")
        with open(path, encoding="utf-8") as fh:
            body = fh.read()
        import fnmatch

        lines = [ln.strip() for ln in body.splitlines()
                 if ln.strip() and not ln.startswith("#")]
        # **An experiment's results are excluded and its code and docs are
        # not**, per the ruling of 2026-09-25 that returned `experiments/` for
        # a live instrument. The earlier form pinned `experiments/` whole.
        self.assertIn("experiments/**/results/", lines)
        self.assertNotIn("experiments/", lines)
        # **Matched by behaviour and not by spelling.** The archive half is
        # written as character classes, so a substring check for "archive"
        # finds nothing while the patterns work - which is how the first form
        # of this assertion failed on a correct file.
        names = [p[3:-1] for p in lines if p.startswith("**/") and p.endswith("/")]
        self.assertTrue(any(fnmatch.fnmatchcase("archive", n) for n in names),
                        "no pattern in .hadesignore matches an archive directory")

    def test_an_archive_directory_anywhere_is_reported(self):
        """The instrument for the ruling of 2026-09-13.

        Two directories, because the two that existed differed in the way
        that matters: one sat under a workspace member and one did not, and a
        check scoped the way `sources()` is would have caught only the first.
        """
        os.makedirs(os.path.join(self.dir, "crates/demo/archive"))
        os.makedirs(os.path.join(self.dir, "docs/archive/handoffs"))
        write(self.dir, "crates/demo/archive/old.rs", "fn gone() {}\n")
        write(self.dir, "docs/archive/handoffs/old.md", "# gone\n")

        self.assertEqual(census.take()["archive_directories"],
                         ["crates/demo/archive/", "docs/archive/"])

    def test_a_tree_with_no_archive_directory_reports_none(self):
        """The perturbation of the test above.

        Without it that test passes on a reading that names every directory,
        or on one that has stopped filtering at all.
        """
        self.assertEqual(census.take()["archive_directories"], [])

    def test_an_unrecognised_argument_refuses(self):
        self.assertEqual(run("--updat"), 2)

    def test_a_member_that_is_not_there_refuses_rather_than_counting_zero(self):
        write(self.dir, "Cargo.toml", 'members = [\n  "crates/gone",\n]\n')
        with self.assertRaises(SystemExit):
            census.take()


def ignore_probe(parent, name):
    """`.hadesignore` read by git, in a repository of its own.

    **The repository under test cannot answer this.** `check-ignore` there
    reads `.gitignore` and never `.hadesignore`, so it answers a question
    nobody asked and answers it confidently. The manifest is copied into a
    fresh repository as its `.gitignore`, because gitignore syntax is settled
    by nothing but gitignore: `fnmatch` lets `*` cross a `/` and a membership
    check over lines accepts any order, where the rule is last-match-wins.

    **Three guards, each holding a way the answer stops being the manifest's.**
    `core.excludesFile=` empties the box's global rules and the truncation
    empties `info/exclude`, which an init template can seed, so a seat with a
    global `*.py` rule does not read `census.py` as excluded and fail a test
    for a reason no manifest holds. And the path is created before the check,
    a trailing `/` meaning directory-only and `--no-index` having no way to
    tell that an absent path is a directory, so without it the right pattern
    and the wrong one both read as no-match.
    """

    with open(os.path.join(REPO, ".hadesignore"), encoding="utf-8") as fh:
        manifest = fh.read()

    probe = os.path.join(parent, name)
    os.makedirs(probe)
    subprocess.run(["git", "-C", probe, "init", "-q"], check=True,
                   capture_output=True)
    with open(os.path.join(probe, ".gitignore"), "w", encoding="utf-8") as fh:
        fh.write(manifest)
    # An init template can seed info/exclude, so it is emptied rather than
    # assumed absent.
    with open(os.path.join(probe, ".git", "info", "exclude"), "w") as fh:
        fh.write("")

    def ignored(rel, is_dir):
        full = os.path.join(probe, rel)
        os.makedirs(full if is_dir else os.path.dirname(full), exist_ok=True)
        if not is_dir:
            open(full, "w").close()
        return subprocess.run(
            ["git", "-C", probe, "-c", "core.excludesFile=",
             "check-ignore", "--no-index", rel],
            capture_output=True,
        ).returncode == 0

    return ignored


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


def run_git(root, *args):
    subprocess.run(["git", "-C", root, *args], check=True, capture_output=True)


SYSTEM_RECORD = """# apex

```graph
node: WeaverTools
kind: system
tag: ratified
```
"""


def write(root, rel, text):
    with open(os.path.join(root, rel), "w", encoding="utf-8") as fh:
        fh.write(text)
    if os.path.isdir(os.path.join(root, ".git")):
        subprocess.run(["git", "-C", root, "add", "-A"], check=True, capture_output=True)


if __name__ == "__main__":
    unittest.main(verbosity=2)
