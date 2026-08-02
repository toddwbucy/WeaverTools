# weaver-harness - Spec

**Status:** DRAFT. Cut 2026-08-02, fourth of the Spec pass and the first above the
floor. No code is written against it until phase three is ratified, per Working
Process section 6.

**Date filed:** 2026-08-02
**Revised:** 2026-08-02, on the review seat's return. The child's handoff gains
an unconditional flag clear and the fork bound counts three calls, the
equal-descriptor `dup2` corner having been compiled and run by the review and
verified again by this seat. The featureless `weaver-types` link states its
true ground, this crate reading no field from the config file, and names the
placement it rests on as owed to `weaver-types-Spec` on the working list.
Descriptor 3 is owned as this Spec's election rather than credited to the
apex. The gate pair's creation states its wait on confirmed residency, which
the run state's invariant rests on. The sink descriptor's count names the
coordination contract's plural as a surfaced question rather than absorbing
it. Two interior citations correct their targets in the same act.
**Document ID:** `weaver-harness-Spec`
**Parent:** `weaver-harness-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

Build instructions for `weaver-harness`: the module layout, the item signatures,
the channel mechanics this crate holds, the descriptor custody as code, and the
elections a builder would otherwise invent. It is derived from
`weaver-harness-PRD` and from the four contracts this crate is party to,
`weaver-admin-harness-contract`, `weaver-harness-trace-contract`,
`weaver-harness-spu-contract`, and `weaver-harness-gate-contract`, together with
`weaver-organ-channel`, the drawn material three of them share.

Level discipline. The charter says what the crate needs and why. This document
says how it is represented, and per gate G2 it elects against grounds the charter
and the contracts state rather than developing grounds of its own. Where this
document and the charter disagree the charter yields nothing.

**This document declares no graph records,** per Document Format section 1. The
charter is the source of this crate's node, its parent edge, its two floor links,
its one declared seam, and its artifact edges.

**It is written from the merged corpus alone,** per the ruling of 2026-08-01 that
keeps the old tree's Specs out of the Spec pass. Where a question of fact about
the old tree mattered, the fact is cited as a fact and decides nothing.

**What this Spec can settle is bounded by what is chartered, and the bound is not
a gap.** The token workflow and the tool workflow are unchartered, so the decode
exchanges, the turn ingress traffic, the per-model assembly layer, and tool
dispatch defer here with their settlers named, the same half-chartered discipline
the SPU and gate charters run on. What is fully specifiable today is the
lifecycle interior of loop 0, trace authorship, and the descriptor custody that
protects the record, which is the spine of the deliverable rather than its
leftovers.

## 1. The crate

**Layout.** One module per obligation, re-exported at the root.

    src/lib.rs         re-exports, and nothing else
    src/channel.rs     organ-channel I/O and descriptor custody, section 2
    src/lifecycle.rs   the harness type, the run state, the fan-out, section 3
    src/authorship.rs  trace authorship, section 4
    src/assembly.rs    prompt assembly's deterministic floor, section 5
    src/tools.rs       the tool system, blocked, section 6
    src/engine.rs      the query loop, deferred, section 6

Seven files, two of them placements, the way `weaver-traits-Spec` section 1
places its blocked and deferred modules.

**Edition and toolchain.** Edition 2024 on the pinned nightly, no nightly
feature used.

**The dependency set is three internal crates and two external ones, and each is
argued.** `weaver-traits` and `weaver-types` are the floor links the charter
declares, the first for the message model the authoring path licenses, the
second for the wire vocabulary of loop 0 and the identity types the envelope
carries. `weaver-types` is taken **without its `config` feature**: this crate
reads no field from the agent config's file, so it links no parser. The model
binding and the gate instruction are config fields this crate does consume,
arriving over the coordination seam inside the enter directive, already
validated, which is the file read staying admin's. The elections the file
carries beyond them, the permission mode, the tool set, and the residual
readout, are consumed by no workflow chartered today, and section 9 holds the
config read as the tool workflow's question. **The featureless link rests on a
placement owed to `weaver-types-Spec`.** `ModelBinding` and `GateInstruction`
are config fields and wire types at once, that Spec defines them in the module
its `config` feature gates while ruling the wire types unconditional, and this
crate can construct a directive with the feature off only if the two types sit
on the unconditional side. The owed edit is filed on the working list rather
than made here, a correction to a merged document landing after this branch
rather than beside it. `weaver-trace` is the seam tagged
`link`, the recorder this crate authors through. `serde_json` encodes and
decodes the loop 0 envelopes, whose JSON election is `weaver-types-Spec`
section 4.3's. `nix` is the OS surface, elected in section 2 where the grounds
live.

**No async runtime, no logging, no HTTP.** The old tree's harness carried tokio
with every feature, `async-trait`, `tracing` with two subscriber crates, and an
outbound HTTP client, and none of it crosses, per apex section 7: nothing in
loop 0 or the trace seam awaits anything, the lifecycle interior is serial by
charter, and the executor question belongs to the token workflow with a latency
measurement in hand, per `weaver-traits-Spec` section 6. A logging crate would
be a second account beside the one the program exists to produce. The absence
is checked by the same build-time `cargo tree` assertion the floor Specs share.

**This crate spawns no thread.** The one auxiliary thread the merged set names
is the stream writer's, and it belongs to `weaver-trace`, per that Spec's
section 5. Everything this crate does in this pass runs on the caller's thread,
and the writer's existence is why the fork sites of section 2 state their
safety bound rather than assuming a single-threaded process.

## 2. The channels, and custody as code

This crate is party to three socket seams and creates two of them. The
mechanics below are one election carried and the obligations the contracts
land here, each with its contract named.

### 2.1 The socket type, carried

**The pairs this crate creates are `SOCK_SEQPACKET`, carrying the election of
`weaver-types-Spec` section 4 rather than re-deciding it.** That Spec elects the
type for the organ channels and names this document as a landing site: the
boundary property of `weaver-organ-channel` section 2 comes from the socket
type, and this crate creates the residency and gate pairs. The election arrives
with its obligation. **The receive buffer is sized to the maximum envelope of
64 kibibytes, and a read that returns with `MSG_TRUNC` set is a channel fault
and never a message.** Verified against a live kernel rather than reasoned:
a short read on `SOCK_SEQPACKET` returns the truncated prefix with the flag set
and the remainder discarded, so an unchecked flag turns a long directive into a
silently shortened one, which is the failure the boundary property was elected
to prevent. The same bound governs this crate's sends: no envelope this crate
writes may exceed it, asserted at the write site, because a bound only the
receiver holds is a bound the sender discovers in production.

**One write is one message and one message is one envelope.** The wire carries
`OrganEnvelope` as one JSON document per write, per `weaver-types-Spec` section
4.3, and the socket type is what keeps the framing out of this crate, per
`weaver-organ-channel` section 2.

### 2.2 Creation, and the atomic flag

**Both ends of a created pair carry close-on-exec from the creating act, by
`SOCK_CLOEXEC` in the `socketpair` call rather than by a later `fcntl`.**
`weaver-harness-spu-contract` section 1 requires the harness's own end flagged
from the pair's creation. The atomic form is elected because the alternative
has a window: this process forks a subprocess per tool call, a fork between
creation and a separate `fcntl` would inherit an unflagged end, and an
inherited end of the residency seam hands the tool surface a release directive.
Verified: `socketpair` with `SOCK_CLOEXEC` yields both descriptors flagged with
no interval between them.

**The child's end crosses the final exec by `dup2` to descriptor 3 and an
unconditional clear of the flag, because the duplicate law has a corner.**
Close-on-exec would otherwise close the child's own end at the exec that
starts the organ binary. A duplicate made by `dup2` is born with the flag
clear, but only when the two descriptors differ: `dup2` onto the same number
is a no-op that returns the descriptor with its flag untouched, so a child
whose end already sat at descriptor 3 would keep the flag, lose the end at
`execve`, and start the organ with no channel, silently, on whatever layout
the deployment happens to produce. Both halves are verified rather than
recalled, the clear-on-copy law for differing descriptors and the
equal-descriptor no-op that defeats it, each run by both seats. So the child
duplicates its end to descriptor 3, the first after the standard streams,
clears the flag on 3 by `fcntl` whether or not the duplication moved
anything, and execs, and the organ binary finds its channel end at descriptor
3 from its first instruction. This realizes apex section 12's topology, the
number being this Spec's own election. **The number is owed to
`weaver-spu-Spec` and `weaver-gate-Spec`**, each of which inherits it rather
than re-deciding it, the same owing shape `weaver-types-Spec` section 4 used
to reach this document.

**Between fork and exec the child performs three calls, `dup2`, `fcntl`, and
`execve`, and nothing else.** All three are async-signal-safe, and the bound
is stated because the worker holds the writer's thread at every fork: a child
of a multithreaded process may safely run only async-signal-safe calls before
its exec, so the enumeration is the safety argument and not a style. An
earlier draft elected two calls and leaned on the duplicate law without its
corner, and the review caught that the tight bound and the silent corner were
one defect, so the middle call is the corner's repair made unconditional.

### 2.3 The receive site, and adoption

**Descriptors enter this crate at exactly one call, and it asks for the flag in
the receive itself.** The trace descriptors cross once, as ancillary data on
the enter directive's own message, per `weaver-admin-harness-contract` section
3, so the one `recvmsg` site on the coordination channel carries
`MSG_CMSG_CLOEXEC`. Verified both ways: a descriptor received with the flag
arrives close-on-exec, and one received without it arrives clear, which is the
window `weaver-organ-channel` section 2 describes and the reason the obligation
is the receiver's. The site takes no flag argument and returns owned handles
the rest of the crate cannot construct another way, which is the pinned shape
`weaver-admin-harness-contract` section 5 names.

**Adoption is the constructor, and it performs the worker's hygiene as sets and
not checks.** The coordination end reaches the worker by the unit's declared
open, per `weaver-admin-harness-contract` section 2, and the composition root
hands it to `Harness::adopt`, which sets close-on-exec on the adopted end, the
set-again-after-the-last-exec obligation of that section landed in code, and
clears the process's dumpable flag, the same section's attach defense. Both are
sets because a check that finds the flag wrong and reports leaves the
descriptor inheritable and the process attachable, which is the set-not-check
rule stated at the contract and applied here.

**No path is taken anywhere in this crate.** There is no call that resolves,
opens, or stores a filesystem path to the trace, per `weaver-harness-PRD`
section 5, and the organ binaries of section 3 are the one exception, supplied
by the composition root as a construction parameter the way `weaver-trace-Spec`
section 6 takes its queue depth: a deployment fact, not an operator election
and not a discovery.

### 2.4 The OS surface, elected

**The OS crate is `nix`, and the ground is that descriptor custody is this
crate's central obligation.** The mechanics above are `socketpair`, `recvmsg`
with control messages, `fcntl`, `fork`, `dup2`, `execve`, and the dumpable
`prctl`, and `nix` covers that surface over the standard library's owned
descriptor types, so a descriptor's ownership is a compile property and a leak
is a type error rather than an integer left behind. Raw `libc` is the old
tree's answer and puts unsafe integer descriptors at the exact seam whose
custody discipline this crate exists to hold. `rustix` holds the same io-safety
posture and declines to offer process forking as a supported surface, and the
fork is not optional here: the topology of apex section 12 has this crate
forking both organ binaries. One crate covering the whole surface beats two
crates covering it between them, on the floor's own thinness doctrine.

## 3. The lifecycle interior

The harness type, the run state, and the fan-out of loop 0, per
`weaver-admin-harness-contract` section 3 and the composition `load-unload-loop`
reads back.

```rust
pub struct Harness { /* private */ }

pub struct OrganBinaries {
    pub spu: PathBuf,
    pub gate: PathBuf,
}

impl Harness {
    /// The crate's one constructor. Adopts the coordination end the unit's
    /// declared open delivered and performs the hygiene of section 2.3.
    pub fn adopt(coordination: OwnedFd, organs: OrganBinaries)
        -> Result<Self, AdoptionFault>

    /// Serves the coordination channel until leave is answered or closure
    /// is observed, or fails on a fault below the exchange layer.
    pub fn serve(self) -> Result<Outcome, ChannelFault>
}
```

**Adoption fails only when a set fails, and a failed set refuses
construction.** A hygiene call that errors leaves the worker attachable or the
end inheritable, so `adopt` returns the fault naming the set rather than
proceeding unset, and a `Harness` in hand means the hygiene held. The fault's
shape is a satellite of section 9.

**The service is a serial loop and the channel state is a type.** One directive
at a time arrives, is judged against the channel's state, and is answered or
refused, per the ordering rules of `weaver-admin-harness-contract` section 4. A
directive out of order for the state answers `OutOfOrder` and is not queued.
The state has three positions, before enter, entered, and left, the last
terminal, and the middle one carries the run.

**The run state is the fan-out's progress held as data, which is what makes the
unwind total.**

```rust
struct Run {
    recorder: Recorder,
    spu: Option<OrganChannel>,
    gate: Option<OrganChannel>,
    turn_in_flight: bool,
}
```

Each `Option` is an arm of the enter fan-out that has or has not stood up, so a
leave arriving after a refused enter unwinds exactly what stands, stopping the
gate where a gate was raised and releasing the SPU where a model was admitted,
and the compiler's match on the options is what makes a forgotten arm
unrepresentable rather than unlikely. This is the mechanical form of
`load-unload-loop` section 4's rule that admin's unwind is a reap plus one
directive: the directive works because the harness knows what stands.

**Enter runs four steps in the charter's order, and the answer is the
aggregate.** Receive the sink descriptor from the directive's own message and
construct the `Recorder`, which stands the empty working structure up. The
count is a surfaced question: `weaver-admin-harness-contract` sections 3 and 5
say the trace descriptors, plural, `weaver-trace-Spec` section 5's receive
takes exactly one, and this Spec builds to the one while naming the divergence
rather than absorbing it, the reconciliation being the human's across a
two-party contract and filed on the working list. Author
the `load` event, the run's opening and the origin of its monotonic clock.
Create the residency pair, fork the SPU binary, and open the admit exchange
carrying the model binding uninterpreted. Create the gate pair only after the
SPU's answer has confirmed residency, per `weaver-harness-gate-contract`
section 1, then fork the gate binary and open the raise exchange carrying the
gate instruction uninterpreted, the gate last so no work arrives before the
interior serves, per apex section 6. The wait is what the run state's
invariant rests on: `gate` is set only ever after a confirmed `spu`, so the
unwind's reverse order is a property of construction order rather than of
timing. Ready is answered when the last arm confirms. A refusing
arm's reason is wrapped `OrganRefused` and carried into the aggregate
unchanged, per `weaver-admin-harness-contract` section 6, and the scoped
account holds: a refusal before the `load` event leaves the stream clean and
the state at before-enter, and a refusal after it leaves the authored bracket
standing and the run in place for the leave that unwinds it.

**Leave runs the reverse order and drains before it answers.** Lower the gate
first, refuse `ActivityNotAtRest` while a turn is in flight, author the
`unload` event, drain the writer's queue, and release the SPU. Left is
answered only after the drain returns, which is what makes the answer mean
what `weaver-admin-harness-contract` section 4 says it means, that everything
admitted reached the stream.

**Stop answers after the record holds the close.** The stop directive aborts
the turn in flight, the turn's close event is placed with the stop reason, and
only then does the answer carry `TurnAborted`, the announce-after-record
discipline of `weaver-admin-harness-contract` section 3. A stop at rest
answers `AtRest`, a clean close and not a refusal. How the abort lands at the
decoder is deferred with the decode seam, per section 8, and the trace
semantics are settled either way, which is what `basic-inference-loop` section
7 already records.

**A fault the worker survives is authored, not signalled.** The pressure and
failure reports the recorder surfaces, and the organ deaths observed through
closure after the enter aggregate, reach the operator as the `fault` event on
the stream, per the fault-carrier ruling of 2026-08-01. No run blocks on
anything downstream of the emission. The fault payload's shape is deferred
with the organs' charters, per section 8, so in this pass the path is shaped
and its submission gated on that landing, stated plainly rather than left to
be discovered at the first fault.

**Loop 0 takes neither a type nor a trait, and the cell closes here.**
`load-unload-loop` section 8 holds the question for the Spec pass,
demand-derived rather than reserved. The demand does not exist: the loop is
the interval between two directives, its state is the `Run` struct above, and
its control flow is the serial service, so an abstraction would have no second
implementor and no caller that varies, which is the reserved slot apex section
9 forbids. The inference loop inside it may yet demand one, and that demand
arrives with the token workflow, which may reopen this with the engine's shape
in hand.

## 4. Trace authorship

The authoring half of `weaver-harness-trace-contract`, landed as one module
with one submit path.

**Identity converts at the submit call, and nowhere else.** The envelope
identifies the session, run, and turn as `weaver-trace`'s opaque newtypes, and
this crate holds the floor's `SessionId` and `TurnKey`, so the conversion the
no-dependency rule of `weaver-trace-Spec` section 1 forces is a total function
at the one call site that submits, exactly as that Spec places it.

**Both timestamps are stamped at authoring, from the standard library's two
clocks.** Wall-clock milliseconds from the system clock, and the monotonic
reading as nanoseconds elapsed since the run's origin, an instant captured
when the `load` event is authored, per `weaver-harness-trace-contract` section
3. No OS crate is needed for either, and the recorder's clock is never
consulted because the contract denies it the fields.

**The licensed combinations are enforced here, before submit.** A message is
judged against the licensing rule of `weaver-traits-Spec` section 3, a `User`
message carrying only `Text`, an `Assistant` message carrying `Text` and
`ToolCall`, a `ToolResult` message carrying only `ToolResult` blocks, and an
unlicensed message is refused by this crate and never submitted. The recorder
cannot hold this rule, per that Spec, so the harness is the party the
perturbation test of `weaver-traits-Spec` section 7 binds, and the test lands
in this document's section 8 set.

**A refused submission is handled as the contract orders.** It is not treated
as recorded, not projected, and not retried under a new sequence, per
`weaver-harness-trace-contract` section 3. A refusal on the authoring path is
a defect in the author, and it surfaces as a fault rather than a retry.

**Pressure becomes an event, authored by this crate.** When the recorder
surfaces `CommitPressure`, the harness authors the `fault` event in response,
per `weaver-trace-Spec` section 6, gated in this pass as section 3 states.
Nothing on any turn path waits on the sink, per `weaver-harness-PRD` section
5, and the working structure's return is the acknowledgment the interior
proceeds on.

## 5. Prompt assembly's deterministic floor

**Assembly reads the message kinds and nothing else, and the discipline is a
kind filter at one site.** The harness assembles a prompt by iterating the
working structure in sequence order, selecting on the lifted `kind` the index
of `weaver-trace-Spec` section 4 holds for exactly this read, taking the three
message kinds, and decoding their payloads. The measurement, lifecycle, and
custody events never enter a prompt because the assembly path cannot see them:
the filter is the kind set at the read site, not a judgment applied after a
full read. This is
the seam `weaver-harness-PRD` section 2 names as what a later recall feature
would breach, held today by one match a reviewer can find.

**The per-model layer is deferred with the token workflow, and the floor
beneath it is fixed.** What phrasing elicits a tool call from a given decoder
is re-erected per model, per the charter, and no model is a dependency of this
crate. What does not vary and is fixed now is the order of parts, the identity
prefix, then the message sequence, then the tool schemas, per apex section 3
step 4, and the property that assembly is deterministic over the working
structure's contents: the same records assemble the same prompt, byte for
byte, which is what makes a replayed run's prompts comparable at all.

**The provider is injected, and this crate names no wire format.** Decode
requests leave through `provider-trait`, constructed at the worker composition
root, per `weaver-traits-Spec` section 6, and every deferred decode shape in
this document defers to the same place that trait's signature does, the token
workflow.

## 6. The tool system, blocked, and the engine, deferred

**Tool dispatch is blocked, and this Spec obeys the block.** `tool-trait` is
held by `weaver-traits-PRD` section 3.1 until the tool workflow, so
`src/tools.rs` is a placement, and what is stated now is only what the charter
already fixes: permission modes are consultation policy and not a boundary,
the kernel bounds what a tool reaches through the uid it runs as, and no
safety classification exists here or is coming, per `weaver-harness-PRD`
section 3. The tool subprocess inherits no descriptor this program holds,
which section 2 delivers by construction rather than by a per-tool argument.

**The engine is deferred with the decode seam.** The query loop runs until the
model returns a final answer or the operator's stop arrives, per
`weaver-harness-PRD` section 2, and everything the loop crosses, the decode
exchanges, sessions, sampling, the flush call, is deferred to the token
workflow by `weaver-spu-PRD` section 8. `src/engine.rs` is the second
placement, and the executor election deferred with it, per section 1.

## 7. The failure vocabulary

```rust
pub enum ChannelFault {
    Truncated { bound: usize },
    Undecodable,
    Closed,
}

pub enum Outcome {
    Left,
    ChannelClosed,
}
```

**Three faults, and the split from refusals is principled.** A refusal is a
typed answer on an exchange, and every refusal this crate issues or carries is
a `lifecycle-refusal`, the floor's type, drawn and never twinned. A channel
fault is a failure below the exchange layer: a truncated read, octets that do
not decode to an envelope and so cannot be attributed to any exchange for a
refusal to answer, and closure, which `weaver-organ-channel` section 2 rules
is observed as death and never synthesized into an answer. `Outcome` names the
two ways service ends, the leave answered or the far process gone, so the
composition root branches on a value rather than a guess.

## 8. What is enforced, and by which instrument

Per apex section 11. The threat walks are stated with their mechanisms, per
the threat-anchoring rule of the external review of 2026-08-01, and each test
below is a walk made executable.

**The reference walk: a rogue elected tool reaches for the trace.** The
adversary is the agent's own tool surface, a subprocess this crate forks per
call, running as the agent uid with `bash` as the reference case. The attack
is inheritance: without close-on-exec every tool subprocess holds a writable
handle to the trace and edits its own account through a descriptor requiring
no path and passing no check. The mechanism is the one receive site of
section 2.3 asking for `MSG_CMSG_CLOEXEC` in the receive itself. The test
forks a child and confirms it holds no trace handle, watched to fail when the
flag is removed from the receive, which is the test `weaver-trace-Spec`
section 10 owed this document, landed.

**The second walk: the same tool inherits a channel end.** The attack is an
inherited residency or gate end handing the tool surface a lifecycle
directive, a release above all. The mechanism is the atomic `SOCK_CLOEXEC` of
section 2.2, no window between creation and flag. The test forks a child and
enumerates its descriptors, confirming exactly the intended end crosses each
organ fork and nothing crosses a tool fork, watched to fail when the atomic
flag is dropped to a later `fcntl`.

**The third walk: a same-uid process attaches.** The attack reads or drives
the worker's descriptors through `ptrace` or `/proc/[pid]/fd`, no tool fork
required. The mechanism is the dumpable flag cleared at adoption, per
`weaver-admin-harness-contract` section 2, which reparents the proc entries
to root and refuses the attach. The test confirms the flag is clear after
`adopt` and clear again in a forked child's parent, watched to fail when the
clearing is removed.

**The fourth walk: the model reaches its own record through the prompt.** The
adversary is the model eliciting its measurement and custody events into
context, the recall-shaped breach `weaver-harness-PRD` section 2 names. The
mechanism is section 5's kind filter at the read site. The test assembles a
prompt from a structure holding measurement and lifecycle events and confirms
none of their content appears, watched to fail when the filter widens.

**Enforced by the compiler.**

- The run state's options make the partial fan-out representable and the
  unwind a checked match, so a forgotten arm is a compile error in the leave
  path rather than a leaked residency.
- The floor's three wire enums are exhaustive, so every directive, answer, and
  refusal case added later reaches this crate's matches loudly.
- Descriptors are owned types end to end, so a handle that escapes its owner
  is a move the borrow checker sees, not an integer copied silently.

**Enforced by compile-fail tests, because the property is an absence.**

- One constructor: code constructing a `Harness` other than through `adopt`
  fails to compile, the fields being private and no second path existing.
- No path-taking surface: doctests handing `channel.rs` a `&str`, a `String`,
  and a `PathBuf` where it takes owned descriptors each fail to compile,
  three named shapes with the general prohibition staying review's, per the
  split the floor Specs make.

**Enforced by the manifest.** The internal dependency set is exactly the two
floor links and the trace seam, read against the graph under gate H2. No async
runtime, no logging crate, and no HTTP client in the resolved external tree,
by the build-time `cargo tree` assertion the floor Specs share.

**Requiring a perturbation-verified test, beyond the four walks.**

- Truncation is a fault: an envelope over the 64 kibibyte bound produces
  `Truncated` and no directive, confirmed by watching a silently shortened
  directive decode when the `MSG_TRUNC` check is removed.
- Announce-after-record: a stop's answer follows the close event's placement,
  confirmed by watching the answer precede the record when the two are
  reordered.
- The scoped refusal account: a refusal before the `load` event leaves the
  stream empty, and one after it leaves a bracket with no `unload`, each
  confirmed by watching the account degrade when the authoring point moves.
- The licensed combinations: an `Assistant` message carrying a `ToolResult`
  block is refused before submit, confirmed by watching it reach the recorder
  when the check is removed, per `weaver-traits-Spec` section 7.
- Deterministic assembly: one working structure assembles one prompt,
  byte-identical across runs, confirmed by watching the comparison fail when
  iteration order stops being sequence order.

## 9. Open elections

Each names what settles it, and none is this Spec's to settle alone.

- **The engine's shape and the executor.** Deferred to the token workflow
  with a latency measurement on the decode path, per `weaver-traits-Spec`
  section 6 and section 1's no-runtime rule, which that measurement may
  overturn.
- **Stop mechanics at the decoder.** Whether stop cancels mid-decode or at
  the next boundary the SPU offers, per `basic-inference-loop` section 7.
  Settled against the SPU's cancellation surface when the token workflow
  charters it.
- **The tee back-pressure election.** Blocking, shedding marked, or detaching
  marked, per `weaver-admin-operator-contract` section 3. A measurement
  against a real consumer at a real rate, taken with the queue's high-water
  mark of `weaver-trace-Spec` section 11, the two settling together.
- **The fault payload's shape.** Awaits the organs' charters naming what they
  raise, per `weaver-spu-PRD` section 10, and until it lands the fault path
  of section 3 is shaped and gated.
- **The config read, and the sink field's custody question.** The charter's
  `reads` edge to `agent-config` is exercised by no chartered workflow, so
  the read arrives with the tool workflow, which consumes the tool set and
  the permission mode. That pass must also answer what section 5 of the
  charter makes awkward to leave implicit: the config's `trace-sink` field
  names the sink, the charter's custody prose has the agent never told the
  name, and the kernel's search-bit lock is what stands between knowledge
  and reach. Whether the read drops the field unretained, or the never-told
  sentence rescopes to the descriptor mechanism, is that pass's to elect
  with the charter in hand.
- **Tool dispatch, the execution context, and the permission-mode
  enforcement point.** Blocked with `tool-trait`, per `weaver-traits-PRD`
  section 3.1.
- **The satellite types.** `AdoptionFault`'s case set, `OrganChannel`'s
  exchange-surface spelling, the licensing error's shape, and the
  channel-state enum's name. Identifier and shape choices with no
  cross-crate consequence, listed so what this Spec leaves to a builder is
  complete rather than implied.
