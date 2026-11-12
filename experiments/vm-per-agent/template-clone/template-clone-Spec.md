# template-clone - Spec

**Status:** DRAFT. On the `act/vm-template` stack, toddwbucy/WeaverTools #21.

**Date filed:** 2026-11-12
**Document ID:** `template-clone-Spec`
**Parent:** toddwbucy/WeaverWeb issue #71 (one agent per virtual machine)
**Editorial:** Per the Working Rules.
**Landing PR:** the first sub-act of the template act on #21

---

## 0. What this document is

The Spec of the scripts in `code/` beside it, which build the template virtual machine,
make clones of it, and run steps inside a clone. It states what the scripts do and what
the offline mock pins. The result notes say what the scripts measured: `README.md` for the
template and clone of 2026-11-11, and `results/2026-11-12-determinism-slice.md` for the
determinism slice of 2026-11-12.

The slice ran on scratch versions of these scripts, which hold the measuring host's facts.
Each behaviour below that the slice added or corrected is marked **(slice)**, and its
section names the run that found it. A behaviour that acts only in a template build is
marked **(rebuild)**. Today's sealed template, of 2026-11-11, predates every such
behaviour, and the later act that rebuilds the template measures them, on the operator's
word.

**Words.** The **host** runs libvirt and QEMU. A **guest** is a virtual machine: the
**template** while it is built, or a **clone**, an overlay naming the sealed template. A
**step** is one named command run in a guest over SSH, with its output and exit status kept
in the guest's log directory. In a clone, `guest/gpu-steps` runs the steps of sections 6
and 7 and hands any other step to `guest/clone-steps`. The guest steps' parameters go with
each step, each value quoted whole. A **round** of the driver step is one apt install: the
**pin** round installs the repository's pin package, and the **open** round installs the
driver.

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
| `VM_ROOT` | the directory holding `images/`, `template/`, `clones/`, `logs/`, `keys/` | none |
| `KEY` | the measurement's login key | `$VM_ROOT/keys/measure_ed25519` |
| `MODELS_DIR` | the host directory shared read-only into a clone | none |
| `NAME`, `PORT`, `VCPUS`, `MEMORY` | a clone's name, its SSH forward's host loopback port, its size | as `clone.sh` states |
| `CPUSET` | host CPUs the guest's vCPUs and QEMU's threads stay on | unset: no pinning |
| `DRIVER_PIN`, `DRIVER_OPEN` | the apt package specifications of the driver step's two rounds | none |
| `DEB_CACHE` | the host directory holding the driver rounds' checked files | `$VM_ROOT/debs` |
| `HARNESS_DIR` | the determinism matrix's `code/` directory inside the guest | none |
| `TOOLCHAIN_READING` | the `rustc --version` line the harness preflight must read | none |
| `APT_LISTS`, `APT_CACHE` | the guest's apt lists and archive cache, which `gpu-steps` reads and fills; the offline mock (section 8) points them at its own | apt's own directories |

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

## 3. SSH is the gate; cloud-init's verdict is judged apart (slice)

**A boot passes once a command runs over SSH under the pinned key, within a bound in
seconds; cloud-init's status is then recorded and judged as its own verdict.** The scripts
of 2026-11-11 waited on `wait_ssh`, which returned `cloud-init status`'s exit. cloud-init
26.1 exits 2 for "degraded done", so on the slice's first run, a boot whose SSH was up read
as "SSH did not come up", and the run stopped at its first step.

For each boot (the first boot, and each reboot):

1. The scripts wait until a command runs under the pinned key, or the bound passes. A
   bound that passes stops the caller, with one attempt's SSH error shown.
2. They record `cloud-init status --long` with its exit status, and `cloud-init status
   --format json`, as `cloud-init-<tag>.txt` and `.json` in the guest's log directory.
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

Both copies carry the IPv4 line and the IPv6 line (WeaverTools commit 78bde86). The sealed
template of 2026-11-11 predates 78bde86, so its `/etc/nftables.conf` carries the IPv4 line
alone. Its clones also skip the seed's copy, because the template's table already stands
at that point. A clone of today's template therefore relies on `ipv6=off` for the IPv6
path (`README.md`, "The network"). Only a rebuilt template carries the IPv6 line.

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
cannot download during it (section 4, and `restrict=on` where set). Every file is checked
on the host against the SHA256 in the guest's own signed apt index before the guest
installs it. `clone.sh driver ROUND` runs one round's four steps:

1. **`driver-uris ROUND` (guest).** apt's `--print-uris` lines for the round's packages
   (`DRIVER_PIN` or `DRIVER_OPEN`), each with its hash replaced.
   - The replacement is the SHA256 of the index stanza whose `Filename` the URI ends
     with: "/" followed by the whole `Filename`. A stanza is found by its `Filename`,
     never by package name.
   - The stanzas come from apt's lists, which apt checked against the signed `InRelease`
     at its last update. apt 3.2.0 prints each file's MD5Sum (the slice's run 2 stopped
     on it), and the host does not check MD5.
   - The step stops where the stanzas naming one file disagree on `Size` or SHA256, where
     their `Size` is not apt's, or where a stanza does not carry apt's own hash under the
     same name, or carries a different value.
   - A line that no stanza names, or whose stanza carries no SHA256, goes out as apt
     printed it. The host checks only SHA256 and SHA512, and refuses any other hash type.
2. **The fetch (host).** For each line:
   - The file name is checked: one path component ending `.deb`. The size must be a
     number.
   - A copy in the host's cache is used only if its hash and size match the line.
     Otherwise the file is fetched to a temporary name, checked, and only then moved into
     the cache.
   - Any mismatch stops the round, and no unchecked file stands in the cache.
   - A round that names no file stops.
3. **`put` (host)** puts the round's files into a new `~/debs` in the guest (section 5).
4. **`driver-install ROUND` (guest).**
   - It refuses unless `~/debs` holds `.deb` files and nothing else.
   - It puts them into apt's cache, then **removes `~/debs` whole**, so that the next
     round's `put` makes it afresh.
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

1. **`harness-preflight REPO` (guest).** With `$HOME/.cargo/bin` first on PATH, and in
   REPO, the repository whose toolchain the matrix reads, it refuses before any load
   unless:
   - `rustc` resolves to `$HOME/.cargo/bin/rustc`;
   - `rustc --version` reads `TOOLCHAIN_READING` exactly;
   - `rustup show active-toolchain` answers;
   - `HARNESS_DIR` holds the matrix's entry point.
   `RUSTUP_AUTO_INSTALL=0` is set, so a toolchain that REPO pins and the guest lacks
   refuses rather than starting a download.
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

**`code/test/offline-mock.sh` runs the real `put`, the real `driver-install` and the real
harness preflight, with no virtual machine, no network and no sudo.**

- The mock's guest is a temporary home directory on the host.
- The SSH helper runs each command locally in it.
- `sudo`, `apt-get`, `install` and `rustc` are stubs that record what they are asked to do.
- It exits non-zero on the first check that fails.

Each of the slice's three script defects has a check that reproduces it against the
behaviour that had it, and shows that this behaviour catches it:

| Defect | The check |
|---|---|
| D1, the nested `~/debs` | Two rounds back to back. After the first `driver-install`, no `~/debs` stands. The second `put` lands its files directly in `~/debs`. A `~/debs` holding a directory is refused by `driver-install`. Under the old install, which left `~/debs`, the second `put` is refused instead of nesting. |
| D2, links flattened | A source holding links arrives with the same links. The same source copied by `cp -rL` (the stand-in for `scp -r`) arrives as plain files, and the check tells them apart. |
| D3, the toolchain reading | With an inherited PATH that has no `$HOME/.cargo/bin`, the preflight, run in a stand-in repository, finds the stub rustc there and passes. With no `$HOME/.cargo/bin/rustc`, or a stub reading another line, it refuses. The bare inherited PATH alone reproduces the exit 127. |

The mock also runs the `uris_sha256` cases from the slice:
- a stanza found by exact `Filename`;
- two stanzas that agree;
- disagreeing stanzas;
- a size mismatch;
- no SHA256;
- no stanza;
- apt's own hash differing.

Before a mock passed, the slice's own offline test stubbed `put` and every guest step,
which is how all three defects reached a run.

## 9. What these scripts do not do

**They do not lay or take down an agent, and they do not move a stack between machines.**
The slice's scratch agent was laid and taken down by the measurement's own steps. The
host's stack was carried into the clone after a loader check (the carry check), and the
two CUDA runtimes were compared. Those steps stay with the measurement: they belong to the
slice, not to the template, and the result note names them.

**They do not rebuild the template.** The template of 2026-11-11 stands until the rebuild
act, which also decides the driver pin with the operator: the template holds 615.71.09,
and the slice moved its clone to 620.70.06 by hand.

**`net-probe.sh` is a record, not a path a guest takes.** It measured the two network
backends with a throwaway guest before any template, on 2026-11-11, with a seed and SSH
of its own (`README.md`, "The network"). Sections 2 and 3 do not apply to it, and nothing
else calls it.

**They do not choose the network for a loaded agent.** `restrict=on` with one `guestfwd` is
a domain argument a caller passes. That network is the operator's to rule (`README.md`,
"The network").
