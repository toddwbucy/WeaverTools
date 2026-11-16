# Pre-registration: the first agent's model on the host and in a guest

Filed 2026-11-16, before any run of these cells, for toddwbucy/WeaverTools#28. The
commit that adds this file is the record of when the prediction was made. Nothing in it
changes after the first run. A change found to be needed is a new dated file beside this
one, and the result note reads against both.

This is a second measurement beside `2026-11-15-preregistration.md`, with its own
prediction and falsifier. Runs 1 to 4 of that file, run on 2026-11-16 with two small
models on one GPU, are a separate and earlier measurement, and this file does not change
how they are read. Runs 5 and 6 of that file are deferred
(`2026-11-16-runs-5-and-6-deferred.md`). The run numbers below belong to this file
alone.

## 1. Question

The first agent's model and adapter, its capacities and sampler, the prompt in force,
the request stream, both GPUs and the driver release are held. Does every turn emit the
same token ids, surprisals and entropies when it runs on the host and when it runs in an
Ubuntu 26.04 guest that holds both GPUs under passthrough? Is this true for both byte
sets of the 2026-11-15 file?

## 2. Prediction and falsifier

**Prediction.** Every cell below is identical to every other cell on every turn, to
float exactness, in:

- the rendered request;
- the input token ids;
- the output token ids;
- the per-token surprisals;
- the per-token entropies;
- the emission.

**Falsifier.** Any of the following, between any two cells:

- a difference in any of the fields above on any turn;
- one cell has a turn that the other lacks.

## 3. What runs, and what it is not

- **The model**: Granite 4.2 30B at bf16, GGUF, sha256
  `082f9db8c5bf3fc25af2bf6475526b784d641f6cd453141d5319c5d2aaf1588b`.
- **The adapter**: the first agent's STE conversion adapter, epoch 2, f32 GGUF, sha256
  `3e668e78aa4ec186d7e3b67a97775d53c819ca60c29ecca452b455078638a212`, base-blake3
  `41b2efe4743c896072aeebf0b6a24c73afb40642b5aa93d9e31f252daef048fc`, scale 1.0.
- **The declaration** is the first agent's declaration of 2026-11-13 with three changes:
  - The state store is `none` and there is no `[state-election]`. The operator ruled
    that the first production agent declares no state, and admin refuses an election
    beside the engine `none`.
  - The access rule names the operator.
  - The trace goes to the scratch agent's own territory.

  It keeps:
  - `binding-kind = "serving"` and `turn-context = "independent"`;
  - `devices = [0, 1]` and the adapter table;
  - `context-capacity = 32768` and `max-tokens-per-turn = 4096`;
  - the sampler: seed 451234785645, temperature 0.7, top-k 40, top-p 0.95,
    repetition-penalty 1.1, repetition-window 64.

  The declaration is the same byte for byte on both sides, apart from the trace path,
  and each side's file is recorded by sha256.
- **The prompt in force** is the worker's compiled system prompt at WeaverAgent
  `6b91747e`:
  "You are a careful assistant. Answer from what you know, say plainly when you do not
  know, and keep answers as short as the question allows."
  The sha256 of its UTF-8 text is
  `3393d8e1c721c8197209a192376c0230bad6829c8a077e0caa5f3eb1d9963e05`.
  - These are the same bytes as in runs 1 to 4 of the 2026-11-15 file. Every load in
    those runs' traces carries this text as its system message.
  - The agents are laid as those were: storeless, with the same unseeded prompt draft,
    and no seeding. The operator ruled for the same prompt as the other agents, so that
    the cells are comparable.
  - It is not the first agent's STE conversion prompt. At `6b91747e` a storeless agent
    under `independent` has no seeded prompt in force past its seeding turn
    (`deploy/HowToDeployANewAgent.md`, lines 533 to 537). Where nothing is seated, the
    compiled prompt leads each turn (lines 560 to 568).
  - How the STE prompt reaches a storeless agent is a separate question, and this
    measurement does not address it.
- **The request stream** is `tools/ste-driver` at `6b91747e` (binary sha256
  `e7c7808b4ea04df1a2b93ca31c5d9fcdf124041dcc6a5f28f023b2e920613127`, built with
  `--locked`), one `evaluate` per run, under one load:
  - the frozen kit of 2026-11-13: 60 segments, sha256
    `ca668d0b296d5bf54086756039c77888afc2bb2ac3f93d3d7ca014977ef235fe`;
  - the six lexicon and prompt-bundle inputs, each by sha256 in the deposit;
  - caps: repair passes 2, offer passes 1, transport stop bound 0, gate-call deadline
    900 seconds;
  - `deploy/turn.py` with sha256
    `2b54f21dab8dd826f398758846312ce98c8a51de3cbe97abe9a445002a4fc59e`.

  The driver chooses each request from the kit and from the model's earlier outputs:
  what it sends after a turn depends on its own checks of that turn's output (repairs,
  within the caps).
  - While the outputs are identical, the streams are identical by construction.
  - At the first differing turn the streams may part. That turn is the result, and no
    turn after it is compared.
  - The turn count is the driver's, between 60 and 180. It need not be the 86 turns of
    2026-11-13.
- **Not measured**: the STE prompt, state, and the quality of the conversions.

## 4. The cells, in the order they run

Both GPUs move together between the host and the guest. The order is host, guest, host
again, so a drift within one side is visible.

| Run | Where | Byte set |
| --- | --- | --- |
| 1 | host | H |
| 2 | host | G |
| 3 | guest | H |
| 4 | guest | G |
| 5 | host | G |
| 6 | host | H |

Before run 1, one pre-flight in the same session: one load of run 1's agent, one short
turn and one unload. It is not a cell and is not compared.

The guest is a clone of the 2026-11-15 file's guest image. It runs driver 620.70.06, the
host's release, with a host-passthrough CPU, 12 virtual CPUs and 96 GiB. The model and
the adapter are shared into it read-only at the same paths as on the host. There is no
CPU-model cell and no driver-branch cell.

## 5. What is held, and how each is read

As in sections 5 and 6 of the 2026-11-15 file, with these additions:

- **Both GPUs**, paired by serial and by UUID on each side, not by bus address. Device 0
  and device 1 are the same physical card on both sides. While a run's process holds
  them, it must hold exactly those two cards and no other.
- **The model and the adapter**, by sha256 before each session. Any GGUF that a run's
  process maps must hash to one of the two.
- **The byte sets H and G**, the CUDA runtime and the driver's userspace, under the same
  rules for mapped files as the 2026-11-15 file.

## 6. The comparison

A comparison script reads the traces alone (sha256
`24eed9379b851e399e7b87e200f1cd6baddfb904d4dd542bb7ce1d081c760ce0`, carried in the deposit). It matches turns by turn key and
compares each of these exactly:

- the rendered request, with the sampling settings and the generation seed;
- the input token ids;
- the output token ids;
- the surprisals and the entropies, each value as its JSON text, so at f32 bits;
- the emission and the finish;
- resident and capacity.

Pairs: every cell against run 1, and H against G within each side.

These run-level fields were ruled before the run to differ by construction:

- Between H and G: the library path, the stack's hashes, the engine's library paths and
  digests, and the declaration's digest, through its trace path.
- Between the host and the guest: the CPU's memory total in the device facts.

Any other run-level difference is reported as a finding.

## 7. Report

As counts, not rates, for each pair:

- turns compared and turns identical;
- the first differing turn and its field, if there is one;
- whether the turn sequences match.

The result note names every held field that turned out to differ.

## 8. Where the evidence goes

A deposit under `weaver-testing/`. This repository carries the result note and the
scripts behind its figures.
