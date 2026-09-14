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
        # Its citation counts, so the corpus's one uncited perturbation is
        # now cited and the backlog is empty.
        self.assertEqual(reading["uncited_perturbations"], [])

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
                         ["fix-uncited-perturbation"])

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
        watch at all: delete it and nothing moved. **Matched by behaviour**, the
        way the archive test does it, so a rename of the pattern that keeps the
        effect passes and one that loses it fails.
        """
        import fnmatch

        with open(os.path.join(REPO, ".hadesignore"), encoding="utf-8") as fh:
            lines = [ln.strip() for ln in fh
                     if ln.strip() and not ln.startswith("#")]
        # The negations are literal by design - a negation that drifts is a
        # gate silently leaving the graph.
        for needed in ("process/*", "!process/gates/", "!process/ingest/"):
            self.assertIn(needed, lines, f".hadesignore lost {needed}")
        # The fixture rule is matched on what it excludes. Directory patterns
        # end in "/", so match the directory path with the slash stripped.
        dirs = [p[:-1] for p in lines if p.endswith("/") and not p.startswith("!")]
        def excluded(path):
            return any(fnmatch.fnmatchcase(path, d) for d in dirs)
        self.assertTrue(excluded("crates/weaver-spu/tests/fixtures"),
                        "a crate's tests/fixtures/ is not excluded")
        self.assertTrue(excluded("crates/weaver-analysis/tests/fixtures"))
        # And it must not reach a test file or a source directory.
        self.assertFalse(excluded("crates/weaver-spu/tests"))
        self.assertFalse(excluded("crates/weaver-spu/src/fixtures_loader"))

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
        self.assertIn("experiments/", lines)
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
    import subprocess
    subprocess.run(["git", "-C", root, *args], check=True, capture_output=True)


def write(root, rel, text):
    with open(os.path.join(root, rel), "w", encoding="utf-8") as fh:
        fh.write(text)
    import subprocess
    if os.path.isdir(os.path.join(root, ".git")):
        subprocess.run(["git", "-C", root, "add", "-A"], check=True, capture_output=True)


if __name__ == "__main__":
    unittest.main(verbosity=2)
