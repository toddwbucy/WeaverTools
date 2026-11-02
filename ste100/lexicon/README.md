# The WeaverTools STE100 lexicon

The program's own additions to ASD-STE100, held as data: the words, senses and rulings
that the standard does not carry and the program's text needs. It is one artifact for
every judge of STE100 text in the suite. Each consumer pins it here, by commit or by its
sha256, and no consumer keeps or edits a copy of its own. The consumers are coupled by
this artifact, not by each other.

The standard's own dictionary is not here and is never committed (ASD reserves
reproduction and publication). Approved words come from the operator's local copy,
whose path the checker reads from `WEAVER_TRAIN_STE100_SOURCE`, and whose sha256 is
pinned in `ste100-scheme.candidate.json` under `reference`.

## Files

| File | What it holds |
|---|---|
| `thesaurus.toml` | How to write the software verbs that STE100 does not approve, sense by sense, each with a rewrite the checker passes. |
| `technical-verbs.toml` | The technical verbs admitted (rule 1.12), each with its one sense in our own words. |
| `technical-nouns.toml` | Technical nouns of the software field (rule 1.8). |
| `technical-names.toml` | Words written in capitals that are technical names: logical operators and SQL keywords. |
| `fixed-mappings.toml` | Operator rulings that fix the replacement for an unapproved verb (`call` -> `use`). |
| `terms.toml` | The program's own technical nouns, each with one sense, why it is here and the issue that added it, and the words that wait on a ruling (`status = "unresolved"`). |
| `usage-notes.md` | How to write common software verbs and the hedge forms; candidates. |
| `rulings.md` | The operator's rulings on words, with dates and reasons. |
| `ste100-scheme.candidate.json` | The verb list in the `ste100-scheme/1` shape, with the sha256 of the operator's copy of the standard. |

Everything except `terms.toml` and `rulings.md` was copied unchanged from
toddwbucy/WeaverTrain at `ab63a918b569e5b8673dc2e78e66575ee9df1bd7` (branch
`ste/500-run`, the head of WeaverTrain PR #4 on 2026-11-02), from its `ste/` directory.
That commit, not WeaverTrain's `main` (`f90d9d7c`), was the source because only it holds
`fixed-mappings.toml` and the 2026-11-01 rulings on "call" and "should", and because
toddwbucy/WeaverAgent#433 already names it as the thesaurus of record. `rulings.md`
copies the three word rulings from WeaverTrain's `docs/decisions.md` at the same commit,
unchanged. `terms.toml` is new.

Not copied: `rewrite-prompt.md` and `rewrite-examples.md`. They are WeaverTrain's
rewriter, a consumer of this lexicon, not part of it.

## Consumers

Each pins this directory by commit or by the sha256 that `ste100/checker/ste_doc.py`
prints (every file here except this README, in name order, each as name, NUL, bytes,
NUL).

- **WeaverTrain**: its checker (`scripts/ste_check.py`) and rewrite prompt. Today they
  read WeaverTrain's own `ste/`; they repoint here when this lands (follow-up, not in
  toddwbucy/WeaverTools#17).
- **WeaverAgent**: the state store, under ruling B (toddwbucy/WeaverAgent#433, one verb
  artifact for every judge).
- **The writing standard**: `ste100/writing-standard.md`, whose header line names the
  sha256 it was checked against. `ste100/checker/test_lexicon.py` fails when that sha256
  is not the lexicon's as it stands.

## Change control

The lexicon as copied on 2026-11-02 is the baseline. Every change after it needs its own
issue and its own pull request, which state the change and its reason. No entry changes
silently, and no consumer edits its own copy.

A pull request that changes any file here also re-runs the checker over
`ste100/writing-standard.md`, puts the new sha256 in its header line, and records the
checker result in its body (the writing standard, section 8).

The status words inside the copied files ("candidate", "unratified") are WeaverTrain's
record of what the operator has ruled. Change control here does not ratify anything: an
entry stays a candidate until the operator rules on it, and that ruling is a change like
any other.
