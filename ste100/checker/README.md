# The STE100 checker

The measure of the writing standard (`ste100/writing-standard.md`, section 8): it reads a
Markdown document and reports the sentences checked, the sentences that pass, and each
failure with the rule it breaks. Stdlib Python 3.11 or later (`tomllib`), plus
`pdftotext` (poppler) for the one-time extraction.

## Running it

The checker needs the approved words of ASD-STE100, which are never committed. Build
them once per computer from the operator's local copy:

```sh
export WEAVERTOOLS_STE100_SOURCE=/path/to/ASD-STE100_ISSUE9.pdf
python3 ste100/checker/ste_extract.py          # writes ste100/var/approved-verbs.json (gitignored)
```

Where `WEAVERTOOLS_STE100_SOURCE` is not set, the extraction reads WeaverTrain's
`WEAVER_TRAIN_STE100_SOURCE`, so a box set up for WeaverTrain needs nothing new. The
variable it used is printed, recorded in the local lexicon, and printed again by
`ste_doc.py` on its `standard copy:` line.

The extraction refuses a copy whose sha256 is not the one pinned in
`ste100/lexicon/ste100-scheme.candidate.json`. On this workshop's boxes the copy is on
the shared bulk store, at `/mnt/bulk-store/handoff/ASD-STE100_ISSUE9.pdf`.

Then check a document:

```sh
python3 ste100/checker/ste_doc.py path/to/document.md [--json result.json]
```

Exit 0: every sentence passes and the header line holds. Exit 1: any failure. Exit 2:
the local lexicon or the document is missing. The output is the record a pull request
carries. `ste_doc.py`'s docstring lists the rules and what is not checked.

## Tests

Plain scripts that exit non-zero on the first failure, as elsewhere in this repository:

```sh
python3 ste100/checker/test_ste_check.py   # the checker, on a synthetic lexicon
python3 ste100/checker/test_ste_doc.py     # the document driver, on a synthetic lexicon
python3 ste100/checker/test_lexicon.py     # the lexicon and the standard's pin
```

`test_lexicon.py` runs its last two checks (the thesaurus rewrites, and the standard
against its own checker) only where the local lexicon is built, and prints SKIP for each
where it is not. A SKIP is not a pass. Every property of `ste_doc.py` that the tests
claim was perturbation-checked on 2026-11-02: removing it fails a test.

## What was copied, and what changed

Copied from toddwbucy/WeaverTrain at `ab63a918b569e5b8673dc2e78e66575ee9df1bd7` (branch
`ste/500-run`):

| Here | There | Changes |
|---|---|---|
| `ste_check.py` | `scripts/ste_check.py` | Paths: `ste/` is `lexicon/`, `var/ste/` is `var/`, both under `ste100/`. Loads `lexicon/terms.toml` as technical nouns. The curly apostrophe written as `\u2019` (ASCII only). |
| `ste_extract.py` | `scripts/ste_extract.py` | The same path changes. Reads `WEAVERTOOLS_STE100_SOURCE`, with `WEAVER_TRAIN_STE100_SOURCE` as a fallback, and records which in its output. |
| `ste_mask.py` | `scripts/ste_convert.py`, lines 47-180 | Only the masking functions, without the converter. `mask(text, prose_parens=True)` keeps a parenthesis that follows a space as prose, so a parenthetical sentence is checked and not hidden. The default is the copied behaviour. The curly apostrophe as above. |
| `test_ste_check.py` | `tests/test_ste_check.py` | A plain script in place of pytest. The curly apostrophe as above. |
| `test_lexicon.py` | `tests/test_ste_thesaurus.py` | The thesaurus check, as a plain script, beside new checks on the lexicon. |

New here: `ste_doc.py` (the document driver) and `test_ste_doc.py`.

**Why a copy and not a pointer.** WeaverTrain's checker reads one segment of training
data against a source text. A document needs a driver that cuts it into sentences,
masks it without a source, checks the header line and reports by line, and no such
driver exists in WeaverTrain to point at. The driver needs the checker beside it at a
fixed version, and the checker reads its lexicon by path, so the standard, the lexicon
and the checker are kept in one place and versioned together. The code is stdlib Python,
which fits this repository. The cost is two copies of `ste_check.py`, the same code at
`ab63a918` apart from paths. By the operator's ruling of 2026-11-02 they stay
independent: WeaverTrain keeps its own checker, with no code dependency on this
repository, and shares only the lexicon, pinned by sha256. A change to this checker
lands by its own issue and PR here (the open faults are toddwbucy/WeaverTools#19), and
whether WeaverTrain takes it is WeaverTrain's call.

## Known limits

- **Open faults, toddwbucy/WeaverTools#19.** An unknown noun ending in "-ing" fails as a
  verb form ("meaning", "ruling"). A noun after a modifier that is not an article or a
  preposition reads as a verb ("a second attempt"). The extraction misses some approved
  entries (17 of 748 headwords in a rough count, `immediately` among them), which are
  then reported unknown.
- **Quoted text is masked.** The conversion masks double-quoted strings, so a quotation
  in a document is not checked. Use quotation marks for words named as words, not for
  prose.
- **"I" passes.** The agent exception is still a candidate (the standard, section 11.2).
- **Senses are not checked.** The checker knows whether a word is approved, not whether
  it is used in its approved sense. One word, one meaning in a document is the writer's
  and the reviewer's to hold.
