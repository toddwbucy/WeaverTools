# HANDOFF 2026-08-10, the harness half of the turn

Version: v0.1, 2026-08-10. From the authoring seat at the close of the seam
service act to the session that opens the harness lane. This is neither a batch
nor a commission under Handoff Format v0.3: no documents are commissioned and no
edits ride it. It carries where the work sits, the facts this seat found that
the next act rests on, the questions that act must settle, and the reading
order. Read against `main` at `b1eae43`. Advice returns in Working Process
section 1 shape: facts found, advice offered, requests to reopen.

## 0. What this is, and what the act is

The deliverable is one complete turn: work in at the gate, through the loop,
decode at the SPU, and a turn-bracketed trace out, per apex section 3. As of
`b1eae43` the SPU serves its half whole. A worker admits over the lifecycle
socket, opens a session over the decode socket, renders and generates against
real weights, answers with the generation and its measurement, flushes, and
releases clean, watched end to end on the device. What does not exist is the
harness's half: nothing dials those exchanges, nothing converts the working
structure into an open and an append, and nothing authors the three `model.*`
events from what returns. That is the act this handoff feeds, and it is the act
that makes the first complete turn reachable.

Working Process section 7 reads 56/60 for the SPU and is one catch-up behind:
the seam act bought `spu-out-of-order-refused-on-decode` and
`spu-fault-below-the-exchange-layer`, both claims cleared by the operator at
review, so **the authoritative figure at this handoff's date is 58/60** with
the remaining two on the readout tap, and this paragraph is the tracked
snapshot. `docs/project/open-items.md` is the working list on this workshop's
tree, gitignored by design, so it is reachable from a fresh session here and
absent from a fresh checkout: where it and this snapshot disagree, the newer
of the two is the finding.

## 1. What the harness already holds

More than the section 7 summary suggests. `weaver-harness` already creates the
decode pair beside the lifecycle pair before the fork, holds its end as
`DecodeChannel`, an own type per `harness-decode-end-own-type`, and carries the
model binding's successor, the SPU instruction, uninterpreted through the admit.
The `Ports` seat in `engine.rs` is the granted surface a loop composes over,
constructor crate-private, holding the assembled prompt today and naming the
decode surface as what arrives with this act. The assembly path converts the
working structure's records into canonical messages, counting what did not
decode rather than hiding it. The trace side is settled: the three `model.*`
payload shapes stand in `weaver-trace` with the splice discipline throughout,
and the harness is the sole writer, authoring at one submit call where the
kind-to-payload pairing is enforced.

## 2. Facts found this session, load-bearing for the act

**The phase discipline is merged and binding.** The SPU serves the lifecycle
seam until admitted, then the decode phase owns the process until the harness
closes its decode end, then the lifecycle seam again for the release. The
harness side must therefore close its decode end to end the phase before it can
release, and must treat its decode end's lifetime as the session's.

**The leave path is defective against that discipline, filed as issue 113.**
The harness binary sends Release before dropping decode and never reads the
answer, so the release lands in a phase that cannot hear it and the worker
exits failure where the contract promises a confirmed release. The seam tests
order it correctly. The act fixes the binary's path: decode drops first, then
the release runs as the exchange it is.

**The wire is bare trio JSON, one frame one message.** No envelope on the
decode socket, per the charter: a frame is a serialized `TokenDirective` in,
and a `TokenAnswer` or `TokenRefusal` out, the two distinguished by parsing,
their tag vocabularies disjoint today. `TokenAnswer` is adjacently tagged
under the fourth arm of the floor's tagging test, the spliced measurement
being why, so the harness's read side must parse `{"kind":"generated",
"body":{...}}` with the measurement as a member of the body.

**The dispositions are all frozen and the map must be empty.** The SPU's
composition root freezes every knob and refuses any tunable entry as
`MalformedDelta`, the engine taking its knobs at open. The harness sends an
empty map until the act that carries a knob to the engine per turn.

**The measurement's optional members are absent, not empty.** `entropies` and
`surprisals` appear only when produced. `blocks` carries text-offset spans over
the turn's rendered delta, labelled from the declared vocabulary, `turn-delta`
alone today. `timings` rides the decimal-string rule. The conformance check at
the submit call is the splice's stated price and this act is where it is paid.

**`model.request` has no wire source for its `rendered` member.** The trace
shapes the request side as the rendered prompt as the family library produced
it, and the family library is the SPU's: nothing in the decode contract's
answer enumeration carries the rendered text back. The act must settle this,
and section 4 states the options.

**The mid-turn stop is a multiplexing question the harness has not faced.**
The stop directive arrives on the coordination socket while a generation is in
flight, and the abort lands at the decoder as a cancel frame, the harness's
interior per the contract. But the harness waiting on the decode answer is not
reading the coordination socket, and its nix feature set carries no poll. The
SPU's precedent is phase discipline plus a non-blocking peek inside the wait.
What shape the harness takes is the act's to elect, and a new syscall on the
OS surface is a Spec edit before it is a line of code.

**A cancel that stops a generation closes second.** The outstanding append
answers first with the partial output marked stopped, then the cancel answers
at rest. The harness's read side must expect two frames in that order.

## 3. The issue cluster, triaged for the lane

- **113, in the act.** The leave-path fix above, first because everything
  else exercises the path it repairs.
- **103, in the act.** The first `model.*` event needs a `Subsystem` value
  and `spu` is the wrong answer once the SPU holds more than a decoder. The
  `spu_decoder` case lands with its first emitter, here, and the enum's
  growth-rule comment corrects with it. A trace floor edit inside this act,
  documents first.
- **106, the lane's judgment, not this act's blocker.** The fault report's
  shape, the run bracket's payload, and the custody rule converge on one
  election. The four served exchanges carry no fault report, so the turn
  completes without it, and the judgment is taken deliberately afterward
  rather than absorbed mid-act. It is labelled high-priority for the lane,
  and its issue carries the two named reopen requests.
- **104, informs the act.** The custody model, the trace owns the boxes and
  the organ owns the contents, is how the submit call's conformance check
  should read. The general rule lands with 106's act.
- **102, after the turn.** The baseline wants a completed turn to measure.
  The act that finishes the turn is the act that makes it possible.
- **105, orthogonal.** One draws edge on the admin-harness contract, its own
  small act whenever convenient.
- **88 and 93, SPU side.** Not this lane's.

## 4. Asked of the receiving session

    The open's timing and material  When does the harness open the decode
      session, at the enter fan-out or lazily at the first turn, and what
      messages are the identity material rendered from? The contract says
      once per residency, after residency confirms. The working structure at
      enter is empty by construction, which argues the identity material is
      configuration rather than history, and nothing in the merged set names
      its source today. This may be a small documents act before the code.
    The rendered prompt's path  model.request wants the rendered prompt and
      the wire does not carry it back. Three shapes. The answer's measurement
      grows a rendered member, a contract amendment with a floor edit. The
      request event is authored from the canonical delta the harness sent,
      which reopens the trace charter's description of the member. Or the
      request's rendered member is deferred with the event authored without
      it, which the trace Spec's required-member shape refuses today. Each
      is a documents act first, and the choice is the operator's to bless.
    The multiplexing shape  How the harness hears the coordination socket
      while a decode answer is outstanding, section 2's stop question. Name
      the mechanism, check it against the harness Spec's OS surface, and
      write the Spec edit before the code if the surface grows.
    The claims  The turn's completion likely buys nothing in the SPU's
      register, the remaining two riding the tap, but the harness's own
      conformance header set may grow. Claims are proposed at review and the
      operator blesses them, per the standing practice.

## 5. Non-negotiables, unchanged and restated for a fresh context

No code without a merged Spec, and the direction never inverts. The harness is
the sole writer of the trace and the organs render their own content. The
splice is spliced: pre-serialized JSON as members, never quoted strings, and
absence is absent, never empty. One directive receives exactly one answer on
the exchange that asked. The harness carries what admin sent uninterpreted.
Every edit under `docs/` and `process/` passes the G1 greps before it lands. A
batch names its base commit. Commits are corpus-voice prose sentences.
CodeRabbit reviews on the PR, findings are verified against the documents
before they are applied, and disagreements are surfaced to the operator, who
adjudicates. A session states its seat before it begins.

## 6. The workshop, for a session that was not here

The toolchain needs one prerequisite, `rustup`, installed from the repos.
Its shim reads `rust-toolchain.toml` and installs the pinned nightly with
its two components the first time cargo runs, and cargo fetches the
dependencies. The full-feature build wants the CUDA
environment: `CUDA_PATH=/opt/cuda`, `PATH` prefixed with `/opt/cuda/bin`,
`NVCC_CCBIN=/usr/bin/g++-15`, and `CMAKE_CUDA_ARCHITECTURES=86` for the
A6000. **The host holds CCCL at 3.3.4 deliberately**: CUDA 13.3 ships CCCL
3.4.2, the pinned llama.cpp's CUB gate breaks against it, and an unguarded
`pacman -Syu` restores the breakage. The `llama-cpp-sys-2` build script
carries a check-then-link race when two feature sets share a target
directory, and the archived tree's `CLAUDE.md` line 57 carries the sweep
that clears it, cited as a fact about the build. The qwen2.5 fixture lives
at `/opt/weaver/models/qwen2.5-0.5b-instruct-q6_k.gguf` and the seam tests
skip loudly without it. Device tests serialize on a lock and want the
`cuda gguf` feature pair.

HADES is unavailable, so the graph can be neither queried nor rebuilt, and no
copy of the mapper survived the reinstall. The corpus is the only census: 293
nodes and 429 edges expected at the next rebuild, the route act's three draws
edges being the whole difference from the standing 426. The review seat is
unreachable while HADES is down, so acts land on the authoring seat with
CodeRabbit and the operator as the checks, stated rather than slid past.

## 7. Reading order

1. `process/WeaverTools-Working-Process.md`, the boot prompt, section 7 with
   the one-catch-up caveat above.
2. `docs/crates/contracts/weaver-harness-spu-decode-contract.md` whole. The
   act implements its harness side.
3. `weaver-types-Spec` sections 4.3 and 4.4, the trio's shapes and the
   fourth arm's tagging.
4. `weaver-harness-Spec` sections 1 and 2, the featureless link and the OS
   surface the multiplexing question presses on.
5. `weaver-trace-PRD` sections 3.1 and 3.2 and `weaver-trace-Spec`'s model
   payloads, the boxes the act fills.
6. `crates/weaver-spu/src/main.rs`, the serve loop, the other half of every
   exchange this act dials.
7. `crates/weaver-harness/src/lifecycle.rs` and `engine.rs`, the enter
   fan-out, the defective leave path, and the Ports seat.
8. Issues 113, 103, 106, 104, 102, in that order.
9. `docs/project/open-items.md`, the working list, gitignored by design:
   present on this workshop's tree and absent from a fresh checkout, which is
   why section 0 carries the dated snapshot.
