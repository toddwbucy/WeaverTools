# WeaverTools STE100 writing standard

STE100: WeaverTools STE100 writing standard 0.1, lexicon sha256:8cb83e623f84519f8f501f67bbe11de6e867f68c047251a4f992c90e5fc3b580

Version 0.1, 2026-11-02. Issue: `toddwbucy/WeaverTools#17`.

## 1. Purpose

This standard tells you how to write the PRDs, Specs and workflow documents of WeaverTools.

Each word in a document must have one meaning. On 2026-11-02, an inspection of the documents found words with 2 meanings. When the second meaning came into a document, no person saw it. One of these words was restatement. On 2026-11-02, the operator gave it one meaning: the repair. `rulings.md` records this ruling.

ASD-STE100 has a rule: one word, one meaning. WeaverTools uses STE100 for the output of the agent. This standard applies STE100 to the documents of WeaverTools too.

## 2. Scope

Write each new PRD, Spec and workflow document in STE100. This applies in each WeaverTools repository.

Papers are not in the scope of this standard. A paper keeps the voice of the person who writes it.

## 3. Forward only

Do not write a document again in STE100 only because this standard starts.

A document changes to STE100 as one unit, at its next revision. Do not change it before that time.

A change to a document that is not in STE100 uses the language of that document. It uses that language until the document changes to STE100. Thus each document is fully in STE100 or not in STE100.

## 4. The header line

Each document that agrees with this standard has one header line. Put it on the line after the name of the document. Write the line as in this example:

```text
STE100: WeaverTools STE100 writing standard <version>, lexicon sha256:<64 hex>
```

The line gives 3 items: the standard, the version of the standard, and the lexicon that the checker used.

The `sha256` of the lexicon identifies it, not a commit. A document in this repository cannot give the commit that contains it. The `sha256` does not have this problem. The checker calculates the `sha256` and compares it with the header line.

This rule applies to the documents of each WeaverTools repository. A document can also give the WeaverTools commit of the lexicon. Add it at the end of the header line, as in this example:

```text
STE100: WeaverTools STE100 writing standard <version>, lexicon sha256:<64 hex>, WeaverTools commit <hex>
```

The checker does not examine the commit. It examines only the `sha256`.

To find the commit of a lexicon, find the commit where `ste100/lexicon/` has that `sha256`.

## 5. The lexicon

The standard has a dictionary of approved words. WeaverTools adds words and rulings to that dictionary. These words and rulings are the lexicon.

The lexicon is in `ste100/lexicon/` in this repository. It is data. It contains these files:

- `thesaurus.toml`: how to write the verbs that are not approved, sense by sense.
- `technical-verbs.toml`: the technical verbs, each with its one sense.
- `technical-nouns.toml`: the technical nouns of the software field.
- `technical-names.toml`: the words in capitals that are technical names.
- `fixed-mappings.toml`: the fixed replacement for a verb, by ruling.
- `terms.toml`: the technical nouns of WeaverTools, each with its one sense.
- `usage-notes.md`: notes on how to write verbs that are frequent in software text.
- `rulings.md`: the rulings on words, with their dates and causes.
- `ste100-scheme.candidate.json`: the verb list, in the `ste100-scheme/1` format.

Each consumer of the lexicon pins it by commit or by `sha256`. These are the consumers:

- The checker of WeaverTrain, and its file `rewrite-prompt.md`. WeaverTrain keeps its checker and pins only the lexicon, by `sha256`.
- The `weaver-state` store of WeaverAgent. The ruling `B` in `toddwbucy/WeaverAgent#433` puts the thesaurus and the verb list there.
- This standard.

Thus the lexicon connects the consumers. The consumers do not connect to each other.

## 6. Change control

The lexicon that this repository received on 2026-11-02 is the baseline. The lexicon will change as the work continues.

Each change after the baseline must have an issue and a pull request for that change only. The issue and the pull request record the change and its cause.

A change can start in WeaverTrain. It must also come to this repository with an issue and a pull request. WeaverTrain does not change its `ste/` directory without this repository.

No entry changes without a record. No consumer changes its copy of the lexicon.

Do not add a word to `terms.toml` only because the checker gives a failure for it. Record the fault of the checker in `toddwbucy/WeaverTools#19`.

## 7. Copyright

ASD-STE100 has a copyright. Do not commit the dictionary of the standard to a repository.

The approved words come from the local copy of the standard that the operator has. The checker reads the path of that copy from `WEAVERTOOLS_STE100_SOURCE`. If that variable has no value, the checker reads `WEAVER_TRAIN_STE100_SOURCE`. The output of the checker tells which variable it used. This repository contains only the words and the rulings that WeaverTools adds. It identifies the standard by its issue: ASD-STE100, Issue 9.

## 8. Measurement

A document agrees with this standard only if the checker shows it. Measure each document.

The pull request of a document that agrees with this standard gives the result of the checker. The result gives these items:

- The number of sentences that the checker examined.
- The number of sentences that have no failure.
- Each failure, with the rule that it breaks.

The checker must have the local copy of the standard. Thus the result is a record in the pull request. It is not a CI gate.

To get the result, do these steps:

1. Set `WEAVERTOOLS_STE100_SOURCE` to the path of the local copy of the standard.
2. Do `python3 ste100/checker/ste_extract.py` one time on each computer.
3. Do `python3 ste100/checker/ste_doc.py <document>` for each document.
4. Copy the output into the pull request.

## 9. Rules for the text

### 9.1 Sentences

An instruction has a maximum of 20 words. A description has a maximum of 25 words. The checker of WeaverTrain uses these limits.

An instruction starts with a verb, or with a condition and then a verb.

### 9.2 Technical names

These items are technical names:

- The names of clauses, for example `CLASS-9` and `H1`.
- Identifiers.
- Paths.
- All text in backticks.

The checker replaces each technical name with a placeholder, as the conversion does with code. It does not examine the words in a technical name. If the checker does not find a technical name, put it in backticks.

### 9.3 One word, one meaning

Use each word with one meaning in a document. Use the same word for the same item each time.

If a technical noun is necessary and the lexicon does not have it, add it to `terms.toml`. Do this in the same pull request. Give its sense and its cause.

### 9.4 The `Working Rules`

The rules of the `Working Rules` continue to apply:

- Use ASCII only.
- Do not use semicolons.
- Do not use a dash as a mark between words. If a dash is necessary, use " - ".
- Do not use these words: `genuinely`, `honestly`, `actually`. The checker gives a failure for each of them.

## 10. The checker

The checker is in `ste100/checker/` in this repository. It is a copy of the checker of WeaverTrain at commit `ab63a918`. We copied it for these causes:

- The standard and its checker are in one location, with the lexicon that they use.
- The checker of WeaverTrain examines segments of training data. A document is not a segment. Thus the checker must have a driver that divides a document into sentences. `ste_doc.py` is that driver, and it is new.
- The checker uses only the standard library of `Python`. Thus it agrees with the other code in this repository.

WeaverTrain keeps its checker, and the 2 checkers use the same lexicon. A change to one checker does not change the other checker. The changes to the copy are in `ste100/checker/README.md`.

## 11. Open items

These items are open. The standard keeps them until the operator gives a ruling.

### 11.1 The "I" exception and the hedge forms

The checker accepts "I" for the agent. The hedge forms tell how sure a statement is. These forms are candidates: "possibly", "I think that", "this is an indication that", and "examine if" or "find if".

The candidates are in `rulings.md` and `usage-notes.md`. The operator has not given a ruling on them.

### 11.2 PRDs

A PRD gives more arguments than instructions. The limit of 25 words can make an argument hard to write.

We will measure the effect of this limit on the first PRD that we write to this standard.

### 11.3 How to know that the standard works

Before the standard starts, we must tell which result shows that it works. These are the candidates:

- Count the problems with words that each inspection of a pull request finds, before and after the standard.
- Do a transmission test of one Spec in STE100 against its original text.

The operator selects the measurement before the standard starts.
