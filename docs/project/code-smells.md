# WeaverTools code smells

**Status:** RUNNING. Opened 2026-08-03 alongside the axiom layer. Entries accumulate
as they are found and nothing here is retired without a note saying why.

**Date filed:** 2026-08-03
**Document ID:** `code-smells`
**Parent:** `WeaverTools-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

A smell is a pattern the code must avoid. It is not an assertion, and the difference
is the direction it arrives from. An assertion is a claim a Spec makes and code must
satisfy, authored top-down from a document. A smell is recognized bottom-up, either
because an invariant makes a whole class of construction wrong or because the same
defect has now been met more than once.

**This document declares no graph records.** The format carries no node kind for a
smell and no edge from one, and inventing both before any code exists would fix a
shape against nothing. Each entry below names the axiom or rule it falls out of, so
the edges are decided when there is code to draw them against.

**Entries are stated as detections, not as advice.** "Prefer composition" is not a
smell. A smell says what pattern to look for, what it breaks, and how a reader or a
query would find it, because a smell that cannot be looked for is a preference.

---

## 1. Smells that fall out of an invariant

These need no experience to justify. The invariant makes the construction wrong, and
the smell is the code-level shadow of a rule the document layer already carries.

### 1.1 A peer organ called directly, bypassing the harness

**Identifier:** `smell-peer-organ-bypass`
**Grounds:** `axiom-organ-and-submodule`, with
`axiom-floor-is-vocabulary-behavior-is-socket`

**The pattern.** One organ reaching another organ without going through the harness -
a Cargo dependency from one organ crate to another, a socket dialed at a path that
resolves to a peer, or a descriptor for a peer's channel held by anything other than
the harness that created the pair.

**What it breaks.** Apex section 5.4 makes the harness the organ whose domain is
coordination, and the hub every other organ is duplex with rather than a spoke. A
peer call is an edge the topology does not have. Section 5.1 then leaves it nowhere
to live: every seam where one crate asks another process to do something is a socket
governed by a named contract, and a peer call has no contract because no contract was
written for a seam the architecture does not admit. So the call is either an
undeclared seam or a path dependency across a process line, and 5.1 forbids both.

The cost is not stylistic. The harness is the sole author of the trace, and a turn
that includes an exchange the harness never brokered produces a trace with a hole in
it that no one can see, because the missing span was never anyone's to write. That
makes the deliverable wrong rather than merely the layering.

**How it is found.** Three ways, cheapest first:

- **The manifest.** An organ crate naming another organ crate as a dependency. This
  is the whole of the static case and it costs one read of seven `Cargo.toml` files.
- **The graph.** A `seam` edge between two non-harness organs, which phase two's
  closing checklist item 4 already queries for as "no lateral edges." The document
  layer is therefore already covered, and this entry exists because code can grow an
  edge the documents never declared.
- **The socket path.** A dial to any path not named by a contract this crate is party
  to. The organ channel case needs no check, since a channel with no name is reachable
  only by the party handed the descriptor, which is the authentication.

**Note on scope.** A submodule reaching its own organ is not this smell. Section 5.4
leaves the shape of that channel unconstrained and makes it the organ's business, so
a submodule-to-organ call is in bounds by construction and only an organ-to-organ
call is the pattern.

---

## 2. Smells earned from repetition

None yet. Entries land here when a defect has been met more than once and the
second sighting is what promotes it, so the entry can name both.

The document-side corpus of this kind already exists and is not repeated here: the
rules the assertion pass settled on 2026-08-03, chiefly that a watch which cannot
fail is not a test, that an enumeration must render the count it carries, and that a
tag follows the mechanism a clause names rather than the heading it sits under. Those
govern documents. Their code equivalents will accumulate the same way once there is
code, and this section is where they land.
