---
title: Contracts
summary: every seam this program governs, with its parties, its tag, and where the merged contract lives
version: v0.2
date: 2026-09-16
commit: unreleased
parent: WeaverTools Technical Documentation
---

# Contracts

**Status:** technical documentation. Describes, decides nothing.

The code described here is unreleased and is scheduled for release in the first
quarter of 2027.

One page rather than one page per contract, settled on length and overturnable.
**This is the one place a contract is read out here.** Every crate paper links to it
and none of them restates a contract, so two pages cannot drift into two accounts of
one seam.

Each entry names the parties, what the seam carries, and the merged document that
governs it. **The merged document is authoritative.** Where an entry here and its
source disagree, this page is the defect.

**Every contract named on this page is MERGED in the corpus.** Where an entry says
its read-out is not drafted, what is owed is this page's prose. The contract
itself is written and in force.

## The internal seams

Ten, declared in the corpus at the declaring side. Eight sockets and two links.

**The count is of seams that carry a seam edge, which is not the same as every
boundary this program has.** The three contracts below this section govern
boundaries whose far side is a front-end principal, an operator, or the init
system, and the graph carries no node for a principal outside the program, so each
of those is a seam that carries no edge and none of them joins the ten, per
`weaver-admin-PRD` section 6. Two of the ten stand at least partly outside the
agent, both of them `weaver-analysis`'s, and they are counted here because both
ends are crates.

### weaver-admin-harness-contract

`weaver-admin` to `weaver-harness`, **socket**. Lifecycle authority inward:
enter, leave, and stop. Admin dials, one connection per verb, and the harness
reads the peer credential at every accept before any byte. Read-out not drafted.

### weaver-analysis-state-contract

`weaver-analysis` to `weaver-state`, **socket**. The preload seam: a finished
record parsed outside the agent, landed in the custodian's holdings so that what it
holds is indistinguishable from what a live tee would have landed. The driver sends
and the custodian speaks at no time here. A named door on the custodian, credential
authenticated, standing only under a diagnostic binding or a load that elects a
restore, and refusing the agent's own credential at the accept. Read-out not
drafted.

### weaver-analysis-web-contract

`weaver-analysis` to `weaver-web`, **socket**. The signals seam, and the one seam
with neither end inside an agent: what a parsed record measured at each generated
position - the tokens drawn, the entropy the generation measured, and the surprisal
where that election stands - out to the connector, which lands it in its own store
and draws from the store rather than from the wire. The emitter sends and the
reader asks nothing. Read-out not drafted.

### weaver-harness-diagnostic-contract

`weaver-harness` to `weaver-diagnostic`, **link**. The second record mechanism, and
the diagnostic binding's: the harness decides what a replay is and when a pass
opens and closes, and the recorder assigns ordering, produces canonical form, and
hands the rendering to the sink admin opened. It holds no policy. Crosses no
process line, so it authenticates nothing. Read-out not drafted.

### weaver-harness-gate-contract

`weaver-gate` to `weaver-harness`, **socket**. The turn inward as a frame, the
deliverable outward, and the execution exchange that carries a tool call.
Read-out not drafted.

### weaver-harness-spu-contract

`weaver-spu` to `weaver-harness`, **socket**. The residency seam: admit a model
binding, release it. The organ channel, two initiators. Read-out not drafted.

### weaver-harness-spu-decode-contract

`weaver-spu` to `weaver-harness`, **socket**. The token seam: a generation and
the measurement that rides with it. Read-out not drafted.

### weaver-harness-spu-classify-contract

`weaver-spu` to `weaver-harness`, **socket**. The label seam: content in, every
label the artifact's head defines with its score out. Read-out not drafted.

### weaver-harness-state-contract

`weaver-harness` to `weaver-state`, **socket**. A member seam rather than an
organ channel: the harness asks and the custodian answers. The election opens the
channel, the distillate feeds it, shape and recall answer. Read-out not drafted.

### weaver-harness-trace-contract

`weaver-harness` to `weaver-trace`, **link**. One of the two seams that cross no
process line, so it authenticates nothing, the diagnostic seam above being the
other and linked for the same reason. Read-out not drafted.

## The two external contracts

**These two are the program's public surface**, written by the ruling of
2026-08-01 for an outside consumer and for nothing else. A frontend or any other
consumer builds against these and against no other document on this site.
[weaver-web](weaver-web/weaver-web.md) is the first to build against them, and
being first is what turned the sufficiency claim into a checkable one. **It found
one reach these two do not cover.** Turns and the record are here. The lifecycle
verbs are not: the operator contract governs the record and says outright that
running the admin binary is running the crate rather than a channel it governs,
so an outside consumer has no page for that surface and builds it on deployment
fact. The ask for one is filed.

**Consuming these two is not the same as being a party**, and `weaver-web` is now
both. It builds against this pair the way anything outside the program would, and
it signs `weaver-analysis-web-contract` above as one of two crates, which is a seam
that carries an edge and reaches no agent. The two here carry none, their far side
being whoever dials and whoever the operator is.

### weaver-gate-world-contract

The world to the agent. One NDJSON line in, one line out, the peer authenticated
at accept against a predicate that admits front-end principals and excludes the
agent's own uid. Order preserved per connection, one turn at a time in arrival
order, and a close that answers a turn names that turn and its run. Read-out not
drafted.

### weaver-admin-operator-contract

The agent's account to the operator. What crosses inward is nothing. What crosses
outward is the NDJSON stream, one event per line, to a sink the operator declares
and owns. Bounded loss is named twice and never silent. Read-out not drafted.

## The system contract

### weaver-admin-systemd-contract

`weaver-admin` to the init system. Not an external consumer surface and listed
apart from the two above for that reason: what is on the far side is the service
manager rather than anyone building against this program. Read-out not drafted.

## Drawn material

### weaver-organ-channel

Not a contract, which its absent suffix marks. The organ channel's mechanics
stated once, drawn by the contracts that carry it: two initiators, the exchange
and its position, boundaries and ordering guaranteed, and closure that is never an
answer. Read-out not drafted.
