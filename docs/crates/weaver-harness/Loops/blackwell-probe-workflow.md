# Blackwell Probe Workflow

**Status:** MERGED. The workflow document for the Weaver probe's Blackwell cell,
filed under the harness's `Loops/` container beside the diagnostic replay loop it
drives. It argues no edges of its own: every seam it walks is declared in the crate
charters and contracts, and the probe is operator tooling rather than a crate, so it
declares no assertion record. Each file under `deploy/tb/` carries a `cites:` line
naming this document's sections, first after its shebang, and not a `conforms:` line,
since a citation naming no node would dangle if the census ever read that tree. The
instruments of section 9 are what hold it.

**Document ID:** `blackwell-probe-workflow`
**Parent:** `weaver-harness-PRD`
**Editorial:** Per the Working Rules.
**Landing PR:** #685

---

## 0. What this document is

The specification the probe's code answers to, written from the code as it stood
after sixteen review passes on pull request #683 and from the registration of the
cell on issues #511 and #679. It exists because the corpus's first rule of phase
three is that no code lands without a ratified document, and the probe landed as a
held preparation with only an issue for authority. This document closes that gap by
stating what the code does and why, in the order the operator meets it, so a reader
of the code can find each refusal's reason here and a reader of this document can
find each claim's instrument in the suite.

Level discipline, stated once. What the probe measures and why is #511's and #679's
and is restated in sections 1 through 3 only as far as the code enforces it. How the
seams it drives behave is the charters' and contracts' and is cited rather than
repeated. What this document owns is the workflow: the tuple as the code holds it,
the arms as the code schedules them, the order of operations, the privilege boundary,
the identity verdict, the assessment, and the refusals.

## 1. What the probe is

**One card, one commit, two stacks, three arms.** The cell runs the Weaver probe of
#511 on this box's RTX PRO 5000 Blackwell under one source commit, `e69916a`, so
the device arm is not also a build arm. `B1` is olympus's `e69916a` stack copied by
bytes, the kernel held as bytes, and is the device cell. `B2` is `e69916a` built on
this box under its own toolkit and CUDA user-space libraries. Whether the two stacks'
executable sections are identical decides whether the kernel arm runs at all, per
section 6.

The three arms are `TB0`, the control, which asks whether the card reproduces under
itself, `TB-d`, the device cell, which re-feeds the Ampere and Ada Q8_0 records of
#511 through `B1`, and `TB-k`, the kernel arm, which re-feeds `B1`'s own records
through `B2`. Every claim, prediction and falsifier is registered before any arm runs,
and section 3 carries them as the code enforces them.

**The driver confound is carried, not hidden.** The olympus deposits were taken on
driver 610.57.04 under CUDA 13.3 and this box runs 615.71.09, so a `B1` reading
against those deposits varies device and driver together until the olympus seat
re-verifies at the same tuple. A `B1` divergence is never attributed to the card
alone, and the report says so.

**It runs the loops the corpus already has.** A free run is a gate turn through
`basic_loop.py` under a serving binding. A re-feed is the diagnostic replay loop,
per `diagnostic-replay-loop`, driven through `weaver-analysis`'s `derive` and
`preload` against a diagnostic load. The probe adds no loop of its own, and the
admin, gate, harness, analysis and state members it drives are the installed
`e69916a` binaries, not the tree at head. That is why a finding about the head's
admin does not reach this workflow unless the stacks are re-staged from head, which
is a ruling on #679 and not this document's.

## 2. The tuple, held

The tuple is fixed in the plan and the coordinator refuses a plan whose tuple is not
exactly this one, field for field, under the `tuple` refusal of `validate_plan`:

| Field | Value |
| --- | --- |
| artifact | `/opt/weaver/models/Qwen3-8B-Q8_0.gguf` |
| weights | sha256 `0cfbf745760f07a76ddeb358dd025a27f2e11d1ca9c9a4169a373d52990fe86e` |
| devices | `[0]` |
| context capacity | 12288 |
| max tokens per turn | 8192 |
| field depth | 200 |
| surprisal | elected |
| residual readout | not elected |
| identity | the `identity` string of olympus's `config-q8.json`, verbatim |
| seeds | 451234785645, 1156316220, 7, 1000003, 123456789, 987654321, 2718281828, 3141592653 |
| runs per seed | 2 |
| driver | 615.71.09 |

**Held means every load carries it and every reading is checked against it.** The
payload renders each job's declaration from the tuple, per `declaration`, with the
seed the job's own, the loop `basic_loop.py` from the job's stack, the operator's uid
as the gate's one allowed dialer, and a fresh trace sink per job. A re-feed's derived
declaration must name the served model and no other artifact and must carry the
tuple's context capacity, token cap and identity, and the seed its source free run
declared where the source is local or a seed the tuple admits where the source is an
external trace, per `holds_tuple`, under the `derived-artifact` and `derived-tuple`
refusals. Every
measured record must carry the tuple's weights hash and the job's seed, under
`weights-held` and `seed-held`, and a re-feed must carry them on both sides of the
comparison, under `source-weights-held`, `source-seed-held`, `replay-weights-held`
and `replay-seed-held`, because a comparison across two tuples is another experiment
under this one's name.

**The elected readings are derived from the tuple and not from a list.** A record is
sufficient when it carries one output token or more, a field entry per token at the
tuple's depth, and one value per token in every per-token series the tuple elects,
entropies always and surprisals because the tuple elects them, per `series` and
`enough`. A true election with no record key raises rather than going unread, so an
election added to the tuple cannot be forgotten by the sufficiency check and the
comparison both. `weaver-spu-Spec` section 6's rule, carrying charter section 13.12,
stands beneath this: elected surprisals render, and disagreement between elected
readings is a defect.

## 3. The arms, and the claims the code enforces

**The schedule is fixed and validated whole.** The plan holds exactly the arms
`TB0`, `TB-d` and `TB-k` in that order, every job identity unique and plain, every
job free or a re-feed on `B1` or `B2`. The control holds two free runs per seed, all
on `B1`, and one own re-feed per free run. A re-feed names a source that precedes it
in the schedule or a reviewed external trace with its run. The device arm holds only
re-feeds on `B1` from reviewed external traces, at least one from each of the
`ampere` and `ada` cells, each measuring a distinct trace and run, with no trace
shared between the cells, because a repeated selection or a shared trace would
complete the arm without measuring one of its cells. The kernel arm holds every `B1`
record as a re-feed on `B2`, or is empty only on the identity evidence of section 6.
The refusals are `arm-order`, `job-identities`, `job-types`, `control-schedule`,
`own-refeeds`, `source-order`, `device-sources`, `device-traces`,
`device-selections-distinct` and `kernel-schedule`.

**TB0, the control.** The claim: holding the seed reproduces on this card, and
changing it moves the draw and not the distribution. The prediction the code checks:
every same-seed pair is exact and emits the same text, every own re-feed is certified
and exact at every position, and every pair of distinct seeds, taken one run per seed
in the tuple's order, parts within the first twenty-four tokens. The falsifier stops
the arm where it
falls: after the operator has unloaded a free run whose earlier same-seed run it does
not match, under `pair-falsifier`, or an own re-feed that is not certified and exact,
under `own-refeed-falsifier`, and at the arm's assessment under `control-falsifier`
and `changed-seed-prediction`. A card that does not reproduce under itself cannot be
read against another, so the halt is the finding.

**TB-d, the device cell.** The claim: Blackwell moves the distribution from position
zero relative to Ampere and to Ada, as Ada moved it against Ampere in #511. The
prediction: each record re-fed through `B1` has per-position readings that differ
from the recorded ones, and the divergence is reported in the historical instrument's
input-plus-output coordinate with the output ordinal beside it. The falsifier: a
re-feed exact to the bit at every position. The arm carries the driver confound of
section 1 as a qualifier on every reading.

**TB-k, the kernel arm.** If every executable section of `B1` and `B2` is identical
by section 6's verdict, the arm is recorded as the kernel held by executable identity
across the two toolchains and nothing runs. If sections differ, the claim is that the
differing kernels move the distribution on one card, the prediction is that a `B1`
record re-fed through `B2` departs at some position, and the falsifier is every
record re-feeding exact. `B2` also differs in its CUDA user-space libraries, and the
reading carries that as part of the kernel stack.

## 4. The order of operations

**One command, one state file, three seats.** The operator runs `next` and nothing
else, never under sudo, per `Order.operator`, and the payload refuses a `next` whose
invoking uid is not the plan's recorded operator, under its `operator` guard. The
state file `tb-state.json` carries the cursor over a schedule the plan determines:
`provision`, then for each arm `start`, and for each job `load`, `measure`, `unload`
and `settle`, then `finish`, and after the last arm `report` and `review`. Each step
names its seat. The operator owns `provision`, `load` and `unload`. The coding seat
owns `start`, `measure`, `settle`, `finish` and `report`. The review seat owns
`review`, which is its own edit of the state file under the coordinator's lock and
never a command of this program, as approval is. A step out of order, by the wrong
seat, or already recorded is refused under `step-order`, `seat` and `not-repeated`,
and every step first re-verifies every earlier receipt against its recorded digest
under `prior-success` and `prior-evidence`.

**Approval gates every step and is not this program's to grant.** `next` waits while
the hold stands or the review is not `PASS`. Once lifted, every step verifies that
the review names its seat and reference, that its artifact map covers the plan and
the nine staged files, the eight scripts and the suite, that every artifact still
hashes as recorded, that the
plan snapshot parses from the bytes that matched its digest, that the plan's file map
is covered by the artifact map and agrees with it, and that no halt is recorded,
under `hold`, `review`, `approval-coverage`, `artifact-hashes`, `plan-snapshot`,
`manifest-coverage`, `manifest-hashes` and `halt`. Staging creates the state held and
unapproved and refuses to overwrite an existing state, so approval is never
manufactured.

**The driver blocks, and the coordinator holds the lock only while it decides.** An
arm runs in one driver invocation that takes a lease bound to its process start time,
per `live`, and the coding-seat steps are refused to any process but the lease
holder under `driver-owner`. Each wait is bounded at four hours, verifies the driver's
own live lease, and runs the full approval verification only when the state bytes
have moved and always before returning, so an idle driver does not hash the deposit
every second under the lock the operator's `next` needs, under `wait-owner`,
`wait-order` and `wait-deadline`. A falsified control is judged after the operator's
unload and before the next load becomes due, so the halt lands with the card
unloaded.

**A halt is evidence.** A refusal records its reason and a transcript in the state and
never advances the cursor. A timeout, a dead driver, a refused payload or an
insufficient measurement leaves the cursor where it was. No refusal is bypassed by
editing the state, and partial resources left by an interrupted step are evidence the
review seat rules on with a bounded recovery step, since the payload holds no general
root shell and no arbitrary command.

## 5. The privilege boundary, and custody

**Root receives the reviewed bytes and never a path.** The coordinator reads
`tb_payload.py` once, checks the bytes against the digest the review seat recorded,
and runs `sudo /usr/bin/python3 -I -c <those bytes> <plan> <digest> <step>` with
standard input closed and the transcript captured, per `payload`. The digest handed
to root is the one recorded at approval, never one recomputed at call time, and the
payload parses the one plan snapshot whose bytes matched it, under `plan-hash`, before
anything else. The payload runs in isolated mode and imports nothing from the working
directory. The reason is stated where it binds: a file the operator's uid can rename,
any process of that uid can swap between a check and sudo's open, and that uid need
not hold the sudo credential. A sudoers allowlist for the payload therefore matches
this form, since there is no payload file to name. The payload refuses to run as
anything but root, refuses a plan not naming the isolated root and the `bravo` agent,
and refuses an invoking uid that is not the plan's recorded operator, under
`root-payload`, `fixed-root-agent` and `operator`. The coordinator and the driver
refuse to run as root, under `operator-not-root` and `driver-not-root`.

**Approval is not yet in root custody, and the hold lift owes one privileged step.**
The review seat's `PASS` and the artifact digests live in `tb-state.json`, which the
operator's uid owns, so the adversary of the paragraph above could rewrite what is
approved before `next` hands it to root. That gap was found by the security review on
#683 and is recorded on #679: before the hold lifts, a privileged step installs the
reviewed payload and the approval digests root-owned, verified against what the
review seat published, and `next` hands sudo the root-owned copy. Until that step
lands, this workflow is held.

**Nothing privileged reads an operator-owned input twice.** A check on a path
followed by a second read of the path binds nothing, so every operator-owned input
is read once into a root-owned private file whose bytes are verified against the
recorded digest, and only that file is used afterwards: the served model and a
re-feed's external source trace, each under `snapshot-hash`, every file the plan
names having first been checked against its manifest digest under
`source-file-hash`. The re-feed
then cuts the plan's source run from that snapshot into a second private file, since
`derive` refuses a record holding two runs, and `derive` and `preload` both receive
that one file, under `source-run-selected`. The driver likewise hashes and compiles
one read of each reader and hashes and parses one read of each source trace, under
`reader-approved` and `source-trace`.

**Everything root serves from is locked when made and verified before it is
trusted.** Provisioning refuses a link anywhere in a source stack before its first
write, under `stack-no-symlinks`, and requires every stack file to be in the plan's
file map, under `stack-file-coverage`. It copies each stack by bytes, then locks the
installed tree with the root included, since the copy preserves the source root's
mode and a walk of descendants never reaches the root, and it locks the isolated root
and every directory it makes beneath it as it makes them, since a directory's mode is
masked by the umask. The installed copy must then hold exactly the reviewed files, no
link, every entry owned by the payload's user with no group or world write, under
`installed-no-symlinks`, `installed-stack-custody`, `installed-stack-coverage` and
`installed-stack-hash`, and every later step checks the served directories again
under `served-directory-custody`. The served model is a regular file with one name,
owned by the payload's user, not group or world writable, in a directory held the
same way, whether it was found or freshly installed, under `existing-model-custody`,
`new-model-custody` and `installed-model-custody`, and its bytes are the tuple's,
under `model-source`, `existing-model` and `installed-model`.

**The installation is isolated and refuses to adopt.** Provisioning uses only
`/var/lib/weaver-tb`, its own two admin roots and the `weaver-bravo` account and
group, refuses an existing root or account rather than adopting custody, under
`fresh-install-root`, installs the model at the historical absolute artifact path or
checks an identical existing file and never overwrites a different one, and changes
nothing of `m1`, `karl`, their declarations or `/etc/weaver/admin`. The sinks
directory is root-owned, its group the operator's own, mode `2750`, so the operator
reads evidence and no member writes outside its own trace.

**Every load stands on an interlock.** A `bravo` load refuses a non-inactive `m1`
unit, an unreadable unit status, a remaining `m1` coordination door, or any process
under `m1`'s uid, under `m1-inactive`, `m1-state-readable`, `m1-no-door` and
`m1-no-process`. It checks the device and driver tuple, that the CUDA engine library
`libggml-cuda.so` resolves every dependency, and that the CUDA runtime resolves from
the stack's own library directory
and nowhere else, under `gpu-tuple`, `resolved-libraries` and `cuda-local`. A
diagnostic load waits for the preload door to stand and for the loader to answer,
under `preload-door` and `diagnostic-load`, and an admin answer is read for its
content and never for its exit status alone, under `admin-answer`.

## 6. Executable identity

**The inventory covers every file a load can execute from.** The served
directories of a stack are one constant, `STACK_ROOTS`, held by the payload and
imported by the inventory: `bin`, `engine-lib` and `cuda-lib`, the last two being
what the payload puts on the library path. The inventory walks every file beneath
them, records ELF files by header, program headers and every section with its hash,
records every other file by whole-file hash, and refuses a link, since a stack is
copied by bytes and a link would vouch for bytes nobody read. It also records every
extracted cubin and PTX member by size and hash, with the cubins' sections. A test
derives the served set from the payload's own environment and configured binaries and
pins it to the constant, so a directory served from outside the inventoried roots is
a test failure and not silent drift.

**The comparison takes `B1` then `B2` and nothing else.** It refuses any other pair,
including one inventory twice, and any inventory with no hosts, no cubins or no PTX,
since an empty scope makes every verdict vacuous. Its report names the two inventories
it read by path and digest, and it records per host file whether the file is
byte-identical and whether each recorded section matches, and per CUDA architecture
group how many members match by container and by code section and which members
changed or went missing, so a container that differs while its code sections match
is told apart from a kernel that changed.

**The verdict is conservative and bound to the stacks it judged.** Identity holds
only when every recorded section, header and CUDA member matches and every host file
is byte-identical. The entry point and segment permissions live outside every
section, so equal sections do not make equal hosts, and an identity verdict cannot be
inferred from equal file counts or `.text` alone. An emptied kernel arm names the
report that empties it, hashed in the plan's file map, and approval reads that report
once against its digest, requires it to be the comparison's verdict on `B1` against
`B2` and true, requires each inventory it names to be a reviewed artifact at the
digest the report records, and requires the inventoried file hashes to equal the
approved hashes of every file under that stack exactly, under `identity-evidence`,
`identity-inputs` and `identity-binds-stacks`. A report over stacks that have since
changed, or over an inventory that missed a reviewed file, empties nothing.

## 7. Measurement and assessment

**A free run is one gate turn, one measurement, one fresh trace.** The driver dials
the gate with the essay prompt, requires an answered close naming a run, waits for the
turn's close in the trace, and requires exactly one measurement in that run, under
`gate-answer` and `single-turn`. The record is extracted by the historical readers,
compiled from the hashed bytes the manifest approved and never from cached bytecode,
and must be sufficient per section 2. Each trace has a fresh directory, so a retry
cannot truncate or relabel earlier evidence.

**A re-feed is one completed replay against one verified source.** The source record
is the earlier free run's record, read by path from the deposit, which is design item
B on #679 and is named in section 10 as not yet held, or the reviewed external trace's
selected run, read once, hashed and parsed, with exactly one measurement, under
`source-trace` and `source-measurement`. The replay must complete, certified or
diverged, and carry exactly one measurement, under `replay-completed`,
`single-replay` and `replay-measurement`. Only the control's own re-feeds must be
certified, per section 3, since a diverged replay is what the device and kernel arms
predict. Exactness is the
output tokens equal, every elected per-token series equal to the bit, and the field
equal at every position, per `exact`, and the reading is the historical instrument's
divergence coordinate with its ordinal.

**Assessment is per arm and the falsifiers halt before it.** The control's report
carries every same-seed pair with its equality and reading, every own re-feed with its
exactness and certification, the first difference for every pair of distinct seeds
taken one run per seed in the tuple's order, and the two verdicts `control_passed`
and `changed_seed_prediction`. The device and kernel arms' reports carry each
re-feed's reading and, for the kernel arm, whether identity emptied it. Each arm's
result is written whole and recorded as that arm's `finish` receipt, and the report
step runs only after every earlier receipt, each `finish` included, stands with its
digest intact.

## 8. The refusals

Every guard is named, and its name is the reason in the halt. Three of the payload's
names arrived with the citations commit on #683, where they had raised as plain
errors: `no-bravo-account` for an existing `bravo` account, `known-step` for a verb
that is neither a load nor an unload, and `command-exit` for a privileged command
exiting nonzero, which prints the command's output to the transcript first. They are
grouped here by the module that raises them, and section 9 says how each is watched.
The
coordinator's: `schema`, `agent`, `isolated-root`, `tuple`, `rulings`, `arm-order`,
`job-identities`, `job-types`, `control-schedule`, `own-refeeds`, `source-order`,
`device-sources`, `device-traces`, `device-selections-distinct`, `kernel-schedule`,
`hold`, `review`, `approval-coverage`, `artifact-hashes`, `halt`, `plan-snapshot`,
`manifest-coverage`, `manifest-hashes`, `identity-evidence`, `identity-inputs`,
`identity-binds-stacks`, `cursor`, `prior-success`, `prior-evidence`, `step-order`,
`seat`, `not-repeated`, `driver-live`, `driver-owner`, `no-live-driver`,
`report-evidence`, `payload-hash`, `payload-exit`, `payload-receipt`, `wait-owner`,
`wait-order`, `wait-deadline` and `operator-not-root`. The payload's: `root-payload`,
`plan-hash`, `fixed-root-agent`, `operator`, `source-file-hash`, `job-found`,
`model-source`, `existing-model-custody`, `existing-model`, `stack-no-symlinks`,
`stack-libraries`, `stack-file-coverage`, `fresh-install-root`,
`no-symlink-destination`, `snapshot-hash`, `new-model-custody`,
`installed-no-symlinks`, `installed-stack-custody`, `installed-stack-coverage`,
`installed-stack-hash`, `installation-plan`, `installed-model-custody`,
`served-directory-custody`, `installed-model`, `m1-inactive`, `m1-state-readable`,
`m1-no-door`, `m1-no-process`, `gpu-tuple`, `resolved-libraries`, `cuda-local`,
`source-run-selected`, `derived-artifact`, `derived-tuple`, `preload-door`,
`diagnostic-load`, `admin-answer`, `no-bravo-account`, `known-step` and
`command-exit`. The driver's: `driver-not-root`,
`reader-approved`, `fresh-arm`, `gate-answer`, `single-turn`, `nonempty-measurement`,
`field-depth`, `seed-held`, `weights-held`, `source-trace`, `source-measurement`,
`source-weights-held`, `source-seed-held`, `replay-completed`, `single-replay`,
`replay-measurement`, `replay-weights-held`, `replay-seed-held`, `pair-count`,
`pair-falsifier`, `own-refeed-falsifier`, `control-falsifier`,
`changed-seed-prediction` and `report-path`. The inventory raises its refusals as
errors on the command line, since it runs before any plan exists.

**The rulings are refusals too.** A plan whose `hold_lifted`, `cuda_provenance` or
`control_count` ruling is empty is refused under `rulings`, and each names the URL
of the decision rather than a placeholder. The `control_count` ruling accepts eight
Q8_0 pairs and sixteen own re-feeds for this schedule, and a larger schedule changes
the validator and its tests in a reviewed rework first.

## 9. What is enforced, and by which instrument

**Two instruments, run unprivileged, and every guard is watched by both.** The suite
`test_tb.py` runs against temporary files, a temporary Unix socket and stubbed admin,
gate and device calls, and makes no change to any installed stack. The mutation run
`perturb.py` copies the scripts to a temporary directory, refuses unless the
unmodified suite passes there, then removes and inverts every named guard of section
8 in turn and requires each run to fail, recording the failing output, and kills with
a process-group timeout any mutation that destroys a wait bound. At the citations
commit on #683 the suite held 66 tests and the mutation run detected 214 of 214 runs
across 107 guards. A guard the mutation run cannot fail is a guard that enforces
nothing, and that count is read at every act rather than copied from here.

**Every stub is built from a capture, never from what the code expects.** Each
constant in `golden.py` is a real tool's output captured unprivileged on this box, a
line copied from a real trace, or a shape read from the tool's source at `e69916a`,
with its command or file and lines and a sha256 beside it. The rule was earned:
a stub written to its own code's assumption is how a quoted artifact in `derive`'s
declaration reached review as a refusal every re-feed would have hit on the device.

**What the instruments cannot buy, named.** No test here touches the card, loads a
model, or runs as root, so `gpu-tuple`, `cuda-local`, `diagnostic-load` and the
provisioning path are watched against stubs and captured shapes and are proven only
by the first held run under the operator's own sequence. The approval custody gap of
section 5 has no instrument until the privileged step lands, and this document says
so rather than claiming otherwise.

## 10. What this document does not carry

The results of any arm, which are #679's evidence and the report's. The design of the
privileged approval step, design item B on #679 (evidence digests passed from the
coordinator's state into the driver rather than re-read by path), the re-staging of
the two stacks without links and with their CUDA libraries, and the `cuda_provenance`
ruling, all of which are hold-lift items on #679. The historical instrument's readers
and gate client, which are #516's and are cited by hash rather than restated. And the
question this document was written to answer, whether operator tooling under
`deploy/` answers to a document in this corpus: it does, and this is the document.
