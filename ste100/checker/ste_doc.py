"""Check a Markdown document against the WeaverTools STE100 writing standard.

    python3 ste100/checker/ste_doc.py <document.md> [--verbs ste100/var/approved-verbs.json]
        [--json <out.json>]

The document is cut into units: a paragraph (soft-wrapped lines joined), a list item, a
heading, or a table cell. Each unit is masked as the conversion masks code
(ste_mask.mask, with prose in parentheses kept as prose), then cut into sentences. Each
sentence goes through ste_check.check on its own, with no source text, so a dictionary
word in capitals always fails. Not checked: the title (the first "# " heading, the
document's name and so a technical name), fenced code (masked whole), quotations (masked
whole), HTML comments, and the header line itself. A table separator row holds no
words, so it gives no sentence.

Rules, each reported by name on the sentence that breaks it:

  verbs, ing, contractions, semicolon, capitals, length, placeholders
                     ste_check's hard checks (20 words for an instruction, 25 for a
                     description)
  unresolved-term    a word that lexicon/terms.toml marks status = "unresolved"
  banned-word        genuinely, honestly, actually (Working Rules)
  ascii              any character outside ASCII, anywhere in the file (Working Rules)
  header             no header line, a header line that names another standard version,
                     or a lexicon sha256 that is not the lexicon's sha256 now

The lexicon sha256 is lexicon_sha256(): every file in lexicon/ except README.md, in name
order, each as its name, a NUL byte, its bytes and a NUL byte. The header line is

    STE100: WeaverTools STE100 writing standard <version>, lexicon sha256:<64 hex>

Exit 0: every sentence passes and the header holds. Exit 1: any failure. Exit 2: the
local lexicon (var/approved-verbs.json, made by ste_extract.py) or the document is
missing. Unapproved and unknown words are reported as a measure and do not fail.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
import tomllib
from dataclasses import dataclass
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import ste_check  # noqa: E402
import ste_mask  # noqa: E402

ROOT = HERE.parent  # ste100/
LEXICON_DIR = ROOT / "lexicon"
STANDARD_VERSION = "0.1"
HEADER = re.compile(r"^STE100: WeaverTools STE100 writing standard (\S+), lexicon sha256:([0-9a-f]{64})\s*$")
BANNED = ("genuinely", "honestly", "actually")
FENCE = re.compile(r"^\s*(```|~~~)")
UNIT_START = re.compile(r"^\s*(#{1,6}\s+|[-*+]\s+|\d+(\.\d+)*[.)]\s+|>\s*)")


@dataclass
class Unit:
    line: int    # 1-based line of the unit's first line
    text: str


def lexicon_sha256(lexicon_dir: Path = LEXICON_DIR) -> str:
    h = hashlib.sha256()
    for p in sorted(lexicon_dir.iterdir(), key=lambda p: p.name):
        if p.is_file() and p.name != "README.md":
            h.update(p.name.encode() + b"\0" + p.read_bytes() + b"\0")
    return h.hexdigest()


def unresolved_terms(lexicon_dir: Path = LEXICON_DIR) -> set[str]:
    terms = tomllib.loads((lexicon_dir / "terms.toml").read_text())["terms"]
    return {k.lower() for k, v in terms.items() if v.get("status") == "unresolved"}


def units(text: str) -> tuple[list[Unit], list[int]]:
    """The checkable units, and the line numbers of header lines found."""
    out: list[Unit] = []
    headers: list[int] = []
    cur: list[str] = []
    start = 0
    fenced = comment = titled = False

    def flush():
        nonlocal cur
        if cur:
            out.append(Unit(start, "\n".join(cur)))
        cur = []

    for n, raw in enumerate(text.splitlines(), 1):
        if comment:
            comment = "-->" not in raw
            continue
        if raw.lstrip().startswith("<!--"):
            flush()
            comment = "-->" not in raw
            continue
        if FENCE.match(raw):
            flush()
            fenced = not fenced
            if not fenced:
                out.append(Unit(n, "``` ```"))  # a fenced block is one masked span
            continue
        if fenced:
            continue
        if not titled and re.match(r"^# \S", raw):
            flush()
            titled = True  # the document's name
            continue
        if HEADER.match(raw) or raw.startswith("STE100:"):
            flush()
            headers.append(n)
            continue
        if raw.lstrip().startswith("|"):
            flush()
            for cell in raw.strip().strip("|").split("|"):  # a separator row has no words
                if cell.strip():
                    out.append(Unit(n, cell.strip()))
            continue
        if not raw.strip():
            flush()
            continue
        if UNIT_START.match(raw):
            flush()
        if not cur:
            start = n
        cur.append(raw)
        if re.match(r"^\s*#{1,6}\s", raw):
            flush()
    flush()
    return out, headers


def check_document(text: str, lex: ste_check.Lexicon, unresolved: set[str], lexicon_hash: str,
                   version: str = STANDARD_VERSION) -> dict:
    failures: list[dict] = []
    checked = passed = 0
    soft: dict[str, set] = {"unapproved_words": set(), "unknown_words": set()}

    for n, line in enumerate(text.splitlines(), 1):
        bad = sorted({c for c in line if ord(c) > 127})
        if bad:
            failures.append({"line": n, "rule": "ascii", "detail": " ".join(f"U+{ord(c):04X}" for c in bad),
                             "sentence": line.strip()[:200]})

    us, headers = units(text)
    if not headers:
        failures.append({"line": 0, "rule": "header", "detail": "no STE100: header line", "sentence": ""})
    for n in headers:
        m = HEADER.match(text.splitlines()[n - 1])
        if not m:
            failures.append({"line": n, "rule": "header", "detail": "the header line is not in the form",
                             "sentence": text.splitlines()[n - 1][:200]})
            continue
        if m.group(1) != version:
            failures.append({"line": n, "rule": "header",
                             "detail": f"standard version {m.group(1)}, the checker is {version}",
                             "sentence": ""})
        if m.group(2) != lexicon_hash:
            failures.append({"line": n, "rule": "header",
                             "detail": f"lexicon sha256 {m.group(2)}, the lexicon now is {lexicon_hash}",
                             "sentence": ""})

    for u in us:
        masked, _ = ste_mask.mask(u.text, prose_parens=True)
        for s in ste_check.sentences(masked):
            if not re.search(r"[A-Za-z]", ste_check.PLACEHOLDER.sub(" ", s)):
                continue  # only placeholders, numbers or marks: nothing to check
            checked += 1
            source = " ".join(ste_check.PLACEHOLDER.findall(s))
            r = ste_check.check(s, source, lex)
            rules = dict(r["hard"])
            words = {w.lower() for w in ste_check.WORD.findall(ste_check.PLACEHOLDER.sub(" ", s))}
            if words & unresolved:
                rules["unresolved-term"] = sorted(words & unresolved)
            if words & set(BANNED):
                rules["banned-word"] = sorted(words & set(BANNED))
            for k in soft:
                soft[k] |= set(r["soft"].get(k, []))
            if rules:
                for rule, detail in sorted(rules.items()):
                    failures.append({"line": u.line, "rule": rule, "detail": detail, "sentence": s[:200]})
            else:
                passed += 1

    return {"sentences_checked": checked, "sentences_passed": passed, "failures": failures,
            "lexicon_sha256": lexicon_hash, "standard_version": version,
            "soft": {k: sorted(v) for k, v in soft.items()}}


def report(path: str, r: dict) -> str:
    lines = [f"document: {path}",
             f"standard: WeaverTools STE100 writing standard {r['standard_version']}",
             f"lexicon sha256: {r['lexicon_sha256']}",
             f"sentences checked: {r['sentences_checked']}",
             f"sentences that pass: {r['sentences_passed']}",
             f"failures: {len(r['failures'])}"]
    for f in r["failures"]:
        detail = f["detail"] if isinstance(f["detail"], str) else json.dumps(f["detail"])
        lines.append(f"  line {f['line']}: {f['rule']}: {detail}")
        if f["sentence"]:
            lines.append(f"    {f['sentence']}")
    for k, v in r["soft"].items():
        lines.append(f"{k.replace('_', ' ')} (reported, not failing): {len(v)}"
                     + (f": {', '.join(v)}" if v else ""))
    return "\n".join(lines)


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("document")
    ap.add_argument("--verbs", default=str(ROOT / "var" / "approved-verbs.json"),
                    help="the local lexicon that ste_extract.py makes from the standard")
    ap.add_argument("--json", help="also write the result as JSON to this path")
    a = ap.parse_args(argv)
    doc, verbs = Path(a.document), Path(a.verbs)
    if not verbs.is_file():
        print(f"no local lexicon at {verbs}: run ste_extract.py first", file=sys.stderr)
        return 2
    if not doc.is_file():
        print(f"no document at {doc}", file=sys.stderr)
        return 2
    lex = ste_check.Lexicon.load(verbs)
    r = check_document(doc.read_text(encoding="utf-8", errors="replace"), lex, unresolved_terms(),
                       lexicon_sha256())
    print(report(a.document, r))
    if a.json:
        Path(a.json).write_text(json.dumps(r, indent=1) + "\n")
    return 0 if not r["failures"] else 1


if __name__ == "__main__":
    sys.exit(main())
