#!/usr/bin/env python3
"""The parity of `grounds` targets across the clauses that recur.

`WeaverTools-Document-Format` section 4 rules that a clause grounded in one Spec is
grounded in every Spec that states it for the same reason, and that where the reason
differs the Spec says so at the clause. This walk reads that rule against the corpus.

**It is not registered and nothing runs it.** Working Process section 6 carries the
phase-three gates and this is not one of them, so it runs when a person types it.
Making it a census reading is `act-20`'s of issue #569, which is where that epic puts
new readings, and it exits 0 until then so it opens no backlog nobody declared.
`test_grounds_parity.py` runs beside it, the way `test_census.py` runs beside the
census, because a gate whose own correctness nobody checks is the thing this epic
exists to remediate.

**It compares axiom sets and never presence.** The first form printed one letter per
statement, `G` for grounded and `.` for bare, and marked every all-`G` row symmetric.
That notation cannot see a disagreement between two grounded statements, and there was
one in the corpus while it was printing: `spu-two-floor-links-types-without-config`
grounds in the socket invariant and in the contract invariant, and its three twins
ground in the socket invariant alone. A presence column reports that row as owing
nothing. So the cell is the set, always.

**It clusters on stems and not on tokens.** `internal-no-dependencies` and
`trace-no-internal-dependency` share no token, `dependencies` not being `dependency`,
and scored 0.25 against a cut of 0.6, so `weaver-internal-Spec` appeared in no row of
a table claiming to enumerate the corpus's recurring clauses.

**The stemmer truncates to a common prefix and does not try to reach a lemma.** The
second form reached for lemmas and split the pairs it was written to unify: `carries`
became `carry` while `carried` became `carri`, `cases` became `cas` while `case`
stayed `case`, and `string` became `str` while `strings` became `string`. Truncating
is what makes the inflections of one word meet, because every one of them is the stem
plus something. The rules are in `stem` below, in order, and the twelve tokens that
broke the second form are fixtures in the test beside this file.

**A cluster is a candidate and never a finding.** Two slugs that read alike are two
clauses to be read in both Specs, and the disagreements this prints are where to look
rather than what is owed. Whether two Specs argue the same reason is a judgment no
string comparison reaches.

**What it refuses to pass over in silence.** A `grounds` edge whose `from` or `to` is
written in a shape the format does not admit, or whose endpoints no document declares,
is the defect class most likely to exist and the one a reader most needs told about.
The first form dropped every one of them without a word. They print under `--faults`
and are counted in every run's last line.

    python3 process/gates/grounds_parity.py            # clusters, cells are axiom sets
    python3 process/gates/grounds_parity.py --tally    # per document, for acts 12-15
    python3 process/gates/grounds_parity.py --band     # the pairs below the cut
    python3 process/gates/grounds_parity.py --faults   # malformed and dangling edges
"""

import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import census  # noqa: E402

# **The fence, the stanza split and `field` are the census's and are imported.**
# The three keys below are not, because `census.py` reads `node:` stanzas and no
# edge at all, so there is nothing there to share and hoisting them would put dead
# patterns in another gate. They are written in that file's shape deliberately:
# matched loosely and judged strictly, so `edge:grounds` with no space is reported
# rather than quietly failing to match. They move into the census with the reading,
# which is `act-20`'s.
EDGE = re.compile(r"^[ \t]*edge:(.*)$", re.M)
FROM = re.compile(r"^[ \t]*from:(.*)$", re.M)
TO = re.compile(r"^[ \t]*to:(.*)$", re.M)

# **The cut for a cluster edge, and the floor for the band printed under it.** Two
# numbers rather than one, because a cut with nothing shown below it is a blind spot
# asserted to be empty. **The floor is one third exactly and not 0.34**, which sits
# just above the score of a pair sharing one stem of three and hid the near misses
# the band exists to show. Python computes `1 / 3` and `len(a & b) / len(a | b)` for
# one-of-three to the same float, so `>=` admits them.
CUT = 0.5
BAND = 1 / 3

DROP = {"the", "a", "an", "is", "are", "and", "of", "in", "it", "its", "at",
        "by", "for", "to", "on", "this", "that", "be"}

VOWELS = set("aeiouy")


def stem(word):
    """One token's stem, by rules applied in this order and no others.

    Small on purpose. The point is not linguistic coverage, it is that two seats
    reading one commit reach the same clusters, which a stemmer nobody can hold in
    their head does not give. **Every rule truncates or maps to a shorter form, and
    the last two run on the result of the first**, so the inflections of one word
    land on one string rather than on several lemmas.

    1. `-ies` and `-ied` become `-y`, so `carries`, `carried` and `carry` meet.
    2. `-ing` drops, but only when what is left holds a vowel, so `string` is not
       `str` while `splicing` is `splic`.
    3. `-ed` drops under the same vowel condition.
    4. `-es` drops after a sibilant, then a plain `-s` drops unless the word ends
       `ss`, `us` or `is`.
    5. A doubled final consonant loses one, so `pinned` reaches `pin`.
    6. A trailing `e` drops, so `case` and `cases` both reach `cas`.
    """
    w = word.lower()
    if len(w) > 4 and w.endswith("ies"):
        w = w[:-3] + "y"
    elif len(w) > 4 and w.endswith("ied"):
        w = w[:-3] + "y"
    elif len(w) > 5 and w.endswith("ing") and VOWELS & set(w[:-3]):
        w = w[:-3]
    elif len(w) > 4 and w.endswith("ed") and VOWELS & set(w[:-2]):
        w = w[:-2]
    elif len(w) > 3 and w.endswith("es") and w[:-2].endswith(("ss", "x", "z", "ch", "sh")):
        w = w[:-2]
    elif len(w) > 3 and w.endswith("s") and not w.endswith(("ss", "us", "is")):
        w = w[:-1]
    if len(w) > 3 and w[-1] == w[-2] and w[-1] not in VOWELS:
        w = w[:-1]
    if len(w) > 3 and w.endswith("e"):
        w = w[:-1]
    return w


def stems(slug):
    return {stem(t) for t in slug.split("-") if t and t not in DROP}


def read_corpus():
    """Assertions, their crates, their `grounds` targets, and what would not read.

    **The crate comes from the `asserts` edge and not from a list.** An earlier form
    carried twelve hard-coded prefixes, and an identifier outside them fell to a
    `None` crate that compared equal to every other such identifier, so a pair of
    them was skipped as same-crate and never reported. The Document Format has an
    assertion identifier be `<crate>-<slug>`, the crate is on the edge that declares
    the node, and an identifier that does not start with its own crate's name is a
    finding rather than a shape to guess around.
    """
    assertions, grounds, asserted_by, axioms, faults = {}, {}, {}, set(), []
    for path in census.docs():
        rel = os.path.relpath(path, census.ROOT)
        for block in census.GRAPH.findall(census.read(path)):
            # **Split on the keyword, not on `keyword:`.** Requiring the colon
            # here means a header that lacks it - `edge grounds` - never starts a
            # stanza, so it folds into the one above and is read as that stanza's
            # body. The walk then reports nothing, which is the failure this file
            # exists to stop. Splitting on the word and demanding the colon below
            # turns a silent merge into a counted fault.
            for stanza in re.split(r"(?=^\s*(?:node|edge)\b)", block, flags=re.M):
                head = stanza.lstrip()[:5].rstrip()
                if head in ("node", "edge") and not re.match(
                    r"^\s*(?:node|edge):\s*\S", stanza
                ):
                    faults.append(f"{rel}: a `{head}` header is not `key: value`")
                    continue
                node, bad_node = census.field(census.NODE_LINE, stanza)
                if node or bad_node:
                    kind, _ = census.field(census.KIND, stanza)
                    if bad_node:
                        faults.append(f"{rel}: a node line is not `key: value`")
                    elif kind == "assertion":
                        tag, _ = census.field(census.TAG, stanza)
                        assertions[node] = (rel, tag)
                    elif kind == "axiom":
                        axioms.add(node)
                    continue
                edge, bad_edge = census.field(EDGE, stanza)
                if bad_edge:
                    faults.append(f"{rel}: an edge line is not `key: value`")
                    continue
                if edge not in ("grounds", "asserts"):
                    continue
                src, bad_from = census.field(FROM, stanza)
                dst, bad_to = census.field(TO, stanza)
                if bad_from or bad_to or not src or not dst:
                    which = "from" if (bad_from or not src) else "to"
                    faults.append(f"{rel}: `{edge}` edge, {which} is not `key: value`")
                    continue
                if edge == "asserts":
                    asserted_by.setdefault(dst, set()).add(src)
                else:
                    grounds.setdefault(src, set()).add(dst)

    # **A dangling edge is reported AND dropped.** The first form reported it and
    # kept it, so an edge naming no declared axiom still reached `--tally` and
    # still widened an axiom set - the instrument counting what it had just called
    # unreadable. Endpoints are known only once every document is read, so the
    # filter runs here rather than at the point of collection.
    kept = {}
    for src, targets in grounds.items():
        if src not in assertions:
            faults.append(f"grounds from `{src}`, which no document declares as an assertion")
            continue
        for dst in sorted(targets):
            if dst not in axioms:
                faults.append(f"grounds to `{dst}`, which no document declares as an axiom")
            else:
                kept.setdefault(src, set()).add(dst)
    grounds = kept

    crates = {}
    for node_id in assertions:
        # **A node may be asserted by more than one crate and one is.**
        # `types-tagging-test` is asserted by both floor crates, per Working
        # Process section 6, which is why the check is that the identifier
        # carries one of its crates' prefixes rather than a named crate's.
        owners = sorted(asserted_by.get(node_id, ()))
        if not owners:
            faults.append(f"`{node_id}` is asserted by no crate, so its crate is unknown")
            crates[node_id] = node_id
            continue
        wearing = [c for c in owners if node_id.startswith(prefix_of(c) + "-")]
        if not wearing:
            faults.append(f"`{node_id}` is asserted by {', '.join(owners)} "
                          f"and carries no prefix of any of them")
        crates[node_id] = wearing[0] if wearing else owners[0]
    return assertions, grounds, crates, faults


def prefix_of(crate):
    return crate[len("weaver-"):] if crate.startswith("weaver-") else crate


def jaccard(a, b):
    return len(a & b) / len(a | b) if a and b else 0.0


def slug_of(node_id, crate):
    prefix = prefix_of(crate)
    return node_id[len(prefix) + 1:] if node_id.startswith(prefix + "-") else node_id


def clusters(assertions, crates):
    rows = [(n, crates[n], stems(slug_of(n, crates[n]))) for n in sorted(assertions)]
    parent = {r[0]: r[0] for r in rows}

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    band = []
    for i, (an, ac, at) in enumerate(rows):
        for bn, bc, bt in rows[i + 1:]:
            if ac == bc:
                continue
            j = jaccard(at, bt)
            if j >= CUT:
                parent[find(an)] = find(bn)
            elif j >= BAND:
                band.append((round(j, 2), an, bn))
    groups = {}
    for n in parent:
        groups.setdefault(find(n), []).append(n)
    return ({k: sorted(v) for k, v in groups.items() if len(v) > 1},
            sorted(band, reverse=True))


def short(axioms):
    return ",".join(sorted(x.replace("axiom-", "") for x in axioms)) or "-"


def tally(assertions, grounds):
    """Per document, the four figures acts 12 through 15 are told to read."""
    per, total = {}, [0, 0, 0]
    axes = {}
    for node_id, (rel, _) in assertions.items():
        slot = per.setdefault(rel, [0, 0, 0])
        slot[0] += 1
        total[0] += 1
        edges = grounds.get(node_id, set())
        if edges:
            slot[1] += 1
            slot[2] += len(edges)
            total[1] += 1
            total[2] += len(edges)
            for ax in edges:
                axes.setdefault(rel, {}).setdefault(ax.replace("axiom-", ""), 0)
                axes[rel][ax.replace("axiom-", "")] += 1
    print(f"{'document':<62}{'records':>8}{'grounded':>9}{'edges':>6}{'ungrounded':>11}")
    for rel in sorted(per, key=lambda r: -per[r][0]):
        n, g, e = per[rel]
        print(f"{rel:<62}{n:>8}{g:>9}{e:>6}{n - g:>11}")
    n, g, e = total
    print(f"{'TOTAL':<62}{n:>8}{g:>9}{e:>6}{n - g:>11}")
    print("\nper document, the edges by axiom")
    for rel in sorted(axes, key=lambda r: -per[r][0]):
        parts = ", ".join(f"{k} {v}" for k, v in sorted(axes[rel].items()))
        print(f"  {os.path.basename(rel):<42} {parts}")


def main():
    what = sys.argv[1:] 
    if len(what) > 1 or (what and what[0] not in ("--tally", "--band", "--faults")):
        # **An unrecognised argument refuses.** The first form fell through to the
        # default report for anything it did not recognise, so a mistyped flag read
        # as a clean run of a walk the caller never asked for - the same defect
        # `chunk_plan.py` records in its own argument parser.
        print(f"grounds_parity: unrecognised argument: {' '.join(what)}", file=sys.stderr)
        for line in __doc__.strip().splitlines()[-4:]:
            print(line.strip(), file=sys.stderr)
        return 2
    what = what[0] if what else ""

    assertions, grounds, crates, faults = read_corpus()

    if what == "--faults":
        for f in faults:
            print(f"  {f}")
        print(f"{len(faults)} edges or declarations this walk could not read")
        return 0

    if what == "--tally":
        tally(assertions, grounds)
        print(f"\n{len(faults)} unreadable, printed by --faults")
        return 0

    groups, band = clusters(assertions, crates)
    if what == "--band":
        print(f"cross-crate pairs scoring [{BAND:.3f}, {CUT}), for reading by hand")
        for j, a, b in band:
            print(f"  {j:.2f}  {a:<46} {b}")
        print(f"{len(band)} pairs, {len(faults)} unreadable, printed by --faults")
        return 0

    agree = disagree = 0
    print(f"recurring clauses, cut {CUT}, cell is the axiom set")
    for key in sorted(groups, key=lambda k: groups[k][0]):
        members = groups[key]
        sets = {frozenset(grounds.get(m, set())) for m in members}
        if len(sets) == 1:
            agree += 1
        else:
            disagree += 1
        print(f"\n[{'AGREE ' if len(sets) == 1 else 'DIFFER'}]")
        for m in members:
            print(f"    {m:<48} {short(grounds.get(m, set()))}")
    print(f"\n{agree} clusters agree, {disagree} differ, {len(band)} pairs in the "
          f"band below the cut, {len(faults)} unreadable")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
