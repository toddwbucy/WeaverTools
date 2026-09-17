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
  ran. **Both archive bugs are moot from 2026-09-13**: every archive directory
  was deleted at PR #563 and the standing rule is that none may exist, so the
  gate's exclusion for them was removed rather than kept. It had silently
  exempted any future directory named `archive/` from the header rule, which
  is the opposite of what a rule against archives wants - one appearing now is
  counted like any other source and shows up as headerless. **And is counted
  directly**, `archive_directories` having joined the reading the same week,
  since a header rule catches an archived `.rs` and says nothing about an
  archived `.md`.
- `--update` truncated the baseline before serialising, so an interrupt left
  the gate's whole memory half written.
- A citation was read as its first token, so `conforms: valid-node trailing`
  credited the node and said nothing about the rest of the line; and a record
  written `node:x` was accepted where the grammar is `key: value`. **A reader
  that tolerates a shape the format does not admit is as wrong as one that
  drops it** - both are matched loosely and judged strictly now.
- That fix reached `node:` and not `kind:` or `tag:` beside it, so
  `kind:assertion` and an empty `tag:` still passed. **The neighbours of a
  changed line are the sweep the corpus already has a rule for**, and this
  file broke it one line after applying it.
- Citations were collected into a set, so a second identical dangling one was
  folded away before `main` compared as a multiset. **The same
  set-for-multiset mistake in a third place**, after the gating path and the
  update path each carried it.
- A node this reader could not name ended the record. The malformed branch
  reported the identifier and returned, so the record's kind, its tag, its
  duplicate and its enforcement row went unread, and **the one node in this
  corpus carrying the one tag outside the assertion vocabulary was never
  judged**: `unknown_tags` printed zero for every act of this program while
  holding a record it had not looked at. It is the same shape as the two
  entries above it, a defect removing a node from every metric in silence
  rather than being reported as the one line it is. The identifier is now a
  finding and the rest of the record is read.
- The tag vocabulary was one set for every kind, which the format has never
  said. Section 5 carries one vocabulary per record kind, so the assertion
  five were being asked of a system record and the system record's own
  `ratified` was unknown to the reader that was supposed to accept it. The
  same edit's first form keyed the identifier exemption on the name and the
  vocabulary on the kind, which **admitted an assertion identifier no
  conformance header could legally cite** and handed any record that typed
  itself `system` a vocabulary the format gives to one node. Both halves key
  on the pair now.
- The selector read `.rs` and `.toml` while the regime governs every supported
  unit, so a CUDA kernel and three Python loop files stood inside workspace
  members and were never asked for a header at all. `sources_without_a_header`
  answered about the Rust subset of the set it names, which is this list's
  oldest shape wearing a fourth suit: a selector too narrow, printing a count
  confidently too low. The baseline carries the figures and this entry does
  not, a count copied into prose being a defect of its own.
  **The obligation was put on the unit rather than a directory some entries
  above and the kinds it walks stayed Rust**, so the rule was general and its
  reader was not. `HEADER_CITE` read `//!` alone, and a Python unit made to
  owe a header against it owes one no Python file can legally carry, which is
  a gate nobody can pass rather than a count that is merely wrong. **The first
  fix widened one pattern to both markers and was wrong in the other
  direction**, `#` opening every Python comment there is, so an inline
  citation in a function body and one inside a string literal each read as the
  unit's header. It made the obligation weaker for the one kind the act was
  admitting. The leaders are read apart now, `//!` against the whole text and
  `#` against the unit's opening block.
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
# **One vocabulary per record kind, which is how section 5 states the rule**
# rather than how this reading used to apply it. `ratified` on the system record
# is what lets the graph's set-level mark be generated from the apex rather than
# hand-edited, per Working Process section 5's checklist item 6. It was
# unreachable until the system node stopped being rejected as malformed, the
# reject arriving before the tag was read, so this reading has never judged the
# one node that carries it. A kind absent from this map takes TAGS.
TAGS_BY_KIND = {"system": {"ratified"}}
# The system node, per Document Format sections 3 and 5. It is the project's own
# name rather than one this format invents, which is the same reason a source
# path is a node identifier as it stands. Two classes stand outside the kebab
# rule, this one and a code node's path identifier, so this is not the only
# name the format exempts. **The exemption is the pair and not the name**: a
# record is excused the kebab rule only where it is this name AND
# types itself `system`, because the name alone admits an assertion identifier no
# conformance header could legally cite, and the kind alone hands a second record
# a vocabulary the format gives to exactly one node.
SYSTEM_NODE = "WeaverTools"

# **Every supported unit and not every tracked file**, per the operator's
# ruling of 2026-09-17. A conformance header exists so the graph can carry
# `code -> assertion -> doc`, and a unit the ingest never reads has no edge
# for a header to carry, so the regime governs the units the ingest supports
# and this reading follows that set. Rust is primary in this workspace and it
# is not alone. **Exempt from the count is not outside the regime**: a
# language absent here is one whose support has not landed, and its units
# enter the count in the act that lands it. Java, JS/TS, GoLang, HTML and CSS
# are issue #635 and shell is deferred rather than refused under #592, the way
# `.hadesignore` names #592 for the ask it closes. SQL is excluded outright
# and is not a deferral, on the same ruling, and `.hadesignore` carries that
# argument rather than this file.
#
# **Reading a citation and owing a header are two questions and the sets
# differ.** A manifest cites - three `tag: manifest` assertions are cited only
# from a `Cargo.toml` - and owes no header, having no module of its own to
# head. One walk answering both questions is already an entry in this file's
# list of its own wrong numbers, so the two sets are named apart.
READS_CITATIONS = ("*.rs", "*.toml", "*.cu", "*.py")
OWES_A_HEADER = (".rs", ".cu", ".py")

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
# Matched loosely and judged strictly, as `node:` is: the separator is part of
# the record, and a reader that tolerates `kind:assertion` accepts a shape the
# format does not admit. The node line's fix did not reach its neighbours.
KIND = re.compile(r"^[ \t]*kind:(.*)$", re.M)
TAG = re.compile(r"^[ \t]*tag:(.*)$", re.M)
HEADER_CITE = re.compile(r"^\s*//!\s*conforms:\s*([a-z0-9-]+)\s*$", re.M)
# **Python's file-level leader is `#`, and position is what makes it one.**
# `//!` is a marker Rust and CUDA give to the file alone, so it heads a unit
# wherever it sits and `HEADER_CITE` reads the whole text. Python has no such
# marker: `#` opens every comment it has, so a reader matching it anywhere
# makes an inline citation in a function body the unit's header, and a `#`
# citation inside a string literal one too. That is strictly weaker than the
# obligation on Rust, where a `///` or `//` citation resolves and leaves the
# file headerless, and it would weaken the rule for the one kind this widening
# admits. So the leader is anchored where `//!` is anchored by its own
# meaning: the head of the unit. **`head_block` is that anchor and this
# pattern carries no second copy of it.** The block yields only blank lines
# and lines beginning at column zero with `#`, so a `^[ \t]*#` here would
# match exactly what `^#` matches and a perturbation of the one into the
# other is a no-op. Two spellings of one guard read as two guards, and the
# one that cannot fail is the one a later act deletes as dead.
HASH_HEADER = re.compile(r"^#[ \t]*conforms:[ \t]*([a-z0-9-]+)[ \t]*$", re.M)
# **The whole rest of the line, and only from a comment.** Capturing one token
# credits `conforms: valid-node trailing` as a sound citation and says nothing
# about the trailing text; capturing the bare word anywhere makes prose that
# quotes the header form into a finding against a file with no defect. A
# citation's own defects are their own metric, a broken header and a broken
# declaration being two things a reader must tell apart.
ANY_CITE = re.compile(r"^[ \t]*(?://[/!]?|#)[ \t]*conforms:(.*)$", re.M)

# A build script is cargo's unit and not the crate's, and conforms to nothing,
# so counting it gives the metric a floor nobody can reach - the shape that
# teaches people to stop reading a number. **`archive/` used to sit here and
# does not**, per 2026-09-13: no archive directory may stand in the tree, so
# exempting one would hide the violation rather than measure it.
# **No sibling for the units that are not Rust**, a question the widening
# above had to answer: neither the kernel nor a loop file is the toolchain's
# unit the way a build script is, so nothing here earns the exemption.
# **`transformer.cu` is an entry that cannot clear while the verbatim carry
# stands, and that is not the floor this comment refuses.** A build script
# conforms to nothing, so counting it asks for a header no act could ever
# write and the metric never reaches zero however much work lands. The kernel
# conforms to something and the absence has a named condition - an act cutting
# the carry - so it is a backlog entry with a price rather than a floor, and
# H6's rule that the baseline is a backlog holds for it. An exemption would
# instead delete the one place a reader meets the cost of the carry.
NO_HEADER_OWED = ("build.rs",)
HEADER_ROW = re.compile(r"^\|\s*claim\s*\|\s*instrument\s*\|", re.I)


def read(path):
    with open(path, encoding="utf-8") as fh:
        return fh.read()


def head_block(text):
    """The lines that open a unit before anything that is not a comment.

    **A shebang, blank lines and `#` comments and nothing else.** The scan
    stops at the first line that is neither, so a docstring, an import or an
    indented comment ends it, and a citation below that point is an item's
    rather than the file's.
    """
    out = []
    for line in text.splitlines():
        if line.strip() and not line.startswith("#"):
            break
        out.append(line)
    return "\n".join(out)


def heads_the_unit(text, rel):
    """Whether the unit carries a file-level header citation.

    **The marker is the language's and so is what anchors it.** `//!` heads a
    file by its own meaning, so it is read against the whole text the way it
    always was. `#` heads nothing by itself, so it is read against the opening
    block alone, which is what keeps the obligation on a Python unit the same
    strength as the obligation on a Rust one.
    """
    if rel.endswith(".py"):
        return HASH_HEADER.search(head_block(text)) is not None
    return HEADER_CITE.search(text) is not None


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
        dirs[:] = sorted(d for d in dirs if d != ".git")
        for f in sorted(files):
            if f.endswith(".md"):
                yield os.path.join(base, f)


def ls_files(*flags):
    """The tracked set, or whatever `flags` narrows it to.

    **NUL separated.** A path with a space is two entries to `split()`, and git
    C-quotes a non-ASCII one, so both arrive as paths that are not there.

    Lifted from `sources()` on the olympus review of 2026-09-13, where it stood
    duplicated verbatim but for the pathspec and the second copy relied on this
    comment silently.
    """
    out = subprocess.run(
        ["git", "-C", ROOT, "ls-files", "-z", *flags],
        capture_output=True, text=True, check=True,
    ).stdout
    return [x for x in out.split("\0") if x]


# **A family of spellings and not one.** `WeaverTools-Working-Process` section 1
# says a rule naming today's instance is the same mistake as deleting today's
# instance, and the first form of this matched `archive` exactly: `Archive/`,
# `ARCHIVE/`, `archives/`, `archived/`, `_archive/` and `archive-2026/` all read
# as clean. The trailing group stops at `-` or `_` so `archiver` is a word and
# not a finding.
#
# **One optional leading underscore and not `_*`.** The wider form had no
# expressible counterpart in `.hadesignore`, gitignore having no way to say
# "any number of", so the two halves of the rule matched different sets - 93
# names apart, found by CodeRabbit on PR #566. **The family is bounded so both
# halves can state it exactly**, which is worth more than reaching `___archive`:
# a set neither half covers is consistent, where a set one covers and the other
# does not is the ingest taking a frozen copy the gate is about to refuse.
ARCHIVE_DIR = re.compile(r"^_?archive(?:s|d)?(?:[-_].*)?$", re.I)


def archives():
    """Every directory in the tree whose name says archive, which must be none.

    **The ruling of 2026-09-13 is that git is the archive**, so no archive
    directory stands in the repository. Deleting the two that existed removed
    that day's instances and nothing stopped the next one, which is the shape
    the ruling was made against - the deletion and this count being the two
    halves of one rule. Section 1 of the Working Process owns it.

    **The whole tracked set and not the workspace members.** `docs/archive/`
    sat outside every member and held sixteen files, so a check scoped the way
    `sources()` is would have seen one of the two directories it exists to
    catch.

    **Untracked too**, on the same reasoning `sources()` gives: the gate runs
    mid-act, and an archive being created is exactly when saying so is cheap.

    **On disk and not only in the index**, which is the guard `sources()` has
    carried since the bug list's entry on the same subject. Without it a
    deleted archive still read as standing until the deletion was staged, so
    **the gate failed the very act that obeys the rule** - found by the olympus
    review of 2026-09-13.

    Two exemptions, both named rather than left to be read off a sentence:

    **An ignored directory is not counted.** A `.gitignore` line is therefore a
    way out of the rule, and it is accepted because HADES honours `.gitignore`
    too, so the ingest agrees with the count. What it costs is a frozen copy on
    disk for a human reader, which is the same cost the rule is about - so a
    seat that finds one adds the deletion rather than the ignore line.

    **An empty directory is not counted**, git listing files and never
    directories. It cannot reach another seat: git carries no empty directory,
    so the window is one working tree, one act long, holding nothing.
    """
    found = set()
    # git reports forward slashes on every platform, so this does not use
    # os.sep - which `sources()` needs and this does not.
    for rel in set(ls_files()) | set(ls_files("--others", "--exclude-standard")):
        if not os.path.isfile(os.path.join(ROOT, rel)):
            continue
        parts = rel.split("/")
        for i, part in enumerate(parts[:-1]):
            if ARCHIVE_DIR.match(part):
                # **The outermost only.** A nested pair reported two entries
                # for one directory, and removing the outer one then moved two
                # baseline rows for one act where the comparison is by
                # identity.
                found.add("/".join(parts[: i + 1]) + "/")
                break
    return sorted(found)


def sources():
    """Every tracked unit a workspace member owns, and whether it owes a header.

    **Tracked and not walked.** The format's rule is over "every tracked unit",
    so a generated or scratch file left under a crate is not the corpus's and
    must not fail a gate. **A manifest cites too**: three `tag: manifest`
    assertions are cited only from a `Cargo.toml`, and a walk over `.rs` alone
    reported them uncited - the same miss as declining to read a crate's tests.
    **The half that is not Rust was the last standing form of it**, closed on
    the ruling of 2026-09-17: the sets above are the supported kinds and the
    subset of them that owes.

    **The obligation follows the unit and never a directory.**
    `WeaverTools-Document-Format` states it and names this exact
    mis-application: "a conformance count is over every tracked unit carrying
    a header, and a directory is never the rule ... a count taken over `src`
    alone reported the crates through 2026-08-16". The first form of this
    gate scoped by `/src/` and inherited the defect it was told about.
    """
    def git(*flags):
        return ls_files(*flags, "--", *READS_CITATIONS)

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
        owes = (rel.endswith(OWES_A_HEADER)
                and os.path.basename(rel) not in NO_HEADER_OWED)
        yield path, owes


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


def field(pattern, stanza):
    """A record's field, and whether it is written in a shape the format
    admits. `None` for absent, which an untagged assertion legitimately is."""
    found = pattern.search(stanza)
    if not found:
        return None, False
    raw = found.group(1)
    if not raw or not raw[0].isspace() or not raw.strip():
        return None, True
    return raw.strip(), False


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
                kind, bad_kind = field(KIND, stanza)
                tag, bad_tag = field(TAG, stanza)
                # **The identifier is judged against the kind, and a bad one no
                # longer ends the record.** The exemption is the pair: this name
                # typed `system`, per Document Format section 3, which declares
                # one such node and no other. The name alone would admit an
                # assertion identifier no conformance header could cite, since
                # NODE_OK still governs the citation side, and the kind alone
                # would hand any record the vocabulary below.
                # **Section 3 declares one system node and no other**, so the
                # two branches together are the exemption: the kind admits only
                # this name, and the kebab rule is waived only for that kind.
                # Keeping the name in a single condition instead leaves a
                # conjunct no case can reach, and a claim no test can fail is
                # what this corpus calls documented rather than enforced.
                if kind == "system" and name != SYSTEM_NODE:
                    malformed.append(f"{rel}: node: {name} "
                                     f"(only {SYSTEM_NODE} is kind: system)")
                elif kind != "system" and not NODE_OK.match(name):
                    malformed.append(f"{rel}: node: {name}")
                    # **Reported and then judged on**, rather than skipped. A
                    # reject that returns here takes the record's tag, its
                    # duplicate and its enforcement row out of every other
                    # reading in silence, which is how the one node carrying
                    # `tag: ratified` went twenty-two revisions without being
                    # judged and `unknown_tags` printed a zero it had not
                    # earned. A name this gate cannot read is one finding and
                    # not a licence to stop reading.
                if bad_kind or bad_tag:
                    which = "kind" if bad_kind else "tag"
                    malformed.append(f"{rel}: {name}, {which} is not `key: value`")
                    continue
                # **Every kind, since a collision is one whatever the kinds
                # are.** The format makes two spellings of one name a defect
                # without qualifying it by kind, and this corpus declares
                # seventy-eight nodes that are not assertions.
                if name in declared:
                    duplicates.append(f"{name}: {declared[name]} and {rel}")
                else:
                    declared[name] = rel
                # One vocabulary per record kind, and the kind disambiguates,
                # per Document Format section 5. **A record that drops the tag
                # its kind owes is a finding too**: the system record's
                # `ratified` is what generates the set-level mark, so its
                # absence ungrounds the mark in silence where a wrong value
                # would be caught.
                allowed = TAGS_BY_KIND.get(kind, TAGS)
                if tag is None:
                    if kind in TAGS_BY_KIND:
                        odd.append(f"{rel}: {name} carries no tag, "
                                   f"{kind} owes one of {sorted(allowed)}")
                elif tag not in allowed:
                    odd.append(f"{rel}: {name} ({kind}) carries `{tag}`, "
                               f"outside {sorted(allowed)}")
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

    # **A list and not a set.** A second identical dangling citation is a
    # second defect; folding it into a set removes it before `main` compares
    # as a multiset. This is the same set-for-multiset mistake the gating path
    # and the update path each carried, in the place the data is built.
    cited, headerless, bad_cites = [], [], []
    for path, owes in sources():
        text = read(path)
        for raw in ANY_CITE.findall(text):
            value = raw.strip()
            if raw and not raw[0].isspace():
                bad_cites.append(f"{os.path.relpath(path, ROOT)}: conforms:{raw}")
            elif NODE_OK.match(value):
                cited.append(value)
            else:
                bad_cites.append(f"{os.path.relpath(path, ROOT)}: conforms: {value}")
        rel = os.path.relpath(path, ROOT)
        if owes and not heads_the_unit(text, rel):
            headerless.append(rel)

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
            n for n, (_, t) in nodes.items() if t == "perturbation" and n not in set(cited)
        ),
        "duplicate_node_ids": sorted(duplicates),
        "malformed_node_ids": sorted(malformed),
        "malformed_citations": sorted(bad_cites),
        "enforcement_table_mismatch": sorted(mismatch),
        "documents_without_an_enforcement_table": sorted(tableless),
        "sources_without_a_header": sorted(headerless),
        "archive_directories": archives(),
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

    # **`archive_directories` is not a backlog and cannot be baselined.**
    # Every other metric here is a count the baseline may hold, the rule being
    # that no defect is new. This one's rule is that it is zero, per
    # `WeaverTools-Working-Process` section 1, and the generic `--update` path
    # defeated it: create an archive, run `--update`, and every later run
    # passes at one. Found by CodeRabbit on PR #566.
    #
    # **Only `--update` refuses.** A plain run reports the archive through the
    # ordinary comparison and exits 1, because a new archive is a new defect
    # and that is what 1 means here. What is closed is the recording of it, and
    # separately the trusting of a baseline that already holds one.
    if "--update" in sys.argv and reading["archive_directories"]:
        print("census: archive directories stand in this tree:", file=sys.stderr)
        for d in reading["archive_directories"]:
            print(f"  {d}", file=sys.stderr)
        print(
            "Git is the archive, per WeaverTools-Working-Process section 1, so\n"
            "this reading is zero and is not a backlog. Delete them rather than\n"
            "recording them; a plain run reports them as the new defect they are.",
            file=sys.stderr,
        )
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

    # **A baseline holding one is a poisoned baseline.** The reading above is
    # already zero or the run returned, so a nonempty entry here was written by
    # a version without that guard, or by hand.
    if before.get("archive_directories"):
        print(
            "census: the baseline records archive directories: "
            f"{before['archive_directories']}.\n"
            "That reading is zero by rule and cannot be carried as a backlog.\n"
            "Clear the entry in the baseline; the tree is already clean.",
            file=sys.stderr,
        )
        return 2

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
