# weaver-spu - Spec

**Status:** MERGED. Cut 2026-08-02, seventh of the Spec pass and the last of the set.
Code is written against it under the gates of Working Process section 6, ratified
2026-08-04.

**Date filed:** 2026-08-02
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

**This document declares its crate's assertion records and no other record,**
per Document Format sections 3 and 4 as of the notation of 2026-08-03, which
retired the no-records sentence this paragraph replaces. The charter stays the
source of this crate's node, its parent edge, its two floor links as this act
corrects them, and its two declared seams, and a Spec that restated any of them
would give the mapper two sources for one record, per that format's section 1.
What this document sources is the claims code must conform to, declared at the
clauses that argue them rather than gathered in one place, per that format's
section 6, and `asserts` runs from the crate rather than from this document,
which is why the document needs no node of its own.

**A claim this Spec cites and another Spec argues carries no record here,** and
there are nine of them, named again in section 10 with the crate that declares
each. Five are `weaver-types-Spec`'s. The floor's exhaustive wire enums are that
Spec's section 4.2. The `SOCK_SEQPACKET` election, the 64 kibibyte envelope
bound that election carries, and the lifecycle channel's JSON encoding are its
section 4.3, the bound being a record that Spec declares beside the election
rather than inside it, which makes the first two a pair of records and not one.
`ModelBinding`'s ordered device set is its section 2. Three are
`weaver-harness-Spec`'s: the descriptor numbering with the order of the two ends
of section 2.2, the OS-surface election of section 2.4, and the fork discipline
that leaves this process two inherited descriptors, of sections 2.2 and 8, which
`weaver-harness-spu-contract` section 1 requires of the harness and this crate
relies on rather than re-tests. The ninth is `weaver-trace-Spec`'s, the
absent-rather-than-zero serialization of its sections 3 and 10, which this
crate's own refusal to produce an empty vector meets from the other side.

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

```graph
node: spu-one-binary
kind: assertion
tag: manifest

edge: asserts
from: weaver-spu
to: spu-one-binary

edge: grounds
from: spu-one-binary
to: axiom-floor-is-vocabulary-behavior-is-socket
```

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
**The absence is review's, and the ground is stated rather than assumed.** Its
feature half is mechanical already, section 1.1's gates being a manifest fact,
and what remains is an absence with no name to pin: a doctest fixes a named
shape and there is no shape here, the claim being about what was not written for
a submodule that does not exist. This is one of the few places in this Spec
where review is the instrument available rather than the instrument unbought.

```graph
node: spu-nothing-laid-in-for-later-operations
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-nothing-laid-in-for-later-operations
```

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
`weaver-traits` into this crate accordingly. The link follows the draw, per apex section
5.3, which is what makes the party list checkable against the dependency graph. **This
record carries both floor edges.** That the crate links the floor at all is 5.1's, the
floor being shared vocabulary a socket cannot carry, and that `weaver-traits` in
particular is linked is 5.3's, the draw determining the set. The feature the
`weaver-types` link is taken without grounds in nothing, being a build election that
would read the same under any invariant, per 5.1 as corrected on 2026-08-04.
`provider-trait` is still not implemented here and the charter's reasoning for that
stands untouched: the abstraction lives at the harness's composition root, on the far
side of this seam's transport.

```graph
node: spu-two-floor-links-types-without-config
kind: assertion
tag: manifest

edge: asserts
from: weaver-spu
to: spu-two-floor-links-types-without-config

edge: grounds
from: spu-two-floor-links-types-without-config
to: axiom-floor-is-vocabulary-behavior-is-socket

edge: grounds
from: spu-two-floor-links-types-without-config
to: axiom-contract-is-a-complete-interface
```

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

```graph
node: spu-fork-pins-resolve-as-pinned
kind: assertion
tag: manifest

edge: asserts
from: weaver-spu
to: spu-fork-pins-resolve-as-pinned

node: spu-cudarc-caret-pinned
kind: assertion
tag: manifest

edge: asserts
from: weaver-spu
to: spu-cudarc-caret-pinned
```

**External, the rest.** `serde_json` for the lifecycle channel's envelopes,
whose JSON election is `weaver-types-Spec` section 4.3's and carries no record
here. `nix` for the OS surface, on the grounds `weaver-harness-Spec` section 2.4
argued and this crate inherits: the calls are `recvmsg`, `fcntl`, the dumpable
`prctl`, and the descriptor adoption of section 2. `safetensors` for the native
backend's weights, `half` and `bytemuck` for host-side tensor math,
`tokenizers` for the family tokenizers, `blake3` and `walkdir` for the weights
hash of section 3.

**What does not cross, from a working tree that had it.** No HTTP client and no
model downloader: an artifact this crate cannot reach is a refused admit, per
charter section 4.1, and fetching one would make this crate a provisioner of
the operator's own artifact. No logging crate, per the corpus's one-account
rule. No async runtime in this pass, the service being one directive at a time
against one resident session, and the executor question belonging to the
measurement `weaver-traits-Spec` section 6 names. No multi-model catalog and no
apparatus for holding more than one agent's model in one process, both dissolved
by one SPU per agent, per the survey.

```graph
node: spu-no-runtime-no-logging-no-http
kind: assertion
tag: manifest

edge: asserts
from: weaver-spu
to: spu-no-runtime-no-logging-no-http
```

**Feature gates are two and each names what it buys.** `cuda` compiles the
kernels of section 10 and the native backend's device path. `gguf` compiles the
llama.cpp backend. A build with neither compiles the channels, the residency
bookkeeping, and the family libraries, which is what keeps the family surface
testable on a machine with no device.

```graph
node: spu-two-feature-gates
kind: assertion
tag: manifest

edge: asserts
from: weaver-spu
to: spu-two-feature-gates
```

## 2. The two channel ends, and the process facts

**Both ends arrive at descriptors 3 and 4, inherited rather than re-decided.**
`weaver-harness-Spec` section 2.2 elects the numbering with its order and owes
it to this document, and this document takes it: **3 is the lifecycle channel
and 4 is the decode socket**, lifecycle first because it is the channel every
organ has and the one an organ with a single end already places there, so the
gate's placement is unchanged and this crate's first end sits where the gate's
does. The correction that gave that paragraph two numbers and an order rather
than one number landed with the decoder cut, in the act the entry of 2026-08-02
records, so what stands here is the order read from the receiving side and the
election carries no record in this document.

**Entry adopts both, then performs its two sets, before the first read.** The
process wraps descriptors 3 and 4 as owned handles, sets close-on-exec on
both, and clears its dumpable flag, per charter section 7. Both are sets and
never checks, per the rule `weaver-organ-channel` section 2 states: a step
that finds a flag wrong and reports leaves the descriptor inheritable and the
process attachable, which is the condition the set exists to prevent. The
close-on-exec set matters here even though this crate forks nothing, because
`execve` clears the flag and the requirement is stated against the last exec.
**The wrap is not a formality.** Descriptors are owned types end to end in this
crate, so no raw number outlives the thing it names and no close happens twice.
**The close-on-exec set is review's by election and not by impossibility,** an
`fcntl` reading the flag as cheaply as the `prctl` that reads the one set beside
it: the second walk of section 10 buys the dumpable flag and this suite does not
buy its neighbour, which is the same split `weaver-gate-Spec` section 2 takes on
the same pair.

```graph
node: spu-descriptors-owned-types
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-spu
to: spu-descriptors-owned-types

node: spu-dumpable-flag-cleared
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-dumpable-flag-cleared

node: spu-channel-ends-close-on-exec
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-channel-ends-close-on-exec
```

**Entry verifies that it holds exactly two descriptors beyond the standard
streams, and refuses to serve if it holds more.** Charter section 7 makes a
build in which this crate holds a trace descriptor broken whether or not it
writes through one, and the harness's fork discipline is what keeps that true,
argued at `weaver-harness-Spec` sections 2.2 and 8 and relied on rather than
re-tested here, but a property this crate depends on and cannot see is a
property worth checking at the one moment it is cheap. The check is a count
rather than an identification, since this crate cannot know what a stray
descriptor refers to, and a count above two means the discipline upstream failed
and this process is not the one to continue past it.

```graph
node: spu-descriptor-count-check
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-descriptor-count-check
```

**The lifecycle channel carries the organ envelope and the decode socket does
not.** Per charter section 13.2 the decode socket is not an organ channel, so
the envelope crosses only on the first end, and the token trio crosses the
second in whatever encoding the measurement of section 11 elects. Both ends
carry the receive obligation the `SOCK_SEQPACKET` election attaches: a buffer
sized to the envelope bound on the lifecycle channel, a read returning with
`MSG_TRUNC` set treated as a channel fault and never a message, and the same
bound asserted on this crate's own writes. The election and its bound are
`weaver-types-Spec` section 4.3's, declared there as two records rather than
one, the bound sitting beside the election rather than inside it, and this crate
carries the obligation without redeclaring either. **The two claims stated here
carry two instruments, and the reason they differ is the reason one was unbought.**
The envelope's confinement to the first end becomes watchable the day the decode
socket's encoding is settled, which section 11 holds open, so no capture test has
a settled shape to assert against yet, and it stays review's by non-purchase on
that ground. The truncation fault was reachable all along and three sibling Specs
buy the instrument for it, `weaver-harness-Spec` section 8, `weaver-admin-Spec`
section 10, and `weaver-gate-Spec` section 6 each naming the same watch. Section
10 names it here too, in the act that closed issue 37, which leaves no receiving
crate in the corpus carrying this obligation on prose alone. **The two also ground
in two different invariants, along the same line their instruments took.** The
envelope's confinement is apex section 5.4 read at this crate's two ends: the
lifecycle channel is the duplex channel with the harness that makes this crate an
organ, and the decode socket is a second end its own contract governs, so an
envelope crossing it would put the organ channel's carriage on a channel that is
not one and leave the organ line unreadable from a capture. The truncation fault
grounds in apex section 5.1 instead, the obligation existing at all only because a
boundary-preserving socket type was elected under that invariant and a short read is
what defeats it.

```graph
node: spu-envelope-on-lifecycle-only
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-envelope-on-lifecycle-only

edge: grounds
from: spu-envelope-on-lifecycle-only
to: axiom-organ-and-submodule

node: spu-truncation-is-a-fault
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-truncation-is-a-fault

edge: grounds
from: spu-truncation-is-a-fault
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**The service is serial per channel and the two channels are one loop.** One
lifecycle directive at a time, one decode exchange at a time, per both
contracts' ordering, and the decode socket's traffic is what the process
spends its time on. Nothing here is concurrent in this pass, and the shape
that would change it is the executor election deferred in section 1.1. **This
is review's by non-purchase,** two asks pressed onto one channel being as
reachable a test here as the one-answer-per-request test `weaver-admin-Spec`
section 10 buys, and section 10 of this document buying none.

```graph
node: spu-service-serial-one-loop
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-service-serial-one-loop
```

## 3. Residency

The two exchanges of `weaver-harness-spu-contract`, in the charter's order.

**Seven of this section's nine records are review's, and the ground is
non-purchase rather than reach.** Section 10's perturbation list was written
before the device-assignment ruling of 2026-08-03 reshaped this section and was
revisited in the act that closed issue 37, which bought what needs no hardware
and no seam work: the cheap refusals below, and, in section 5, the family lookup
against an artifact header and the width refusal against a declared set. What is
left standing on prose divides in two, and the division is worth stating because
the two halves are unbought for different reasons. The room condition, the peer
reachability condition, and the release's free-before-answer ordering each read a
driver, and they reach it through a seam a suite could double and this Spec does
not introduce, so buying them is a larger act than the three this one takes. The
remainder needs no device at all and is simply unbought: the devices coming from
the binding, the headroom's placement, the hash's failure sentinel, and the
once-only admit. Stating that here once is what keeps seven review tags from
reading as seven findings that no instrument exists.

**Admit runs the charter's five steps, and the first three are free.**
Resolve the binding to an artifact, read what the artifact declares about
itself without loading it, judge the assigned devices, take them in shard order
and load each shard, confirm.
The header read is the salvaged mechanic the survey names: parsing an
artifact's header and metadata answers what family this is and what its
dimensions are without touching tensor data or the device, which converts the
common shape of a bad binding, an artifact present and wrong, into a refusal
costing no device work. **What section 10 buys is the ordering rather than the
parsing,** per the charter's rule that every step before the fourth is refusable
at no cost: a fixture that would fail the judgment as well as the resolution is
what makes the refusal's identity report which step ran first, and a test that
asserted only that the header parses would pass with the whole ordering inverted.

```graph
node: spu-header-read-touches-no-device
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-header-read-touches-no-device
```

**The devices are the binding's and this crate selects none.** The assignment
arrives inside the model binding, per `weaver-types-Spec` section 2, ordered,
with the order the shard order. There is no device survey, no ranking, and no
fallback in this crate: the archived tree's `auto_select_gpu` does not cross,
per charter section 3 and the salvage survey's seventh yielding. What this
crate does with the set is judge it, in the order section 3's steps state. The
binding's own shape, the artifact with an ordered device set and an empty set a
parse error, is `weaver-types-Spec` section 2's and carries no record here.

```graph
node: spu-devices-from-the-binding
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-devices-from-the-binding
```

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

**The three are judged cheapest first, and the ordering is this document's to
elect.** Charter section 4.1 step 3 enumerates the three conditions and sequences
none of them, so the order is a representation question, and it takes the
charter's own before-the-fourth-step rule one level inward: the width condition
is a comparison between the binding's count and the family's declaration and
reads nothing, while the room and reach conditions each cost a driver query, so
the width is judged first and a set failing more than one condition is refused on
the cheapest. That ordering is what puts the width refusal inside a test on a
machine with no device, which is why it is elected here rather than left to a
builder. **This record stays review's and the ground is the seam rather than the
purchase.** Two of its three conditions read a driver, a suite could reach them
through a double, and this Spec introduces no such seam, so the judgment taken
whole has no instrument here yet. The width condition alone is watched, at the
section 5 clause that declares the set, and dividing it out here would mint a
second record for a property that clause already carries with the same test.

```graph
node: spu-admission-judges-room-reach-and-width
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-admission-judges-room-reach-and-width
```

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

```graph
node: spu-device-authority-is-the-driver
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-device-authority-is-the-driver
```

**The headroom term stays a construction parameter until a measurement
replaces it.** Charter section 9 stages the figure and names the entry
condition, a measurement on a real artifact against a real device, so the
admission inequality takes the headroom from the worker's composition root the
way `weaver-trace-Spec` section 6 takes its queue depth: a deployment fact
rather than an operator election, and a number a builder can supply before the
measurement exists.

```graph
node: spu-headroom-is-a-construction-parameter
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-headroom-is-a-construction-parameter
```

**The weights hash is computed at admit and travels with every measurement.**
The salvaged mechanic is BLAKE3 over a canonical manifest, a single file or a
walked directory, with a sidecar cache and an **empty-string sentinel on every
failure path**, which is the property worth carrying verbatim: a hash that
cannot be computed reports that it could not rather than reporting a wrong
value, and apex section 8 rests replay on the identity being right. **The third
walk's test reaches the hash and not the sentinel,** an alteration between two
admits showing a changed hash and showing nothing about a failure path. The
sentinel is a claim of its own rather than the periphery of that one, and it is
review's by non-purchase, an unreadable artifact being as reachable a fixture as
an altered one.

```graph
node: spu-weights-hash-at-admit
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-weights-hash-at-admit

node: spu-hash-failure-sentinel
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-hash-failure-sentinel
```

**Release frees the device before it answers, and the ordering is the
contract's.** Stop serving, free the weights and the working allocations and
the cache together because they are one residency, then confirm. A
confirmation is a fact about the device rather than a statement of intent, per
charter section 4.2, and the archived tree's inverse ordering is what its own
record names as producing an overcommit.

```graph
node: spu-release-frees-before-answering
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-release-frees-before-answering
```

**Nothing is idempotent and nothing retries.** This crate begins empty, admits
once, and dies, so a second admit has no prior residency to match, per charter
section 4.1, and the archived tree's idempotence rule loses its premise with
the daemon it was written for.

```graph
node: spu-no-idempotence-no-retry
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-no-idempotence-no-retry
```

## 4. The decode submodule

### 4.1 The backend seam

**One trait, two backends, peers rather than a legacy and a target.** The
archived tree's own ruling of 2026-06-11 made the GGUF and native paths
first-class peers and the survey carries the reasoning forward: GGUF owns
quantized artifacts on consumer devices, and the native path owns what a
tensor-parallel forward and a fine-tunable artifact need, since a GGUF cannot
be fine-tuned and a program that intends training as a continuation cannot let
that path decay. The seam is one trait over open, append-and-generate, cancel,
flush, and close, with the family library above it and the device beneath. The
peer status is review's by non-purchase: one trait with two implementations is a
compile property a bound would pin, and section 10 buys no such pin.

```graph
node: spu-backends-are-peers
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-backends-are-peers
```

**Which backend serves is a property of the artifact, decided at admit.** The
header read of section 3 already answers it, so nothing elects a backend
separately and no configuration field names one. The config's field list is
`weaver-types-Spec` section 2's and what this crate asserts is the derivation,
reachable by admitting two artifacts of different kinds under one configuration
and unbought here.

```graph
node: spu-backend-from-artifact
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-backend-from-artifact
```

### 4.2 The session

**Append-only, `resident_len`, and nothing ever rewinds.** The session holds
the resident length and no prefix cache, and every generation decodes only the
delta at absolute positions from that length forward. The archived tree's
proof is the reason and the survey records it: a rewind fails silently on
families that keep recurrent layers, because the clear returns success while
the recurrent state stays, and the failure surfaces later as a position error
far from its cause. **This crate calls no scoped-clear on a resident range,**
which is the discipline stated as an absence, and the absence is what section
10 pins. **The behaviour and the surface are two records with two instruments,**
the monotonic resident length being section 10's perturbation test and the
missing method being its compile-fail pin, and neither claims the other's half:
a pin says a rewind cannot be written and says nothing about a turn that
re-prefills through the append path instead.

```graph
node: spu-session-never-rewinds
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-session-never-rewinds

node: spu-no-scoped-clear-surface
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-spu
to: spu-no-scoped-clear-surface
```

**The prefix is established at open and is permanent.** The open renders the
identity prefix through the family library, decodes it once, and records the
resident length it produced. No operation short of the flush reduces that
length, per charter section 13.3.

**Overflow refuses and sheds nothing.** A delta that would exceed the
session's capacity is refused with the overflow named and the session's own
account of itself, the harness deciding what a full context means. This crate
evicts and compacts nothing, per the charter, because either would be this
crate deciding which part of the agent's context matters. Review's by
non-purchase, an overflowing delta being a fixture the family libraries can
produce with no device present. **It grounds in apex section 5.5.** What a full
context means is a question about the turn, which is the harness's domain, and
an organ that evicted to make room would be reasoning about a domain that is
not its own, which that invariant names as what a hub carrying traffic alone
invites. The refusal carrying the session's own account is what lets the loop
decide with the fact rather than without it, so the claim is this crate handing
a decision back rather than withholding one.

```graph
node: spu-overflow-refuses-sheds-nothing
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-overflow-refuses-sheds-nothing

edge: grounds
from: spu-overflow-refuses-sheds-nothing
to: axiom-harness-integrates-by-the-loop
```

**The turn terminator is made resident before the answer returns, on every
path.** The archived tree's own hard-won correctness note: a generation that
stops at the end-of-generation marker without decoding it leaves the
terminator absent, and the next turn's framing is then malformed at a
boundary nobody looks at. This crate decodes the terminator after the
generation loop, **on the clean path and the cancelled path alike**, which is
what makes the cancel of charter section 13.5 leave a well-framed session.

```graph
node: spu-terminator-on-every-path
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-terminator-on-every-path
```

### 4.3 The turn, and the cancel

**Append and generate is one call with a token-boundary check.** The delta is
rendered by the family library, decoded at the resident end, and the
generation loop samples until the family's stop condition, the harness's
cancel, or the session's capacity. The cancel is checked between sampled
tokens, per charter section 13.5, which bounds the stop by one token's decode
rather than by a kernel's completion.

```graph
node: spu-cancel-bounded-by-one-token
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-cancel-bounded-by-one-token
```

**The cancel arrives on the same socket and is read without blocking the
loop.** The decode socket is the one carrier, so the loop polls it between
tokens rather than waiting on it, which is why the check is a boundary check
rather than a signal handler: nothing asynchronous is needed to make a
token-boundary stop, and nothing asynchronous is introduced. The bound this
buys is section 10's test and the carrier election beneath it is review's by
non-purchase, the two being separate claims about one loop. **The carrier election
grounds in apex section 5.1,** which is why a signal is not the alternative it looks
like: a cancel is the harness asking this process to stop, so it is a seam, and that
invariant admits no seam across a process line that is not a socket. Remove it and a
signal handler becomes a shape a builder could reach for on latency grounds alone.

```graph
node: spu-cancel-polled-not-signalled
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-cancel-polled-not-signalled

edge: grounds
from: spu-cancel-polled-not-signalled
to: axiom-floor-is-vocabulary-behavior-is-socket
```

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
reads the declaration rather than inferring it from a version string. Review's
by non-purchase, a family declaring each way being two fixtures and one
assertion about the resident length after the flush.

```graph
node: spu-flush-mechanism-from-declaration
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-flush-mechanism-from-declaration
```

## 5. The family libraries

**One module per family, holding everything that family defines.** Per charter
section 14: the template and its rendering, the marker vocabulary, the
tokenizer conventions, the parsing of the family's own output, and the forward
orchestration quirks the shared kernels are driven by. Nothing family-specific
lives outside its module, and the kernels beneath are shared, which is the
archived tree's share-kernels-own-orchestration rule promoted to structure.
Review's by non-purchase, the placement of a family's code being as readable to
a module-boundary test as to a reader and neither being bought here.

```graph
node: spu-share-kernels-own-orchestration
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-share-kernels-own-orchestration
```

**The surface a family implements is small and named.** Render an identity
prefix from canonical messages, render a turn's delta, parse an emission into
canonical content with the family's markers recognized, declare the stop
conditions, declare whether the session's state permits truncation, and
declare the capabilities admission judges against, per charter section 14,
which are the readout tap and **the device counts the backend can shard a
model across**. That last is a set of widths rather than a maximum, because a
backend that serves one device and a pair is not thereby serving three, and a
maximum would imply it does. The surface's membership is the charter's
enumeration and takes no record. The declaration's shape does, being this
document's own and what section 3 judges against, and section 10 buys the test:
a declared set with a pair in it and a binding naming three devices is arithmetic
that runs with no device present. **The fixture the test uses declares a
non-contiguous set on purpose,** because the set reading and the maximum reading
answer alike on every contiguous declaration, so a fixture declaring one and two
would leave the perturbation that matters unwatchable while the test went on
passing. **The declaration's field is a set type and that half is pinned, per
the operator's election of 2026-08-04:** a doctest reads a declaration carrying
a non-contiguous set literal, so a maximum can no longer be declared, only read
wrongly, and the perturbation keeps the judgment honest where the type cannot
reach. The two are two records for that reason, per the division rule of
Document Format section 3.

```graph
node: spu-shard-widths-are-a-set
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-shard-widths-are-a-set

node: spu-widths-set-pinned-by-doctest
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-spu
to: spu-widths-set-pinned-by-doctest
```

**The registry is compile-time and admission consults it.** A table of the
families this binary carries, keyed by what the artifact's header declares,
with no default and no fallback: an artifact whose family this binary does not
carry is a refused admit naming the family, which is the archived tree's
own no-silent-substitution ruling carried forward from its encoder registry.
Section 10 buys the test, an artifact header naming a family the binary does
not carry being a fixture and the refusal arriving before any device call, and
what the test reads is the family the refusal names rather than the load's
outcome, so a refusal arriving on some other ground does not satisfy it.

```graph
node: spu-registry-no-silent-substitution
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-registry-no-silent-substitution
```

**Both directions of the template requirement bind here, per the charter, and
both are bought.** Inbound, the reference test shape is the archived tree's
marker promotion: every control marker of a family tokenizes to exactly one
token under the family's tokenizer, because a marker that degrades to subword
text is structure the model reads as prose. Outbound, the parsers are
the recorded bridge from the verbatim emission to the canonical form, and a
parse that recognizes no call where the emission attempted one is its own
reported fact rather than a clean turn, which is the archived tree's
distinction between a call that could not be rendered and a call whose name
could not be recovered. **The two directions are two records and each carries its
own bullet in section 10,** the two being separate behaviours of one module
rather than the halves of one claim: a marker that tokenizes cleanly says nothing
about what the parser does with an emission it cannot recover. The outbound
bullet lands in the act that closed issue 37, an emission attempting a call the
parser cannot recover being a fixture rather than a device. An earlier wording of
this clause said both directions are tested here while the suite bought only one,
and the assertion pass demoted the outbound half to review rather than naming a
test, which is the position this act closes from the other end.

```graph
node: spu-marker-promotion
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-marker-promotion

node: spu-parse-reports-unrecovered-call
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-parse-reports-unrecovered-call
```

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
correct one at the scale this runs at. **The placement and the arithmetic are
two claims and two records,** neither the periphery of the other, and section 10
buys the first alone. The second is review's by non-purchase rather than by
reach: a logit vector carrying a value above that threshold is a literal, and
what it should produce is a finite number.

```graph
node: spu-signals-pre-sampler
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-signals-pre-sampler

node: spu-log-sum-exp-stable
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-log-sum-exp-stable
```

**The vectors are positionally paired with the token identifiers and absent
when not produced,** per charter section 13.6 and `weaver-trace-PRD` section
3.2. This crate produces no empty vector to mean nothing was measured: it
sends no vector, and the trace's own `skip_serializing_if` carries the absence
to the record. The serialization half is `weaver-trace-Spec`'s, argued and
tested at its sections 3 and 10, and this record is the production half, which
is a different party's obligation and not a second copy of one claim. **The
absence takes a type instead of a fixture, per the operator's election of
2026-08-04:** the vectors travel as an option of a non-empty vector, head and
tail, so an empty vector meaning nothing was measured is unrepresentable rather
than unproduced, and the earlier non-purchase ruling priced a fixture the shape
now retires. The pairing itself stays review's, positional correspondence being
a semantics no field type carries. The two are two records for that reason, per
the division rule of Document Format section 3, and the pin's doctest reads the
field's type at section 10's bullet.

```graph
node: spu-absent-not-empty-vector
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-absent-not-empty-vector

node: spu-absent-shape-pinned-by-doctest
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-spu
to: spu-absent-shape-pinned-by-doctest
```

**What travels with a generation.** The token identifiers in and out, the two
signal vectors, the timings the charter's row names, the model identity and
its weights hash from section 3, the template identity from the family
library, the prompt-block partition, and the residual reductions when elected.
The partition comes from the tokenizer's offsets, which is the salvaged
`tokenize_with_offsets` mechanic with its invariant intact: the last offset
equals the rendered text's length, so a partition covers the prompt exactly
and a consumer joining text to tokens has no gap to guess at. The row's
membership is the charter's and takes no record. The offsets invariant does,
and it is review's by non-purchase, an assertion over one rendered prompt
needing no device.

```graph
node: spu-prompt-partition-covers-exactly
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-prompt-partition-covers-exactly
```

**Nothing is retained after the answer.** Produced, reported, gone, per
charter section 3's no-state rule, which is what keeps a measurement from
becoming a second account this crate holds. This is the crate's face of the
program's statelessness and it is review's by non-purchase, which is the least
comfortable of this Spec's review tags: two identical generations answering
identically, with nothing of the first reachable from the second, is a
behaviour a suite can watch, and section 10 buys no watch for it.

```graph
node: spu-nothing-retained-after-the-answer
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-nothing-retained-after-the-answer
```

## 7. Residual readout

**The election arrives at admit and no tap runs without it.** The binding
carries the readout election, per charter section 13.7, and an admit whose
election the backend cannot honor refuses at admit rather than failing at the
first turn, which is the charter's fail-cheap-or-lie-expensive rule.

```graph
node: spu-readout-refused-at-admit
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-readout-refused-at-admit
```

**Two backends, two taps, one shape returned.** The native path uses the
candle fork's `forward_with_intermediates`, which returns the per-layer
tensors and is the readout's working ancestor per the survey. The GGUF path
uses the ggml scheduler's eval callback the llama-cpp fork exposes, which the
archived tree pinned and never drove: the pin exists, the tap does not, and
this Spec states plainly that standing the GGUF tap up is code this program
writes rather than salvage it inherits. The one-shape claim is review's by
non-purchase, the pin section 10 buys on this seam being the fork's callback
and not the shape either tap returns.

```graph
node: spu-two-taps-one-shape
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-two-taps-one-shape
```

**The reduction happens in place, at the tap, before anything leaves the
device.** Apex section 3 step 6 has the activations reduced in place and the
reduction returning by the same path as the generation, so no per-layer tensor
crosses the seam and the volume the seam carries is the reduction's. Review's
by non-purchase, a capture of the decode seam under an elected readout being
the watch and section 11's open encoding being what it waits on.

```graph
node: spu-reduction-in-place-at-the-tap
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-reduction-in-place-at-the-tap
```

**One forward per prompt when readout is elected, never a batch.** Batching
prompts changes what the attention sees, so residuals taken from a batched
forward are not the residuals of the prompt in isolation, which the archived
tree's probe records as the reason it clears the cache between prompts. The
same fact means the readout's cost is not amortizable, and that cost is the
operator's to elect per load. Review's by non-purchase, and this one wants a
device: the property is a count of forwards under a real backend.

```graph
node: spu-one-forward-per-prompt
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-one-forward-per-prompt
```

**The tap's failure while elected is a fault, not an absence.** Per charter
section 13.10, because an elected observability that silently stopped
observing is a record that reads as a run without readout rather than a run
whose readout broke. Review's by non-purchase, a tap made to fail under an
election being reachable wherever the tap itself is.

```graph
node: spu-tap-failure-is-a-fault
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-tap-failure-is-a-fault
```

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
them one line and a recompile. A knob left without an election does not
compile, the type carrying no third case and no default, which is the claim
section 10 sorts under the compiler.

```graph
node: spu-disposition-compels-election
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-spu
to: spu-disposition-compels-election
```

**The knob set is temperature, top-k, top-p, the repetition penalty and its
window, and the seed.** The seed is a knob for the first time in this code's
lineage: the archived tree carried it as a hardcoded default and a determinism
test and never made it configurable, so the disposition mechanism is its first
real home, and a frozen seed beside a frozen sampling surface is what makes a
binary's declared starting field re-enterable, per apex section 8. Compile-pin
on the operator's election of 2026-08-04, superseding this clause's earlier
non-purchase ruling: the membership is a struct literal a doctest reads, every
member riding its `Disposition`, so a knob added or dropped stops the build,
and the bullet section 10 was missing is the bullet that now reads it. The
ruling's premise was the missing bullet, so the ruling falls with it.

```graph
node: spu-knob-set-includes-the-seed
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-spu
to: spu-knob-set-includes-the-seed
```

**Frozen values never cross the wire and that is checkable.** Only the
operator-tunable remainder travels on the token seam, per the decode
contract's conformance list, and section 10 makes it a test rather than a
promise.

```graph
node: spu-frozen-values-never-cross
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-frozen-values-never-cross
```

**The effective values are what the record holds, whichever side set them.**
This crate reports the values it sampled with into `model.request`'s payload,
per `weaver-trace-PRD` section 3.2, so a frozen knob is as visible in the
record as a tunable one and a replay reads one list rather than joining two.
Review's by non-purchase, and it is the near twin of the test above read the
other way: that one asserts what the seam does not carry and this one asserts
what the record does.

```graph
node: spu-effective-values-recorded
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-effective-values-recorded
```

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
**The split itself is `weaver-harness-Spec` section 7's and the record here is
this crate's side of it.** That Spec argues the principle and `weaver-gate-Spec`
section 5 adopts it with a record of its own, so one node with three edges would
say the wrong thing: what each crate asserts is which floor type its own seams
carry, and this crate is the only one of the three holding two seams and two
refusal types. The principle is cited and the mapping is recorded. Review's by
non-purchase, a refusal raised on each seam being reachable without a device. **It
grounds in apex section 5.3.** A contract names the errors each party can return, so
a crate that twinned a local error type beside the floor's would answer with a case
its own contract does not describe, and the completeness that invariant claims would
hold for the document and not for the code.

```graph
node: spu-fault-below-the-exchange-layer
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-fault-below-the-exchange-layer

edge: grounds
from: spu-fault-below-the-exchange-layer
to: axiom-contract-is-a-complete-interface
```

**A directive out of order for its seam's state answers `OutOfOrder` and is not
queued, on both seams.** `weaver-types-Spec` section 5 owes the refusal to each
organ and enforces it nowhere, and section 3 of `weaver-harness-spu-contract` and
section 3 of `weaver-harness-spu-decode-contract` each bind it here against a
state of their own, so this document states it twice rather than once in the
abstract. **The order is judged against the channel's recorded position before
the directive reaches residency or the session,** which is what makes not-queued
mean anything at all, a refusal that had already run the work being a refusal
about nothing, and it is what puts every position inside a test with no artifact
resolved and no device taken.

**The residency seam has three positions and the last is terminal,** per that
contract's section 3: before-admit, admitted, released. Admit is first and
happens exactly once, so a second admit answers `OutOfOrder` whatever the first
answered, this crate admitting once and dying per section 3 rather than matching
a prior residency against a later request. A release with no completed admit
before it is refused and not queued, there being no residency for it to end, and
a directive of any kind arriving after a release answers the same.

**The decode seam holds the richer state and the decode contract rules all of
it.** Open is first, happens once, and is valid only after the residency it
serves is confirmed, so an open before residency answers `OutOfOrder`. One
generation is in flight at a time, so a second append-and-generate while one is
outstanding answers the same, one turn behind one intent. Flush is valid only
between turns, so a flush arriving mid-generation answers the same, the cancel of
section 4.3 being what that case has instead. **Cancel is the one directive whose
window is the session rather than the generation,** and a cancel at rest answers
at rest rather than refusing, which is the contract's own reading and not a
fourth position this document adds.

**Each seam carries its own refusal type and neither carries a twin of the
other's,** per the split above: the residency seam answers a `lifecycle-refusal`
and the decode seam a `token-refusal`, both drawn from the floor with the
`OutOfOrder` case the floor already holds. **The two are two records,** because
the two seams hold two different ordered states watched by two different
fixtures, and one record would report a single instrument for behaviours that
fail independently. Section 10 buys both, which discharges this crate's side of
the owing on both seams, the way `weaver-gate-Spec` section 6 discharged the
gate's on its one. **Both ground in apex section 5.3,** whose completeness reaches
the ordering guarantees a party relies on and provides: an interface that names its
vocabulary and its errors and leaves what happens outside its ordered state to the
implementation has a hole in it, and a queue is what fills that hole silently.
**Both also ground in apex section 5.5, and that is a second reason rather than the
first one restated.** A directive out of order arrived in an order this crate did
not choose, the harness driving both seams and no organ ordering its own traffic, so
the mis-ordering is a timing failure between domains and that invariant makes the
loop answerable for it. Queueing would be this crate reconciling that failure inside
its own domain, which is the reconciliation 5.5 assigns to the loop by construction,
and the refusal is what returns the failure to the party answerable for it. The
decode seam carries the sharper case, an open being valid only after the residency
it serves is confirmed, which is one contract's ordering read against the other's.

```graph
node: spu-out-of-order-refused-on-residency
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-out-of-order-refused-on-residency

edge: grounds
from: spu-out-of-order-refused-on-residency
to: axiom-contract-is-a-complete-interface

edge: grounds
from: spu-out-of-order-refused-on-residency
to: axiom-harness-integrates-by-the-loop

node: spu-out-of-order-refused-on-decode
kind: assertion
tag: perturbation

edge: asserts
from: weaver-spu
to: spu-out-of-order-refused-on-decode

edge: grounds
from: spu-out-of-order-refused-on-decode
to: axiom-contract-is-a-complete-interface

edge: grounds
from: spu-out-of-order-refused-on-decode
to: axiom-harness-integrates-by-the-loop
```

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
  between the fork and a quiet loss. **Its record is tagged for the mechanism
  this bullet names rather than for the heading it sits under,** which is a
  doctest that must fail to compile, and it sits at the end of this section
  because the argument for it is nowhere else.
- The floor's wire enums are exhaustive, so every case added later reaches this
  crate's matches loudly. `weaver-types-Spec` section 4.2 argues that property
  and declares its record, and this crate consumes it.
- Descriptors are owned types end to end.
- `Disposition<T>` makes a knob without an election a compile error, so a
  builder cannot leave one unstated.
- The knob struct is a literal a doctest reads, every member named and the seed
  among them, so section 8's set is membership the build checks rather than a
  list a builder consults.
- The signal vectors' field is an option of a non-empty vector, its doctest
  reading the field's type, so the empty vector section 6 forbids is
  unrepresentable and the pairing is what remains for review there.
- The width declaration's field is a set type, its doctest reading a
  non-contiguous set literal, so a maximum cannot be declared, and the admission
  judgment against it is what remains for the perturbation below to watch.

**Enforced by compile-fail tests, because the property is an absence.**

- No path-taking model loader beyond the binding's own resolution: doctests
  constructing a loader from a bare `&str` and a `PathBuf` outside the
  admission path fail to compile. Two named shapes rather than a claim about
  all possible shapes, **with the general prohibition staying review's,** per
  the split the floor Specs make. **The split is two assertions rather than
  one,** the pinned shapes and the prohibition itself, both sitting at the end
  of this section because this bullet is the only clause that argues either: a
  single record tagged for the mechanical half would claim the doctests for the
  whole, which is the overclaim this corpus refuses in prose and has no reason
  to admit in a graph.
- No scoped-clear over a resident range: the session type exposes no method
  that reduces `resident_len` except the flush, so a rewind is unrepresentable
  rather than forbidden, which is the append-only discipline made structural.
  Its record sits at the section 4.2 clause that argues the absence, beside the
  behavioural record the perturbation bullet below carries, the two being two
  claims about one property rather than one claim twice.

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

- Truncation is a fault: an envelope over the 64 kibibyte bound on the lifecycle
  channel produces `Truncated` and no directive, confirmed by watching a silently
  shortened directive decode when the `MSG_TRUNC` check is removed. This is the
  watch `weaver-harness-Spec` section 8, `weaver-admin-Spec` section 10, and
  `weaver-gate-Spec` section 6 each buy, and until issue 37 closed this crate was
  the only one of the four receiving crates carrying the obligation on prose
  alone.
- A lifecycle directive out of order is refused and not queued: a release before
  any admit answers `OutOfOrder`, a second admit answers the same whatever the
  first answered, and any directive after a release answers the same, the
  released position being terminal. Confirmed by watching the second admit reach
  the resolution step when the admitted position is allowed to accept one, and by
  watching a held release run at the next directive when the refusal is replaced
  by a queue. The positions are driven against the channel's recorded state, per
  section 9, so the fixture resolves no artifact and takes no device. **The
  second-admit arm watches the refusal and not the idempotence claim beside it,**
  section 3's no-matching rule being about what a completed residency would be
  compared against, which this fixture never produces.
- A decode directive out of order is refused and not queued: an open before the
  residency it serves is confirmed answers `OutOfOrder`, a second
  append-and-generate while one is in flight answers the same, and a flush
  arriving mid-generation answers the same. Confirmed by watching the second
  generation reach the session when the in-flight position is dropped, and by
  watching the flush reach the session's truncation path when the between-turns
  condition is removed. What each arm reads is whether the directive was
  dispatched or refused, which is what not-queued means and what the recorded
  state puts within reach of a fixture holding no residency. **The flush arm is
  the one worth having,** its perturbation corrupting a session rather than
  answering wrong, which the next turn's framing reports and the flush's own
  answer does not.
- The cheap refusals precede the device judgment: against a fixture family
  declaring the widths one and two, with a binding naming three devices, a
  binding whose artifact resolves to nothing answers the resolution refusal
  rather than the width refusal, and an artifact whose header cannot be read
  answers on the header rather than on the devices. Confirmed by watching both
  fixtures answer on the width when the resolve and header steps move below the
  judgment. **The three-device set is what makes the watch a watch,** since it
  fixes what the reordered path would answer, so the two arms differ by the
  ordering alone and neither reaches a device under either arrangement.
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
  special-token path is bypassed. **This is the inbound direction of section 5
  and only that,** the outbound direction taking the bullet below, which is why
  that clause reads as two records.
- The parse reports an unrecovered call: an emission opening a family's call
  marker and naming nothing the parser can recover answers with that fact rather
  than with a clean turn's content, confirmed by watching the fragment arrive as
  ordinary assistant text when the unrecovered case collapses into the text path.
  The fixture is a rendered string against a family module, so the watch reads
  the parse's own answer and reaches neither a model nor a device.
- The registry substitutes nothing: an artifact whose header names a family this
  binary does not carry refuses at admit naming that family, confirmed by
  watching the admit proceed on a carried family's module when the table gains a
  nearest-match fallback. The watch reads the family the refusal names rather
  than the fact of a refusal, so a refusal arriving on some other ground does not
  pass it.
- The width condition refuses against the declared set: a fixture family
  declaring the widths one, two, and four refuses a binding naming three devices,
  and the refusal names the width. Confirmed twice, by watching the binding pass
  the condition when the width test is dropped from the judgment, and by watching
  it pass when the declaration is read as a maximum. **The non-contiguous fixture
  is what makes the second watch able to fail,** a declaration of one and two
  answering alike under both readings, so a contiguous fixture would leave that
  perturbation unwatchable while the test still passed. The width is judged
  before either driver query, per section 3, which is what keeps the whole test
  on a machine with no device.
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
not checked. Their provenance is the survey. **This one is review's by reach
and not by election,** which is the rare case: what the fixtures assert about a
kernel fires when the kernel drifts, and no suite can watch a comparison that
was left behind, a test being unable to detect its own absence. Whether the
comparisons crossed is therefore a fact about the carry that a reader
establishes and a runner cannot.

**Which invariant each claim serves, and why most serve none.** Twelve `grounds` edges
run from nine of the sixty: four to `axiom-contract-is-a-complete-interface`, four
to `axiom-floor-is-vocabulary-behavior-is-socket`, three to
`axiom-harness-integrates-by-the-loop`, and one to
`axiom-organ-and-submodule`, the two out-of-order refusals carrying two edges each
because the contract states the ordering and the loop is answerable for that
ordering holding. **The test applied is whether the axiom is the reason the claim
exists, or the claim a precondition of the axiom's own stated reason.** Remove the
socket invariant and this crate has no reason to be a process the harness starts
rather than a library it calls, no reason to carry a truncation obligation attached
to a socket type nobody would have elected, and no reason to refuse a signal-borne
cancel, so those three ground in it. Remove it and the session is still append-only,
the registry still substitutes nothing, and every sampling knob still carries a
disposition, so those ground in nothing.
`axiom-join-key-travels-with-the-work` takes nothing from this crate, and the
absence is that invariant's own scope rather than a gap: every directive on the
residency seam belongs to no turn and apex section 5.2 exempts it by name, and on
the decode seam, where the work does belong to a turn, this document asserts
nothing about attribution because that seam's encoding is section 11's open
election. The nine claims section 0 names are another Spec's to ground, including
the socket election and the envelope bound the truncation obligation rests on.

**The loop invariant reaches three claims and stops at this crate's interior.** It
binds what crosses between domains and says nothing about what happens inside one,
so the decode submodule's relationship with its own organ, the one service loop over
two channels, and the placement of each family's code are outside it whatever they
resemble one level up. The three claims are on the other side of that line, each
being this crate declining what the harness orchestrates rather than settling it
here, and each argued at its own clause: the two out-of-order refusals of section 9,
where the mis-ordering is a timing failure the loop is answerable for, and the
overflow refusal of section 4.2, where what a full context means is the harness's to
decide. **Two candidates were weighed and refused.** The envelope's confinement to
the lifecycle channel grounds in the organ axiom, which is the reason it exists, and
the loop invariant adds no second reason to it. The entry check on the descriptor
count exists for the first walk above, a writable handle to the agent's own record
sitting inside the agent's model server, and the discipline it checks is the
harness's at the fork rather than the loop's at a seam.

**Fifty-one claims grounding in no invariant is the expected result and not a gap**,
per Document Format section 4, and the ratio is low here for a structural reason
rather than an unfinished one. The bulk of this document is decode mechanics,
sampling, family libraries, kernels, device judgment, residency bookkeeping, and
session monotonicity, and none of that is a question the five invariants ask. Much
of what makes this crate correct is not an invariant question. The one claim a
reader will look for and not find is `spu-nothing-retained-after-the-answer`, this
crate's face of the program's statelessness: the apex states that at section 2, as
what proto-stateful means, and its section 5 declares five invariants that do not
include it. An edge there would point at the nearest heading rather than at a
reason.

**Where the assertion records sit, and which of these bullets another crate
declares.** The records are at the clauses that argue the claims, across
sections 1 through 9, rather than gathered here, per Document Format section 6:
this section sorts by instrument and the arguments are elsewhere, so a block
here would sit apart from the prose that earns it. Four are the exception and
sit at the end of this section, being the claims argued only here: the fork
seam's doctest, the path-taking loader's two pinned shapes, that same claim's
general prohibition, and the kernels' comparisons. Sixty records in all,
thirty-one from this section's sorting with the walks and the kernels counted
in, and twenty-nine from the elections outside it, the elections taking nodes
because gate H1 would otherwise leave the largest decisions in this Spec
untraceable. A divided claim's two halves both count with the sorting, per
Document Format section 3, which is where the absence and width divisions of
the operator's 2026-08-04 election count. Three of the thirty-one are tagged
for review and every other one of them carries a mechanical instrument: the
loader's general prohibition, which is the review half of a split, the pairing
half of section 6's absence claim, divided likewise, and the kernels'
comparisons, which the paragraph above argues. **One bullet
above is a claim another crate argues,** and carries no record here: the floor's
exhaustive wire enums, which `weaver-types-Spec` section 4.2 declares. Eight
more claims this Spec cites and another Spec argues are named where the sections
use them and listed in section 0, and the shape behind all nine is that this
crate holds neither the floor's definitions nor the fork that creates its
channels.

**Thirty records carry `review`, five fewer than the assertion pass left, and
what moved is what needed neither hardware nor seam work.** The five are the
cheap refusals of section 3, the registry's refusal and the width refusal of
section 5, the truncation fault of section 2, and the outbound parse of section
5, all of them reachable with a fixture and none of them touching a device. What
remains on prose in the residency half is what reads a driver: the room and reach
conditions of the admission judgment and the release's free-before-answer
ordering. Those reach the driver through a seam a suite could double and this
Spec does not introduce, so they are unbought for the seam rather than for the
watch, and introducing that seam is a larger act than this one. Every `review`
tag in this document states its ground at its own clause, and outside two cases
the ground is that no instrument was bought rather than that none exists. The two
exceptions say so where they sit, the absence of anything laid in for an
operation type that does not exist and the kernels' comparisons above.

**The out-of-order refusal is stated and bought on both seams, per section 9,
and this crate's side of the owing is discharged.** `weaver-types-Spec` section
5 owes it to each organ and enforces it nowhere, the gate discharged its one seam
at `weaver-gate-Spec` section 6, and the two bullets above are the residency and
decode seams answering the same demand against two different ordered states. What
made this crate's half the largest of the three is that the gate and the harness
each state the behaviour and lack only a test, while this document stated it
nowhere, so the act had to write the claim before it could buy one.

```graph
node: spu-eval-callback-pinned-by-doctest
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-spu
to: spu-eval-callback-pinned-by-doctest

node: spu-loader-shapes-pinned-by-doctest
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-spu
to: spu-loader-shapes-pinned-by-doctest

node: spu-no-path-taking-loader
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-no-path-taking-loader

node: spu-kernels-cross-with-their-fixtures
kind: assertion
tag: review

edge: asserts
from: weaver-spu
to: spu-kernels-cross-with-their-fixtures
```

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
