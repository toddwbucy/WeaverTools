# weaver-appliance - PRD (appliance charter)

**Status:** DRAFT. Not merged. It authorizes no Spec and no code until the operator
merges it. It is written from the appliance sketch (v0.3, 2026-11-14) and the operator's
rulings of 2026-11-14.

**Date filed:** 2026-11-14
**Document ID:** `weaver-appliance-PRD`
**Parent:** the WeaverTools suite, as `weaver-web-PRD` is. This charter is not a member
of the weaver-agent domain. It binds one charter inside that domain and one outside it.
**Bound charters:** `weaver-admin-PRD` in `toddwbucy/WeaverAgent`, and `weaver-web-PRD`
in `toddwbucy/WeaverWeb`
**Editorial:** Per the Working Rules. ASCII, absolute dates, prose at 88 characters.
**Landing PR:** #30

---

## 0. What this document is

The charter of the agent appliance. The appliance is one sealed virtual machine image
that carries the WeaverAgent system for one agent, together with the way an operator
brings a machine made from that image under the console. This document states what the
appliance needs and why.

Level discipline, stated once. This document carries requirements, and the substrate
decisions the operator ruled: the operating system, the driver route, the first-boot
loader of settings, and the firewall mechanism. What crosses a seam is a contract's. How
a requirement is represented is a Spec's, and that covers port numbers, file formats,
time limits and command names. Each section ends with what it leaves open for a Spec.

Authority. Every requirement here is the operator's ruling of 2026-11-14 unless the
sentence says otherwise. Three sources carry those rulings, in descending order: the
operator's written brief of that date on the appliance path, his rulings later that day,
and the appliance sketch, whose sections he ratified. A sentence that states a proposal
says that it is a proposal. Section 12 lists what the operator has not ruled.

**Five terms carry one meaning each in this document.**

- The **image** is the sealed disk image. One image exists for each version, and it is
  published with its hash.
- An **appliance** is a virtual machine that an operator made from the image.
- The **console** is the Weaver Web server, the operator's management surface for many
  agents.
- The **agent** is the bounded thing: the processes of one WeaverAgent that run under
  their own UIDs.
- The **WeaverAgent system** is everything installed to run and to bound that agent:
  admin, the gate, the harness, the SPU, state management, and the two connectors.

This document never uses "the agent" for a component that holds a privilege. It names
the component.

```graph
node: weaver-appliance
kind: artifact

edge: parent
from: weaver-appliance
to: WeaverTools
```

The block is a proposal, per section 12.

## 1. What the appliance is

**A sealed image that carries the WeaverAgent system for one agent and nothing else.**
An operator installs Weaver Web, takes the image, loads it into a hypervisor they
already run, and makes a virtual machine from it. From then on the operator manages the
agent service through the console and manages the machine through the hypervisor. The
console knows the agent. The hypervisor knows the machine. Neither reaches into the side
of the other.

**It exists to hand an operator a held deployment tuple.** The output of a turn depends
on what computes it: the binaries, the libraries they load, the driver, the model bytes
and the declaration. An image holds the first three as one hashed object. An operator
who runs a version of the image then runs the same bytes as every other operator who
runs that version. Section 11 states what has been measured about this claim and what is
still owed.

**The agent is bounded by the operating system under a UID.** The appliance does not
give the agent an operating system. It gives the WeaverAgent system a machine with
nothing else on it. The agent stays bounded inside that machine by its accounts and
permissions, as it is on any install. A privilege that an operator gives the agent
beyond what the WeaverAgent system needs is the operator's choice, and it is outside
this charter.

## 2. What it is not

**Not a hypervisor driver.** Weaver Web never touches a hypervisor. It does not create,
clone, start, stop, snapshot, network or destroy a virtual machine. Those acts belong to
the operator and their hypervisor, and the suite holds no opinion about them. To start
and stop the agent is to load and unload it through admin, inside a running appliance.
The earlier direction, in which the console drove the lifecycle of a virtual machine, is
withdrawn.

**Not an installer.** Nothing is installed into an appliance after its image is sealed.
No site hook exists. A site condition, such as a limit that one operator sets on their
own cards, is a unit that the operator adds to their own machines. The image never
learns of it.

**Not a rule about deployment.** The shape of a deployment is the operator's choice. The
suite recommends a machine given to one agent: a single-use machine of the Spark and
GB10 class, or a virtual machine. The reason is the boundary of section 1 and the
firewall table of section 7. The operating system of the machine is the whole boundary
of the agent, so the machine should be one that nothing else depends on. This charter
states a recommendation and no rule. In the operator's words: "we just bring the board
and the wires".

**Not a tutorial.** The entry condition is that the operator already runs a hypervisor
with GPU passthrough working. It is a condition. The suite does not teach it.

## 3. The image

### 3.1 Contents and build

**The operating system is Ubuntu 26.04.**

**The binaries are compiled inside the image from named commits.** After the build, the
build tooling and the source trees are removed and the image is sealed. Two reasons
hold. The image then states by commit what code it carries. The binaries are built
against the libraries they run with.

**A commit does not yet name every byte of an image.** Repeated builds from the same
commits give byte-identical files, with two exceptions: the SPU and the CUDA engine
library, which are the two files that the CUDA compiler produces. A measurement on
2026-11-14 found two causes. The compiler puts its own process id into the names of its
temporary files, and those names reach the symbol tables of both files. The compiler
also emits intermediate code that differs from run to run, in 9 of the 264 intermediate
code images of the engine library. The compiled kernels are identical in every build,
for every card architecture that the library targets. So the code that a targeted card
runs does not differ between builds. Until the build changes, the identity of an image
is its own manifest and its hash, and a rebuild from the same commits does not confirm
an image. Section 12 carries the election that would close this.

**The image states the commits it was compiled from**, where a reader can find them
without the build log. WeaverAgent also records, for each released version of the image,
the commit that the version was compiled from. How WeaverAgent records it is open, per
section 12.

### 3.2 The driver

**The NVIDIA driver is the distribution's own prebuilt and signed kernel module of the
615 branch**, with the kernel held to match it. No compiler runs near the driver, and
the module loads under Secure Boot. The driver version is a member of the deployment
tuple, and the image states it.

### 3.3 Sealed

**Nothing changes an image after its seal.** Automatic package updates and firmware
updates are off.

**No shared key or secret ships in the image.** An image key was once proposed and is
dropped. The pairing line and the console's fingerprint do its work without a secret
that every copy of the image carries.

### 3.4 Versions

**One image exists for each version.** A security fix is a new version. Each version is
tested and evaluated before its release, which is one reason the appliance model was
chosen.

**Every published version stays published with its hash.** The deployment tuple of an
agent names its image version, and a replay needs that image.

**An upgrade of an appliance is a change of the deployment tuple and is recorded as
one.** State that was made under an older version is recorded as made under it and, from
the load onward, as used under the newer version.

### 3.5 Thin

**The image carries no model and no state.** Both live on disks that the operator
attaches, per section 6.

### 3.6 Formats and distribution

**One image comes first, for KVM.** Proxmox is out of scope for now, and it is the first
format named as later. `toddwbucy/WeaverTools` issue 29 records one Proxmox condition to
settle then. Other hypervisor formats are later and are named as later.

**The console offers the image for download.** It checks the hash of the image against
the published hash before it hands the file over.

### Open for spec

The cleanup list of the build, and the thread counts of the build and of the sealed
image. Where the image states its commits, its driver version and its own version. The
version scheme.

## 4. Getting in

**SSH is the one way in, and no fallback exists.**

**The appliance listens on SSH alone.** No other port is open unless a declaration names
it, per section 7. The operator's rule: where a port does not have to be open, it is not
open.

**First-boot settings come through cloud-init, and cloud-init is limited to three
settings:** the operator's SSH key, the network settings and the hostname. Cloud-init
runs no command and installs no package. Either act would make the sealed claim false at
the first boot. The image holds this limit itself and does not rely on a setting of the
hypervisor.

**Cloud-init prints the SSH host key fingerprints on the console of the machine at first
boot, and that print is kept.** It gives the operator a value to compare at the first
SSH, per section 5.6.

### Open for spec

Which cloud-init modules run. The account that receives the operator's key, and what
that account may do.

## 5. Pairing and identity

### 5.1 The direction of the connection

**The console begins pairing, and the appliance makes the connection.** The appliance
never listens for the console. It dials the link port of the console. The console is a
network service, and to listen is its work.

### 5.2 The pairing line

**The console shows one line for the operator to run in the appliance over SSH.** The
line carries the address of the console, a one-time secret and the certificate
fingerprint of the console. The appliance dials the link port, checks the fingerprint,
presents the secret, hands over the public halves of keys that it made itself, and
receives its certificates. From then on every connection between the two is mutual TLS,
and the secret is spent.

### 5.3 The one-time secret

**The one-time secret is the only secret that travels on a command line.** It is the
exception because no other way to begin exists. Three bounds hold it. It is single-use.
It is short-lived, on the order of ten minutes. It is bound to one pending enrollment
that the operator opened, so it cannot enroll an appliance that the operator did not
expect. No other secret travels on a command line.

### 5.4 Keys

**Each connector makes its own private key inside the appliance, and the key never
leaves it.** The console signs the public half. Each appliance is meant to hold its own
certificate. Section 5.6 states the one case that breaks this, a machine cloned after
enrollment.

### 5.5 The unauthenticated surface

**The enrollment connection is the one unauthenticated surface of the console**, and the
console's documents name it as one.

### 5.6 Two hazards

**Trust on first use.** The first SSH of the operator into an appliance trusts the host
key that it meets. A wrong machine at the given address could be enrolled. The operator
compares the host key with the fingerprints that cloud-init printed at first boot.

**A cloned identity.** The console no longer makes machines, so an operator can clone a
configured appliance and produce two machines with one identity. The console refuses a
second live holder of a certificate that it issued.

### Open for spec

The lifetime and the form of the one-time secret. The default of the link port, which
the operator can change. The operator suggested the range 6000 to 7000. What the console
does with the appliance that connected second.

## 6. Storage

**Two mount points are fixed: `/mnt/weaver/model` and `/mnt/weaver/state`.**

**The state disk is always required.** Every agent writes a trace. The state disk holds
the trace, the state store and the save points.

**The operator attaches the disks in the hypervisor and mounts them over SSH.** The
WeaverAgent system only checks and reports. It checks that each mount point is a
separate filesystem. It reports the sizes to the console. Where storage is not
sufficient, the console says so and asks the operator to add storage.

**The check runs at every load, not once at enrollment.** A state mount that went
missing between two loads would otherwise put the trace into the sealed root.

**State that a newer version cannot read refuses the load, and the refusal says why.** A
load never starts fresh silently.

### Open for spec

Which component makes the check, and the form of its report. The sizes that count as
sufficient.

## 7. The firewall is part of the declaration

**Admin owns one named nftables table and writes only that table.** It never owns the
whole firewall.

**A declaration that names a tool or exposes the gate names the ports.** Admin opens
those ports in its table at load and closes them at unload.

**The open ports and the rules of the table are recorded with the run**, on the run
card, as a condition of the arrangement.

**The gate is a door that the declaration names and admin opens at load.** It is not a
surface that the image exposes.

**One property of the mechanism binds the design.** In nftables an accept in one table
does not override a drop in another table. So the table of admin opens a port only where
nothing else on the machine drops it. On an appliance this binds the image: the default
policy of the image must sit where the table of admin can open a declared port. Section
13 names the open cell. On a machine where the operator runs other rules, those rules
must also allow the port, and that is the operator's act. This is the technical reason
behind the recommendation of section 2.

### Open for spec

The name of the table and the shape of its rules. How a declaration names ports. The
member of the run card that carries them.

## 8. Models

**The console fetches models, and the appliance never reaches the internet.** The
console offers four sources: a Hugging Face search limited to what the engine supports,
a local store, an S3 bucket, and a list of recommended models that are known to work.

**Every fetched model is pinned by exact revision and digest, and it is verified after
download.**

**Access tokens live in the store of the console and never in an appliance.**

**The console keeps what it fetched, for other agents to use.** The appliance pulls a
model from the cache of the console over the same mutual TLS trust, and it checks the
digest. This keeps the rule that the appliance only dials out.

**The cost is stated.** The console becomes the model store. On a network larger than
one site, that is a question of disk and of bandwidth that the console has to answer.
Section 13 carries it.

### Open for spec

The layout of the cache. The transfer, and how an interrupted transfer continues.

## 9. Behaviour

**After a reboot the connectors come up and the console sees the appliance. The agent
stays unloaded until the console loads it.** No autostart exists.

**Load and unload are the verbs of admin, as built**, per `weaver-admin-PRD` section 4.
The console asks for them through the admin connector.

**An upgrade is recorded as section 3.4 states.**

## 10. Asks upstream

Each ask below is a gap in the documents of another repository. Each becomes an issue on
that repository, one for each interface question. None is an edit from this document.

| Repository | Document | What this charter asks |
|---|---|---|
| WeaverAgent | `weaver-admin-PRD` and its Spec | The one named firewall table, opened at load and closed at unload (section 7). The storage check at every load (section 6). The refusal of state that this version cannot read (section 6). |
| WeaverAgent | the declaration's grammar | How a declaration names the ports of a tool and of the gate (section 7). |
| WeaverAgent | the run record | The member that carries the open ports and the rules of the table (section 7). The record of an upgrade (section 3.4). |
| WeaverAgent | the repository itself | The record of the commit behind each released version of the image (section 3.1). |
| WeaverWeb | `weaver-web-PRD` and its Spec | Enrollment on the link port, as the one unauthenticated surface, with keys made in the appliance and signed by the console (section 5). The refusal of a second live holder (section 5.6). The model store and its four sources (section 8). The image download with its hash check (section 3.6). The storage report (section 6). |
| WeaverWeb | the connectors | The pairing line. A connector that only dials out (section 5). |
| WeaverTools | this repository | The build of the image (section 3). The measurements of section 11. |

## 11. Evidence, and what is owed

### 11.1 What is measured

Two measurements exist. Each is a count with its tuple. Neither is a rate.

| Member | One card, 2026-11-12 | Two cards, 2026-11-13 |
|---|---|---|
| Count | 464 of 464 turn slots identical | 86 of 86 turns identical |
| Model and precision | qwen2.5-0.5b-instruct at q6_k | Granite 4.2 30B at bf16, with one LoRA adapter |
| Cards | one NVIDIA RTX A6000, passed through whole | two NVIDIA RTX A6000, passed through whole |
| Binaries | compiled on the host, carried unchanged into the guest | the same |
| Driver | 620.70.06 on host and guest | the same |
| Host | CachyOS, kernel 7.3.3-1-cachyos | the same |
| Guest | Ubuntu 26.04.1, kernel 7.0.0-38-generic | the same |

The one-card measurement is recorded in this repository at
`experiments/vm-per-agent/template-clone/results/2026-11-12-determinism-slice.md`. The
result note of the two-card measurement is drafted and is not yet in a repository. It is
owed here.

### 11.2 What those counts rest on

They rest on less than every byte held by hash. The binaries of the WeaverAgent system
and the engine libraries were the same files by digest. The CUDA runtime was a separate
install on each side, compared by a 16-digit prefix of its digests. The kernel modules
were separate builds of one driver version. The C library, the math library and the C++
runtime were each distribution's own. Nothing was pre-registered. The host was not run
again after the guest.

### 11.3 What they do not cover

The image of this charter compiles its binaries in place and carries the 615 branch of
the driver. So the measured pair matches the image in its distribution, and it differs
from the image in build site and in driver branch. A run of the sealed image against
that host varies three things at once: the build site, the driver branch and the system
libraries.

### 11.4 What is owed

`toddwbucy/WeaverTools` issue 28 holds the measurement and its cells. Binaries built on
each side are run on each side. The driver branch is varied alone. The sealed image is
run against the host. A respin is run against its parent. Every library is hashed from
the running process on both sides. The math library is the first suspect: the SPU
computes entropy and surprisal with it on the CPU, and the samplers of the engine call
it there too.

**A power cap arm is also owed.** One card at a time, never two. The same probe set runs
at two caps with everything else held. Its result goes into this repository, as the
clock-state result did. The clock-state result is what exists today: 2,616 of 2,616
turns reproduced on 2026-09-07 with the graphics clock locked from 210 to 1890 MHz. No
cap arm has been run.

These measurements come before a Spec relies on the claim of section 1.

### 11.5 Falsifier

Any difference in the emitted token ids, or in the entropy of any token, on any turn of
the probe set falsifies the claim for the cell that showed it. A failed cell is not a
setback. It names what still has to be captured into the binary or held by the image.
The operator's hypothesis: capture into the binaries all that affects the arithmetic,
and hold the rest in place with the image.

### 11.6 The card

A different card is a different distribution. The appliance holds the deployment tuple
only where the operator holds the card.

## 12. Open rulings

These are the operator's to rule. Each states the proposal on the table.

1. **The parent and the node of this document.** Proposed: the parent is the WeaverTools
   suite, and the node is `weaver-appliance` of kind `artifact`, as the block in section
   0 states.
2. **Where this document lives.** Proposed: `docs/` in the suite repository, since the
   charter binds two repositories.
3. **How WeaverAgent records the commit of each released version.** Proposed by the
   WeaverAgent Executor: one annotated tag for each version, on the commit that the
   version was compiled from, pushed by the operator. The tag message carries the
   version, the digest of the install record of the image, and the test count.
4. **The version scheme of the image.**
5. **Three design points of the power cap arm.** Proposed: the widest lever that the
   card allows, the order low, high, low, and the prediction written before the run.
6. **Whether the image is built to be reproducible from its commits.** Two build
   settings, measured together on 2026-11-14, make every shipped file byte-identical
   across builds. The first is a compiler flag that fixes the names of the temporary
   files. The second is a list of card architectures that carries compiled kernels only
   and no intermediate code. The cost of the second: a card whose architecture is not in
   the list has no fallback and cannot run the engine. The list that was measured holds
   the architectures sm_86, sm_89, sm_120 and sm_121. Proposed: adopt both settings for
   the image, because section 3.4 and the first cell of section 13 both need a rebuild
   that can be compared with its parent.

## 13. Open cells, each with what settles it

- **Build once, respin many.** If every security respin recompiles, every respin changes
  the binary hashes, and a patch cannot be told from a change of behaviour. The
  candidate rule is to build once for each release and to carry the same binaries into
  its respins. The operating system layer of a respin is not inert by construction: a
  fix to the C library replaces the math library, and a fix to the kernel brings a new
  build of the driver module. Settled by one respin: compare the binaries by digest and
  the output by the probe set.
- **The CUDA objects across builds.** Section 3.1 states what was measured. One cause
  stays open: why the intermediate code differs from run to run where it is kept. No
  setting removed it. It matters only if the election of section 12 keeps that code.
- **The sealed-image run.** Section 11.4. Settled by the cells of issue 28.
- **Which table holds the default policy on an appliance.** Section 7 leaves two shapes.
  In the first, the image ships no table that drops input, and the table of admin
  carries the drop policy and the SSH accept at all times. In the second, the image
  ships a base table that hands the declared ports to admin, for example as a set that
  admin fills. The first matches "admin owns one table". The second gives admin a part
  of a table that it does not own. Settled by the Spec, with one test of each shape on a
  sealed image.
- **Trust on first use.** Settled by one first boot of the sealed image, with the
  printed fingerprints read from the console of the machine.
- **Clone detection.** How the console notices a second live holder of a certificate,
  and what it does with the appliance that connected second. Settled by the Spec of the
  console.
- **A stopped service against a machine that is down.** What the appliance exposes so
  that the operator can tell the two apart. Settled when the first stop is run from the
  console.
- **The console as the model store.** The model count or fleet size at which its cache
  stops being tolerable. No measurement exists, because the fleet is one.
- **The time source.** Section 8 says that the appliance never reaches the internet. The
  stock time service of the operating system points at a public time pool. Which source
  an appliance takes its time from is not ruled. Settled by the operator's ruling.
- **Hypervisor formats after KVM, and where images are published for the console to
  find.** Later, and named as later.
- **The GPU architectures that the stack inside the image is compiled for.** The
  election of section 12, item 6, sets this list for the first image. Further
  architectures are later, and named as later.

One open cell of the brief is retired. The brief asked whether a pairing code is typed
or scanned. The pairing line of section 5.2 replaced the pairing code.

## 14. Children

None yet. Once this charter is merged, three Specs take their guidance from it: the Spec
of the image build in this repository, the enrollment and model store additions to the
Spec of Weaver Web, and the load additions to the Spec of admin. Each is opened by its
own repository after the matching ask of section 10 is filed.
