# The decode loop's baseline: transfer against compute, measured per leg

**Measured 2026-08-17, per issue #102.** That issue filed estimates from
adjacent kernel work so that SPU tuning would start from a measurement rather
than from intuition, and said taking the numbers was the point. These are the
numbers, taken on one workshop; the deployment box takes its own with the same
method and this document gains a second column when it does.

**Two isolated legs, not a turn profile.** Each leg is measured alone, so what
this document supports is the *ratio* between them and nothing finer: it does
not measure what fraction of a real turn's wall clock transfers occupy, and it
says nothing about overlap, queueing, synchronization cost, context-assembly
cost, or contention between the two legs, because no end-to-end trace was
taken. Where the conclusions below lean on the ratio they are marked as
inferences from measured inputs, which is one step firmer than the issue's
estimates and one step short of a turn profile.

**Box:** RTX PRO 5000 Blackwell Generation Laptop GPU, 24 GB, CUDA 13.3,
`llama-cpp-rs` at the pinned `277e4100`, tree at `5ceecea`.

## Leg one: context transfer, host to device

A standalone CUDA microbench: `cudaMemcpy` host to device, 2,000 iterations
per cell after 200 warm-up copies, **host wall clock through device
synchronization**, so every figure includes the staging and DMA work a caller
actually waits for. Two timing shapes per cell: *batched* runs the copies back
to back with one synchronization at the end, the throughput view; *synced*
synchronizes after every copy, the latency view a single context handoff
sees. Each figure is the mean over the 2,000 iterations of one run.

| size | pageable, batched | pageable, synced | pinned, batched | pinned, synced |
|---|---|---|---|---|
| 4 KiB | 6.40 us | 7.84 us | 5.71 us | 7.61 us |
| 16 KiB | 7.90 us | 7.56 us | 7.44 us | 8.89 us |
| 64 KiB | 27.70 us | 28.60 us | 7.97 us | 8.07 us |
| 256 KiB | 82.92 us | 83.07 us | 12.57 us | 12.48 us |
| 1 MiB | 326.25 us | 327.51 us | 32.17 us | 32.60 us |

The issue's estimate for the 4 to 64 KiB band was 8.5 to 11 us. **Measured:
7.6 to 8.9 us pinned in the synced view**, which is the view the estimate
described. The fact the estimate did not carry: pageable transfer leaves the
band at 64 KiB and is 10x out by 1 MiB, so the allocation class matters an
order of magnitude before the transfer size does.

## Leg two: the decode step

Read from the SPU's own measurement payload through the full decode seam -
the same path a production turn takes, socket framing and measurement
production included. One generation per model, single-run observations rather
than aggregates, so these are order-of-magnitude figures; a
longer-generation pass sharpens them when tuning actually wants precision.

**The timing denominator, stated exactly.** `timings.decode_ns` opens after
the delta's prefill and closes after the generation returns, so its span
holds: one decode-and-draw per retained token, the draw of the stop token
that is not retained, and the decode that makes the terminator resident. Two
readings follow and both are tabled - per draw divides by retained + 1 and
excludes only the terminator's decode from its denominator; per retained
token divides by what the answer carries and is the conservative upper read.

| model | class | decode span | per draw | per retained token |
|---|---|---|---|---|
| Devstral-Small-2 24B Q4_K_M | dense | 153.1 ms / 3 retained | ~38.3 ms | ~51.0 ms |
| gemma-4 26B-A4B Q4_K_XL | sparse, 128 experts | 96.8 ms / 3 retained | ~24.2 ms | ~32.3 ms |
| Qwen3.6 35B-A3B IQ1_M | sparse, 256 experts | 121.8 ms / 5 retained | ~20.3 ms | ~24.4 ms |

Prefill of a one-message delta ran 13 to 19 ms across the three
(`timings.prefill_ns`, same payload).

## The ratio

Ratio = decode time per token / transfer time per copy, taking the pinned
synced 4 to 64 KiB band (7.6 to 8.9 us) for the transfer and both decode
readings for the compute:

- per draw: 20.3 ms / 8.9 us to 38.3 ms / 7.6 us, **roughly 2,300 to 5,000x**
- per retained token: 24.4 ms / 8.9 us to 51.0 ms / 7.6 us, **roughly 2,700
  to 6,700x**

The issue estimated 3,000 to 6,000x on an A6000. The measured band brackets
it under either denominator.

## What the ratio supports, and what it does not

**Supported, as inferences from measured inputs:** a context-sized pinned
transfer costs three to four orders of magnitude less than one decode step on
this box. Tuning effort aimed at the transfer path buys time in a currency
the decode step dwarfs, and design room that spends transfers to simplify
logic - re-assembly per step, host-resident structures with slices pulled on
demand - is cheap *in transfer cost*.

**Not supported without an end-to-end trace, and asserted nowhere in this
document:** what fraction of a real turn transfers occupy; how transfers and
decode overlap or contend on the copy and compute engines; what queueing or
synchronization adds when both legs run under the serial service loop; what
context assembly itself costs on the host side; and whether batching
transfers buys anything. A turn profile through the trace is the instrument
for those, and taking one is the natural next measurement if a design
decision ever actually turns on them. The measured qualifier that survives
either way: the cheap-transfer claim is about **pinned** allocations, and
pageable at 64 KiB and above erodes it by an order of magnitude.

## The constraint the baseline does not measure

Recorded on the issue and carried here so the ledger travels with the
numbers: **the record's splice economy is contingent on the wire encoding
staying JSON.** The record's payload boxes splice spliceable members as they
arrive - `RawValue` members carried as JSON text, avoiding a
parse-and-re-encode on the harness - and the turn frame member crosses as
octets encoded base64, `TurnFrame::carry` encoding and
`TurnFrame::octets()` validating and decoding, so the economy is
transform-avoidance on JSON text rather than raw octets moving unencoded.
Any future proposal that re-encodes the decode seam for speed weighs that
splice economy against a transfer budget this baseline shows is already
negligible - which is to say, the burden of proof sits on the re-encoding.

## Method, exactly, for the second column

Transfer leg:

    nvcc -O2 -o xferbench2 xferbench2.cu   # source shape as described above:
    ./xferbench2                           # 2000 iters/cell, 200 warm, wall
                                           # clock through cudaDeviceSynchronize,
                                           # batched and synced shapes, pageable
                                           # and pinned, mean per copy reported

Decode leg, per model:

    CUDA_PATH=/opt/cuda WEAVER_TEST_GGUF=<artifact> \
      cargo test -p weaver-spu --features cuda,gguf --test loaded \
      an_opened_session_generates -- --nocapture

and read `timings.decode_ns`, `timings.prefill_ns`, and the length of
`output_tokens` from the measurement the seam already carries; no instrument
needs building. Report the span and the retained count, then both per-token
readings with the denominators above. Take both legs on the same box in the
same session and record the box, tree, and pin beside them.
