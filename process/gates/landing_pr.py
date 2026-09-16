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
LANDING_RE = re.compile(r"^\*\*Landing PR:\*\* #(\d+)$", re.M)


def run(args):
    return subprocess.run(args, capture_output=True, text=True).stdout


def is_field(line):
    match = FIELD_RE.match(line)
    return bool(match) and match.group(1) in FIELDS


def said(text):
    """What the document says: header entries and blank lines removed."""
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
        if lines[index].strip():
            kept.append(lines[index].rstrip())
        index += 1
    return "\n".join(kept)


def landing_commit(path, ref):
    commits = run(["git", "log", "--format=%H", ref, "--", path]).split()
    tip = None
    landing = None
    for commit in commits:
        current = said(run(["git", "show", f"{commit}:{path}"]))
        if tip is None:
            tip, landing = current, commit
            continue
        if current != tip:
            break
        landing = commit
    return landing


def pull_request(commit, repo):
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
                 f"{commit}..origin/main"]).split("\n")
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
    check = "--check" in sys.argv
    repo = "toddwbucy/WeaverTools"
    ref = "origin/main"
    listed = run(["git", "grep", "-l", r"^\*\*Landing PR:\*\*", "HEAD",
                  "--", "docs", "process"]).split()
    paths = [entry.split(":", 1)[1] for entry in listed]
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
    sys.exit(main())
