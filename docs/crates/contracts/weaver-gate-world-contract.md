# weaver-gate / world - contract

**Status:** MERGED. In `main` and the source of truth for now. Written on the human's
ruling of 2026-08-01: the two external boundaries take contracts before any Spec is
written, this document and `weaver-admin-operator-contract` in one act, and both
are blockers on the Spec phase. One party is an external principal rather than a
crate, the second instance of the category that ruling settles.

**Date filed:** 2026-08-01
**Revised:** 2026-08-01, second entry. The filename gains the `-contract` suffix on
the human's correction, per `weaver-admin-operator-contract`, and every citation in
the corpus follows the rename in the same act.
**Document ID:** `weaver-gate-world-contract`
**Parent:** `WeaverTools-PRD`, invariant 5.3
**Editorial:** Per the Working Rules.

---

## 0. What this document is

The agreement over the client socket: how the world reaches a loaded agent and how
the agent's responses return. It governs the boundary `weaver-gate-PRD` section 2
names, and it is read alongside that charter and `weaver-harness-gate-contract`.

Three parties act at this boundary and only two sign. The client speaks, the gate
admits and relays, and the harness interprets. The gate stays opaque per its charter,
so the framing of section 2 is an agreement between the client and the harness that
the gate carries without reading, delimiter and octets, never fields. The harness is
bound to this page through `weaver-harness-gate-contract` when the token workflow
charters the turn exchanges, and this document does not restate that seam.

**The filename carries the `-contract` suffix like every contract,** per
`weaver-admin-operator-contract` section 0, whose naming correction of 2026-08-01
this document shares.

```graph
node: weaver-gate-world-contract
kind: document

edge: party
from: weaver-gate-world-contract
to: weaver-gate
```

The second party is the world: a local client principal admitted by the boundary
predicate. The graph carries no node for a principal outside the program, so the
party is named in prose and the missing category rides
`weaver-admin-operator-contract`'s register entry rather than taking a twin.

## 1. The channel

The named Unix socket the gate instruction declares, operator-declared,
admin-validated, resolved and bound by the gate, per `weaver-gate-PRD` section 2. It
exists between raise and lower and at no other time: a connection before ready or
after stopped finds no listener, which is the boundary the lifecycle protects.

**Admission is by peer credential.** Every connection is authenticated by
`SO_PEERCRED` and judged by the `authorization-predicate` of `weaver-types-PRD`
section 2.2, admitting front-end principals only. **The predicate excludes the agent
uid**, so an elected tool cannot dial the agent's own mouth and prompt it through its
own front door. Whoever connects gets to converse with the agent and gets nothing
else.

## 2. What crosses in, and its format

One turn's work: a prompt. **The format is newline-delimited JSON, UTF-8, one request
per line.** The field list of a request is the Spec's. The framing is this document's,
because a client is built against this page and nothing else.

The gate does not parse the line. Delimiting and meaning are the harness's question,
and the gate relays octets in order, per its charter's opacity rule. A line that does
not parse as one JSON value is a refused turn, and the refusal returns by the path
the line took.

## 3. What crosses out, and its format

The turn's close: one JSON line per turn, carrying the response, or carrying the
close reason where no response exists, a stopped turn closing with the stop reason
marked in its place per the grammar of the basic loop. The close names its kind, so
a client can tell a clean close from a stopped one without reading anything else.

The crossing delivers and does not clock. A response returning through this socket
belongs to a turn already closed in the record, per `basic-inference-loop` section 4,
so a client that never receives its line has lost a delivery and not a turn.

**One line in and one line out is the resting shape, and streaming is deferred.** The
token workflow rules streaming, partial output, and whatever else elaborates the
response path, as extensions to this page rather than replacements of it.

## 4. Ordering

- Order is preserved per connection, in both directions.
- A response returns by the path its request took, and by no other.
- One turn is in flight per agent, per the basic loop. What happens to a second
  request while a turn is in flight, and to concurrent clients against the one-turn
  loop, is the token workflow's, deferred rather than settled here.

## 5. Failure

- A peer that fails the predicate is refused at accept, before any content is read.
- A request while the hook is lowered finds no listener, which is refusal by absence
  and not a typed answer.
- A line that does not parse is a refused turn, per section 2.
- A gate death mid-turn is the loss of the delivery and not of the turn: the record
  holds the close, and what the harness does with the death is the coordination
  seam's, per `weaver-harness-gate-contract` section 5.

## 6. Prohibitions

**On the client.** It holds no channel to the program but this socket. It reaches
nothing past the gate, learns nothing of the interior, and receives responses and
refusals and nothing else.

**On the gate.** It reads no content and translates nothing, in either direction. It
retains nothing about a turn after the response returns. It admits no peer the
predicate does not name, and it does not degrade the predicate to a warning.

**On both.** Neither party carries a fact about the other's interior. The client
does not know what serves it and the gate does not know what a prompt means, and the
crossings above are the whole of what either learns.

## 7. Vocabulary

**Drawn from `weaver-types`:** `peer-identity`, `authorization-predicate`,
`gate-instruction`.

**Drawn from `weaver-traits`:** nothing. The clause is present with that answer
because `weaver-types-PRD` section 5 asks for it even when it is empty.

**Drawn from `weaver-trace`:** nothing. No event kind, envelope field, or payload
shape crosses this boundary, and what the record holds about a turn is authored
inside, by the harness, on the other side of the gate.

The turn frame definitions this boundary implies are owed on demand when the token
workflow charters the turn exchanges, and nothing enters `weaver-types-PRD` section
2.3 before that demand fires.

## 8. What this document changes elsewhere

- `weaver-gate-PRD` section 10: the client-boundary cell and the wire-framing cell
  close against this document. Landed in the same act.
- `basic-inference-loop` section 7: the wire-framing cell leaves, settled here at
  the delimiter level with the field shapes staying with the spec seat. Landed in
  the same act.
- `WeaverTools-Document-Format.md`: the external-principal party category rides
  `weaver-admin-operator-contract`'s register entry.
