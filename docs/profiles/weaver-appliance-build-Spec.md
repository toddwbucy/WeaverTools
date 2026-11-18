# The appliance image build - Spec

**Status:** Merged with release 1.0 (pull request 33, 2026-11-16); revised for release
2.0 in pull request 35, not merged. Moved on 2026-11-15 from section 10 of
`template-clone-Spec`, where it stood on pull request 31.
**Date filed:** 2026-11-15
**Document ID:** `weaver-appliance-build-Spec`
**Parent:** `weaver-appliance-profile` (this directory), whose own artifact this build makes
**Gated on:** `toddwbucy/WeaverTools` issue 28. This is the one artifact of the profile gated
on it.
**Editorial:** Per the Working Rules. ASCII, absolute dates, prose at 88 characters.
**Landing PR:** the `act/appliance-v1` stack (release 1.0, merged as pull request 33);
the `act/appliance-arch` stack (release 2.0, pull request 35)

---

## 0. What this document is

The Spec of the appliance image's build. It covers the scripts that make, seal and prove
one image: `build-template.sh`, `guest/template-steps` and the helpers of `lib.sh` they
use, all in `experiments/vm-per-agent/template-clone/code/`. It represents the build and
nothing else, as `deployment-profile-convention` section 3 allows. It states no
requirement on admin, the console or the agent.

The operator ruled on 2026-11-15 that the image build has its own Spec under its profile.
This document took section 10 of `template-clone-Spec` whole, and the build's rows of that
Spec's parameter table. The scripts stay where they were. What the build shares with the
template and clone scripts stays stated in `template-clone-Spec`: `VM_ROOT`, `KEY` and
`CPUSET` (its section 1), the guest's host key (section 2), the boot gate (section 3), the
guard (section 4) and the offline mock (section 8).

```graph
node: weaver-appliance-build-Spec
kind: method

edge: parent
from: weaver-appliance-build-Spec
to: weaver-appliance-profile
```

## 1. Parameters

**No fact about a host is committed.** `VM_ROOT`, `KEY` and `CPUSET` are as
`template-clone-Spec` section 1 states. The build's own:

| Parameter | What it names | Default |
|---|---|---|
| `OS` | the appliance image's operating system: `arch-20261115`, the arch-boxes cloud image v20261115.625457, its packages brought to the Arch Linux Archive's snapshot of 2026-11-15. `build-template.sh` refuses any other. `lib.sh` keeps `26.04`, release 1.0's base, for `clone.sh` | none for `build-template.sh create` |
| `VERSION` | the image's version, `<release>.<respin>`. This tree builds a release, `<release>.0`, and refuses a respin (section 2) | none |
| `DRIVER`, `DRIVER_ROUTE` | the driver release the image carries and the route it comes by | none |
| `COMPUTE_CAP` | the compute capability the image's CUDA kernels are compiled for, without its dot | none |
| `WA_COMMIT`, `WW_COMMIT` | the WeaverAgent and WeaverWeb commits the image is compiled from, each in full | none |
| `NAME`, `PORT` | the build guest's domain name and its SSH forward's host loopback port | `weaver-appliance-build`, 2230 |
| `VCPUS`, `MEMORY`, `SIZE` | the build guest's vCPUs, memory in MiB and disk | 24, 65536, 120G |
| `SEAL_WAIT` | the seconds the host waits for the guest's final seal step to power it off | 1200 |
| `PROOF_BOOT` | the seconds `proof` lets the overlay of the sealed image run with its frames captured | 180 |
| `PROOF_PORT` | the host port `proof`'s first-boot overlay forwards SSH on, bound to the host's loopback (clear of `clone.sh`'s default port). The next port, `PROOF_PORT` + 1, forwards the inbound probe to the guest's TCP 8080, bound the same way | `PORT` + 100, and + 101 |
| `PROOF_FIRST_BOOT` | the seconds `proof` waits for the first boot's login prompt on the console | 300 |
| `PROOF_ACPI` | the seconds `proof` waits, after it presses the first boot's power button, for the guest to power itself off | 120 |
| `CUOBJDUMP` | the host's `cuobjdump`, from a CUDA toolkit that knows the four architectures `proof` checks for (section 2) | `cuobjdump` |
| `QEMU_USER` | the account QEMU runs as under libvirt's system connection. `proof` grants it alone, by a POSIX ACL, write access to the console and capture files QEMU opens, to the directory of the first boot's console socket, and to the overlays it boots | `libvirt-qemu` |

A step that needs a parameter with no default refuses before it does anything.

## 2. The build

`build-template.sh` builds the appliance image: one sealed qcow2 that an operator loads into
their own KVM hypervisor, never booted again as itself. KVM first. Proxmox is out of
scope (WeaverTools #29). On the operator's rulings of 2026-11-14 the binaries are compiled
inside the image's own build guest from two named commits, in place, and the guest is then
cleaned by a checked list and sealed. Nothing built on the host enters the image.

Release 2.0 builds on Arch Linux (the operator's ruling of 2026-11-16), pinned: the
arch-boxes cloud image `OS` names, its packages brought to one day of the Arch Linux
Archive and never past it. It is release 2.0 under the operator's decision 4 of
2026-11-15: an OS change with a recompile moves the release. Release 1.0 was built on
Ubuntu 26.04. Its build tree is WeaverTools `main` at `06d42bd` (the merge of pull
request 33), whose copy of this document is its Spec, and its stack is WeaverAgent at
the tag `appliance/1.0`.

**Inputs, all named.** `OS` (the cloud image by digest, section 1), `VERSION`, `DRIVER`
and `DRIVER_ROUTE`, `COMPUTE_CAP`, `WA_COMMIT` and `WW_COMMIT` in full. `create` refuses
unless all seven are set. The cloud image comes with its `.SHA256` file, whose signature
must be made under the Arch Linux image signing key, pinned by its primary key's
fingerprint, and the file must carry the pinned digest alone. The key's signing subkey
expires on 2027-05-28, so an image pinned later needs the key read again. `COMPUTE_CAP`
is the compute capability the CUDA kernels are compiled for, without its dot. The build
guest has no card to read it from, so it is named, and the stamp records it. `DRIVER`
has one form on the host and in the guest, one regex in both scripts (a release such as
`620.70.06`), and `DRIVER_ROUTE` is `arch-prebuilt`, the one route this tree builds.
Each guest step is given those that are set on its command line, and a step that needs
one refuses where it is unset. The build guest has 24 vCPUs and 64 GiB by default. The
domain a box later runs under is the operator's own.

**The version** (the operator's decision 4 of 2026-11-15). `VERSION` is
`<release>.<respin>`, each a number, the release from 1. The release moves when the
binaries move. The respin moves when only the OS layer moves. A respin never recompiles,
and if the binaries change for any reason, the release moves. This tree always compiles
the binaries in the build guest, so it builds a release alone, `<release>.0`. A respin is
refused on the host with a sentence: a respin carries a release's binaries into a new OS
layer, and no path here does that (section 3). Every other form is refused too. The facts
step records the version in the stamp as `appliance.version`, and the guest refuses a
`VERSION` that is not a release.

**The tag** (the operator's decision 3 of 2026-11-15). Each released version is named in
WeaverAgent by one annotated tag, `appliance/<version>`, on the WeaverAgent commit the
version was compiled from. The operator makes the tag and pushes it. It is keyed by the
version, not by the commit. `build-template.sh tag` prints the tag's name, its commit
and its message for the operator, and writes the message to the build's log for
`git tag -F`. It applies nothing and runs no git command. The message carries the
version, the digest of the install record (`/etc/weaver/stack/install-tuple`, as written
at image build) and the stack's test count. Each is read from the stamp that `proof`
read out of the sealed image (`appliance.version`, `stack.install-tuple`,
`stack.tests`), as is the commit (`weaveragent.commit`). `VERSION` and `WA_COMMIT`,
where named, must equal the stamp's. `tag` refuses unless a proof has passed for the
sealed file the seal recorded, and unless the file on disk, hashed again, still has that
digest, so the message never describes a file changed since its proof. A proof that
passes writes a record naming the sealed file's sha256 and the MANIFEST's, and each
proof removes the record when it starts. The
test count is the count of WeaverAgent's tests that `bootstrap-stack.sh` runs at its
test step in the build guest. A run that passes removes that step's own log, so the
stack step keeps bootstrap's output as it streams, records the count in the stamp as
`stack.tests`, and refuses unless the test step printed one count with no test failed
(step 8). The fifth build's stamp names neither a version nor a test count, so `tag`
refuses it. Checked on 2026-11-15 on the measuring host with stand-in records, and
against the fifth build's kept logs: its stack log reads 2867 passed, 0 failed. The
record build of release 2.0, on 2026-11-17, wrote its message: version 2.0, and 2869
passed, 0 failed.

**The ggml device code: four architectures, no PTX** (the operator's decision 6 of
2026-11-15). WeaverAgent's deploy scripts, from its commit `1f6ef77e`, build the stack
with two settings: nvcc names its temporaries after each object rather than after its
process id, whose digits otherwise land in the objects' symbol tables
(`NVCC_APPEND_FLAGS` with `-objtemp`), and ggml's CUDA kernels are compiled as cubins
for four architectures with no PTX (`CMAKE_CUDA_ARCHITECTURES`
`86-real;89-real;120a-real;121a-real`). The build sets nothing for this: the stack step
runs those scripts unmodified. The architectures are named `sm_86`, `sm_89`, `sm_120a`
and `sm_121a`. A card whose architecture is off the list gets no ggml device code, and
the gguf engine does not run on it. weaver-spu's own kernels keep their `compute_86`
PTX. The settings reach a fresh target only, since the engine and kernel builds do not
rerun on them, and the image build is one: its target is made new in the build guest.
The proof checks the shipped library. It follows `/opt/weaver/lib/libggml-cuda.so.0`
through its links in the image, copies the file out, and lists it with the host's
`cuobjdump` (`CUOBJDUMP`). Any PTX image fails the proof, as do any of the four cubin
sets absent, a different count of cubins for one of the four than for the rest, a cubin
for any other architecture, a file that cannot be read and a listing that cannot run. So
does a listing line in a form the check does not read, and a PTX listing that names no
PTX image without cuobjdump's own line saying it found none. cuobjdump 13.4.92 prints
`No PTX file found` then (measured on 2026-11-15 on the measuring host). The count is
not fixed, and the record names each architecture's: an object compiled for fewer
architectures than the rest would leave some cards without its kernels. On 2026-11-15
the check passed stand-ins made with nvcc 13.4.92 on the measuring host, failed
stand-ins with PTX, a missing architecture, an extra one, or one object compiled for
three of the four beside one compiled for all four. It failed the fifth build's kept
image, read-only, which was compiled before the change, on its PTX alone: it carries 132
PTX images each for `sm_75` and `sm_80` beside the four cubin sets of 132 each. The
record build of release 2.0 passed it on 2026-11-17: 132 cubins for each of the four,
and no PTX. The measurement behind the decision is WeaverAgent's, at `1f6ef77e` on
2026-11-15, on nvcc 13.4.92: two fresh builds into one target path gave all 13 shipped
files equal by sha256, and `libggml-cuda` with no PTX and 132 cubins for each of the
four. The image's toolkit carries the same nvcc, 13.4.92, as Arch's `cuda` 13.4.3-1
(`guest/template-steps` pins it, and the stamp records it). A rebuild confirms those
shipped binaries only. This Spec does not claim the whole image reproducible. It is the
first measurement in the program to show the build site taking part in what is built
(the process id in the symbol tables), which bears on WeaverTools#28.

**The network.** The build guest uses QEMU user-mode networking, as the template did
(`experiments/vm-per-agent/template-clone/README.md`, "The network"), with the seed's
guard, which the facts step requires loaded before anything is installed. The sealed
image configures no network: no file under `/etc/systemd/network` (the cleanup removes
the one cloud-init wrote for the build guest), none of systemd's own `.network` files
matching an Ethernet name, no NetworkManager connection, and no resolver address.
resolved gets a drop-in, `/etc/systemd/resolved.conf.d/10-weaver-appliance.conf`, as
release 1.0 had it where Arch's compiled defaults differ: `FallbackDNS=` empty, since
Arch's build falls back to public resolvers, and LLMNR and multicast DNS off, both on in
Arch's build, where LLMNR listens on every address and the image's listeners beyond
loopback are SSH alone (ruling 4). The initramfs, made again at the cleanup, holds no
networkd, and a probe reads it. systemd 262's cloud metadata service is masked with its
generator: the generator pulls the service in where the machine's DMI identity names a
cloud vendor udev's hwdb lists, and the service then
reads metadata over the network. An operator's hypervisor can present such an identity,
so the service is not shown inert, and is masked. The record build's boot with no seed
sent no frame (step 12). The network is the one the operator sets after logging in
(decision 16 of 2026-11-15). timesyncd stays enabled with the servers it is built with,
unchanged here, so the clock syncs where a server answers. `systemd-time-wait-sync`,
which the image's builder enables, is masked: where no time server answers, it holds
`time-sync.target` and the boot never finishes (the 2.0 rehearsal's proof of
2026-11-17).

**The base inbound policy: drop, but SSH** (the operator's decision 9 of 2026-11-15).
The image's own boot rules drop inbound traffic and accept SSH, with no agent loaded.
The cleanup writes one table of the image's own, `inet weaver_appliance`, to
`/etc/nftables.conf`, in place of Arch's shipped file, and enables Arch's
`nftables.service`, which is to load it before the network comes up (by the unit's own
ordering, not timed on a boot of this image). The table's input chain has policy drop
and accepts four things: loopback, replies (connection state established or related),
the ICMPv6 neighbour-discovery types without which IPv6 does not work (neighbour
solicitation and advertisement, router advertisement), and TCP 22. It has no output
chain, so outbound is not limited (decision 17). The file opens by destroying the
image's own table and never flushes the ruleset, so a load, a reload or a second load
replaces that table and leaves every other table as it stands. Arch's unit (nftables
1.1.7) is a oneshot with no stop command and no `RemainAfterExit`, so nothing removes
the table at a stop, and no stop command may be added: on such a unit systemd runs
`ExecStop` as soon as the start exits. Release 1.0's drop-in, which made Ubuntu's stop
remove this table alone, is not carried. The text is defined once in
`guest/template-steps`, and a probe sets the file against it and reads the unit,
drop-ins and all, loading `/etc/nftables.conf` with no stop command. The rules are
enabled for the next boot and not loaded in the build guest. Checked on 2026-11-15 in a
user namespace on the measuring host (nftables 1.1.7): the file loads twice, and a
second table survives both loads. Measured on the record build's first boot (step 12):
the table loaded, with its input policy drop and its SSH accept, a listener on TCP 8080
silent from the host, and SSH answering through its forward. Admin's table is separate
and is admin's. In nftables a drop in one table's base chain is final, so an accept in
admin's table cannot open a port through this one (section 3). An address by DHCPv6 is
not provided for: a DHCPv6 server's reply does not match the multicast solicitation as a
reply, so the policy drops it. Multicast listener queries (MLD) are dropped too, so on a
switch that snoops MLD, neighbour discovery can lapse. Both are read from the rules and
not measured.

**The first login: root, set at the console** (the operator's ruling of 2026-11-16). The
image carries no secret. Root's password field is empty and its last change is 0 (Arch
ships it locked, `*`), so at the console `login` asks root for no password and requires
a new one before the shell. After that the operator places a key and sets the network
from the root shell (decision 16 of 2026-11-15). SSH admits keys alone. The cleanup
writes `/etc/ssh/sshd_config.d/10-weaver-appliance.conf` with `PasswordAuthentication
no`, `KbdInteractiveAuthentication no`, `PermitEmptyPasswords no`, `PermitRootLogin
prohibit-password`, `AuthorizedKeysFile .ssh/authorized_keys` and `AuthorizedKeysCommand
none`. Arch's `sshd_config` includes that directory's files, in name order, before its
own settings, and sshd keeps the first value it reads for each keyword, so this file,
first by name, decides all six. systemd-tmpfiles lays a second drop-in at every boot,
`20-systemd-userdb.conf`, which asks `userdbctl` for a user's keys. Keys come from
root's `authorized_keys` alone, which setup writes from Weaver Web (the operator's
addendum of 2026-11-16, as the suite Planner recorded it), so the cleanup removes that
link and masks its tmpfiles line. The text is defined once in `guest/template-steps`,
and a probe reads sshd's own merged
configuration for root. So the first password is set at the console, never over SSH.
Every other account stays locked as Arch ships it, and none has a login shell. The image
ships no cron. Measured by the record build's proof on 2026-11-17 (step 12):
- At the serial console `login` asked root for no password, announced the forced change,
  asked the new password twice and gave a root shell.
- Over SSH, before the change and after it, an empty password, the password the console
  set and a wrong one were each refused: sshd offered `publickey` alone. A key root did
  not yet hold was refused too.
- sudo ran a command as nobody, as root, after the first login: what `create-agent.sh`
  asks of sudo (WeaverAgent #581). Before that login sudo's account check refuses even
  root, whose password is expired.

On release 1.0's base, and not yet measured on this one: a key in root's
`authorized_keys` before the first login authenticated over SSH, and sshd then required
the change. With no terminal it refused ("Password change required but no TTY
available."), and with one (`ssh -tt`) it made the change over SSH. The image ships no
key, so the change is at the console unless the operator feeds a key in first
(systemd's first-boot paths, below).

**The data mount points, laid empty.** The profile fixes two mount points,
`/mnt/weaver/models` and `/mnt/weaver/state` (`weaver-appliance-profile` section 3.4).
The models mount is plural by the operator's rulings of 2026-11-16 on the setup pages,
which supersede the singular of 2026-11-14. The operator attaches the disks in the
hypervisor and chooses each at the appliance's console, where the appliance formats and
mounts it once the operator confirms (the same rulings; a later act). The cleanup lays
both, and `/mnt/weaver`, as empty directories owned by root with mode 0755. They are
defined once in `guest/template-steps`, which `lib.sh` reads for the proof. The image
carries no model and no state, so a probe requires each mount point empty, on the
image's own filesystem, and `/mnt/weaver` to hold the two alone. The proof's first boot
reads them with no disk attached. They are not in the MANIFEST: its scope is `/usr`,
`/etc`, `/opt`, `/boot` and `/efi`, and it lists regular files and links only. The
stack's agent directory is the state mount, `/mnt/weaver/state` (the operator's rulings
of 2026-11-16 on the setup pages): the stack step hands it to `bootstrap-stack.sh`,
which leaves it root's, mode 0755 and empty, and a probe reads the stack record naming
it. Whether a load finds
a separate filesystem on each is admin's test, made at every load
(`weaver-admin-Spec-vm-environment` section 1, merged in WeaverAgent at 6b91747e). It
judges the filesystem and not what lies on it, so an empty directory on the image's root
fails it. As of 2026-11-16 that test is specified and not built, and nothing in this
tree stands in for it. That Spec names `/mnt/weaver/model`, singular, as of 2026-11-16.

**systemd's first-boot paths, left and named** (the operator's ruling of 2026-11-16).
The image keeps systemd's own first-boot paths as Arch's systemd 262 ships them, and
masks none. A probe reads each present and not masked:
- `/usr/lib/tmpfiles.d/provision.conf`, naming the credential `ssh.authorized_keys.root`
  (for `/root/.ssh/authorized_keys`), with no file of that name over it in
  `/etc/tmpfiles.d`
- `systemd-firstboot.service` and `systemd-creds.socket`
- `systemd-network-generator.service`, enabled. It writes a credential named
  `network.network.<name>` to `/run/systemd/network/<name>.network`, where networkd
  reads it. The proof hands its first boot the network this way (step 12).
- `systemd-ssh-generator`, among the system generators

The probes also list every consumer of a system credential in the image, the whole set a
hypervisor can feed: PID 1; each unit whose own settings import or load a credential,
with its enablement and
whether the boot pulls it in; and each generator that reads one, or its mask. A listed
line is a record, never a hit, and none is disabled for being listed. On the record
build they listed PID 1, 28 units and 6 generators, the metadata service's generator
masked.

What an operator feeds through them is the operator's to account for. Two effects are
measured. On release 1.0's base, a key given as `ssh.authorized_keys.root` reached
root's `authorized_keys` before the first login, and the forced change then happened
over SSH, as above. On this base, a network given as a credential brought the network up
at the record build's first boot.

**cloud-init: the build's seed, then removed.** The build guest boots from a NoCloud
seed, which makes the build account, pins the guest's host key and sets its network and
its guard (`template-clone-Spec` sections 2 and 4). The cleanup then disables
cloud-init's units and removes `cloud-init` and `cloud-guest-utils` (growpart, which
only cloud-init runs) by name, the stronger form of the operator's ruling of 2026-11-16
that cloud-init is off. Their leftovers that no package owns go with them: the network
file cloud-init wrote for the build guest, and the empty directory of its units' links.
The seal removes cloud-init's state, its configuration and its logs, and its default
account `arch` with its home and sudoers rule, where a seed ever made it. A probe reads
cloud-init absent in both passes. A seed attached to a box is not acted on, and nothing
grows the root filesystem at a box's boot: it stays at `SIZE`, to which the build's
first boot grew it (read from the steps, not measured). The proof's first boot attaches
a seed only to show it ignored (step 12).

**The fixed layout.** The binaries carry their build paths, so the build account (`op`,
made by the seed), the checkouts (`~/WeaverAgent`, `~/WeaverWeb`) and the targets
(`~/target/weaveragent`, `~/target/weaverweb`) are fixed. The image lays the agent stack
where WeaverAgent's deploy scripts put it (`/opt/weaver`, `/etc/weaver/stack`), the deploy
scripts an enrolled box runs at `/opt/weaver/deploy`, the connectors at
`/opt/weaver-web/bin`, and its own record at `/etc/weaver-appliance`.

**The steps, in order.**

1. `create`: the cloud image, checked against its signed digest file, becomes a
   standalone disk grown to `SIZE`, then the seed, the definition and the boot.
2. `facts`: the base image's package list is recorded before anything is installed. The
   seed's guard must be loaded, since the build guest's network is otherwise open. The
   stamp gets the base image, the snapshot, the version, the OS and kernel, and the
   build's figures.
3. `os-probes`: whether `test` run through sudo honours supplementary groups, and
   whether visudo takes the admin connector rule's `Defaults` line, with a throwaway
   account removed in the same step. Ubuntu 26.04 failed both (uutils' `test`, sudo-rs),
   which is why the operator moved the next release to Arch on 2026-11-16. Arch ships
   GNU coreutils and the original sudo, and the step refuses where either read fails. It
   also refuses unless `/usr/sbin/nologin`, which the deploy scripts name, resolves to
   `/usr/bin/nologin`, and `/etc/shells` names neither. The account and its group count
   as gone only where getent reports them not found (its status 2); a read that fails is
   not an absence.
4. `packages`, `rustup`: pacman pointed at the archive's snapshot alone, its mirror list
   naming the snapshot and `pacman.conf` naming `core` and `extra` alone through that
   list. The system is brought to that day (`-Syyuu`, which also takes a package back
   where the image's is newer), then the build's packages are installed, `quota-tools`
   among them. The kernel must read its pin after the update and be the kernel running:
   an update that moved it would need a reboot the step does not make. Then the
   toolchain both trees pin, by rustup's installer, stamped by digest.
5. `trees`: both repositories cloned and checked out detached at their commits. A tree
   not exactly its commit, clean, refuses, as does a toolchain file that names another
   toolchain.
6. `toolkit`: CCCL 3.3.4-1 first, the one package from outside the snapshot. The day's
   `cccl` is 3.5.0, outside the window WeaverAgent's bootstrap accepts (3.1.4 to 3.3.4),
   so it comes from the archive's
   package store, pinned by sha256, its signature checked against the build guest's
   keyring. Then Arch's `cuda` 13.4.3-1 from the snapshot, whose dependency on cccl the
   pinned package meets. No driver: the stack's build needs nvcc, not a module.
7. `driver`: route `arch-prebuilt`, in place of release 1.0's prebuilt route: Arch's
   `nvidia-open` 620.70.06-3, the open module prebuilt for the snapshot's one kernel
   (`linux` 7.3.4.arch1-2), and `nvidia-utils`
   620.70.06-1, its userspace at the module's version (libcuda, NVML, `nvidia-smi`,
   `nvidia-modprobe`, `nvidia-persistenced`), each package named. No DKMS, kernel
   headers or compiler touches the driver: the step refuses on DKMS or any headers
   package of Arch's kernels, and on any kernel's module build tree. `linux-api-headers`
   is not one of them: it is the kernel's userspace interface, which glibc requires, and
   no module is built from it. Arch signs no module with a key a firmware trusts, so
   Secure Boot stays off and the stamp records the signer as none. The kernel and both
   driver packages are held from any later upgrade by `pacman.conf`'s one `IgnorePkg`
   line, the hold release 1.0 made with apt-mark. The runtime package joins them at the
   cleanup. Nothing updates after the seal; the hold is there for an operator who points
   pacman elsewhere. The step refuses unless one kernel stands, the running one at its
   pin, with `/boot` holding its image, its initramfs and GRUB's directory alone. It
   checks, with no card,
   that each module is present for the kernel, that modinfo reads the release and a
   vermagic equal to the kernel, that modprobe resolves its dependencies, and that every
   firmware file it names is present. It runs before the stack: bootstrap's install
   refuses unless `weaver-spu` resolves `libcuda.so.1`, which the driver carries (the
   first build's break, 2026-11-14).
8. `stack`: WeaverAgent's `deploy/bootstrap-stack.sh --install`, unmodified, with nvcc's
   host compiler `g++-15` (as release 1.0 compiled; Arch's base g++ is 16), the
   canonical CUDA library directory, the compute capability
   (no card) and the agents' base, `/mnt/weaver/state`, named. The two settings its test
   step needs, unprivileged user namespaces with a uid map and `/bin/sh` as bash, are
   read and never changed: Arch gives both, where release 1.0 set them for the step and
   put them back. The install tuple's commit must equal `WA_COMMIT`. The test step's
   count is recorded in the stamp as `stack.tests` (the tag, above). The deploy scripts
   are copied to `/opt/weaver/deploy` before the checkout goes. The step refuses first,
   before bootstrap runs, unless each file the deploy copy names is a regular file in
   the checkout.
9. `connectors`: `gate-con` and `admin-con` built with no default features and installed.
   No unit is laid: the image half of WeaverWeb's staging is that repository's act. From
   the fifth build on, `build-template.sh mock` runs the offline mock after this step
   (`template-clone-Spec` section 8).
10. `cleanup`, the checked list, in this order:
    - The CUDA runtime made into a package of its own from the toolkit's files (the
      runtime set, below).
    - The source trees, their build directories, the Rust toolchain and cargo's caches.
    - The compilers, the build tools, the CUDA toolkit, cloud-init and the QEMU guest
      agent. Every base package still installed, less the three base packages named
      below, is marked explicitly installed first, what the build added is marked a
      dependency, and the runtime set is marked explicit. Then these are removed by
      name, with what only they held (`pacman -Rns`): every installed package the
      build-package pattern matches, base or added, less the runtime set, and three base
      packages: `cloud-init`, `cloud-guest-utils` (above), and `qemu-guest-agent`. The
      image's builder installs the guest agent, udev starts it wherever the hypervisor
      offers its channel, and nothing in the appliance uses it: a host's shutdown
      reaches the guest by ACPI, which the proof reads. Then what the build added that
      nothing kept needs is removed, round by round,
      until none is left. Each removal is simulated first (`pacman -Rs --print`, which
      prints the set `-Rns` removes, since pacman 7 refuses `--nosave` with `--print`).
      The simulation must remove no package of the runtime set and no base package but
      those named or matched. binutils is not a build package here: `mkinitcpio`, which
      the kernel package needs for its initramfs, and `libmakepkg-dropins`, which pacman
      needs, require it. So it stays, with `ld` and `as` on PATH, and a probe holds it
      to those two.
    - The runtime package installed in the toolkit's place, each file read back as the
      toolkit had it and owned by the package, and held with the kernel and driver.
    - `ldconfig` and the loader check.
    - The cloud image's swap file removed, its subvolume and its fstab line.
    - The EFI system partition mounted read-only at `/efi` by one fstab line naming it
      by UUID. The cloud image installs GRUB's EFI loader there and leaves the partition
      out of fstab, so without the line the
      MANIFEST would not cover the loader. Nothing writes the bootloader after the seal,
      and GRUB needs no mounted ESP to boot. In the build guest, systemd's gpt-auto
      generator served the partition as an automount, whose layer stays in the kernel
      after a reload, so every layer is unmounted and the partition mounted once by the
      line, as every later boot has it.
    - The initramfs made again without the `kms` hook, so it carries no nouveau (Arch's
      instructions for NVIDIA's driver; `nvidia-utils` also blacklists nouveau). The
      module loads from the root filesystem. A probe reads the initramfs holding no
      networkd and no nouveau.
    - The base inbound policy written and enabled, and the SSH settings written and read
      back as sshd reads them for root, with userdb's key command removed (above).
    - Crashes made a journal record and nothing more, so that a worker holding large
      host buffers cannot fill the root disk. A drop-in sets `Storage=none` with
      `ProcessSizeMax=0`: `Storage=none` alone
      would still write the core under `/var/lib/systemd/coredump` while it is
      processed, up to `ProcessSizeMax` (32G on 64-bit). The probe reads what
      `systemd-coredump` reads in the end, through `systemd-analyze cat-config`.
    - resolved's drop-in (the network, above), the two data mount points laid empty, and
      the image's own hostname (step 11).
    - Last, the units that would update the image or its keys masked: the keyring's
      weekly sync, systemd-sysupdate's units, and `pacman-init.service`, which would
      make a pacman master key, a private key, at every box's first boot (its file, the
      image builder's, is removed first). Then the metadata service and its generator,
      and 43 units the appliance does not use, masked too (below). pacman's sources are
      read on the snapshot alone, and its package caches and sync databases emptied.

    **Masked, not disabled**, under the operator's direction of 2026-11-14 that nothing
    is installed into a box after it is made. Nothing installs, overlays or updates
    software after the seal, the bootloader included, and nothing runs that the
    appliance does not use. A box's first boot applies systemd's presets, which would
    enable a disabled unit again, and presets
    leave a mask alone. The 43: sysext and confext (an image file dropped on disk would
    overlay `/usr` or `/etc`); homed (its first-boot unit makes accounts from
    credentials); sysupdate's notification sockets, the TPM clear (no TPM) and the
    report sockets, which no member of the appliance reads; userdbd (nss-systemd and PID
    1's dynamic users answer without it, and its one user here, sshd's key command, is
    removed); journalctl's and networkd's metrics sockets, mountfsd, nsresourced and
    `machines.target`; `systemd-time-wait-sync` (the network, above); and boot-update,
    which would rewrite a systemd-boot in the ESP after the seal, and
    boot-clear-sysfail. Kept: pstore, which only archives the kernel's crash records
    under `/var`, and coredump, which logs each crash and writes no dump.

    A rerun keeps the first run's size before the cleanup and its runtime package, and
    marks only the base packages still installed.
11. `seal`: the probes over SSH, the system libraries' records, a dry run of the
    MANIFEST, then a root oneshot once the session ends. The image's own hostname is
    `weaver-appliance`. The cleanup writes it to `/etc/hostname` in place of the build
    guest's own, so a box does not greet with the build's name, and so the recipe's
    `probes` step, which runs before the seal, reads it as the seal's probes do. Until
    2026-11-16 the seal wrote it, and the `probes` step
    before it always found the build guest's name. The running guest keeps its name, and
    the probes read that name as the build guest's. `create` refuses a `NAME` equal to
    the image's hostname. The dry run writes the MANIFEST as root to a scratch file in
    `/run` and checks its form (each file line a digest and a path, the marker once,
    each link line a path, one tab and a target, at least one of each). The oneshot
    writes the MANIFEST again only after removals that cannot be undone, so a find or a
    formatter that fails must fail in the dry run, with the guest intact and the step
    rerun. find's errors are kept, in the step's log and in `seal-final.log`. The
    oneshot removes the build account and its sudoers rule, and cloud-init's default
    account where a seed ever made it. It empties root's password field and sets its
    last change to 0 (`passwd -d root`, `chage -d 0 root`, the first login of the
    operator's ruling of 2026-11-16). It removes the SSH host keys, the machine identity
    (with systemd's random seed and credential secret, after it stops the random-seed
    service, and D-Bus's machine id), the pacman keyring this guest made at its first
    boot, whose master key is private, cloud-init's state, configuration and logs, and
    the rest of the logs: the journal emptied and every file under `/var/log` truncated.
    `/var/log/wtmp` is removed rather than emptied: `systemd-update-utmp`
    appends its records at the power-off, after the final probe, and glibc appends only
    to a wtmp that stands. A box's first boot lays it again, empty, from systemd's
    tmpfiles line (the proof's late-write read found the power-off's records in the 2.0
    rehearsal). It assembles the stamp from the build's records and removes those
    records and this script. The build's pacman transactions arm systemd's
    `ConditionNeedsUpdate`, since Arch's hook touches `/usr` after each one, so the
    oneshot rebuilds the loader cache once more, as `ldconfig.service` would, and writes
    `/etc/.updated` and `/var/.updated` at `/usr`'s time. A box's first boot then runs
    no update unit and writes nothing the MANIFEST does not carry. Nothing touches
    `/usr` after that. Then it runs the probes again. SSH stays enabled with no host key
    and no authorized key. `sshd.service` wants `sshdgenkeys.service`, as Arch's openssh
    ships them, and at the first boot it makes the host keys where none stands
    (`ssh-keygen -A`); the proof reads its condition met and its run a success (step
    12). After the console login the operator places a key and sets the network, and
    reads the host keys' fingerprints (decision 16 of 2026-11-15). If any probe hits, it
    writes `SEAL-REFUSED` and no MANIFEST. Otherwise it writes the stamp and then the
    MANIFEST last. If the MANIFEST cannot be written whole, the stamp goes too and it
    writes `SEAL-REFUSED`. Either way it powers off. Before any of this, the seal step
    refuses unless each commit the build recorded is the input the host names
    (`WA_COMMIT`, `WW_COMMIT`), and the version it recorded is `VERSION`. On the host,
    `seal` then reads the verdict out of the image before it marks anything sealed. It
    first refuses unless one of the domain's disks has the build's disk as its whole
    source path, read from the domain's XML, so a shut-off domain of that name on
    another disk is never undefined. A parser that fails, whatever its exit, is taken
    for a definition that cannot be read, never for a mismatch. A refusal, a missing
    MANIFEST, or a stamp that does not name both commits and the version, each equal to
    its input, stops it there, with the domain defined and the disk writable, for
    `destroy`. Only a passing verdict is followed by the compaction, the sha256, the
    read-only disk and the undefine. The seal's removals cannot be undone, so a refused
    image is destroyed and the build begins again from `create`. The read tells a file
    that is absent from one it could not read (on this base's btrfs root: no `btrfs` on
    the host, a restore that fails, a path that did not come out of the image; step 12).
    A read that fails stops `seal` with nothing changed. `seal` cannot run twice,
    because the guest is off and its account gone, so `build-template.sh verdict` reads
    the verdict again from the shut-off guest and goes on from there. `proof` tells the
    two apart in the same way.
    **The seal ends with a compaction, and the published digest is of the compacted
    file** (the operator's decision 18 of 2026-11-15). A qcow2 never moves its end
    offset back, so the build's disk is as long as the high-water mark of its writes.
    The fifth build's sealed file, made before this, is 20,964,769,792 bytes long for
    3.23 GiB of guest data (52,921 clusters of 64 KiB), and a transfer that does not
    keep holes writes the whole length. Once the verdict passes, and while the shut-off
    domain is still defined, `qemu-img convert` writes the disk again as a new
    standalone file of the allocated, non-zero clusters alone, beside it, and the copy
    is synced. The copy must compare equal to the disk, guest byte for guest byte
    (`qemu-img compare`), and pass `compact_check` (step 12) before it replaces the disk
    by a rename, and the directory is synced after. Then the sha256 and the file's
    figures are recorded, with the domain still defined and the disk writable, so a seal
    cut short there is finished by `build-template.sh verdict`. Only then is the disk
    made read-only and the domain undefined. A cut between those two leaves a sealed
    disk, its digest recorded, under a defined domain: `proof` runs on it, and the
    domain is undefined by hand. A compaction that fails, a copy that does not compare
    equal or is not compact, or a signal during any of these stops the seal with the
    copy removed, the domain defined and the disk as it was, writable, and
    `build-template.sh verdict` runs again from there. A copy left by a run cut short
    harder than that is refused and never reused. It may be removed by hand, alone, and
    `verdict` run again. `destroy` removes it with the rest of the build, the disk
    included. The record keeps both lengths, the comparison, and `compact_check`'s
    figures for the copy and for the sealed file. Measured on 2026-11-15 on a copy of
    the fifth build's file, made under scratch and never beside the kept image: the
    compacted file is 3,473,080,320 bytes long, holds 52,907 clusters (the 14 fewer are
    clusters of zeroes), compares equal to the original, and two compactions of the same
    file wrote byte-identical files. The record build of release 2.0 compacted
    12,246,843,392 bytes to 2,224,750,592, compared equal.
12. `proof`, on the host: the stamp, the MANIFEST, the probe record, and the network and
    update configuration read out of the sealed image without root. qemu-storage-daemon
    exports the image's root partition over FUSE as the user, and the reads go by the
    filesystem blkid finds on it: `btrfs restore` for this base's btrfs root, debugfs
    for release 1.0's ext4. Neither follows a symbolic link. `btrfs restore` copies out
    the paths a regex names, and each path asked for must come out or be refused by
    name: an expected absence is its own call, and a path through a link is not read.
    A link whose target climbs with `..` is refused, not resolved.
    Nothing writes to the sealed image or to an overlay before a boot. Then the sealed
    file itself: its sha256 must be the one the seal recorded after its compaction, and
    the file must be compact. `compact_check` reads the file's end offset and its
    allocated clusters with `qemu-img check`, which opens it read-only, and fails a file
    whose end lies beyond its allocated clusters by more than a bound, or that names a
    backing file, or that cannot be checked. What lies beyond them is the format's own
    tables. The bound is 64 MiB for the header and the first-level and refcount tables,
    plus the most the second-level tables can take, one 8-byte entry for each cluster of
    the virtual size: 15 MiB more for a 120 GiB disk in 64 KiB clusters, 64 MiB more for
    512 GiB. So a larger `SIZE` does not fail a compact file. On 2026-11-15 the check,
    with its 64 MiB alone, failed the fifth build's kept file, read-only (17,496,539,136
    bytes beyond its clusters), and passed the compacted copy of it (5,767,168 bytes
    beyond). Both verdicts stand under the larger bound. The stamp must name one version
    of the form `<release>.<respin>`, and `VERSION` where the caller names it. The
    shipped ggml library must carry the four cubin sets, of one count, and no PTX
    (above). Then one boot of an overlay with no seed, every frame its NIC sends
    captured and counted by kind. That NIC is `restrict=on` (`template-clone-Spec`
    section 4), because the sealed image carries no guard of its own and the second
    build's no-seed boot, unrestricted, reached public time servers and resolved their
    names through the host's resolver. The capture still holds every frame the guest
    sends, and past QEMU's own DHCP none is answered. An image that configures no
    network sends nothing, so any frame the guest sends fails the proof (the fourth
    build's five boots sent none, and so did the record build of release 2.0), and so
    does a capture with no header, which did not run. Then a second overlay booted as an
    operator's first boot is, with no host key. A seed is attached, which the image must
    ignore: its meta-data names a key and a hostname, and its user-data a bootcmd, a
    runcmd and the default account. With cloud-init removed, the sealed image's one way
    in is the console: root has no password and must set one, no key is authorized and
    no seed is read. **The proof logs in at the console as the operator does** (the
    operator's ruling of 2026-11-16). The operator's steps after logging in are two:
    place a key, set the network (decision 16 of 2026-11-15). The proof hands the
    network in, and never writes it into the image or its overlay, so SSH answers from
    the boot and the proof can show it refusing root before the first login. A networkd
    file naming a static address, gateway and resolver, matched by the NIC's MAC
    address, is given as the systemd credential `network.network.50-weaver-proof`, in an
    SMBIOS type 11 string. systemd imports it at boot, and `systemd-network-generator`
    writes it to `/run/systemd/network`, out of the MANIFEST's scope, where networkd
    reads it. SMBIOS is read through dmi-sysfs, which Arch's kernel builds in, so the
    import works from the initramfs on. The image configures no network, so without the
    credential the guest has no address, every login over SSH fails, and so does the
    proof. Measured on the record build of 2026-11-17: the credential brought the
    network up at the first boot. The key is placed at the console, after the first
    login.

    The console is QEMU's unix socket, in a directory only the proof's owner and
    `QEMU_USER` may enter, made as the console files are, whose default ACL lets the
    owner connect to the socket QEMU makes there. QEMU writes every byte the console
    sends to a file beside it as well, from the firmware's first line (`logfile=`, which
    QEMU truncates as it opens it). A socket's path holds at most 107 bytes, and the
    proof refuses a longer one before anything boots. That overlay's network is QEMU's
    `restrict=on`, with the SSH forward. The credential names the gateway and resolver
    because `restrict=on`'s DHCP offers neither. On release 1.0's base, netplan's wait
    then held the boot for its full 120 s. On this base no wait-online unit stands in
    the boot (the record build: none in `systemd-analyze blame`). The proof waits for
    the console's login prompt, which must greet with the image's hostname. It then
    reads the keys SSH presents with one `ssh-keyscan` of three of the types `ssh-keygen
    -A` makes, and one retry after 30 s, whose keys are added to the first scan's and
    never replace them. OpenSSH's `ssh-keygen -A` also makes an ML-DSA hybrid key, which
    the scan does not name. One scan, because the guest's sshd counts each
    unauthenticated connection against its source (`PerSourcePenalties`), every
    forwarded connection arrives from the same address, and repeated scans dropped a key
    (the fourth build's proof runs 2 to 4).

    Before the first login the proof tries root over SSH three ways, and each must be
    refused with the server offering `publickey` alone: an empty password, a wrong one,
    and the proof key, not yet placed. A password is given through `SSH_ASKPASS` with
    public keys off, so the client sends it only if the server offers a password method.
    A connection cut before authentication, as `PerSourcePenalties` cuts one, names no
    method, and fails the proof: a refusal that did not happen proves nothing. Then the
    first login, typed at the console through the socket by `console-drive.py`: `root`
    at the prompt, a password the proof makes (24 letters and digits) given twice when
    the change is asked, then from that root shell the proof key placed as root's one
    authorized key, one read line naming the terminal, the account, root's password
    state and the keys held, and the logout. The password is made into a file only the
    proof's owner can read, typed and offered to SSH from that file, never printed or
    logged, and removed after the last try. The driver logs each such send as `[sent:
    <file pw>]`, and the console does not echo it. `console_judge` grades the driver's
    log: the prompt before `root` greets with the image's hostname; after `root` no
    password is asked and no refusal shown, the forced change is announced and a new
    password asked; after the first entry it is asked again, and no error from the
    change follows; and the read line names `/dev/ttyS0`, root, `P` (a usable password)
    and one key. A driver that stopped on a step it waited for is a finding too.
    Measured on the record build of 2026-11-17: the prompt greeting as
    `weaver-appliance`, the three refusals, the login with the change, the key, SSH by
    that key, and two refusals after.

    It then logs in under the scanned keys as root, with the key it placed at the
    console, and reads the host keys' fingerprints in the guest, as the operator does
    after logging in. Each key SSH presents must be one the guest holds.
    Both sides are read from the same guest, so the match shows that sshd presents the
    keys on its disk, not where they came from. On the record build SSH presented three
    of the guest's four keys, each one the guest holds; the fourth is the hybrid key.
    Only on that match does the proof read on:
    - cloud-init absent with the seed attached (not installed, `/etc/cloud` and
      `/var/lib/cloud` gone), and none of the seed's asks carried out;
    - the hostname, by `uname -n` (Arch ships no `hostname` command);
    - the two data mount points (present, empty, root's, mode 0755, nothing mounted);
    - the network as the credential named it, and its wait;
    - the boot as systemd timed it, its critical chains to `multi-user.target` and to
      the serial console's login, and its slowest units. A boot not finished by then
      fails the proof: a unit holds it on what the proof's network cannot give, as
      time-wait-sync did before its mask. The record build's boot finished in 6.427 s
      (kernel 843 ms, initrd 2.235 s, userspace 3.347 s), its chain through
      `systemd-networkd`, with no wait in it;
    - `systemd-time-wait-sync` masked, and the ESP mounted read-only;
    - sudo running a command as nobody, as root after the first login;
    - pacman's sources after the first boot;
    - SSH's units, and whether `sshdgenkeys.service` ran: its condition met and its
      result a success;
    - the listening sockets (below), and the stamp's commits.

    Then two more tries over SSH, each refused as before: the password the console set,
    and a wrong one. It then sets the MANIFEST against the booted guest, on the host,
    and last measures the base inbound policy with no agent loaded. In the guest it
    reads the tables the kernel holds, and the image's table with its input policy drop
    and its SSH accept. It starts a listener on TCP 8080, serving a new empty directory,
    which must answer on the guest's own loopback. From the host, that listener, reached
    through a second forward (`PROOF_PORT` + 1 to the guest's 8080), must be silent.
    QEMU's forward accepts the host's connection whatever the guest does, then ends it
    at once where the guest refuses, and leaves it silent where the guest drops the
    connection. That is how the forward is expected to behave, and the host tests stand
    in for it with loopback servers. It is not measured against a guest. `inbound_probe`
    reads for 2 s, sends one HTTP request line, and reads again, for 15 s in all. The
    listener is stopped, and last SSH must answer with its banner through its own
    forward. SSH is probed last because the probe leaves without authenticating, which
    sshd counts against the forward's one source address. The probe is TCP over IPv4
    alone: the proof's network has IPv6 off, and no UDP or IPv6 packet reaches the guest
    on the wire. The guest's files are listed as the seal lists them, less the host
    keys' private halves, so no digest of a private key is made. Its links are formatted
    on the host by the seal's own formatter text. Each file or link removed, added or
    changed is printed by its path, never with a digest. A difference the first boot
    makes by design is marked expected. With no cloud-init, the list is short. Changed:
    the machine identity, which the seal leaves uninitialized, and the shadow file,
    where the first login sets root's password. Added: the host keys' public halves,
    which `sshdgenkeys.service` makes, and the account tools' lock file, which the list
    allows and does not require. The proof's network lands under `/run`, out of the
    scope. The fourth and fifth builds' first boots ran cloud-init from a seed, which
    also changed the account files and the hostname and added its own netplan file and
    sudoers rule. None of those is expected now. Measured on the record build: 41,653
    files and 8,888 links sealed, and no difference outside the list. No link is
    expected to change. On the fourth build's first boot, systemd ran its first-boot
    preset of all units, and the masks held against it, so a link it changes is a
    finding. Every other difference fails the proof, as do a listing that fails and a
    MANIFEST that does not carry the marker once. The files' listing runs under
    `pipefail`, so a `find` error fails it. The marker and the scope have one
    definition, in `guest/template-steps`, which `lib.sh` reads, and the offline mock
    checks that the two copies of the formatter are the same text. Each boot's serial
    console (the first boot's beside its socket), and the no-seed boot's capture, is
    written by QEMU to a file the proof makes. Only the file's owner and `QEMU_USER` may
    write it, by a POSIX ACL. An entry the file inherits from its directory's default
    ACL is removed before the grant, since chmod only masks it and the grant recomputes
    the mask. The overlays are made the same way. The file is made new and exclusively,
    so a file or a symbolic link already at that path is refused, never reused or
    written through. QEMU writing through the ACL was first proved on the fifth build. A
    file libvirt opens for a serial device becomes libvirt's, unreadable without root,
    and a file every local account could write would let any of them write the console
    the proof reads the login prompt from.
    **The power button.** After the reads, the proof presses the first boot's power
    button by ACPI (`virsh shutdown --mode acpi`), as a host's shutdown or Cockpit
    presses it with no guest agent in the image, and waits up to `PROOF_ACPI` seconds
    for the guest to power itself off. libvirt must
    name the guest's own shutdown as the stop's reason (`shut off (shutdown)`). The
    serial log after the press must reach systemd's `Reached target System Power Off`
    and the kernel's `reboot: Power down`, which it prints at every console level, with
    no `[FAILED]` line and no stop job waited on. A guest not off by then is stopped as
    every boot ends. The record build's guest was off 2 s after the press, cleanly.
    **The sealed disk is not relabelled.** Each boot names its overlay with a DAC
    security label that libvirt must not relabel, and
    the label holds for the overlay's whole backing chain. Without it, libvirt's DAC
    driver relabels every image in the chain to `QEMU_USER` and restores none that is
    read-only. On the fifth build that left the sealed disk owned by the account every
    QEMU on the host runs as, which as the owner may make it writable again, and the
    operator could no longer change its mode without root. With the label, libvirt leaves
    the sealed disk's owner and mode alone, and QEMU reads it as the 0444 file it is. The
    overlay is QEMU's to write by the same kind of ACL as the console files. The proof
    reads the sealed disk's owner and mode before its offline read and again after both
    boots, and a change fails it. A disk already relabelled by an earlier proof (the
    fifth build's kept image is one) stays owned by `QEMU_USER` until root re-owns it.
    The proof cannot undo that and does not try, and re-owning is the operator's root
    step.
    **The proof refuses as well as reads.** An image with no MANIFEST, or with a seal
    refusal, is not booted at all. After both boots, each finding of the sealed file
    (its sha256 and its compactness), of the stamp's version, of the shipped library's
    device code, each finding the first boot recorded, and each finding of the power
    button fails the proof (a non-zero exit, and the findings in `PROOF-FAILED`). The
    proof's network credential not made, a socket path too long, or a password or helper
    that cannot be made, stop it before the first boot. The first boot's findings are a
    login prompt that does not greet with the image's hostname, any of the five SSH
    tries as root not refused with `publickey` alone offered, a first login at the
    console that asks a password, does not require the change, fails the change, lands
    elsewhere than root on `/dev/ttyS0`, leaves root's password unusable or places other
    than one key, a console driver that stopped, a key SSH presents that the guest does
    not hold, a failed login as root, a MANIFEST difference outside the first boot's
    expected list, cloud-init installed or its directories standing, any of the seed's
    asks carried out (the bootcmd, the runcmd or the default account), a read of the
    default account that fails, a hostname other than the image's, a data mount point
    absent, not empty, not root's with mode 0755, or with a filesystem mounted on it, a
    boot not finished after the first login, `systemd-time-wait-sync` not masked, the
    ESP not mounted read-only, sudo not running a command as nobody,
    `sshdgenkeys.service` not run at the first boot, a stamp that does not name both
    commits, the image's table not loaded or not dropping inbound by default or not
    accepting SSH, a listener on TCP 8080 that does not answer on the guest's loopback
    or that answers from the host, SSH not answering through its forward, and any
    listening socket beyond loopback other than SSH on TCP 22. The power button's
    findings are a press the hypervisor cannot make, a guest not off within `PROOF_ACPI`
    seconds or off for another reason than its own shutdown, a serial log that does not
    reach the power-off target or the kernel's power-down line, and each failed unit or
    stop job waited on. The last of the first boot's is the measure of the operator's
    ruling 4 (SSH only): the first boot lists every listening TCP and UDP socket with
    its process, and fails on any bound beyond loopback but SSH. A socket bound to the
    loopback device (`%lo`) counts as loopback. A listing that fails, or that is empty
    though SSH listens, fails the proof too, and the record says the listing ran and how
    many sockets it found. On the fifth build's first boot it listed 8 sockets, and on
    the record build of release 2.0 it listed 6. On both, only SSH listened beyond
    loopback.
    **What the power-off wrote.** The seal's final probe runs before the guest powers
    off, and a power-off can write. So the offline read also looks in the sealed image,
    without booting it, for systemd's random seed (`/var/lib/systemd/random-seed`), its
    credential secret (`/var/lib/systemd/credential.secret`) and any non-empty file
    under `/var/log`. Each one found fails the proof with the first boot's findings,
    because no secret ships in every copy of the image. The login records under
    `/var/lib/wtmpdb` and `/var/lib/lastlog` are listed and not judged. A listing of
    `/var/log` that does not complete fails the proof too. On this base's btrfs root
    that is `/var/log` not coming out of the image as a directory, or a restore that
    fails. On release 1.0's ext4 it was an error debugfs reports on its own output (it
    exits 0 on a listing it cannot make), or a directory past 12 levels. These reads
    were checked against stand-in filesystem images of both kinds. The fifth build's
    sealed image passed them, and so did the record build of release 2.0: no seed, no
    secret, no non-empty log. In the 2.0 rehearsal they found `/var/log/wtmp` written at
    the power-off, which the seal now removes (step 11). The seal removes both files
    before its final probe, which names them. `systemd-random-seed.service` saves a seed
    when it stops, so left running it would write one at the power-off, after that
    probe. The seal stops it first (that stop saves a seed, which the seal then
    removes), and a stopped unit is not stopped again at the power-off. The proof's read
    confirms it.
    **What an interrupted proof or seal leaves.** `proof` holds a lock file in `VM_ROOT`
    while it runs, so a second proof refuses instead of tearing the first down. It also
    refuses where an overlay domain or a directory of its names stands, left by a run cut
    short, and only after those refusals does it set its cleanup, for every exit and for
    INT, TERM and HUP. A domain is undefined only once it is shut off, and taken for gone
    only from a domain list that was read. One still running after its stop, one that
    cannot be undefined, or one whose list or definition cannot be read keeps its
    directory, since an undefined domain that still runs goes on writing its disk. Each
    boot ends that way,
    and so do the cleanup and `destroy`. The cleanup removes only what that run made:
    each overlay domain (only where the domain stands on its own overlay), its directory
    and pool, and the
    offline read's FUSE export and directory. It keeps any console or capture not yet
    copied as `interrupted-*`. The boot's wait runs in the background, so a signal
    reaches the cleanup at once. Each wait runs without the lock's descriptor and the
    cleanup kills it, so the lock is free once the proof exits. On the fifth build, before
    this, a boot wait the TERM had orphaned held the lock for 104 s, and a second proof
    or a `destroy` would have refused meanwhile. The offline read's daemons start without
    the descriptor too. `seal`'s verdict read refuses where its read directory
    stands and closes only its own. `destroy` takes the same lock, so it refuses while a
    proof runs. It removes the build guest only where its domain stands on the build's
    disk, and refuses a domain of that name on any other disk. It then removes whatever
    a proof or a seal left.

**The runtime set kept.** It is marked explicitly installed, so the removals' recursion
never reaches it: the CUDA runtime package, the driver's packages, `acl` (decommission's
`setfacl`), `gcc-libs` (the C++ runtime the members load), `python` (the deploy scripts;
`create-agent.sh` needs 3.11 or later), `util-linux` (`runuser`) and `quota-tools`.
`setquota` sets the ext4 project quota on the agent's home, so the agent's own writes
cannot fill the state drive (the operator's ruling of 2026-11-17; `create-agent.sh`'s
option, WeaverAgent #585). e2fsprogs, in the base image, carries `chattr -p`. The state
drive's format and mount options are the setup script's act.

Arch's `cuda` is one package, toolkit and runtime together, so the runtime is made into
a package of its own inside the build guest, `weaver-cuda-runtime`. It is made from the
toolkit's own files, unchanged: the libraries the installed members and engine load,
each soname link to them, and the
`/opt/cuda/lib64` link. The libraries are cudart, cuBLAS, cuBLASLt, cuRAND, and NVRTC
with its builtins (cuBLASLt names NVRTC, and it is kept until a carded load shows it is
never mapped). Its one file of its own is the loader file `/etc/ld.so.conf.d/cuda.conf`,
naming the canonical library directory alone. The toolkit's own loader file also names
`nvvm/lib64`, which goes with the toolkit, and ldconfig would warn of it. makepkg builds
the package as the build account under fakeroot, with nothing stripped, from
`code/guest/cuda-runtime/PKGBUILD`. Each file's sha256 is taken from the toolkit before
the build and read again from the installed package. The cleanup then removes `cuda` and
installs this package in its place, so pacman owns every runtime file the install tuple
names, which a probe reads. The driver's packages are kept whole, with `nvidia-smi`,
NVML and `nvidia-persistenced`, and the image enables no NVIDIA unit.

**The probes.** Any hit refuses the seal:
- compilers, toolchains and build tools on a fixed PATH. `ld` and `as` count only where
  they are not binutils' own.
- their directories (`/usr/lib/gcc` left out where every file under it is libatomic's,
  a runtime library `gcc-libs` requires), static CUDA archives, and the build packages
- a kernel's module build tree, or binutils required by more than `mkinitcpio` and
  `libmakepkg-dropins`
- the tools for the agent home's project quota (`setquota`, `repquota`, `chattr`,
  `lsattr`, `mkfs.ext4`) missing, or not their package's. The pre pass tries them as the
  setup will format and mount the state drive: an ext4 filesystem in a 64 MiB file, made
  with `-O quota,project` and mounted `prjquota`, a directory given a project ID with
  the inherit flag (`chattr -p ID +P`), a block limit set on the project (`setquota
  -P`), both read back (`lsattr -p`, `repquota -P`), then all of it unmounted and
  removed. The record build's pre pass ran it as root and it passed.
- `test` or `sudo` not GNU's and sudo's own, or uutils' coreutils or sudo-rs installed
- source trees and build directories anywhere on the root filesystem
- package caches and sync databases
- package sources other than the snapshot, update units not masked, or the metadata
  service or its generator not masked
- a resolver address, or a file of resolved's other than the image's setting LLMNR or
  multicast DNS; networkd or NetworkManager configuration; networkd or nouveau in the
  initramfs
- an agent configured, the stack record's agent directory other than the state mount, a
  connector config or unit
- the data mount points not as laid: `/mnt/weaver` holding other than the two, or either
  of them absent, a link, not root's with mode 0755, not empty, or on another filesystem
  than the image's root
- a swap, or the ESP not mounted read-only at `/efi`, alone, through one fstab line
  naming it by UUID, or without the loader the firmware's removable path boots
- a stack member, a connector or a deploy script missing
- the loader check failing
- the CUDA runtime not as the toolkit had it (in the pre pass, against the toolkit's
  digests; in both, `pacman -Qkk` against the package's own list), or a runtime file the
  install tuple names that is not the package's or not of the digest the tuple names
- a unit link that leads nowhere
- more than one kernel: other than one version-named tree under `/usr/lib/modules`, the
  running kernel's, at its pin, and `/boot` holding its image, its initramfs and GRUB's
  directory alone, the image byte for byte the tree's `vmlinuz`
- the kernel or the driver off its pin, or not held by `IgnorePkg`
- the module's preconditions
- the base inbound policy not as built: `/etc/nftables.conf` not the text the steps
  define, nft's check refusing it, `nftables.service` not enabled, or the unit, drop-ins
  and all, carrying a stop command or not loading the file
- sshd not enabled, or not wanting `sshdgenkeys.service`
- cloud-init or the QEMU guest agent installed, or cloud-init's default account standing
- the SSH settings not as built: the drop-in not the text the steps define, or sshd's
  own merged configuration for root (`sshd -T`, whatever the case of its keywords) not
  naming each of the six. sshd -T will not run with no host key it can load, and the
  final pass runs after the seal removes them, so both passes give it a throwaway key
  under `/run`, which no key probe scans, and remove it after. A run that fails is a
  hit.
- userdb's key command for sshd standing, or its tmpfiles line not masked
- a unit the appliance does not use not masked, or a crash written as a dump
- sudo not as `create-agent.sh` needs it (WeaverAgent #581): `/etc/sudoers` not
  including `/etc/sudoers.d`, visudo refusing the files, or no account `nobody`. The pre
  pass also asks that sudo run a command as nobody, as the build account and by root's
  own rule. The final pass leaves that out: root's password is expired until the first
  login, and sudo's account check refuses an expired account even to root, so the proof
  asks it after that login.
- systemd's first-boot paths masked or missing: `provision.conf` not naming
  `ssh.authorized_keys.root` or overridden in `/etc/tmpfiles.d`,
  `systemd-firstboot.service`, `systemd-creds.socket` or
  `systemd-network-generator.service` masked or not found, the network generator not
  enabled, or `systemd-ssh-generator` absent or masked in
  `/etc/systemd/system-generators`
- an account that can log in, other than as sealed: a password field in `passwd` other
  than `x`, any account but root with a login shell (one `/etc/shells` lists) or not
  locked (its shadow field beginning `!` or `*`), root without a login shell, or a
  password hash (`$`) in `shadow`, `gshadow` or the tools' `shadow-` and `gshadow-`
  copies. The pre pass leaves out the build account and root's two fields, which the
  final step removes and sets, so any other account that can log in is found before the
  removals.
- `/etc/hostname` not the image's hostname, or the build guest's name in any file under
  `/etc`. The name is matched whole, with no letter, digit or hyphen on either side, so
  a longer word it begins is not a hit: the default name begins
  `weaver-appliance-build-Spec`, which `/etc/nftables.conf` names. The pre pass leaves
  out the files the final step removes, and the final pass leaves out its own probe
  record, which it writes while it runs.
- private-key files. The pre pass leaves out the files the seal itself removes (the SSH
  host keys and the pacman keyring), so a hit is found before the removals. A `.pem` is
  judged by its content, so a public certificate is not a hit.
- an authorized-keys file with any content under `/root`, `/home` or `/etc/ssh`, read as
  root. The pre pass leaves out the build account's own, which the seal removes with the
  account, so any other key is found before the removals.

Every consumer of a system credential is listed beside the probes, never as a hit
(systemd's first-boot paths, above).

A file a package owns is not a source-tree hit, nor is man-db's cache tag.

The final pass adds:
- the build account and its group, its home and cloud-init's sudoers rule. The account
  and the group count as gone only where getent reports them not found (its status 2);
  getent finding either, or failing, is a hit.
- shell histories, and cloud-init's state, configuration and logs
- the machine identity not `uninitialized`, as the seal writes it (an empty or absent
  file is a hit too, since the first boot rests on that word), systemd's random seed and
  credential secret, D-Bus's machine id, and the SSH host keys
- the pacman keyring
- root's password field not empty, or its last change not 0 (the accounts probe above,
  now judging root's two fields)
- the authorized-keys files again, the build account's gone with it
- non-empty logs
- the build's records or this script still present
- the first boot's update armed: `/etc/.updated` or `/var/.updated` not naming `/usr`'s
  time
- a stamp other than the one being sealed: anything in `/etc/weaver-appliance` but the
  stamp being assembled and the probe record
- private-key files, the seal's own removals included.

**The stamp and the MANIFEST.** `/etc/weaver-appliance/stamp` holds one `key = value` per
line. `sealed` (the time of the seal) comes first, then the build's records sorted by key,
then `manifest.scope` (which form the MANIFEST has) last. The records are these.
- The image's version, `appliance.version`. The seal refuses a stamp without it.
- The base image by name and digest, the archive's snapshot, the OS and kernel.
- Both commits and their origins, and the toolchain. The seal refuses a stamp without
  both commits.
- nvcc, CCCL, the CUDA toolkit, the compute capability compiled for, nvcc's host
  compiler and `cc`.
- The one package from outside the snapshot, CCCL, by URL and sha256, with its signature
  checked, and rustup's installer, stamped by digest (it is served under one unversioned
  name and changes with each rustup release).
- Each driver package with its version, and the repository and snapshot it came from.
- The module's version, vermagic, signer (none: Arch signs no module) and each file's
  sha256, and the CUDA runtime's minimum driver.
- The install tuple's digest, each stack file's, each deploy script's and each
  connector's sha256, the connectors' build command and the two unit templates' digests.
- The C library, the math library, the loader, the C++ runtime and `libgcc_s`, each by
  package version and sha256 (for setting the image against a host, WeaverTools#28).
- The build's vCPUs, memory and peak memory, and the sizes before and after the cleanup.

Every record is checked when written. A record that cannot be written fails its step,
and so does a value whose read fails or gives nothing, such as the digest of a file that
cannot be read.

The stamp is where a later reader finds the commits the image was compiled from, with no
build log: `weaveragent.commit` and `weaverweb.commit`, with their origins.

`/etc/weaver-appliance/MANIFEST` covers `/usr`, `/etc`, `/opt`, `/boot` and `/efi`, the
EFI system partition the cleanup mounts read-only (step 10), in two parts. The image's
identity is the MANIFEST's sha256, which covers both parts.
- **Regular files:** the sha256 of each one, as `sha256sum` writes it, less the MANIFEST
  itself and GRUB's environment block, which GRUB writes at boot.
- **Symbolic links:** after the marker line `# links: path TAB target, C-locale order`,
  one line per link: its path, a tab, and its target as stored. In both fields, backslash,
  tab and newline are escaped as `\\`, `\t` and `\n`.

Both parts are in C-locale order. A mask is a link, and unmasking a unit changes what the
image does, so a MANIFEST that cannot see links does not name the image. To check the
files with `sha256sum -c`, feed it the lines above the marker.

**The form changed after the fourth build.** The third and fourth builds used the earlier
form: regular files only, with no marker and no links section. Their identities
(`cd1ce9c4...` and `1374f698...`) do not cover the masks or any other link. The stamp's
`manifest.scope` line says which form an image has.

**Seams left clean.** The image boots with no agent, no connector config, no unit
instance, no credential, no key and no password. Root alone can log in, at the console,
with no password, and must set one there (the operator's ruling of 2026-11-16). SSH is
enabled, with host keys the first boot makes, and admits a login by key alone, once the
operator has placed one. There is no guest agent: a host's shutdown reaches the
appliance by ACPI. The hostname is `weaver-appliance`. There is no site-check hook and
no image key (both ruled out by the operator on 2026-11-14). A condition of the
operator's site is a unit the operator adds over SSH to their own VM, and the image
never learns of it. The appliance's setup script, to which its console goes straight in
place of a login prompt, its drive setup, the survey script and the console's download
(the profile's sections 3.5 and 8, by the operator's rulings of 2026-11-16 on the setup
pages and his addendum of that day) are later acts. The script's first question setting
root's password, and the operator's SSH key arriving through the console over TLS,
change the first login above and the console proof of step 12. The pairing itself
belongs to the connectors.

**The host logic tests run on a host, with no root and no guest.** They are in
`test/host/` beside the scripts. Each `t-*.sh` takes the functions it tests out of
`build-template.sh`, `lib.sh` and `guest/template-steps` by their own text, and runs
them against stand-ins: small qcow2 files, stand-in images opened read-only, loopback
servers, and stubs for what a guest or the hypervisor would answer. No test stubs a
privileged command. Where a guest function calls one, the test drops that prefix from
the function's text. Each run works in a new `mktemp` directory and leaves it for
reading. A loopback fixture is waited for within a bound, so one that never comes up is
a failed check, not a hang. A test that needs a tool the host lacks is skipped with exit
status 77. The harness clears the build's inputs and the scripts' other environment
knobs before any test reads them, so a test runs the same in a clean shell and in one
that has exported a build's inputs, as the recipe does (`t-harness-env.sh` checks it).
`t-device-code.sh` builds its stand-in libraries only where `nvcc` and `cuobjdump` are
on the host, and reads a sealed image only where `KEPT_IMAGE` names one. `run-all.sh`
runs them all. On 2026-11-17, at the record build's first step, the 22 tests ran 627
checks and all passed, on a host with CUDA 13.4's `nvcc` and OpenSSH 10.6p1.
`t-first-boot.sh` stands a server on the unix socket the proof names to QEMU, which
plays getty, the forced change and a shell, and the ways each can go wrong, and runs the
power button's read against a stand-in hypervisor. `t-login-seal.sh` runs `sshd_check`
against the host's own sshd with a stand-in configuration. `t-btrfs-reads.sh` builds
stand-in disks, a GPT with one Linux root partition holding a btrfs that `mkfs.btrfs`
fills from a directory, or ext4, or nothing, exports them over FUSE as the user, and
checks the offline reads and their guards. `t-arch-judges.sh` checks this base's judges:
the cloud image's signature check, the removal judge, pacman's sources and hold, the one
kernel, the runtime package's file list, the firewall, resolver, metadata and update
checks, the masks, the ESP, the crash settings and the quota tools, with pacman and
systemctl stood in where a judge asks them. `t-runtime-package.sh` builds the runtime
package with the host's makepkg, under fakeroot, from a stand-in toolkit, and checks
that it holds the listed files byte for byte and nothing else. The offline mock
(`template-clone-Spec` section 8) is not one of them, and runs only in a build guest.
`t-wait-for.sh` takes the mock's bounded wait out of its text, checks it is the
harness's own, and runs it, without running the mock.

## 3. Open

- **How admin's table opens a declared port through the image's rules.** A drop in the
  image's table is final, so an accept in admin's table does not open a port. As of
  2026-11-15 no declaration can name a port, and nothing is built for it: no set, no mark
  and no hand-over. One candidate, named as a candidate only: admin's table, at an
  earlier priority, marks the packet, and the image's static rule accepts the mark, so
  admin still writes only its own table. Settled in the act that adds the port member to
  the declaration, by admin's Spec and this one together.
- **The respin path.** A respin carries a release's binaries into a new OS layer and never
  recompiles (the operator's decision 4 of 2026-11-15). No such path is built: this tree
  compiles, and refuses a respin. Until one is built, a security fix to the OS layer is a
  new release. Settled by this Spec, in the act that builds the path.
