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
| 4 KiB | 5.13 us | 6.96 us | 5.58 us | 6.51 us |
| 16 KiB | 7.21 us | 8.11 us | 6.94 us | 7.80 us |
| 64 KiB | 27.60 us | 29.58 us | 8.11 us | 8.26 us |
| 256 KiB | 83.52 us | 83.78 us | 12.65 us | 12.33 us |
| 1 MiB | 326.80 us | 327.88 us | 32.09 us | 31.95 us |

The issue's estimate for the 4 to 64 KiB band was 8.5 to 11 us. **Measured:
6.5 to 8.3 us pinned in the synced view**, which is the view the estimate
described. The fact the estimate did not carry: pageable synchronized
transfer runs about 3.6x slower than pinned at 64 KiB, 29.6 us against 8.3,
and about 10x slower at 1 MiB, 327.9 us against 32.0 - roughly 30 to 39x
above the estimate band - so the allocation class matters an order of
magnitude before the transfer size does.

## Leg two: the decode step

Read from the SPU's own measurement payload through the full decode seam -
the same path a production turn takes, so socket framing and measurement
production are exercised by the run, **and excluded from the timed span**:
`timings.decode_ns` opens and closes inside the SPU around the sampling loop,
so what it times is the loop through the terminator's decode and nothing of
the seam around it. One generation per model, single-run observations rather
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
synced 4 to 64 KiB band (6.5 to 8.3 us) for the transfer and both decode
readings for the compute:

- per draw: 20.3 ms / 8.3 us to 38.3 ms / 6.5 us, **roughly 2,500 to 5,900x**
- per retained token: 24.4 ms / 8.3 us to 51.0 ms / 6.5 us, **roughly 3,000
  to 7,800x**

The issue estimated 3,000 to 6,000x on an A6000. The per-retained-token
range brackets that estimate; the per-draw range overlaps its lower half and
tops out below it. Under either denominator the estimate's order of
magnitude holds, which is what the issue asked the measurement to settle.

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

Transfer leg: the bench is small enough to carry whole, so the source is
here rather than referenced, and the reported results came from exactly this
text compiled with `nvcc -O2 -o xferbench xferbench.cu` under CUDA 13.3.

```c
// H2D transfer microbench: host wall-clock through device synchronization,
// batched and per-copy-synced, pageable and pinned. Every CUDA call and the
// pageable allocation are checked.
#include <cstdio>
#include <cstdlib>
#include <chrono>
#include <cuda_runtime.h>
#define CK(x) do{ cudaError_t e=(x); if(e){printf("ERR %s\n",cudaGetErrorString(e)); return 1;} }while(0)
using clk = std::chrono::steady_clock;
int main(){
    cudaDeviceProp p; CK(cudaGetDeviceProperties(&p,0));
    printf("device: %s\n", p.name);
    const size_t sizes[] = {4096, 16384, 65536, 262144, 1048576};
    const int iters = 2000, warm = 200;
    void *dev; CK(cudaMalloc(&dev, 1048576));
    for (int pinned = 0; pinned <= 1; pinned++){
        void *host;
        if (pinned) { CK(cudaMallocHost(&host, 1048576)); }
        else { host = malloc(1048576); if (!host) { printf("ERR malloc\n"); return 1; } }
        for (size_t s : sizes){
            for (int i=0;i<warm;i++) CK(cudaMemcpy(dev,host,s,cudaMemcpyHostToDevice));
            CK(cudaDeviceSynchronize());
            auto t0 = clk::now();
            for (int i=0;i<iters;i++) CK(cudaMemcpy(dev,host,s,cudaMemcpyHostToDevice));
            CK(cudaDeviceSynchronize());
            double batched = std::chrono::duration<double,std::micro>(clk::now()-t0).count()/iters;
            t0 = clk::now();
            for (int i=0;i<iters;i++){ CK(cudaMemcpy(dev,host,s,cudaMemcpyHostToDevice)); CK(cudaDeviceSynchronize()); }
            double synced = std::chrono::duration<double,std::micro>(clk::now()-t0).count()/iters;
            printf("%s %7zu B  batched %8.2f us/copy   synced %8.2f us/copy\n",
                   pinned?"pinned  ":"pageable", s, batched, synced);
        }
        if (pinned) { CK(cudaFreeHost(host)); } else { free(host); }
    }
    return 0;
}
```

Decode leg, per model, with the artifact's path in a variable so the command
pastes as written:

```sh
ARTIFACT=/path/to/model.gguf
CUDA_PATH=/opt/cuda WEAVER_TEST_GGUF="$ARTIFACT" \
  cargo test -p weaver-spu --features cuda,gguf --test loaded \
  an_opened_session_generates -- --nocapture
```

and read `timings.decode_ns`, `timings.prefill_ns`, and the length of
`output_tokens` from the measurement the seam already carries; no instrument
needs building. Report the span and the retained count, then both per-token
readings with the denominators above. Take both legs on the same box in the
same session and record the box, tree, and pin beside them.
