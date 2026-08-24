# The reasoning loop: IS and IS NOT

**Status:** ARCHIVED 2026-08-23. Never cited by anything but its own companion. Its
IS and IS NOT lists are the part most directly overtaken by the loop taxonomy settled
2026-08-23.

Previously: v0.2, 2026-08-11. Architecture-seat material, outside the document set,
landed by the authoring seat with the discussion's edits applied. Companion to
`reasoning-loop-boundary`, which states the membership criterion, and to
`finding-loop-agent-split`, which records how the boundary was arrived at. This
document states what the thing that now stands is, and what it is not, so that the
vocabulary stops drifting between the loop and the larger construction it sits at
the center of.

**Document ID:** `reasoning-loop-is-and-is-not`
**Editorial:** Per the Working Rules.

Changed from v0.1, each on the operator's ruling of 2026-08-11: the minimum takes
the name the reasoning minimum, the word floor staying owned by the two linked
crates. The harness coordinates rather than orchestrates, the corpus's own pairing,
admin coordinates the load and the harness coordinates the turn, and the banned
word appears only in the entry that refuses it. The network entry narrows to what
the loop does not pay for, the security model living with the role-scoped socket
handoffs in `weaver-harness-PRD` section 5. The individuation entry stands as the
firewall it is.

The subject throughout is the reasoning loop. That is what was built and what is
being worked on. Every line below is decided by one criterion and by nothing else.
Does the thing directly enhance the processing of meaning. If a line cannot be
traced back to that test it does not belong on the page, so each IS NOT is paired
with the reason it fails.

## IS

- The semantic reasoning core. The part that processes meaning, which is the whole
  of what the criterion admits.
- The reasoning minimum: a decoder, a minimal harness, and enough state to hold a
  turn. This is what could not be removed, not what was chosen.
- Extensible by organs that contribute to the meaning being processed. Memory is
  the first extension and perception a later one, both admitted by the same rule
  rather than by a second one written to let them in.
- Coordinated by the harness. Coordination of the turn is the harness's domain,
  held alone, and driving the organs that process meaning is itself the processing
  of meaning, so the criterion admits it. What it drives with is the turn's own
  content, the working structure assembled into what the organs reason over,
  which is what a content-blind scheduler never holds.
- Harness-mediated in shape as well as in domain. Contracts are strictly one to
  one, there are no lateral edges at any level, and siblings route through the
  parent recursively.
- Traced. The trace is the loop's primary artifact, an in-RAM working structure
  that lives and dies with the process, with durability belonging to the emitted
  record.
- Transform-free on the hot path. Payloads splice opaque, and the price of that
  opacity is paid deliberately in exchange for the latency.
- Bounded at load and unload. The load boundary is the only change boundary and no
  in-RAM mutation of behavior is supported in any path.

## IS NOT

- Not where tools live, the bash shell included. A tool supplies material to reason
  over. It transports rather than contributing to the meaning being processed, so
  the criterion puts it on the far side of the gate.
- Not coordinated by any organ other than the harness. Every other organ's domain
  is named in it and coordination is in none of them. The semantic processing unit
  processes meaning. The memory unit manages state. An organ reaching sideways to
  drive another would be working outside its declared domain, which is the same
  fact the no-lateral-edges rule states from the other direction.
- Not the application-level orchestrator. Pulling application concerns together
  toward a purpose is a different job that lives outside the loop entirely, and the
  collision is only in the word, which is why that word appears on this page in
  this entry alone.
- Not decided by where data came from. That test would misfile a visual processing
  unit outside the loop even though a visual organ enhances the processing of
  meaning, so origin describes what the criterion admits and never decides a hard
  case.
- Not decided by hot path versus cold path. Same demotion. Latency is a consequence
  of the membership, not the test for it.
- Not a network participant. No crate exposes a listening port and the hot path
  pays for no network stack. The agent's own edge is the network boundary, per the
  operator's ruling of 2026-08-11, and what crosses that line is outside the agent
  entirely.
- Not where verification happens. Latency bans in-loop checking, so faithfulness is
  held at author time by the vocabulary guard and after emission by the
  observability consumer, per `weaver-harness-PRD` section 5.
- Not durable. Durable substrate-resident state is written through admin, which is
  an egress and sits outside this membership rule.
- Not evidence for individuation. The loop is the apparatus the hypothesis needs in
  order to be tested and it is not a result about the hypothesis. The engineering
  stands on its own and survives the hypothesis failing, which is why this is the
  hypothesis's one appearance in the engineering documents.
- Not the whole of what is being built. Failing the criterion moves a thing outside
  the loop, never outside the program, and outside the loop still has to earn its
  place against a different bar.

## Declared, and built 2026-08-17

The property this section held back is true as of the tool workflow's opening
act, issue #116's arc: the loop is closed at the type level. The granted
`ToolResult` on the harness's surface carries three compile-pinned negatives,
no `Deserialize`, no public construction, no conversion from the
conversation's record block, and is constructed at the execution exchange's
completion alone, with the author door for a tool-result message taking the
grant and the turn's own door refusing a supplied result. The section's
original statement stands below as the intent the act made true.

The loop is meant to be closed at the type level,
such that a tool result entering the loop is obtainable only from the gate, which
makes the boundary a compile-time fact rather than a convention. Only from the
gate means a construction door the crossing alone opens, no public constructor,
no conversion, and no floor deserialization path beside it. The audit of
`weaver-types` and `weaver-traits` that would establish this is owed and
unperformed. It is marked here the way the encoder is marked in the figure,
declared and not built, because writing an owed act as a fact is how a corpus
starts lying to itself.

## What this does not decide

Whether the gate spawns tool processes or whether tools are provisioned as peers
that connect to it, the shape of the tool trait after the reversal, and the audit
named above all belong to the seat holding the codebase. This document fixes
vocabulary and membership. It rules on no mechanism.
