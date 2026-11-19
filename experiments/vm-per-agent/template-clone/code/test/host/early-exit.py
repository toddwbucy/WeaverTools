#!/usr/bin/env python3
"""early-exit.py FILE...: each line of each shell script that pipes a producer into a
reader that can exit before its input ends: head, or grep with -q, -m, -l or -L (or their
long forms). Under pipefail such a pipeline fails with the producer's SIGPIPE (status 141)
whenever the reader exits first, so a check built on it can read as failed, or, negated,
as passed. Whole-line comments are skipped; a pipe is a lone '|', not '||' or '|&'.
Prints FILE:LINE: TEXT for each, and exits 1 if there is any. Standard library only.
t-pipefail.sh runs it (weaver-appliance-build-Spec section 2, the host tests)."""
import re, sys

EARLY_GREP = re.compile(r"^-[A-Za-z]*[qmlL]|^--(quiet|silent|max-count|files-with)")
PIPE = re.compile(r"(?<![|])\|(?![|&])")


def early(segment):
    """Whether the command a pipe feeds, SEGMENT up to its end, is an early reader."""
    words = re.split(r";|&&|\|\||\)|`", segment, maxsplit=1)[0].split()
    if not words:
        return False
    if words[0] == "head":
        return True
    if words[0] == "grep":
        for w in words[1:]:
            if not w.startswith("-") or w == "--":
                return False
            if EARLY_GREP.match(w):
                return True
    return False


def findings(path):
    out = []
    with open(path, encoding="utf-8", errors="replace") as f:
        for n, line in enumerate(f, 1):
            if re.match(r"\s*#", line):
                continue
            if any(early(line[m.end():]) for m in PIPE.finditer(line)):
                out.append((n, line.strip()))
    return out


if __name__ == "__main__":
    bad = 0
    for p in sys.argv[1:]:
        for n, text in findings(p):
            print("%s:%d: %s" % (p, n, text[:160]))
            bad += 1
    sys.exit(1 if bad else 0)
