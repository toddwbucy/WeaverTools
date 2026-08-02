# weaver-gate - Spec

**Status:** DRAFT. Cut 2026-08-02, sixth of the Spec pass, specced to the same
boundary its charter is chartered to: the lifecycle half, with the traffic
arriving via the token workflow. No code is written against it until phase
three is ratified, per Working Process section 6.

**Date filed:** 2026-08-02
**Document ID:** `weaver-gate-Spec`
**Parent:** `weaver-gate-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

Build instructions for `weaver-gate`: the binary's layout, the seam end's
adoption, the hook's mechanics, the predicate at accept, and the elections a
builder would otherwise invent. It is derived from `weaver-gate-PRD` and from
the two contracts this crate is party to, `weaver-harness-gate-contract` and
`weaver-gate-world-contract`, together with `weaver-organ-channel`, the drawn
material the first of them draws.

Level discipline. The charter says what the crate needs and why. This document
says how it is represented, and per gate G2 it elects against grounds the
charter and the contracts state rather than developing grounds of its own.
Where this document and the charter disagree the charter yields nothing.

**This document declares no graph records,** per Document Format section 1.
The charter is the source of this crate's node, its parent edge, its one floor
link, and its one declared seam.

**It is written from the merged corpus alone,** per the ruling of 2026-08-01
that keeps the old tree's Specs out of the Spec pass.

**The bound is the charter's own half-chartered line.** What is fully
specifiable is the raise and the lower, the boundary predicate, and the
process facts. The turn exchanges, the relay's interior, streaming,
backpressure, cancellation, drain, and concurrent clients defer with the
token workflow, per charter section 8, each waiting rather than missing.

## 1. The crate

**One binary.** The gate is its own executable, forked and exec'd by the
harness during the enter fan-out, per apex section 12, and nothing links it.

**Layout.** One module per obligation, with one placement.

    src/main.rs     entry, the two hygiene sets, and wiring, and nothing else
    src/channel.rs  the seam end and the exchange service, section 2
    src/hook.rs     the instruction's resolution, the bind, the predicate, section 3
    src/relay.rs    the pass-through, deferred, section 4

Four files, `relay.rs` standing as a placement the way the harness Spec
places its deferred modules.

**Edition and toolchain.** Edition 2024 on the pinned nightly, no nightly
feature used.

**The dependency set is one internal crate and two external ones.**
`weaver-types` is the charter's one floor link, taken **without its `config`
feature**: this crate reads no configuration file, per charter section 3, the
gate instruction arriving over the seam instead, so no parser enters a
process whose whole argument is that it holds little, which is the thinness
the feature gate exists for, per `weaver-types-Spec` section 1. No direct
`weaver-traits` line exists, matching the charter's floor-link set.
`serde_json` encodes and decodes the seam's envelopes and touches no client
byte, because the client's line is octets this crate must not read, per the
opacity rule. `nix` is the OS surface, on the grounds
`weaver-harness-Spec` section 2.4 argued: the needed calls are `bind`,
`listen`, `accept`, `getsockopt` for the peer credential, `fcntl`, and the
two `prctl` sets of section 2.

**No async runtime, no logging crate, nothing else.** The lifecycle traffic
is two exchanges and the client traffic is deferred, so nothing here needs an
executor, and this crate writes no account of anything, per charter section
1: a logging crate would be a second author's first step. The absences are
checked by the build-time `cargo tree` assertion the floor Specs share.

## 2. The seam end, and the process facts

**The channel end arrives at descriptor 3, inherited rather than
re-decided.** `weaver-harness-Spec` section 2.2 elects the number and owes it
to this document, and this document takes it: the exec'd binary finds its
one inherited descriptor at 3, the first after the standard streams, and
wraps it as an owned handle at entry. It is the only descriptor this process
begins with beyond the standard streams, per the fork discipline of
`weaver-harness-gate-contract` section 1, which makes a build in which this
crate holds a trace, coordination, or residency handle broken whether or not
it uses one. That property is the harness's to enforce at the fork and this
crate's to rely on, and the reliance is stated rather than re-tested here,
the enforcing test living in the harness Spec's section 8.

**Entry performs two hygiene sets and one election, before the first read.**
The dumpable flag is cleared and the channel end is set close-on-exec, both
sets and never checks, per charter section 7 and the set-not-check rule of
`weaver-organ-channel` section 2: this crate spawns nothing, so the flag is
defense against a compromise's exec rather than a planned fork, and it costs
one call. The election is the parent-death signal, which the charter leaves
to this Spec: taken, one `prctl` naming `SIGTERM`, so a gate blocked
anywhere other than the channel read still dies with the interior it
protects. It backs the closure observation rather than replacing it, the
requirement standing on closure alone per charter section 4, and the signal
covering the window where the gate is not reading.

**The exchange service is a serial loop over the channel.** Directives
arrive as `OrganEnvelope` JSON, one message per envelope, on the
`SOCK_SEQPACKET` end the harness created, and this crate carries the
election's receive obligation as every receiving crate does, per
`weaver-types-Spec` section 4: the buffer is sized to the 64 kibibyte
envelope bound, a read returning with `MSG_TRUNC` set is a channel fault and
never a message, and the same bound is asserted on this crate's sends. A
directive out of order for the channel's state answers `OutOfOrder`, per
`weaver-harness-gate-contract` section 3, and the state has three positions,
before-raise, raised, and lowered, the last terminal.

**Closure is death, and the response is the charter's.** A read that returns
closure means the interior is gone: this crate closes its listener if one
stands and exits, per charter section 4, never treating closure as an
answer, per the drawn material.

## 3. The hook

**The instruction is resolved, never interpreted beyond its fields.** The
raise directive carries the `gate-instruction` the operator declared and
admin validated, uninterpreted by the harness, and this crate consumes
exactly two things from it: the socket path to bind and the access rule the
predicate judges against. The field list is the floor's satellite, per
section 6, and the demand stated here is what that satellite must carry.

**The client socket is `SOCK_STREAM`, elected on the same ground as the
operator surface.** `weaver-gate-world-contract` section 2 fixes
newline-delimited JSON, one request per line, so the newline is the framing
and a boundary-preserving type would carry a second framing under the
first. A stream is also what a local client's ordinary tooling dials, which
is the audience that contract names. The opposite election from the organ
channels, principled the same way twice now, per `weaver-admin-Spec`
section 2.

**The bind takes the path as given and refuses what it finds in the way.**
The socket is created with the close-on-exec flag in the creating call and
bound to the instruction's path. A path already occupied refuses the raise
with `BindFailed` and the reason carried, and this crate unlinks nothing:
the path is the operator's artifact, a stale socket left by an unclean
death is the operator's to clear, and a gate that deleted filesystem
entries to make room for itself would hold an authority its charter never
grants. Ready is answered only after both the bind and the listen have
returned, which is what makes ready a fact about the listener, per
`weaver-harness-gate-contract` section 2.

**Every connection is authenticated at accept, before any byte is read.**
The accepting call sets close-on-exec on the connection, the peer's
credential is read with `SO_PEERCRED`, and the identity is judged by the
floor's one predicate against the instruction's rule. Verified on the
admin Spec's pass and relied on here: the credential on an accepted
connection reports the connecting peer's own uid, gid, and pid.

**The agent uid is denied by construction, not by configuration.** This
process runs as the agent uid, so it knows the one uid the boundary exists
to exclude: its own. The deny set the predicate judges is the instruction's
rule with this process's own uid added at raise, unconditionally, so no
operator mistake in the rule can readmit the agent, denial winning over
permission per `weaver-types-Spec` section 3. A peer that fails is refused
by closure before any content is read, per `weaver-gate-world-contract`
section 5, and nothing is written to it, an admitted-looking answer to a
refused peer being a conversation the boundary already declined.

**The lower closes the listener first and confirms after.** Stopped is
answered only after the close has returned, per the contract, so nothing
new can arrive anywhere in the interior once the harness proceeds. In this
pass no traffic exists, so the close is the whole of it, and what happens
to an in-flight connection at lower is drain, deferred with the token
workflow.

**A refusal leaves nothing held.** A failed bind holds no listener and no
half-bound socket, so the aggregate's rollback has nothing of this crate's
to unwind, per charter section 5, and the refusal is answered rather than
exited on, a party that exited replacing a typed reason with an observed
death.

## 4. The relay, deferred

`src/relay.rs` is the placement for the pass-through: inbound octets to the
harness, outbound octets to the client, order preserved, nothing retained,
per charter section 2. Everything it needs is the token workflow's, the
turn exchanges toward the harness above all, per charter section 8, and
nothing is shaped here ahead of that charter, per apex section 9. What this
Spec fixes about it is only what the merged contracts already fix: the
relay reads no content, and a line that does not parse is refused by the
harness with the refusal returning by the path the line took, the gate
carrying both directions unread.

## 5. The failure vocabulary

```rust
pub enum ChannelFault {
    Truncated { bound: usize },
    Undecodable,
    Closed,
}
```

**Three faults, the harness Spec's split adopted.** A refusal is a typed
answer on an exchange, every refusal this crate issues is a
`lifecycle-refusal` drawn from the floor, `BindFailed` above all, and a
channel fault is a failure below the exchange layer. `Closed` is the one
this crate does not survive: the exit path of section 2 is its handling,
and the other two are answered as faults of the exchange in flight.

## 6. What is enforced, and by which instrument

Per apex section 11, the walks first.

**The reference walk: an elected tool dials the agent's own mouth.** The
adversary is the agent's tool surface running as the agent uid, the attack
a dial of the named socket the instruction declares, the prompt-yourself
loop the charter names. The mechanism is the predicate at accept with the
agent's own uid denied by construction, per section 3. The test is the one
`weaver-types-Spec` section 3 owed this document, landed: a connection
from the agent uid is refused at accept, before any content is read,
confirmed by watching content reach the harness when the predicate is
weakened.

**The second walk: a same-uid process attaches to the gate.** The adversary
is the same tool surface, the attack a `ptrace` or `/proc/[pid]/fd` reach
into the one process of the worker's family that faces outward. The
mechanism is the dumpable flag cleared at entry, a set and not a check.
The test confirms the flag is clear after entry, watched to fail when the
set is removed.

**The third walk: the boundary outlives its interior.** The adversary is
timing, a client conversing with a gate whose harness is gone. The
mechanisms are the closure observation, exit on a closed channel, and the
elected parent-death signal backing it. The test kills the harness side of
a standing pair and confirms the gate exits and the listener is gone,
watched to fail when the closure handling is removed.

**Enforced by the compiler.**

- The floor's three wire enums are exhaustive, so every directive, answer,
  and refusal case reaches this crate's matches loudly.
- Descriptors are owned types end to end.
- The channel state's three positions are a type, so a directive against a
  lowered hook is refused by a match arm rather than a flag check.

**Enforced by compile-fail tests.** One absence is this crate's own to pin:
`hook.rs` exposes one bind site taking the instruction's path, and a
doctest constructing a listener from a bare `&str` or `PathBuf` anywhere
else in the crate fails to compile, the two named shapes with the general
prohibition staying review's, per the floor Specs' split. The
load-bearing absence this crate relies on, `PeerIdentity` deriving no
`Deserialize`, is the floor's pin, per `weaver-types-Spec` section 3.

**Enforced by the manifest.** The internal dependency is exactly
`weaver-types` without the `config` feature, read against the graph's one
floor-link under gate H2. No async runtime, no logging crate, and no YAML
implementation in the resolved tree, by the build-time `cargo tree`
assertion the floor Specs share.

**Requiring a perturbation-verified test, beyond the walks.**

- Ready follows the bind: the answer is sent only after bind and listen
  return, confirmed by watching a client's dial succeed against an
  unconfirmed raise when the ordering is reversed.
- Stopped follows the close: confirmed by watching a dial succeed after a
  stopped answer when the ordering is reversed.
- A refused raise holds nothing: after a `BindFailed`, no listener exists
  and no socket file was created by this crate, confirmed by watching a
  leaked listener appear when the cleanup-on-refusal is removed.
- Truncation is a fault: an over-bound envelope produces `Truncated` and
  no directive, confirmed by watching a silently shortened directive
  decode when the `MSG_TRUNC` check is removed.
- The unparseable-line refusal path stays whole-cloth deferred: no test
  here parses a client line, and a test that did would be the opacity rule
  breached by the suite, which review checks for.

## 7. Open elections

Each names what settles it, and none is this Spec's to settle alone.

- **The `gate-instruction` field list.** A satellite of
  `weaver-types-Spec`, with this document's demand stated in section 3:
  the socket path and the access rule. The demand is recorded here so the
  satellite is shaped against a consumer rather than invented.
- **Everything the token workflow charters.** The turn exchanges and their
  shapes, the relay's interior, streaming, backpressure, cancellation,
  drain on stop, and concurrent clients against the one-turn loop, per
  charter section 8.
- **The tool-uid ruling.** Charter section 7's pending candidate, settled
  by the architecture seat's ratification or the tool workflow's threat
  measurement, and nothing here builds against the separate-uid arm.
- **The satellite types.** `ChannelFault`'s spelling against the harness
  Spec's identical enum, one shared shape in two crates being tolerable
  where a shared crate would be a dependency taken for a name, and the
  channel-state enum's name. Choices with no cross-crate consequence,
  listed so what this Spec leaves to a builder is complete.
