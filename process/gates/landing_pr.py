#!/usr/bin/env python3
"""Derive each document's `Landing PR:` field and check the tree against it.

The field names **the pull request that last changed what the document says**,
per `WeaverTools-Working-Rules` section 1. It is not the last pull request to
touch the file, and the difference is the whole reason this script exists.

The derivation, in three steps per document:

  1. Walk the file's commits newest first.
  2. Normalize each version to what the document says: its header entries and
     its blank lines removed. Compare against the tip's normalization and take
     the oldest commit in the run that still matches. That commit produced the
     content standing today.
  3. Resolve that commit to a pull request through the commits API, which
     reaches a squash merge and a merge commit alike. A subject line reaches
     only the first.

**Two ways of getting this wrong, both of which happened before it was right**,
recorded because each printed a confident answer:

  Deriving by touch put twenty five of forty documents at one sweep's number.
  That act had inserted blank lines between header entries and changed no other
  line in any of them, so it touched every file and said nothing in any.

  Ending a header entry at the next blank line leaks. An entry with a blank
  line in its body stops being skipped halfway, and the rest of it counts as
  content, which moved six documents onto the wrong act. An entry runs to the
  next header field or to the rule, which is what FIELDS below is for.

**It needs the network and is therefore an instrument rather than a gate.**
Step 3 asks GitHub. A seat runs it when a sweep lands or when a field is
doubted, and the census does not.

    python3 process/gates/landing_pr.py            # print the derivation
    python3 process/gates/landing_pr.py --check    # exit 1 on any mismatch
"""

import json
import os
import re
import subprocess
import sys

FIELDS = (
    "Status", "Version", "Date filed", "Date started", "Document ID", "Parent",
    "Editorial", "Landing PR", "Companion contract", "Base", "Scope", "Method",
    "Staged", "Prior version", "External boundaries", "Depends on", "Revised",
)
FIELD_RE = re.compile(r"^\*\*([A-Z][A-Za-z ]*):\*\*")
ENTRY_RE = re.compile(r"^\*\*Revised:\*\*")
# re.M so `search` over a whole file anchors per line. `match` over a single
# line is unaffected by it, anchoring at position zero either way.
LANDING_RE = re.compile(r"^\*\*Landing PR:\*\* #(\d+)\s*$", re.M)


class Unchecked(Exception):
    """A command this script depends on did not run."""


def run(args, spare=()):
    """Standard output of a command, refusing an exit status it did not expect.

    **A helper that discards the status prints a clean nothing for a command
    that never ran**, and nothing reads like success. `git grep` exits 1 when it
    matches nothing and 128 when the ref or the pathspec is wrong, and both give
    an empty stdout, so a walk over a bad ref finds no documents and reports
    every one of them sound. `spare` names the non-zero statuses that carry
    meaning at a call site, and every other one raises.
    """
    done = subprocess.run(args, capture_output=True, text=True)
    if done.returncode and done.returncode not in spare:
        first = done.stderr.strip().splitlines()
        raise Unchecked(f"{' '.join(args)}\n  exit {done.returncode}: "
                        f"{first[0] if first else 'no message'}")
    return done.stdout


def is_field(line):
    """Whether a line opens a header field rather than bold prose.

    A bolded sentence inside an entry can end in a colon, so the name is
    checked against FIELDS rather than the shape alone.
    """
    match = FIELD_RE.match(line)
    return bool(match) and match.group(1) in FIELDS


def said(text):
    """What the document says: header entries, the field, and blank lines gone.

    **The field is removed along with the entries, and leaving it in is a
    self-reference.** It lives in the header and records provenance rather than
    content, so a version carrying it would differ from the version before it
    and the walk would stop at whatever act wrote it. Every document would then
    derive the sweep that filled the field, which is the defect this whole
    derivation exists to avoid, arriving one level up.
    """
    lines = text.split("\n")
    kept = []
    index = 0
    while index < len(lines):
        if ENTRY_RE.match(lines[index]):
            index += 1
            while index < len(lines) and not is_field(lines[index]) \
                    and not lines[index].startswith("---"):
                index += 1
            continue
        if LANDING_RE.match(lines[index]):
            index += 1
            continue
        if lines[index].strip():
            kept.append(lines[index].rstrip())
        index += 1
    return "\n".join(kept)


def landing_commit(path, ref):
    """The oldest commit whose content still matches the tip's."""
    commits = run(["git", "log", "--format=%H", ref, "--", path]).split()
    tip = None
    landing = None
    for commit in commits:
        # 128 is the path being absent at that commit, which is a content
        # difference and ends the walk on the next comparison.
        current = said(run(["git", "show", f"{commit}:{path}"], spare=(128,)))
        if tip is None:
            tip, landing = current, commit
            continue
        if current != tip:
            break
        landing = commit
    return landing


def pull_request(commit, repo):
    """The pull request a commit arrived in, squashed or merged."""
    env = {**os.environ, "GITHUB_TOKEN": ""}
    direct = subprocess.run(
        ["gh", "api", f"repos/{repo}/commits/{commit}/pulls", "-q", ".[].number"],
        capture_output=True, text=True, env=env,
    ).stdout.split()
    if direct:
        return direct[0]
    # A merge commit rather than a squash: the branch commit belongs to no pull
    # request, and the merge that brought it in does.
    merge = run(["git", "log", "--ancestry-path", "--merges", "--reverse",
                 f"{commit}..origin/main"], spare=(128,)).split("\n")
    if merge and merge[0].startswith("commit "):
        sha = merge[0].split()[1]
        found = subprocess.run(
            ["gh", "api", f"repos/{repo}/commits/{sha}/pulls", "-q", ".[].number"],
            capture_output=True, text=True, env=env,
        ).stdout.split()
        if found:
            return found[0]
    return None


def main():
    """Print the derivation, and under --check exit 1 on any mismatch."""
    check = "--check" in sys.argv
    repo = "toddwbucy/WeaverTools"
    ref = "origin/main"
    listed = run(["git", "grep", "-l", r"^\*\*Landing PR:\*\*", "HEAD",
                  "--", "docs", "process"], spare=(1,)).split()
    paths = [entry.split(":", 1)[1] for entry in listed]
    if not paths:
        raise Unchecked("no document carries the field, which is not a corpus "
                        "this rule has ever described")
    mismatches = 0
    for path in sorted(paths):
        commit = landing_commit(path, ref)
        derived = pull_request(commit, repo) if commit else None
        match = LANDING_RE.search(open(path).read())
        carried = match.group(1) if match else None
        flag = ""
        if derived != carried:
            flag = "  MISMATCH"
            mismatches += 1
        print(f"{path}  carries #{carried}  derives #{derived}{flag}")
    if check:
        print(f"\n{len(paths)} documents, {mismatches} mismatched")
        return 1 if mismatches else 0
    return 0


if __name__ == "__main__":
    # Three-valued like process/gates/lock.sh: 0 in step, 1 a mismatch the
    # tree owns, 2 the derivation could not run and the fields are unchecked
    # rather than sound.
    try:
        sys.exit(main())
    except Unchecked as unchecked:
        print(f"landing_pr: unchecked: {unchecked}", file=sys.stderr)
        sys.exit(2)
