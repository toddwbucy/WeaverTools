# Pre-registration: the first agent's model reproduces in the 3.0 appliance

Drafted 2026-11-18, before any run of these cells. A cardless rehearsal of the
appliance's kit ran first: it loads no model and sends no turn, and is not a cell. Two
sections are filled from the host
pilot before the first counted load, as section 9 states: each cell's measured token
counts, and the load event's members in the expected-differ list. The commit that fills
them is the record of when the prediction was made, and nothing changes after the first
counted load. A change found to be needed after it is a new dated file beside this one,
and the result note reads against both.

The operator's scope for this measurement: first "the simple reproducibility of the
model in the VM/Appliance", with a simple system prompt and about twenty prompts of
varied input and output lengths, verified reproducible on the VM and compared with the
host. The first agent's STE conversion work is built later on a clone of the VM this
measurement proves, and is not part of it.

## 1. Question

The model, its capacities and sampler, the prompt in force, the asks and their order,
both GPUs and the driver release are held. Does every turn emit the same token ids,
surprisals and entropies across separate loads of the agent, across cold boots of the
appliance, and between the host and the appliance? Does this hold across a range of
prompt lengths up to the gate's line bound, and across output lengths up to the
declaration's cap?

## 2. Prediction and falsifier

**Prediction.** Every counted load is identical to every other on every turn, to float
exactness, in:

- the rendered request, with the sampling settings and the generation seed;
- the input token ids;
- the output token ids;
- the per-token surprisals and entropies;
- the emission and the finish;
- resident and capacity.

**Falsifier.** Any of the following, between any two counted loads:

- a difference in any of the fields above on any turn;
- one load has a turn that the other lacks.

## 3. What runs, and what it is not

- **The model**: Granite 4.2 30B at bf16, GGUF, sha256
  `082f9db8c5bf3fc25af2bf6475526b784d641f6cd453141d5319c5d2aaf1588b`. No adapter.
- **The appliance**: release 3.0, built at WeaverAgent
  `be56d8c281a0debe304adebe57d3a59159531a40` (tag `appliance/3.0`) with WeaverTools
  `c64918b6`, sealed file sha256
  `68c570cc84833b746f839df4999919d7f8808d6190550df811891170bb69ed23`. Each cold boot
  runs a copy of that file, unchanged.
- **The agent**: one, laid by `deploy/create-agent.sh` at that commit with
  `--root-operator --artifact <the model> --devices 0,1 --turn-context independent`,
  which writes:
  - the sqlite store, `binding-kind = "serving"`, `turn-context = "independent"`;
  - `devices = [0, 1]`, `context-capacity = 32768`, `max-tokens-per-turn = 4096`;
  - the sampler: seed 451234785645, temperature 0.7, top-k 40, top-p 0.95,
    repetition-penalty 1.1, repetition-window 64; surprisals elected.

  On the host the agent is laid by the same script at the same commit, with the same
  options apart from the operator and the model's path. Each side's declaration is
  recorded by sha256, and the two are compared apart from their paths and the access
  rule's `allowed-uids` (section 7).
- **The prompt in force** is the worker's compiled system prompt at `be56d8c2`:
  "You are a careful assistant. Answer from what you know, say plainly when you do not
  know, and keep answers as short as the question allows."
  - No seeding line is sent at any load, on either side.
  - Under independent turns with nothing seated, the worker composes each turn from the
    compiled prompt and the ask alone, with no continuity line and no re-entry, and asks
    the store for nothing. The store keeps its records, but it cannot reach a request.
  - The operator ruled for the compiled prompt in this measurement.
- **The turn path** is the appliance's own: each ask goes in through the agent's gate
  as the gate-con account, with `deploy/turn.py` (sha256
  `2b54f21dab8dd826f398758846312ce98c8a51de3cbe97abe9a445002a4fc59e`), and the answer
  comes back on the same connection. The record of each turn is the agent's trace, which
  the relay writes to its sink outside the agent's reach.
- **Not measured**: the STE adapter, the STE prompt, state carried into a turn, and the
  quality of the answers.

## 4. The cells

Twenty-four asks, each a cell: six prompt lengths by four output lengths, sent in one
pinned order in every load.

**Prompt length, by the excerpt's own token count:**

| Row | Target | Source |
| --- | --- | --- |
| P1 | 32 | the smoke kit |
| P2 | 256 | the smoke kit |
| P3 | 1024 | the smoke kit |
| P4 | 4096 | the twenty kit, lines 61 onward |
| P5 | 8192 | the twenty kit, lines 61 onward |
| P6 | the gate's line bound | the twenty kit, lines 61 onward |

- **The sources** are the conversion kits the first agent's adapter work was cut from,
  masked text from agent trajectories:
  - the smoke kit of 2026-11-13, 60 segments, sha256
    `ca668d0b296d5bf54086756039c77888afc2bb2ac3f93d3d7ca014977ef235fe`;
  - the twenty kit it was cut from, 664 segments, sha256
    `8777a81629f7a4844a8d6265c6462073007e35b7305252e05a29b379b3abc451`, whose first 60
    lines are the smoke kit byte for byte.
- **Why two sources.** The smoke kit holds about 3,400 tokens, too few for P4 to P6.
  Repeating it would make the long rows a repeated context, and a result could then rest
  on patterns that distinct text would not give. So P4 to P6 take the twenty kit's lines
  61 onward, 604 segments from 18 other trajectories, and no text repeats within a row.
- **The excerpts.** Each source's segments are joined in file order by a blank line.
  Each row's excerpt is a prefix of its source's stream, cut at the last sentence end at
  or before its target: P1 lies within P2 within P3, and P4 within P5 within P6. The
  target is reached through 4.01 characters a token, measured on this text with the
  model's tokenizer in an earlier run's traces. The exact counts are read from the
  pilot's trace (section 9).
- **P6** is the largest cut whose request line, with the longest instruction, is at most
  49,041 octets, the gate's line bound, encoded as `turn.py` encodes it.

**Output length, by the instruction that ends each ask:**

| Column | Output | Instruction |
| --- | --- | --- |
| O1 | one word | "In one word, what is the main subject of this text? Answer with one word." |
| O2 | about 150 words | "Summarize this text in about 150 words." |
| O3 | about 700 words | "Explain this text in about 700 words." |
| O4 | the cap, 4096 tokens | "Write a detailed commentary on this text of at least 5000 words." |

O4 ends at the declaration's cap, finish `length`, the only exact column. An O4 cell
that finishes before the cap is compared like any other turn, since the prediction is
exact agreement wherever a turn stops. It is named in section 9 and in the result note
with its counts, and the question of output at the cap is then answered by the O4 cells
that reach it. O1 to O3 are bands, read from the pilot (section 9).

**Each ask** is `Here is a text.`, a blank line, the excerpt between a `<<<TEXT` line
and a `TEXT>>>` line, a blank line and the column's instruction, with no trailing
newline. The 24 asks and their order are pinned by `SHA256SUMS`, sha256
`bdc8138c0f08f5c860aa360904efd4eaf27f0f4080feba9508a4c7495f92583a`, made by
`make-asks.py`, sha256
`4aac6ccdfd37cee6e7bd6458d6c9d425423e4189a6dab13913242c61d76663eb`. Both are carried in
the deposit. The order is P1-O1, P1-O2, and so on to P6-O4.

## 5. The loads, in the order they run

**A load** is one load of the agent, the 24 asks in the pinned order, and one unload.
The unload adds the worker's wind-down turn, compared like the others, so a load has 25
turns.

**A repeat is a whole load.** A turn's generation seed derives from the declared seed
and its turn key, `t-<n>`, its place in the load. Turn n of every load draws the same
seed, and the same ask sent twice inside one load draws two seeds. So loads are compared
turn by turn, and no ask is repeated inside a load.

| Load | Where |
| --- | --- |
| 1, 2 | the host |
| 3, 4 | the appliance, boot 1 |
| 5, 6 | the appliance, boot 2 |
| 7, 8 | the appliance, boot 3 |

- **The host first, then the appliance.** Both GPUs move together between them.
- **A boot** is a cold start of one appliance VM whose disks persist between boots, as
  an appliance's do. Boot 1 lays the agent.
- **The appliance VM**: q35 with UEFI, 8 virtual CPUs with a host-passthrough CPU, 96
  GiB, both GPUs under passthrough, no network interface. The model is on a read-only
  drive at `/mnt/weaver/models`, the agent's state on a drive at `/mnt/weaver/state`.
- **The pilot** runs on the host before load 1 and is not counted. Its first load runs
  the whole sequence once. Its second load sends the first ask alone, and its turn 1
  must equal the first load's (section 9).

## 6. What is held, and how each is read

- **Both GPUs**, paired by serial and by UUID on each side, not by bus address. Device 0
  and device 1 are the same physical card on both sides, and each side lists them in the
  same order.
- **The site's card condition**, applied and read back on each card before every load,
  again after the load answers idle and before any turn, and after the last turn. A load
  whose cards do not read back as applied after it is unloaded with no turn sent, and is
  not counted. A load whose cards read back as applied after it but not after its last
  turn has sent its turns under a condition that did not hold to the end. It is not
  counted either: its readings and its trace are kept, and one further load on the same
  side takes its place, on the same boot in the appliance. Its trace is still compared
  with load 1's, and that comparison is reported beside the counted ones, marked as
  outside the held conditions, so that no load is set aside without showing what it
  said. The site holds this condition for its own reasons; it is not predicted to change
  any compared field. In the appliance, `nvidia-persistenced`, which the image ships and
  does not enable, is started for each boot before the condition is applied. Whether a
  sealed image should enable it is #40.
- **The model**, by sha256 before each session on the host and at each boot in the
  appliance.
- **The asks**, by `SHA256SUMS` at each boot and before each host session.
- **The declaration**, by sha256 at each load.
- **The driver release**, 620.70.06 on both sides.

## 7. The comparison

The comparison script of issue 28 reads the traces alone (sha256
`24eed9379b851e399e7b87e200f1cd6baddfb904d4dd542bb7ce1d081c760ce0`, carried in the
deposit). It matches turns by turn key and compares each field of section 2 exactly, the
surprisals and entropies each as its JSON text, so at f32 bits.

**Pairs:** every counted load against load 1. Identity carries through, so that covers
repeats, boots and sides. A difference is located by the pairs within one boot, between
boots and within the host.

**Run-level fields ruled before the run to differ by construction:**

- Never compared: the run identifier and every time and wall clock.
- Between any two loads: the load event's `reset`, absent on an agent's first load and
  naming the prior run on a later one.
- Between the host and the appliance:
  - in the load event, `cause` (the operator: root in the appliance, the operator's own
    account on the host), `boundary` (a digest over the agent's accounts), `declaration`
    (its digest, through the model's and the territory's paths and the access rule's
    `allowed-uids`, the gate connector's uid, which each box assigns when it lays that
    account), `library_path` and `stack` (a host build at `be56d8c2` against the
    appliance's own build of it);
  - the CPU's memory total in the device facts;
  - the engine libraries' paths and digests.

Further load-event members found to differ by construction in the pilot are added here
before the first counted load (section 9). Any other run-level difference is reported as
a finding.

## 8. Report

As counts, not rates, for each pair:

- turns compared and turns identical;
- the first differing turn, its field and its first differing token, if there is one;
- whether the turn sequences match.

Each cell's input and output token counts, and its finish, as the traces give them. The
result note names every held field that turned out to differ.

## 9. Filled from the pilot, before the first counted load

- **Each cell's measured counts:** the excerpt's input tokens, and the output tokens and
  finish. A cell outside its band, O1 to O3, is named here with its counts, never
  dropped in silence.
- **Turn 1 of the pilot's second load** against its first load's.
- **The load event's members** that differ by construction between the host and the
  appliance, if the pilot shows any not listed in section 7. The appliance's side is
  read from the cardless rehearsal at the same commit: its loads refuse at the device,
  but each writes a whole load event first. Nothing from the rehearsal is counted. The
  device facts come only with a card, so the one of them that differs by construction,
  the CPU's memory total, is ruled in section 7 without an appliance reading.

## 10. Where the evidence goes

A deposit under `weaver-testing/`, holding each load's trace as copied from the sink
with its sha256, the asks and their builder, the declarations and the comparison's
output. This repository carries the result note and the scripts behind its figures.
