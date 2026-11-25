#!/usr/bin/env python3
"""t-weaver-survey.py: weaver-survey against made-up machines, and its report against a
reader written from weaver-survey/2 sections 1.1 and 3, which reads version 1 too.
Standard library only.

It names nvidia-smi, modinfo, findmnt, ip and systemd-detect-virt, though it runs none of
them: a host that keeps tests naming a card's driver confined runs it through its own
runner (HOST_TESTS, weaver-appliance-build-Spec section 1). test/host/t-survey.sh runs it.

Every machine here is made up and illustrative: its CPU, its cards and their addresses.

Each machine is a fixture tree in a fresh folder (the stamp, the MANIFEST, /proc and /sys
files) and a stand-in command runner that answers by the command's name and records what
it was asked; no command is executed except the survey itself, run once for real at the
end, where it must find no stamp and write nothing. The reader is checked against
WeaverWeb's own worked example of version 1 (tests/fixtures at e0575e65, blob d754dc50),
and against the format document's worked example of version 2 (section 6, kept byte for
byte as a fixture and compared with the document), before it judges any report. Prints each case; exits 1 if any fails. The folder, made under TMPDIR,
is removed when every case passes.
"""
import calendar, hashlib, importlib.machinery, importlib.util, io, json, os, re, shutil, subprocess, sys, tempfile, time

sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
SURVEY = os.path.join(os.path.dirname(HERE), "weaver-survey")
EXAMPLE = os.path.join(HERE, "fixtures", "weaver-survey-1-worked-example.json")
EXAMPLE_SHA = "334959d40dd7c01450782852da4b7ef7982a90f0a6af70cf9c66b30656673acc"
EXAMPLE2 = os.path.join(HERE, "fixtures", "weaver-survey-2-worked-example.json")
FORMAT_DOC = os.path.join(HERE, *[os.pardir] * 7, "docs", "profiles", "weaver-survey-format.md")
# The survey has no .py suffix, as the image carries it, so its loader is named.
spec = importlib.util.spec_from_file_location(
    "weaver_survey", SURVEY, loader=importlib.machinery.SourceFileLoader("weaver_survey", SURVEY))
ws = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ws)

passed = failed = 0


def check(what, cond):
    global passed, failed
    if cond:
        passed += 1; print("ok    " + what)
    else:
        failed += 1; print("FAIL  " + what)


# ---- The reader: weaver-survey/2 sections 1.1 and 3, and version 1 (survey_reader.py). ----
sys.path.insert(0, HERE)
from survey_reader import Refuse, reader, refused


# ---- The reader against WeaverWeb's worked example, before it judges anything. ----
ex = open(EXAMPLE, "rb").read()
check("the worked example is WeaverWeb's at e0575e65, by digest", hashlib.sha256(ex).hexdigest() == EXAMPLE_SHA)
check("the reader takes the worked example", not refused(ex))
exo = json.loads(ex)


def mutated(f):
    o = json.loads(ex); f(o); return json.dumps(o).encode()


def refusal(raw):
    """The reader's refusal, "" where it takes raw, or the exception it raised where it
    raised one other than Refuse, so a reader that crashes fails its case by name."""
    try:
        reader(raw)
    except Refuse as e:
        return str(e)
    except Exception as e:
        return "raised %s: %s" % (type(e).__name__, e)
    return ""


check("the reader refuses a member given twice", refused(ex.replace(b'"stage": "drives",', b'"stage": "drives", "stage": "drives",')))
check("the reader refuses a known integer written as a fraction, as not an integer",
      refusal(ex.replace(b'"logical_cpus": 16', b'"logical_cpus": 16.0')) == "cpu.logical_cpus is not an integer")
check("the reader refuses a known integer written with an exponent, as not an integer",
      refusal(ex.replace(b'"logical_cpus": 16', b'"logical_cpus": 1.6e1')) == "cpu.logical_cpus is not an integer")
check("the reader refuses a missing member", refused(mutated(lambda o: o["kernel"].pop("release"))))
check("the reader refuses flags out of order", refused(mutated(lambda o: o["cpu"]["flags"].reverse())))
check("the reader refuses a second newline after the object", refused(ex.rstrip(b"\n") + b"\n\n"))
check("the reader refuses an unmounted drive with a label", refused(mutated(lambda o: o["drives"][0].update(mounted=False))))
check("the reader refuses ordinals out of index", refused(mutated(lambda o: o["cards"][1].update(ordinal=0))))
check("the reader keeps and ignores an unknown member", not refused(mutated(lambda o: o["cpu"].update(extra=[1, 2]))))
check("the reader keeps and ignores an unknown member holding a fraction and an exponent, at depth",
      not refused(mutated(lambda o: o["cpu"].update(extra={"deep": [1.5, 2.5e-3]}))))
check("the reader refuses NaN or Infinity, even in an unknown member",
      refused(ex.replace(b'"logical_cpus": 16', b'"logical_cpus": 16, "extra": NaN'))
      and refused(ex.replace(b'"logical_cpus": 16', b'"logical_cpus": 16, "extra": -Infinity')))

# ---- The reader against the format's own worked example of version 2. ----
ex2 = open(EXAMPLE2, "rb").read()
blocks = re.findall(r"```json\n(.*?)```", open(FORMAT_DOC).read(), re.S)
check("the version 2 example is the format document's section 6, byte for byte",
      len(blocks) == 1 and blocks[0].encode() == ex2)
check("the reader takes the version 2 worked example", not refused(ex2))


def mutated2(f):
    o = json.loads(ex2); f(o); return json.dumps(o).encode()


def net(o, plane, **kv):
    o["network"][plane].update(kv)


check("version 2: a stage of drives is refused", refusal(mutated2(lambda o: o.update(stage="drives"))) == "stage 'drives'")
check("version 1 still reads its stage of drives", not refused(ex))
check("version 2: a place outside the one spelling is refused",
      all(refused(mutated2(lambda o, v=v: o["drives"][1].update(mount_point=v)))
          for v in ("/srv//weaver-models", "/srv/weaver-models/", "/srv/./models", "/", "srv/models", "/srv/weaver models")))
check("version 2: two equal places, or one inside the other, are refused",
      refused(mutated2(lambda o: o["drives"][1].update(mount_point="/mnt/weaver/state")))
      and refused(mutated2(lambda o: o["drives"][1].update(mount_point="/mnt/weaver/state/models")))
      and refused(mutated2(lambda o: o["drives"][0].update(mount_point="/srv"))))
check("version 2: a place beside another of the same prefix is taken",
      not refused(mutated2(lambda o: o["drives"][1].update(mount_point="/mnt/weaver/statex"))))
check("version 2: no network is refused", refused(mutated2(lambda o: o.pop("network"))))
check("version 2: an address outside the one spelling is refused",
      all(refused(mutated2(lambda o, v=v: net(o, "data", address=v)))
          for v in ("198.51.100.010", "2001:DB8::10", "2001:db8:0:0:0:0:0:10", "2001:db8::0:10")))
check("version 2: an address of a refused kind is refused",
      all(refused(mutated2(lambda o, v=v: net(o, "data", address=v)))
          for v in ("0.0.0.0", "::", "169.254.1.1", "fe80::1", "224.0.0.1", "ff02::1", "255.255.255.255", "::ffff:198.51.100.10")))
check("version 2: a loopback address is taken", not refused(mutated2(lambda o: net(o, "data", address="127.0.0.1", prefix_length=8))))
check("version 2: a prefix length outside its family's bound is refused",
      all(refusal(mutated2(lambda o, n=n: (net(o, "data", prefix_length=n), o["network"].update(planes="shared"))))
          == "network.data.prefix_length %d outside 1 to 32" % n for n in (33, 0)))
check("version 1: a mount point other than the constant is refused",
      refused(mutated(lambda o: o["drives"][1].update(mount_point="/srv/weaver-models"))))
check("version 2: an IPv6 plane takes a prefix length up to 128",
      not refused(mutated2(lambda o: net(o, "data", address="2001:db8::10", prefix_length=128))))
check("version 2: planes other than separate or shared are refused", refused(mutated2(lambda o: o["network"].update(planes="both"))))
check("version 2: separate planes on one address, or in one network, are refused",
      refused(mutated2(lambda o: net(o, "data", address="192.0.2.10")))
      and refused(mutated2(lambda o: net(o, "data", address="192.0.2.99")))
      and refused(mutated2(lambda o: (net(o, "data", address="192.0.2.99", prefix_length=16)))))
check("version 2: shared planes in one network are taken",
      not refused(mutated2(lambda o: (net(o, "data", address="192.0.2.99"), o["network"].update(planes="shared")))))
check("version 2: separate planes where only the data plane's network holds the management address are refused",
      refused(mutated2(lambda o: (net(o, "management", address="192.0.2.10", prefix_length=24),
                                  net(o, "data", address="192.0.3.10", prefix_length=16)))))
check("version 2: separate planes where only the management plane's network holds the data address are refused",
      refused(mutated2(lambda o: (net(o, "management", address="192.0.3.10", prefix_length=16),
                                  net(o, "data", address="192.0.2.10", prefix_length=24)))))
check("version 2: an address with a zone is refused, and named so",
      refusal(mutated2(lambda o: (net(o, "management", address="2001:db8::10%eth0", prefix_length=64))))
      == "network.management.address '2001:db8::10%eth0' carries a zone")
check("version 2: a place holding a lone surrogate is refused, not raised",
      refusal(b'{"x":1}'.replace(b'{"x":1}', mutated2(lambda o: o["drives"][1].update(mount_point="/srv/\udc80"))))
      == "drives[1].mount_point '/srv/\\udc80' is not a place in the one spelling")
check("version 2: a place with a .. part, and one of 256 bytes, are refused, and one of 255 taken",
      refused(mutated2(lambda o: o["drives"][1].update(mount_point="/srv/../models")))
      and refused(mutated2(lambda o: o["drives"][1].update(mount_point="/" + "a" * 200 + "/" + "b" * 54)))
      and not refused(mutated2(lambda o: o["drives"][1].update(mount_point="/" + "a" * 200 + "/" + "b" * 53))))
check("version 2: a taken of second 60 is refused, and of second 59 taken",
      refusal(mutated2(lambda o: o.update(taken="2026-12-31T23:59:60Z"))) == "taken '2026-12-31T23:59:60Z'"
      and not refused(mutated2(lambda o: o.update(taken="2026-12-31T23:59:59Z"))))
check("version 2: a taken with a lowercase t or z, or a date that does not exist, is refused",
      all(refused(mutated2(lambda o, v=v: o.update(taken=v)))
          for v in ("2026-11-17t09:00:00Z", "2026-11-17T09:00:00z", "2026-02-30T09:00:00Z", "2026-11-17T24:00:00Z")))
check("version 2: -0 where an integer is known is refused, and kept in a member the reader does not know",
      refusal(ex2.replace(b'"family": 25', b'"family": -0')) == "cpu.family is not an integer"
      and not refused(ex2.replace(b'"stage": "pairing",', b'"stage": "pairing", "later": -0,')))
check("version 2: -0 where a string is known is refused",
      refusal(ex2.replace(b'"label": "weaver-state"', b'"label": -0')) == "state.label is not str")
check("version 2: cards ascend by number, not by bytes",
      not refused(mutated2(lambda o: (o["cards"][0].update(pci_address="ffff:06:00.0"),
                                      o["cards"][1].update(pci_address="10000:06:00.0"))))
      and refused(mutated2(lambda o: (o["cards"][0].update(pci_address="10000:06:00.0"),
                                      o["cards"][1].update(pci_address="ffff:06:00.0")))))
check("version 2: the report's 64 KiB counts its newline: 65,536 bytes taken, 65,537 refused",
      ex2.endswith(b"}\n") and not refused(b" " * (65536 - len(ex2)) + ex2)
      and refusal(b" " * (65537 - len(ex2)) + ex2) == "over 64 KiB")
check("version 2: whitespace before the object is taken, and after it only one newline",
      not refused(b" \n\t" + ex2) and refused(ex2 + b"\n") and refused(ex2.rstrip(b"\n") + b"\r\n"))

# ---- Made-up machines. ----
T = tempfile.mkdtemp(prefix="t-weaver-survey.")
FLAGS = "fpu sse sse2 avx avx2 fma aes bmi1 bmi2 sse4_1 sse4_2 ssse3 f16c avx512f avx512bw"
NOW = time.gmtime(1795000000)


def write(root, path, data, mode="w"):
    p = os.path.join(root, path.lstrip("/"))
    os.makedirs(os.path.dirname(p), exist_ok=True)
    with open(p, mode) as f:
        f.write(data)


def machine_root(name, version="4.0", stamp_extra="", loaded=True, nvidia_fns=("0000:06:00.0", "0000:07:00.0"),
                 procs=2, flags=None, model_name="Illustrative CPU 16-Core Processor", sys_vendor="QEMU\n"):
    root = os.path.join(T, name)
    write(root, "/etc/weaver-appliance/stamp",
          "sealed = 2026-11-19T00:00:00Z\nappliance.version = %s\n%sweaveragent.commit = b74062bf\n" % (version, stamp_extra))
    write(root, "/etc/weaver-appliance/MANIFEST", "abc  /etc/weaver-appliance/stamp\n")
    write(root, "/proc/sys/kernel/osrelease", "7.3.4-arch1-2\n")
    write(root, "/sys/class/dmi/id/sys_vendor", sys_vendor)
    write(root, "/sys/class/dmi/id/product_name", "Standard PC (Q35 + ICH9, 2009)\n")
    write(root, "/sys/class/dmi/id/product_version", "pc-q35-9.2\n")
    blocks = []
    for n in range(procs):
        f = (flags[n] if isinstance(flags, list) else flags) if flags is not None else FLAGS
        blocks.append("processor\t: %d\nvendor_id\t: AuthenticAMD\ncpu family\t: 25\nmodel\t\t: 1\n"
                      "model name\t: %s\nstepping\t: 1\nflags\t\t: %s\n" % (n, model_name, f))
    write(root, "/proc/cpuinfo", "\n".join(blocks) + "\n")
    write(root, "/proc/meminfo", "MemTotal:       98765432 kB\nMemFree:        12345678 kB\n")
    if loaded:
        write(root, "/sys/module/nvidia/version", "620.70.06\n")
    dev = os.path.join(root, "sys/bus/pci/devices")
    os.makedirs(dev, exist_ok=True)
    for a, v, c in [("0000:00:01.0", "0x8086", "0x060000")] + [(a, "0x10de", "0x030000") for a in nvidia_fns] \
            + [(a[:-1] + "1", "0x10de", "0x040300") for a in nvidia_fns]:
        write(root, "/sys/bus/pci/devices/%s/vendor" % a, v + "\n")
        write(root, "/sys/bus/pci/devices/%s/class" % a, c + "\n")
    os.makedirs(os.path.join(root, "usr/lib"), exist_ok=True)
    write(root, "/usr/lib/libcudart.so.13.0.96", "")
    os.symlink("libcudart.so.13.0.96", os.path.join(root, "usr/lib/libcudart.so.13"))
    return root


FINDMNT_STATE = json.dumps({"filesystems": [{"fstype": "ext4", "label": "weaver-state", "options": "rw,relatime,prjquota",
                                             "size": 16729894912, "avail": 15832121344}]})
FINDMNT_MODELS = json.dumps({"filesystems": [{"fstype": "ext4", "label": None, "options": "rw,relatime",
                                              "size": "84230021120", "avail": "79831982080"}]})


def ip_card(name, state, *addrs):
    """A card as ip -j addr show lists it: addrs are (address, prefix length, scope)."""
    return {"ifindex": 1, "ifname": name, "operstate": state,
            "addr_info": [{"family": "inet6" if ":" in a else "inet", "local": a, "prefixlen": n, "scope": sc}
                          for a, n, sc in addrs]}


IP_ADDR = json.dumps([ip_card("lo", "UNKNOWN", ("127.0.0.1", 8, "host"), ("::1", 128, "host")),
                      ip_card("enp1s0", "UP", ("192.0.2.10", 24, "global"), ("2001:db8::10", 64, "global"),
                              ("fe80::5054:ff:feaa:101", 64, "link")),
                      ip_card("enp2s0", "UP", ("198.51.100.10", 24, "global"), ("198.51.100.20", 24, "link")),
                      ip_card("enp3s0", "DOWN", ("203.0.113.10", 24, "global"))])
PLACES = ("/mnt/weaver/state", "/mnt/weaver/models")


class Runner:
    """A stand-in for the survey's commands: answers by name, records each call, runs nothing."""

    def __init__(self, **over):
        self.calls = []
        self.answers = {
            "systemd-detect-virt": (0, "kvm\n"),
            "modinfo": (0, "620.70.06\n"),
            "ldconfig": (0, "1234 libs found in cache `/etc/ld.so.cache'\n"
                            "\tlibcudart.so.13 (libc6,x86-64) => /usr/lib/libcudart.so.13\n"
                            "\tlibc.so.6 (libc6,x86-64) => /usr/lib/libc.so.6\n"),
            "nvidia-smi": (0, "00000000:07:00.0, NVIDIA RTX A5000, 24564\n00000000:06:00.0, NVIDIA RTX A5000, 24564\n"),
            "findmnt:/mnt/weaver/state": (1, ""),
            "findmnt:/mnt/weaver/models": (1, ""),
            "ip": (0, IP_ADDR),
        }
        self.answers.update(over)

    def __call__(self, argv, timeout=60):
        self.calls.append(list(argv))
        key = argv[0] + (":" + argv[4] if argv[0] == "findmnt" else "")
        # A place no case names has nothing mounted there.
        return self.answers.get(key, (1, "")) if argv[0] == "findmnt" else self.answers[key]


def run_survey(root, stage="pairing", places=PLACES, management="192.0.2.10/24", data="198.51.100.10/24",
               planes="separate", **over):
    r = Runner(**over)
    try:
        return ws.survey(stage, places[0], places[1], management, data, planes, root=root, runner=r, now=NOW), r, None
    except ws.Fail as e:
        return None, r, str(e)


# 1. Two cards, loaded, before the drives are mounted.
root = machine_root("two-cards")
body, r, why = run_survey(root)
check("two cards, nothing mounted: a report (%s)" % why, body is not None)
o = json.loads(body) if body else {}
check("the reader takes it", body is not None and not refused(body))
check("format first, then stage and taken, network last", list(o)[:3] == ["format", "stage", "taken"] and list(o)[-1] == "network")
check("format is weaver-survey/2 and stage is pairing", o.get("format") == "weaver-survey/2" and o.get("stage") == "pairing")
check("network: the two planes' addresses and prefix lengths as given, and separate",
      o.get("network") == {"management": {"address": "192.0.2.10", "prefix_length": 24},
                           "data": {"address": "198.51.100.10", "prefix_length": 24}, "planes": "separate"})
check("ip asked exactly ip -j addr show, once", [c for c in r.calls if c[0] == "ip"] == [["ip", "-j", "addr", "show"]])
check("one newline after the object, ASCII alone", body is not None and body.endswith(b"}\n") and not body.endswith(b"\n\n") and body.isascii())
check("taken is the time read", o.get("taken") == time.strftime("%Y-%m-%dT%H:%M:%SZ", NOW))
check("image.version is the stamp's 4.0", o.get("image", {}).get("version") == "4.0")
check("image.manifest_sha256 is the MANIFEST's sha256",
      o.get("image", {}).get("manifest_sha256") == hashlib.sha256(b"abc  /etc/weaver-appliance/stamp\n").hexdigest())
check("machine: kvm and the three DMI values without their newline",
      o.get("machine") == {"virtualization": "kvm", "vendor": "QEMU", "product": "Standard PC (Q35 + ICH9, 2009)", "version": "pc-q35-9.2"})
check("cpu: the numbers, two logical CPUs", (o.get("cpu", {}).get("family"), o.get("cpu", {}).get("model"),
                                              o.get("cpu", {}).get("stepping"), o.get("cpu", {}).get("logical_cpus")) == (25, 1, 1, 2))
check("cpu.flags sorted in byte order, without duplicates",
      o.get("cpu", {}).get("flags") == sorted(set(FLAGS.split()), key=lambda s: s.encode()))
check("memory.total_bytes is MemTotal times 1024", o.get("memory", {}).get("total_bytes") == 98765432 * 1024)
check("driver: the loaded module's version, loaded", o.get("driver") == {"version": "620.70.06", "loaded": True})
check("modinfo is not asked where the module is loaded", not any(c[0] == "modinfo" for c in r.calls))
check("cuda_runtime.version is the real file's name after libcudart.so.", o.get("cuda_runtime") == {"version": "13.0.96"})
check("cards in PCI bus order, the domain as sysfs writes it, MiB times 1048576",
      o.get("cards") == [{"ordinal": 0, "name": "NVIDIA RTX A5000", "pci_address": "0000:06:00.0", "memory_bytes": 24564 * 1048576},
                         {"ordinal": 1, "name": "NVIDIA RTX A5000", "pci_address": "0000:07:00.0", "memory_bytes": 24564 * 1048576}])
check("nvidia-smi asked exactly the format's query", [c for c in r.calls if c[0] == "nvidia-smi"] == [ws.NVIDIA_SMI])
check("the drives not mounted: role, mount point and mounted false alone",
      o.get("drives") == [{"role": "state", "mount_point": "/mnt/weaver/state", "mounted": False},
                          {"role": "models", "mount_point": "/mnt/weaver/models", "mounted": False}])
check("findmnt asked the format's columns at each mount point",
      [c for c in r.calls if c[0] == "findmnt"] == [["findmnt", "-J", "-b", "--mountpoint", mp, "-o", "FSTYPE,LABEL,OPTIONS,SIZE,AVAIL"]
                                                    for mp in ("/mnt/weaver/state", "/mnt/weaver/models")])
check("no command beyond the six", {c[0] for c in r.calls} <= {"systemd-detect-virt", "modinfo", "ldconfig", "nvidia-smi", "findmnt", "ip"})

# 2. The same machine with both drives mounted.
body, r, why = run_survey(root, **{"findmnt:/mnt/weaver/state": (0, FINDMNT_STATE), "findmnt:/mnt/weaver/models": (0, FINDMNT_MODELS)})
o = json.loads(body) if body else {}
check("both mounted: a report the reader takes (%s)" % why, body is not None and not refused(body))
check("the state drive: label, ext4, sizes and the project quota",
      o.get("drives", [{}])[0] == {"role": "state", "mount_point": "/mnt/weaver/state", "mounted": True, "label": "weaver-state",
                                   "filesystem": "ext4", "bytes": 16729894912, "free_bytes": 15832121344, "project_quota": True})
check("the models drive: no label read as empty, sizes given as strings read as integers, no project quota",
      o.get("drives", [{}, {}])[1] == {"role": "models", "mount_point": "/mnt/weaver/models", "mounted": True, "label": "",
                                       "filesystem": "ext4", "bytes": 84230021120, "free_bytes": 79831982080, "project_quota": False})

# 2b. The operator's own places: findmnt reads them, and the report names them.
mine = ("/var/lib/weaver-state", "/srv/weaver-models")
body, r, why = run_survey(root, places=mine, **{"findmnt:/var/lib/weaver-state": (0, FINDMNT_STATE), "findmnt:/srv/weaver-models": (1, "")})
o = json.loads(body) if body else {}
check("the operator's own places: a report the reader takes (%s)" % why, body is not None and not refused(body))
check("the operator's own places: findmnt asked at each, and each named as its mount point",
      [c[4] for c in r.calls if c[0] == "findmnt"] == list(mine) and [d.get("mount_point") for d in o.get("drives", [])] == list(mine))

# 2c. Shared planes on one card, the management plane on IPv6.
body, r, why = run_survey(root, management="2001:db8::10/64", data="192.0.2.10/24", planes="shared")
o = json.loads(body) if body else {}
check("shared planes on one card, the management plane on IPv6: a report the reader takes (%s)" % why,
      body is not None and not refused(body) and o["network"] == {"management": {"address": "2001:db8::10", "prefix_length": 64},
                                                                   "data": {"address": "192.0.2.10", "prefix_length": 24},
                                                                   "planes": "shared"})

# 2d. The format's worked example of version 2, written by the program itself: the example's
# machine laid out, the program driven through main with its options in another order and
# all four values different, and its bytes compared with the example as the program writes
# a report (section 2: an indent of one space, the members in the example's order, one
# newline). A swap of two values between arguments and survey, a change of indent, or a
# change of a member's order goes red here.
EX_FLAGS = ("sse4_2 avx512vl fpu avx2 sse aes avx512f bmi1 fma avx ssse3 avx512bw sse2 "
            "f16c avx512cd bmi2 avx512dq sse4_1 avx512_vnni avx512_bf16")
ex_root = machine_root("worked-example", version="2.0", procs=16, flags=EX_FLAGS,
                       model_name="AMD EPYC 9354P 32-Core Processor")
write(ex_root, "/proc/cpuinfo", open(os.path.join(ex_root, "proc/cpuinfo")).read().replace("model\t\t: 1\n", "model\t\t: 17\n"))
write(ex_root, "/proc/meminfo", "MemTotal:       32296504 kB\nMemFree:        12345678 kB\n")
os.remove(os.path.join(ex_root, "usr/lib/libcudart.so.13"))
write(ex_root, "/usr/lib/libcudart.so.13.4.92", "")
os.symlink("libcudart.so.13.4.92", os.path.join(ex_root, "usr/lib/libcudart.so.13"))
ex_runner = Runner(**{
    "findmnt:/mnt/weaver/state": (0, json.dumps({"filesystems": [{"fstype": "ext4", "label": "weaver-state",
        "options": "rw,relatime,prjquota", "size": 25191575552, "avail": 23863877632}]})),
    "findmnt:/srv/weaver-models": (0, json.dumps({"filesystems": [{"fstype": "ext4", "label": "weaver-models",
        "options": "rw,relatime", "size": "135089524736", "avail": "72464760832"}]}))})


class Captured:
    def __init__(self):
        self.buffer = io.BytesIO()
        self.text = ""

    def write(self, t):
        self.text += t


out_cap, err_cap, saved = Captured(), Captured(), (sys.stdout, sys.stderr)
sys.stdout, sys.stderr = out_cap, err_cap
try:
    ex_rc = ws.main(["weaver-survey", "--planes", "separate", "--data", "198.51.100.10/24",
                     "--models-at", "/srv/weaver-models", "--management", "192.0.2.10/24",
                     "--stage", "pairing", "--state-at", "/mnt/weaver/state"],
                    root=ex_root, runner=ex_runner, now=time.gmtime(calendar.timegm((2026, 11, 17, 9, 0, 0))))
finally:
    sys.stdout, sys.stderr = saved
written = out_cap.buffer.getvalue()
check("the worked example through main: exit 0, nothing on standard error (%r)" % err_cap.text[:120],
      ex_rc == 0 and err_cap.text == "")
check("the worked example through main: the program's values are the example's", written and json.loads(written) == json.loads(ex2))
check("the worked example through main: its bytes are the example as the program writes a report",
      written == (json.dumps(json.loads(ex2), ensure_ascii=True, indent=1) + "\n").encode("ascii"))

# 2e. One address on both planes under shared: taken.
body, r, why = run_survey(root, management="192.0.2.10/24", data="192.0.2.10/24", planes="shared")
check("one address on both planes under shared: a report the reader takes (%s)" % why,
      body is not None and not refused(body) and json.loads(body)["network"]["data"]["address"] == "192.0.2.10")

# 3. No card, the module not loaded: the cardless rehearsal's machine.
root3 = machine_root("cardless", loaded=False, nvidia_fns=())
body, r, why = run_survey(root3)
o = json.loads(body) if body else {}
check("no card: a report the reader takes (%s)" % why, body is not None and not refused(body))
check("no card: cards empty, the driver read from the module file, not loaded",
      o.get("cards") == [] and o.get("driver") == {"version": "620.70.06", "loaded": False})
check("no card: modinfo asked the format's question; nvidia-smi never run",
      ["modinfo", "-F", "version", "nvidia"] in r.calls and not any(c[0] == "nvidia-smi" for c in r.calls))

# 3b. A domain of eight hex digits, the widest the format allows, is kept whole as sysfs
# writes it: 16 characters.
rootw = machine_root("wide-domain", nvidia_fns=("1000abcd:06:00.0",))
body, r, why = run_survey(rootw, **{"nvidia-smi": (0, "1000ABCD:06:00.0, NVIDIA RTX A5000, 24564\n")})
check("a domain of eight hex digits: the card's address kept whole, 16 characters (%s)" % why,
      body is not None and [c["pci_address"] for c in json.loads(body)["cards"]] == ["1000abcd:06:00.0"])

# 4. Loaded, nvidia-smi exits 6 (found nothing), no NVIDIA display function in sysfs.
root4 = machine_root("loaded-none", nvidia_fns=())
body, r, why = run_survey(root4, **{"nvidia-smi": (6, "")})
check("loaded, nvidia-smi exit 6, nothing in sysfs: cards empty (%s)" % why, body is not None and json.loads(body)["cards"] == [])

# 5. Refusals: each writes nothing and names its cause.
cases = [
    ("nvidia-smi exits 9", root, {"nvidia-smi": (9, "")}, "nvidia-smi exited 9"),
    ("a card in sysfs the driver does not list", machine_root("unlisted"), {"nvidia-smi": (0, "00000000:06:00.0, NVIDIA RTX A5000, 24564\n")},
     "0000:07:00.0"),
    ("a card in sysfs with the module not loaded", machine_root("unserved", loaded=False), {}, "does not list"),
    ("two appliance.version lines", machine_root("two-versions", stamp_extra="appliance.version = 4.1\n"), {}, "2 appliance.version"),
    ("a version that is not N.N", machine_root("bad-version", version="4"), {}, "image.version"),
    ("processors whose flags differ", machine_root("flags-differ", flags=[FLAGS, FLAGS + " sha_ni"]), {}, "flags differ"),
    ("a flag outside the pattern", machine_root("bad-flag", flags="fpu SSE"), {}, "cpu.flags entry"),
    ("a model name over 128 bytes", machine_root("long-model", model_name="X" * 129), {}, "over 128"),
    ("a DMI value outside printable ASCII", machine_root("dmi-utf8", sys_vendor="Q\u00e9MU\n"), {}, "printable ASCII"),
    ("systemd-detect-virt fails other than none", root, {"systemd-detect-virt": (2, "")}, "systemd-detect-virt"),
    ("modinfo cannot find the module", root3, {"modinfo": (1, "")}, "modinfo"),
    ("no libcudart in the cache", root, {"ldconfig": (0, "\tlibc.so.6 (libc6,x86-64) => /usr/lib/libc.so.6\n")}, "libcudart"),
    ("two libcudart majors", root, {"ldconfig": (0, "\tlibcudart.so.13 (libc6,x86-64) => /usr/lib/libcudart.so.13\n"
                                                  "\tlibcudart.so.12 (libc6,x86-64) => /usr/lib/libcudart.so.12\n")}, "libcudart"),
    ("findmnt fails other than nothing mounted", root, {"findmnt:/mnt/weaver/state": (2, "")}, "findmnt"),
    ("two filesystems at one mount point", root, {"findmnt:/mnt/weaver/state": (0, json.dumps({"filesystems": [json.loads(FINDMNT_STATE)["filesystems"][0]] * 2}))},
     "not one"),
    ("a label over 16 bytes", root, {"findmnt:/mnt/weaver/state": (0, FINDMNT_STATE.replace("weaver-state", "weaver-state-long-x"))}, "over 16"),
    ("a domain of nine hex digits", machine_root("nine-digit", nvidia_fns=()),
     {"nvidia-smi": (0, "100000000:06:00.0, NVIDIA RTX A5000, 24564\n")}, "gave the address"),
    ("seventeen cards", machine_root("seventeen", nvidia_fns=["0000:%02x:00.0" % (0x10 + n) for n in range(17)]),
     {"nvidia-smi": (0, "".join("00000000:%02X:00.0, NVIDIA RTX A5000, 24564\n" % (0x10 + n) for n in range(17)))}, "over 16"),
]
for what, rt, over, cause in cases:
    body, r, why = run_survey(rt, **over)
    check("%s: no report, and the cause names it (%s)" % (what, why), body is None and why is not None and cause in why)

# 5b. Refusals of the arguments and the planes: each writes nothing and names its cause.
ONE_NET = json.dumps([ip_card("enp1s0", "UP", ("192.0.2.10", 24, "global")),
                      ip_card("enp2s0", "UP", ("192.0.2.99", 24, "global"))])
args_cases = [
    ("a stage of drives", dict(stage="drives"), {}, "not pairing"),
    ("planes other than separate or shared", dict(planes="both"), {}, "not separate or shared"),
    ("a place with an empty part", dict(places=("/mnt//state", "/mnt/weaver/models")), {}, "one spelling"),
    ("a place with a trailing slash", dict(places=("/mnt/weaver/state/", "/mnt/weaver/models")), {}, "one spelling"),
    ("a place of / alone", dict(places=("/", "/mnt/weaver/models")), {}, "one spelling"),
    ("a place with a .. part", dict(places=("/mnt/../state", "/mnt/weaver/models")), {}, "one spelling"),
    ("a relative place", dict(places=("mnt/state", "/mnt/weaver/models")), {}, "one spelling"),
    ("a place over 255 bytes", dict(places=("/" + "a" * 200 + "/" + "b" * 60, "/mnt/weaver/models")), {}, "one spelling"),
    ("two equal places", dict(places=("/mnt/weaver/state", "/mnt/weaver/state")), {}, "equal"),
    ("one place inside the other", dict(places=("/mnt/weaver", "/mnt/weaver/models")), {}, "inside"),
    ("an address in another spelling", dict(management="2001:DB8::10/64"), {}, "which writes it 2001:db8::10"),
    ("an IPv4 address with a leading zero", dict(management="192.0.2.010/24"), {}, "not an IPv4 or IPv6 address"),
    ("an unspecified address", dict(management="0.0.0.0/8"), {}, "refuses"),
    ("a link-local address", dict(management="fe80::5054:ff:feaa:101/64"), {}, "refuses"),
    ("an IPv4 address written as IPv6", dict(data="::ffff:198.51.100.10/96"), {}, "refuses"),
    ("no prefix length", dict(management="192.0.2.10"), {}, "<address>/<prefix length>"),
    ("a prefix length over 32 for IPv4", dict(management="192.0.2.10/33"), {}, "outside 1 to 32"),
    ("a prefix length with a leading zero", dict(management="192.0.2.10/024"), {}, "<address>/<prefix length>"),
    ("separate planes on one address", dict(data="192.0.2.10/24"), {}, "both are the address"),
    ("an address ip does not list", dict(data="198.51.100.11/24"), {}, "is not listed"),
    ("an address on a card that is down", dict(data="203.0.113.10/24"), {}, "on a card that is up"),
    ("an address with another prefix length", dict(data="198.51.100.10/16"), {}, "prefix length 16"),
    ("a loopback address, of host scope", dict(data="127.0.0.1/8"), {}, "global scope"),
    ("an address of link scope on a card that is up", dict(data="198.51.100.20/24"), {}, "global scope"),
    ("separate planes on one card", dict(data="2001:db8::10/64"), {}, "both addresses are on the card enp1s0"),
    ("separate planes in one network", dict(data="192.0.2.99/24"), {"ip": (0, ONE_NET)}, "lie in one network"),
    ("ip -j addr show fails", {}, {"ip": (1, "")}, "ip -j addr show exited 1"),
    ("ip -j addr show gives no JSON", {}, {"ip": (0, "enp1s0 UP")}, "ip -j addr show gave"),
    ("the state place inside the models place", dict(places=("/mnt/weaver/models/state", "/mnt/weaver/models")), {}, "inside"),
    ("a place with a character outside the one spelling", dict(places=("/mnt/weaver state", "/mnt/weaver/models")), {}, "one spelling"),
    ("a place with a . part", dict(places=("/mnt/./state", "/mnt/weaver/models")), {}, "one spelling"),
    ("a multicast address", dict(management="224.0.0.1/24"), {}, "refuses"),
    ("the limited broadcast address", dict(data="255.255.255.255/32"), {}, "refuses"),
    ("an address with a zone", dict(management="2001:db8::10%enp1s0/64"), {}, "carries a zone"),
    ("separate planes where only the data plane's network holds the management address",
     dict(management="192.0.2.10/24", data="192.0.3.10/16"),
     {"ip": (0, json.dumps([ip_card("enp1s0", "UP", ("192.0.2.10", 24, "global")),
                            ip_card("enp2s0", "UP", ("192.0.3.10", 16, "global"))]))}, "lie in one network"),
    ("separate planes where only the management plane's network holds the data address",
     dict(management="192.0.3.10/16", data="192.0.2.10/24"),
     {"ip": (0, json.dumps([ip_card("enp1s0", "UP", ("192.0.3.10", 16, "global")),
                            ip_card("enp2s0", "UP", ("192.0.2.10", 24, "global"))]))}, "lie in one network"),
]
for what, kw, over, cause in args_cases:
    body, r, why = run_survey(root, **kw, **over)
    check("%s: no report, and the cause names it (%s)" % (what, why), body is None and why is not None and cause in why)

# 6. The script as written: it opens nothing for writing, no socket, no temporary file.
import ast
tree = ast.parse(open(SURVEY).read())
imported = {a.name for n in ast.walk(tree) if isinstance(n, (ast.Import, ast.ImportFrom)) for a in n.names} \
    | {n.module for n in ast.walk(tree) if isinstance(n, ast.ImportFrom)}
check("the survey imports %s alone: no socket, no temporary file" % sorted(imported),
      imported <= {"hashlib", "ipaddress", "json", "os", "re", "subprocess", "sys", "time"})
opens = [n for n in ast.walk(tree) if isinstance(n, ast.Call) and isinstance(n.func, ast.Name) and n.func.id == "open"]
modes = [n.args[1].value if len(n.args) > 1 and isinstance(n.args[1], ast.Constant) else ("r" if len(n.args) < 2 else None)
         for n in opens]
check("every open() reads: modes %s" % modes, opens and all(m in ("r", "rb") for m in modes) and not any(k.arg == "mode" for n in opens for k in n.keywords))
writes = sorted({n.attr for n in ast.walk(tree) if isinstance(n, ast.Attribute)
                 and n.attr in ("mkdir", "makedirs", "remove", "unlink", "rename", "replace", "chmod", "chown", "symlink", "rmdir", "truncate")})
check("no os call that writes, removes or renames (%s)" % writes, not writes)
consts = {n.value for n in ast.walk(tree) if isinstance(n, ast.Constant) and isinstance(n.value, str)}
check("no command that mounts or loads", not consts & {"mount", "umount", "modprobe", "insmod", "rmmod", "nvidia-modprobe"})

# 7. The survey for real, on a machine with no stamp, so nothing on standard output.
ARGS = ["--stage", "pairing", "--state-at", "/mnt/weaver/state", "--models-at", "/mnt/weaver/models",
        "--management", "192.0.2.10/24", "--data", "198.51.100.10/24", "--planes", "separate"]


def for_real(args):
    return subprocess.run([sys.executable, "-I", SURVEY] + args, capture_output=True, timeout=60)


p = for_real(ARGS)
check("for real, with no stamp: exit 1, nothing on standard output, the stamp named (%r)" % p.stderr[:120],
      p.returncode == 1 and p.stdout == b"" and b"the stamp" in p.stderr)
p = for_real(ARGS[4:] + ARGS[:4])
check("for real, the options in another order: read alike (%r)" % p.stderr[:80], p.returncode == 1 and b"the stamp" in p.stderr)
usage_cases = [
    ("no option", []),
    ("version 1's call, --stage pairing alone", ["--stage", "pairing"]),
    ("a stage of drives", ["--stage", "drives"] + ARGS[2:]),
    ("an unknown stage", ["--stage", "later"] + ARGS[2:]),
    ("planes other than separate or shared", ARGS[:-1] + ["both"]),
    ("an option given twice", ARGS[:-2] + ["--state-at", "/srv/x"]),
    ("an unknown option", ARGS[:-2] + ["--plane", "separate"]),
    ("an option with no value", ARGS[:-1]),
]
p = subprocess.run([sys.executable, "-I", SURVEY] + [a.encode() for a in ARGS[:2]] + [b"--state-at", b"/mnt/\xff"]
                   + [a.encode() for a in ARGS[4:]], capture_output=True, timeout=60)
check("for real, a place holding a byte that is not UTF-8: exit 1, the place named, no traceback (%r)" % p.stderr[:160],
      p.returncode == 1 and p.stdout == b"" and b"not a place in the one spelling" in p.stderr and b"Traceback" not in p.stderr)
for what, args in usage_cases:
    p = for_real(args)
    check("%s: exit 2, nothing on standard output, the usage named" % what,
          p.returncode == 2 and p.stdout == b"" and p.stderr.startswith(b"usage: weaver-survey --stage pairing --state-at <place>"))

print("== %d passed, %d failed (in %s)" % (passed, failed, T))
if failed:
    sys.exit(1)
if os.path.dirname(T) == tempfile.gettempdir() and re.fullmatch(r"t-weaver-survey\.[A-Za-z0-9_]{8}", os.path.basename(T)):
    shutil.rmtree(T)
