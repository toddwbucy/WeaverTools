# The appliance image build - Spec

**Status:** DRAFT. Not merged. Moved on 2026-11-15 from section 10 of `template-clone-Spec`,
where it stood on pull request 31.
**Date filed:** 2026-11-15
**Document ID:** `weaver-appliance-build-Spec`
**Parent:** `weaver-appliance-profile` (this directory), whose own artifact this build makes
**Gated on:** `toddwbucy/WeaverTools` issue 28. This is the one artifact of the profile gated
on it.
**Editorial:** Per the Working Rules. ASCII, absolute dates, prose at 88 characters.
**Landing PR:** the `act/appliance-v1` stack

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
| `OS` | the appliance image's operating system: `26.04`, with `24.04` named and refused until its cloud image is pinned | none for `build-template.sh create` |
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
| `CUOBJDUMP` | the host's `cuobjdump`, from a CUDA toolkit that knows the four architectures `proof` checks for (section 2) | `cuobjdump` |
| `QEMU_USER` | the account QEMU runs as under libvirt's system connection. `proof` grants it alone, by a POSIX ACL, write access to the console and capture files QEMU opens and to the overlays it boots | `libvirt-qemu` |

A step that needs a parameter with no default refuses before it does anything.

## 2. The build

`build-template.sh` builds the appliance image: one sealed qcow2 that an operator loads into
their own KVM hypervisor, never booted again as itself. KVM first. Proxmox is out of
scope (WeaverTools #29). On the operator's rulings of 2026-11-14 the binaries are compiled
inside the image's own build guest from two named commits, in place, and the guest is then
cleaned by a checked list and sealed. Nothing built on the host enters the image.

**Inputs, all named.** `OS` (the cloud image by digest, section 1), `VERSION`, `DRIVER`
and `DRIVER_ROUTE`, `COMPUTE_CAP`, `WA_COMMIT` and `WW_COMMIT` in full. `create` refuses
unless all seven are set. `COMPUTE_CAP` is the compute capability the CUDA kernels are
compiled for, without its dot. The build guest has no card to read it from, so it is
named, and the stamp records it. `DRIVER` has one form on the host and in the guest, one
regex in both scripts (a release such as `615.71.09`), and `DRIVER_ROUTE` is
`ubuntu-prebuilt`, the one route this tree builds. Each guest step is given those that are
set on its command line, and a step that needs one refuses where it is unset. The build
guest has 24 vCPUs and 64 GiB by default. The domain a box later runs under is the
operator's own.

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
sealed file the seal recorded. A proof that passes writes a record naming the sealed
file's sha256 and the MANIFEST's, and each proof removes the record when it starts. The
test count is the count of WeaverAgent's tests that `bootstrap-stack.sh` runs at its
test step in the build guest. A run that passes removes that step's own log, so the
stack step keeps bootstrap's output as it streams, records the count in the stamp as
`stack.tests`, and refuses unless the test step printed one count with no test failed
(step 8). The fifth build's stamp names neither a version nor a test count, so `tag`
refuses it. Checked on 2026-11-15 on the measuring host with stand-in records, and
against the fifth build's kept logs: its stack log reads 2867 passed, 0 failed.

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
sets absent, a cubin for any other architecture, a file that cannot be read and a
listing that cannot run. So does a listing line in a form the check does not read, and
a PTX listing that names no PTX image without cuobjdump's own line saying it found
none. cuobjdump 13.4.92 prints `No PTX file found` then (measured on 2026-11-15 on the
measuring host). The check counts architectures, not cubins: one cubin for an
architecture passes it. On 2026-11-15 the check passed stand-ins made with nvcc 13.4.92
on the measuring host, failed stand-ins with PTX, a missing architecture or an extra
one, and failed the fifth build's kept image, read-only, which was compiled before the
change: it carries 132 PTX images each for `sm_75` and `sm_80` beside the four cubin
sets of 132 each. The measurement behind the decision is WeaverAgent's, at `1f6ef77e` on
2026-11-15, on nvcc 13.4.92: two fresh builds into one target path gave all 13 shipped
files equal by sha256, and `libggml-cuda` with no PTX and 132 cubins for each of the
four. The image's toolkit is the same release, CUDA 13.4.2 with nvcc 13.4.92
(`guest/template-steps` pins it, and the stamp records it). A rebuild confirms those
shipped binaries only. This Spec does not claim the whole image reproducible. It is the
first measurement in the program to show the build site taking part in what is built
(the process id in the symbol tables), which bears on WeaverTools#28.

**The network.** The build guest uses QEMU user-mode networking, as the template did
(`experiments/vm-per-agent/template-clone/README.md`, "The network"). The sealed image configures no network: no netplan file, no
networkd or NetworkManager configuration, no resolver address, and Ubuntu's sources set
`Enabled: no`. The 26.04 initramfs carries dracut's `systemd-networkd` module, which
leaves `zzzz-dracut-default.network` (DHCP on every link) in `/run`. The second build's
proof boot with no seed took a lease through it and reached Ubuntu's NTP pool, so the
cleanup masks it with a link of the same name in `/etc/systemd/network` to `/dev/null`,
which outranks `/run`. That file was put there by `systemd-networkd` running inside the
initramfs, which took a lease there on every boot (the third build's proof boot with no
seed: four DHCP frames, then nothing after the switch to the root filesystem). It is in
the initramfs because `cloud-initramfs-dyn-netconf`'s dracut module depends on it. The
appliance has no network root, so the cleanup purges that package and makes the
initramfs again, and a probe checks it holds no networkd. The network is then the one
the operator sets after logging in (decision 16 of 2026-11-15), or the one an attached
seed names (below). chrony keeps Ubuntu's NTS pool as its servers, unchanged here.

**The base inbound policy: drop, but SSH** (the operator's decision 9 of 2026-11-15). The
image's own boot rules drop inbound traffic and accept SSH, with no agent loaded. The
cleanup writes one table of the image's own, `inet weaver_appliance`, to
`/etc/nftables.conf` and enables Ubuntu's `nftables.service`, which is to load it before
the network comes up (by the unit's own ordering, not measured on a boot of this
image). The table's input chain has policy drop and accepts four things:
loopback, replies (connection state established or related), the ICMPv6
neighbour-discovery types without which IPv6 does not work (neighbour solicitation and
advertisement, router advertisement), and TCP 22. It has no output chain, so outbound is
not limited (decision 17). The file opens by destroying the image's own table and never
flushes the ruleset, so a load, a reload or a second load replaces that table and leaves
every other table as it stands. Ubuntu's unit flushes the whole ruleset when it stops, so
a drop-in, `/etc/systemd/system/nftables.service.d/weaver-appliance.conf`, replaces the
unit's stop with the destruction of the image's table alone. Both texts are defined once
in `guest/template-steps`, and a probe sets the files against them. The rules are enabled
for the next boot and not loaded in the build guest. Checked on 2026-11-15 in a user
namespace on the measuring host (nftables 1.1.7): the file loads twice, and a second
table survives both loads and the stop. ufw is installed and inactive (`ENABLED=no`), as
the build expects the cloud image to ship it, and a probe reads that it stays so. No
build of this tree has run that probe yet. Admin's table is separate and
is admin's. In nftables a drop in one table's base chain is final, so an accept in admin's
table cannot open a port through this one (section 3). An address by DHCPv6 is not
provided for: a DHCPv6 server's reply does not match the multicast solicitation as a
reply, so the policy drops it. Multicast listener queries (MLD) are dropped too, so on a
switch that snoops MLD, neighbour discovery can lapse. Both are read from the rules and
not measured. Stopping `nftables.service` destroys the image's table, so inbound is
open until the unit starts again, and at the end of a shutdown, once the unit has
stopped.

**cloud-init: the build's seed, and a seed's limit in the image.** The build guest boots
from a NoCloud seed, which makes the build account, pins the guest's host key and sets
its network and its guard (`template-clone-Spec` sections 2 and 4). The proof's first
boot used a seed too, through the fifth build. It uses none now: it stands in for the
operator's steps after logging in (step 12). In the sealed image, a boot with no seed
attached runs no cloud-init, because no datasource is found. Where a seed is attached,
cloud-init acts on it within the limit the operator ruled on 2026-11-14 (the SSH key,
the network and the hostname). Whether the sealed image acts on an attached seed at all
is open (section 3). The cleanup writes the limit to
`/etc/cloud/cloud.cfg.d/99-weaver-appliance.cfg`: `allow_userdata: false`, vendor-data
and vendor-data2 disabled, and the module lists cut to `growpart`, `resizefs`,
`set_hostname`, `update_hostname`, `update_etc_hosts`, `users_groups` and `ssh` at init,
none at config, and `keys_to_console` at final. User-data is discarded before any module
runs, so a seed cannot name modules back, run a command or install a package. The seed
gives the SSH key in its meta-data (`public-keys`, for the default account), the
hostname in its meta-data (`local-hostname`) and the network in its `network-config`.
With none, cloud-init's fallback is DHCP on the first link. A key given in user-data is
not read. The check is cloud-init's own merged system configuration, read the way the
next boot reads it. `growpart` and `resizefs` take no setting from the seed. They grow
the root filesystem to the disk the operator gave. Measured on the third build, before
this limit: the first boot's `apt_configure` rewrote `ubuntu.sources`.

**The fixed layout.** The binaries carry their build paths, so the build account (`op`,
made by the seed), the checkouts (`~/WeaverAgent`, `~/WeaverWeb`) and the targets
(`~/target/weaveragent`, `~/target/weaverweb`) are fixed. The image lays the agent stack
where WeaverAgent's deploy scripts put it (`/opt/weaver`, `/etc/weaver/stack`), the deploy
scripts an enrolled box runs at `/opt/weaver/deploy`, the connectors at
`/opt/weaver-web/bin`, and its own record at `/etc/weaver-appliance`.

**The steps, in order.**

1. `create`: the cloud image becomes a standalone disk, then the seed, the definition and
   the boot.
2. `facts`: the base image's package list is recorded before anything is installed.
3. `os-probes`: whether `test` run through sudo honours supplementary groups, and whether
   visudo takes the admin connector rule's `Defaults` line, with a throwaway account
   removed in the same step.
4. `packages`, `rustup`: the build's tools and the toolchain both trees pin.
5. `trees`: both repositories cloned and checked out detached at their commits. A tree
   not exactly its commit, clean, refuses, as does a toolchain file that names another
   toolchain.
6. `toolkit`: NVIDIA's repository, the CUDA toolkit pinned to one release, and a CCCL inside
   the stack's window.
7. `driver`: route `ubuntu-prebuilt` (the operator's ruling of 2026-11-14): Ubuntu's own
   branch, its module prebuilt and signed by Canonical for the image's one kernel
   (restricted), its userspace at the same version (multiverse), each package named. No
   DKMS, kernel source or compiler touches the driver. The kernel is held with every
   image and module package installed for it: its image, its modules, the driver's
   module, and on 26.04 `linux-main-modules-zfs`, which its modules depend on (installed
   and not held on the fifth build, finding 8). The step refuses unless the first three
   are installed. It checks, with no card, that each module is present for the kernel, that modinfo
   reads the release and a vermagic equal to the kernel, that modprobe resolves its
   dependencies, and that every firmware file it names is present. It runs before the
   stack: bootstrap's install refuses unless `weaver-spu` resolves `libcuda.so.1`, which
   the driver carries (the first build's break, 2026-11-14).
8. `stack`: WeaverAgent's `deploy/bootstrap-stack.sh --install`, unmodified, under
   `relaxed`, with the canonical CUDA library directory and the compute capability named
   (no card). The install tuple's commit must equal `WA_COMMIT`. The test step's count
   is recorded in the stamp as `stack.tests` (the tag, above). The deploy scripts are
   copied to `/opt/weaver/deploy` before the checkout goes.
9. `connectors`: `gate-con` and `admin-con` built with no default features and installed.
   No unit is laid: the image half of WeaverWeb's staging is that repository's act. From
   the fifth build on, `build-template.sh mock` runs the offline mock after this step
   (`template-clone-Spec` section 8).
10. `cleanup`: the source trees and build directories, the Rust toolchain and cargo's
    caches, every package the build added but the runtime set, the CUDA repository,
    `git`, `unattended-upgrades` and `snapd`. Every installed package the build-package
    pattern matches is purged by name, base or added: the 26.04 cloud image itself ships
    kernel headers that depend on gcc, `libc6-dev`, which recommends one, binutils, and
    `bpftrace` and the BPF compiler collection, which depend on libclang. Those, and the
    metapackages and libraries that go with them, are named. So is `linux-image-virtual`,
    the kernel image's metapackage (finding 8, closed by removal rather than a hold).
    Nothing installed depends on it: in the fifth build's guest a simulated purge removed
    it alone. Its one use is to pull the next kernel at an update, and the appliance is
    never updated in place, so a hold would keep a package with no use in every copy. The
    kernel it named stays, held. Every other base package is
    marked manual first, and a simulation must remove no package of the runtime set and no
    base package but those named or matched. Then `ldconfig` and the loader check,
    cloud-init limited, the initramfs's catch-all network masked and the initramfs made
    again without networkd, the base inbound policy written and enabled, and last the
    update units masked, apt's periodic jobs off, the
    sources disabled and the caches emptied. A rerun keeps the first run's size before the
    cleanup, and marks manual only the base packages still installed, since apt-mark
    refuses a whole call on one name it cannot find. A rerun does not keep the caches
    empty while it runs: its first apt command rebuilds apt's binary caches in
    `/var/cache/apt` (measured on the fifth build, whose second run stopped at that
    apt-mark), and only its last section removes them again. A rerun that stops before
    then leaves them, and the probes refuse the seal on them.
11. `seal`: the image's own hostname, the probes over SSH, the system libraries'
    records, a dry run of the MANIFEST, then a root oneshot once the session ends. The
    hostname is `weaver-appliance`, written to `/etc/hostname` in place of the build
    guest's own, so a box does not greet with the build's name (the Planner's note of
    2026-11-15). The running guest keeps its name, and the probes read that name as the
    build guest's. `create` refuses a `NAME` equal to the image's hostname. The dry run
    writes the MANIFEST as root to a
    scratch file in `/run` and checks its form (each file line a digest and a path, the
    marker once, each link line a path, one tab and a target, at least one of each). The
    oneshot writes the MANIFEST again only after removals that cannot be undone, so a find
    or a formatter that fails must fail in the dry run, with the guest intact and the step
    rerun. find's errors are kept, in the step's log and in `seal-final.log`. The oneshot
    removes the build account and its sudoers rule, the SSH host keys, the machine
    identity (with systemd's random seed and credential secret, after it stops the
    random-seed service), cloud-init's state and the logs, assembles the stamp from the
    build's records, removes
    those records and this script, and runs the probes again. SSH stays enabled with no
    host key and no authorized key. `sshd-keygen.service` is left enabled, as its
    package ships it, and at the first boot it makes the host keys (`ssh-keygen -A`, on
    the condition that the boot is a first boot, which the uninitialized machine
    identity makes it). No build of this tree has measured that yet: the proof's first
    boot reads the unit's condition and result. After the console login the operator
    places a key and sets the
    network, and reads the host keys' fingerprints (decision 16 of 2026-11-15). Through
    the fifth build the seal masked `sshd-keygen.service`, the first boot's seed made
    the account and the host keys, and cloud-init's `keys_to_console` printed the
    fingerprints on the serial console. If any probe hits, it writes `SEAL-REFUSED` and
    no MANIFEST. Otherwise it writes the
    stamp and then the MANIFEST last. If the MANIFEST cannot be written whole, the stamp
    goes too and it writes `SEAL-REFUSED`. Either way it powers off. Before any of this,
    the seal step refuses unless each commit the build recorded is the input the host
    names (`WA_COMMIT`, `WW_COMMIT`), and the version it recorded is `VERSION`. On the host, `seal` then reads the verdict out of
    the image before it marks anything sealed. It first refuses unless one of the
    domain's disks has the build's disk as its whole source path, read from the domain's
    XML, so a shut-off domain of that name on another disk is never undefined. A parser
    that fails, whatever its exit, is taken for a definition that cannot be read, never
    for a mismatch. A refusal,
    a missing MANIFEST, or a stamp
    that does not name both commits and the version, each equal to its input, stops it there, with
    the domain defined and the disk writable, for `destroy`. Only a passing verdict is
    followed by the compaction, the sha256, the read-only disk and the undefine. The
    seal's removals cannot be undone, so a refused image is destroyed and the build
    begins again from
    `create`. The read tells a file that is absent from one it could not read (no
    debugfs, a filesystem it cannot open, a copy short of the file's size). A read that
    fails stops `seal` with nothing changed. `seal` cannot run twice, because the guest
    is off and its account gone, so `build-template.sh verdict` reads the verdict again
    from the shut-off guest and goes on from there. `proof` tells the two apart in the
    same way.
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
    file wrote byte-identical files.
12. `proof`, on the host: the stamp, the MANIFEST, the probe record and the network
    configuration read out of the sealed image without root (qemu-storage-daemon over
    FUSE, then debugfs). Then the sealed file itself: its sha256 must be the one the
    seal recorded after its compaction, and the file must be compact. `compact_check`
    reads the file's end offset and its allocated clusters with `qemu-img check`, which
    opens it read-only, and fails a file whose end lies beyond its allocated clusters by
    more than a bound, or that names a backing file, or that cannot be checked. What
    lies beyond them is the format's own tables. The bound is 64 MiB for the header and
    the first-level and refcount tables, plus the most the second-level tables can
    take, one 8-byte entry for each cluster of the virtual size: 15 MiB more for a
    120 GiB disk in 64 KiB clusters, 64 MiB more for 512 GiB. So a larger `SIZE` does
    not fail a compact file. On 2026-11-15 the check, with its 64 MiB alone, failed the
    fifth build's kept file, read-only (17,496,539,136 bytes beyond its clusters), and
    passed the compacted copy of it (5,767,168 bytes beyond). Both verdicts stand under
    the larger bound. The stamp
    must name one version of the form `<release>.<respin>`, and `VERSION` where the
    caller names it. The shipped ggml library must carry the four cubin sets and no PTX
    (above). Then one boot of an overlay with no seed, every frame its NIC
    sends captured and counted by kind. That NIC is `restrict=on` (`template-clone-Spec` section 4), because the
    sealed image carries no guard of its own and the second build's no-seed boot,
    unrestricted, reached public time servers and resolved their names through the host's
    resolver. The capture still holds every frame the guest sends, and past QEMU's own DHCP
    none is answered. An image that configures no network sends nothing, so any frame the
    guest sends fails the proof (the fourth build's five boots sent none), and so does a
    capture with no header, which did not run. Then a second overlay booted as an
    operator's first boot is, with no seed and no host key. **The proof stands in for
    the operator's steps after logging in** (decision 16 of 2026-11-15). Those steps are
    two: place a key, set the network. The proof writes both into its own overlay
    offline, before the boot, and never into the image: the proof key as root's
    authorized key, and a netplan
    file, `/etc/netplan/90-weaver-proof.yaml`, naming a static address, gateway and
    resolver, matched by the NIC's MAC address. The overlay is opened writable through
    qemu-storage-daemon and written with debugfs, only where the file names a backing
    file, so the sealed image can never be opened writable this way, and QEMU opens the
    backing file read-only. A filesystem that is not clean is refused. Each file is read
    back from the overlay opened again read-only, with its bytes, its owner (root) and
    its mode (0600), and `e2fsck -n` must find the filesystem clean, before anything
    boots. Tested on 2026-11-15 against a scratch disk made on the measuring host: the
    sealed stand-in's digest did not change. The placement rests on three things, and
    the proof fails closed where any does not hold:
    - `/root/.ssh` stands (0700 in the fifth build's kept image). The placement refuses
      where it does not.
    - root may log in over SSH by key. The fifth build's kept image sets no
      `PermitRootLogin`, so OpenSSH's default, `prohibit-password`, applies. The login
      fails where it does not.
    - netplan applies a file placed by hand with no cloud-init. Not measured: the login
      fails where it does not.

    Root's entry (`/bin/bash`, password field `*`) was read from the fifth build's kept
    image. That no other account has a login shell is not measured. What waits for the
    operator is the console
    login itself and the account it lands in (section 3). The proof does not stand in
    for those, and its record names the gap on a line of its own, which is not a
    failure. That overlay's network is QEMU's `restrict=on`, with the SSH forward. The
    network is named because `restrict=on` DHCP offers no gateway and no resolver, and
    netplan's wait (`systemd-networkd-wait-online --any --dns -o routable`) then holds
    the boot for its full 120 s (the fourth build's first proof run): an appliance on a
    network with no default route or no resolver pays that wait on every boot. The proof
    waits for the console's login prompt, which must greet with the image's hostname. It
    then reads the keys SSH presents with one `ssh-keyscan` of the three types
    `ssh-keygen -A` makes, and one retry after 30 s, whose keys are added to the first
    scan's and never replace them. One scan, because the guest's sshd
    counts each unauthenticated connection against its source (`PerSourcePenalties`),
    every forwarded connection arrives from the same address, and repeated scans dropped
    a key (the fourth build's proof runs 2 to 4). It logs in under those keys as root,
    with the key it placed, and reads the host keys' fingerprints in the guest, as the
    operator does after logging in. Each key SSH presents must be one the guest holds.
    Both sides are read from the same guest, so the match shows that sshd presents the
    keys on its disk, not where they came from.
    Only on that match does the proof read on: cloud-init's status (with no seed it does
    not run), the hostname, the network and its wait, `ubuntu.sources`, SSH's units and
    whether `sshd-keygen.service` ran, and the stamp's commits. It then sets the
    MANIFEST against the booted guest, on the host, and last measures the base inbound
    policy with no agent loaded. In the guest it reads the tables the kernel holds, and
    the image's
    table with its input policy drop and its SSH accept. It starts a listener on TCP 8080,
    serving a new empty directory, which must answer on the guest's own loopback. From
    the host, that listener, reached
    through a second forward (`PROOF_PORT` + 1 to the guest's 8080), must be silent. QEMU's
    forward accepts the host's connection whatever the guest does, then ends it at once
    where the guest refuses, and leaves it silent where the guest drops the connection.
    That is how the forward is expected to behave, and the host tests stand in for it
    with loopback servers. It is not measured against a guest.
    `inbound_probe` reads for 2 s, sends one HTTP request line, and reads again, for 15 s in
    all. The listener is stopped, and last SSH must answer with its banner through its own
    forward. SSH is probed last because the probe leaves without authenticating, which
    sshd counts against the forward's one source address. The probe is TCP over IPv4
    alone: the proof's network has IPv6 off, and no UDP or IPv6 packet reaches the guest
    on the wire. The guest's files are listed as the seal lists
    them, less the host keys' private halves, so no digest of a private key is made. Its
    links are formatted on the host by the seal's own formatter text. Each file or link
    removed, added or changed is printed by its path, never with a digest. A difference
    the first boot makes by design is marked expected. With no seed, the list is short.
    Changed: the machine identity, which the seal leaves uninitialized. Added: the host
    keys' public halves, which `sshd-keygen.service` makes, the proof's own network
    file, and the account tools' lock file, which the list allows and does not require.
    The fourth and fifth builds' first boots ran cloud-init from a seed, which also
    changed the account files and the hostname and added its own netplan file and
    sudoers rule. None of those is expected now, and the shorter list is not yet
    measured on a build. No link is expected to change. On the fourth build's first
    boot, systemd ran its first-boot
    preset of all units, and the masks held against it, so a link it changes is a finding.
    Every other difference fails the proof, as do a listing that fails and a MANIFEST
    that does not carry the marker once. The marker and the scope have one definition, in
    `guest/template-steps`, which `lib.sh` reads, and the offline mock checks that the two
    copies of the formatter are the same text. Each boot's serial console, and the
    no-seed boot's capture, is written by QEMU to a file the proof makes. Only the file's
    owner and `QEMU_USER` may write it, by a POSIX ACL. An entry the file inherits from
    its directory's default ACL is removed before the grant, since chmod only masks it
    and the grant recomputes the mask. The overlays are made the same way. The file is
    made new and exclusively, so a file or a symbolic link already at that path is refused, never
    reused or written through. QEMU writing through the ACL was first proved on the fifth
    build. A file libvirt opens for a serial device becomes
    libvirt's, unreadable without root, and a file every local account could write would
    let any of them write the console the proof reads the login prompt from.
    **The sealed disk is not relabelled** (the Planner's ruling of 2026-11-15). Each
    boot names its overlay with a DAC security label that libvirt must not relabel, and
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
    device code, and each finding the first boot recorded fails the proof (a non-zero
    exit, and the findings in `PROOF-FAILED`). The proof's two files not read back as
    written, or a filesystem that is not clean, stop it before the first boot. The first
    boot's findings are a login prompt that does not
    greet with the image's hostname, a key SSH presents that the guest does not hold, a
    failed login as root, a MANIFEST difference outside the first boot's expected list,
    cloud-init run on a boot with no seed (its status not `disabled`), a hostname other
    than the image's, `sshd-keygen.service` not run at the first boot, a stamp that does
    not name both commits, the image's table not loaded or
    not dropping inbound by default or not accepting SSH, a listener on TCP 8080 that does
    not answer on the guest's loopback or that answers from the host, SSH not answering
    through its forward, and any listening
    socket beyond loopback other than SSH on TCP 22. The last is the measure of the
    operator's ruling 4 (SSH only): the first boot lists every listening TCP and UDP
    socket with its process, and fails on any bound beyond loopback but SSH. A socket
    bound to the loopback device (`%lo`) counts as loopback. A listing that fails, or
    that is empty though SSH listens, fails the proof too, and the record says the listing
    ran and how many sockets it found. On the fifth build's first boot it listed 8
    sockets, and only SSH listened beyond loopback.
    **What the power-off wrote.** The seal's final probe runs before the guest powers
    off, and a power-off can write. So the offline read also looks in the sealed image,
    without booting it, for systemd's random seed (`/var/lib/systemd/random-seed`), its
    credential secret (`/var/lib/systemd/credential.secret`) and any non-empty file
    under `/var/log`. Each one found fails the proof with the first boot's findings,
    because no secret ships in every copy of the image. The login records under
    `/var/lib/wtmpdb` and `/var/lib/lastlog` are listed and not judged. These reads were
    checked against scratch filesystem images, and the fifth build's sealed image passed
    them: no seed, no secret, no non-empty log. The seal removes both files before its final probe, which names
    them. `systemd-random-seed.service` saves a seed when it stops, so left running it
    would write one at the power-off, after that probe. The seal stops it first (that
    stop saves a seed, which the seal then removes), and a stopped unit is not stopped
    again at the power-off. The proof's read confirms it.
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

**The runtime set kept.** The CUDA runtime the installed SPU and engine resolve
(cuda-cudart, libcublas, libcurand), NVRTC (cuBLASLt names it, and it is kept until a carded
load shows it is never mapped), the driver's packages (its compute library whole, with
`nvidia-smi` and NVML), `acl`, the C++ runtime and `python3`.

**The probes.** Any hit refuses the seal:
- compilers, toolchains and build tools on a fixed PATH
- their directories, static CUDA archives and the build packages
- source trees and build directories anywhere on the root filesystem
- package caches
- apt sources enabled, update units unmasked, `unattended-upgrades` or `snapd`, an apt
  proxy
- a resolver address, networkd or NetworkManager configuration, the initramfs's
  catch-all network not masked, or networkd in the initramfs
- an agent configured, a connector config or unit
- a stack member, a connector or a deploy script missing
- the loader check failing
- more than one kernel (a version-named tree under `/lib/modules`, not the empty
  `/lib/modules/kernel/nvidia-615-open` the 615 module package also ships)
- an installed kernel image or module package (`linux-image-*`, `linux-modules-*`,
  `linux-main-modules-*`, the driver's module among them) not held, or the running
  kernel's image not read as held. The holds are read from dpkg, not from `apt-mark`:
  apt run as root rebuilds the binary caches the probes refuse.
- the module's preconditions
- the base inbound policy not as built: `/etc/nftables.conf` or the drop-in not the texts
  the steps define, nft's check refusing the file, `nftables.service` not enabled, or ufw
  not inactive
- SSH not enabled, or `sshd-keygen.service` not enabled
- cloud-init not limited to the SSH key, network and hostname (its module lists, with
  `keys_to_console` at final)
- `/etc/hostname` not the image's hostname, or the build guest's name in any file under
  `/etc`. The name is matched whole, with no letter, digit or hyphen on either side, so
  a longer word it begins is not a hit: the default name begins
  `weaver-appliance-build-Spec`, which `/etc/nftables.conf` names. The pre pass leaves
  out the files the final step removes, and the final pass leaves out its own probe
  record, which it writes while it runs.
- private-key files. The pre pass leaves out the files the seal itself removes, so a hit
  is found before the removals. A `.pem` is judged by its content: fwupd ships its public
  LVFS CA certificates as `.pem` under `/etc/pki`.
- an authorized-keys file with any content under `/root`, `/home` or `/etc/ssh`, read as
  root. The pre pass leaves out the build account's own, which the seal removes with the
  account, so any other key is found before the removals.

A file a package owns is not a source-tree hit (Ubuntu's Rust-built packages ship their
`Cargo.lock` under `/usr/share/doc`), nor is man-db's cache tag.

The final pass adds:
- the build account, shell histories, cloud-init's instance state, netplan files
- the machine identity not `uninitialized`, as `cloud-init clean` writes it (an empty or
  absent file is a hit too, since the first boot's host keys rest on that word),
  systemd's random seed and credential secret, and the SSH host keys
- the authorized-keys files again, the build account's gone with it
- non-empty logs
- the build's records or this script still present
- a stamp other than the one being sealed: anything in `/etc/weaver-appliance` but the
  stamp being assembled and the probe record
- private-key files, the seal's own removals included.

**The stamp and the MANIFEST.** `/etc/weaver-appliance/stamp` holds one `key = value` per
line. `sealed` (the time of the seal) comes first, then the build's records sorted by key,
then `manifest.scope` (which form the MANIFEST has) last. The records are these.
- The image's version, `appliance.version`. The seal refuses a stamp without it.
- The base image, the OS and kernel.
- Both commits and their origins, and the toolchain. The seal refuses a stamp without
  both commits.
- nvcc, CCCL, the CUDA toolkit and the compute capability compiled for.
- The two downloaded installers: the CUDA repository's keyring package, pinned by digest,
  and rustup's installer, stamped by digest (it is served under one unversioned name and
  changes with each rustup release).
- Each driver package with its version and the pocket and component it came from.
- The module's version, vermagic, signer and each file's sha256, and the CUDA runtime's
  minimum driver.
- The install tuple's digest, each stack file's, each deploy script's and each
  connector's sha256, the connectors' build command and the two unit templates' digests.
- The C library, the math library, the loader, the C++ runtime and `libgcc_s`, each by
  package version and sha256 (for setting the image against a host, WeaverTools#28).
- The build's vCPUs, memory and peak memory, and the sizes before and after the cleanup.

Every record is checked when written, and a record that cannot be written fails its
step.

The stamp is where a later reader finds the commits the image was compiled from, with no
build log: `weaveragent.commit` and `weaverweb.commit`, with their origins.

`/etc/weaver-appliance/MANIFEST` covers `/usr`, `/etc`, `/opt`, `/boot` and `/boot/efi`
in two parts. The image's identity is the MANIFEST's sha256, which covers both parts.
- **Regular files:** the sha256 of each one, as `sha256sum` writes it, less the MANIFEST
  itself and GRUB's environment block, which GRUB writes at boot.
- **Symbolic links:** after the marker line `# links: path TAB target, C-locale order`,
  one line per link: its path, a tab, and its target as stored. In both fields, backslash,
  tab and newline are escaped as `\\`, `\t` and `\n`.

Both parts are in C-locale order. A mask is a link, and unmasking a unit changes what the
image does, so a MANIFEST that cannot see links does not name the image (the Planner's
call of 2026-11-14). To check the files with `sha256sum -c`, feed it the lines above the
marker.

**The form changed after the fourth build.** The third and fourth builds used the earlier
form: regular files only, with no marker and no links section. Their identities
(`cd1ce9c4...` and `1374f698...`) do not cover the masks or any other link. The stamp's
`manifest.scope` line says which form an image has.

**Seams left clean.** The image boots with no agent, no connector config, no unit
instance, no credential, no key and no account that can log in. SSH is enabled, with
host keys the first boot makes, and admits a login once the operator has placed a key.
The hostname is `weaver-appliance`. There is no site-check hook and no
image key (both ruled out by the operator on 2026-11-14). A condition of the operator's
site is a unit the operator adds over SSH to their own VM, and the image never learns of
it. The first-boot pairing, the data disks' mount points and the console's download are
later acts.

**The host logic tests run on a host, with no root and no guest.** They are in
`test/host/` beside the scripts. Each `t-*.sh` takes the functions it tests out of
`build-template.sh`, `lib.sh` and `guest/template-steps` by their own text, and runs
them against stand-ins: small qcow2 files, stand-in images opened read-only, loopback
servers, and stubs for what a guest or the hypervisor would answer. No test stubs a
privileged command. Where a guest function calls one, the test drops that prefix from
the function's text. Each run works in a new `mktemp` directory and leaves it for
reading. A test that needs a tool the host lacks is skipped with exit status 77.
`t-device-code.sh` builds its stand-in libraries only where `nvcc` and `cuobjdump` are
on the host, and reads a sealed image only where `KEPT_IMAGE` names one. `run-all.sh`
runs them all. On 2026-11-15 the 11 tests ran 192 checks and all passed, on a host with
CUDA 13.4's `nvcc`. The offline mock (`template-clone-Spec` section 8) is not one of
them, and runs only in a build guest.

## 3. Open

- **How admin's table opens a declared port through the image's rules.** A drop in the
  image's table is final, so an accept in admin's table does not open a port. As of
  2026-11-15 no declaration can name a port, and nothing is built for it: no set, no mark
  and no hand-over. One candidate, named as a candidate only: admin's table, at an
  earlier priority, marks the packet, and the image's static rule accepts the mark, so
  admin still writes only its own table. Settled in the act that adds the port member to
  the declaration, by admin's Spec and this one together.
- **The console login and the account it lands in.** Root has no password (read from
  the fifth build's kept image), and the build expects it to be the image's one account
  with a login shell, which is not measured. How the operator logs in at the console,
  and which account that lands in, waits for the operator's ruling. The proof names the
  gap and does not stand in for it: it logs in over SSH as root, with the key it placed
  in its own overlay. Root as the proof's login is the proof harness's choice, and it
  does not answer the operator's question. Settled by the operator's ruling, then by
  this Spec and the proof.
- **cloud-init in the sealed image.** Proposed on 2026-11-15, pending the operator: the
  sealed image disables cloud-init, and the package may stay. An image that still acts
  on an attached seed has a first-boot path the profile does not state. Until the
  operator rules, a seed attached to the image is acted on within the limit of
  2026-11-14 (section 2). Settled by the operator's ruling, then by this Spec.
- **systemd's first-boot paths in the image.** Beside cloud-init, the image holds
  systemd's own first-boot paths as Ubuntu 26.04 ships them. Read on 2026-11-15 from the
  fifth build's kept image, read-only:
  - `/usr/lib/tmpfiles.d/provision.conf`, whose lines name the systemd credentials
    `ssh.authorized_keys.root` (for `/root/.ssh/authorized_keys`), `network.hosts` (for
    `/etc/hosts`), and `login.issue` and `login.motd` (for files under `/etc/issue.d`
    and `/etc/motd.d`)
  - `systemd-firstboot.service`, wanted by `sysinit.target`
  - `systemd-creds.socket`, wanted by `sockets.target`
  - `systemd-ssh-generator`, among the system generators

  The build masks none of them. Whether the image keeps them, masks them or names them
  in the profile waits for the operator's ruling. Settled by the operator's ruling, then
  by this Spec.
- **The respin path.** A respin carries a release's binaries into a new OS layer and never
  recompiles (the operator's decision 4 of 2026-11-15). No such path is built: this tree
  compiles, and refuses a respin. Until one is built, a security fix to the OS layer is a
  new release. Settled by this Spec, in the act that builds the path.
