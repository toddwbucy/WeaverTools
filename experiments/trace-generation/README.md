# 8B trace generation

The scripted form of the sketch of 2026-08-31. The traces are the product:
every measurement discussed downstream of the Stage 1 report runs against
long multi-turn 8B traces that did not exist, because everything recorded
before this was two turns and out on a model below the working band. This
driver makes the source material and takes no measurement beyond the
fidelity check each session produces as a byproduct of the
serve-unload-reload-reissue protocol.

It is not a diagnostic run, not the qualification sweep, and not a matrix.
The commit pin is read from the repository at run time and lands in every
deposit - the run must not reproduce the Stage 1 report's provenance gap.

## Vocabulary

A trace contains sessions. A session is what happens between a load and an
unload. Turns live inside a session. The Stage 1 report's 25,840 are
minimal sessions under this ruling, and this run's headline count will be
in the tens while its generation per session is up around fiftyfold - the
next report says the unit got bigger rather than putting the two numbers
side by side.

## The two machines are two acts

Olympus is where numbers come from and runs the full 32k window. The
thinkpad answers whether the stack runs on Blackwell at all, at whatever
window 24 GB reaches - 16k expected, and finding the ceiling is the
thinkpad's own finding (cell 7.3). The runs share this protocol and do not
share a claim. A completed thinkpad run at the smaller window is a pass on
its own terms, and its schedule is sized to its window rather than to
olympus's.

## The session set

Three runs per sweep, sweep before repeat, three sweeps:

| session | turns | content |
|---|---|---|
| A-long, A-short | 1 each | one story prompt, asked at ~2,000 then ~500 words |
| B-a, B-b | 1 each | the long ask twice, full unload between |
| C | 8 | factual questions, requested length stepping up |

The finding per run: A, the position of first divergence between the two
emissions. B, byte equality extended into depth. C, byte equality across
the replay of a session that stays loaded.

The story subject has no famous single continuation, so the model
generates rather than recites. A divergence between A-long and A-short is
expected - the length instruction changes the prompt - and the quantity of
interest is where: at the first position, the requested length reshapes
the plan, where a long shared prefix means length is a stopping behavior.
Run C states its prediction before the run: a divergence at turn N is turn
N-1's error amplified, so the finding is the first failing turn, never
the fact of failure.

## Parameters and the arithmetic that set them (cell 7.1)

Schedules, in output tokens per turn, words derived at ~1.35 tokens/word:

    full-32k  500 750 1000 1500 2000 2500 3000 3750  = 15,000
    fit-16k   250 375  500  750 1000 1250 1500 1875  =  7,500

The full session ends near 15.4k resident in 32,768. The property that
sized it: a model overshooting every request by half still ends near 23k
and fits. The fit-16k schedule holds the same property against 16,384.

`max-tokens-per-turn` is patched to 6,144 on both boxes - 1.6x the largest
request - and a turn that hits it anyway **fails the session** rather than
recording, because a capped turn is a defective specimen for source
material. Three of five turn-2 cells in the 08-30 ladder hit the old
1,024 cap, which is what this parameter exists to not repeat.

Elision is held at one depth by not eliding: the loops composing prompts
are unfinished (#382), so varying elision tonight would measure the loop
rather than the replay.

## The night, priced (cell 7.2)

At the A6000's measured 40.6 tok/s on this artifact (08-30 ladder, bf16
t-2: 1,024 tokens in 25.24 s):

    per sweep   ~48,400 generated tokens (source + reissue)  ~20 min
                10 loads of a 16 GB artifact                  ~5-10 min
    three sweeps                                              ~1.5 h

The night fits with hours to spare, so repetition is not the thing to cut,
and the Ada arm (cell 7.4) is affordable if reached. The thinkpad's
fit-16k sweep is ~32k tokens at an unmeasured Blackwell rate - even a
slow one clears three sweeps in an evening.

## Running it

Both boxes, having installed the bf16 rung locally from the handoff share
(the driver hashes the file it loads and refuses nothing - the hash lands
in the deposit for the comparison to judge):

    python3 generate_traces.py --config thinkpad-8b.json --outdir <deposit>
    python3 generate_traces.py --config olympus-8b.json  --outdir <deposit>

The declaration is patched (artifact, context-capacity,
max-tokens-per-turn), hashed before and after, and restored on every exit
path. The report is rewritten after every session, so a failure mid-night
costs the tail and never the record.

## What the deposit carries

Per session: the source and replay runs as ndjson, the eight compared
fields per turn, wall times. Once: commit, build flags, toolchain, binary
and engine-library hashes, the artifact's sha256 as loaded, both
declaration hashes, the serving device per load half, the cccl version
read from the header the build includes, and the kernel guards derived
from it - named `_derived` because until the loop-identity act lands, the
guard state is computed rather than read from a shipped object.

## The loop is declared by digest

Every config carries `loop_sha256`, the sha256 of the loop file the box
composes with, and the shared driver refuses a session whose load event
records another digest, or none, before a turn is served through it. A
refused session deposits no runs and carries
`"loop_refused": {"declared": ..., "recorded": ...}` beside a verdict that
names it, and the night's summary counts those sessions apart from the
REPRODUCED tally. The digest and not the filename is the identity, a config
with no key is unchecked, and the digest is read with `sha256sum` against
the deployed file. The cross-precision README is the authority on the key,
per issue #426. The olympus configs declare `alpha_loop.py`, the loop their
declaration names, and the thinkpad config declares `dev_loop.py`, the loop
both boxes ran when #426 was measured, so a thinkpad that has moved is told
so by the refusal rather than by a guess made here.

## Open cells this run settles, and does not

    7.1  settled here, by the arithmetic above
    7.2  settled here, by the pricing above - cut repetition, never coverage
    7.3  settled by the thinkpad run itself, recorded as its finding
    7.4  settled by one comparison if the night reaches it
    7.5  not tonight - bf16-only is the bet, the ladder is a separate question
    7.6  settled by the qualification sweep, later, over this family
