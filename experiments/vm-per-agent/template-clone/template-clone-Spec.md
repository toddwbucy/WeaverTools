# template-clone - Spec

**Status:** DRAFT. Landed on `main` with toddwbucy/WeaverTools #21. Section 10 and the
appliance build's changes elsewhere are on the `act/appliance-build` stack.

**Date filed:** 2026-11-12
**Document ID:** `template-clone-Spec`
**Parent:** toddwbucy/WeaverWeb issue #71 (one agent per virtual machine)
**Editorial:** Per the Working Rules.
**Landing PR:** the first sub-act of the template act on #21

---

## 0. What this document is

The Spec of the scripts in `code/` beside it, which built the template virtual machine
and since 2026-11-14 build the appliance image (section 10), make clones of the template, and run steps
inside a clone. It states what the scripts do and what
the offline mock pins. The result notes say what the scripts measured: `README.md` for the
template and clone of 2026-11-11, and `results/2026-11-12-determinism-slice.md` for the
determinism slice of 2026-11-12.

The slice ran on scratch versions of these scripts, which hold the measuring host's facts.
Each behaviour below that the slice added or corrected is marked **(slice)**, and its
section names the run that found it. A behaviour that acts only in a template build is
marked **(rebuild)**. Today's sealed template, of 2026-11-11, predates every such
behaviour, and the later act that rebuilds the template measures them, on the operator's
word.

**What the tree can and cannot rebuild, as of 2026-11-14.** Since the appliance act of that day,
`build-template.sh` builds the appliance image of section 10, at
`$VM_ROOT/appliance/appliance.qcow2` under the domain name `weaver-appliance-build`. It no
longer builds the template, and no script in this tree rebuilds the template of sections 1
to 9. The steps that made it were replaced, among them its `finish` step, which wrote the
guard's `/etc/nftables.conf` (section 4) and the models' `fstab` line. `clone.sh` still
makes clones of a template built before that act, the sealed template of 2026-11-11 at
`$VM_ROOT/template/template.qcow2`. It refuses an appliance image with a sentence, read
offline before anything is made, because the appliance's cloud-init discards the clone
seed's user-data: the seed's account, pinned host key and guard would never apply. The read
uses qemu-storage-daemon's FUSE export, `sfdisk` and `debugfs` on the host. Where it fails,
`clone.sh` says it could not read the image and stops, rather than wait out its 600 s
boot gate on what might be an appliance image. The behaviours marked **(rebuild)** wait for an act that brings the template's build back, if
one is ruled.

**Words.** The **host** runs libvirt and QEMU. A **guest** is a virtual machine: the
**build guest**, the appliance image while `build-template.sh` builds it (section 10), the
**template** while it was built, or a **clone**, an overlay naming the sealed template. A
**step** is one named command run in a guest over SSH, with its output and exit status kept
in the guest's log directory. A step whose guest command exits non-zero stops the host
script that ran it, once its log is kept: `build-template.sh step` and `seal`, and
`clone.sh step` and `driver`, each exit non-zero there. In a clone, `guest/gpu-steps` runs
the steps of sections 6 and 7 and hands any other step to `guest/clone-steps`. The guest
steps' parameters go with each step, each value quoted whole. A **round** of the driver
step is one apt install: the **pin** round installs the repository's pin package, and the
**open** round installs the driver.

```graph
node: template-clone
kind: method

edge: parent
from: template-clone
to: weaverweb-71
```

## 1. Parameters

**No fact about a host is committed.** The scripts take each one as a parameter, with no
default where a default would be a box path, a host name, an account, an address or a key:

| Parameter | What it names | Default |
|---|---|---|
| `VM_ROOT` | the directory holding `images/`, `template/`, `appliance/`, `clones/`, `logs/`, `keys/`, made canonical (`realpath -m`) | none |
| `KEY` | the measurement's login key | `$VM_ROOT/keys/measure_ed25519` |
| `MODELS_DIR` | the host directory shared read-only into a clone | none |
| `NAME`, `PORT`, `VCPUS`, `MEMORY` | a clone's name, its SSH forward's host loopback port, its size | as `clone.sh` states |
| `CPUSET` | host CPUs the guest's vCPUs and QEMU's threads stay on | unset: no pinning |
| `CI_WAIT` | the seconds a boot waits for cloud-init to finish, a positive number (section 3) | 900 |
| `DRIVER_PIN`, `DRIVER_OPEN` | the apt package specifications of the driver step's two rounds | none |
| `DEB_CACHE` | the host directory holding the driver rounds' checked files | `$VM_ROOT/debs` |
| `DEB_ORIGINS` | the URL prefixes a driver round may fetch from, space-separated, each ending `/` | the template's NVIDIA repository, which served every file of the slice's two rounds |
| `HARNESS_DIR` | the determinism matrix's `code/` directory inside the guest | none |
| `TOOLCHAIN_READING` | the `rustc --version` line the harness preflight must read | none |
| `APT_LISTS`, `APT_CACHE` | the guest's apt lists and archive cache, which `gpu-steps` reads and fills; the offline mock (section 8) points them at its own | apt's own directories |
| `RELAXED_LOCK` | the lock file that keeps a guest to one `-relaxed` step at a time (section 8); the offline mock points it at its own | `$HOME/.weaver-relaxed.lock`, in the operator's home |
| `OS` | the appliance image's operating system (section 10): `26.04`, with `24.04` named and refused until its cloud image is pinned | none for `build-template.sh create`, 26.04 for `clone.sh` |
| `DRIVER`, `DRIVER_ROUTE` | the driver release the image carries and the route it comes by (section 10) | none |
| `COMPUTE_CAP` | the compute capability the image's CUDA kernels are compiled for, without its dot (section 10) | none |
| `WA_COMMIT`, `WW_COMMIT` | the WeaverAgent and WeaverWeb commits the image is compiled from, each in full | none |
| `SEAL_WAIT` | the seconds the host waits for the guest's final seal step to power it off | 1200 |
| `PROOF_BOOT` | the seconds `proof` lets the overlay of the sealed image run with its frames captured | 180 |
| `PROOF_PORT` | the host port `proof`'s first-boot overlay forwards SSH on, bound to the host's loopback (clear of `clone.sh`'s default port) | `PORT` + 100 |
| `PROOF_FIRST_BOOT` | the seconds `proof` waits for the first boot's fingerprint block on the console | 300 |
| `QEMU_USER` | the account QEMU runs as under libvirt's system connection. `proof` grants it alone, by a POSIX ACL, write access to the console and capture files QEMU opens | `libvirt-qemu` |

A step that needs a parameter with no default refuses before it does anything.

## 2. The guest's SSH host key (slice)

**Each guest's host key is made on the host, handed in through its seed, and pinned.** The
scripts of 2026-11-11 connected with `StrictHostKeyChecking=no` and no known-hosts file
(`README.md`, "The network"). A local process that held the forward's port first would have
been answered as the guest. The slice's clone of 2026-11-12 ran pinned from its first boot.

- When the scripts make a guest, they make an ed25519 key pair for it under the guest's
  own directory, with the private key 0600.
- They write the guest's own known-hosts file, holding one line: `[127.0.0.1]:PORT`
  followed by that public key.
- Every SSH and file copy to the guest uses `StrictHostKeyChecking=yes`, that file as
  `UserKnownHostsFile`, and `/dev/null` as `GlobalKnownHostsFile`. A guest that answers
  with any other key is refused.
- The seed carries the pair as `ssh_keys` (`ed25519_private`, `ed25519_public`), with
  `ssh_deletekeys: true` and `ssh_genkeytypes: [ed25519]`.
  - Naming the type makes the seed valid against cloud-init's schema. The slice's seed
    carried `ssh_genkeytypes: []`, which the schema rejects as too short; cloud-init
    still applied it and recorded a schema warning on every boot.
- The seed's sources hold the private key, so they are removed once the seed image is
  made. The image is made 0600.
- A copy of the user-data, with the private key replaced by a placeholder line, is kept
  in the guest's log directory.
- When the template is sealed, the build guest's key pair, its known-hosts file and its
  seed image are removed from the host. The sealed template is never booted again.
- A guest made before these scripts pinned (the clone of 2026-11-11) has no known-hosts
  file of its own, and the scripts refuse to reach it.

**The template masks `sshd-keygen.service` before the seal (rebuild)**, so that the pinned
key is a clone's only host key. The journal of the slice's first boot shows that unit, and
not cloud-init, made the clone's RSA and ECDSA keys: "Generate sshd host keys on first
boot", then "ssh-keygen: generating new host keys: RSA ECDSA". `ssh_genkeytypes` does not
reach that unit. The mask acts only in a rebuilt template and is measured there. **A clone
of today's template still presents the two extra keys.** Neither key is pinned, and the
scripts refuse a guest that answers with either.

**The seal judges its private keys gone before the guest powers off (rebuild).** It
removes the SSH host keys, the module signing pair DKMS made (`MOK.priv`) and fwupd's
client pair (`secret.key`), and each removal's exit is checked. It then fails if any of
those files is still present. A seal that fails is never followed on the host by the
undefine, the read-only disk or the checksum, so no checksum is recorded for a template
whose seal did not pass.

## 3. SSH is the gate; cloud-init's verdict is judged apart (slice)

**A boot passes once a command runs over SSH under the pinned key, within a bound in
seconds; cloud-init's status is then recorded and judged as its own verdict.** The scripts
of 2026-11-11 waited on `wait_ssh`, which returned `cloud-init status`'s exit. cloud-init
26.1 exits 2 for "degraded done", so on the slice's first run, a boot whose SSH was up read
as "SSH did not come up", and the run stopped at its first step.

For each boot (the first boot, and each reboot):

1. The scripts wait until a command runs under the pinned key, or the bound passes. A
   bound that passes stops the caller, with one attempt's SSH error shown.
2. They wait for cloud-init to finish, for at most `CI_WAIT` seconds (section 1). Then
   they record `cloud-init status --long` with its exit status, and `cloud-init status
   --format json`, as `cloud-init-<tag>.txt` and `.json` in the guest's log directory.
   - A wait that passes its bound is recorded as exit 124, and it stops the caller.
     `cloud-init status` exits 0 while cloud-init is still running, so a status read
     after a plain timeout would pass a boot that was never judged.
3. On exit 2, they also record the lines of `cloud-init schema --system` that name an
   error, with no line that could carry key text, as `cloud-init-<tag>-schema.txt`.
4. The verdict:
   - **exit 0 passes**;
   - **exit 2 passes only** when the status is `done`, its `errors` are empty, and every
     recoverable item matches an entry on the named allow-list;
   - **any other exit stops**, as does any item not on the list.
   The verdict prints the name of each allowed item it saw, and the first line of each
   item that stopped it.

**The allow-list has one entry**:

| Name | Level | What it matches |
|---|---|---|
| `network-wait` | ERROR | `Failed to wait for network`, from `systemd-networkd-wait-online.service` exiting 1 |

`network-wait` cost about 120 s on each boot of the slice's clone, and its cause is not
proven. The slice's list also allowed the schema warning that the empty `ssh_genkeytypes`
caused. Section 2 removes that cause, so this list does not carry it, and a schema item
stops.

## 4. The guard

**A guest drops every new connection it opens to the addresses that user-mode networking
maps onto the host's loopback.** That is 10.0.2.2 for the host's 127.0.0.1, and fec0::2 for
the host's ::1. Replies on connections the host opens, the SSH forward among them, still
pass. Every new domain is also defined with `ipv6=off`. The guard is laid twice:

- by the seed's `bootcmd`, before any network service starts, where the table does not
  already stand;
- by the template's `/etc/nftables.conf` **(rebuild)**.

No script in this tree has laid the template's copy since 2026-11-14. The step that wrote it went with the
template's build (section 0).

Both copies carry the IPv4 line and the IPv6 line (WeaverTools commit 78bde86). The sealed
template of 2026-11-11 predates 78bde86, so its `/etc/nftables.conf` carries the IPv4 line
alone. Its clones also skip the seed's copy, because the template's table already stands
at that point. A clone of today's template therefore relies on `ipv6=off` for the IPv6
path (`README.md`, "The network"). Only a rebuilt template carries the IPv6 line.

**The appliance image of section 10 carries no copy of the guard.** The image's firewall
belongs to the operator: admin owns one named table in it and never the whole firewall,
as the operator ratified on 2026-11-14. This guard's file opens with `flush ruleset`, and
10.0.2.2 means the host only under QEMU's user-mode networking. So the rig guards its own
boots instead:
- the build VM gets the guard from its seed's `bootcmd`, as above.
- `proof`'s boot with no seed and its first boot both run on `restrict=on`, so neither
  boot of the sealed image can reach the host or anything beyond it.

The second, third and fourth builds ran their no-seed boot without `restrict=on`.

**`clone.sh netdev VALUE` takes only a user-mode argument whose forwards all stay on the
host's loopback.**
- VALUE must start `user,id=un0,`.
- Every `hostfwd` in it must bind the host's `127.0.0.1`:
  `hostfwd=tcp:127.0.0.1:N-GUESTADDR:M`, or the same with `udp`.
- One of them must be the SSH forward, `hostfwd=tcp:127.0.0.1:PORT-:22`.

The check of 2026-11-11 asked only for `user,id=un0,` and some `hostfwd`. A forward with no
host address binds every address, so `hostfwd=tcp::N-:22` would have offered the guest's
port beyond the host.

## 5. put (slice)

**`put` copies a host directory's contents into a guest directory that does not yet
stand, as a tar stream.**

- Symbolic links arrive as links.
- If the guest path already stands, `put` refuses and writes nothing.
- The guest path is checked before use. It must be made of letters, digits, `.`, `_`,
  `/` and `-`, start with a letter or digit, and hold no `..`.
- The source must be a directory.

The slice's scratch `put` was `scp -r`, which broke twice. It nests the source inside a
destination that already stands: the open round's `.deb` files landed in
`~/debs/debs-open/`, and run 3 stopped. It also follows links: the ten links in the staged
engine library directory arrived as full copies, found in the rehearsal on run 3's clone.

## 6. The offline driver round (slice)

**A clone's driver moves by a round the host fetches and the guest installs.** The guest
installs with apt's `--no-download` (step 4), so apt takes no file from the network for
it. That does not make the guest offline. The guard (section 4) drops only connections to
the host's loopback. `restrict=on`, which leaves the guest only the domain's named
`guestfwd`, is a domain argument the caller passes (section 9). Every file is checked on
the host against the SHA256 in the guest's own signed apt index before the guest installs
it. `clone.sh driver ROUND` runs one round's four steps:

1. **`driver-uris ROUND` (guest).** apt's `--print-uris` lines for the round's packages
   (`DRIVER_PIN` or `DRIVER_OPEN`), each with its hash replaced.
   - The replacement is the SHA256 of the index stanza whose `Filename` the URI ends
     with: "/" followed by the whole `Filename`. A stanza is found by its `Filename`,
     never by package name.
   - The stanzas come from apt's lists, which apt checked against the signed `InRelease`
     at its last update. apt 3.2.0 prints each file's MD5Sum (the slice's run 2 stopped
     on it), and the host does not check MD5.
   - **The step relies on this template keeping apt's lists uncompressed.** The slice's
     run 4 read its NVIDIA list uncompressed, as its own `driver-uris` log names it. A
     compressed `Packages` list (apt's `Acquire::GzipIndexes`) would leave its
     repository out of the lookup. So the step stops, naming each one, where it finds any
     compressed `Packages` list, among uncompressed ones or alone, and where it finds no
     list at all.
   - The step stops where the stanzas naming one file disagree on `Size` or SHA256, where
     their `Size` is not apt's, or where a stanza does not carry apt's own hash under the
     same name, or carries a different value.
   - A line that no stanza names, or whose stanza carries no SHA256, goes out as apt
     printed it. The host checks only SHA256 and SHA512, and refuses any other hash type.
2. **The fetch (host).** For each line:
   - **The URL must lie under one of `DEB_ORIGINS`**: it starts with one of those
     prefixes, and the rest is a plain path with no `..`, query, fragment or user part.
     Otherwise the round stops before anything is fetched. The guest names every URL a
     round fetches, so the host takes the origins from its own parameter, never from the
     guest. A percent-encoded dot segment in the rest (`%2e%2e`) can at most name another
     path on the same host, because each origin fixes the scheme, host and port; the hash
     check still decides what is kept.
   - **No redirect is followed.** Any answer but HTTP 200 stops the round, because a
     redirect would take the host to a URL that no origin allowed. The template's NVIDIA
     repository answered a request for one of the slice's files with 200 and no redirect
     (2026-11-12).
   - The file name is checked: one path component ending `.deb`. The size must be a
     number.
   - **Each file is checked in the round's own stage directory**, so the bytes put into
     the guest are the bytes checked. A cached copy is used only if its copy in the stage
     matches the line's hash and size.
   - Otherwise the file is fetched to a temporary name of its own in the cache, so two
     rounds sharing a cache never share one. It is checked, copied into the stage, and
     only then renamed into the cache, and the rename's exit is checked. A round never
     removes a file that another round names.
   - Any mismatch stops the round, and no unchecked file stands in the cache.
   - The round's temporary file and stage are removed on every exit: a refusal, or INT,
     TERM or HUP. A temporary file left by a round killed outright (SIGKILL, power loss)
     is never read, because no other round uses its name. Removing it is the operator's,
     since a sweep could remove another round's download in progress.
   - Two rounds sharing a cache may fetch the same file twice. No lock is needed for
     correctness, so none is taken.
   - A round that names no file stops.
3. **`put` (host)** puts the round's files into a new `~/debs` in the guest (section 5).
4. **`driver-install ROUND` (guest).**
   - It refuses unless `~/debs` holds `.deb` files and nothing else.
   - It puts them into apt's cache, then **removes `~/debs` whole**, so that the next
     round's `put` makes it afresh. A copy into the cache that fails removes `~/debs` as
     well, and says so, so the round can be put again.
   - It installs the round's packages with `--no-download`, and apt checks each file
     against its signed lists. The pin round installs with recommends; the open round
     with `--no-install-recommends`.

The pin round runs before the open round, because the template's old pin package holds
the open round's resolution until the new pin is installed. After the open round,
`clone.sh reboot` restarts the clone onto the new module, and the boot is judged as in
section 3. Each round's checked files are listed, with their size and hash, as
`debs-ROUND.txt` in the guest's log directory.

## 7. The harness run on the template's toolchain (slice)

**The determinism matrix runs in a clone with the template's own Rust toolchain on its
PATH.** No compiler is carried in.

- The template installs rustup with `--no-modify-path`, so the PATH a non-interactive SSH
  command gets has no `$HOME/.cargo/bin`.
- The matrix reads `rustc --version` at the repository it names, and refuses a run that
  has no toolchain reading.
- In the slice's rehearsal that read exited 127, which was read at the time as "the guest
  has no toolchain".
- The slice then carried the host's compiler in for the reading. After the run, a
  read-only check of the clone showed the template's own toolchain under
  `$HOME/.cargo/bin`, reading the same line as the host's
  (`results/2026-11-12-determinism-slice.md`).

The steps:

1. **`harness-preflight REPO` (guest).** In the environment the run gets (step 2), with
   `$HOME/.cargo/bin` first on its PATH, and in REPO, the repository whose toolchain the
   matrix reads, it refuses before any load unless:
   - `rustc` resolves to `$HOME/.cargo/bin/rustc`;
   - `rustc --version` reads `TOOLCHAIN_READING` exactly;
   - `rustup show active-toolchain` answers;
   - `HARNESS_DIR` holds the matrix's entry point.
   `RUSTUP_AUTO_INSTALL=0` is set, so a toolchain that REPO pins and the guest lacks
   refuses rather than starting a download. Nothing the step inherits reaches the
   readings (`RUSTUP_TOOLCHAIN`, `RUSTUP_HOME`, `CARGO_HOME`, a `GUEST_ENV` setting), so
   the preflight cannot read a toolchain that the matrix does not run.
2. **`harness-run CONFIG OUTDIR HOURS` (guest).** The preflight first, in the repository
   that CONFIG names. Then the matrix runs, detached from the SSH session, as the guest's
   operator account with a fresh group list.
   - Its environment is emptied and set to `HOME`, `USER`, `LOGNAME`, `LANG`,
     `RUSTUP_AUTO_INSTALL=0`, and a PATH with `$HOME/.cargo/bin` first.
   - Its output goes to a file in OUTDIR.
   - OUTDIR must not already stand.

The scratch agent the matrix drives is laid and taken down by the measurement, not by
these steps (section 9).

## 8. The offline mock

**Where the mock runs, and why.** The mock stubs `sudo`, so it runs only inside a build
guest, never on a host. On a host, a stub that missed would reach the real `sudo`. It
starts only where `systemd-detect-virt --vm` names a virtual machine and
`/run/weaver-build-guest` stands, owned by root, and anywhere else it refuses with a
sentence before it makes anything. `build-template.sh mock` runs it in the build guest:
it copies the code tree in, writes that marker (in `/run`, so it is gone at the next boot
and never reaches the sealed image), runs the mock, records its output and count in
`mock.txt`, and removes the copy. It runs before `cleanup`, while the guest still has the
tools the mock uses. Its count is taken there.

**`code/test/offline-mock.sh` runs the real `put`, `fetch_round`, `driver-install`,
`uris_sha256`, `ci_judge`, `cloud_init_verdict`, `netdev_ok`, `ensure_key`, harness
preflight and `relaxed`, the host scripts' `step`, `seal` and `seal_verdict`, the seal's
MANIFEST functions and proof's comparison, with no virtual machine of its own, no sudo and
no network beyond loopback.**

- The mock's guest is a temporary home directory inside the build guest it runs in. The
  SSH helper runs each
  guest command there, in an emptied environment. Its PATH holds the stubs, then the host
  tools the steps need, and no `rustc`.
- `sudo`, `apt-get` and `install` are stubs that record what they are asked to do:
  - `sudo` runs the command as the caller;
  - `apt-get --print-uris` answers from the mock's fixtures, and an install succeeds;
  - `install` copies.
- `APT_LISTS` and `APT_CACHE` (section 1) point at the mock's own index and cache.
- A Python HTTP server on loopback stands in for the archive, and `DEB_ORIGINS` names it.
- The guest's toolchain is a stub `rustc` and `rustup` in `$HOME/.cargo/bin`. Under an
  inherited `RUSTUP_TOOLCHAIN`, the stub `rustc` reads that toolchain's line.
- `cloud-init` and `template-steps` are stubs too. The host scripts' `step` and `seal` run
  as the scripts define them, with the functions they call (`inputs_ok`, `inputs_env`,
  `steps_in`), under their `set -euo pipefail`, with the copy of the steps into the guest
  stubbed, and `virsh_` and `qemu-img` recorded. The seal's offline read of its verdict
  (`image_open`, `image_cat`, `image_close`) is replaced by functions over a stand-in
  directory for the sealed image's root filesystem.
- `relaxed` runs as each guest file defines it, with `sysctl`, `readlink`, `ln` and `sudo`
  as functions over two files in the mock's directory, so no setting of the host moves.
  Its lock is a file in the mock's directory, taken with the real `flock`.
- It exits non-zero on the first check that fails. With `KEEP` set, its temporary
  directory stays for reading.

Each of the slice's three script defects has a check that reproduces it against the
behaviour that had it, and shows that this behaviour catches it:

| Defect | The check |
|---|---|
| D1, the nested `~/debs` | Two rounds back to back through `fetch_round`. After the first `driver-install`, no `~/debs` stands. The second round's `put` makes a new `~/debs`, and its install passes. A `~/debs` holding a directory is refused by `driver-install`, and a copy into apt's cache that fails removes `~/debs`. Under the old install, which left `~/debs`, the second `put` is refused instead of nesting. The old copy (`cp -r`, the stand-in for `scp -r`) nests, as run 3 found. |
| D2, links flattened | A source holding links arrives with the same links. The same source copied by `cp -rL` (the stand-in for `scp -r`) arrives as plain files, and the check tells them apart. |
| D3, the toolchain reading | With an inherited PATH that has no `$HOME/.cargo/bin`, the preflight, run in a stand-in repository, finds the stub rustc there and passes, reading in that repository with `RUSTUP_AUTO_INSTALL=0`. With no `$HOME/.cargo/bin/rustc`, or a stub reading another line, it refuses. The bare inherited PATH alone reproduces the exit 127. An inherited `RUSTUP_TOOLCHAIN` does not reach the readings: the matrix's line passes under it, and a line that only it gives is refused. |

The fetch (section 6, step 2):
- each round installs with `--no-download`, the open round without recommends, and each
  round's files are listed with their SHA256;
- a second round takes the cached copy;
- a file whose SHA256 is not the index's stops the round, and leaves no file and no
  partial file in the cache;
- a line left with apt's MD5Sum is refused;
- a round that names no file stops;
- a URL from an origin not in `DEB_ORIGINS`, or climbing out of one with `..`, is refused
  and never fetched, though its file and hash are good;
- an origin not ending in `/` is refused;
- a URL answered with a redirect is refused, naming its status; the redirect is never
  requested, and nothing of it stays in the cache.

apt's lists (section 6, step 1): lists kept only compressed, a compressed list among
uncompressed ones, and no list at all each stop `driver-uris`, naming what was found. The
uncompressed list alone passes.

A round's own files, in a cache two rounds share (section 6, step 2):
- a round that fetches leaves another round's download in progress alone, whether it
  has the old fixed name or one of its own, and leaves no temporary file or stage of its
  own;
- a cached copy of the right size that is not the file is fetched again, and the checked
  file replaces it in the cache and reaches the guest's apt cache;
- a refusal in `put` leaves no stage behind;
- a TERM during a fetch removes the round's temporary file and stage.

Two rounds actually running at once are not staged. The mock checks the mechanism that
keeps them apart, because a race between them depends on timing.

`uris_sha256`, on synthetic lists shaped like the slice's:
- a stanza found by exact `Filename`;
- two stanzas that agree;
- disagreeing stanzas;
- a size mismatch;
- no SHA256;
- no stanza;
- apt's own hash differing;
- a stanza found by `Filename`, never by package name.

`ci_judge` (section 3): exit 2 with `network-wait` alone passes, and so does `done` with no
item. A schema warning, a status other than `done`, and an unreadable status each stop.

`cloud_init_verdict` (section 3): a wait past `CI_WAIT`, while `cloud-init status` still
answers 0, is recorded as exit 124 and stops. A wait that ends in time with status `done`
passes. A `CI_WAIT` that is not a positive number of seconds is refused, 0 among them,
since `timeout 0` sets no deadline.

`netdev_ok` (section 4): the SSH forward alone, `restrict=on` with a `guestfwd`, and a
second forward on loopback each pass. A forward bound to every address, the SSH forward or
another, is refused, as are an argument with no SSH forward on `PORT` and one not starting
`user,id=un0`.

`ensure_key` (section 1) makes the key's directory, 0700, and the key in it.

`put` (section 5) refuses a destination that stands, and a path holding `..`.

The host scripts' steps (section 0):
- `build-template.sh step` and `clone.sh step` each exit non-zero on a guest step that
  fails, and nothing after them runs; a step that passes goes on;
- a seal that fails stops before the undefine, the read-only disk and the checksum. So
  does one whose image holds `SEAL-REFUSED` or no MANIFEST, or whose stamp names one
  commit or a commit other than its input. A read error stops it too, and is not taken
  for an absent MANIFEST. `build-template.sh verdict`, run again after it, reads the
  verdict and marks the image sealed. A seal that passes goes on to all three.

The MANIFEST (section 10):
- `LINKS_MARK` and `MANIFEST_SCOPE` are each defined once, in `guest/template-steps`, the
  marker's text is in no other code file, and `lib.sh` reads the same values.
- `links_format`'s two copies, `lib.sh`'s and `guest/template-steps`', are the same text.
- The seal's dry run passes on a stand-in scope with an escaped name and a link to
  `/dev/null`, and fails, with find's error shown, where a scope directory is missing.
- proof's comparison passes a change and an addition on the expected lists, and fails a
  change, a removal and a link beyond them. A MANIFEST without its marker fails it.

`relaxed`, in `guest/template-steps` and in `guest/clone-steps`:
- it runs the command with both settings relaxed and restores both, and its traps are
  gone after it;
- a command that fails is answered, with both restored;
- a change to `/bin/sh` that fails restores the sysctl, and the command never runs;
- a TERM or a HUP during the command restores both, and the step exits 143 or 129;
- a restore that fails after a command that passed fails the step and says so, and so
  does a restore whose commands answer 0 while a setting does not read back;
- an exit inside the command restores both and keeps its status, and an exit 0 with a
  restore that fails exits 1. The command is a shell function in the step's shell, so
  this covers an `exit` and an unset variable under `set -u`. After the unset variable,
  bash unwinds every function before the EXIT trap runs, so the two saved settings are
  kept in global variables;
- a write to a closed pipe, which is what the step meets after a dropped SSH session,
  restores both, and the step exits 141;
- a `/bin/sh` that is not a link is refused, with nothing changed;
- a step started while another holds the lock is refused at once: nothing is relaxed or
  restored, and the command never runs. A lock that cannot be opened is refused the same
  way, so a step never runs unlocked;
- the step holds the lock while the command runs, and the command runs without the
  lock's descriptor, so nothing it leaves running holds the lock after the step's shell
  ends. The lock file is made 0600.

The lock keeps one `relaxed` at a time in a guest. Without it, a second step that started
while the first had both settings relaxed would save the relaxed values as its own and put
them back after the first restored, and both steps would pass. The lock is taken before
the two settings are read. Its descriptor stays open for the step shell's life, so the
restore runs under it on every way out, the EXIT trap included, and the kernel frees it
however the shell ends. The file is in the operator's home, where the steps keep the
WeaverAgent checkout and cargo's target directory, so its directory is there on the
template as built; an empty file stays there after the
step. No other local account can use the lock to block a step: it can neither create nor
open a 0600 file in the operator's home, and root on a single-operator guest is the
operator. Two steps actually running at once are not staged. The mock checks the lock
that keeps them apart, because a race between them depends on timing.

Bash runs the EXIT trap itself when a terminating signal ends the shell (measured on bash
5.3.20 for TERM, HUP, INT and PIPE). So the mock cannot tell the explicit signal traps from
the EXIT trap. Removing the TERM or PIPE trap alone leaves every check passing. The explicit
traps stay, so the step does not depend on that behaviour.

The seal's own check of its private keys (section 2) runs only in a guest. The mock does
not reach it; a rebuild measures it.

Before a mock passed, the slice's own offline test stubbed `put` and every guest step,
which is how all three defects reached a run.

## 9. What these scripts do not do

**They do not lay or take down an agent, and they do not move a stack between machines.**
The slice's scratch agent was laid and taken down by the measurement's own steps. The
host's stack was carried into the clone after a loader check (the carry check), and the
two CUDA runtimes were compared. Those steps stay with the measurement: they belong to the
slice, not to the template, and the result note names them.

**They do not read compressed apt lists.** Reading them through apt's own helper
(`apt-helper cat-file`, which decompresses) is a later change. It is to be measured in a
guest when a template needs it, because the measuring host has no apt and could only stub
that path (section 6, step 1).

**`net-probe.sh` is a record, not a path a guest takes.** It measured the two network
backends with a throwaway guest before any template, on 2026-11-11, with a seed and SSH
of its own (`README.md`, "The network"). Sections 2 and 3 do not apply to it, and nothing
else calls it.

**They do not choose the network for a loaded agent.** `restrict=on` with one `guestfwd` is
a domain argument a caller passes. That network is the operator's to rule (`README.md`,
"The network").

## 10. The appliance build

`build-template.sh` builds the appliance image: one sealed qcow2 that an operator loads into
their own KVM hypervisor, never booted again as itself. KVM first. Proxmox is out of
scope (WeaverTools #29). On the operator's rulings of 2026-11-14 the binaries are compiled
inside the image's own build guest from two named commits, in place, and the guest is then
cleaned by a checked list and sealed. Nothing built on the host enters the image.

**Inputs, all named.** `OS` (the cloud image by digest, section 1), `DRIVER` and
`DRIVER_ROUTE`, `COMPUTE_CAP`, `WA_COMMIT` and `WW_COMMIT` in full. `create` refuses
unless all six are set. `COMPUTE_CAP` is the compute capability the CUDA kernels are
compiled for, without its dot. The build guest has no card to read it from, so it is
named, and the stamp records it. `DRIVER` has one form on the host and in the guest, one
regex in both scripts (a release such as `615.71.09`), and `DRIVER_ROUTE` is
`ubuntu-prebuilt`, the one route this tree builds. Each guest step is given those that are
set on its command line, and a step that needs one refuses where it is unset. The build
guest has 24 vCPUs and 64 GiB by default. The domain a box later runs under is the
operator's own.

**The network.** The build guest uses QEMU user-mode networking, as the template did
(`README.md`, "The network"). The sealed image configures no network: no netplan file, no
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
initramfs again, and a probe checks it holds no networkd. The network is then the first
boot's seed alone. chrony keeps Ubuntu's NTS pool as its servers, unchanged here.

**cloud-init: the SSH key, the network and the hostname (operator's ruling,
2026-11-14).** The cleanup writes `/etc/cloud/cloud.cfg.d/99-weaver-appliance.cfg`:
`allow_userdata: false`, vendor-data and vendor-data2 disabled, and the module lists cut
to `growpart`, `resizefs`, `set_hostname`, `update_hostname`, `update_etc_hosts`,
`users_groups` and `ssh` at init, none at config, and `keys_to_console` at final.
User-data is discarded before any module runs, so a seed cannot name modules back, run a
command or install a package. The seed gives the SSH key in its meta-data
(`public-keys`, for the default account), the hostname in its meta-data
(`local-hostname`) and the network in its `network-config`. With none, cloud-init's
fallback is DHCP on the first link. A key given in user-data is not read. The check is
cloud-init's own merged system configuration, read the way the next boot reads it.
`growpart` and `resizefs` take no setting from the seed. They grow the root filesystem to
the disk the operator gave. Measured on the third build, before this limit: the first
boot's `apt_configure` rewrote `ubuntu.sources`.

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
   DKMS, kernel source or compiler touches the driver. The kernel packages are held. The
   step checks, with no card, that each module is present for the kernel, that modinfo
   reads the release and a vermagic equal to the kernel, that modprobe resolves its
   dependencies, and that every firmware file it names is present. It runs before the
   stack: bootstrap's install refuses unless `weaver-spu` resolves `libcuda.so.1`, which
   the driver carries (the first build's break, 2026-11-14).
8. `stack`: WeaverAgent's `deploy/bootstrap-stack.sh --install`, unmodified, under
   `relaxed`, with the canonical CUDA library directory and the compute capability named
   (no card). The install tuple's commit must equal `WA_COMMIT`. The deploy scripts are
   copied to `/opt/weaver/deploy` before the checkout goes.
9. `connectors`: `gate-con` and `admin-con` built with no default features and installed.
   No unit is laid: the image half of WeaverWeb's staging is that repository's act. From
   the fifth build on, `build-template.sh mock` runs the offline mock after this step
   (section 8).
10. `cleanup`: the source trees and build directories, the Rust toolchain and cargo's
    caches, every package the build added but the runtime set, the CUDA repository,
    `git`, `unattended-upgrades` and `snapd`. Every installed package the build-package
    pattern matches is purged by name, base or added: the 26.04 cloud image itself ships
    kernel headers that depend on gcc, `libc6-dev`, which recommends one, binutils, and
    `bpftrace` and the BPF compiler collection, which depend on libclang. Those, and the
    metapackages and libraries that go with them, are named. Every other base package is
    marked manual first, and a simulation must remove no package of the runtime set and no
    base package but those named or matched. Then `ldconfig` and the loader check,
    cloud-init limited, the initramfs's catch-all network masked and the initramfs made
    again without networkd, and last the update units masked, apt's periodic jobs off, the
    sources disabled and the caches emptied. A rerun keeps the first run's size before the
    cleanup.
11. `seal`: the probes over SSH, the system libraries' records, a dry run of the MANIFEST,
    then a root oneshot once the session ends. The dry run writes the MANIFEST as root to a
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
    host key and no account, and `sshd-keygen.service` is
    masked: at first boot cloud-init makes the account and the host keys from the
    operator's seed and its `keys_to_console` prints the keys' fingerprints on the serial
    console, the trust on first use at the SSH step the operator ruled on 2026-11-14. If
    any probe hits, it writes `SEAL-REFUSED` and no MANIFEST. Otherwise it writes the
    stamp and then the MANIFEST last. If the MANIFEST cannot be written whole, the stamp
    goes too and it writes `SEAL-REFUSED`. Either way it powers off. Before any of this,
    the seal step refuses unless each commit the build recorded is the input the host
    names (`WA_COMMIT`, `WW_COMMIT`). On the host, `seal` then reads the verdict out of
    the image before it marks anything sealed. A refusal, a missing MANIFEST, or a stamp
    that does not name both commits, each equal to its input, stops it there, with
    the domain defined and the disk writable, for `destroy`. Only a passing verdict is
    followed by the undefine, the read-only disk and the sha256. The seal's removals
    cannot be undone, so a refused image is destroyed and the build begins again from
    `create`. The read tells a file that is absent from one it could not read (no
    debugfs, a filesystem it cannot open, a copy short of the file's size). A read that
    fails stops `seal` with nothing changed. `seal` cannot run twice, because the guest
    is off and its account gone, so `build-template.sh verdict` reads the verdict again
    from the shut-off guest and goes on from there. `proof` tells the two apart in the
    same way.
12. `proof`, on the host: the stamp, the MANIFEST, the probe record and the network
    configuration read out of the sealed image without root (qemu-storage-daemon over
    FUSE, then debugfs), and one boot of an overlay with no seed, every frame its NIC
    sends captured and counted by kind. That NIC is `restrict=on` (section 4), because the
    sealed image carries no guard of its own and the second build's no-seed boot,
    unrestricted, reached public time servers and resolved their names through the host's
    resolver. The capture still holds every frame the guest sends, and past QEMU's own DHCP
    none is answered. An image that configures no network sends nothing, so any frame the
    guest sends fails the proof (the fourth build's five boots sent none), and so does a
    capture with no header, which did not run. Then a second overlay booted as a first
    boot, with no host key, from
    a seed in three parts. Its meta-data names an SSH key and a hostname. Its
    network-config names a static address, gateway and resolver. Its user-data asks for a
    bootcmd, a runcmd, a package and an account, all of which must be discarded. That
    overlay's network is QEMU's `restrict=on`, with the SSH forward. The network is named
    because `restrict=on` DHCP offers no gateway and no resolver, and netplan's wait
    (`systemd-networkd-wait-online --any --dns -o routable`) then holds the boot for its
    full 120 s (the fourth build's first proof run): an appliance on a network with no
    default route or no resolver pays that wait on every boot. The fingerprints on the
    console are set against the keys SSH presents, read with one `ssh-keyscan` once the
    console's block has printed, and one retry after 30 s. One scan, because the guest's
    sshd counts each unauthenticated connection against its source (`PerSourcePenalties`),
    every forwarded connection arrives from the same address, and repeated scans dropped a
    key (the fourth build's proof runs 2 to 4). Only on a match does the proof log in,
    under those keys, as the default account, and read cloud-init's verdict and the
    modules it ran, what became of each user-data ask, the hostname, the network and its
    wait, `ubuntu.sources`, SSH's units and the stamp's commits. It then sets the MANIFEST
    against the booted guest, on the host. The guest's files are listed as the seal lists
    them, less the host keys' private halves, so no digest of a private key is made. Its
    links are formatted on the host by the seal's own formatter text. Each file or link
    removed, added or changed is printed by its path, never with a digest. A difference
    the first boot makes by design is marked expected. The changed files of that list
    were measured on the fourth build: the account files and their backups, the hostname
    and the machine identity. Its added files follow cloud-init's documented behaviour and
    are first measured at the fifth build: the host keys' public halves, its netplan file,
    its sudoers rule for the default account and the account tools' lock file. No link is
    expected to change. On the fourth build's first boot, systemd ran its first-boot
    preset of all units, and the masks held against it, so a link it changes is a finding.
    Every other difference fails the proof, as do a listing that fails and a MANIFEST
    that does not carry the marker once. The marker and the scope have one definition, in
    `guest/template-steps`, which `lib.sh` reads, and the offline mock checks that the two
    copies of the formatter are the same text. Each boot's serial console, and the
    no-seed boot's capture, is written by QEMU to a file the proof makes. Only the file's
    owner and `QEMU_USER` may write it, by a POSIX ACL. The file is made new and
    exclusively, so a file or a symbolic link already at that path is refused, never
    reused or written through. QEMU writing through the ACL is
    unproved until the fifth build. A file libvirt opens for a serial device becomes
    libvirt's, unreadable without root, and a file every local account could write would
    let any of them write the console that trust on first use is read from.
    **The proof refuses as well as reads.** An image with no MANIFEST, or with a seal
    refusal, is not booted at all. After both boots, each finding the first boot recorded
    fails the proof (a non-zero exit, and the findings in `PROOF-FAILED`). The findings
    are a fingerprint mismatch, a failed login, a MANIFEST difference outside the first
    boot's expected list, a user-data ask carried out (the
    command, the package or the account), cloud-init not done (the proof waits up to
    600 s with `cloud-init status --wait`, then requires the status text `done` and exit
    0, since the status exits 0 for running, disabled and not started too, 1 an error, 2
    degraded), a stamp that does not name both commits, and any listening
    socket beyond loopback other than SSH on TCP 22. The last is the measure of the
    operator's ruling 4 (SSH only): the first boot lists every listening TCP and UDP
    socket with its process, and fails on any bound beyond loopback but SSH. A socket
    bound to the loopback device (`%lo`) counts as loopback. A listing that fails, or
    that is empty though SSH listens, fails the proof too, and the record says the listing
    ran and how many sockets it found. It is first tested at the fifth build.
    **What the power-off wrote.** The seal's final probe runs before the guest powers
    off, and a power-off can write. So the offline read also looks in the sealed image,
    without booting it, for systemd's random seed (`/var/lib/systemd/random-seed`), its
    credential secret (`/var/lib/systemd/credential.secret`) and any non-empty file
    under `/var/log`. Each one found fails the proof with the first boot's findings,
    because no secret ships in every copy of the image. The login records under
    `/var/lib/wtmpdb` and `/var/lib/lastlog` are listed and not judged. These reads were
    checked against scratch filesystem images and are first exercised on a sealed image
    at the fifth build. The seal removes both files before its final probe, which names
    them. `systemd-random-seed.service` saves a seed when it stops, so left running it
    would write one at the power-off, after that probe. The seal stops it first (that
    stop saves a seed, which the seal then removes), and a stopped unit is not stopped
    again at the power-off. The proof's read confirms it.
    **What an interrupted proof or seal leaves.** `proof` holds a lock file in `VM_ROOT`
    while it runs, so a second proof refuses instead of tearing the first down. It also
    refuses where an overlay domain or a directory of its names stands, left by a run cut
    short, and only after those refusals does it set its cleanup, for every exit and for
    INT, TERM and HUP. The cleanup removes only what that run made: each overlay domain
    (only where the domain stands on its own overlay), its directory and pool, and the
    offline read's FUSE export and directory. It keeps any console or capture not yet
    copied as `interrupted-*`. The boot's wait runs in the background, so a signal
    reaches the cleanup at once. `seal`'s verdict read refuses where its read directory
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
- the module's preconditions
- SSH not enabled
- cloud-init not limited to the SSH key, network and hostname, or `keys_to_console` not
  in its final modules
- private-key files. The pre pass leaves out the files the seal itself removes, so a hit
  is found before the removals. A `.pem` is judged by its content: fwupd ships its public
  LVFS CA certificates as `.pem` under `/etc/pki`.

A file a package owns is not a source-tree hit (Ubuntu's Rust-built packages ship their
`Cargo.lock` under `/usr/share/doc`), nor is man-db's cache tag.

The final pass adds:
- the build account, shell histories, cloud-init's instance state, netplan files
- the machine identity, systemd's random seed and credential secret, and the SSH host
  keys, `sshd-keygen.service` not masked
- non-empty logs
- the build's records or this script still present
- a stamp other than the one being sealed: anything in `/etc/weaver-appliance` but the
  stamp being assembled and the probe record
- private-key files, the seal's own removals included.

**The stamp and the MANIFEST.** `/etc/weaver-appliance/stamp` holds one `key = value` per
line. `sealed` (the time of the seal) comes first, then the build's records sorted by key,
then `manifest.scope` (which form the MANIFEST has) last. The records are these.
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
instance, no credential, no key and no login account. SSH is enabled and answers once the
first boot's seed has made a host key and an account. There is no site-check hook and no
image key (both ruled out by the operator on 2026-11-14). A condition of the operator's
site is a unit the operator adds over SSH to their own VM, and the image never learns of
it. The first-boot pairing, the data disks' mount points and the console's download are
later acts.
