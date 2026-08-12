# HANDOFF 2026-08-12, the turn authors its trace

Version: v0.1, 2026-08-12. From the authoring seat at the close of the harness
half to the session that picks the work up. This is neither a batch nor a
commission under Handoff Format v0.3: no documents are commissioned and no edits
ride it. It carries where the work sits, the facts the next acts rest on, the
issue register with its lanes, and the workshop a fresh session inherits. Read
against `main` at `83baa5c`. Advice returns in Working Process section 1 shape:
facts found, advice offered, requests to reopen.

## 0. What runs now, and what it means

**The reasoning loop completes a turn.** On `main`, loop 1 hands the harness a
delta, and loop 0 opens the turn, sends append-and-generate over the decode
socket, consumes the streamed tokens as the model draws them, and authors a
seven-event bracket: the user's turn, `model.request`, `model.output`,
`model.measurement`, the assistant's turn, and the close. The three model events
obey the custody model, the request and the measurement spliced verbatim from
what the SPU rendered and the output shaped from the emission the harness
consumes. The turn method is watched whole against a scripted decode peer and
the decode seam runs green end to end on the A6000 against real qwen2.5 weights.

**What this is not yet.** The turn method is built and proven, but nothing in a
running worker calls it. `serve()` handles the coordination directives, enter,
leave, and stop, and reaches loaded-and-idle, but no path invokes loop 1's turn
between enter and leave. In the wired system turns arrive over the gate's
turn-frame, whose interior is unspecified, so a live turn waits on one of the
two acts of section 2. The center is built. The last wiring is not.

## 1. The conformance and census state, verified at the tip

Counted over the `conforms:` headers against the `graph` blocks, at `83baa5c`:

    weaver-types      17      weaver-harness   49
    weaver-traits     24      weaver-gate      23
    weaver-trace      38      weaver-admin     31
    weaver-spu        58

The SPU stands at 58 of its 60, the two open being the tap's:
`spu-one-forward-per-prompt` and `spu-two-taps-one-shape`, both waiting on the
GGUF readout tap the Spec authorizes and no act has written. The harness reads
49 of the 51 the decode-surface act chartered, the two open being
`harness-session-opens-at-enter`, buildable and owed only its conforms header,
and `harness-stop-polled-during-the-stream`, the stop-multiplexing act's. The
trace settled at 38 when the model-events splice retired
`trace-measurement-absent-not-zero`.

**The census stands at 295 nodes and 431 edges**, moved by the acts of the past
week: the route act's three `draws` edges, the decode-surface act's three
harness assertions, and the model-events act's one retirement. Working Process
section 7 trails the code and is owed a catch-up, per the standing habit. Verify
against `docs/project/open-items.md`, the gitignored working list, rather than
trusting this paragraph. **HADES is unavailable, so the graph cannot be queried
or rebuilt**, and the corpus text is the only census.

## 2. The acts owed, in the order they unblock the demonstration

**The composition that invokes loop 1, the nearest to a live turn.** Something in
the running worker must, between enter and leave, hand loop 1 the granted seat
and let it drive turns. Two shapes present themselves and the choice is real. One,
the gate's turn-frame delivers the turn: the harness reads a frame off the gate
channel, renders it to a delta, drives the turn, and answers a response frame.
This is the chartered path, and it is blocked on the `TurnFrame` election, issue
124's sibling, the frame's interior deferred to a measurement over real client
traffic the gate's own Spec produces. Two, a loop-invocation charter names how
loop 0 pauses at loaded-and-idle to run loop 1 directly, which the basic
inference loop needs regardless of the gate. Either is a documents act before it
is code. The turn method is proven against a scripted peer meanwhile, so this act
wires a caller to a working callee rather than building both.

**The stop polled during the stream.** Section 6.1 charters it and it is the
trickiest piece deferred from the harness half: while a generation streams, loop
0 waits against the decode channel and the coordination listener at once, and a
stop dialed mid-stream cancels the turn at the seam. It re-enables the `poll`
feature the harness half deferred, which returns to the manifest with this act
that uses it, per the no-dangling-capability rule. Two flagged elections from the
decode-surface Spec ride here for the operator's confirmation: that the turn
authors internally rather than handing loop 1 a lower surface, and that the stop
multiplexes by `poll` rather than a thread.

**Then a turn runs in a live worker**, gate ingress to trace out, which is the
apex's demonstration and the deliverable's proof.

## 3. The register, triaged into lanes

**The harness lane, where the next acts sit.**

- **124, the operator's ruling, the one decision blocking record fidelity.**
  Does `model.request` record the full effective prompt the model received, or
  the turn's delta with the full context reconstructable by accumulation. Both
  satisfy apex section 8's reproducibility rule, the first is PRD-literal and
  has the SPU re-render its resident session each turn, the second is cheap and
  rewords the PRD. The code takes the delta reading meanwhile. This is the
  operator's, and it wants ruling before the SPU's request rendering is final.
- **106, the fault report's shape, high priority.** The decode seam's fifth
  exchange, the SPU-opened fault report, is unbuilt because `FaultReport`'s
  shape is an open election that converges with the run bracket's payload and
  the custody rule. Named as the harness lane's judgment, taken deliberately
  rather than mid-act, with two reopen requests recorded on the issue.
- **104, the custody rule gathered, and 103, the subsystem granularity.** The
  organ-renders-content rule now has its worked example landed in the model
  events, and 104 is the document that gathers it. 103 gives the trace's
  `Subsystem` its engine-grade cases, `spu_decoder` with its first emitter,
  which the model events now provide.
- **105, one draws edge** the admin-harness contract owes for the gate
  instruction, orthogonal and small.

**The boundary lane, gating the tool workflow.**

- **115, the reversal act's extent**, the corpus corrections the ratified
  reasoning-loop boundary forces, the apex's outer-membrane phrase deepest among
  them. **116, the type-level closure audit**, shaping the floor so a tool result
  is obtainable from the gate alone, with the conversation-model deserialization
  hurdle named going in. Both gate the tool workflow and nothing earlier.

**Waiting on a turn or on their own tracks.**

- **102, the decode-loop baseline**, transfer against compute per leg, which
  wants a completed live turn to measure and carries the splice-contingency the
  encoding election rests on.
- **88, the registry's template keying**, consequence B alone, SmolLM2 taking a
  silent wrong template. **93, the encoder's deferral argument** sharpened from
  the order-of-construction. Both SPU side-work, neither this lane's.

## 4. Facts the next acts rest on

**The seam is append-only.** The SPU holds the resident session from the open at
enter, so each turn crosses only its delta and the SPU accumulates. The harness
sends no full prompt per turn. This is why issue 124 matters: what the model
received grows each turn while only the delta crosses.

**The model events splice, the output is shaped.** `Generation` carries
`emission` and `finish` typed, which the harness consumes and shapes into
`model.output` and the assistant message, and `request` and `measurement` as
opaque `RawValue` the harness splices into `model.request` and
`model.measurement` without reading. The SPU renders both blobs. The harness
checks each spliced member's conformance at the author call, which is where the
splice's opacity is paid for.

**The turn closes its bracket on every exit.** A failed turn authors
`turn.closed` with `StopReason::Fault`, and a failed close returns `ChannelLost`
because the record is then untrustworthy. A model-side stop, capacity or the
model's own stop token, closes clean, its truncation recorded in
`model.output.finish`, distinct from the directive-or-fault abort
`TurnClose::Stopped` names.

**`Ports` is a borrowed seat.** It holds the decode channel, the author, the
recorder, and the turn counter, lent across the extension seam for the run's
life, so a loop holds the surface exactly as long as loop 0 lends it. The blade
holds: the constructor is crate-private and a loop that needs a port this
surface does not offer is a charter edit, pinned by an `E0624` compile-fail
doctest.

**The unbounded organ receive is the merged pattern.** `exchange()`, driving
admit, release, and lower, blocks on `channel.recv()` against the harness's own
credential-checked forked child, and `open_session` takes the same shape.
Bounding is a whole-lifecycle design change, not a gap the harness half
introduced, and it waits on the act that decides it for every organ exchange.

## 5. Non-negotiables, restated for a fresh context

No code without a merged Spec, and the direction never inverts: a change found
necessary in code reopens phase one for that piece. The harness is the sole
writer of the trace and the organs render their own content. The splice is
spliced, pre-serialized JSON as members, never a quoted string, and absence is
absent, never empty. One directive receives exactly one answer. Tools are
outside the loop and reached through the gate, per the ratified boundary. Every
edit under `docs/` and `process/` passes the G1 greps before it lands. A batch
names its base commit. Commits are corpus-voice prose sentences. CodeRabbit
reviews on the PR, findings are verified against the documents before they are
applied, and a review comment answered in prose stays an open thread until it is
resolved on the PR, so resolve the threads. A session states its seat before it
begins.

## 6. The workshop, for a session that was not here

The toolchain restores itself: `rust-toolchain.toml` pins the nightly and the
`rustup` shim installs it on first `cargo` run. The full-feature build wants the
CUDA environment: `CUDA_PATH=/opt/cuda`, `PATH` prefixed with `/opt/cuda/bin`,
`NVCC_CCBIN=/usr/bin/g++-15`, and `CMAKE_CUDA_ARCHITECTURES=86` for the A6000.
**The host holds CCCL at 3.3.4 deliberately**: CUDA 13.3 ships CCCL 3.4.2, the
pinned llama.cpp's CUB gate breaks against it, and an unguarded `pacman -Syu`
restores the breakage. The `llama-cpp-sys-2` build script carries a
check-then-link race when two feature sets share a target directory, and the
archived tree's `CLAUDE.md` line 57 carries the sweep that clears it. The device
seam tests want the `cuda gguf` feature pair and skip loudly without the qwen2.5
fixture at `/opt/weaver/models/qwen2.5-0.5b-instruct-q6_k.gguf`. Featureless the
suite is 248 tests over 42 targets, the full-feature run adds the device seam
tests.

HADES is unavailable, so the graph can be neither queried nor rebuilt, and no
copy of the mapper survived the reinstall. The corpus is the only census, 295
nodes and 431 edges expected. The review seat is unreachable while HADES is down,
so acts land on the authoring seat with CodeRabbit and the operator as the
checks, stated rather than slid past.

## 7. Reading order

1. `process/WeaverTools-Working-Process.md`, the boot prompt, section 7 with the
   catch-up caveat above.
2. `docs/project/reasoning-loop-boundary.md` and its companions, the ratified
   frame the whole program now reads inside.
3. `docs/crates/weaver-harness/weaver-harness-Spec.md` section 6.1, the decode
   surface the turn implements, and section 6, loop 1's seat.
4. `docs/crates/contracts/weaver-harness-spu-decode-contract.md`, the seam the
   turn drives, streaming and all.
5. `crates/weaver-harness/src/engine.rs`, the turn method and the granted seat,
   the summit of what the harness half built.
6. `crates/weaver-harness/src/lifecycle.rs`, the enter fan-out that opens the
   session and the leave that closes it, and `crates/weaver-spu/src/main.rs`,
   the serve loop that answers every exchange the turn dials.
7. `docs/crates/weaver-harness/weaver-trace/weaver-trace-Spec.md`, the record's
   shape the turn authors into.
8. Issues 124, 106, then 115 and 116 for the boundary lane.
9. `docs/project/open-items.md`, the working list, gitignored and current.
