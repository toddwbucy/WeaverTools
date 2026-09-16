#!/usr/bin/env python3
"""The fixture for `grounds_parity.py`, run beside it.

**The stemmer is what this mostly tests, because the stemmer is what broke.** The
first form split on hyphens and compared words, so `dependencies` never met
`dependency`. The second reached for lemmas and split the pairs it was written to
unify: `carries` became `carry` while `carried` became `carri`, `cases` became `cas`
while `case` stayed `case`, `closes` became `clos` while `close` stayed `close`, and
`string` became `str` while `strings` became `string`. Those sentences name eleven
corpus words and four stemmer outputs, and ten of the eleven are live as slug
tokens, `cases` having left the slugs 2026-09-15. A table built on that reading
reports two statements of one clause as two clauses.

So the fixtures are not invented words. Each group below is a set of inflections
this corpus actually writes, and the test is that they reach one string. What that
string is does not matter and is deliberately not asserted: the stemmer truncates
towards a common prefix rather than towards a lemma, and pinning the output would
make a better truncation a test failure.

**Two different corpora are read in this file and confusing them is what the
comments here kept getting wrong.** `CONVERGE` is prose: every word in it is
written somewhere in `docs` or `process`, all thirty-six of them, and that is the
only claim it makes. The liveness test below reads **slug tokens**, the words
`node:` identifiers are built from, which is a far smaller set - eight `CONVERGE`
words are spelled by no node id at all. A word can leave the slugs while staying
ordinary prose, which is exactly what `cases` did, and neither fact says anything
about the other.
"""

import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import grounds_parity as gp


# Each tuple is one word's inflections as this corpus writes them in prose. That
# is the whole claim: no group is tied to a node id, and none needs to be. What
# the tuples watch is the stemmer, which must bring every member of a group to
# one string.
CONVERGE = [
    ("carries", "carried", "carry"),
    ("denies", "denied", "deny"),
    ("retries", "retried", "retry"),
    ("cases", "case"),
    ("closes", "close", "closed"),
    ("strings", "string"),
    ("ends", "end"),
    ("clears", "cleared", "clear"),
    ("refuses", "refused", "refuse"),
    ("dependencies", "dependency"),
    ("owns", "owned", "own"),
    ("pinned", "pin"),
    ("splices", "splicing"),
    ("states", "stated", "state"),
]

# Words that must NOT collapse into each other. A stemmer that truncates can
# over-reach, and an over-reaching stemmer builds clusters out of unrelated
# clauses, which is the opposite failure and just as wrong.
DISTINCT = [
    ("string", "strong"),
    ("bound", "bind"),
    ("read", "recorded"),
    ("close", "clock"),
    ("device", "deviant"),
]


class Stemmer(unittest.TestCase):
    def test_inflections_of_one_word_meet(self):
        for group in CONVERGE:
            got = {gp.stem(w) for w in group}
            self.assertEqual(len(got), 1, f"{group} -> {sorted(got)}")

    def test_unrelated_words_stay_apart(self):
        for a, b in DISTINCT:
            self.assertNotEqual(gp.stem(a), gp.stem(b), f"{a} and {b} collapsed")

    def test_short_words_survive(self):
        # A rule that fired on a three-letter word would eat `one`, `two`, `no`
        # and `set`, which carry the meaning of a slug rather than decorate it.
        for w in ("no", "one", "two", "set", "end", "own", "pin"):
            self.assertEqual(gp.stem(w), w, w)

    def test_no_rule_strips_a_vowelless_remainder(self):
        # `string` -> `str` is the shape of the defect: a suffix rule that does
        # not check what it leaves behind.
        self.assertEqual(gp.stem("string"), gp.stem("strings"))
        self.assertTrue(len(gp.stem("string")) > 3)

    def test_stems_drops_only_the_listed_words(self):
        self.assertEqual(gp.stems("the-a-of"), set())
        self.assertIn("no", gp.stems("no-internal-dependency"))


class Clustering(unittest.TestCase):
    def test_the_named_instance_the_first_form_missed(self):
        # `internal-no-dependencies` against `trace-no-internal-dependency`, which
        # scored 0.25 under the first reading and is the reason this file exists.
        a = gp.stems("no-dependencies")
        b = gp.stems("no-internal-dependency")
        self.assertGreaterEqual(gp.jaccard(a, b), gp.CUT)

    def test_the_band_floor_admits_one_stem_of_three(self):
        # BAND was 0.34, which sits just above one third and hid exactly the
        # near misses the band exists to show.
        self.assertGreaterEqual(1 / 3, gp.BAND)
        self.assertGreater(gp.CUT, gp.BAND)

    def test_jaccard_is_zero_on_an_empty_side(self):
        self.assertEqual(gp.jaccard(set(), {"a"}), 0.0)

    def test_prefix_and_slug_round_trip(self):
        self.assertEqual(gp.prefix_of("weaver-spu"), "spu")
        self.assertEqual(gp.prefix_of("spu"), "spu")
        self.assertEqual(gp.slug_of("spu-one-binary", "weaver-spu"), "one-binary")
        # An identifier that does not carry its crate's prefix keeps its whole
        # name rather than being silently truncated by the wrong number of bytes.
        self.assertEqual(gp.slug_of("types-tagging-test", "weaver-traits"),
                         "types-tagging-test")


class Corpus(unittest.TestCase):
    """The walk against the tree it is written for, which must at least read."""

    def setUp(self):
        self.assertions, self.grounds, self.crates, self.faults = gp.read_corpus()

    def test_the_walk_reads_every_edge_it_meets(self):
        self.assertEqual(self.faults, [])

    def test_every_assertion_has_a_crate(self):
        self.assertEqual(sorted(self.assertions), sorted(self.crates))

    def test_every_grounds_edge_runs_from_an_assertion(self):
        for src in self.grounds:
            self.assertIn(src, self.assertions, src)

    def test_the_tokens_the_second_form_split_are_in_the_corpus(self):
        # A chosen subset of slug tokens, not every CONVERGE word and not
        # every live token: these are the spellings the second stemmer split,
        # so a rename that retires one would quietly restore the bug this
        # fixture exists to catch. Fifteen further CONVERGE words are live
        # slug tokens and are deliberately absent - they were never split and
        # nothing about them is at risk.
        #
        # `cases` was removed 2026-09-15. The only slug spelling the plural was
        # `harness-outcome-two-cases`, renamed `harness-outcome-one-case` when
        # the Spec was read against an enum of one variant. Its CONVERGE pair
        # stays, because CONVERGE is prose and `cases` is still written 85
        # times in `docs` and twice more in `process`. The two sets move
        # independently, per the docstring.
        live = set()
        for node_id, crate in self.crates.items():
            live.update(gp.slug_of(node_id, crate).split("-"))
        for word in ("carried", "carries", "denies", "denied", "retries",
                     "retried", "case", "closes", "close", "string",
                     "strings", "dependencies", "dependency"):
            self.assertIn(word, live, f"{word} is no longer a corpus token")


if __name__ == "__main__":
    unittest.main(verbosity=2)
