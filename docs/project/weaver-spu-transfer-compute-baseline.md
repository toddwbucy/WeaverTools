# The decode loop's baseline: transfer against compute, measured per leg

**Measured 2026-08-17, per issue #102.** That issue filed estimates from
adjacent kernel work so that SPU tuning would start from a measurement rather
than from intuition, and said taking the numbers was the point. These are the
numbers, taken on one workshop; the deployment box takes its own with the same
method and this document gains a second column when it does.

**Box:** RTX PRO 5000 Blackwell Generation Laptop GPU, 24 GB, CUDA 13.3,
`llama-cpp-rs` at the pinned `277e4100`, tree at `5ceecea`.

## Leg one: context transfer, host to device

A standalone CUDA microbench: `cudaMemcpy` host-to-device, 2,000 iterations
per size after 200 warm, timed with device events. Context-sized buffers,
both host allocation classes.

| size | pageable | pinned |
|---|---|---|
| 4 KiB | 6.99 us | 5.59 us |
| 16 KiB | 7.46 us | 6.43 us |
| 64 KiB | 27.61 us | 6.23 us |
| 256 KiB | 83.20 us | 10.83 us |
| 1 MiB | 330.52 us | 31.24 us |

The issue's estimate for the 4 to 64 KiB band was 8.5 to 11 us. **Measured:
5.6 to 6.2 us pinned, and pageable holds the band only to 16 KiB.** The one
fact here the estimates did not carry: pageable transfer leaves the band by
64 KiB and is 10x out by 1 MiB, so the allocation class matters an order of
magnitude before the transfer size does.

## Leg two: the decode step

Read from the SPU's own measurement payload, `timings.decode_ns` over the
retained token count, through the full decode seam - the same path a
production turn takes, socket framing and measurement production included.
One generation each, single-digit token counts, so these are
order-of-magnitude figures rather than tight ones; a longer-generation pass
sharpens them when tuning actually wants the precision.

| model | class | decode | per token, approx |
|---|---|---|---|
| Devstral-Small-2 24B Q4_K_M | dense | 153.1 ms / 3 tokens | ~38 ms |
| gemma-4 26B-A4B Q4_K_XL | sparse, 128 experts | 96.8 ms / 3 tokens | ~24 ms |
| Qwen3.6 35B-A3B IQ1_M | sparse, 256 experts | 121.8 ms / 5 tokens | ~20 ms |

Prefill of a one-message delta ran 13 to 19 ms across the three.

## The ratio, and what it buys

**A 4 to 64 KiB pinned transfer costs 5.6 to 6.2 us against a 20 to 38 ms
decode step: roughly 3,000 to 6,500x.** The issue estimated 3,000 to 6,000x
on an A6000 and the estimate holds on this box.

The implications the issue stated therefore stand as measured facts here
rather than as projections:

- **Tuning the transfer path is not where the milliseconds are.** Shaving
  microseconds off a number contributing a few hundredths of a percent of the
  loop is unmeasurable at the application level, and trading DMA isolation
  for it is a bad exchange.
- **The headroom opens architectural options.** Re-assembling context per
  step, swapping working sets between host and device, and keeping structures
  in host RAM with slices pulled on demand are all close to free at these
  transfer costs. Being wasteful with transfers in exchange for simpler logic
  is a good trade at this ratio.
- One measured qualifier to both: the free lunch is **pinned** transfer.
  Pageable at context sizes above 16 KiB erodes the ratio by an order of
  magnitude, so a design leaning on cheap transfers names its allocation
  class.

## The constraint the baseline does not measure

Recorded on the issue and carried here so the ledger travels with the
numbers: **the trace's transform-free splice is contingent on the wire
encoding staying JSON.** The record splices wire bytes as they stand,
`RawValue` members throughout, which is what keeps the harness's job byte
movement rather than transformation. Any future proposal that re-encodes the
decode seam for speed weighs that loss against a transfer budget this
baseline shows is already negligible - which is to say, the burden of proof
sits on the re-encoding.

## Method, for the second column

Transfer: `xferbench.cu`, `nvcc -O2`, sizes as tabled, 2,000 iterations,
event-timed. Decode: run `tests/loaded.rs`
`an_opened_session_generates_across_the_decode_seam` with `WEAVER_TEST_GGUF`
naming the artifact and read `timings` from the measurement the seam already
carries; no instrument needs building. Take both legs on the same box in the
same session and record the tree and the pin beside them.
