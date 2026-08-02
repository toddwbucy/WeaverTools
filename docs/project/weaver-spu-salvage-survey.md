# weaver-spu - salvage survey

**Status:** SURVEY, 2026-08-02. Outside the document set, filed at `docs/project/`
beside the other documents the mapping does not read. Reference, not prescription:
this document describes what the archived tree's SPU holds and how it bears on the
token workflow and the SPU Spec, and it decides nothing. Where it and a merged
document disagree, the merged document wins and this survey is corrected. The
quarry is read-only throughout, per the workspace's standing rule.

**Sources:** the archived tree at `WeaverTools-archived/crates/weaver-spu` (75
files, roughly 37.6k lines) and `WeaverTools-archived/docs/architecture/crates/
weaver-spu/` (12 documents), swept 2026-08-02, with the operator's rulings of the
same date recorded in section 3. The quarry-side facts rest on the authoring
seat's sweep alone, the review seat's environment not reaching the archived
tree, and the corpus-side citations carry both seats' verification.

**Editorial:** Per the Working Rules.

---

## 0. Why this survey exists

The SPU is the last unspecced crate, held for last on the operator's ruling
because the prior program's SPU work was productive and much of it will serve
here if it is integrated properly, through the same charter-contract-Spec
pattern every other crate took. This is the third program to hold this code's
lineage: the first tree built it, the archived tree was mid-salvage of it when
the extraction was cut, and the archived tree's own survey of 2026-06-08 calls
itself the salvage map. This document is that map redrawn against the merged
corpus, so the token workflow's charter pass cites a survey instead of
rediscovering one.

The carry rule governs everything here: nothing crosses because the old tree
has it. A part crosses as live code only when a completed turn's path through
it can be named, and the classification below is drawn for that test.

## 1. What the quarry holds

**The only working SPU in existence.** One crate holding both halves: a live
llama.cpp GGUF decoder behind a fork pin, a salvaged raw-CUDA forward path
with per-family forwards for the Qwen, Llama, and Mistral families plus
separate Gemma 4 and gpt-oss paths, two working encoders behind the
`Embedder` trait, a GPU orchestrator that is the single device authority, a
per-token measurement surface, BLAKE3 weight provenance, and a test culture
that already practices this corpus's instruments.

The load-bearing parts, by location in the archived tree:

- `src/decoder/gguf.rs` - the decoder backend. Two decode paths per agent:
  a stateless prefix-reuse path, and the append-only session path that
  decodes only the turn's delta at the resident end and never touches
  resident positions. The append-only form is forced rather than elected:
  hybrid and recurrent decoder families cannot roll KV state back, so the
  rollback the prefix path performs silently fails on them.
- `src/decoder/backend.rs` - the `InferenceBackend` trait, the seam behind
  which GGUF and candle-native backends are peers by the archived tree's
  own 2026-06-11 ruling, plus per-generation metrics.
- The measurement surface - per-token entropies and surprisals computed
  pre-sampler with log-sum-exp stability, token identifiers, prefill and
  decode timings, and `tokenize_with_offsets` yielding byte offsets for an
  addressable prompt-block partition. Nearly the whole of apex section 8's
  deterministic re-feed input list exists as working code.
- `src/weights_hash.rs` and `src/core/pin.rs` - BLAKE3 snapshot identity
  over a canonical manifest with a sidecar cache and an empty-string
  sentinel on every failure path, and a cohort pin verified at load with
  per-field deltas on mismatch.
- `src/core/gpu_orchestrator.rs` - two-phase admission, pre-check then
  admit then commit or cancel, with RAII in-flight reservations so a failed
  load returns its budget on every error path, and busy-guards so nothing
  is released mid-inference.
- `src/decoder/prompt.rs` and `family.rs` - per-family chat rendering and
  tool-call parsing with three wire guarantees the archived docs pin:
  order-preserving tool render, byte-exact name round-trip with two
  distinct never-collapsed failure flags, and verbatim raw output kept as
  the training target.
- `kernels/transformer.cu` and `build.rs` - the twice-salvaged CUDA kernel
  set, 23 launchers, already marked verbatim-salvage in the archived tree.
- The readout paths - the candle fork's `forward_with_intermediates`
  returns per-layer residuals and is exercised by a working probe binary.
  The GGUF side is pinned but unused: the fork exposes the ggml scheduler
  eval callback, and a compile-fail test guards the seam so reverting to
  the upstream crate cannot silently remove the capability.
- The test culture - golden fixtures with candle as the numerical oracle,
  a session end-to-end test whose acceptance is zero errors across many
  turns with flat per-turn prompt tokens, a marker-promotion test that is
  perturbation-shaped, and a conformance corpus of deliberately flawed
  stimuli marked do-not-fix.

## 2. What maps across

| Quarry part | Where it lands in this corpus |
|---|---|
| Append-only session protocol, `resident_len`, delta-only decode | The decode socket's native protocol, chartered by the token workflow, with the forced-by-model-family argument as charter-grade rationale |
| Per-token entropies, surprisals, ids, timings, byte offsets | The `model.measurement` payload of `weaver-trace-PRD` section 3.1 |
| `weights_hash` and the cohort pin | Model identity and weights hash in the measurement payload, pin verification at admission |
| Two-phase admission with RAII reservations | The admit exchange's obligations in `weaver-harness-spu-contract` sections 4 and 5, mechanism included |
| Encoder and decoder claims, admission inequality, one agent per pair | The residency accounting the SPU charter owns, re-derived for one agent per SPU |
| Family registry, per-family rendering, tool-call parsers | The harness's per-model assembly layer and the token workflow's message shapes |
| `forward_with_intermediates` and the eval-callback pin | Residual readout, proven on the candle path and one spike from proven on GGUF |
| Session e2e, golden fixtures, marker promotion, conformance corpus | The perturbation-verified test set and review stimuli, ready-made |
| `InferenceBackend` with GGUF and candle-native as peers | The decoder's interior seam, under the umbrella the vision's section 9 grows |

## 3. What yields to merged rulings

Working code loses to merged intent at six places, and naming them is what
integrating properly means.

**The tenancy apparatus dissolves entirely, and it is the largest
simplification available.** The quarry's SPU was a fleet daemon: one process
under the admin principal, a socket at mode 0666, many agent uids
connecting, owner strings bound to the peer credential, monotonic session
tokens with zero reserved invalid, admin-only force verbs, and passwd scans
failing closed on ambiguity. The merged topology makes the SPU a per-agent
child the harness forks over an unnamed pair, so possession replaces every
line of it. The ownership Spec's lessons, fail loud on stale and reserve
the zero token, stay as design wisdom. The apparatus does not cross.

**Auto-eviction does not cross.** The quarry's slot policy evicts an
occupant on a new load. `weaver-spu-PRD` section 2 rules that admission
refuses and never evicts, and the ruling wins.

**The anvil inverts.** The quarry deliberately kept models resident across
agent lifetimes, unload unwired from worker shutdown, because agents came
and went over one hot daemon. Under one SPU per agent, residency is the
run: admit on enter, release on leave, the device freed. The cache that
stays hot across turns inside a run survives, which is the property apex
section 2 protects. Hot across agents does not.

**Chat templates are an end-to-end correctness requirement in both
directions, per the operator's ruling of 2026-08-02, and the ownership
call sits inside it.** Each model carries its own template and its own
output style, so the requirement reads in two halves. Inbound: what a
client sends crosses the gate opaque, the harness interprets and
assembles it per model, and the SPU receives it, and the whole chain must
deliver input formatted correctly for that model's family, a property
verified per family rather than assumed, the quarry's marker-promotion
test being the reference shape, every control marker tokenizing to
exactly one token because a degraded marker is structure the model reads
as prose. Outbound: the model emits in its family's own style,
think-blocks, Harmony channels, family tool-call markers, and the trace
must handle that style by holding both layers, the verbatim family-styled
emission in `model.output`, the quarry's `raw` kept as the training
target, and the canonical parsed form in the message kinds, with the
per-family parsers and their never-collapsed failure flags as the
recorded bridge between them, never a flattening that loses the raw.
Inside this sits the replay half: the mapping from canonical messages to
the rendered token reality is trace content, carried by the measurement
payload's token identifiers and prompt-block partition, the quarry's
`tokenize_with_offsets` its working ancestor, per apex section 8's rule
that tokenization is reproducible from what is recorded. The quarry's
session Spec ruled the server owns framing and the merged harness charter
owns assembly per model, and the token workflow adjudicates that owner
with the whole requirement in hand rather than before it.

**Protocol compatibility sits in front of the gate, outside the program,
per the operator's ruling of 2026-08-02.** The quarry's own late ruling
moved OpenAI compatibility outward to its gate. This corpus goes one step
further: the program's surface ends at the world contract's NDJSON line,
and a client that speaks another dialect gets an adapter the operator runs
on the operator's own compute, before the gate. The symmetry is the point
and worth keeping: everything past the sink is the operator's, everything
before the gate is the operator's, and the program's two external surfaces
both end at NDJSON lines. The quarry's OpenAI server surface stays behind
in its entirety as reference.

**Small frictions.** The quarry SPU carries tracing with a subscriber, which
this corpus's no-second-account rule declines. It carries tokio, where the
executor election is deferred to the token workflow with a latency
measurement. Its shared-encoder statelessness contract dissolves under one
SPU per agent, though per-agent-blindness survives as hygiene when the
encoder arrives.

## 4. The doors

Per the workspace's carry rule, each crossing names its door at the moment
it crosses. This survey pre-classifies candidates and binds nothing, with
one ranking recorded above the classification.

**The operator's ranking, ruled 2026-08-02: three parts are the salvage's
spine, the ones not to hand-roll again.** The CUDA kernels, hand-rolled
once and salvaged verbatim twice already, so a third carry is a proven
motion. The GGUF integration, which carries the append-only session core
with it, the two being one body of code in the decoder backend. And the
residual stream tap this program's lineage built, the candle fork's
`forward_with_intermediates` with its working probe on one side and the
llama-cpp fork's eval-callback seam on the other. The third is the one
whose existence rests entirely on the fork pins of the preconditions
below, which is why those pins are verified at first use rather than
assumed.

**Live-code candidates, a completed turn's path nameable through each:**
the CUDA kernels and `build.rs`, the per-family forwards and sharding, the
GGUF decode and append-only session core, the sampling chain, the entropy
and surprisal math, `weights_hash` and the pin, the prompt renderers and
tool-call parsers, the family registry, the orchestrator's two-phase
admission core, the session e2e and golden fixtures.

**Stays behind:** the OpenAI-compatible server surface, the tenancy and
ownership apparatus, the multi-model fleet catalog, the slot policy's
eviction, the hf-hub download path, model-suggestion tooling, the legacy
Python-embedder remnants, and the logging stack. Each is either dissolved
by the merged topology, ruled out by a merged document, or operator-side
tooling under the rulings of section 3.

**Preconditions, verified before anything crosses:** the two fork pins are
the load-bearing external facts. The llama-cpp fork exposes the eval
callback seam, guarded in-tree by a compile-fail test. The candle fork
carries `forward_with_intermediates` for the models the readout needs.
Both pins are named in the workspace's standing notes and both are
verified at the moment of first use rather than assumed from this survey.

## 5. What the token workflow inherits

The decoder cut is the workflow's brief, and this survey fills in its
material. The workflow charters: the decode socket's native verbs, open
and append-generate and close, drawn from the session protocol minus its
tenancy. The measurement payload's wire shape, mostly done in section 1's
terms. The framing-ownership ruling of section 3, taken together with the
template-to-trace mapping that section binds: who renders, how the
rendered form and its block partition reach `model.request` and
`model.measurement`, and how a template's identity is visible in the
record rather than silently re-rendered at replay. Stop at the decoder,
where the quarry offers nothing: no cancellation surface exists in the
archived tree, a generation cannot be aborted mid-loop, so that cell is
new work rather than salvage. The disposition knobs, where the
quarry's seed exists only as a hardcoded default and a determinism test,
never wire-configurable, so the disposition principle's type,
`Disposition<T>` the natural candidate and the SPU Spec's to take or
leave, is the seed's first real home. And the backend-peers seam as the decoder's interior, GGUF and
candle-native first-class under the umbrella.

The SPU Spec then follows the workflow, residency plus the decoder per the
decoder cut, written against settled material with this survey as its
quarry map.
