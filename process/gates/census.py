#!/usr/bin/env python3
"""The census: what the other enforcement devices structurally cannot see.

Clippy, fmt, the tests and the compile pins each verify an artifact against
itself. **None compares a claim in a document against a fact in code**, which
is the drift that stood three weeks in `weaver-trace-Spec` and that issue #558
records thirty-three instances of.

**The rule is that no defect is new, not that every number is zero.** A gate
nobody can pass is a gate everyone learns to ignore, which the enforcement
section already says of clippy, so the backlog is a baseline rather than a
failure. The comparison is by identity and not by count: swapping one defect
for another of the same kind leaves the count still and is caught anyway.

    python3 process/gates/census.py            # against the baseline
    python3 process/gates/census.py --update   # take a new reading
    python3 process/gates/test_census.py       # the fixture, per defect

**Every bug of this script's own is recorded here**, because each printed a
confident wrong number and a gate that does that is worse than no gate. Two
review passes found them; the second found as many as the first:

- A graph block declares several nodes, one stanza each. Reading the first
  `node:` per block called seventy sound citations dangling.
- A section 9 is the enforcement table in `weaver-web-Spec` and the failure
  vocabulary in `weaver-spu-Spec`. The table is found by its own header.
- `\\w` does not match a hyphen, so `compile-pin` and `compile-fail` - two of
  the Document Format's five tags - read as untagged, and fifty four of them
  were published in `CLAUDE.md` as a defect count.
- Declining to read a crate's `tests/` stopped collecting the citations there
  and moved fifteen perturbations into the uncited column. **Citing and owing
  a header are different questions** and one walk answered both.
- `members` is a substring of `default-members`, so an unanchored search read
  the wrong list; a glob expanded to nothing and a missing directory walked
  nothing, both driving the counts to a clean zero that reads as improvement.
- Splitting stanzas on a blank line dropped every record written back to back
  against the one before it, which is the first bug in this list wearing
  different clothes.
- Measuring citations against assertions alone called a sound header dangling:
  the corpus declares vocabulary, document, crate, term and axiom nodes too,
  and the Document Format names one a source file may cite.
- Comparing by set called a defect's second occurrence familiar while the
  count visibly rose, which the stated rule - no defect is new - forbids.
- The obligation was scoped by a `/src/` directory, which the Document Format
  names as a defect in a count rather than a tightening of it, in the sentence
  that also names this exact mis-application.
"""

import glob
import json
import os
import re
import sys
from collections import Counter

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
HERE = os.path.dirname(os.path.abspath(__file__))
BASELINE = os.path.join(HERE, "census-baseline.json")

# `WeaverTools-Document-Format` fixes this vocabulary. A tag outside it is a
# finding rather than a silent miss.
TAGS = {"compile-pin", "compile-fail", "perturbation", "manifest", "review"}

GRAPH = re.compile(r"```graph(.*?)```", re.S)
NODE_LINE = re.compile(r"^node: (.*)$", re.M)
NODE_OK = re.compile(r"^[a-z0-9-]+$")
KIND = re.compile(r"^kind: ([\w-]+)$", re.M)
TAG = re.compile(r"^tag: ([\w-]+)$", re.M)
HEADER_CITE = re.compile(r"^//! conforms: ([a-z0-9-]+)\s*$", re.M)
# **Anchored to the end of the word.** Unanchored, `conforms: weaver_types-x`
# captures `weaver` and the gate names an identifier that is in no file, so a
# reader grepping for the offender finds nothing.
ANY_CITE = re.compile(r"conforms: (\S+)")

# A build script is cargo's unit and not the crate's, and conforms to nothing.
# `archive/` is not a workspace member and is never compiled, so counting
# either gives the metric a floor nobody can reach - the shape that teaches
# people to stop reading a number.
NO_HEADER_OWED = ("build.rs",)


def read(path):
    with open(path, encoding="utf-8") as fh:
        return fh.read()


def members():
    """The workspace's members, or a refusal.

    **`members` is anchored** because `default-members` contains it as a
    substring and an unanchored search reads the wrong list. A glob is
    expanded and every path is checked, since a member directory that is not
    there walks nothing and drives the counts to a clean zero - the failure
    the enforcement section calls the most expensive line it holds.
    """
    text = read(os.path.join(ROOT, "Cargo.toml"))
    block = re.search(r"^members\s*=\s*\[(.*?)\]", text, re.S | re.M)
    if not block:
        raise SystemExit("census: no workspace members in Cargo.toml")
    found = []
    for raw in block.group(1).split(","):
        entry = raw.strip()
        if not entry or entry.startswith("#"):
            continue
        entry = entry.strip('"')
        matches = sorted(glob.glob(os.path.join(ROOT, entry))) if "*" in entry else [
            os.path.join(ROOT, entry)
        ]
        matches = [m for m in matches if os.path.isdir(m)]
        if not matches:
            raise SystemExit(f"census: workspace member is not a directory: {entry}")
        found.extend(matches)
    return found


def docs():
    for base, dirs, files in os.walk(os.path.join(ROOT, "docs")):
        # **Pruned as `sources` prunes it.** A frozen copy of a Spec declares
        # every node the live one does, so ingesting both files each id under
        # duplicates, files every perturbation it declares under uncited, and
        # lets the last declaration read win.
        dirs[:] = [d for d in dirs if d not in (".git", "archive")]
        for f in sorted(files):
            if f.endswith(".md"):
                yield os.path.join(base, f)


def sources():
    """Every `.rs` a workspace member owns, and whether it owes a header.

    **The obligation follows the unit and never a directory.**
    `WeaverTools-Document-Format` states it and names this exact
    mis-application: "a conformance count is over every tracked unit carrying
    a header, and a directory is never the rule ... a count taken over `src`
    alone reported the crates through 2026-08-16". The first form of this
    gate scoped by `/src/` and inherited the defect it was told about.
    """
    for member in members():
        for base, dirs, files in os.walk(member):
            dirs[:] = [d for d in dirs if d not in ("target", "archive")]
            for f in sorted(files):
                if not f.endswith(".rs"):
                    continue
                path = os.path.join(base, f)
                owes = f not in NO_HEADER_OWED
                yield path, owes


def enforcement_table(text):
    """Rows of a document's enforcement table, header excluded, or `None`."""
    lines = text.splitlines()
    for i, line in enumerate(lines):
        if re.match(r"^\|\s*claim\s*\|\s*instrument\s*\|", line, re.I):
            rows = 0
            for row in lines[i + 2:]:
                if not row.startswith("|"):
                    break
                rows += 1
            return rows
    return None


def take():
    """One reading. Every value is a sorted list of the offenders themselves,
    so the comparison is by identity rather than by count."""
    # **`declared` holds every node whatever its kind.** The corpus declares
    # vocabulary, document, crate, term and axiom nodes beside its assertions,
    # and the Document Format names `tool-trait` as one a source file may
    # cite. Measuring citations against assertions alone reports a sound
    # header as dangling.
    nodes, declared, duplicates, malformed, odd = {}, set(), [], [], []
    texts = {}

    for path in docs():
        rel = os.path.relpath(path, ROOT)
        text = read(path)
        texts[rel] = text
        for block in GRAPH.findall(text):
            # **Split on the record's own keyword, not on a blank line.** Two
            # records written back to back are one stanza to a blank-line
            # splitter, which drops all but the first - the same class of
            # miss as reading one `node:` per block, and invisible for the
            # same reason.
            for stanza in re.split(r"(?=^node: )", block, flags=re.M):
                line = NODE_LINE.search(stanza)
                if not line:
                    continue
                name = line.group(1).strip()
                if not NODE_OK.match(name):
                    malformed.append(f"{rel}: {name}")
                    declared.add(name)
                    continue
                kind = KIND.search(stanza)
                declared.add(name)
                if not kind or kind.group(1) != "assertion":
                    continue
                tag = TAG.search(stanza)
                tag = tag.group(1) if tag else None
                if tag is not None and tag not in TAGS:
                    odd.append(f"{name} ({tag})")
                if name in nodes:
                    duplicates.append(f"{name}: {nodes[name][0]} and {rel}")
                nodes[name] = (rel, tag)

    cited, headerless = set(), []
    for path, owes in sources():
        text = read(path)
        for raw in ANY_CITE.findall(text):
            if NODE_OK.match(raw):
                cited.add(raw)
            else:
                malformed.append(f"{os.path.relpath(path, ROOT)}: conforms: {raw}")
        if owes and not HEADER_CITE.search(text):
            headerless.append(os.path.relpath(path, ROOT))

    # **The documents without a table are the metric, not the ones with.** A
    # count of mismatches over the one document that has a table reads as
    # corpus-wide assurance and is not: rename that header and the mismatch
    # count stays zero while nothing is examined at all. Listed this way, a
    # document losing its table is a new entry and fails.
    mismatch, tableless = [], []
    for rel in sorted({r for r, _ in nodes.values()}):
        rows = enforcement_table(texts[rel])
        count = sum(1 for r, _ in nodes.values() if r == rel)
        if rows is None:
            tableless.append(f"{rel} ({count} assertions)")
            continue
        if rows != count:
            mismatch.append(f"{rel} ({count} nodes, {rows} rows)")

    return {
        "dangling_citations": sorted(i for i in cited if i not in declared),
        "untagged_assertions": sorted(n for n, (_, t) in nodes.items() if t is None),
        "unknown_tags": sorted(odd),
        "uncited_perturbations": sorted(
            n for n, (_, t) in nodes.items() if t == "perturbation" and n not in cited
        ),
        "duplicate_node_ids": sorted(duplicates),
        "malformed_node_ids": sorted(malformed),
        "enforcement_table_mismatch": sorted(mismatch),
        "documents_without_an_enforcement_table": sorted(tableless),
        "sources_without_a_header": sorted(headerless),
    }


def main():
    reading = take()

    if "--update" in sys.argv:
        # **It says what moved.** The rule is that moving the baseline is a
        # sentence in the act's commit message, and an operator who has to
        # re-run the gate first and read the difference by eye is doing the
        # quiet re-reading the rule forbids.
        old = {}
        if os.path.exists(BASELINE):
            with open(BASELINE, encoding="utf-8") as fh:
                old = json.load(fh)
        for key in sorted(reading):
            gained = [x for x in reading[key] if x not in old.get(key, [])]
            lost = [x for x in old.get(key, []) if x not in reading[key]]
            if gained or lost:
                print(f"{key}: {len(old.get(key, []))} -> {len(reading[key])}")
                for x in gained:
                    print(f"  + {x}")
                for x in lost:
                    print(f"  - {x}")
        with open(BASELINE, "w", encoding="utf-8") as fh:
            json.dump(reading, fh, indent=2, sort_keys=True)
            fh.write("\n")
        print("baseline written:", json.dumps({k: len(v) for k, v in reading.items()}))
        return 0

    if not os.path.exists(BASELINE):
        print("no baseline; run --update for the first reading", file=sys.stderr)
        return 2

    with open(BASELINE, encoding="utf-8") as fh:
        before = json.load(fh)

    appeared_anywhere = False
    for key in sorted(reading):
        # **A multiset and not a set.** Two of these lists can hold the same
        # string twice - one id declared a third time, one malformed id
        # written twice - and a set comparison calls the second occurrence
        # familiar while the count visibly rises.
        remaining = Counter(before.get(key, []))
        now = reading[key]
        appeared = []
        for item in now:
            if remaining[item]:
                remaining[item] -= 1
            else:
                appeared.append(item)
        was = list(before.get(key, []))
        mark = "NEW" if appeared else ("down" if len(now) < len(was) else "")
        print(f"{mark:>4}  {key:<28} {len(now):>4}   (baseline {len(was)})")
        shown = appeared[:10]
        for item in shown:
            print(f"        + {item}")
        if len(appeared) > len(shown):
            print(f"        + and {len(appeared) - len(shown)} more")
        if appeared:
            appeared_anywhere = True

    if appeared_anywhere:
        print(
            "\nSomething is new. Cite it, mark its enforcement row owed, or say in\n"
            "the act why the baseline moves, then run with --update.",
            file=sys.stderr,
        )
    return 1 if appeared_anywhere else 0


if __name__ == "__main__":
    sys.exit(main())
