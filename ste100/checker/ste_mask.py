"""Masking of code-like spans, copied from WeaverTrain's scripts/ste_convert.py at
ab63a918 (lines 47-180: SPAN_PATTERNS, IDENT, opens_string, mask_brackets,
mask_listings, mask, unmask). Fenced blocks, inline code, quotations, quoted strings,
URLs, paths, file names, calls and literals with brackets, and dotted and snake_case
names become [[Cn]] placeholders, so the checker reads only the prose around them.

One change from the copy, for documents: mask(text, prose_parens=True) keeps a
parenthesis that follows a space or starts a line as prose, so a parenthetical
sentence is checked and not hidden. A call such as f(x) is still masked. The
conversion uses the default, prose_parens=False, which is the copied behaviour.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import ste_check  # noqa: E402

SPAN_PATTERNS = [
    re.compile(r"```.*?```", re.S),                       # fenced code
    # Markdown blockquote lines, run together: a quotation (of the issue, of output)
    # is someone else's text and stays as written, nested quotes and all
    re.compile(r"(?m)^[ \t]*>[^\n]*(?:\n[ \t]*>[^\n]*)*"),
    re.compile(r"`[^`\n]+`"),                             # inline code
    re.compile(r"https?://\S+"),                          # URLs
    # an exception name and its message, to the end of the line: program output
    re.compile(r"\b[A-Z]\w*(?:Error|Exception|Warning)(?:: [^\n]+|:(?=\n|$))"),
    # HTML tags (<details>, </summary>, <br/>); the prose between tags stays
    re.compile(r"</[A-Za-z][\w-]*\s*>|(?<![\w<])<[A-Za-z][\w-]*(?:\s+[^<>\n]*)?/?>"),
    re.compile(r'"\s*"'),                                 # empty or blank string literal
    # double-quoted strings, escape-aware; a backslash before a quote that ends the span
    # (space, punctuation, end) is content, as in "C:\tmp\", not an escape. The text
    # inside does not start or end with a space, so in 'message "Python int ..." when'
    # inside a longer quotation the inner string pairs, not the outer quote's halves.
    re.compile(r'"(?!\s)(?:[^"\\\n]|\\"(?![\s;,.:)\]}]|$)|\\[^"\n]|\\(?=")){1,300}(?<!\s)"'),
    re.compile(r"(?<![\w/])(?:\.{0,2}/)?(?:[\w.-]+/)+[\w.-]*"),  # paths with a slash
    re.compile(r"\b[\w-]+\.(?:py|pyx|pyi|txt|md|rst|toml|json|ya?ml|cfg|ini|sh|c|h|cpp|rs|js|ts|csv)\b"),
]
IDENT = re.compile(r"\b(?:[A-Za-z_]\w*\.)+[A-Za-z_]\w*\b|\b\w*_\w*\b")  # dotted or snake_case


def opens_string(text: str, j: int) -> bool:
    """A quote at j opens a string literal unless it is an apostrophe inside a word
    (don't, it's); a string prefix (r'', b'', f'', rb'') still opens one."""
    k = j
    while k > 0 and (text[k - 1].isalnum() or text[k - 1] == "_"):
        k -= 1
    word = text[k:j]
    return not word or word.lower() in {"r", "b", "f", "u", "rb", "br", "fr", "rf", "e"}


def mask_brackets(text: str, codes: list[str], prose_parens: bool = False) -> str:
    """Mask a word followed by a bracketed span on one line: f(x, y), x[0], {'a': 1}.
    With prose_parens, a "(" with no word before it opens prose and is not masked."""
    out, i = [], 0
    opener = re.compile(r"[\w.]*[(\[{]")
    while True:
        m = opener.search(text, i)
        if not m:
            out.append(text[i:])
            return "".join(out)
        if prose_parens and m.start() == m.end() - 1 and text[m.start()] == "(":
            out.append(text[i:m.end()])  # "(" after a space: a prose aside, checked
            i = m.end()
            continue
        ph = ste_check.PLACEHOLDER.match(text, m.end() - 1)
        if ph and m.start() == m.end() - 1:  # an earlier placeholder: copy it whole
            out.append(text[i:ph.end()])
            i = ph.end()
            continue
        depth, j = 0, m.end() - 1
        pairs = {"(": ")", "[": "]", "{": "}"}
        stack, quote = [], None
        while j < len(text) and text[j] != "\n":
            c = text[j]
            if quote:  # inside a string literal, brackets are content
                if c == "\\":
                    j += 2
                    continue
                if c == quote:
                    quote = None
            elif c in "'\"" and stack and opens_string(text, j):
                quote = c
            elif c in pairs:
                stack.append(pairs[c])
            elif stack and c == stack[-1]:
                stack.pop()
                if not stack:
                    break
            j += 1
        if stack or j >= len(text) or text[j] == "\n":
            out.append(text[i:m.end()])
            i = m.end()
            continue
        span = text[m.start():j + 1]
        if re.fullmatch(r"\(\w+(['\u2019]\w+)?( \w+(['\u2019]\w+)?){0,3}\)", span):  # prose aside "(it's a list)": keep
            out.append(text[i:m.end()])
            i = m.end()
            continue
        codes.append(span)
        out.append(text[i:m.start()] + f"[[C{len(codes)}]]")
        i = j + 1


LISTING_LINE = re.compile(r"[\w.\-\[\]]+ *:(?: .*)?")   # "name : value" (blank value allowed)
ALIGNED = re.compile(r"^\S+ {2,}:")                   # padded to a column: program output


def mask_listings(text: str, codes: list[str]) -> str:
    """Mask program output listings such as pandas' "INSTALLED VERSIONS": a run of
    "name : value" lines (blank lines inside allowed) in which at least 3 lines pad the
    name to a column. Prose labels ("File: x", "Lines: 3-4") are not padded and stay."""
    lines = text.split("\n")
    out, i = [], 0
    while i < len(lines):
        if not LISTING_LINE.fullmatch(lines[i]):
            out.append(lines[i])
            i += 1
            continue
        j = i
        while j < len(lines) and (LISTING_LINE.fullmatch(lines[j]) or not lines[j].strip()):
            j += 1
        while j > i and not lines[j - 1].strip():  # trailing blank lines stay outside
            j -= 1
        block = lines[i:j]
        if sum(bool(ALIGNED.match(ln)) for ln in block) >= 3:
            codes.append("\n".join(block))
            out.append(f"[[C{len(codes)}]]")
        else:
            out.extend(block)
        i = j
    return "\n".join(out)


def mask(text: str, prose_parens: bool = False) -> tuple[str, list[str]]:
    codes: list[str] = []

    def sub(m):
        codes.append(m.group(0))
        return f"[[C{len(codes)}]]"

    text = mask_listings(text, codes)
    for pat in SPAN_PATTERNS:
        text = pat.sub(lambda m: m.group(0) if m.group(0).startswith("[[C") else sub(m), text)
    text = mask_brackets(text, codes, prose_parens)
    text = IDENT.sub(lambda m: sub(m) if not re.fullmatch(r"\d+(\.\d+)*", m.group(0)) else m.group(0), text)
    return text, codes


def unmask(text: str, codes: list[str]) -> str:
    # restore in reverse so [[C1]] is not matched inside [[C12]]; nested placeholders
    # (a span masked inside an earlier one cannot happen: earlier spans are replaced first)
    for n in range(len(codes), 0, -1):
        text = text.replace(f"[[C{n}]]", codes[n - 1])
    return text

