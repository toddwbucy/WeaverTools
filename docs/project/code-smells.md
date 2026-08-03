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

### 1.2 One socket carrying two services

**Identifier:** `smell-multiplexed-seam`
**Grounds:** `axiom-contract-is-a-complete-interface`, with
`axiom-floor-is-vocabulary-behavior-is-socket`

**The pattern.** A single socket carrying more than one service or modality. An
encoder and a decoder sharing one channel is the reference case. The correct shape is
two sockets, one per service, each with its own contract.

**What it breaks.** Apex section 5.3 requires a contract to name, for each party, the
vocabulary that crosses, the errors it can return, and the ordering guarantees it
relies on and provides. A socket carrying two services forces one of two wrongs: one
contract describing both, or two contracts describing one socket. The first destroys
the property 5.3 states outright, that an agent handed one side of a contract can
build that side without asking what the other side does, because an encoder builder
must now read the decoder's vocabulary, its errors, and its ordering to know which of
them reach the wire it is writing to. The second has no home in 5.1, which governs a
seam by **a** named contract and not by two.

The ordering clause is where it bites first. Two services on one channel share one
ordering regime, so a rule that exists for one constrains the other for no reason a
reader can find. A flush ordering written for decode traffic silently becomes a rule
about encode traffic, and nobody wrote that rule or can say why it holds.

There is a second cost the apex does not have to carry, because the Working Rules
already do. Service is serial per channel, so a multiplexed socket puts a long
operation in front of a short one that shares nothing with it. Latency is the enemy
of agency, and head-of-line blocking between unrelated services is that cost taken
for no gain.

**The corpus already has the correct shape, which is why this is a code smell and not
an open question.** `weaver-spu` holds two channel ends at descriptors 3 and 4 and
declares two `seam` edges to `weaver-harness`, one via `weaver-harness-spu-contract`
and one via `weaver-harness-spu-decode-contract`, both tagged `socket`. The decision
is made. The smell guards it against code that merges what the documents separated.

**How it is found.**

- **The count.** A crate holding fewer channel ends than it has seam edges. The two
  numbers are stated in every organ's Spec and are a direct comparison.
- **The dispatch.** A read loop whose first act is to branch on a field naming which
  service the message is for. That branch is the multiplexing, and it is visible at
  the top of the loop rather than buried.
- **The vocabulary.** One envelope enum whose variants span two contracts. If the
  wire type names both a residency directive and a decode ask, the socket beneath it
  carries both.

**Note for the graph.** Two `seam` edges between the same crate pair is correct here
and a query that treats a crate pair as unique would report this program's own
intended shape as a duplicate. The seam's identity is its `via` contract, not its
endpoints.

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
