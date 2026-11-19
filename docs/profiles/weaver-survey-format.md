# The appliance's survey report - format weaver-survey/1

**Status:** Merged in pull request 37 (2026-11-17). Revised for the appliance template,
release 5.0, in the `act/appliance-template` stack (pull request 44), not merged.
**Date filed:** 2026-11-17
**Document ID:** `weaver-survey-format`
**Parent:** `weaver-appliance-profile` (this directory), whose image carries the survey
**Class:** format document, per `deployment-profile-convention.md` section 3 in this
directory
**Authority:** the operator's rulings of 2026-11-16 on the setup pages (the survey runs
locally, and its report crosses over TLS), his ruling that the deployment tuple names
cards by model, and his rulings of 2026-11-19 on the appliance template (the image
carries the survey).
**Editorial:** ASCII, absolute dates, prose at 88 characters.

---

## 0. What this document is

It states the format of the report that the appliance's survey makes of its machine,
`weaver-survey/1`. The image's survey writes the report. Weaver Web reads it through
admin-con's `setup report` command, which carries one JSON object of at most 64 KiB and
parses nothing more. The console stores the latest report verbatim, with its stage and
the time it arrived. This document states the report's members, its encoding, what a
reader refuses, how a later version is told apart, and where the survey runs.

A format is a decision that two repositories meet on: the image writes it, and Weaver
Web reads it. The profile decides nothing about any component, so the format is stated
here, as the format document that `deployment-profile-convention` section 3 allows
beside the profile, and it decides this format and nothing else. It requires nothing of
Weaver Web beyond what reading this format means. What the console's
pages do with a member is stated in Weaver Web's own Spec.

## 1. The members

Every member is required, except a mounted drive's members on a drive that is not
mounted (below). Each string is printable ASCII (0x20 to 0x7E) within its byte
bound. Each integer is a JSON integer from 0 to 2^53 - 1. A value that breaks its bound
fails the survey; the survey never truncates a value.

| Member | Meaning | Source, read only | Bound |
| --- | --- | --- | --- |
| `format` | the format and its version | constant | exactly `weaver-survey/1` |
| `stage` | which run of the survey this is | the setup script's argument | `pairing` or `drives` |
| `taken` | when the survey read the machine | `date -u +%Y-%m-%dT%H:%M:%SZ` | RFC 3339, UTC, whole seconds, `Z` |
| `image.version` | the image's version | the `appliance.version` line of `/etc/weaver-appliance/stamp`, which the seal writes as `appliance.version = <version>`: the value after ` = `. The survey fails unless the stamp holds exactly one such line | `^[0-9]+\.[0-9]+$` |
| `image.manifest_sha256` | the image's identity, which the build Spec names as the MANIFEST's sha256 (its section 2). The MANIFEST lists the stamp's own digest, so it pins the stamp too | `sha256sum /etc/weaver-appliance/MANIFEST`, which the seal writes 0644 | 64 lowercase hex digits |
| `kernel.release` | the running kernel | `uname -r` | 64 bytes |
| `machine.virtualization` | what the guest sees under it | `systemd-detect-virt` (`kvm`, or `none` on bare metal) | 32 bytes |
| `machine.vendor`, `machine.product`, `machine.version` | the hypervisor's machine as the guest sees it; in a QEMU guest `version` names the machine type | `/sys/class/dmi/id/sys_vendor`, `product_name` and `product_version`, which any user may read. The serial and UUID files are never read | 128 bytes each |
| `cpu.vendor` | the CPU's vendor string | `/proc/cpuinfo`, `vendor_id` | 32 bytes |
| `cpu.model_name` | the CPU's model name as the guest sees it | `/proc/cpuinfo`, `model name` | 128 bytes |
| `cpu.family`, `cpu.model`, `cpu.stepping` | the CPU's identification numbers | `/proc/cpuinfo`, `cpu family`, `model` and `stepping` | 0 to 65535 each |
| `cpu.logical_cpus` | the CPUs the guest has | the count of `processor` lines in `/proc/cpuinfo` | 1 to 4096 |
| `cpu.flags` | every CPU feature flag, since the math libraries select code paths on them and which ones each selects on is the build's | the `flags` line of processor 0, sorted in byte order, without duplicates. The survey fails if any processor's flags differ | 1 to 512 entries, each `^[a-z0-9_]{1,32}$` |
| `memory.total_bytes` | the memory the guest has | `/proc/meminfo`, `MemTotal` times 1024 | integer |
| `driver.version` | the NVIDIA driver's release | `/sys/module/nvidia/version` where the module is loaded, else `modinfo -F version nvidia`, which reads the module file and loads nothing | `^[0-9]+(\.[0-9]+){1,2}$` |
| `driver.loaded` | whether the driver's kernel module is loaded | `/sys/module/nvidia` present | boolean |
| `cuda_runtime.version` | the CUDA runtime the image carries | the real file name of `libcudart.so.<major>` as `ldconfig -p` finds it, after `libcudart.so.` | `^[0-9]+\.[0-9]+(\.[0-9]+)?$` |
| `cards` | each NVIDIA card, in PCI bus order | `nvidia-smi --query-gpu=pci.bus_id,name,memory.total --format=csv,noheader,nounits`, sorted by address, run only where `driver.loaded` is true, since without the module `nvidia-smi` fails and may load it. `cards` is empty where the module is not loaded, and where `nvidia-smi` exits 6, its documented code for a query that found nothing. Any other failure fails the survey. The survey also fails if sysfs holds an NVIDIA display function (vendor 0x10de, class 0x0300 or 0x0302) that this does not list | 0 to 16 objects |
| `cards[].ordinal` | the card's number in PCI bus order, the number the agent gives its devices and the declaration's devices take | the card's index in that order | equal to the index, from 0 |
| `cards[].name` | the card's model, which names the card in the deployment tuple | `name` | 96 bytes |
| `cards[].pci_address` | the card's address as the guest sees it, the ground of its ordinal | `pci.bus_id`, lowercased, its domain written as sysfs writes it | `^[0-9a-f]{4,8}:[0-9a-f]{2}:[0-9a-f]{2}\.[0-7]$`, strictly ascending |
| `cards[].memory_bytes` | the card's memory as the driver reports it | `memory.total`, in MiB, times 1048576 | integer |
| `drives` | the two mounts the profile fixes | `findmnt -J -b --mountpoint <mount point> -o FSTYPE,LABEL,OPTIONS,SIZE,AVAIL` | exactly two objects, `state` then `models` |
| `drives[].role`, `drives[].mount_point` | which mount | constant | `state` with `/mnt/weaver/state`; `models` with `/mnt/weaver/models` |
| `drives[].mounted` | whether a filesystem is mounted there | `findmnt` finds one | boolean |
| `drives[].label` | the filesystem's label, empty where it has none | `LABEL` | 16 bytes |
| `drives[].filesystem` | the filesystem's type | `FSTYPE` | 16 bytes |
| `drives[].bytes`, `drives[].free_bytes` | the filesystem's size, and the space free to a user | `SIZE`, `AVAIL` | integer |
| `drives[].project_quota` | whether it is mounted with project quotas | `OPTIONS` holds `prjquota` | boolean |

**A drive with `mounted` false carries `role`, `mount_point` and `mounted` alone.** At
the stage `pairing` neither drive is mounted yet. The report states what the survey read
and judges nothing. A box without a card is a valid report: `cards` is empty, and
`driver.version` is read from the module file, with `driver.loaded` false. A box whose
card the module does not serve fails the survey, by the sysfs check of `cards`.

**The report carries none of the following:**
- a card's serial or UUID;
- the machine's serial or UUID;
- a power limit or wattage;
- the microcode revision;
- any disk other than the two mounts;
- any address, port or account.

## 2. The encoding

- One JSON object (RFC 8259), UTF-8, with no byte-order mark. Whitespace is free, and
  one newline may follow the object.
- At most 64 KiB, the bound admin-con enforces. A report of two cards and the whole
  flags line is under 4 KiB.
- The members nest as section 1 names them: `image.version` is the member `version` of
  the object `image`. `cards` and `drives` are arrays of objects.
- The survey writes `format` first. A reader never depends on the order of members.

## 3. What a reader refuses

A reader refuses the whole report, and reads no member of it, for any of the following:

1. Bytes that are not UTF-8, that are not one JSON object, or that carry anything after
   the object except one newline.
2. More than 64 KiB.
3. A member name given twice in one object, at any depth. Parsers differ on which one
   wins, so a reader takes neither.
4. A `format` that is missing, or is not exactly a version string the reader knows.
5. A member of section 1 that is missing or `null`, or of the wrong JSON type. A drive
   with `mounted` false carries `role`, `mount_point` and `mounted` alone, and its other
   members are not missing.
6. An integer that is negative, fractional, written with an exponent, or above
   2^53 - 1.
7. A string outside printable ASCII, over its bound, or not matching its pattern.
8. A `stage` other than `pairing` or `drives`, or a `taken` not in section 1's form.
9. A card whose `ordinal` is not its index, or `pci_address` values that do not strictly
   ascend.
10. `drives` that is not exactly `state` then `models` with their mount points, or a
    drive with `mounted` false that carries a mounted drive's members.
11. An integer outside its bound in section 1, more than 16 cards, or a `cpu.flags` with
    no entry, more than 512 entries, entries out of byte order, or an entry given twice.

**A member the reader does not know is kept and ignored, at any depth.** It is not a
cause to refuse. A refused report stays stored verbatim, and nothing is read from it.

## 4. How a later version is told apart

- **`format` is `weaver-survey/N`**, N a positive integer. A reader reads the versions
  it knows and refuses any other.
- **Within one N, every member keeps its meaning, its unit and its bound.** The survey
  may add an optional member without moving N, since a reader ignores a member it does
  not know.
- **N moves** when a member is removed, renamed, retyped or rebounded, when a member's
  meaning or unit changes, or when an optional member becomes required.
- This document records each version and what changed in it. A reader of N + 1 may also
  read N.

## 5. Where the survey runs, and what it does not do

- **The image carries the survey from release 5.0, at
  `/opt/weaver-appliance/bin/weaver-survey`** (the operator's rulings of 2026-11-19;
  `weaver-appliance-build-Spec` section 2). The image build's proof runs it once on a
  box's first boot, as `nobody` with `--stage pairing`, and its reader must accept the
  output.
- **The appliance's setup script will run it,** in phase 2 of the rulings of 2026-11-19
  (the profile's section 8). The setup script, run as root at the console of the virtual
  machine, runs the survey twice: at the stage `pairing`, once mutual TLS is up, and at
  the stage `drives`, after the drive setup. Each time it hands the survey's standard
  output to admin-con's `setup report`.
- **The survey runs as `nobody`**, so the operating system holds it to reading. Every
  source of section 1 is readable by any user.
- **It writes nothing.** Its output is its standard output alone: no file and no
  temporary file. It loads no module, mounts nothing, changes no setting of a card, and
  opens no socket. The report crosses to the console through admin-con alone.
- **It never gives a partial report.** Where it cannot read a source, or a value breaks
  its bound, it writes nothing to standard output, writes the cause to standard error,
  and exits with a status other than 0.

## 6. A worked example

**Every value of the machine below is made up, and illustrative**: the time, the
MANIFEST's digest, the machine, the CPU, the memory, the cards and their addresses, and
the drives. The image's own facts are release 2.0's. The flags are cut to 20 entries; a
real report carries the whole line. It is a box at the stage `drives`, so every member
shows. At the stage `pairing` the same box reports each drive with `mounted` false and
nothing more.

```json
{
  "format": "weaver-survey/1",
  "stage": "drives",
  "taken": "2026-11-17T09:00:00Z",
  "image": {
    "version": "2.0",
    "manifest_sha256":
      "5b1f0c9e7a2d4486b3e0f19c2d7a65e8c41f03b9d2a7e6158c0b94f3a1d2e7c6"
  },
  "kernel": {"release": "7.3.4-arch1-2"},
  "machine": {
    "virtualization": "kvm",
    "vendor": "QEMU",
    "product": "Standard PC (Q35 + ICH9, 2009)",
    "version": "pc-q35-9.2"
  },
  "cpu": {
    "vendor": "AuthenticAMD",
    "model_name": "AMD EPYC 9354P 32-Core Processor",
    "family": 25,
    "model": 17,
    "stepping": 1,
    "logical_cpus": 16,
    "flags": ["aes", "avx", "avx2", "avx512_bf16", "avx512_vnni", "avx512bw",
              "avx512cd", "avx512dq", "avx512f", "avx512vl", "bmi1", "bmi2", "f16c",
              "fma", "fpu", "sse", "sse2", "sse4_1", "sse4_2", "ssse3"]
  },
  "memory": {"total_bytes": 33071620096},
  "driver": {"version": "620.70.06", "loaded": true},
  "cuda_runtime": {"version": "13.4.92"},
  "cards": [
    {"ordinal": 0, "name": "NVIDIA RTX A5000", "pci_address": "0000:06:00.0",
     "memory_bytes": 25757220864},
    {"ordinal": 1, "name": "NVIDIA RTX A5000", "pci_address": "0000:07:00.0",
     "memory_bytes": 25757220864}
  ],
  "drives": [
    {"role": "state", "mount_point": "/mnt/weaver/state", "mounted": true,
     "label": "weaver-state", "filesystem": "ext4", "bytes": 25191575552,
     "free_bytes": 23863877632, "project_quota": true},
    {"role": "models", "mount_point": "/mnt/weaver/models", "mounted": true,
     "label": "weaver-models", "filesystem": "ext4", "bytes": 135089524736,
     "free_bytes": 72464760832, "project_quota": false}
  ]
}
```

## 7. Versions

| Version | Date | What it states |
| --- | --- | --- |
| `weaver-survey/1` | 2026-11-17 | sections 1 to 6 |

## 8. Open

- **A loaded module with no card.** Whether the open module loads where no card stands,
  and that `nvidia-smi` then exits 6, is read from NVIDIA's documented return codes and
  not yet measured on the image. The next cardless run of a booted image measures it.
- **The setup script's answer to a failed survey.** Settled by the setup script's act,
  in phase 2.
- **The node kind of this document in the document graph.** The graph is suspended until
  release by the ruling of 2026-09-27. Settled by the operator when the graph resumes,
  as for the profile.
