# weaver-spu - Spec

**Status:** DRAFT. Cut 2026-08-02, seventh of the Spec pass and the last of the
set. No code is written against it until phase three is ratified, per Working
Process section 6.

**Date filed:** 2026-08-02
**Revised:** 2026-08-03, the device-assignment ruling. Section 3 takes the
devices from the binding and selects none, judging a set on room, peer
reachability, and the backend's declared shard width, with two the width the
salvaged path proves. Section 5's family surface gains that declaration as a
set of widths rather than a maximum, and section 11 files the N-way path as
work rather than salvage.
**Document ID:** `weaver-spu-Spec`
**Parent:** `weaver-spu-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

Build instructions for `weaver-spu`: the binary's layout, the two channel ends
it holds, the residency mechanics, the decode submodule's interior, the family
libraries' shape, and the elections a builder would otherwise invent. It is
derived from `weaver-spu-PRD` and from the two contracts this crate is party
to, `weaver-harness-spu-contract` and `weaver-harness-spu-decode-contract`,
together with `weaver-organ-channel`, the drawn material the first of them
draws.

Level discipline. The charter says what the crate needs and why. This document
says how it is represented, and per gate G2 it elects against grounds the
charter and the contracts state rather than developing grounds of its own.
Where this document and the charter disagree the charter yields nothing.

**This document declares no graph records,** per Document Format section 1.
The charter is the source of this crate's node, its parent edge, its two floor
links as this act corrects them, and its two declared seams.

**It is written from the merged corpus, with the salvage survey as its quarry
map.** `docs/project/weaver-spu-salvage-survey.md` records what the archived
tree holds, what maps across, and the seven places its working code yields to
merged rulings, the seventh filed with the device-assignment ruling. This
document elects against the merged corpus and cites the
survey where a mechanic's provenance matters, per the ruling of 2026-08-01
that keeps the old tree's Specs out of the pass.

**Two corrections to merged documents land in this act, because this Spec
cannot be written truthfully without them,** and both are the token
workflow's own reach arriving late. The charter's section 6 said this crate
does not link `weaver-traits`, and the framing ruling made the family library
render the harness's canonical messages, so it does. The harness Spec's
handoff placed one inherited descriptor at number 3, and the decoder cut gave
this crate a second channel, so the placement needs two numbers and an order.
Each is corrected where it lives, in this act, rather than filed.

## 1. The crate

**One binary, forked and exec'd by the harness during the enter fan-out,** per
apex section 12. Nothing links this crate: it is a process the harness starts,
not a library it calls, which is what the seam being a socket rather than a
link means read as a manifest fact.

**Layout.** The crate is the umbrella's substrate, per charter section 2: the
family libraries and the socket presentation, with each semantic domain in its
own submodule. One submodule exists.

    src/main.rs             entry, the hygiene sets, the service loop
    src/channel.rs          the two channel ends and envelope I/O, section 2
    src/residency.rs        admit and release, the device, section 3
    src/family/mod.rs       the family surface and its registry, section 5
    src/family/<name>.rs    one module per family, template and parsing
    src/decoder/mod.rs      the decode submodule, section 4
    src/decoder/session.rs  the resident session and its append path
    src/decoder/backend.rs  the backend seam, GGUF and native as peers
    src/decoder/gguf.rs     the GGUF backend
    src/decoder/native.rs   the candle-native backend
    src/decoder/measure.rs  measurement production, section 6
    src/decoder/readout.rs  the residual tap, section 7
    src/gpu/                the CUDA forward path, sharding, and its kernels
    kernels/transformer.cu  the salvaged kernels, section 10

**A later operation type is a sibling of `decoder/`, in its own process.**
Per charter section 13.1 the encoder and the other operation types arrive as
their own submodules with their own sockets, and per the operator's ruling of
2026-08-02 each runs in its own process under this domain root. Nothing is
laid in for them: no trait, no variant, no feature, no socket bound early. What
this layout does provide is that adding one is adding a directory beside
`decoder/`, which is the reversibility test passing on this crate's own future.

**Edition and toolchain.** Edition 2024 on the pinned nightly. Unlike the
floor, this crate may use a nightly feature where the GPU path needs one, and
names any it takes at the site, because a nightly requirement here is a
requirement in one binary rather than everywhere.

### 1.1 The dependency set, and each is argued

**Internal, and one of them is a correction.** `weaver-types` is the charter's
floor link, taken **without its `config` feature**, because this crate reads no
configuration file, per charter section 3, and a parser in a process whose
argument is that it holds little is the weight `weaver-types-Spec` section 1
gates against. **`weaver-traits` is now a second floor link, which corrects
charter section 6 in this act.** That section said this crate draws no trait
and ended by leaving the answer to the decode workflow, and the decode workflow
changed it: the framing ruling of 2026-08-02 has the family library rendering
the harness's canonical messages, and
`weaver-harness-spu-decode-contract` section 7 draws the message model from
`weaver-traits` into this crate accordingly. The link follows the draw, per
apex section 5.3, which is what makes the party list checkable against the
dependency graph. `provider-trait` is still not implemented here and the
charter's reasoning for that stands untouched: the abstraction lives at the
harness's composition root, on the far side of this seam's transport.

**External, the model half.** The two fork pins are the load-bearing
dependencies of the whole crate and the survey names them preconditions.
`llama-cpp-2` and `llama-cpp-sys-2` come from the same git source and revision,
never one from the fork and one from the registry, because two sys crates
resolve to two distinct types for one C library and the flash-attention policy
type stops typechecking across them, which the archived tree recorded against
itself. The fork exists for one reason, the ggml scheduler's eval callback,
which is the only route to per-layer activations from a GGUF model without
replacing the engine, and section 7 makes that seam a compile-time pin rather
than a comment. `candle-core`, `candle-nn`, and `candle-transformers` come from
their own pinned fork for `forward_with_intermediates`, the readout's working
path. `cudarc` is caret-pinned rather than exact, so Cargo unifies this crate's
device handles with candle's inside one minor line, exactness coming from the
lock file rather than from a requirement that becomes unsatisfiable the day
candle raises its floor.

**External, the rest.** `serde_json` for the lifecycle channel's envelopes,
whose JSON election is the floor's. `nix` for the OS surface, on the grounds
`weaver-harness-Spec` section 2.4 argued and this crate inherits: the calls are
`recvmsg`, `fcntl`, the dumpable `prctl`, and the descriptor adoption of
section 2. `safetensors` for the native backend's weights, `half` and
`bytemuck` for host-side tensor math, `tokenizers` for the family tokenizers,
`blake3` and `walkdir` for the weights hash of section 3.

**What does not cross, from a working tree that had it.** No HTTP client and no
model downloader: an artifact this crate cannot reach is a refused admit, per
charter section 4.1, and fetching one would make this crate a provisioner of
the operator's own artifact. No logging crate, per the corpus's one-account
rule. No async runtime in this pass, the service being one directive at a time
against one resident session, and the executor question belonging to the
measurement `weaver-traits-Spec` section 6 names. No multi-model catalog and no
tenancy apparatus, both dissolved by one SPU per agent, per the survey.

**Feature gates are two and each names what it buys.** `cuda` compiles the
kernels of section 10 and the native backend's device path. `gguf` compiles the
llama.cpp backend. A build with neither compiles the channels, the residency
bookkeeping, and the family libraries, which is what keeps the family surface
testable on a machine with no device.

## 2. The two channel ends, and the process facts

**Both ends arrive inherited, at descriptors 3 and 4, and the numbering is
this act's election.** `weaver-harness-Spec` section 2.2 elected descriptor 3
for an organ's inherited end when an organ held one, and the decoder cut gave
this crate a second. The election here is the order: **3 is the lifecycle
channel and 4 is the decode socket**, lifecycle first because it is the
channel every organ has and the one an organ with a single end already places
there, so the gate's placement is unchanged and this crate's first end sits
where the gate's does. The harness Spec's handoff paragraph is corrected in
this act to place ends in that order rather than to name one number.

**Entry adopts both, then performs its two sets, before the first read.** The
process wraps descriptors 3 and 4 as owned handles, sets close-on-exec on
both, and clears its dumpable flag, per charter section 7. Both are sets and
never checks, per the rule `weaver-organ-channel` section 2 states: a step
that finds a flag wrong and reports leaves the descriptor inheritable and the
process attachable, which is the condition the set exists to prevent. The
close-on-exec set matters here even though this crate forks nothing, because
`execve` clears the flag and the requirement is stated against the last exec.

**Entry verifies that it holds exactly two descriptors beyond the standard
streams, and refuses to serve if it holds more.** Charter section 7 makes a
build in which this crate holds a trace descriptor broken whether or not it
writes through one, and the harness's fork discipline is what keeps that true,
but a property this crate depends on and cannot see is a property worth
checking at the one moment it is cheap. The check is a count rather than an
identification, since this crate cannot know what a stray descriptor refers
to, and a count above two means the discipline upstream failed and this
process is not the one to continue past it.

**The lifecycle channel carries the organ envelope and the decode socket does
not.** Per charter section 13.2 the decode socket is not an organ channel, so
the envelope crosses only on the first end, and the token trio crosses the
second in whatever encoding the measurement of section 11 elects. Both ends
carry the receive obligation the `SOCK_SEQPACKET` election attaches: a buffer
sized to the envelope bound on the lifecycle channel, a read returning with
`MSG_TRUNC` set treated as a channel fault and never a message, and the same
bound asserted on this crate's own writes.

**The service is serial per channel and the two channels are one loop.** One
lifecycle directive at a time, one decode exchange at a time, per both
contracts' ordering, and the decode socket's traffic is what the process
spends its time on. Nothing here is concurrent in this pass, and the shape
that would change it is the executor election deferred in section 1.1.

## 3. Residency

The two exchanges of `weaver-harness-spu-contract`, in the charter's order.

**Admit runs the charter's five steps, and the first three are free.**
Resolve the binding to an artifact, read what the artifact declares about
itself without loading it, judge the assigned devices, take them in shard order
and load each shard, confirm.
The header read is the salvaged mechanic the survey names: parsing an
artifact's header and metadata answers what family this is and what its
dimensions are without touching tensor data or the device, which converts the
common shape of a bad binding, an artifact present and wrong, into a refusal
costing no device work.

**The devices are the binding's and this crate selects none.** The assignment
arrives inside the model binding, per `weaver-types-Spec` section 2, ordered,
with the order the shard order. There is no device survey, no ranking, and no
fallback in this crate: the archived tree's `auto_select_gpu` does not cross,
per charter section 3 and the salvage survey's seventh yielding. What this
crate does with the set is judge it, in the order section 3's steps state.

**A set larger than one is judged on three things rather than one.** Each
assigned device must have room for its shard plus the residency's headroom,
which is the one inequality read per device. The devices must be able to reach
each other, checked by asking the driver whether peer access holds between each
pair in the set, because a sharded forward exchanges activations across them
and a set without peer access is a set that cannot serve, discovered at admit
rather than at the first turn. And the family's declaration must say the
backend can shard across that many, per section 5. **Today that number is two
where it is greater than one,** because the salvaged tensor-parallel path is a
two-device implementation, `forward_tp2` with an all-reduce kernel written for
a pair, per the survey. A wider set refuses against the declaration rather than
against a hidden limit, so the day an N-way path lands the declaration changes
and nothing else does.

**The device judgment reads the driver rather than this crate's own
accounting, and the charter's cell is settled that way.** Charter section 10
holds the question open with the archived tree's answer recorded as a
preference without a reason: that tree held its own allocation ledger as the
authority and marked the driver query as diagnostics only, which prefers the
number that cannot see the thing the check exists for. This crate holds no
fleet ledger to prefer, per charter section 3, and the case it exists to catch
is a device occupied by something this program did not put there, so the
authority is what the device reports free at the moment of admission. **The
cost is named rather than hidden:** one driver query on the admit path, which
happens twice per residency and never inside a turn.

**The headroom term stays a construction parameter until a measurement
replaces it.** Charter section 9 stages the figure and names the entry
condition, a measurement on a real artifact against a real device, so the
admission inequality takes the headroom from the worker's composition root the
way `weaver-trace-Spec` section 6 takes its queue depth: a deployment fact
rather than an operator election, and a number a builder can supply before the
measurement exists.

**The weights hash is computed at admit and travels with every measurement.**
The salvaged mechanic is BLAKE3 over a canonical manifest, a single file or a
walked directory, with a sidecar cache and an **empty-string sentinel on every
failure path**, which is the property worth carrying verbatim: a hash that
cannot be computed reports that it could not rather than reporting a wrong
value, and apex section 8 rests replay on the identity being right.

**Release frees the device before it answers, and the ordering is the
contract's.** Stop serving, free the weights and the working allocations and
the cache together because they are one residency, then confirm. A
confirmation is a fact about the device rather than a statement of intent, per
charter section 4.2, and the archived tree's inverse ordering is what its own
record names as producing an overcommit.

**Nothing is idempotent and nothing retries.** This crate begins empty, admits
once, and dies, so a second admit has no prior residency to match, per charter
section 4.1, and the archived tree's idempotence rule loses its premise with
the daemon it was written for.

## 4. The decode submodule

### 4.1 The backend seam

**One trait, two backends, peers rather than a legacy and a target.** The
archived tree's own ruling of 2026-06-11 made the GGUF and native paths
first-class peers and the survey carries the reasoning forward: GGUF owns
quantized artifacts on consumer devices, and the native path owns what a
tensor-parallel forward and a fine-tunable artifact need, since a GGUF cannot
be fine-tuned and a program that intends training as a continuation cannot let
that path decay. The seam is one trait over open, append-and-generate, cancel,
flush, and close, with the family library above it and the device beneath.

**Which backend serves is a property of the artifact, decided at admit.** The
header read of section 3 already answers it, so nothing elects a backend
separately and no configuration field names one.

### 4.2 The session

**Append-only, `resident_len`, and nothing ever rewinds.** The session holds
the resident length and no prefix cache, and every generation decodes only the
delta at absolute positions from that length forward. The archived tree's
proof is the reason and the survey records it: a rewind fails silently on
families that keep recurrent layers, because the clear returns success while
the recurrent state stays, and the failure surfaces later as a position error
far from its cause. **This crate calls no scoped-clear on a resident range,**
which is the discipline stated as an absence, and the absence is what section
10 pins.

**The prefix is established at open and is permanent.** The open renders the
identity prefix through the family library, decodes it once, and records the
resident length it produced. No operation short of the flush reduces that
length, per charter section 13.3.

**Overflow refuses and sheds nothing.** A delta that would exceed the
session's capacity is refused with the overflow named and the session's own
account of itself, the harness deciding what a full context means. This crate
evicts and compacts nothing, per the charter, because either would be this
crate deciding which part of the agent's context matters.

**The turn terminator is made resident before the answer returns, on every
path.** The archived tree's own hard-won correctness note: a generation that
stops at the end-of-generation marker without decoding it leaves the
terminator absent, and the next turn's framing is then malformed at a
boundary nobody looks at. This crate decodes the terminator after the
generation loop, **on the clean path and the cancelled path alike**, which is
what makes the cancel of charter section 13.5 leave a well-framed session.

### 4.3 The turn, and the cancel

**Append and generate is one call with a token-boundary check.** The delta is
rendered by the family library, decoded at the resident end, and the
generation loop samples until the family's stop condition, the harness's
cancel, or the session's capacity. The cancel is checked between sampled
tokens, per charter section 13.5, which bounds the stop by one token's decode
rather than by a kernel's completion.

**The cancel arrives on the same socket and is read without blocking the
loop.** The decode socket is the one carrier, so the loop polls it between
tokens rather than waiting on it, which is why the check is a boundary check
rather than a signal handler: nothing asynchronous is needed to make a
token-boundary stop, and nothing asynchronous is introduced.

**A cancelled generation answers with what it produced, marked stopped, after
the terminator lands.** The partial output, its measurement for the tokens
that were produced, and the stopped marking, in that one answer, so the
harness closes the turn with the reason and the record holds what the model
said before it was stopped.

### 4.4 The flush

**The outcome is fixed and the mechanism is per family.** After a flush the
identity prefix is resident and the accumulated turns are gone, per charter
section 13.9. Where a family's state permits truncation to a position the
outcome is reached by truncating to the prefix's recorded length. Where it
cannot roll back, the outcome is reached by re-establishing the session and
decoding the prefix again, which is expensive and correct where the cheap path
is silently wrong. **The family declares which it is** and the decode path
reads the declaration rather than inferring it from a version string.

## 5. The family libraries

**One module per family, holding everything that family defines.** Per charter
section 14: the template and its rendering, the marker vocabulary, the
tokenizer conventions, the parsing of the family's own output, and the forward
orchestration quirks the shared kernels are driven by. Nothing family-specific
lives outside its module, and the kernels beneath are shared, which is the
archived tree's share-kernels-own-orchestration rule promoted to structure.

**The surface a family implements is small and named.** Render an identity
prefix from canonical messages, render a turn's delta, parse an emission into
canonical content with the family's markers recognized, declare the stop
conditions, declare whether the session's state permits truncation, and
declare the capabilities admission judges against, per charter section 14,
which are the readout tap and **the device counts the backend can shard a
model across**. That last is a set of widths rather than a maximum, because a
backend that serves one device and a pair is not thereby serving three, and a
maximum would imply it does.

**The registry is compile-time and admission consults it.** A table of the
families this binary carries, keyed by what the artifact's header declares,
with no default and no fallback: an artifact whose family this binary does not
carry is a refused admit naming the family, which is the archived tree's
own no-silent-substitution ruling carried forward from its encoder registry.

**Both directions of the template requirement are tested here, per the
charter.** Inbound, the reference test shape is the archived tree's marker
promotion: every control marker of a family tokenizes to exactly one token
under the family's tokenizer, because a marker that degrades to subword text
is structure the model reads as prose. Outbound, the parsers are the recorded
bridge from the verbatim emission to the canonical form, and a parse that
recognizes no call where the emission attempted one is its own reported fact
rather than a clean turn, which is the archived tree's distinction between a
call that could not be rendered and a call whose name could not be recovered.

**Modules become member crates when a second operation type consumes them,
and not before,** per charter section 14. A directory move is what that
promotion costs, which is the reversibility test paying out.

## 6. Measurement production

**The signals are computed where the distribution is, before the sampler.**
Entropy over the logits at each step and the surprisal of the token that was
drawn, both in bits, computed inside the generation loop because the
distribution exists there and nowhere else. **The math is log-sum-exp stable
and the reason is arithmetic rather than taste:** a naive exponential overflows
`f32` above a logit around eighty-eight, which the archived tree records as
happening routinely on production vocabularies, so the stable form is the only
correct one at the scale this runs at.

**The vectors are positionally paired with the token identifiers and absent
when not produced,** per charter section 13.6 and `weaver-trace-PRD` section
3.2. This crate produces no empty vector to mean nothing was measured: it
sends no vector, and the trace's own `skip_serializing_if` carries the absence
to the record.

**What travels with a generation.** The token identifiers in and out, the two
signal vectors, the timings the charter's row names, the model identity and
its weights hash from section 3, the template identity from the family
library, the prompt-block partition, and the residual reductions when elected.
The partition comes from the tokenizer's offsets, which is the salvaged
`tokenize_with_offsets` mechanic with its invariant intact: the last offset
equals the rendered text's length, so a partition covers the prompt exactly
and a consumer joining text to tokens has no gap to guess at.

**Nothing is retained after the answer.** Produced, reported, gone, per
charter section 3's no-state rule, which is what keeps a measurement from
becoming a second account this crate holds.

## 7. Residual readout

**The election arrives at admit and no tap runs without it.** The binding
carries the readout election, per charter section 13.7, and an admit whose
election the backend cannot honor refuses at admit rather than failing at the
first turn, which is the charter's fail-cheap-or-lie-expensive rule.

**Two backends, two taps, one shape returned.** The native path uses the
candle fork's `forward_with_intermediates`, which returns the per-layer
tensors and is the readout's working ancestor per the survey. The GGUF path
uses the ggml scheduler's eval callback the llama-cpp fork exposes, which the
archived tree pinned and never drove: the pin exists, the tap does not, and
this Spec states plainly that standing the GGUF tap up is code this program
writes rather than salvage it inherits.

**The reduction happens in place, at the tap, before anything leaves the
device.** Apex section 3 step 6 has the activations reduced in place and the
reduction returning by the same path as the generation, so no per-layer tensor
crosses the seam and the volume the seam carries is the reduction's.

**One forward per prompt when readout is elected, never a batch.** Batching
prompts changes what the attention sees, so residuals taken from a batched
forward are not the residuals of the prompt in isolation, which the archived
tree's probe records as the reason it clears the cache between prompts. The
same fact means the readout's cost is not amortizable, and that cost is the
operator's to elect per load.

**The tap's failure while elected is a fault, not an absence.** Per charter
section 13.10, because an elected observability that silently stopped
observing is a record that reads as a run without readout rather than a run
whose readout broke.

## 8. Sampling, and the dispositions

**Every knob carries a `Disposition`, elected at the worker's composition
root.** Per the composability ruling of 2026-08-02 and charter section 13.8:

```rust
pub enum Disposition<T> {
    Frozen(T),
    OperatorTunable,
}
```

A knob is `Frozen` with its value compiled into the binary, or
`OperatorTunable` and routed from the agent's configuration at load. The
machinery beneath takes a plain value and never learns which side supplied it,
which is what makes the two dispositions cost the same and a change between
them one line and a recompile.

**The knob set is temperature, top-k, top-p, the repetition penalty and its
window, and the seed.** The seed is a knob for the first time in this code's
lineage: the archived tree carried it as a hardcoded default and a determinism
test and never made it configurable, so the disposition mechanism is its first
real home, and a frozen seed beside a frozen sampling surface is what makes a
binary's declared starting field re-enterable, per apex section 8.

**Frozen values never cross the wire and that is checkable.** Only the
operator-tunable remainder travels on the token seam, per the decode
contract's conformance list, and section 10 makes it a test rather than a
promise.

**The effective values are what the record holds, whichever side set them.**
This crate reports the values it sampled with into `model.request`'s payload,
per `weaver-trace-PRD` section 3.2, so a frozen knob is as visible in the
record as a tunable one and a replay reads one list rather than joining two.

## 9. The failure vocabulary

```rust
pub enum ChannelFault {
    Truncated { bound: usize },
    Undecodable,
    Closed,
}
```

**Refusals are the floor's and faults are below the exchange layer,** the same
split `weaver-harness-Spec` section 7 and `weaver-gate-Spec` section 5 make.
Every refusal this crate issues is a `lifecycle-refusal` on the residency seam
or a `token-refusal` on the decode seam, drawn from the floor and never
twinned. A fault the worker survives is a `fault-report`, also the floor's,
carried on whichever seam the exchange belongs to, per charter section 13.10.

**The admit refusal cases are the charter's enumeration and this Spec adds
none.** Charter section 10 holds the set open with a candidate list and names
the decode workflow's measurements as what closes it, and the cases the
merged floor already carries, the artifact unresolvable or unreadable, the
device unable to admit, are what this crate maps its failures onto.

## 10. What is enforced, and by which instrument

Per apex section 11, the threat walks first.

**The first walk: this process holds a handle it should not.** The adversary
is a fork discipline that failed upstream, leaving this process holding the
trace descriptor or the coordination channel to admin, which would put a
writable handle to the agent's own record inside the agent's model server. The
mechanism is the harness's close-on-exec discipline at the fork, per
`weaver-harness-spu-contract` section 1, and this crate's own count check at
entry, per section 2. The test starts the binary with a deliberately leaked
extra descriptor and confirms it refuses to serve, watched to fail when the
count check is removed.

**The second walk: a same-uid process attaches.** The adversary is an elected
tool running as the agent uid, the attack a `ptrace` or `/proc/[pid]/fd` reach
into the process holding the weights and both channels. The mechanism is the
dumpable flag cleared at entry, a set and not a check. The test confirms the
flag is clear after entry, watched to fail when the set is removed.

**The third walk: the agent alters the artifact it runs on.** The adversary is
the agent's tool surface writing to the model artifact, which would change what
the record's weights hash describes. The mechanism is the operator's
provisioning, the artifact owned by a principal that is not the agent and
readable into it, per charter section 7, and this crate's contribution is the
hash computed at admit from the bytes it loaded rather than from a
manifest handed to it. The test alters an artifact between two admits and
confirms the recorded hash differs, watched to fail when the hash is cached
across the change.

**Enforced by the compiler.**

- The fork seam is a compile-fail pin, per the archived tree's own device and
  this Spec's inheritance of it: a doctest calling the eval-callback setter
  fails to compile against the upstream crate and compiles against the fork, so
  reverting the pin breaks the build rather than silently removing the readout
  capability. This is the pin the survey names as the one thing standing
  between the fork and a quiet loss.
- The floor's wire enums are exhaustive, so every case added later reaches this
  crate's matches loudly.
- Descriptors are owned types end to end.
- `Disposition<T>` makes a knob without an election a compile error, so a
  builder cannot leave one unstated.

**Enforced by compile-fail tests, because the property is an absence.**

- No path-taking model loader beyond the binding's own resolution: doctests
  constructing a loader from a bare `&str` and a `PathBuf` outside the
  admission path fail to compile.
- No scoped-clear over a resident range: the session type exposes no method
  that reduces `resident_len` except the flush, so a rewind is unrepresentable
  rather than forbidden, which is the append-only discipline made structural.

**Enforced by the manifest.** The internal dependencies are exactly
`weaver-types` without its `config` feature and `weaver-traits`, read against
the graph's two floor links under gate H2, the second landing with the charter
correction of section 1.1. No async runtime, no logging crate, and no HTTP
client in the resolved tree, by the build-time `cargo tree` assertion the floor
Specs share. Both fork pins resolve to their pinned revisions and
`llama-cpp-sys-2` resolves to the same source as `llama-cpp-2`, checked in the
same assertion, because the two-sys-crates failure is a resolution fact rather
than a code fact.

**Requiring a perturbation-verified test.**

- The session never rewinds: a multi-turn session's resident length is
  monotonic and each turn's prompt tokens are the delta's alone, confirmed by
  watching the count return to the full history when the append path is
  replaced by a re-prefill. This is the archived tree's own session test
  carried over, and it is the one that proved the recurrent-family failure.
- The terminator lands on both paths: a cancelled turn leaves a session that
  accepts the next delta cleanly, confirmed by watching the next turn's framing
  break when the terminator step is skipped on the cancelled path.
- The stop is bounded: a cancel lands within one token's decode, confirmed by
  watching the bound fail when the check moves outside the loop.
- Marker promotion, per family: every control marker tokenizes to exactly one
  token, confirmed by watching a marker degrade to subword text when the
  special-token path is bypassed.
- Signals are pre-sampler: the entropy of a step matches the distribution
  before sampling, confirmed by watching it collapse when the computation moves
  after the sampler.
- Frozen knobs never cross: a capture of the token seam contains no frozen
  value, confirmed by watching one appear when a knob's disposition is changed
  to tunable without changing the binary's declaration.
- Readout elected but untappable refuses at admit, confirmed by watching the
  load succeed and the first turn fail when the check moves off the admit path.

**The kernels cross verbatim and their tests cross with them.** The CUDA
kernel set and its build script are carried unchanged, per the operator's
ruling of 2026-08-02 and the carry rule's first door, and the golden fixtures
that compare each kernel against a candle reference come with them, because a
salvaged kernel with its comparison left behind is a kernel this program has
not checked. Their provenance is the survey.

## 11. Open elections

Each names what settles it, and none is this Spec's to settle alone.

- **The token seam's encoding.** The hot-path measurement the charter and both
  floor documents defer to, per `weaver-types-Spec` section 4's boundary rule.
  Taken against real decode traffic, which this crate's own first
  demonstration produces.
- **The headroom figure.** Charter section 9's staged item, a construction
  parameter until a measurement on a real artifact against a real device
  replaces it.
- **Whether the artifact's declared shape is verified against its tensor
  data.** Charter section 9's second staged item, priced by what the check
  costs on the admit path.
- **The family set this binary carries.** Which families ship is a
  deployment's question rather than this Spec's, and the registry of section 5
  is what makes the answer a list rather than a rewrite.
- **Shard widths beyond two.** The salvaged path is a two-device
  implementation and the capability declaration is what a wider one would
  change, so an N-way forward and its all-reduce are work this program does
  rather than salvage it inherits, entered when a deployment needs a model
  wider than a pair.
- **The executor.** Deferred with `weaver-traits-Spec` section 6's
  measurement, and this crate is where the latency it would buy or cost is
  measurable.
- **The satellite types.** `Disposition<T>`'s companion accessor shape,
  `ChannelFault`'s spelling against the two identical enums in the harness and
  gate Specs, the family surface's trait name, and the backend seam's. Choices
  with no cross-crate consequence, listed so what this Spec leaves to a builder
  is complete rather than implied.
