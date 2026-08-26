# weaver-analysis / weaver-state - contract

**Status:** MERGED. In `main` and the source of truth.

**Revised:** 2026-08-26, first of this date, the name moves behind the wall.
The operator's ruling of
this date recuts the wire clause: the first door loses its name, making this
door the member's one named socket, and this door's name moves from the worker's
runtime directory into the member's own territory on the operator's side, where
the agent's identity holds nothing and the driver, an operator principal,
traverses. The election, the seal, the refusals, and every ask are untouched.
`weaver-analysis` still learns the name from the operator, no exchange carrying
a path.
**Revised:** 2026-08-24, second of this date, the sender is `weaver-analysis`.
The operator's ruling of this date moved `weaver-diagnostic` inside the agent
as the harness's third member, the mechanism the harness authors a
diagnostic-trace through, which vacated the sending side of this seam: an
inside crate cannot dial a door as an operator principal over the operator's
own storage. `weaver-analysis`, chartered in the same act, takes the party and
the document takes its name, a contract being named for its parties with the
initiator first. Nothing of the mechanism moves - the door, the seal, the
election, and the refusals are as they were, and the far side never knew which
crate held the near one. The seam edge relocates to `weaver-analysis-PRD` with
the party, the from side's charter carrying the edge.
**Revised:** 2026-08-24, first of this date, the seal ends the preload. Section 2 gains
the
seal, one empty frame after the last distillate, because a close looks the
same from a finished sender and a dying one and the replay ask of
`weaver-harness-state-contract` answers at the seal, which must not happen
over a prefix that looks whole. Section 5's dead-driver clause names the
prefix unsealed. Landed with the replay loop's act, every party merging.
**Revised:** 2026-08-26, second of this date, the seam states its mechanics.
Two facts the code act of this date elected are pinned where the seam's
parties read them, per the audit of the same date: the seal's spelling is
the empty JSON object on its own line, a bare line being framing residue
and not a seal, and the preload door re-stands after any close of its
channel, which is what carries the dead-driver retry and scopes the
at-most-one-preload owing to a live driver. Nothing of the traffic moves.
**Date filed:** 2026-08-24
**Document ID:** `weaver-analysis-state-contract`
**Editorial:** Per the Working Rules.

---

## Parties

- **`weaver-analysis`, the driver and the only sender.** Parses the finished
  trace outside the agent, as an operator principal over the operator's own
  storage, and sends across this seam what the parse projects. Decides what to
  send from the record's content and the election it declares, and asks
  nothing. It is also what reads the diagnostic-trace the run produces, per
  `weaver-analysis-PRD` section 1, so one crate stands at both ends of the
  replay and neither end is inside the agent.
- **`weaver-state`, the custodian.** The same member, the same charter, the
  same store, a second door. Receives the preload, holds it organized, and
  speaks at no time on this seam. What it holds here is answered on the other
  door, to the harness, exactly as if a tee had landed it.

No third party reaches this seam. The agent holds no end of it: the harness
never dials this door, the model has no path to it, and the door refuses the
agent's own credential at the accept. The one ruled crossing into the agent
that is not the two external contracts of 2026-08-01, per `weaver-agents-PRD`
section 0 as amended 2026-08-24, and it exists only where the load declared
the diagnostic kind.

**This seam is a wire.** A named Unix socket on the state member, the member's
one named door since the first lost its name to the operator's ruling of
2026-08-26, stood only under a diagnostic binding and authenticated by
credential per the first invariant's rule for a channel with a name. Its name
stands in the member's own territory on the operator's side, per the same
ruling: the driver is an operator principal and traverses, the worker's
identity holds nothing there, and the squat an agent-writable directory
invited is unrepresentable rather than defended. The door's absence under a
serving binding is the charter's cheap refusal made structural: a driver
pointed at a serving agent finds nothing to dial.

```graph
node: weaver-analysis-state-contract
kind: document

edge: party
from: weaver-analysis-state-contract
to: weaver-analysis

edge: party
from: weaver-analysis-state-contract
to: weaver-state
```

## Vocabulary

Every contract names the vocabulary it depends on, grouped by where it is
defined, and a group is stated even when empty.

**From `weaver-harness-state-contract`.** The `election` and the
`distillate`, defined there and drawn here rather than restated, one
authority per gate G5. The preload is the ingest direction of that contract
re-run with a different author, and a second definition of its nouns would
give the mapper two sources for one record. What those terms mean does not
shift with the author: the election is the opener declaring the session and
the elected kinds with their payload key paths, and the distillate is one
distilled event, envelope whole and elected pairs beside it.

**From `weaver-trace`.** The canonical event JSON and its envelope fields,
spelled as that crate's canonical form spells them. The driver reads the
record itself, outside the agent, and its distillates are projections of
canonical events exactly as the tee's are: every name that crosses this seam
is a name the record already carries.

**From `weaver-types`.** Nothing crosses. The binding kind conditions this
door's existence and never rides it, the declaration having done its work at
the load, per `weaver-agents-PRD` section 6.

**This seam's own.** Nothing. A seam whose whole vocabulary is drawn is the
point rather than an omission: the preload's claim is that what it lands is
indistinguishable in the holdings from what a live tee would have landed,
and a term of its own would be a place for that claim to quietly fail.

```graph
edge: draws
from: weaver-analysis-state-contract
to: election

edge: draws
from: weaver-analysis-state-contract
to: distillate

edge: draws
from: weaver-analysis-state-contract
to: canonical-event
```

## 1. What this contract governs

The second door on the state member: the preload traffic that flows when a
diagnostic binding stands, the condition under which the door exists, what
each party owes, how the seam fails, and what neither party may do. It is
read alongside `weaver-analysis-PRD` and `weaver-state-PRD` section 3, and
none of the three is complete without the others.

## 2. The traffic

**Preload, flowing, one direction, and the election opens it.** The first
traffic on every standing of the channel is the election itself, whole, with
the session it declares being the replayed session's own name: the holdings
the loop later asks against must answer as that session, and the serve
restriction on the other door binds to the opener's session, so a preload
declaring anything else would land holdings no ask can reach. After the
opener, the driver sends a `distillate` per event it elects, in the record's
sequence order, and is owed nothing back: the fact has one home and a
confirmation whose one reader would discard it is the retired receipt's
error, the same on this door as on the first.

**The seal ends the preload, and the close alone does not.** After the last
distillate the driver sends the seal, one frame carrying nothing, and then
closes. **Carrying nothing is spelled**: the empty JSON object, one line
reading `{}`, every frame on this seam being a JSON object on its own line.
A bare empty line is a sender's framing residue and not a seal, so a driver
that sealed with a blank line has not sealed and its close reads as a dying
sender's. A close without the seal is a dead driver, per section 5, and the
distinction is the whole of the seal's job: a channel's close looks the same
from a finished sender and a dying one, and the party waiting on the
preload, the replay ask of `weaver-harness-state-contract` section 2, must
not answer over a prefix that looks whole. One standing, one preload, one
seal, and the driver owes at most one preload per standing of the member.
**The owing is a live driver's, and the door outlives any one driver**: the
member stands the name again after any close of the preload channel, sealed
or sealless, for its own standing's life, which is the mechanism the
dead-driver retry of section 5 rides - a retry is a new standing of the
driver against the same standing of the member, its opener retiring the
dead prefix, and a driver that preloads twice on one of its own standings
has broken its owing whether or not the member's door would admit it.

**The opener retires the declared session's prior holdings, in the same
transaction that records it.** A preload therefore lands against empty and
never beside anything, whatever the store held for that session, a whole
earlier preload or a dead driver's prefix. That is what makes a retry a
replacement rather than a double, and it is the record re-asserting itself
rather than custodial judgment: the preload is the session's account
projected from the authoritative record, so what it replaces was at best an
older projection of the same authority. The retirement is this door's
opener's and no other traffic performs it - the first door never retires
anything, per its own contract.

**Completeness is not this seam's judgment.** The record is the authority on
what a whole preload would have carried, and whether the holdings match it is
the certification's question, per `weaver-diagnostic-PRD` section 4, answered
from the record and the shape ask rather than by any confirmation crossing
here. A driver that died mid-preload leaves a prefix, and the prefix is
custody doing its job on what arrived.

**No asks cross this door, ever.** The ask vocabulary lives on the harness
door alone, and the who-else-may-ask cell of `weaver-state-PRD` section 5
stands untouched: the driver is a sender and never an asker, so the serve
direction still has exactly the two ends it had.

## 3. What the driver owes

- **The parse kept outside.** State never opens a trace file, per its charter
  and the custody rule: what crosses is distillate-shaped already, the
  driver having read the operator-held record as an operator principal.
- **The envelope always.** Every distillate carries all five envelope fields
  as the canonical form spelled them, in the record's sequence order. An
  unattributable distillate is a defect in the sender, on this door as on
  the first.
- **The election faithful to what follows.** What the opener declares is what
  the stream delivers: kinds outside the election do not cross, and the
  declared session is the replayed session's name.
- **Its own credential.** The driver dials as an operator principal and never
  as the agent, and a driver that cannot present that credential has no
  business this contract recognizes.

## 4. What state owes

- **Custody whole, indistinguishably.** What the preload lands is held,
  organized, and attributable exactly as a tee's landing would be, one store
  and one set of obligations, so the loop's asks on the other door answer
  against it with nothing marking how it arrived. That indistinguishability
  is this contract's whole point and the charter's inversion made real: same
  organ, opposite direction.
- **The refusal at the accept.** A peer bearing the agent's credential is
  refused before any byte is read, this member's one credential judgment
  since the first door authenticates by possession, and the door itself
  stands only where the binding declared the diagnostic kind.
- **Silence.** No traffic flows driver-ward, at any time, for any reason.

## 5. Failure vocabulary

**A dead driver costs the preload and never the holdings.** A distillate
lands whole or not at all, per `weaver-state-Spec` section 4's transaction,
so a driver dying mid-stream leaves a clean prefix and no corruption, and
the prefix is unsealed, so nothing downstream mistakes it for a whole
preload. The
recovery is the next preload's opener, which retires the prefix with
everything else the session held, per section 2, so no cleanup act exists
between the death and the retry and none is needed. What became of the
partial one is the certification's to notice, and it notices nothing once a
whole preload has replaced it.

**A malformed distillate is the sender's defect**, dropped whole without
closing anything, exactly as the first door drops it.

**A dead member costs the session nothing that matters.** The loss clause of
`weaver-state-PRD` section 3 covers this door as it covers the first: the
holdings are rebuildable by construction, here trivially, because the driver
re-preloads from the record it still holds.

## 6. What neither party may do

- Neither party writes the trace through this seam, in either direction. The
  record stays the operator's, read by the driver outside the agent, and
  `weaver-trace` keeps its write-only pin.
- Neither party exposes this seam to the model or to the loop's interior.
- Neither party persists anything across the session through this seam. The
  door adds no life to the store's file.
- State never learns a path. Distillates arrive as content, and everything
  path-shaped was resolved by the driver on its own side of the boundary.

## 7. Change protocol

A change to what crosses this door, to the door's standing condition, or to
the credential judgment touches this contract, and every party merges in the
same act. The `election` and `distillate` shapes are the drawn contract's
own: a change to them there reaches here through the draw, and such an act
merges the parties of both contracts.

## 8. Conformance

The door is testable against the living pair under a diagnostic load: a real
member stood with the preload name, a real driver landing a real record's
projection, and the holdings answering the shape ask on the harness door with
exactly the replayed session's runs. The wrong-peer refusal and the
kind-conditioned standing are perturbation-grade and their assertions live in
`weaver-state-Spec` section 4. How the member learns the preload name at its
standing follows the pattern of `weaver-state-Spec` section 2 and is elected
in the code act that opens the door.
