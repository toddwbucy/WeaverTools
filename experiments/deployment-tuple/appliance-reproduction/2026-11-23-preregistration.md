# Pre-registration: the first agent's model reproduces on presented VMs

Drafted 2026-11-23, before any run of these cells. It repeats the measurement of
2026-11-18, whose pre-registration is beside this file, under conditions that the
operator ruled on 2026-11-23. Entries marked **[to fill]** are filled before the first
counted load of the run they belong to, and the commit that fills them is the record of
when the prediction was made. Nothing changes after a run's first counted load. A change
found to be needed after it is a new dated file beside this one, and the result note
reads against both.

The operator's scope, ruled 2026-11-23:

- the measurement of 2026-11-18 is run again on virtual machines that the operator
  presents;
- it is run twice: first the model with no adapter, then the model with its adapter;
- the sides are separate loads across cold boots of one VM, and then the same binaries
  with the same two GPUs in a second VM.

## 1. Question

The model, its capacities and sampler, the prompt in force, the asks and their order,
both GPUs, the driver release and the installed stack are held. Does every turn emit the
same token ids, surprisals and entropies:

- across separate loads of the agent;
- across cold boots of the VM that holds the cards;
- between that VM and a second VM that is given the same two cards and the same
  binaries?

Does this hold across the prompt lengths and output lengths of the 24 cells, and does it
hold again with the adapter loaded?

## 2. Prediction and falsifier

**Prediction.** Within a run, every counted load is identical to every other on every
turn, to float exactness, in the seven fields of 2026-11-18:

- the rendered request, with the sampling settings and the generation seed;
- the input token ids;
- the output token ids;
- the per-token surprisals and entropies;
- the emission and the finish;
- resident and capacity.

**A second prediction, new here.** Between the two VMs the load event's `stack` and
`library_path` and the engine libraries' paths and digests are equal, because one carried
set is installed on both at the same prefix. On 2026-11-18 these differed by
construction.

**Falsifier.** Any of the following, between any two counted loads of one run:

- a difference in any of the seven fields on any turn;
- one load has a turn that the other lacks;
- a run-level difference outside the list of section 7.

Run A and run B are not compared with each other. They differ by design.

## 3. What runs, and what it is not

- **The two VMs.** The card VM is a virtual machine that the operator presents with both
  GPUs under a working driver. The build VM is the virtual machine that compiles the
  stack. For loads 7 and 8 the operator gives the build VM the same two GPUs. The suite
  makes, starts, stops and changes no machine.
- **The model**: Granite 4.2 30B at bf16, GGUF, sha256
  `082f9db8c5bf3fc25af2bf6475526b784d641f6cd453141d5319c5d2aaf1588b`.
- **Run A**: the model with no adapter.
- **Run B**: the same model with the first agent's adapter, GGUF, sha256
  `3e668e78aa4ec186d7e3b67a97775d53c819ca60c29ecca452b455078638a212`, at scale 1.0 (the
  operator's ruling of 2026-11-23), over the base's blake3
  `41b2efe4743c896072aeebf0b6a24c73afb40642b5aa93d9e31f252daef048fc`. Every load of run B
  records the adapter by digest in its first measurement. A load that does not is not a
  load of run B.
- **The stack**: built once, on the build VM, from WeaverAgent commit **[to fill]**, and
  carried as one set, manifest sha256 **[to fill]**. The same set is installed on both
  VMs at the same prefix. Neither install compiles anything.
- **The agents**: one for each run, laid by `deploy/create-agent.sh` at that commit, run
  by the operator's account with sudo, with
  `--artifact <the model> --devices 0,1 --turn-context independent`. Run B adds the four
  adapter options. The script writes what it wrote on 2026-11-18:
  - the sqlite store, `binding-kind = "serving"`, `turn-context = "independent"`;
  - `devices = [0, 1]`, `context-capacity = 32768`, `max-tokens-per-turn = 4096`;
  - the sampler: seed 451234785645, temperature 0.7, top-k 40, top-p 0.95,
    repetition-penalty 1.1, repetition-window 64; surprisals elected.

  Each agent is laid once on each VM, at the same paths. Each declaration is recorded by
  sha256.
- **The prompt in force** is the worker's compiled system prompt, in both runs, on the
  operator's ruling of 2026-11-23. No seeding line is sent at any load. The adapter's own
  prompt belongs to later work.
- **The turn path**: each ask goes in through the agent's gate as the gate connector's
  account, with `deploy/turn.py` (sha256 **[to fill]**; it was
  `2b54f21dab8dd826f398758846312ce98c8a51de3cbe97abe9a445002a4fc59e` on 2026-11-18). The
  record of each turn is the agent's trace, which the relay writes to its sink outside
  the agent's reach.
- **Not measured**: the adapter's prompt, state carried into a turn, the working loop, and
  the quality of the answers.

**What differs from 2026-11-18, by the operator's rulings:**

| Then | Now |
| --- | --- |
| A sealed image, compiled inside itself | VMs the operator presents; binaries built on the build VM and carried |
| Arch guest | Fedora guest |
| Root at a serial console, `--root-operator` | The operator's account with sudo, over SSH |
| No network interface | A network interface |
| The model on a read-only drive, filled beforehand | The suite copies the model into storage attached to the VM |
| The kit applied the site's card condition | The site applies it; the kit reads it back |
| A bare-metal host as the second side | A second VM as the second side |

## 4. The cells

Unchanged. The 24 asks of 2026-11-18, six prompt lengths by four output lengths, in the
pinned order, held by `SHA256SUMS`, sha256
`bdc8138c0f08f5c860aa360904efd4eaf27f0f4080feba9508a4c7495f92583a`. Section 4 of the file
of 2026-11-18 describes them and is not repeated here.

## 5. The loads, in the order they run

**A load** is one load of the agent, the 24 asks in order, and one unload. The unload
adds the worker's wind-down turn, so a load has 25 turns. **A repeat is a whole load**,
for the reason given on 2026-11-18: turn n of every load draws the same seed.

Each run has the same eight loads:

| Load | Where |
| --- | --- |
| 1, 2 | the card VM, boot 1 |
| 3, 4 | the card VM, boot 2 |
| 5, 6 | the card VM, boot 3 |
| 7, 8 | the build VM, holding the same two cards |

- **Run A first, then run B.** Run B's first counted load starts after run A's six loads
  on the card VM have been compared.
- **A boot** is a cold start of the VM: it is stopped and started, not restarted from
  inside. Each boot, and the move of the cards to the build VM, is the operator's action,
  taken when the kit reports a boot's loads done.
- **One move of the cards.** Loads 7 and 8 of both runs are taken in one stay of the
  cards in the build VM, after both runs' loads on the card VM.
- **The pilot** runs on the card VM before load 1 of each run and is not counted. Its
  first load runs the whole sequence once. Its second load sends the first ask alone, and
  its turn 1 must equal the first load's.

## 6. What is held, and how each is read

- **Both GPUs**, paired by serial and by UUID on each VM, not by bus address. Device 0 and
  device 1 are the same physical card on both VMs.
- **The site's card condition**, read back on each card before every load, again after
  the load answers idle and before any turn, and after the last turn. The kit applies
  nothing. A load whose cards do not read back as held is not counted, by the rules of
  2026-11-18: its trace is kept, compared and reported beside the counted ones, and one
  further load takes its place.
- **The model**, and in run B the adapter, by sha256 at each boot.
- **The asks**, by `SHA256SUMS` at each boot.
- **The declaration**, by sha256 at each load.
- **The installed stack**: the carried set by its manifest, and the install tuple read on
  each VM. Every member and library digest is equal on the two VMs.
- **The CUDA runtime files** that `weaver-spu` resolves, by sha256, equal on the two VMs.
- **Every other object `weaver-spu` maps**, the C and C++ runtime among them, by sha256,
  read by the kit from the process's own map at each load, as issue 28 asks. They are the
  VM's and not the carried set's. A difference between the two VMs is reported before
  load 7 is taken.
- **The operator's account**, the same name and uid on both VMs **[to fill]**.
- **The driver release**, the same on both VMs **[to fill]**.
- **The guest's kernel release**, recorded on each VM. A difference is reported.

## 7. The comparison

The comparison script of 2026-11-18, unchanged (sha256
`24eed9379b851e399e7b87e200f1cd6baddfb904d4dd542bb7ce1d081c760ce0`). It reads the traces
alone, matches turns by turn key and compares each of the seven fields exactly.

**Pairs:** within each run, every counted load against load 1. A difference is located
by the pairs within one boot, between boots and within the build VM.

**Run-level fields ruled before the run to differ by construction:**

- Never compared: the run identifier, every time and wall clock, and the load event's
  `lineage`.
- Between any two loads: the load event's `reset`, as on 2026-11-18.
- Between the two VMs:
  - in the load event, `declaration`, through the access rule's `allowed-uids` alone,
    which each VM assigns when it lays the gate connector's account;
  - the CPU's memory total in the device facts, where the two VMs differ in memory.

`cause`, `stack`, `library_path` and the engine libraries' paths and digests were on this
list on 2026-11-18 and are not on it now. Members found in the pilot to differ by
construction are added before the first counted load **[to fill]**. Any other run-level
difference is a finding.

**One comparison outside the score.** Load 1 of run A is compared with the bare-metal
host's load 1 of 2026-11-18 (trace sha256
`8c3b2b3de65219b3922d23fb3afebe4dfcedeb1e5cb81f9f9dc9959ede96e594`). That load ran an
earlier commit, built by another compiler on another distribution. No prediction is
registered for it. It is reported as counts, whatever it shows.

## 8. Report

For each run, as counts and not rates, for each pair: turns compared and turns identical;
the first differing turn, its field and its first differing token, if there is one;
whether the turn sequences match. Each cell's input and output token counts and finish.
Every held field that turned out to differ.

## 9. Filled from the pilots, before each run's first counted load

- **[to fill]** each cell's measured token counts in run A. They are predicted equal to
  the table of 2026-11-18, and a difference is reported before load 1.
- **[to fill]** each cell's measured token counts in run B.
- **[to fill]** the load event's members on each VM. The build VM's side is read from a
  rehearsal with no card, if its install allows one, as the appliance's was on
  2026-11-18. Otherwise section 7 rules its members without a reading.
- **[to fill]** the kit's digests.

## 10. Where the evidence goes

A deposit holding each load's trace as copied from the sink with its sha256, the asks and
their builder, the declarations, the install tuples and the comparison's output. Its
place is named before the first counted load **[to fill]**. This repository carries the
result note and the scripts behind its figures.
