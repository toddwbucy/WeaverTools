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

**Four bugs of this script's own are recorded here**, because each printed a
confident wrong number and a gate that does that is worse than no gate:

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
"""

import json
import os
import re
import sys

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
ANY_CITE = re.compile(r"conforms: ([a-z0-9-]+)")

# A build script conforms to nothing. `archive/` is not a workspace member and
# is never compiled. Counting either gives the metric a floor nobody can
# reach, which is the shape that teaches people to stop reading a number.
NO_HEADER_OWED = ("build.rs",)


def read(path):
    with open(path, encoding="utf-8") as fh:
        return fh.read()


def members():
    block = re.search(r"members\s*=\s*\[(.*?)\]", read(os.path.join(ROOT, "Cargo.toml")), re.S)
    return [m.strip().strip('"') for m in block.group(1).split(",") if m.strip()]


def docs():
    for base, dirs, files in os.walk(os.path.join(ROOT, "docs")):
        dirs[:] = [d for d in dirs if d != ".git"]
        for f in sorted(files):
            if f.endswith(".md"):
                yield os.path.join(base, f)


def sources():
    """Every `.rs` a workspace member owns, and whether it owes a header.

    **A test directory cites and does not owe.** An integration test is an
    instrument, so its citations count; phase three's rule that every source
    file carries a header is about the crate's own `src/`.
    """
    for member in members():
        for base, dirs, files in os.walk(os.path.join(ROOT, member)):
            dirs[:] = [d for d in dirs if d not in ("target", "archive")]
            for f in sorted(files):
                if not f.endswith(".rs"):
                    continue
                path = os.path.join(base, f)
                owes = f"{os.sep}src{os.sep}" in path and f not in NO_HEADER_OWED
                yield path, owes


def enforcement_table(path):
    """Rows of a document's enforcement table, header excluded, or `None`."""
    lines = read(path).splitlines()
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
    nodes, duplicates, malformed, odd = {}, [], [], []

    for path in docs():
        rel = os.path.relpath(path, ROOT)
        for block in GRAPH.findall(read(path)):
            for stanza in re.split(r"\n\s*\n", block):
                line = NODE_LINE.search(stanza)
                if not line:
                    continue
                name = line.group(1).strip()
                if not NODE_OK.match(name):
                    malformed.append(f"{rel}: {name}")
                    continue
                kind = KIND.search(stanza)
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
        cited |= set(ANY_CITE.findall(text))
        if owes and not HEADER_CITE.search(text):
            headerless.append(os.path.relpath(path, ROOT))

    # **The count of tables examined is reported**, so a zero here cannot be
    # read as corpus-wide assurance: most documents carry no such table.
    mismatch, examined = [], 0
    for rel in sorted({r for r, _ in nodes.values()}):
        rows = enforcement_table(os.path.join(ROOT, rel))
        if rows is None:
            continue
        examined += 1
        count = sum(1 for r, _ in nodes.values() if r == rel)
        if rows != count:
            mismatch.append(f"{rel} ({count} nodes, {rows} rows)")

    return {
        "dangling_citations": sorted(i for i in cited if i not in nodes),
        "untagged_assertions": sorted(n for n, (_, t) in nodes.items() if t is None),
        "unknown_tags": sorted(odd),
        "uncited_perturbations": sorted(
            n for n, (_, t) in nodes.items() if t == "perturbation" and n not in cited
        ),
        "duplicate_node_ids": sorted(duplicates),
        "malformed_node_ids": sorted(malformed),
        "enforcement_table_mismatch": sorted(mismatch),
        "sources_without_a_header": sorted(headerless),
    }, examined


def main():
    reading, examined = take()

    if "--update" in sys.argv:
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
        was = set(before.get(key, []))
        now = reading[key]
        appeared = [x for x in now if x not in was]
        mark = "NEW" if appeared else ("down" if len(now) < len(was) else "")
        print(f"{mark:>4}  {key:<28} {len(now):>4}   (baseline {len(was)})")
        shown = appeared[:10]
        for item in shown:
            print(f"        + {item}")
        if len(appeared) > len(shown):
            print(f"        + and {len(appeared) - len(shown)} more")
        if appeared:
            appeared_anywhere = True

    print(f"\n      enforcement tables examined  {examined}")

    if appeared_anywhere:
        print(
            "\nSomething is new. Cite it, mark its enforcement row owed, or say in\n"
            "the act why the baseline moves, then run with --update.",
            file=sys.stderr,
        )
    return 1 if appeared_anywhere else 0


if __name__ == "__main__":
    sys.exit(main())
