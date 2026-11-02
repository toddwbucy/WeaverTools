"""Checks on the lexicon itself and on the standard that pins it.

Without the local lexicon (no ASD-STE100 copy on this computer):
  - every lexicon file parses, and every terms.toml entry has its fields
  - no term in terms.toml is also a technical verb (a technical noun is never a verb)
  - the standard's header line names the lexicon sha256 as it is now, so a lexicon
    change cannot land without the standard being checked again against it

With the local lexicon (ste100/var/approved-verbs.json, from ste_extract.py), also:
  - every "after" rewrite in thesaurus.toml passes the checker with no unapproved word
    (WeaverTrain's tests/test_ste_thesaurus.py, as a plain script)
  - ste100/writing-standard.md passes ste_doc.py with no failure

The second group prints SKIP when the local lexicon is missing; it is not a pass.

    python3 ste100/checker/test_lexicon.py
"""

import json
import sys
import tomllib
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import ste_check  # noqa: E402
import ste_doc  # noqa: E402

LEX_DIR = HERE.parent / "lexicon"
STANDARD = HERE.parent / "writing-standard.md"
VERBS = HERE.parent / "var" / "approved-verbs.json"


def test_every_lexicon_file_parses():
    for p in sorted(LEX_DIR.iterdir()):
        if p.suffix == ".toml":
            tomllib.loads(p.read_text())
        elif p.suffix == ".json":
            json.loads(p.read_text())
        else:
            assert p.suffix == ".md", f"unexpected file in the lexicon: {p.name}"


def test_terms_have_their_fields():
    terms = tomllib.loads((LEX_DIR / "terms.toml").read_text())["terms"]
    assert terms, "terms.toml holds no terms"
    for k, v in terms.items():
        assert "added" in v, k
        if v.get("status") == "unresolved":
            assert len(v.get("senses", [])) >= 2 and v.get("ruling"), k
        else:
            assert "status" not in v, (k, v["status"])
            assert v.get("sense") and v.get("reason"), k


def test_no_term_is_a_technical_verb():
    terms = {k.lower() for k in tomllib.loads((LEX_DIR / "terms.toml").read_text())["terms"]}
    verbs = {k.lower() for k in tomllib.loads((LEX_DIR / "technical-verbs.toml").read_text())["verbs"]}
    assert not terms & verbs, sorted(terms & verbs)


def test_the_standard_pins_the_lexicon_as_it_is():
    heads = [ste_doc.HEADER.match(l) for l in STANDARD.read_text().splitlines()]
    heads = [m for m in heads if m]
    assert len(heads) == 1, "the standard must have one header line"
    assert heads[0].group(1) == ste_doc.STANDARD_VERSION, heads[0].group(1)
    now = ste_doc.lexicon_sha256()
    assert heads[0].group(2) == now, (
        f"the standard names lexicon {heads[0].group(2)}, the lexicon is {now}: "
        "check the standard with ste_doc.py and put the new sha256 in its header line")


def local_thesaurus_rewrites_are_clean_ste(lex):
    for verb, entry in tomllib.loads((LEX_DIR / "thesaurus.toml").read_text()).items():
        for sense in entry["senses"]:
            r = ste_check.check(sense["after"], sense["before"], lex)
            assert r["hard"] == {}, (verb, sense["after"], r["hard"])
            assert "unapproved_words" not in r["soft"], (verb, sense["after"], r["soft"])


def local_the_standard_passes_its_own_checker(lex):
    r = ste_doc.check_document(STANDARD.read_text(), lex, ste_doc.unresolved_terms(),
                               ste_doc.lexicon_sha256())
    assert r["failures"] == [], ste_doc.report(str(STANDARD), r)


if __name__ == "__main__":
    tests = [(n, f) for n, f in sorted(globals().items()) if n.startswith("test_") and callable(f)]
    local = [(n, f) for n, f in sorted(globals().items()) if n.startswith("local_") and callable(f)]
    lex = ste_check.Lexicon.load(VERBS) if VERBS.is_file() else None
    for name, fn in tests + local:
        if name.startswith("local_") and lex is None:
            print(f"SKIP {name}: no local lexicon at {VERBS} (run ste_extract.py)")
            continue
        try:
            fn(lex) if name.startswith("local_") else fn()
        except AssertionError as e:
            print(f"FAIL {name}: {e}")
            sys.exit(1)
        print(f"ok   {name}")
    print("done")
