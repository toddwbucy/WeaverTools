---
title: weaver-web
summary: the first consumer: channel, lifecycle, and trace surfaces reaching an agent across the two external contracts
version: v0.1
date: 2026-08-24
commit: unreleased
parent: WeaverTools Technical Documentation
---

# weaver-web

**Status:** technical documentation. Describes, decides nothing.

**Rough draft, first pass.**

The code described here is unreleased and is scheduled for release in the first
quarter of 2027.

## What it is

**The suite's frontend, and the first thing to reach an agent from outside.**
One binary serving three surfaces over a browser: a channel where humans and
agents converse, a lifecycle view where an operator drives the verbs, and a
live view over an agent's record.

**It is a consumer, and that word is structural here rather than descriptive.**
It sits outside the agent boundary. It links no crate of the agent domain, holds
no weights, authors nothing in the record, and knows the agent only as a socket
to dial and a binary to run. Everything it is allowed to do is written on
[the contracts page](../contracts.md) - two documents, which are its entire
build surface. The agent never grips it and does not know it exists.

That makes it the working proof of a claim the rest of this site makes
repeatedly: that the boundary is real, that a consumer needs nothing but the
contracts, and that both are true in practice rather than only on paper. It
lives in the same repository as the agent it talks to, absorbed 2026-08-24, and
**that changed nothing about the seam** - it is a fact about where the source
sits, not about what crosses.

## What it owns

**The channel, and its own record of it.** Multi-party conversation where agents
are participants rather than features: invoked by mention, answering whole turns,
each answer carrying the run and turn labels that link it to the agent's own
record. The channel log is weaver-web's, on weaver-web's disk, with one writer -
its own custody discipline, borrowed from the one the agent keeps over its trace.
**It is not the trace, never writes to the trace, and links to it by label only.**

**The lifecycle view.** The three verbs an operator drives, each answer rendered
verbatim rather than interpreted. Load state is shown from the gate socket's
existence and **labeled as the inference it is**, because the program has no
status verb to ask and inventing one on the client side would be the frontend
answering a question the framework has not.

**The trace view.** A live, turn-bracketed reading of an agent's record, with
field selection and search, faults prominent, and discontinuities marked rather
than smoothed. A rotation or truncation surfaces as a mark, which mirrors the
record's own honesty rule: a gap is never filtered out of sight.

**The browser as a display engine, which is a constraint rather than a
preference.** The browser receives a rendered projection and submits authored
text, and holds nothing else - no keys, no signatures, no protocol state, no
routing or ordering logic. Everything that means anything happens server-side,
where the boundary and the operator's trust already live. Any future
architecture change is tested against this constraint first.

## Seams

**None into the agent, and that is the point.** It holds no seam in the sense
the crate papers use the word: no contract binds it to any component, because
contracts bind parties inside the program and this sits outside.

What it has instead is **the two external contracts, which are pages rather than
partners** - written for whoever builds against them and owing nothing back.
See [the contracts page](../contracts.md). Across the first it dials the gate
socket and speaks one line per turn. Across the second it reads the record the
operator holds. It reaches the operator's own lifecycle verbs by running them as
a subprocess, which is not a seam either - it is the operator's command, run on
the operator's behalf, under a rule the deployment declares.

## How it works

**One turn.** A mention in a channel enqueues an invocation for that agent. The
worker assembles context from the channel log, dials the agent's gate socket,
writes one JSON line, and waits for one line back. The close is appended to the
channel with its kind and its labels, and the browser learns about it through a
server-sent event rather than a page turn. One turn in flight per agent: a
second mention waits, because the agent serves one turn at a time and queueing
in front of it is more honest than discovering the serialization inside the
socket.

**Nothing streams, because nothing can yet.** A long generation returns no bytes
until its close, so the interface is built honestly around whole-turn latency
rather than around a progressive rendering the boundary cannot supply. That gap
is filed upward rather than worked around, which is the rule the whole
relationship runs on.

**The verbs.** A lifecycle request runs the admin binary as a subprocess with a
generous ceiling, and whatever it answers is rendered as it came, parsed when it
parses and shown raw when it does not. Nothing is swallowed and nothing is
interpreted into a friendlier shape.

**The record.** The trace view tails the file the agent's declaration names,
read-only, and projects it. It never writes there and holds no descriptor that
could.

## What it refuses

**Reaching around the socket.** When a contract lacks something this frontend
needs, the answer is an ask into the framework by the contract's own change
protocol - never a workaround, never parsing something a contract calls opaque,
never linking a crate to get at an internal. Four such asks are open, and the
designing-around is itself the evidence each carries.

**Interpreting what it relays.** Verb answers render verbatim. Trace events
render as what they are. The frontend has opinions about layout and none about
meaning.

**Writing to the record.** The agent's account is the agent's, authored by one
writer inside the boundary. This side holds a read.

**Keys and signatures in the browser, permanently.** Not a v1 deferral but an
exclusion at every horizon, per the display-engine constraint: identity is
adjudicated where the operating system adjudicates it, not by a page.

**Its own opinion about who may pass.** The gate admits it by credential or
refuses it, and being refused is a fact it reports rather than one it routes
around.

## What is not built

- **Identity, authentication, and transport encryption.** No login and no TLS.
  The roles exist - user and admin, assigned from the deployment's own list -
  and they are boundary hygiene over anonymous sessions rather than access
  control, which is worth knowing exactly: **before that act lands, anyone who
  can reach the listener and knows a configured admin name holds the admin
  role**, and the admin role drives the lifecycle verbs. The deployment shape
  is a LAN, the deferral is deliberate and on the roadmap with a named
  mechanism, and it is stated here rather than in a footnote because a deployer
  meets it before they meet the roadmap.
- **Streaming through the gate.** The largest gap, and the ask with the most
  weight behind it: turns serialize per agent, so a busy room multiplies
  whole-turn waits.
- **A status verb to ask.** Load state is inferred from socket existence, which
  is why the interface labels it as an inference.
- **An operator read on agent state.** Session state exists inside the agent and
  the operator has no window on it.
- **Upstream model participants.** The adapter seam is named and the first
  provider is its own act - a mention of a model participant is ignored today
  rather than half-answered.
