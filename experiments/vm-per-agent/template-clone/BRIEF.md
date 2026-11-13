# Brief: template and clone, one agent per virtual machine, 2026-11-10

The suite Planner's brief to the virtualization Executor, for toddwbucy/WeaverWeb issue
#71, as the first commit of `act/vm-template`. It is restated here with every fact about
the measuring host left out; the hub report carries those.

## The job

A measurement. Build one template virtual machine, install the WeaverAgent stack into it
with the existing deploy scripts, clone it, stand an agent up in the clone, and record
everything that breaks.

## Read first

- toddwbucy/WeaverWeb issue #71: the direction and the security properties.
- The suite workflow, `process/WeaverTools-Suite-Workflow.md`.
- In WeaverAgent, read only: `deploy/update-stack.sh`, `deploy/create-agent.sh`,
  `deploy/verify-lifecycle.sh`, `deploy/REDEPLOY.md`, `deploy/HowToDeployANewAgent.md`.
  The tree to install is the connector accounts stack head, commit
  fba665b67c087c7f1ff004fc5946b5abb64c1fb5 (branch `code/unseeded-turn-check`), not main.

## What the operator decided, 2026-11-10

- One agent per virtual machine. libvirt over QEMU/KVM on a plain Linux host. Another
  hypervisor's API and containers are later backends, out of scope.
- One template image, not one per GPU. It holds the operating system, the NVIDIA driver
  and CUDA runtime, the agent stack as the deploy scripts lay it, the connector binaries
  (gate-con, admin-con, built from WeaverWeb) with their service definitions and no
  configs, and the two connector box accounts.
- The template holds no agent, no agent record, no connector credential or key, and no
  machine identity: the machine id and the SSH host keys regenerate on first boot. The
  template is sealed while shut down, and clones are made from it.
- Per clone: `create-agent.sh` runs, the agent is registered in WeaverWeb, the connector
  configs are placed, and the connectors dial out. The agent still listens on nothing.
- The GPU is a per-VM profile outside the disk, naming which cards pass through. The
  host's display card never passes through.
- Security language only in anything written.

## Constraints

- The Executor does not take privileged steps on the host: no packages, services, kernel
  parameters, device binding or firewall change. Each privileged host step goes to the
  operator as one block, with a line saying what each command does.
- libvirt's default NAT network only. The connectors only dial out, so NAT is enough.
- No GPU is touched before the Planner's word. Phases 1 to 3 are CPU-only on purpose, to
  separate the breaks that have nothing to do with a card from the passthrough ones.
- WeaverAgent, WeaverWeb and WeaverAnalysis are not edited. A break in their scripts is an
  issue draft for that repository (what was measured, what is asked, which document
  would move), sent to the Planner, not filed and not fixed from here. Inside the guest a
  copy may be patched to get past a break and keep measuring, and the patch is recorded.
- Nothing committed names a host, a LAN address, a box path, a person's account or a
  credential. ASCII only, absolute dates, commit subjects `code:`, `docs:` or `process:`.

## Phases

0. Survey by reading: every assumption the three deploy scripts make about the machine,
   a guest operating system with reasons, how the build reaches the guest, and where the
   models live.
1. The operator's host setup: packages, libvirtd, group membership, the default network.
2. Build the template reproducibly from a script (cloud image plus cloud-init), run the
   stack install inside it, record each break and what got past it.
3. Seal the template, clone once, run `create-agent.sh` and `verify-lifecycle.sh` in the
   clone with the small model (qwen2.5-0.5b-instruct-q6_k.gguf), record each break.
4. The GPU profile for one card, only on the Planner's word, after phase 3.

## Rulings since the brief, 2026-11-10 and 2026-11-11

- The template carries the CUDA build the scripts make. Phase 3 runs to the agent load's
  refusal, recorded as how far a box without a card gets; lifecycle steps 2 to 9 run in
  phase 4.
- The WeaverAgent commit reaches the guest as a git bundle that stays off every remote,
  so the clone's origin is a bundle and the scripts' reachable-origin check proves less
  than it would against GitHub.
- The measurement template may carry the toolchain, the CUDA toolkit, both source trees
  and the build directory, as a stated deviation with a list of what it holds beyond the
  operator's list.
- Guest operating system: a current Ubuntu Server LTS cloud image, the Planner's default,
  not yet the operator's ruling.
- Models stay on the host, shared read-only into the guest. One share exposing every
  model to every guest is accepted for the measurement and stands as an open question for
  #71, not as a design.
- Issue drafts follow the measurement of each break.
- The first guest's outbound network is measured before the template is built. If it is
  blocked, the measurement and the options go to the Planner; the host firewall is the
  operator's ruling.

## Where the work lives

Scripts and the result note here, in `experiments/vm-per-agent/template-clone/`
(`README.md` the result note, `COMMANDS.md`, `code/`). Bulk evidence and logs go to a
deposit under `weaver-testing/` on the shared bulk store. No push until the Planner says
so.

## The brief of 2026-11-12: the slice folded into the scripts

The suite Planner's second brief to the virtualization Executor, on the operator's rulings
of the evening of 2026-11-12, for this same stack.

**The job.** Repository work only: no virtual machine started, no template rebuilt, no
card touched. Fold into these scripts what the determinism slice of 2026-11-12 proved
(`results/2026-11-12-determinism-slice.md`), and what the slice's scratch scripts carry
that the repository does not:

- the cloud-init verdict with its named allow-list, in place of `wait_ssh`'s exit;
- the step that takes each driver file's SHA256 from the guest's signed index;
- `put` as a tar stream;
- the harness run with the template's own toolchain on its PATH, not a carried compiler;
- the seed naming the pinned key type in `ssh_genkeytypes`, and the template masking
  `sshd-keygen.service` before the seal, so that no other host key is made;
- the IPv6 line in the guard the template carries;
- an offline mock that exercises `put`, the driver install and the harness preflight in the
  ways that would have caught each of the slice's three script defects.

**The 2026-11-12 slice ran on the scratch versions of these scripts**, which hold the
measuring host's facts. The repository's versions take every such fact as a parameter. No
box path, host name, account, address or key is committed.

**The template itself is rebuilt and re-measured in a later act, on the operator's word.**
Until then the sealed template of 2026-11-11 stands as it is. Changes that act only in a
template build (the guard's IPv6 line in `/etc/nftables.conf`, the `sshd-keygen.service`
mask) are measured there. The first question that act puts to the operator is the driver
pin: the template holds 615.71.09, and the slice moved its clone to 620.70.06 by hand.

**Not folded.** The scratch agent's lay and take-down, the carry of the host's stack, the
carry check and the CUDA comparison stay with the measurement. They belong to the slice,
not to the template.

**Sub-acts**, each a draft pull request against `act/vm-template`, in this order:

1. The Spec of these scripts (`template-clone-Spec.md`), this brief, and the slice's
   result note.
2. `lib.sh`, `clone.sh`, `build-template.sh`: the pinned host key and the seed, SSH as the
   gate with the recorded cloud-init verdict, and `put`.
3. `guest/gpu-steps` and the host's fetch round: the offline driver round, and the harness
   preflight and run.
4. `code/test/offline-mock.sh`: the offline mock.

The operator's rulings of the same evening put GPU1 back on the host for the first real
agent, ahead of any further virtual machine stage. The A/B result is banked, and the stage
resumes later.
