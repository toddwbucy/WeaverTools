#!/usr/bin/env python3
"""The census: five counts the other gates cannot see.

Clippy, fmt and the test suite each verify an artifact against itself. None
of them compares a claim in a document against a fact in code, which is the
drift that went three weeks unnoticed in `weaver-trace-Spec` and that issue
#558 records twenty-eight instances of.

**The rule is that no number goes up, not that every number is zero.** A gate
nobody can pass is a gate everyone learns to ignore, which the enforcement
section already says of clippy. The baseline beside this file is the reading
taken when the gate landed; `--update` moves it, and moving it upward is a
thing an act says out loud in its commit message.

Run from the repository root:

    python3 process/gates/census.py            # compare against the baseline
    python3 process/gates/census.py --update   # take a new reading
"""

import json
import os
import re
import sys
from collections import Counter

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
BASELINE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "census-baseline.json")

GRAPH = re.compile(r"```graph(.*?)```", re.S)
NODE = re.compile(r"^node: ([a-z0-9-]+)$", re.M)
KIND = re.compile(r"^kind: (\w+)$", re.M)
TAG = re.compile(r"^tag: (\w+)$", re.M)
CITE = re.compile(r"conforms: ([a-z0-9-]+)")


def walk(top, suffix):
    for base, dirs, files in os.walk(os.path.join(ROOT, top)):
        dirs[:] = [d for d in dirs if d not in ("target", ".git")]
        for f in files:
            if f.endswith(suffix):
                yield os.path.join(base, f)


def read(path):
    with open(path, encoding="utf-8") as fh:
        return fh.read()


def assertions():
    """Every `kind: assertion` node in docs, with its tag and its document.

    **A graph block declares several nodes**, one stanza each, separated by a
    blank line - so a reader of the block that takes its first `node:` reads
    one of them and misses the rest. The first form of this script did that
    and reported seventy citations dangling that resolve perfectly well.
    """
    out = {}
    for path in walk("docs", ".md"):
        for block in GRAPH.findall(read(path)):
            for stanza in re.split(r"\n\s*\n", block):
                node, kind = NODE.search(stanza), KIND.search(stanza)
                if not node or not kind or kind.group(1) != "assertion":
                    continue
                tag = TAG.search(stanza)
                out[node.group(1)] = (path, tag.group(1) if tag else None)
    return out


def citations():
    """Every `conforms:` identifier in code, and the files carrying none."""
    cited, bare = Counter(), []
    for path in walk("crates", ".rs"):
        text = read(path)
        ids = CITE.findall(text)
        if ids:
            cited.update(set(ids))
        else:
            bare.append(os.path.relpath(path, ROOT))
    return cited, bare


def enforcement_table(path):
    """Rows of a document's enforcement table, header excluded.

    `None` where the document has no such table, which is most of them.
    **The table is found by its own header and not by a section number**: a
    section 9 is the enforcement table in `weaver-web-Spec` and the failure
    vocabulary in `weaver-spu-Spec`, so a check keyed on the number reports
    four documents in disagreement with themselves and means nothing by it.
    """
    lines = read(path).splitlines()
    for i, line in enumerate(lines):
        if re.match(r"^\|\s*claim\s*\|\s*instrument\s*\|", line):
            rows = 0
            for row in lines[i + 2:]:
                if not row.startswith("|"):
                    break
                rows += 1
            return rows
    return None


def take():
    nodes = assertions()
    cited, bare = citations()

    dangling = sorted(i for i in cited if i not in nodes)
    untagged = sorted(n for n, (_, t) in nodes.items() if t is None)
    uncited_perturbations = sorted(
        n for n, (_, t) in nodes.items() if t == "perturbation" and n not in cited
    )

    # An enforcement table names one instrument per assertion its document
    # makes, so the two counts move together or one of them was not updated.
    mismatched = []
    per_doc = Counter(doc for doc, _ in nodes.values())
    for path, count in sorted(per_doc.items()):
        if not path.endswith("-Spec.md"):
            continue
        rows = enforcement_table(path)
        if rows is not None and rows != count:
            mismatched.append(f"{os.path.relpath(path, ROOT)} ({count} nodes, {rows} rows)")

    return {
        "dangling_citations": dangling,
        "untagged_assertions": untagged,
        "uncited_perturbations": uncited_perturbations,
        "enforcement_table_mismatch": mismatched,
        "sources_without_a_citation": sorted(bare),
    }


def main():
    reading = take()
    counts = {k: len(v) for k, v in reading.items()}

    if "--update" in sys.argv:
        with open(BASELINE, "w", encoding="utf-8") as fh:
            json.dump(reading, fh, indent=2, sort_keys=True)
            fh.write("\n")
        print("baseline written:", json.dumps(counts))
        return 0

    if not os.path.exists(BASELINE):
        print("no baseline; run with --update to take the first reading", file=sys.stderr)
        return 2

    with open(BASELINE, encoding="utf-8") as fh:
        was = {k: len(v) for k, v in json.load(fh).items()}

    worse = False
    for key in sorted(counts):
        before, now = was.get(key, 0), counts[key]
        mark = "  "
        if now > before:
            mark, worse = "UP", True
        elif now < before:
            mark = "down"
        print(f"{mark:>4}  {key:<28} {now:>4}   (baseline {before})")
        if now > before:
            new = [x for x in reading[key] if x not in set(json.load(open(BASELINE))[key])]
            for item in new[:10]:
                print(f"        + {item}")

    if worse:
        print("\nA number went up. Cite it, mark its section 9 row owed, or say", file=sys.stderr)
        print("in the act why the baseline moves, then run with --update.", file=sys.stderr)
    return 1 if worse else 0


if __name__ == "__main__":
    sys.exit(main())
