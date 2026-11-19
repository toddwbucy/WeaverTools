#!/usr/bin/env python3
"""early-exit.py FILE...: each line of each shell script that pipes a producer into a
reader that can exit before its input ends: head, or grep with -q, -m, -l or -L (or their
long forms). Under pipefail such a pipeline fails with the producer's SIGPIPE (status 141)
whenever the reader exits first, so a check built on it can read as failed, or, negated,
as passed. Whole-line comments are skipped; a pipe is a lone '|', not '||' or '|&'.
A cluster of short options is read one letter at a time, and a letter that takes a word
(-e, -f, -A, -B, -C, -d or -D) ends it: the rest of the cluster is that word where
anything follows the letter, and the next word is skipped where nothing does. So `grep -e
PAT -q` and `grep -eproof -q` are found, and `grep -eq x`, whose pattern is q, is not.
--regexp and --file have the next word skipped (CodeRabbit on pull request 44,
2026-11-19). Left, and not found: grep's options after its pattern, which GNU grep also
takes (`grep PAT -q`), since a split on whitespace cannot tell such a line from a quoted
pattern holding "-q"; and the other long options that take a separate word (`grep
--context 2 -q`), whose word is read as the pattern. Prints FILE:LINE: TEXT for each, and
exits 1 if there is any. Standard library only. t-pipefail.sh runs it
(weaver-appliance-build-Spec section 2, the host tests)."""
import re, sys

EARLY_SHORT = "qmlL"
TAKES_WORD = "efABCdD"
EARLY_LONG = re.compile(r"^--(quiet|silent|max-count|files-with)")
LONG_TAKES_NEXT = ("--regexp", "--file")
PIPE = re.compile(r"(?<![|])\|(?![|&])")


def early(segment):
    """Whether the command a pipe feeds, SEGMENT up to its end, is an early reader."""
    words = re.split(r";|&&|\|\||\)|`", segment, maxsplit=1)[0].split()
    if not words:
        return False
    if words[0] == "head":
        return True
    if words[0] == "grep":
        skip = False
        for w in words[1:]:
            if skip:
                skip = False
                continue
            if not w.startswith("-") or w == "--":
                return False
            if w.startswith("--"):
                if EARLY_LONG.match(w):
                    return True
                skip = w in LONG_TAKES_NEXT
                continue
            for i, c in enumerate(w[1:], 1):
                if c in EARLY_SHORT:
                    return True
                if c in TAKES_WORD:
                    skip = i == len(w) - 1
                    break
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
