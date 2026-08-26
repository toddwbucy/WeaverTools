# weaver-web - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth. Ratified on its own
terms under the per-charter rule of 2026-08-23, conforming to the pattern the
2026-08-04 act established.

**Revised:** 2026-08-25, the confirm view, second revision of the day.
The admin surface gains a composed view at section 4.4: pull a run from
the record, drive its turns back through the gate on a fresh load, and
show the verdict beside the stats. It is not a fourth surface and adds
no reach - the source is the record (4.3), the fresh load is the verbs
(4.2), and the reissue is gate turns (4.1's own boundary). It stops at
the serving path: anything past gate-shaped reissue is the diagnostic
domain's, per the lab report of 2026-08-25.

**Revised:** 2026-08-25, the two-process shape. The deployment section
now states the architecture as two processes joined by one dialed link:
the connector holds the box-bound reaches and the server holds the
presentation stack, colocated by default and separated by changing one
address. The operator's direction of 2026-08-25 makes this the shape
from the beginning rather than a later mode. The Spec's section 16 pins
the link, and the refactor of the built one-binary v1 into the shape is
owed as its own code act.

**Revised:** 2026-08-24, the crate takes its place. It lands at
`crates/weaver-web/` and this document at `docs/crates/weaver-web/`,
container coming from the directory so placement is the declaration. The
seam is unchanged: this crate is contract-coupled across
`weaver-gate-world-contract` and `weaver-admin-operator-contract` and links
no crate of this workspace. Three collisions are surfaced and held rather than
resolved: the edition, the licence, and the first database dependency in
the workspace.

**Revised:** 2026-08-19, the channel amendment. The chat surface becomes the
multi-party channel surface on the operator's adoption of the front-end
architecture exploration's three positions, per the buzz prior-art survey of
2026-08-19, held in the operator's report archive. The browser-as-display-engine
rule hardens from stack preference to constraint, the IAM roadmap item gains
its mechanism (passkeys, TLS bundled), and the authorship-responsibility
ruling lands in section 6.

**Date filed:** 2026-08-19
**Document ID:** `weaver-web-PRD`
**Parent:** the WeaverTools suite, whose governing document is deliberately
not yet written, per `weaver-agents-PRD` section 0. The graph parent edge
names the `WeaverTools` system node, and the header and the edge name the
same thing. **This crate sits outside the agent boundary and does not enter
the `weaver-agents` roster**, per that document's section 0: it reaches
inward as a consumer of the two external contracts, and the agent never
grips it.
**Editorial:** Per the Working Rules.

```graph
node: weaver-web
kind: crate

edge: parent
from: weaver-web
to: WeaverTools
```

---

## 1. Purpose

weaver-web is the frontend of the WeaverTools suite: a web surface over the
suite's agents, serving the two roles the framework already recognizes. The
*user* converses with agents. The *operator* drives their lifecycle and
reads their record. It is the suite's first consumer application, per the
vision document's section 13, and its discipline is the section's closing
rule: satellites are consumers or contract peers, never retrofits. Nothing
here reaches into the organism. What weaver-web needs and does not have, it
asks for through the framework's own change protocol.

The conversation surface is a **multi-party channel**: multiple humans,
multiple local weaver agents, and eventually upstream foundation models,
participating in one channel toward a shared goal. The channel is
weaver-web's own construct entirely - see the scope ruling in section 5.

The application also carries a second, quieter purpose: it is the "real
consumer at a real rate" several framework elections are waiting on. Its
honest limitations are the demand signal that pulls streaming, a status
verb, and an operator state read through the framework's front doors.

## 2. Build surface

weaver-web builds against published pages and observable behavior, nothing
else:

- **`weaver-gate-world-contract`** - the client boundary. Newline-delimited
  JSON over the gate's Unix socket, one request line in, one close line out
  per turn. The close names its kind (`answered`, `stopped`, `refused`)
  and, where a turn opened, the `turn` and `run` it answers.
- **`weaver-admin-operator-contract`** - the trace's exit. The program tees
  its record to an NDJSON stream at the operator-declared sink, ordered,
  with loss bounded and always marked in-stream. Every view weaver-web
  builds on the record is built on this stream, on weaver-web's own
  compute, per the live-view ruling.
- **The verb invocation surface** - `weaver-admin {validate|load|unload}
  <agent>`, run as root, one JSON object on stdout per verb. This surface
  is currently documented by deployment fact rather than by a page written
  for an outside consumer, and section 9 files the ask.

Prohibitions, absolute: no parsing of anything a contract calls opaque, no
channel to the program other than the gate socket and the verb invocation,
no reading past the published stream, no workaround where a contract falls
short.

## 3. Deployment shape

Two processes, one crate, split by what each must touch. The
**connector** (`weaver-web-connector`) runs on the agents' own box as
the operator's uid (the principal the gate's predicate admits) and holds
every box-bound reach: the gate sockets, the verb invocation, the trace
sinks, the load-state observable, and the read of the agent
declarations. It renders nothing and stores nothing. The **server**
(`weaver-web`) presents HTTP on the LAN and holds everything that is not
box-bound: the channel store, the registry, the router and queues,
rendering, and the SSE fan-out. It reaches the box only through the
connector.

They meet over links the connectors dial: one connection per box, any
number of boxes, the server listening on one address its config names,
loopback by default, each connector's config naming that same address.
Colocated, the link is loopback and the deployment is one box exactly
as before. Offloading the presentation stack to another device on the
LAN is deploying the server there and changing that one address - a
deployment fact, never an architectural event - and a second agents'
box is a second connector dialing the same server with its own roster.
No agents' box opens a listening port for this in any shape, because
connectors only dial out.

The trust posture does not move with the address. The verb surface is
already network-reachable through the listener under the posture section
6 states, and a link whose remote peer is one server is a narrower
admission than that listener. The IAM act is where peer proof arrives,
for the listener and the link alike. The custody invariant survives the
split intact: the thing that speaks on the network is the thing that
holds the socket, and the connector is that thing, exactly as the one
process was.

State and connection, by owner:

1. The **gate sockets** (`/run/weaver-<agent>/gate.sock`) - the
   connector's, one client connection per turn, for turns.
2. The **verb invocation**, via a sudoers rule scoped to exactly the
   three lifecycle verbs on the admin binary, `NOPASSWD`, for the
   operator's uid - the connector's. This is the one privilege widening
   the application asks of the box, and it is narrow, auditable, and
   declared in the deployment notes.
3. The **trace sinks** (the NDJSON files the agents' configurations
   declare), tailed read-only - the connector's.
4. The **channel store**: weaver-web's own record of channel logs and
   the participant registry, on the server's own disk - the server's. It
   is not the trace, never touches the trace, and links to traces by run
   and turn label only. Custody follows the trace's philosophy: one
   trusted writer, the server, and no other.

**The browser is a display engine. This is a constraint, not a
preference.** The browser receives a rendered projection and submits
authored text, and holds nothing else: no keys, no signatures, no protocol
state, no routing or ordering logic, no identity assertion beyond the
transport session. All processing that means anything happens server-side,
where the trace boundary and the operator's trust already live. Any future
architecture change is tested against this constraint first.

The browser side is server-rendered HTML with htmx. Server-sent events
carry what must move without a page turn: channel events as they append,
and the trace view's live tail. The channel surface is built from day one
as a bounded component fed by a server-owned channel event stream, so that
if interactive density later demands it (threads, in-place editing,
virtualized history - the named triggers), the rendering of that one
region can be replaced by a small scripted island consuming the same
stream, without touching the shell or moving any processing into the
browser. No separate frontend toolchain, no SPA.

## 4. The three surfaces, on two boundaries

The surfaces group by the framework boundary they cross and the role
that boundary belongs to, per the operator's ruling of 2026-08-19. The
channel (4.1) is the **user surface**, crossing the gate. Lifecycle
(4.2) and trace (4.3) are the **admin surface**, crossing the operator
boundary, and sit behind the admin role. The separation is structural -
routes, modules, and the role gate exist now - so the IAM act attaches
authentication to standing roles instead of rearchitecting, and so the
frontend never presents as one combined mechanic what the framework
holds as two boundaries with different owners.

### 4.1 Channel

The conversation surface: a multi-party channel over one server-side log.

**Participants.** A participant is an identity in the registry plus an
adapter plus a respond policy. Three adapter kinds:

- **Browser session** - a human. Display surface only, per section 3.
- **Weaver agent** - a client connection to that agent's gate socket,
  held by weaver-web. The adapter serializes channel context into the
  request line, speaker-labeled and windowed server-side to the line
  bound, sends one line, receives one close, and appends it to the log
  with its run and turn labels.
- **Upstream model** - a stateless HTTP call (Claude, GPT) behind the
  same adapter interface. An upstream model is weaver-web's guest and
  never the framework's concern, its credentials being weaver-web
  configuration.

Agents are first-class participants: their own registry identity, their
own channel membership, their own name on every message. Agent-ness is
additive metadata on a common participant shape, never a parallel type.

**Invocation.** Mention-gated by default: an agent speaks when named. Each
participant carries a respond policy (anyone, allowlist, nobody). Self-
and agent-authored messages do not trigger agents by default, so two
agents cannot ping-pong, an agent-to-agent exchange happening only under an
explicit policy, never emergently.

**Queue discipline.** The gate serves one turn at a time and later
requests wait. weaver-web holds that queue itself, per agent, in front of
the gate connection - visible, inspectable - rather than letting requests
stack invisibly inside the socket. One turn in flight per agent, and queued
mentions batch into a single turn on drain. The channel renders the truth:
an agent shown answering, with its queue depth, is the honest presentation
of whole-turn latency in a room.

**Design constraints taken from the gate contract rather than chosen:**

- **Whole-turn answers, presented whole.** The gate does not stream. A
  clear in-flight state, never simulated streaming.
- **Closes render by kind.** `answered` is the message body. `stopped`
  and `refused` render distinctly with the reason the close carries. An
  unnamed close (a line that never parsed) is weaver-web's own defect,
  since weaver-web authors the request lines, and surfaces as an
  application error, never styled as an agent's message.
- **Run and turn labels are kept and shown.** Every agent close lands in
  the log carrying them, and a visible turn label links to that turn in
  the trace surface - what the agent said, joined to what the agent did.
- **The line bound is enforced before sending** (32 KiB inclusive today,
  the Spec's number), since a violating line closes the connection below
  any turn. Context windowing to fit the bound is the server's job.

### 4.2 Lifecycle

The operator's control surface, over the declared agents (one, `alpha`,
today):

- The three verbs as actions, each rendering the JSON answer the
  invocation returns, success and refusal alike, verbatim and formatted.
- Load state, shown from the one observable the boundary offers: the gate
  socket's existence. The view labels this as an inference until the
  status verb lands, and never presents it as the program's own word.
- No verb chains another, mirroring the framework's own rule. The surface
  offers `validate`, `load`, `unload` as separate acts and nothing
  composite.

### 4.3 Trace

The operator's read on the record, and the live-view ruling made real. The
server tails each agent's NDJSON stream and serves a turn-bracketed live
view:

- Events grouped by run, and within a run by turn, presented in stream
  order.
- Event kinds rendered distinctly, with `fault` events prominent - the
  stream is the program's one fault carrier, and this view is where an
  operator sees a fault at all.
- Loss marks (the shed-gap and detachment marks the tee promise defines)
  rendered as first-class objects, never dropped or smoothed over, so the
  view's completeness claim is exactly the stream's.
- Channel turns link here by run and turn label, closing the loop between
  what an agent said and what it did.

The view interprets the durable event schema as published format. Where an
event kind is unknown to the view, it renders as raw JSON rather than
being hidden, so schema growth degrades the view gracefully instead of
silently.

### 4.4 Confirm

A composed admin view, not a fourth surface: it asks one question of the
two surfaces above it - does this record reproduce? The operator picks a
run from the record. The view unloads and reloads the agent by the verbs,
reissues each recorded turn's request text byte-exact through the gate in
order, and compares the fresh record against the source field by field:
rendered prompt, derived seed, knobs, emission, token ids, entropies.
The verdict renders per turn beside the stats the record carries (whole
turn latency, tokens in and out), and a divergence renders as itself,
never smoothed.

Three honesty rules, stated because the view drives a live agent:

- The confirm is disruptive and says so: it unloads the agent it
  confirms, and the operator is told before the verbs run.
- Replay runs beside live traffic. A channel turn landing mid-confirm
  shifts the turn ordinals and the confirm fails honestly rather than
  serializing the world to protect itself.
- The boundary is the serving path. The view reissues what the gate can
  carry and reads what the record says, and nothing else: instrumented
  replay, capture, and perturbation are the diagnostic domain's, and
  this view is not their door.

## 5. Scope ruling: the channel is weaver-web's

Adopted 2026-08-19 on the exploration's third position. The multi-party
channel is deployment, not framework. The decisive structural fact: from
each weaver agent's perspective, the channel does not exist - the agent
sees its gate socket, one request line, one close, exactly as it does
today. A construct the framework cannot even observe is not a framework
surface. The corpus's guardrails say the same from the other side:
multi-participant coordination toward a shared goal is orchestration,
which the primitives program refuses.

Consequences: weaver-web owns the channel log, the participant registry,
the routing, and the invocation policy, wholly. The framework changes
nothing. The channel makes the streaming ask more urgent (rooms amplify
whole-turn latency, since turns serialize per agent), and at most one
future ask exists - multi-party attribution reaching an agent as structure
rather than as speaker-labeled prompt text, which would enter through the
gate contract's Spec field list. Today, labeled text within the line bound
is sufficient and honest, and no such ask is filed.

## 6. Identity, authorship, and responsibility

**Authentication identifies. Authorship responsibility follows the
authenticated account.** A session left open and abused remains the
account holder's responsibility, remediated outside this application's
scope. This ruling (2026-08-19) permanently excludes per-message
cryptographic signing from the requirements - not deferred, excluded. The
record is single-writer testimony end to end, matching the trace's custody
model: identity is established at admission, and what a session authors,
the account owns.

What the application owes in exchange, so responsibility is assignable in
practice:

- **Session boundaries reconstructible in the record.** The channel log
  marks when a session opens and closes and how it authenticated, so
  "everything between these marks was this session" is readable after the
  fact.
- **Revocation as a first-class verb** (IAM era): list active sessions,
  kill one.
- **Session expiry** as standing policy.

**Roles exist now and proof arrives with IAM.** Every participant holds a
role, `user` or `admin`. Holding auth to converse with an agent never
implies admin access: the admin role gates the operator surface, and v1
assigns it by the operator's declaration in config. The accepted
development-cycle caveat: sessions are anonymous until the IAM act, so
the role gate is boundary hygiene rather than access control until
sessions prove who they are. The IAM act changes the proof, never the
gate.

In v1 there is no authentication: the trust boundary is the LAN and the
box, and a session is whoever holds the connection. The participant
registry carries an unused credential slot from day one so the IAM era is
additive rather than a migration.

## 7. Non-goals for v1

- **Identity, authentication, and transport encryption.** No login and no
  TLS. Deliberate deferral, held on the roadmap with a named trigger and
  now a named mechanism (section 8). **Roles are not among the deferrals
  and are not access control either**: the user and admin roles exist and
  are assigned from the configuration's admin list, which is boundary
  hygiene over anonymous sessions until IAM supplies the identity proof
  that would make them enforceable.
- **Per-message signing and keys in the browser, permanently.** Excluded
  by the section 6 ruling and the section 3 constraint respectively, at
  every horizon, not merely in v1.
- **Multi-agent fleet UI.** One agent exists, so the surfaces take the agent
  as a parameter internally so the assumption stays shallow, but no fleet
  surface is built.
- **Streaming simulation.** No token-drip theater over a whole-turn
  answer.
- **Anything of weaver-store or the memory leg.** A record, a memory, and
  a commons are three things. This application renders the first and
  touches neither of the others.
- **Trace persistence, indexing, or retention tooling.** Durability is
  the operator's per the contract, so v1 renders the live stream and reads
  the file as it stands.

## 8. Roadmap

Held in order, each with its trigger:

1. **v1, the three surfaces** - this document. The channel surface may
   land single-human-plus-one-agent first and grow participants without
   architectural change, since the routing layer is multi-party from the
   start.
2. **IAM and network encryption, one act** - passkeys (WebAuthn) for
   human participants: the private key lives in the user's platform
   authenticator, never in the browser page and never on this server,
   which stores public keys only. Sessions are minted and verified
   server-side. TLS arrives in the same act by necessity, since WebAuthn
   requires a secure context. Includes session listing, revocation, and
   expiry per section 6. Trigger: the three surfaces stable in daily use.
   Release blocker for any exposure beyond the LAN, and nothing before it
   is.
3. **Streaming chat** - when the gate's streaming extension lands
   upstream. The channel surface is built so a streamed body slots into
   the same conversation view.
4. **Channel surface carve-out** - only if triggered: threads, in-place
   editing, or virtualized history, whichever is demanded first, per
   section 3.
5. **Fleet view** - when a second agent is declared and the need is real.
   The link makes the shape additive: a second box is a second connector
   dialing the same server, carrying its own roster (Spec section 16).

## 9. Asks upstream

The gaps this application designs around, each an ask into the framework
by its own change protocol, routed through the human between seats:

1. **Token streaming through the gate.** The largest UX gap, and the
   channel amendment raises its urgency: turns serialize per agent, so a
   busy room multiplies whole-turn waits. The contract already names the
   token workflow as the door.
2. **A status verb on admin.** The lifecycle view infers load state from
   socket existence, where a `status` ask would let it report the program's own
   word. The split sharpens the ask: load state now crosses the link as
   a relayed inference, one hop further from that word.
3. **An operator read on agent state.** Session state exists within the
   agent, so the operator has no window on it. Deferred need, filed when the
   concrete read is known.
4. **A published page for the verb invocation surface.** The operator
   contract's recut moved the verbs to the invocation, whose shape lives
   in `weaver-admin-PRD` section 8 and the admin Spec - documents an
   outside consumer is not meant to build against. weaver-web builds today
   on deployment fact, where a page written for the outside party would close
   the gap the two-contract rule intends to cover.

Nothing in this list blocks v1. Each is designed around, and the designing
around is itself the evidence the ask carries upstream.

## 10. Prior art

The channel design adapts, with credit, patterns read from Buzz
(`github.com/block/buzz`, Block Inc., Apache 2.0), surveyed 2026-08-19:
the symmetric participant abstraction with agent-ness as additive metadata
and an ownership edge, mention-gated invocation with per-participant
respond policies and self-invocation suppression (agent-to-agent routing
opened 2026-08-20 for coordination), and per-channel single-flight
queueing with batch-on-drain. The survey's full read and the falsification
case against forking are recorded in the exploration's report, the buzz
prior-art survey of 2026-08-19 in the operator's report archive, and this
section carries the three positions the operator adopted from it. Buzz's
identity architecture - participant-held keys, client-side signing, the
browser as protocol participant - is deliberately not adopted, per
sections 3 and 6: its trust problem, verifiable authorship among mutually
untrusting network peers, does not exist on a box where the OS
adjudicates identity, and its client-side custody is the surface this
project refuses on principle.
