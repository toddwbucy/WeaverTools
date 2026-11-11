# One template, one clone: the WeaverAgent stack in a virtual machine, 2026-11-11

A dated measurement for toddwbucy/WeaverWeb issue #71, read by no gate: one template
virtual machine built from a cloud image with the WeaverAgent deploy scripts as they
stand, sealed, cloned once, and an agent created in the clone, with every break
recorded and what got past it. No GPU is attached; the brief (`BRIEF.md`, the first
commit of this act) keeps the card for a later phase on the Planner's word. Nothing in
WeaverAgent or WeaverWeb was edited, and no copy of their scripts was patched inside a
guest either: where a script broke, the guest's environment was changed for the run and
the change is recorded below. Each break that is a gap in another repository's interface
is an issue draft sent to the Planner, not filed.

The deposit is `weaver-testing/vm-template-2026-11-10/` on the shared bulk store. It
holds every step's log, the domain definitions, the template's digest and the issue
drafts, and none of it is copied here. The scripts that produced every result below are
beside this note in `code/`, and `COMMANDS.md` gives how each was run.

## Summary

- **The stack installs and an agent is created in a clone of a sealed template, and its
  load refuses where a box without a card must.** The template, from the Ubuntu Server
  26.04.1 cloud image, ran `deploy/bootstrap-stack.sh --install` at
  fba665b67c087c7f1ff004fc5946b5abb64c1fb5 to the end with the CUDA build
  (`weaver-spu/cuda,weaver-state/sqlite`) and NVIDIA driver 615.71.09 installed. The
  clone booted to SSH in 16 s from its domain's definition, with its own machine id and
  host keys. In it, `deploy/verify-lifecycle.sh --apply` passed steps 0, 1 and 1b: the
  stack record, `create-agent.sh`'s whole layout and walls, and the seven near misses
  each answering `boundary_unverified`. Step 2's load answered
  `{"kind":"organ_refused","organ":"spu","reason":{"kind":"device_cannot_admit"}}`, the
  worker naming `{"ordinal":0,"refusal":"device_unreachable"}`, and admin rolled it back
  whole. `create-agent.sh`, `weaver-admin validate` and `load` run by hand for a second
  agent answered the same.
- **Fourteen breaks**, eleven during the template and three in the clone, each in the
  table under "Breaks". They gave nine issue drafts for WeaverAgent and one for
  WeaverWeb; a tenth for WeaverAgent (draft 10) asks about the build material every box
  must carry.
- **Two of the clone's breaks come from Ubuntu 26.04's Rust userland, and one weakens a
  security check without failing.** Ubuntu 26.04's `test` (the Rust coreutils) ignores
  supplementary groups, so every "this account cannot read, write or pass" wall check in
  `create-agent.sh` and `verify-lifecycle.sh` answers "cannot" for access granted through
  a group, which is how those walls would be breached. It surfaced only because one check
  is positive. Ubuntu 26.04's `sudo` (sudo-rs) refuses the agent's sudo rule outright.
- **Tests in the stack scripts' test step fail run to run, two of them often, and every
  script that installs or verifies refuses on any.** On one unchanged tree the step
  passed 3 of 17 runs under heavy host load, 2 of 6 after that load was confined away
  from the guest's CPUs, 3 of 6 in the clone's lifecycle runs, 4 of 10 in test-only
  runs beside a confined stress, and 1 of 10 with no stressor on the host (0 of the 4
  that started at a host load under 5): 13 of 49 in all.
- **User-mode networking reaches the host's loopback over IPv4 and IPv6, and the
  guest's guard as built covered IPv4 alone,** so the guest could reach three host
  services over IPv6 throughout. Turning IPv6 off on the domain closes that; and
  `restrict=on` with one `guestfwd` refuses everything but the named endpoint from the
  host side, a fit for a loaded agent but not for the build. NAT, which the brief named,
  was blocked by the host's firewall and is the operator's to rule.

## The network

Before any template, a throwaway guest measured what each of two network backends
reaches (`code/net-probe.sh`, `code/guest/net-probe`).

| Backend | DHCP | DNS | ICMP out | TCP 443 out | HTTP, HTTPS out |
|---|---|---|---|---|---|
| libvirt's default NAT network | no lease in 120 s | no | no | no | no |
| QEMU user-mode networking (slirp) | lease in under 9 s | yes | yes | yes | 200 from every endpoint the build needs |

On the measuring host, libvirt's NAT network is blocked by the host's own firewall
policy, which drops what the guest sends to the host and what it sends through it; the
guest's DHCP requests reached the bridge and no offer came back. Changing that policy
is the operator's to rule, and nothing on the host was changed. The template was built
on user-mode networking instead, which needs no host change: the guest's traffic leaves
as the QEMU process's own. That, and the conditions on it below, are the Planner's
overnight decision, not yet the operator's ruling.

**User-mode networking gives the guest a path to the host's loopback, over IPv4 and
IPv6.** It maps the guest's gateway address, 10.0.2.2, onto the host's 127.0.0.1, and
with IPv6 on (QEMU's default) fec0::2 onto the host's ::1, so a guest reaches whatever
the host binds to loopback. Five such ports were probed with a bare TCP connect and no
protocol: over IPv4 five connected; over IPv6 the three the host also binds on ::1
connected, and the other two answered "refused", which tells the guest they are bound
on IPv4.

**The guest's guard, as the template and clone were built, covered IPv4 alone.** From
first boot an nftables rule drops every new connection the guest opens to 10.0.2.2,
replies on connections the host opens (the SSH forward) passing. It is laid twice, by
the seed's `bootcmd` before SSH starts and by the template's `/etc/nftables.conf`. With
it none of the five connected over IPv4 and outbound traffic was unchanged. The IPv6
alias was not measured until after the clone's lifecycle, prompted by a review of the
committed scripts: with the guard as built, the clone still reached the host's three
::1 services over fec0::2. **So through the template's build and the clone's runs the
guest had a path to those three host services.** Nothing in the measurement used it,
and no agent served.

Each option against the same five ports, measured in the clone (`clone.sh netdev`, then
the `loopback-probe` step; IPv4 and IPv6 each):

| Domain's `-netdev user` | Guard in the guest | Host loopback reached | Internet | A named endpoint |
|---|---|---|---|---|
| as built (IPv6 on) | IPv4 line | 3 of 10 (the ::1 services) | yes | none set |
| as built (IPv6 on) | IPv4 and `ip6 daddr fec0::2 drop` | 0 of 10 | yes | none set |
| `ipv6=off` | IPv4 line | 0 of 10 (IPv6 unreachable) | yes | none set |
| `ipv6=off,restrict=on` plus one `guestfwd` | none: the guard deleted | 0 of 10 (IPv4 refused, IPv6 unreachable) | no, nor DNS | the `guestfwd` endpoint answered 200 |

`code/lib.sh` now gives every new domain `ipv6=off` and lays the IPv6 line in both
copies of the guard; the sealed template predates both, so its `/etc/nftables.conf`
carries the IPv4 line alone, which is enough once the domain has IPv6 off.

**This QEMU can refuse the guest's path to the host's loopback from the host side only
by refusing everything but named endpoints.** libslirp 4.9.5 carries a narrower switch
(`bool disable_host_loopback; /* Prohibit connecting to 127.0.0.1:* */` in
`libslirp.h`), and QEMU 11.1.1 does not expose it: `-netdev user,...,disable-host-loopback=on`
answers `Invalid parameter 'disable-host-loopback'`, as do the underscore form and
`map-host-loopback=`. The passt backend has `map-host-loopback`, and passt is not
installed on the measuring host (`-netdev passt` answers that it cannot execute
`passt`); installing it is a host package, the operator's to rule. What this QEMU does
offer is `restrict=on`, which isolates the guest from the host and the outside, with
`guestfwd` entries as the only ways out: each names one host endpoint that QEMU itself
connects to when the guest dials a virtual address. The build cannot run that way (it
downloads), but a loaded agent whose connectors dial one WeaverWeb endpoint could, with
the connector's server set to the virtual address and its TLS name to the server's own.
That is a host-side boundary needing no host change beyond the domain's arguments; the
guard inside the guest is not one, since the guest's root can remove it. Running QEMU in
a network namespace of its own is another host-side shape, which needs privileged host
steps and was not measured.

**This fixes a condition of the method**, set by the Planner and not yet the operator's
ruling. User-mode networking carried the template's build and the clone below. In the clone the load was attempted twice and refused at the
device before any agent served; its worker started and ended inside admin's rollback.
No agent is loaded on user-mode networking, in this measurement or later, without the
operator's ruling: a loaded agent's processes with a path to the host's loopback
services is what the virtual machine boundary is meant to remove. The sealed template
does not depend on user-mode networking: its network configuration is regenerated at
each clone's first boot for whatever interface the domain gives it, and the loopback
guard matches nothing on another network.

SSH reaches each guest through a QEMU port forward on host loopback, given as the
domain's own QEMU arguments, since libvirt's port forwarding needs the passt backend.
**The scripts do not pin the guest's host key** (`StrictHostKeyChecking=no`, no known
hosts file, in `code/lib.sh`): a local process holding the forward's port first would be
answered as the guest, take what the scripts send and could answer for it. The key the
scripts log in with is not exposed by that, since public-key authentication does not
send it. A host key made per guest, handed in through its seed and pinned in a known
hosts file of its own, would close it; that is left for the next phase, where a new
clone can show it working, rather than changed untested here.

## The template

`code/build-template.sh` builds it one step at a time from the Ubuntu Server 26.04.1
cloud image (signature verified against Ubuntu's cloud image key; kernel 7.0.0-38,
systemd 259, gcc 15.2.0, Python 3.14) on 16 virtual CPUs and 32 GiB, UEFI without
secure boot. The steps, in order, are in `COMMANDS.md`. What it holds:

- NVIDIA's ubuntu2604 repository: `nvidia-driver-open` 615.71.09 (built by DKMS, which
  finds no device to load on), `cuda-toolkit-13-4` 13.4.2 (nvcc V13.4.92), and
  `cccl-13-3` 13.3.3.4.1 for CCCL 3.3.4 (break 6).
- WeaverAgent at fba665b, carried in as a git bundle and checked out detached; the
  toolchain the tree pins (nightly-2026-02-13); the stack installed by
  `bootstrap-stack.sh --install`, its install tuple naming the commit and features.
- The two box accounts, `weaver-gatecon` and `weaver-admincon` (system, no login shell,
  no home, a group of their own), which `create-agent.sh` later judged as the box's.
- gate-con and admin-con from WeaverWeb at c8b9c01 under `/opt/weaver-web/bin`, with
  disabled units and no configs (break 11 says why every part of that is invented).
- An fstab line mounting the host's models over virtiofs, read-only and `nofail`, and
  the loopback guard in `/etc/nftables.conf`.

Beyond the operator's list (the Planner's rulings in the brief allow it as a stated
deviation), the
template carries the Rust toolchain (616 MB), the cargo registry (975 MB), both source
trees (43 MB and 7 MB), both target directories (2.1 GB and 693 MB), the CUDA toolkit
(5.2 GB), the build packages, the bundle, and the guest operator account with
passwordless sudo. 20 GB of its 120 GB disk is used. The deploy scripts need all of the
build material on the box (draft 10).

**The seal**, as the template's last step: the machine id emptied to `uninitialized`,
the SSH host keys and the guest operator account's `authorized_keys` removed, the DKMS
module-signing pair removed, `cloud-init clean --logs --seed --machine-id --configs
network`, the journal vacuumed, `fstrim`, power off, the domain undefined and the disk
made read-only.
Its last section lists every key-named file left on the disk.

**The first seal missed a per-machine key.** That list showed fwupd's client pair
(`/var/lib/fwupd/pki/secret.key` and `client.pem`), made at fwupd's first start, which
every clone would have shared. The template was booted once more, and before the second
seal the second pass looked for others: the files a first boot makes per machine
(systemd's credential secret and random seed, snapd's device key, OpenSSL's random state
beside the module-signing pair), the guest's home directories and root's, and a search
of `/etc`, `/var/lib` and `/root` for names holding `secret`, `private` or `.seed`. It found
fwupd's pair and OpenSSL's `.rnd` beside the module-signing directory; no snap is
installed and systemd's credential secret does not exist. All three were added to the
seal step and the template resealed. What is left of the key-named files is public CA
certificates, test fixtures inside crate sources, and certificate bundles shipped with
NVIDIA's tools. The first seal's logs are kept in the deposit as `*-attempt1`.

The sealed template is 21.9 GB, sha256
`a094ef413d4f86dfb16b924de88d20691a020f195720e74441afa0f5a5ae2d29` (the first seal's was
`04f14b32f53525a8fd06178db52648e8b94a672ae49e58ed7363c44607bed638`).

## The clone

`code/clone.sh` makes an overlay naming the sealed template, a seed of its own, its own
firmware variables and SSH port, 12 virtual CPUs and 24 GiB, and the host's models
directory shared by virtiofs with libvirt's `readonly` set.

**Identity.** Its own hostname, a new machine id and new SSH host keys, cloud-init done
from its own seed, a fresh address on the interface the domain gave it, the loopback
guard standing once, and the stack's install tuple and the checkout at fba665b as the
template left them.

**The models share is read-only from both sides.** libvirt started virtiofsd as root with
`--readonly`. The guest mounted the share `ro`, and a write as root answered
`Read-only file system`. Guest root then remounted it `rw` and wrote again: the host's
virtiofsd refused it with the same error, and nothing reached the host directory. The
small model read whole in 0.8 s (sha256
`2f82233630c349ccf6b8daccf48f9a7865713d9f08a2eadfa456cebe9b97c7f5`). Two things about one
share for every guest, for #71's open question: numeric owners pass through unchanged, so
files the host's uid 1000 owns read in the guest as owned by the guest's operator
account, uid 1000; and every model in the directory is visible to every guest, as are
links pointing outside it (they dangle in the guest).

**Lifecycle.** `verify-lifecycle.sh --agent vtest1 --artifact <the small model> --apply`
was rerun past each break, the agent taken down with `--cleanup` between runs. Each
run's start, the host's load average at that moment and its outcome:

| Run | Start (UTC) | Host load (1 min) | Outcome |
|---|---|---|---|
| 1 | 09:38:50 | 4.2 | step 0: the test step failed (backpressure) |
| 2 | 09:39:37 | 3.4 | step 1: `create-agent.sh` refused (break 12) |
| 3 | 09:43:23 | 36.3 | step 0: the test step failed (backpressure) |
| 4 | 09:43:39 | 37.8 | step 1: `create-agent.sh` refused (break 13) |
| 5 | 09:45:26 | 38.3 | step 0: the test step failed (backpressure) |
| 6 | 09:45:42 | 37.2 | steps 0, 1 and 1b passed; step 2: the load refused at the device |

Between runs 2 and 3, and runs 4 and 5, the guest's environment was changed for the
break (below). The host carried other seats' work throughout: from about 09:35Z to
about 09:47Z a test run on twelve of the guest's host CPUs, and from 09:45:27Z a CPU
stress confined to host CPUs the guest does not use. The load figures are the host's
total; not every source of the jump at 09:43Z was identified.

**The refusal, and how far a box without a card gets.** The load reached admin's start:
the agent's cgroup was made, the worker and SPU started, and the trace took a `load`
event, a `refusal` and an `unload`. The SPU refused at its first driver query:
`devices::judge_identical` (`crates/weaver-spu/src/devices.rs` line 154) found ordinal 0
unreachable, `residency.rs` line 852 carried it as `AdmitRefusal::DeviceRefused`, and
line 178 maps that to `DeviceCannotAdmit`; the worker's log line is `main.rs` line 1498.
(A build without the `cuda` feature would refuse one step earlier, at `residency.rs`
lines 849-850, with no driver to ask.) Admin's rollback answered
`rolled back: marker left open on the run, leave undone, constituents ended, cgroup ended, sink closed`,
and `show` answered `unloaded`. Lifecycle steps 2 (from the load) to 9 need a card and
stand for the GPU phase: the turn, save points, reload, restore, the moved access rule,
the forced take-down, the crash and the older root.

## Breaks

Each break, what got past it, and where it is drafted. "Guest" means the change was made
inside the guest for the run, never in a repository.

| # | Where | Break | Got past by | Draft |
|---|---|---|---|---|
| 1 | template, `nvidia` | Installing `nvidia-open=615.71.09` in the same apt run as the repository's pin package resolved the driver to 620.70.06 | the pin package first, then an apt preference holding the CUDA 13.4.2 metapackages; template rebuilt from the image | method, none |
| 2 | `bootstrap-stack.sh` | `nvcc` not on PATH (NVIDIA's packages install under `/usr/local/cuda`): `REFUSED: no nvcc on PATH and the build carries weaver-spu/cuda` | guest PATH | WeaverAgent 7 |
| 3 | `bootstrap-stack.sh` | `CUDA_LIB_DIR` defaults to `/opt/cuda/lib64`, absent here, refused as "not held closed by root" | guest `CUDA_LIB_DIR=/usr/local/cuda/lib64` | WeaverAgent 7 |
| 4 | the test step | 15 tests fail on `unshare: write failed /proc/self/uid_map: Operation not permitted` (Ubuntu's AppArmor restriction on unprivileged user namespaces) | guest `kernel.apparmor_restrict_unprivileged_userns=0` for the step, restored after | WeaverAgent 1 |
| 5 | the test step | the gate's descriptor test assumes bash: dash as `/bin/sh` holds descriptors 10 and 11 | guest `/bin/sh` as bash for the step, restored after | WeaverAgent 2 |
| 6 | the build | llama.cpp's CUDA objects fail against the toolkit's CCCL 3.4.3 (`namespace "cuda" has no member "make_strided_iterator"`), outside the scripts' window, which they read only through `pacman` | guest `cccl-13-3` (CCCL 3.3.4) ahead through `NVCC_PREPEND_FLAGS`, CUDA crates rebuilt clean | WeaverAgent 5 |
| 7 | the build | candle-kernels reads the compute capability from `nvidia-smi`, which has no card to ask: `ComputeCapDetectionFailed` | guest `CUDA_COMPUTE_CAP=86` | WeaverAgent 4 |
| 8 | the lock gate | `process/gates/lock.sh` answers 2 offline on a cold cargo cache (`failed to download crunchy`) | `cargo fetch --locked` first | method, none |
| 9 | the test step | two tests fail run to run, and the step refuses on either (figures below) | rerun | WeaverAgent 3 |
| 10 | install tuple | with the driver and no card, nvidia-smi's error line is recorded as the driver and as one GPU (`gpus = 1`) | none; recorded as found | WeaverAgent 6 |
| 11 | connectors | WeaverWeb defines no binary location, unit or config path, and the install script its Spec names does not exist | the template invented them, marked measurement-only | WeaverWeb 1 |
| 12 | clone, `create-agent.sh` | `REFUSED: the admin-con account weaver-admincon cannot read the declaration ...`: Ubuntu 26.04's `test` ignores supplementary groups, so the wall checks answer "cannot" for any access through a group | guest: GNU's `test` linked into `/usr/local/bin`, ahead on sudo's path | WeaverAgent 8 |
| 13 | clone, `create-agent.sh` | sudo-rs's visudo refuses the agent's rule: `unknown setting: 'pam_session'` | guest: the `sudo` alternative switched to the original sudo, which Ubuntu also installs | WeaverAgent 9 |
| 14 | clone, the load | no card: `device_cannot_admit` | none; the expected stop | none |

Two of the clone's changes were narrowed on purpose. Ubuntu's own switch to GNU
coreutils (`coreutils-from-gnu` in place of `coreutils-from-uutils`) would remove
`build-essential` and the CUDA compiler packages, so only `test` was put ahead, by a
link no package owns. Both changes were made in the clone alone; the sealed template
still carries Ubuntu's defaults for both.

The template's guest environment for every bootstrap and lifecycle run, as the commands
named it: `PATH` with `/usr/local/cuda/bin`, `CUDA_LIB_DIR=/usr/local/cuda/lib64`,
`CUDA_COMPUTE_CAP=86`,
`NVCC_PREPEND_FLAGS=-I/usr/local/cuda-13.3/targets/x86_64-linux/include/cccl`, and the
two test-step settings of breaks 4 and 5 set by the step and restored after it.

## The tests that answer differently run to run

One unchanged tree and build, the stack scripts' own test step
(`cargo test --release --locked -p weaver-trace -p weaver-harness -p weaver-state --features weaver-state/sqlite`).
The two that fail most are `flow::coordinator::tests::trace_backpressure_holds_valid_completion_and_prevents_the_next_file`
(`coordinator_tests.rs:454`, "bounded fixture did not finish ordered filing") and the
preload door's user-namespace test, whose inner
`preload_door::recorded_rule_three_way_at_matched_cuts` panics at `preload_door.rs:883`
("three-way shape disagreement"). Three others failed now and then:
`capture_tests::queued_release_...` once in the template, a `pre_enter` test twice in the
clone, and `lifecycle::tests::an_organ_that_never_exits_is_killed_then_reaped`
(`lifecycle/tests.rs:4226`) once in the clone. Every guest virtual CPU is a host thread,
so host load is given beside each figure. It is the host's whole load average, the
guest's own threads among it: twelve virtual CPUs running the test step lift it by up to
about twelve on their own.

| Where | Runs (UTC) | Host load | Test step passed |
|---|---|---|---|
| template, plan runs 2-10 | 08:58-09:08 | another seat's CPU stress on every host thread, load rising from about 18 toward 120 | 3 of 9 |
| template, plan runs 11-18 | 09:11-09:15 | the same, about 115 to 120 | 0 of 8 |
| template, plan runs 19-22 and install runs 1-2 | 09:19-09:30 | that stress confined to host CPUs the guest does not use | 2 of 6 (19: preload door; 20, 21, install 1: backpressure) |
| clone, lifecycle runs 1-6 | 09:38-09:46 | 3.4 to 38, per run in the lifecycle table | 3 of 6 (each failure backpressure) |
| clone, test-only runs 1-10 | 09:48-09:52 | 12.7 to 40.2, per run below | 4 of 10 |
| clone, test-only runs 11-20 | 10:10-10:13 | no stressor; 2.7 to 27.1, per run below | 1 of 10 (0 of 4 under a host load of 5) |

The clone's test-only runs, each with the host's load average as it started; from
09:45:27Z another seat ran a CPU stress confined to host CPUs the guest does not use:

| Run | Start (UTC) | Host load (1 min) | Outcome |
|---|---|---|---|
| 1 | 09:48:24 | 38.3 | passed |
| 2 | 09:48:53 | 38.1 | preload door (`preload_door.rs:883`, both three-way tests) |
| 3 | 09:49:20 | 40.0 | backpressure (`coordinator_tests.rs:454`) |
| 4 | 09:49:35 | 40.2 | passed |
| 5 | 09:50:05 | 39.4 | passed |
| 6 | 09:50:34 | 39.8 | preload door |
| 7 | 09:51:01 | 29.1 | backpressure |
| 8 | 09:51:17 | 23.8 | preload door |
| 9 | 09:51:43 | 16.9 | passed |
| 10 | 09:52:12 | 12.7 | a third test: `flow::context::worker_startup::pre_enter::tests::pending_answer_waits_for_capacity_without_reading_a_second_packet_or_inventing_a_bound`, at `context_worker_pre_enter_owner_tests.rs:256` ("actual admin EOF retires the connection") |

Runs 11-20, after the other seat's stress had ended: the clone restarted with
`ipv6=off`, nothing on the host but other seats' builds pinned to host CPUs the guest
does not use, and the load rising from run 15 mostly with the guest's own runs:

| Run | Start (UTC) | Host load (1 min) | Outcome |
|---|---|---|---|
| 11 | 10:10:22 | 2.7 | backpressure |
| 12 | 10:10:37 | 3.3 | preload door (both three-way tests) |
| 13 | 10:11:04 | 4.6 | preload door |
| 14 | 10:11:31 | 4.1 | backpressure |
| 15 | 10:11:46 | 11.6 | backpressure |
| 16 | 10:12:10 | 24.6 | backpressure and the `pre_enter` test |
| 17 | 10:12:26 | 22.4 | backpressure |
| 18 | 10:12:41 | 18.1 | `an_organ_that_never_exits_is_killed_then_reaped` (`lifecycle/tests.rs:4226`) |
| 19 | 10:12:57 | 21.8 | backpressure |
| 20 | 10:13:19 | 27.1 | passed |

So host load is not what makes these tests fail: on the quietest host this measurement
had, the test step failed four runs out of four inside the guest.

Template plan run 1 is left out: it ran before the settings of breaks 4 and 5 and failed
on those. The per-run logs are in the deposit.

## Conditions

- The guest's virtual CPUs and QEMU's own threads were pinned (libvirt `cpuset`) to the
  host CPUs the Planner left for this work, away from another seat's measurement.
- User-mode networking for the template's build and the card-less clone only, by the
  Planner's overnight decision, not yet the operator's ruling; the loopback guard in
  every guest, covering IPv4 alone until the
  IPv6 measurement above. No agent served.
- For the netdev measurements, a temporary HTTP listener on host loopback served as the
  `guestfwd` endpoint and was stopped after; the clone's `-netdev` was set back to the
  value it was created with.
- No host package, service, firewall rule, kernel parameter or device binding was
  changed; no GPU was touched.

## Left standing

The sealed template (read-only, its domain undefined) and the clone, shut down and
defined, holding the agent `vmagent1` created and unloaded, for a look before the GPU
phase. The throwaway network-probe guests were removed.
