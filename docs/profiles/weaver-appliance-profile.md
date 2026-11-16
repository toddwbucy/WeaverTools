# The agent appliance - deployment profile

**Status:** DRAFT. Not merged.
**Date filed:** 2026-11-15
**Document ID:** `weaver-appliance-profile`
**Class:** deployment profile, per `deployment-profile-convention.md` in this directory
**Authority:** the operator's written brief of 2026-11-14 on the appliance path, his
rulings of 2026-11-14, and his rulings of 2026-11-15. Where two of them differ, the
later one holds.
**Editorial:** ASCII, absolute dates, prose at 88 characters.

---

## 0. What this document is

The deployment profile of the agent appliance: a virtual machine, made from one sealed
image, that carries the WeaverAgent system for one agent.

This document decides nothing about any component and binds no Spec. A requirement on a
component is in that component's charter, and section 6 names the charter for each. A
sentence here is a ruling of the operator with its date, a requirement with its charter,
or a measurement with its record. Section 8 lists what is open.

**Five terms carry one meaning each in this document.**

- The **image** is the sealed disk image. One image exists for each version.
- An **appliance** is a virtual machine that an operator made from the image.
- The **console** is the Weaver Web server, the operator's management surface for many
  agents.
- The **agent** is the bounded thing: the processes of one WeaverAgent that run under
  their own UIDs.
- The **WeaverAgent system** is everything installed to run and to bound that agent:
  admin, the gate, the harness, the SPU, state management, and the two connectors.

This document never uses "the agent" for a component that holds a privilege. It names
the component.

## 1. The stance

The operator's ruling of 2026-11-15:

- We ship a tuple, and the operator produces their own.
- The image is disposable and carries what runs. The mounts carry what the agent keeps.
- Whatever the operator adds or changes after boot is the operator's to account for. No
  guarantee is offered about a changed box.
- Inbound is ours. Outbound is the operator's.
- The virtual machine and the service are built as simply as will work. Machinery for a
  later need is a later item.

Each section below applies it.

**The agent is bounded by the operating system under a UID.** The appliance does not
give the agent an operating system. It gives the WeaverAgent system a machine with
nothing else on it. The agent stays bounded inside that machine by its accounts and
permissions, as it is on any install. A privilege that an operator gives the agent
beyond what the WeaverAgent system needs is the operator's choice.

## 2. The substrate and the entry condition

**The substrate is a virtual machine on a hypervisor that the operator already runs.**
The entry condition is that GPU passthrough already works there. It is a condition. The
suite does not teach it.

**The first image is for KVM.** Proxmox is out of scope by the ruling of 2026-11-14, and
`toddwbucy/WeaverTools` issue 29 records one Proxmox condition to settle later. Other
hypervisor formats are later.

**The machine belongs to the operator and their hypervisor.** The operator makes the
virtual machine, attaches its disks, starts it and stops it. Weaver Web never touches a
hypervisor. It does not create, clone, start, stop, snapshot, network or destroy a
virtual machine. To start and stop the agent is to load and unload it through admin,
inside a running appliance.

**At the first boot the operator logs in at the console of the virtual machine, places
their SSH key, and sets the network by hand** (ruling of 2026-11-15). The operator logs
in as root, which has no password, and sets one at that first login; SSH admits keys
alone, and the image carries no secret (ruling of 2026-11-16). The image disables
cloud-init (ruling of 2026-11-16). Cloud-init is not part of this profile at this stage
and is a later item, per section 8. systemd's own first-boot paths stay as Ubuntu ships
them and are named in the Spec of the image build: what an operator feeds through them
is the operator's to account for (ruling of 2026-11-16).

**The shape of a deployment is the operator's choice.** The suite recommends a machine
given to one agent. It is a recommendation and no rule. In the operator's words: "we
just bring the board and the wires".

## 3. What the profile fixes

### 3.1 What the image carries

**The operating system is Ubuntu 26.04.**

**The NVIDIA driver is the distribution's own prebuilt and signed kernel module of the
615 branch**, with the kernel held to match it. No compiler runs near the driver, and
the module loads under Secure Boot.

**The binaries of the WeaverAgent system are compiled inside the image from named
commits.** After the build, the build tooling and the source trees are removed and the
image is sealed. The image states the commits it was compiled from, its driver version
and its own version.

**The build is made to be reproducible from its commits** (ruling of 2026-11-15). Two
build settings do it: a compiler flag that fixes the names of the compiler's temporary
files, and a list of card architectures that carries compiled kernels only and no
intermediate code. The list is sm_86, sm_89, sm_120a and sm_121a, in the toolchain's own
names.

**The cost is a condition of this profile: a card whose architecture is not on the list
does not run this image.**

**The image carries no model and no state.** Both live on disks that the operator
attaches.

**The image carries no shared key and no shared secret.**

**The image does not update itself.** Automatic package updates and firmware updates are
off. What the operator changes after boot is the operator's, per section 1.

**The seal ends with a compaction, and the published digest is the digest of the
compacted file.**

### 3.2 Inbound

**The image's own boot rules drop inbound traffic and accept SSH** (ruling of
2026-11-15). The appliance listens on SSH alone unless a declaration names a port.

**Admin owns one named table for each agent and never the whole firewall** (rulings of
2026-11-14 and 2026-11-15). A
declaration that names a tool or exposes the gate names its ports. Every load replaces
admin's table before it opens anything, and the table is not persisted (ruling of
2026-11-15). Ports that a declaration opens belong to the operator's arrangement.
Admin's table is the mechanism and not the authority.

**One property of the mechanism is stated here because it binds two documents.** In
nftables an accept in one table does not override a drop in another table. So an accept
in admin's table does not by itself open a port through the image's boot rules. As of
2026-11-15 no declaration can name a port, and nothing is built for it. Section 8
carries the open cell.

### 3.3 Outbound

**The image carries no egress limit** (ruling of 2026-11-14, confirmed 2026-11-15). What
an appliance reaches is the operator's declaration and the operator's network.

**The time source is the operating system's default, a server the operator can set.**
Any accurate source serves.

### 3.4 Storage

**Two mount points are fixed: `/mnt/weaver/model` and `/mnt/weaver/state`.** The
operator attaches the disks in the hypervisor and mounts them.

**The model mount is required.** A load is refused where the model mount is not a
separate filesystem.

**The state mount is required only where the declaration elects state.** The same test
then refuses the load. An agent whose declaration elects no state runs on the model
mount alone.

**The agent's territory is `/var/lib/weaver-agent`, on the image's root.** The image is
disposable. What the agent keeps is on the mounts.

**State and save points on an appliance are out of scope as of 2026-11-15.** The first
agent declares no state (ruling of 2026-11-15). Section 8 names it as a later item.

### 3.5 The agent service

**Nothing in the image starts the agent.** After a boot the connectors come up and the
console sees the appliance. The agent stays unloaded until the console asks admin to
load it. An operator's own packaging is the operator's business.

**Connector keys and certificates live on the image's root disk.** So a new image is a
new enrollment. The operator does it from the console with one pairing line.

**An upgrade moves the state and the model to a machine made from the new version.** The
mounts carry them. Section 5 states what is claimed about the result.

**The pairing line carries a one-time secret, and it is the only secret that travels on
a command line** (ruling of 2026-11-14). The appliance dials the console. The console
does not dial the appliance.

## 4. What the declaration still chooses

The profile fixes the machine's software. The operator still chooses the following for
each agent, and each choice is part of that agent's arrangement.

| Choice | Where it is made | Note |
|---|---|---|
| The model | the declaration | The bytes live on the model mount. |
| Whether the agent elects state | the declaration | Where it does, the state mount is required. |
| Where the trace goes | a field on Weaver Web's declaration page, set when the agent is created | The suite sends the trace where it is told. A file, `/dev/null` or anything else: what stands behind it is the operator's. |
| The ports of a tool or of the gate | the declaration | Admin opens them in its table at load. |
| The card, and the virtual CPU model | the hypervisor | Both are members of the tuple, per section 5. |
| What the appliance may reach | the operator's network | The image places no limit. |

## 5. How a deployment is recorded on the tuple

**The image version is a member of the deployment tuple.** A version is written
`<release>.<respin>`. The release moves when the binaries move. The respin moves when
only the operating system layer moves. A respin never recompiles. If the binaries change
for any reason, the release moves.

**Each image version has one annotated tag, named `appliance/<version>`,** in
`toddwbucy/WeaverAgent`, on the commit that the version was compiled from. The tag
message carries the version, the digest of the image's install record and the test
count. The operator pushes the tag. The record is keyed by version and not by commit.

**The driver version, the card and the guest CPU are members of the tuple.** The guest
CPU joined on 2026-11-15: the math library selects code paths from CPU features.

**The run card records the tuple as shipped.** It also records the ports that a
declaration opened and the trace sink that the operator chose. No guarantee is offered
about the tuple as running. An operator who changes the box has produced their own
tuple.

**The image carries a manifest of its files and links.** It stays available as a tool
that the operator may run. Nothing runs it automatically.

**Reproducibility is claimed within one version on one hardware set.**

## 6. The asks of the charters

Each row is a requirement that this profile needs from a component. The requirement
lives in the charter named. This document does not state it with authority.

| Charter | What the profile needs | Where it stands on 2026-11-15 |
|---|---|---|
| `weaver-admin-PRD` (WeaverAgent) | The mount tests of section 3.4, made at every load. One named table, replaced at every load and not persisted. No autostart. | Sections 7.1 to 7.5, on pull request 561, under amendment for the rulings of 2026-11-15. |
| `weaver-agent-PRD` (WeaverAgent) | One clarifying sentence: the rule that no listening socket exists binds the program's own processes. | Owed in the act that adds the port member to the declaration. |
| `weaver-admin-operator-contract` (WeaverAgent) | The member that carries the operator's choice of trace sink from the console to admin. | Not filed. |
| the deployment tuple (WeaverAgent) | The guest CPU as a member. The tag of section 5. | Not filed. |
| `weaver-web-PRD` (WeaverWeb) | Enrollment on the link port, with keys made in the appliance and signed by the console. The link's one unauthenticated surface, with the two browser pages that answer before a session named apart. The console described as management-plane software. The trace sink as a declared field. The clone of an enrolled appliance as a note. The model sources ruled on 2026-11-14. | On pull request 118, under amendment for the rulings of 2026-11-15. |
| this repository | The Spec of the image build, under this profile. | As of 2026-11-15 it is section 10 of `template-clone-Spec`, on pull request 31. It moves to its own document. |

**The image build's Spec is the one artifact gated on issue 28.** The enrollment work
and the admin work are not gated on it.

**On models.** The rulings of 2026-11-14 stand as asks of `weaver-web-PRD`: the console
fetches models from four sources, pins each by exact revision and digest, and keeps
access tokens in its own store. They state what the console offers. They place no limit
on an appliance, since the claim that an appliance never reaches the internet was
withdrawn on 2026-11-15.

## 7. The evidence, and what is owed

### 7.1 What is measured

**Two determinism counts exist.** Each is a count with its tuple. Neither is a rate.

| Member | One card, 2026-11-12 | Two cards, 2026-11-13 |
|---|---|---|
| Count | 464 of 464 turn slots identical | 86 of 86 turns identical |
| Model and precision | qwen2.5-0.5b-instruct at q6_k | Granite 4.2 30B at bf16, with one LoRA adapter |
| Cards | one NVIDIA RTX A6000, passed through whole | two NVIDIA RTX A6000, passed through whole |
| Binaries | compiled on the host, carried unchanged into the guest | the same |
| Driver | 620.70.06 on host and guest | the same |
| Host | CachyOS, kernel 7.3.3-1-cachyos | the same |
| Guest | Ubuntu 26.04.1, kernel 7.0.0-38-generic | the same |

The one-card count is recorded at
`experiments/vm-per-agent/template-clone/results/2026-11-12-determinism-slice.md`. The
result note of the two-card count is not in a repository. It is owed here.

**Those counts rest on less than every byte held by hash.** The binaries and the engine
libraries were the same files by digest. The CUDA runtime was a separate install on each
side. The kernel modules were separate builds of one driver version. The C library, the
math library and the C++ runtime were each distribution's own. Nothing was
pre-registered. The host was not run again after the guest.

**They do not cover the image.** The image compiles its binaries in place and carries
the 615 branch of the driver. A run of the sealed image against that host varies the
build site, the driver branch and the system libraries at once.

**The build site is a participant.** A measurement on 2026-11-14 compared repeated
builds from the same commits. Two shipped files differed between builds: the SPU and the
CUDA engine library, the two files that the CUDA compiler produces. The compiled kernels
were identical in every build. With the two settings of section 3.1, every shipped file
was byte-identical across builds. This is the first measurement in the program that
shows the build site to be a participant, and it bears on issue 28. Its result note is
not in a repository. It is owed with the act that adopts the two settings.

**A sealed image was built and proved on 2026-11-15, as the fifth build.** The proof
found SSH as the only listener, every file and link of the manifest in place, no
first-boot seed, no secret, and the held kernel. That image predates the rulings of
2026-11-15. It is a proof artifact and is not version 1. The proof's result note is not
in a repository. It is owed with the Spec of the image build.

### 7.2 What is owed

**`toddwbucy/WeaverTools` issue 28 holds the cells.** Binaries built on each side are
run on each side. The driver branch is varied alone. The sealed image is run against the
host. A respin is run against its parent. Every library is hashed from the running
process on both sides. One cell was added on 2026-11-15: the same image, the same card,
a different virtual CPU model, the same probe set.

**A power cap arm is owed** (ruled in full on 2026-11-15). One card at a time, never
two. The widest lever that the card allows. The order low, then high, then low. The
prediction is written before the run. The result goes into this repository. What exists
is the clock-state result: 2,616 of 2,616 turns reproduced on 2026-09-07 with the
graphics clock locked from 210 to 1890 MHz.

**The order is issue 28's cells, then the cap arm.**

### 7.3 Falsifier

Any difference in the emitted token ids, or in the entropy of any token, on any turn of
the probe set falsifies the claim for the cell that showed it. A failed cell names what
still has to be captured into the binary or held by the image.

## 8. Later items, and what is open

**Later, and named as later.**

- First-boot settings through cloud-init.
- Proxmox, and other hypervisor formats.
- Card architectures beyond the list of section 3.1.
- Any claim across image versions. None is made until a rig exists to test one.
- State and save points on an appliance: how the state mount meets the agent's
  territory. Out of scope until the first agent, which declares no state, runs from the
  console (ruling of 2026-11-15).

**Open, each with who settles it.**

- **The run card's definition.** The run card is the operator's term for a JSON file.
  The rulings of 2026-11-15 record three things on it. The paper that defines it is not
  in a repository as of 2026-11-15. Settled when the operator places it.
- **Whether `toddwbucy/WeaverWeb` also carries the tag of section 5.** The image
  compiles the two connectors from a WeaverWeb commit. Settled by the operator.
- **Where the image is published.** Settled by the operator.
- **How admin's table opens a declared port through the image's boot rules.** Settled
  in the act that adds the port member to the declaration, by the Spec of admin and the
  Spec of the image build together.
- **The respin path.** A respin never recompiles, so it must carry a release's binaries
  into a new operating system layer. No such path is built. Until one is, a security fix
  is a new release. Settled by the Spec of the image build.
