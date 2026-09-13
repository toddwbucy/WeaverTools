#!/usr/bin/env python3
"""Cut the few files that exceed the embedder's window, at semantic seams.

**Most files need nothing.** 245 of 255 tracked units fit a 32,768-token
window whatever the tokenizer says, so this is not a chunking strategy for a
corpus - it is a held-out list for the handful that do not fit, and it exists
because a one-size cut through those files would break them mid-thought.

Three rules, in the operator's terms of 2026-09-13:

- **A boundary is a semantic seam.** The end of a function, the end of a
  section, never the middle of either. Where one unit is itself too large the
  cut recurses into it - a `mod tests` becomes its test methods - which is the
  same rule applied one level down rather than a different rule.
- **The overlap is one whole unit, repeated.** A sliding window repeats two
  half-thoughts and a boundary-aligned one repeats a single whole one, so both
  neighbours hold the argument entire and nothing is duplicated that does not
  buy continuity.
- **A graph fence is atomic.** A ```graph block carries `node`/`kind`/`tag`
  and `edge`/`from`/`to` stanzas, and a cut inside one yields a piece holding
  half a declaration. No boundary falls inside a fence.

**Offsets rather than files on disk**, so a Spec edit does not silently
desynchronise from pieces cut before it. **The content hash is what makes that
safe**: offsets against changed content are worse than no offsets, because
they are wrong rather than absent. An ingester applying this manifest refuses
where the hash does not match and says which file moved.

**Read from a ref and never from the working tree.** The ingest runs against
what is merged, so a manifest hashed from a branch carries offsets for content
that may never reach main, and `--check` fails on every branch that touches a
held-out file for no reason. Measured on 2026-09-13: the working tree held one
document main did not, which read as a file missing from the graph until the
ref was named.

    python3 process/ingest/chunk_plan.py                    # write, at main
    python3 process/ingest/chunk_plan.py --check --ref HEAD # in an act
    python3 process/ingest/chunk_plan.py --check            # against main

**In an act, name HEAD.** The default ref is `main`, which is right for
building the manifest the ingest reads and wrong for checking one: the edit
being checked is the act's own and `main` does not have it, so the default
reports current on exactly the branch that moved it. HEAD is a commit rather
than the working tree, so this runs after committing.
"""

import hashlib
import json
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MANIFEST = os.path.join(os.path.dirname(os.path.abspath(__file__)), "chunk-plan.json")

# The embedder's ceiling, and the target this cuts to. The gap is headroom for
# the tokenizer disagreeing with the ratio below, which it will.
#
# **The hold-out test uses TARGET and not CEILING**, per the olympus review of
# 2026-09-13. Testing at the ceiling applied the headroom to the pieces of a
# file already held out and never to the decision about which files to hold,
# so `crates/weaver-spu/src/main.rs` at about 31,844 estimated tokens - 2.8
# percent under the ceiling - was left to the embedder. A file inside the
# ratio's own error is exactly the file the headroom exists for.
CEILING = 32768
TARGET = 28000

# Characters per token, per kind. **These are estimates and the manifest says
# so.** Two of the eight files sit within three percent of the ceiling, which
# is inside the error of any ratio, so the honest artifact records what it
# assumed rather than presenting a count it did not make.
RATIO = {".rs": 3, ".md": 4, ".toml": 4}

# **A visibility is `pub`, `pub(crate)`, `pub(super)` or `pub(in ...)`.** The
# first form of this took a bare `pub ` and nothing else, which made
# `pub(crate) struct GatePort` invisible.
_VIS = r"(?:pub(?:\([^)]*\))?\s+)?"
# **A qualifier stack and not one optional word.** `pub const unsafe fn` is
# four tokens before the keyword, and the first form allowed only `async`.
_QUAL = r"(?:(?:async|const|unsafe|extern\s+\"[^\"]*\"|default)\s+)*"
# **`impl` is followed by `<` as often as by a space.** `impl<'a> Ports<'a>`
# missed on that alone, which is most of why a 157 KB file offered five seams.
_ITEM = r"(?:fn|impl|mod|struct|enum|trait|union|type|const|static)\b"

UNIT = {
    ".rs": rf"^{_VIS}{_QUAL}{_ITEM}",
    ".md": r"^## ",
}
INNER = {".rs": rf"^    {_VIS}{_QUAL}(?:fn|impl|mod|struct|enum|trait)\b"}
FENCE = re.compile(r"```graph.*?```", re.S)

# **A run of attributes and doc comments belongs to the item below it.** The
# regexes above anchor on the keyword, so a cut at the match left `#[test]` in
# the previous piece and opened the next one at a bare `fn` - realized in the
# manifest and not merely possible, at 26 of 31 unit starts in `lifecycle.rs`.
# Severing an item from its attribute is the middle of a thought, which the
# first rule in this module's docstring forbids.
# **No `^` anchor.** This is matched with an explicit `pos` at a known
# line start, and Python anchors `^` to the real start of the string
# rather than to `pos` - so the first form of this never matched and
# `back_up` was a no-op that looked like a fix.
ATTACHED = re.compile(r"[ \t]*(?:#!?\[|///|//!)")

# The fewest seams a held-out file may offer before the cut is a guess. **Five
# seams in 157 KB is a piece of luck rather than a structural cut**, which is
# what `engine.rs` had under the narrower regex above.
MIN_SEAMS = 8


DEFAULT_REF = "main"


def parse_args(argv):
    """The flags, or a message and no run.

    **An unrecognised flag refuses.** The first form fell through to the write
    branch for anything that was not exactly `--check`, so `--chek` rewrote
    the manifest and exited 0 where the caller asked to verify it, and `--help`
    rewrote it too. **`--ref` with nothing after it refuses** rather than
    silently meaning `main`, which reported a reading for a ref the caller
    never named.
    """
    opts = {"check": False, "ref": DEFAULT_REF}
    rest = list(argv)
    while rest:
        a = rest.pop(0)
        if a == "--check":
            opts["check"] = True
        elif a == "--ref":
            if not rest:
                return None, "--ref needs a value"
            opts["ref"] = rest.pop(0)
        else:
            return None, f"unrecognised argument: {a}"
    return opts, None


OPTS = {"check": False, "ref": DEFAULT_REF}


def ref():
    return OPTS["ref"]


def resolve(rev):
    """The ref as a commit, or a message saying it is not one here.

    **A fresh clone has no local `main`.** In a CI checkout the branch is a
    detached HEAD and `main` exists only as `origin/main`, where the first
    form raised `CalledProcessError` and exited 1 - the same code staleness
    uses, so a misconfigured job read as a stale manifest.
    """
    for cand in (rev, f"origin/{rev}"):
        out = subprocess.run(["git", "-C", ROOT, "rev-parse", "--verify", "-q",
                              f"{cand}^{{commit}}"], capture_output=True, text=True)
        if out.returncode == 0:
            return out.stdout.strip(), cand, None
    return None, None, (f"cannot resolve ref {rev!r} here, and no origin/{rev} "
                        "either. Fetch it, or name one with --ref.")


# **Archived text is not this plan's to cut.** The ingest excludes `**/archive/`
# per issue #565, so a held-out archive file would carry a cut plan for content
# nothing reads. No archive stands in the tree from 2026-09-13, and this is the
# guard for a ref older than that, which `--ref` makes reachable.
ARCHIVED = "/archive/"


def tracked():
    out = subprocess.run(
        ["git", "-C", ROOT, "ls-tree", "-r", "-z", "--name-only", ref()],
        capture_output=True, text=True, check=True,
    ).stdout
    return [f for f in out.split("\0")
            if f.endswith((".rs", ".md", ".toml")) and ARCHIVED not in f"/{f}"]


def read_bytes(rel):
    """The raw blob at the ref, undecoded and unconverted.

    **No `text=True`.** It decodes the blob and converts newlines, so the
    length and every offset taken from it counted characters while the manifest
    declared bytes, and the hash was over converted text rather than over what
    git holds. A single multibyte character before a boundary made a consumer
    slicing bytes cut in the wrong place, and nothing in the manifest could say
    so - the hash matched, being computed from the same decoded copy. Found by
    CodeRabbit on PR #562, and raised unfixed by this act's own self-review.
    """
    return subprocess.run(["git", "-C", ROOT, "show", f"{ref()}:{rel}"],
                          capture_output=True, check=True).stdout


def read(rel):
    """The blob decoded, for matching only. Offsets come from `to_bytes`."""
    return read_bytes(rel).decode("utf-8")


def to_bytes(text, at):
    """A character offset as the byte offset the manifest promises."""
    return len(text[:at].encode("utf-8"))


def fences(text):
    return [(m.start(), m.end()) for m in FENCE.finditer(text)]


def back_up(text, at):
    """Move a boundary above the attribute and doc-comment run that leads it.

    **The item's attributes are part of the item.** Cutting at the keyword
    leaves `#[test]` in one piece and opens the next at a bare `fn`, which is
    what `lifecycle.rs` piece 3 did before this existed.
    """
    while at > 0:
        prev = text.rfind("\n", 0, at - 1) + 1
        if not ATTACHED.match(text, prev, at):
            return at
        at = prev
    return at


def units(text, pattern, inner, cpt):
    """Structural units, recursing into any that exceed the target alone."""
    guard = fences(text)
    starts = [back_up(text, m.start()) for m in re.finditer(pattern, text, re.M)
              if not any(a < m.start() < b for a, b in guard)]
    # Backing up can collide two starts onto one offset, and can walk one
    # above a preceding start when a doc comment runs long.
    starts = sorted(set(starts))
    if not starts or starts[0] != 0:
        starts = [0] + starts
    starts.append(len(text))
    out = []
    for i in range(len(starts) - 1):
        off, size = starts[i], starts[i + 1] - starts[i]
        if size / cpt <= TARGET or not inner:
            out.append((off, size))
            continue
        # **The inner pass backs up too.** The first form applied `back_up`
        # to the outer matches only, which left one dangling `#[test]` at the
        # end of `lifecycle.rs` piece 2 - the recursion into `mod tests` being
        # exactly where attributes are densest.
        deep = [back_up(text, m.start() + off)
                for m in re.finditer(inner, text[off:off + size], re.M)]
        deep = sorted({off} | {d for d in deep if off < d < off + size}) + [off + size]
        out.extend((deep[j], deep[j + 1] - deep[j]) for j in range(len(deep) - 1))
    return out


def pieces_for(rel):
    ext = os.path.splitext(rel)[1]
    raw = read_bytes(rel)
    text = raw.decode("utf-8")
    cpt = RATIO[ext]
    if len(raw) / cpt <= TARGET:
        return None
    us = units(text, UNIT.get(ext, r"^## "), INNER.get(ext), cpt)
    if len(us) < MIN_SEAMS:
        # **Reported rather than shipped.** A held-out file the cutter can see
        # few seams in is one it is guessing at, and a plan that says nothing
        # reads the same as a plan that worked.
        print(f"warning: {rel} offers {len(us)} seams, fewer than {MIN_SEAMS}; "
              "its cut is not structural", file=sys.stderr)
    out, cur = [], []
    for off, size in us:
        if cur and (sum(s for _, s in cur) + size) / cpt > TARGET:
            out.append(cur)
            # **The overlap is one whole unit, repeated - unless repeating it
            # will not fit.** The first form appended the new unit after the
            # overlap with no budget test, so a piece was bounded by two units
            # rather than by TARGET and three adjacent 27,900-token sections
            # gave pieces of 55,800. Continuity is worth a repeat and is not
            # worth a piece the embedder truncates.
            prev = cur[-1]
            cur = [prev] if (prev[1] + size) / cpt <= TARGET else []
        cur.append((off, size))
    if cur:
        out.append(cur)
    # **Offsets converted once, here.** Everything above matched on the decoded
    # copy because that is what the regexes need; what the manifest publishes is
    # the byte offset a consumer slices with.
    pieces = [
        {"start": to_bytes(text, p[0][0]),
         "end": to_bytes(text, p[-1][0] + p[-1][1]),
         "units": len(p),
         "estimated_tokens": sum(s for _, s in p) // cpt}
        for p in out
    ]
    # **The one property this file exists to guarantee, asserted.** A manifest
    # naming a piece the embedder will truncate is worse than no manifest,
    # because it looks like a plan.
    over = [q for q in pieces if q["estimated_tokens"] > CEILING]
    if over:
        raise SystemExit(
            f"{rel}: {len(over)} piece(s) exceed the {CEILING}-token ceiling: "
            + ", ".join(f'{q["start"]}..{q["end"]} at {q["estimated_tokens"]}'
                        for q in over)
        )
    return {
        "path": rel,
        "bytes": len(raw),
        "content_sha256": hashlib.sha256(raw).hexdigest(),
        "chars_per_token_assumed": cpt,
        "pieces": pieces,
    }


def build():
    held = [pieces_for(f) for f in sorted(tracked())]
    held = [h for h in held if h]
    commit, _, _ = resolve(ref())
    return {
        "ref": commit,
        "ceiling_tokens": CEILING,
        "target_tokens": TARGET,
        "overlap": "one whole structural unit, repeated at each boundary",
        "estimates": "token counts are characters divided by the per-kind ratio "
                     "and have not been measured with the embedder's tokenizer",
        "held_out": held,
    }


def main():
    opts, bad = parse_args(sys.argv[1:])
    if bad:
        print(f"chunk_plan: {bad}", file=sys.stderr)
        for line in __doc__.strip().splitlines()[-3:]:
            print(line.strip(), file=sys.stderr)
        return 2
    OPTS.update(opts)

    # **Resolved before any work.** An unresolvable ref used to surface as a
    # CalledProcessError traceback exiting 1, which is the code staleness
    # uses, so a CI job pointed at a ref it had not fetched read as a stale
    # manifest.
    commit, used, why = resolve(ref())
    if why:
        print(f"chunk_plan: {why}", file=sys.stderr)
        return 2
    if used != ref():
        print(f"note: {ref()} is not a local ref here, reading {used}",
              file=sys.stderr)
    OPTS["ref"] = used

    plan = build()
    if opts["check"]:
        if not os.path.exists(MANIFEST):
            print("no manifest, so run without --check", file=sys.stderr)
            return 2
        with open(MANIFEST, encoding="utf-8") as fh:
            was = json.load(fh)
        # **The ref is recorded and not compared.** A manifest built at main is
        # valid for any ref whose held-out content is identical, and comparing
        # the field would report "content changed" for a commit that changed
        # nothing this manifest describes.
        if was.get("ref") != plan["ref"]:
            print(f"manifest built at {was.get('ref','?')[:12]}, "
                  f"checking {plan['ref'][:12]}")
        if {k: v for k, v in was.items() if k != "ref"} != \
           {k: v for k, v in plan.items() if k != "ref"}:
            # **`.get` and not `[...]`.** A truncated manifest raised KeyError
            # inside the branch that exists to report a bad manifest, so the
            # one input this path is written for crashed it.
            moved = ({h["path"] for h in plan["held_out"]}
                     ^ {h.get("path") for h in was.get("held_out", [])})
            print("the manifest is stale.", file=sys.stderr)
            if moved:
                print(f"files entering or leaving the held-out set: {sorted(moved)}",
                      file=sys.stderr)
            else:
                print("the same files, with content or offsets changed.",
                      file=sys.stderr)
            return 1
        print(f"current: {len(plan['held_out'])} files held out")
        return 0
    with open(MANIFEST, "w", encoding="utf-8") as fh:
        json.dump(plan, fh, indent=2)
        fh.write("\n")
    for h in plan["held_out"]:
        sizes = ", ".join(f"{p['estimated_tokens']:,}" for p in h["pieces"])
        print(f"{h['path']}  {len(h['pieces'])} pieces  [{sizes}]")
    print(f"\n{len(plan['held_out'])} files held out of {len(tracked())} tracked")
    return 0


if __name__ == "__main__":
    sys.exit(main())
