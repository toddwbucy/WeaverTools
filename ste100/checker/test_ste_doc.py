"""Controls for ste_doc.py, the document driver, on a small synthetic lexicon (ordinary
English words chosen for the cases, no text of ASD-STE100). Each rule is shown failing
alone, and each exclusion is shown hiding only what it names.

    python3 ste100/checker/test_ste_doc.py
"""

import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import ste_check  # noqa: E402
import ste_doc  # noqa: E402

APPROVED = {w: ["x"] for w in (
    "the a an i will is are it this that when if not do does make find examine use "
    "error test function in of to and then one result shows show file data with".split())}
LEX = ste_check.Lexicon.build(
    approved_pos=APPROVED,
    unapproved={"fix": ["v"], "run": ["v"], "honestly": ["adv"]},
    unapproved_verbs={"fix": ["REPAIR (v)"], "run": ["OPERATE (v)"]},
    technical_verbs=["commit"], technical_nouns=["repository", "restatement"],
    approved_verbs=["do", "make", "find", "examine", "use", "show"],
)
HASH = "a" * 64
HEAD = f"STE100: WeaverTools STE100 writing standard {ste_doc.STANDARD_VERSION}, lexicon sha256:{HASH}"


def doc(body: str, header: str = HEAD) -> str:
    return f"# The Fix Running Title\n\n{header}\n\n{body}\n"


def run(body: str, header: str = HEAD, unresolved=frozenset({"restatement"})) -> dict:
    return ste_doc.check_document(doc(body, header), LEX, set(unresolved), HASH)


def rules(r: dict) -> list[str]:
    return sorted(f["rule"] for f in r["failures"])


def test_clean_document_passes_and_counts_sentences():
    r = run("Examine the file. The test shows the error.\n\n- Use the function.")
    assert r["failures"] == [] and r["sentences_checked"] == 3 == r["sentences_passed"], r


def test_title_and_header_line_are_not_checked():
    # the title holds "Fix" and "Running"; the header line is not prose
    r = run("Examine the file.")
    assert r["failures"] == [] and r["sentences_checked"] == 1, r


def test_only_the_first_title_is_skipped():
    r = run("# Fix the error\n\nExamine the file.")
    assert rules(r) == ["verbs"], r


def test_each_checker_rule_fails_alone():
    assert rules(run("I will fix the error.")) == ["verbs"]
    assert rules(run("The test is running.")) == ["verbs"]
    assert rules(run("It is the error; examine it.")) == ["semicolon"]
    assert rules(run("It doesn't show the error.")) == ["contractions"]
    assert rules(run("Examine " + " ".join(["the error"] * 10) + ".")) == ["length"]


def test_capitals_fail_with_no_source_text():
    # a document has no source: a dictionary word in capitals is never copied style
    assert rules(run("The RESULT shows the error.")) == ["capitals"]


def test_unresolved_term_fails_in_prose_and_passes_in_backticks():
    assert rules(run("The restatement shows the error.")) == ["unresolved-term"]
    assert run("The `restatement` shows the error.")["failures"] == []
    assert run("The restatement shows the error.", unresolved=())["failures"] == []


def test_banned_word_fails():
    r = run("The test honestly shows the error.")
    assert "banned-word" in rules(r), r


def test_non_ascii_fails_anywhere():
    r = run("The test shows the error \u2014 examine it.")
    assert "ascii" in rules(r), r
    r = run("```\nx = '\u00e9'\n```")
    assert "ascii" in rules(r), r


def test_header_missing_wrong_version_wrong_hash():
    assert rules(run("Examine the file.", header="")) == ["header"]
    bad_version = HEAD.replace(f"standard {ste_doc.STANDARD_VERSION},", "standard 9.9,")
    assert rules(run("Examine the file.", header=bad_version)) == ["header"]
    assert rules(run("Examine the file.", header=HEAD.replace("a" * 64, "b" * 64))) == ["header"]
    assert rules(run("Examine the file.", header="STE100: some other form")) == ["header"]


def test_header_may_name_a_commit_and_only_the_sha256_is_checked():
    assert run("Examine the file.", header=HEAD + ", WeaverTools commit fbab640")["failures"] == []
    assert run("Examine the file.", header=HEAD + ", WeaverTools commit " + "0" * 40)["failures"] == []
    wrong_hash = HEAD.replace("a" * 64, "b" * 64) + ", WeaverTools commit fbab640"
    assert rules(run("Examine the file.", header=wrong_hash)) == ["header"]
    assert rules(run("Examine the file.", header=HEAD + ", commit fbab640")) == ["header"]


def test_source_variable_prefers_the_suite_name_and_falls_back():
    import ste_extract
    suite, train = ste_extract.SOURCE_VARS
    assert suite == "WEAVERTOOLS_STE100_SOURCE" and train == "WEAVER_TRAIN_STE100_SOURCE"
    assert ste_extract.source_path({suite: "/a", train: "/b"}) == (Path("/a"), suite)
    assert ste_extract.source_path({train: "/b"}) == (Path("/b"), train)
    assert ste_extract.source_path({suite: "", train: "/b"}) == (Path("/b"), train)
    try:
        ste_extract.source_path({})
    except SystemExit:
        pass
    else:
        raise AssertionError("no variable set must stop the extraction")


def test_report_names_the_variable_the_copy_came_from():
    assert ste_doc.source_note({"source_sha256": "d1f4", "source_variable": "WEAVERTOOLS_STE100_SOURCE"}) \
        == "sha256 d1f4, found by $WEAVERTOOLS_STE100_SOURCE"
    assert "not recorded" in ste_doc.source_note({"source_sha256": "d1f4"})
    r = run("Examine the file.")
    r["source"] = "sha256 d1f4, found by $X"
    assert "standard copy: sha256 d1f4, found by $X" in ste_doc.report("d.md", r)


def test_code_and_quotations_are_masked_whole():
    body = "Examine the file.\n\n```python\nrun_and_fix()  # fix running\n```\n\n> I will fix it; run it."
    assert run(body)["failures"] == []


def test_html_comment_is_not_checked():
    assert run("<!-- fix the running error;\nrun it -->\nExamine the file.")["failures"] == []


def test_prose_in_parentheses_is_checked_but_a_call_is_masked():
    # the conversion masks a long parenthesis whole; a document keeps it as prose
    assert rules(run("Examine the file (then fix the error in the function).")) == ["verbs"]
    assert run("Examine the file with run_fix(error, fix).")["failures"] == []


def test_table_cells_are_units_and_separator_rows_give_no_sentence():
    body = "| Item | Result |\n|---|---|\n| The test | I will fix it. |"
    r = run(body)
    assert rules(r) == ["verbs"] and r["sentences_checked"] == 4, r


def test_list_items_are_separate_sentences():
    long_item = "- " + " ".join(["The error is the result"] * 4) + "."   # 20 words
    r = run("\n".join([long_item, long_item]))
    assert r["failures"] == [], r   # 40 words if joined, 20 each as items


def test_a_failure_names_the_line_of_its_own_list_item():
    r = run("- Examine the file.\n- Then fix the error.")
    # doc(): title on line 1, header on line 3, body from line 5
    assert [(f["line"], f["rule"]) for f in r["failures"]] == [(6, "verbs")], r


def test_wrapped_paragraph_is_one_sentence():
    words = ("Examine " + " ".join(["the error"] * 10) + ".").split()
    r = run("\n".join(words))
    assert rules(r) == ["length"], r


def test_lexicon_sha256_ignores_readme_and_tracks_every_other_file():
    with tempfile.TemporaryDirectory() as d:
        p = Path(d)
        (p / "terms.toml").write_text("[terms]\n")
        (p / "a.toml").write_text("x = 1\n")
        h0 = ste_doc.lexicon_sha256(p)
        (p / "README.md").write_text("notes\n")
        assert ste_doc.lexicon_sha256(p) == h0
        (p / "a.toml").write_text("x = 2\n")
        h1 = ste_doc.lexicon_sha256(p)
        assert h1 != h0
        (p / "a.toml").rename(p / "b.toml")   # a renamed file changes the hash too
        assert ste_doc.lexicon_sha256(p) != h1


def test_unresolved_terms_read_from_terms_toml():
    with tempfile.TemporaryDirectory() as d:
        p = Path(d)
        (p / "terms.toml").write_text(
            '[terms.word]\nsense = "x"\n[terms.Restatement]\nstatus = "unresolved"\n')
        assert ste_doc.unresolved_terms(p) == {"restatement"}


def test_main_exits_2_without_the_local_lexicon():
    with tempfile.TemporaryDirectory() as d:
        f = Path(d) / "doc.md"
        f.write_text(doc("Examine the file."))
        assert ste_doc.main([str(f), "--verbs", str(Path(d) / "missing.json")]) == 2
        assert ste_doc.main([str(Path(d) / "missing.md"), "--verbs", str(f)]) == 2


if __name__ == "__main__":
    tests = [(n, f) for n, f in sorted(globals().items()) if n.startswith("test_") and callable(f)]
    for name, fn in tests:
        try:
            fn()
        except AssertionError as e:
            print(f"FAIL {name}: {e}")
            sys.exit(1)
        print(f"ok   {name}")
    print(f"{len(tests)} passed")
