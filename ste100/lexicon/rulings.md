# Rulings on words

The operator's rulings on words that the lexicon carries, newest first. The three
entries below are copied unchanged from WeaverTrain's `docs/decisions.md` at
`ab63a918b569e5b8673dc2e78e66575ee9df1bd7` (branch `ste/500-run`), which holds them
among that repository's other decisions. Paths in them are WeaverTrain's: `ste/` there
is `ste100/lexicon/` here, and `scripts/ste_check.py` there is `ste100/checker/` here.

A ruling made after the baseline (2026-11-02) is added here by its own issue and PR,
with the lexicon change it makes (README.md, change control).

## 2026-11-01: "call" is not admitted; "should" has three senses

- **call** is not admitted as a technical verb. The mapping is fixed: call -> use
  ("the function calls [[C1]]" -> "the function uses [[C1]]"). It is in the rewriter
  rules, the thesaurus and `ste/fixed-mappings.toml`, and the checker's repair note
  gives "use" instead of the standard's alternatives.
- **should** has three senses:
  - advice -> "I recommend that" ("You should add a regression test." -> "I recommend
    that you add a test for the regression.");
  - expectation -> "I think that" ("This should fix the error." -> "I think that this
    change will repair the error.");
  - a requirement or a correct value ("X should be Y", "no error should be raised") ->
    "must" ("No error should be raised." -> "The code must not give an error.").
- **Still owed:** the four hedge forms ("possibly", "I think that", "this is an
  indication that", "examine if" / "find if") and the "I" exception stay candidates.
  They are used in the 500-segment run and listed as owed (`ste/usage-notes.md`).

## 2026-11-01: logical operators and SQL keywords in capitals are technical names

In prose, a logical operator or an SQL keyword is written in capitals and is a technical
name: AND, OR, NOT, XOR, TRUE, FALSE, NULL; SELECT, JOIN, DROP TABLE and the other
keywords in `ste/technical-names.toml`. The lowercase words keep their approved STE100
meanings, so the 2026-10-31 boolean ruling stands for the conjunctions: "If the file is
open and the lock is free" is plain prose, while "The filter uses AND" names the
operator. A technical name is not a verb ("Use SELECT to get the rows", not "select the
rows"), so the SQL verb rulings are needed only where text uses those words as verbs.
GRANT and REVOKE wait on the permission-scope ruling.

Capitals mean a technical name only for a listed word, or for a word that is not in
the dictionary (an acronym such as PPM or AWS), or for capitals copied from the source
text. Any other dictionary word in capitals fails the checker: the pilot passed "All
tests GO" and "OPERATE" because every all-capitals word counted as a technical name.

## 2026-10-31: STE100 technical verbs, SQL verbs and boolean names (candidate)

Rulings for the candidate scheme file `ste/ste100-scheme.candidate.json`
(`ste100-scheme/1` shape). Every entry stays candidate and unratified. The base is
WeaverAgent's verb dictionary v0.6, which this repository reads and does not modify.

Technical verbs:

- **run**: provisional. Sense: to execute a program, a test, or a command. Condition:
  execute must not already be an approved STE100 verb. If it is, stop and report, and do
  not admit run.
- **commit**: admitted. One sense: to make a set of changes permanent in a repository or
  a database. It covers both Git and SQL, since Git is a store of changes.
- **merge**: to combine branches, or to combine changes into a store. Whether SQL MERGE
  is the same act under this sense is to be checked and reported.
- **push**: to send changes to a remote repository.

SQL verbs, candidate set: select, insert, update, delete, join, create, drop, alter,
rollback. Add grant and revoke only if permission work is in scope; ask first.

Boolean terms get no special handling. and, or, not keep their approved STE100
meanings in prose. Inside code they are operators and are not checked. Deferred to the
conditions grammar under O02-1.

Synonym check before anything is admitted. For each new verb, search the STE100
approved-word list (the operator's local copy, path in `.env` as
`WEAVER_TRAIN_STE100_SOURCE`, nothing committed) and the existing dictionary for a word
with the same meaning. Report each verb as one of:

1. no overlap: admit;
2. an exact approved match exists: use the approved word, do not admit;
3. a near-synonym or shared meaning: list it, and the operator rules.

Also flag any new verb that is itself an approved everyday word, and say whether its
approved meaning fits the SQL sense. Flag rollback against git revert.

Owed by the operator, not decided here: the sentence limit (20 for all emissions, or 20
for instructions and 25 for descriptions).

