# The determinism slice: one agent on the host, then in a virtual machine, 2026-11-12

A dated measurement for toddwbucy/WeaverWeb issue #71, read by no gate. The operator's
question was to run the agent on the host, move that agent into a virtual machine and run
it there, then compare: are the bits different or the same?

The slice ran on scratch versions of the scripts in `code/`, which hold the measuring
host's facts. The template act of 2026-11-12 folds what it proved into `code/`, per
`template-clone-Spec.md`. The evidence (each run's matrix records, summaries, comparisons
and step logs) is held on the measuring host and is not yet in a deposit.

## The answer, and its scope

**The agent's emissions in the virtual machine are bit for bit the same as on the host.**
Every comparison reads all 464 turn slots identical and none different:

| Comparison | Identical | Different |
|---|---|---|
| H0 against H1 (the host's two cards) | 464 | 0 |
| H1 against G1 (the same card, host against guest) | 464 | 0 |
| H1 against G1b | 464 | 0 |
| G1 against G1b (the guest's two runs) | 464 | 0 |
| H0 against G1 | 464 | 0 |
| H0 against G1b | 464 | 0 |

The scope is exactly this:

- **One card.** The host's second RTX A6000, passed through to the guest whole.
- **One model.** qwen2.5-0.5b-instruct at q6_k (sha256 `2f822336...`).
- **One declaration.** The storeless scratch declaration, with the sampler as declared.
  The host's and the guest's declarations differ in two lines only: `devices` and the
  trace path.
- **One clone.** It was made by the scratch scripts from the template of 2026-11-11, with
  the host's stack carried in: WeaverAgent `61d99dce`, its binaries, its engine libraries
  and its install tuple.
- **Script defects from three stopped runs and a rehearsal**, fixed on the way and named
  below.

## What it does not show

- **A guest-built stack.** The guest ran the host's binaries. The template's own build
  (`fba665b`) was set aside, not run.
- **A second model, or a two-card binding.**
- **The console driving the agent through the connectors.** The matrix drove the agent
  through the gate's socket inside the guest. The one `guestfwd` to a WeaverWeb endpoint
  was in place and was not used.
- **Speed.** Nothing about speed beyond the session counts below.
- **Another clone, template or host.** Section "Known deviations" lists what was
  particular to this one.

## How it ran

**The instrument** is the determinism matrix from this repository at `e4893730`, the
graded head of #23, whose tree reached `main` with #22. Each run lasted 0.12 hours, with
a floor of 64 sessions; the matrix has 32 matrix cells and 464 turn slots. The harness
tests passed before each half's first load.

**The host half** ran first, by the operator, on the host's own driver, 620.70.06. Two scratch agents were laid by
`HowToDeployANewAgent.md` section 3, the operator's account alone in each gate rule (the
operator's ruling of 2026-11-11, for these scratch agents only), and taken down after:

| Run | Card | Sessions | Reproduced | Diverged | Errors |
|---|---|---|---|---|---|
| H0 | the host's first A6000 | 125 | 125 | 0 | 0 |
| H1 | the host's second A6000 | 126 | 126 | 0 | 0 |
| H1b | the host's second A6000 | 127 | 127 | 0 | 0 |

**The guest half** ran on the Planner's word. The operator's word, relayed, was "proceed
with the second part of the test". Steps, in order:

1. **The clone.** A clone of the template of 2026-11-11, 12 vCPUs and 24 GiB.
   - The second A6000's two functions went to it as host devices that libvirt never
     moves.
   - User-mode networking with `ipv6=off`, `restrict=on` and one `guestfwd`.
   - Its SSH host key was made on the host and pinned (Spec section 2).
   - SSH was the boot gate, and cloud-init's verdict was judged apart (Spec section 3).
2. **Setup B**, a hard stop. With the guard briefly lifted, the clone's reach matched the
   earlier clone's, apart from the one line that the restricted network must refuse. The
   guard was then restored.
3. **The driver.** Two fixes for Ubuntu 26.04.1 (GNU `test` ahead on sudo's path, the
   original sudo), as in `README.md`. Then the driver moved from 615.71.09 to 620.70.06 by
   the offline round (Spec section 6): 1 file in the pin round and 15 in the open round,
   each checked against its SHA256 in the guest's signed index. DKMS built the module, and
   the clone rebooted onto it. The guest's `libcuda` reads the same digest as the host's
   (`aa6bd511`).
4. **The carry.** The host's stack was carried in after a loader check that passed at
   GLIBC 2.43, the guest's ceiling. The four CUDA runtime libraries the stack loads read
   the same in guest and host.
5. **The runs.** A scratch agent was laid as on the host, and its harness
   tests passed:

| Run | Card | Sessions | Reproduced | Diverged | Errors |
|---|---|---|---|---|---|
| G1 | the second A6000, in the guest | 98 | 98 | 0 | 0 |
| G1b | the second A6000, in the guest | 99 | 99 | 0 | 0 |

Then the agent was taken down and the comparisons made. Across H0, H1, H1b, G1 and G1b,
these readings are the same:

- the weights;
- the ten engine libraries, `libcuda` among them;
- the worker, SPU and gate binaries;
- `rustc`.

The one reading that differs is `active_toolchain`, and it comes from a workaround, not
from the guest (D3, below).

## Session counts: fewer sessions, the same generation speed

In the same 0.12 hours, the guest ran 98 and 99 sessions against the host's 125, 126
and 127: about 22 percent fewer on the means. Every run covered all 464 turn slots. The
clone's two boot delays (about 120 s each, section "Known deviations") fall before the
runs, outside the timed window.

All figures below come from the matrix's own records: each session's `seconds`, and each
turn's `source_ms` and `replay_ms`. Nothing new was measured. Per-session means:

| | H0 | H1 | H1b | G1 | G1b |
|---|---|---|---|---|---|
| seconds | 3.47 | 3.43 | 3.42 | 4.43 | 4.37 |
| generation (`source_ms` + `replay_ms`) | 1.121 | 1.115 | 1.129 | 1.136 | 1.130 |
| the rest | 2.35 | 2.32 | 2.30 | 3.29 | 3.24 |
| source start to replay start | 1.73 | 1.71 | 1.71 | 2.21 | 2.18 |

- **Generation runs at the host's speed.** Per turn, the host reads 39.2 to 39.8 ms and the
  guest 39.9 to 40.1 ms, within about 1 ms.
- **The guest's extra time is outside generation.** It spends about 0.95 s more per
  session. About half of that falls between source start and replay start, and half after
  replay start.
- **The records do not break that remainder down further.** Which part of a session's
  work outside generation is slower in the guest was not measured.

## Known deviations of this clone

1. **cloud-init exit 2 on both boots.** It passed on two named items:
   - `network-wait`, the network wait failing after about 120 s, with no proven cause;
   - the schema warning that the seed's `ssh_genkeytypes: []` caused. Spec section 2
     now names the type instead.
2. **The template's guard is IPv4 only.** The template predates commit 78bde86. The
   domain's `ipv6=off` and `restrict=on` closed the IPv6 path, and setup B read as
   expected (Spec section 4).
3. **Two extra host keys.** Only the ed25519 key was pinned. `sshd-keygen.service` made an
   RSA key and an ECDSA key at first boot. Neither was used, and both died with the clone.
   The rebuilt template masks that unit (Spec section 2).
4. **D3, a carried compiler, not needed.** The matrix refuses a run without a toolchain
   reading. In the rehearsal, its `rustc --version` exited 127, so the host's
   nightly-2026-02-13 `rustc` was staged in the guest for that reading only, and the guest
   compiled nothing.
   - A read-only check after the run found the template's own toolchain in the clone:
     `$HOME/.cargo/bin/rustc`, a rustup link; `rustup show active-toolchain` reading
     `nightly-2026-02-13-x86_64-unknown-linux-gnu (default)`; and `rustc --version`
     reading `rustc 1.95.0-nightly (47611e160 2026-02-12)`, the host's line.
   - It was only off the PATH of a non-interactive command, because rustup was installed
     with `--no-modify-path`.
   - So the one differing reading, `active_toolchain`, which reads absent for a bare
     `rustc`, is the workaround's and not the guest's. No emission comparison depends on
     it. Spec section 7 runs the matrix on the template's own toolchain.
5. **No time sync.** Under `restrict=on` the guest has no NTP, and chrony stayed
   unsynchronised. On the clone of the third run, made from the same template by the same
   scripts, the guest's clock read at least 1.2 s behind the host's. No comparison reads
   wall-clock time; the per-session figures are durations on one clock.
6. **The NVIDIA index was checked by hand once.** The guest's NVIDIA Packages list (sha256
   `a6bc48f7...`) was checked against its signed `InRelease` before the third run. apt
   verified it at update, and each `.deb`'s SHA256 came from it.

## The runs that stopped, and the defects they found

Each stop was reported and left as it stood. No run was repaired by hand, and no run was
repeated without the Planner's word. On the Planner's ruling, a rehearsal walked the
remaining steps by hand on the third run's clone, with a short G1. Nothing from the
rehearsal is a result.

| Where it stopped | Defect | Now (Spec section) |
|---|---|---|
| Run 1, the first boot | `wait_ssh` returned `cloud-init status`'s exit, so "degraded done" (exit 2) read as SSH not up | SSH is the gate, and cloud-init is judged on a named allow-list (3) |
| Run 2, the driver's pin round | apt 3.2.0's `--print-uris` gives MD5Sum, a hash the host refuses | SHA256 from the signed index by exact `Filename` (6) |
| Run 3, the driver's open round | D1: the pin round left `~/debs`, so the open round's `scp -r` nested into it | `driver-install` removes `~/debs` and refuses anything but `.deb` files; `put` refuses a standing destination (5, 6) |
| The rehearsal | D2: `scp -r` turned the staged library's ten links into copies | `put` is a tar stream (5) |
| The rehearsal | D3: the toolchain reading exited 127 (`rustc` off the PATH, not absent) | the harness runs on the template's own toolchain (7) |

The fourth run, on the corrected scripts, ran through without a stop.

## Not folded into the scripts

The scratch agent's lay and take-down, the carry of the host's stack, the carry check and
the CUDA runtime comparison stay with the measurement. They belong to the slice, not to
the template (Spec section 9).
