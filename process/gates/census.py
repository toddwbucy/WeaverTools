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
  that also names this exact mis-application. It walked the filesystem, where
  the format's rule is over the tracked set, and read `.rs` alone, where three
  `tag: manifest` assertions are cited only from a `Cargo.toml`.
- Widening the citation check to every node kind, on a misreading of that same
  document: a header names an assertion, which the format states outright, and
  no citation in this corpus names anything else. **A review pass asked for
  the widening and the next one called it a defect**, which is what four
  rounds on one act look like.
- ```graph matched ```graphviz, so a diagram fence would declare phantom nodes
  that mask the dangling citations this exists to catch.
- `node:` with no space, a trailing space after `kind:`, an indented stanza:
  each removed a node from every metric silently rather than being reported as
  a line the gate cannot read.
- The reporting path of `--update` compared as a set while the gating path
  compared as a multiset, so a recurrence under a name already baselined moved
  the baseline with nothing printed.
- A metric's entry carried its own count, so adding one assertion to a
  document with no enforcement table read as one defect gone and another
  arrived. **It would have fired on the first act of issue #558's backlog**,
  which is a gate crying wolf on its first real use.
- `os.walk` is unordered, so the first declaration of a duplicated identifier -
  the one kept - depended on the filesystem, and two seats could baseline
  different strings from one commit.
- `archive/` was excluded in a comment and not in the code, putting four files
  that are never compiled into a backlog with no way to close them.
- Only the first enforcement table in a document was read, and `--update`
  raised on a baseline key the reading no longer had.
- Reading every table then ran one table's row loop into the next one's
  header, counting two adjacent tables of one row each as five - a fix whose
  own watch only covered the case where a blank line separated them.
- The index lists a file the disk may not hold, so a tracked file deleted and
  not yet staged killed the gate with a traceback, and a new file not yet
  added was invisible to the header count. **Both are the ordinary mid-act
  state**, which is when the gate is told to run.
- `ls-files` output was split on whitespace, so a path with a space became two
  paths that are not there.
- Text in `archive/` still fed the citation set, so a citation in a file that
  is never compiled could take a perturbation out of the backlog while no test
  ran.
- `--update` truncated the baseline before serialising, so an interrupt left
  the gate's whole memory half written.
- A citation was read as its first token, so `conforms: valid-node trailing`
  credited the node and said nothing about the rest of the line; and a record
  written `node:x` was accepted where the grammar is `key: value`. **A reader
  that tolerates a shape the format does not admit is as wrong as one that
  drops it** - both are matched loosely and judged strictly now.
"""

import glob
import json
import os
import re
import subprocess
import sys
from collections import Counter

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
HERE = os.path.dirname(os.path.abspath(__file__))
BASELINE = os.path.join(HERE, "census-baseline.json")

# `WeaverTools-Document-Format` fixes this vocabulary. A tag outside it is a
# finding rather than a silent miss.
TAGS = {"compile-pin", "compile-fail", "perturbation", "manifest", "review"}

# **The info string is the whole word.** An unanchored `graph` also opens
# a ```graphviz or ```graphql fence, whose contents would declare phantom
# nodes that mask the dangling citations this gate exists to catch.
GRAPH = re.compile(r"```graph[ \t]*\r?\n(.*?)```", re.S)
# **The separator is part of the record.** `node:x` is not `node: x`, and a
# reader that tolerates the first accepts a record the grammar does not admit
# while a reader that simply fails to match it drops the node into silence -
# so the line is matched loosely and judged strictly.
NODE_LINE = re.compile(r"^[ \t]*node:(.*)$", re.M)
NODE_OK = re.compile(r"^[a-z0-9-]+$")
KIND = re.compile(r"^\s*kind:\s*(\S+)\s*$", re.M)
TAG = re.compile(r"^\s*tag:\s*(\S+)\s*$", re.M)
HEADER_CITE = re.compile(r"^\s*//!\s*conforms:\s*([a-z0-9-]+)\s*$", re.M)
# **The whole rest of the line, and only from a comment.** Capturing one token
# credits `conforms: valid-node trailing` as a sound citation and says nothing
# about the trailing text; capturing the bare word anywhere makes prose that
# quotes the header form into a finding against a file with no defect. A
# citation's own defects are their own metric, a broken header and a broken
# declaration being two things a reader must tell apart.
ANY_CITE = re.compile(r"^[ \t]*(?://[/!]?|#)[ \t]*conforms:(.*)$", re.M)

# A build script is cargo's unit and not the crate's, and conforms to nothing.
# `archive/` is not a workspace member and is never compiled, so counting
# either gives the metric a floor nobody can reach - the shape that teaches
# people to stop reading a number.
NO_HEADER_OWED = ("build.rs",)
ARCHIVED = f"{os.sep}archive{os.sep}"
HEADER_ROW = re.compile(r"^\|\s*claim\s*\|\s*instrument\s*\|", re.I)


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
    # **Comments go before the split, not after.** Splitting on the comma
    # first makes a comment part of the *next* entry's chunk, so one `#`
    # anywhere in the array silently drops the member that follows it and
    # every count falls without the guard below ever firing.
    body = "\n".join(line.split("#", 1)[0] for line in block.group(1).splitlines())
    found = []
    for raw in body.split(","):
        entry = raw.strip().strip('"')
        if not entry:
            continue
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
        # **Sorted, so two boxes read one commit the same way.** `os.walk`
        # yields directories in the filesystem's order, and the first
        # declaration of a duplicated identifier is the one kept, so an
        # unsorted walk lets two seats baseline different strings from the
        # same tree - the machine-dependent gate the enforcement section
        # already records for the clippy count.
        dirs[:] = sorted(d for d in dirs if d not in (".git", "archive"))
        for f in sorted(files):
            if f.endswith(".md"):
                yield os.path.join(base, f)


def sources():
    """Every tracked unit a workspace member owns, and whether it owes a header.

    **Tracked and not walked.** The format's rule is over "every tracked unit",
    so a generated or scratch file left under a crate is not the corpus's and
    must not fail a gate. **A manifest cites too**: three `tag: manifest`
    assertions are cited only from a `Cargo.toml`, and a walk over `.rs` alone
    reported them uncited - the same miss as declining to read a crate's tests,
    unfixed for the half that is not Rust.

    **The obligation follows the unit and never a directory.**
    `WeaverTools-Document-Format` states it and names this exact
    mis-application: "a conformance count is over every tracked unit carrying
    a header, and a directory is never the rule ... a count taken over `src`
    alone reported the crates through 2026-08-16". The first form of this
    gate scoped by `/src/` and inherited the defect it was told about.
    """
    def git(*flags):
        # **NUL separated.** A path with a space is two entries to `split()`,
        # and git C-quotes a non-ASCII one, so both arrive as paths that are
        # not there.
        out = subprocess.run(
            ["git", "-C", ROOT, "ls-files", "-z", *flags, "--", "*.rs", "*.toml"],
            capture_output=True, text=True, check=True,
        ).stdout
        return [x for x in out.split("\0") if x]

    # **Tracked, plus work that is not staged yet.** The format's rule is over
    # the tracked unit to exclude what is generated or scratch, not to make an
    # act's own new file invisible - and the gate is run mid-act, which is
    # exactly when a new source file is written and not yet added. Untracked
    # here means untracked and not ignored, so a build product stays out.
    seen = set(git()) | set(git("--others", "--exclude-standard"))
    inside = tuple(os.path.relpath(m, ROOT) + os.sep for m in members())
    for rel in sorted(seen):
        if not rel.startswith(inside):
            continue
        path = os.path.join(ROOT, rel)
        # **The index lists it; the disk may not.** A tracked file deleted and
        # not yet staged is the ordinary mid-act state, and reading it is a
        # traceback where the gate owes a reading.
        if not os.path.isfile(path):
            continue
        archived = ARCHIVED in f"{os.sep}{rel}"
        owes = (
            rel.endswith(".rs")
            and os.path.basename(rel) not in NO_HEADER_OWED
            and not archived
        )
        yield path, owes, archived


def enforcement_table(text):
    """Rows across every enforcement table a document carries, or `None`.

    **Every table and not the first.** Nothing in the Document Format stops a
    document splitting its enforcement across two tables, and stopping at the
    first counts one section's rows against the whole document's assertions -
    a loud false mismatch beside a silent omission of the rows that do exist.
    """
    lines = text.splitlines()
    found, i = None, 0
    while i < len(lines):
        if not HEADER_ROW.match(lines[i]):
            i += 1
            continue
        # **A row loop that does not stop at the next header** counts that
        # table's header and separator as claims and then counts its rows
        # again when the outer scan reaches it. Two adjacent tables of one
        # row each read as five.
        found = found or 0
        i += 2
        while i < len(lines) and lines[i].startswith("|") and not HEADER_ROW.match(lines[i]):
            found += 1
            i += 1
    return found


def take():
    """One reading. Every value is a sorted list of the offenders themselves,
    so the comparison is by identity rather than by count."""
    # **`declared` holds every node whatever its kind**, which is what makes
    # a duplicated identifier detectable across kinds. It is not what
    # citations are measured against: a header names an assertion, per the
    # Document Format, and the widening that once measured against every kind
    # was a misreading this file records in its docstring.
    nodes, declared, duplicates, malformed, odd = {}, {}, [], [], []
    texts = {}

    for path in docs():
        rel = os.path.relpath(path, ROOT)
        text = read(path)
        declared_here = False
        for block in GRAPH.findall(text):
            # **Split on the record's own keyword, not on a blank line.** Two
            # records written back to back are one stanza to a blank-line
            # splitter, which drops all but the first - the same class of
            # miss as reading one `node:` per block, and invisible for the
            # same reason.
            for stanza in re.split(r"(?=^\s*(?:node|edge):)", block, flags=re.M):
                line = NODE_LINE.search(stanza)
                if not line:
                    continue
                raw = line.group(1)
                name = raw.strip()
                if raw and not raw[0].isspace():
                    malformed.append(f"{rel}: node:{raw} (no space after the key)")
                    continue
                if not NODE_OK.match(name):
                    malformed.append(f"{rel}: node: {name}")
                    continue
                kind = KIND.search(stanza)
                kind = kind.group(1) if kind else None
                tag = TAG.search(stanza)
                tag = tag.group(1) if tag else None
                # **Every kind, since a collision is one whatever the kinds
                # are.** The format makes two spellings of one name a defect
                # without qualifying it by kind, and this corpus declares
                # seventy-eight nodes that are not assertions.
                if name in declared:
                    duplicates.append(f"{name}: {declared[name]} and {rel}")
                else:
                    declared[name] = rel
                if tag is not None and tag not in TAGS:
                    odd.append(f"{name} ({tag})")
                if kind != "assertion":
                    continue
                # **The first declaration keeps the node.** Letting the last
                # win moves a duplicate's assertions to the later document and
                # drops the earlier one out of per-document accounting
                # entirely, so its table is never examined.
                nodes.setdefault(name, (rel, tag))
                declared_here = True

        if declared_here:
            texts[rel] = text

    cited, headerless, bad_cites = set(), [], []
    for path, owes, archived in sources():
        # **An archived file cites nothing.** It is never compiled, so a
        # citation in it buys no instrument - and letting its text satisfy a
        # perturbation would take that claim out of the backlog while no test
        # runs, which is the opposite of what this metric is for.
        if archived:
            continue
        text = read(path)
        for raw in ANY_CITE.findall(text):
            value = raw.strip()
            if raw and not raw[0].isspace():
                bad_cites.append(f"{os.path.relpath(path, ROOT)}: conforms:{raw}")
            elif NODE_OK.match(value):
                cited.add(value)
            else:
                bad_cites.append(f"{os.path.relpath(path, ROOT)}: conforms: {value}")
        if owes and not HEADER_CITE.search(text):
            headerless.append(os.path.relpath(path, ROOT))

    # **The documents without a table are the metric, not the ones with.** A
    # count of mismatches over the one document that has a table reads as
    # corpus-wide assurance and is not: rename that header and the mismatch
    # count stays zero while nothing is examined at all. Listed this way, a
    # document losing its table is a new entry and fails.
    per_doc = Counter(r for r, _ in nodes.values())
    mismatch, tableless = [], []
    for rel in sorted(per_doc):
        rows = enforcement_table(texts[rel])
        count = per_doc[rel]
        if rows is None:
            # **The entry is the document and not its count.** With the count
            # in the string, adding one assertion to a tableless Spec reads as
            # one defect gone and another arrived, and the gate fails for a
            # document that was tableless before and after. It would have
            # fired on the first act of issue #558's own backlog.
            tableless.append(rel)
            continue
        if rows != count:
            mismatch.append(f"{rel} ({count} nodes, {rows} rows)")

    return {
        # **Against the assertions.** `WeaverTools-Document-Format`: "a header
        # naming an assertion the corpus does not declare is a dangling edge".
        # A previous form measured against every node kind on a misreading of
        # that document's kebab-case rule, which admits a class the format
        # forbids; no citation in this corpus names a non-assertion node.
        "dangling_citations": sorted(i for i in cited if i not in nodes),
        "untagged_assertions": sorted(n for n, (_, t) in nodes.items() if t is None),
        "unknown_tags": sorted(odd),
        "uncited_perturbations": sorted(
            n for n, (_, t) in nodes.items() if t == "perturbation" and n not in cited
        ),
        "duplicate_node_ids": sorted(duplicates),
        "malformed_node_ids": sorted(malformed),
        "malformed_citations": sorted(bad_cites),
        "enforcement_table_mismatch": sorted(mismatch),
        "documents_without_an_enforcement_table": sorted(tableless),
        "sources_without_a_header": sorted(headerless),
    }


def difference(now, was):
    """What `now` has that `was` did not, and the reverse, as multisets.

    **Both paths use this.** The gating path was fixed to a multiset and the
    reporting path was left on a set, so a defect recurring under a name the
    baseline already held moved the baseline with nothing printed - and the
    rule is that moving it is a sentence in the act's commit message.
    """
    remaining = Counter(was)
    gained = []
    for item in now:
        if remaining[item]:
            remaining[item] -= 1
        else:
            gained.append(item)
    return gained, sorted(remaining.elements())


def main():
    reading = take()

    unknown = [a for a in sys.argv[1:] if a != "--update"]
    if unknown:
        # **A mistyped flag refuses.** Falling through to the comparison runs
        # a mode the operator did not ask for and prints a pass, which for a
        # script whose subject is a confident wrong number is the same hazard.
        print(f"census: unrecognised argument: {unknown[0]}", file=sys.stderr)
        return 2

    if "--update" in sys.argv:
        # **It says what moved.** The rule is that moving the baseline is a
        # sentence in the act's commit message, and an operator who has to
        # re-run the gate first and read the difference by eye is doing the
        # quiet re-reading the rule forbids.
        old = {}
        if os.path.exists(BASELINE):
            with open(BASELINE, encoding="utf-8") as fh:
                old = json.load(fh)
        for key in sorted(set(reading) | set(old)):
            gained, lost = difference(reading.get(key, []), old.get(key, []))
            if gained or lost:
                print(f"{key}: {len(old.get(key, []))} -> {len(reading.get(key, []))}")
                for x in gained:
                    print(f"  + {x}")
                for x in lost:
                    print(f"  - {x}")
        # **Written beside and moved into place.** `open(..., "w")` truncates
        # first, so an interrupt or a serialisation error leaves the gate's
        # whole memory half written and every later run dies in `json.load`.
        tmp = BASELINE + ".new"
        with open(tmp, "w", encoding="utf-8") as fh:
            json.dump(reading, fh, indent=2, sort_keys=True)
            fh.write("\n")
        os.replace(tmp, BASELINE)
        print("baseline written:", json.dumps({k: len(v) for k, v in reading.items()}))
        return 0

    if not os.path.exists(BASELINE):
        print("no baseline; run --update for the first reading", file=sys.stderr)
        return 2

    with open(BASELINE, encoding="utf-8") as fh:
        before = json.load(fh)

    missing = sorted(set(before) - set(reading))
    if missing:
        print(
            f"census: the baseline holds keys this reading does not: {missing}.\n"
            "A metric that disappears stops being gated silently; rename it in\n"
            "the baseline or say in the act why it goes.",
            file=sys.stderr,
        )
        return 2

    appeared_anywhere = False
    for key in sorted(reading):
        # **A multiset and not a set.** Two of these lists can hold the same
        # string twice - one id declared a third time, one malformed id
        # written twice - and a set comparison calls the second occurrence
        # familiar while the count visibly rises.
        now = reading[key]
        appeared, _ = difference(now, before.get(key, []))
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
