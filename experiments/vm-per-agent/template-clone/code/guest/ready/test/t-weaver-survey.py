#!/usr/bin/env python3
"""t-weaver-survey.py: weaver-survey against made-up machines, and its report against a
reader written from weaver-survey/1 section 3. Standard library only.

It names nvidia-smi, modinfo, findmnt and systemd-detect-virt, though it runs none of
them: a host that keeps tests naming a card's driver confined runs it through its own
runner (HOST_TESTS, weaver-appliance-build-Spec section 1). test/host/t-survey.sh runs it.

Every machine here is made up and illustrative: its CPU, its cards and their addresses.

Each machine is a fixture tree in a fresh folder (the stamp, the MANIFEST, /proc and /sys
files) and a stand-in command runner that answers by the command's name and records what
it was asked; no command is executed except the survey itself, run once for real at the
end, where it must find no stamp and write nothing. The reader is checked against
WeaverWeb's own worked example (tests/fixtures at e0575e65, blob d754dc50) before it
judges any report. Prints each case; exits 1 if any fails. The folder, made under TMPDIR,
is removed when every case passes.
"""
import hashlib, importlib.machinery, importlib.util, json, os, re, shutil, subprocess, sys, tempfile, time

sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
SURVEY = os.path.join(os.path.dirname(HERE), "weaver-survey")
EXAMPLE = os.path.join(HERE, "fixtures", "weaver-survey-1-worked-example.json")
EXAMPLE_SHA = "334959d40dd7c01450782852da4b7ef7982a90f0a6af70cf9c66b30656673acc"
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


# ---- The reader: weaver-survey/1 section 3 (survey_reader.py, beside this test). ----
sys.path.insert(0, HERE)
from survey_reader import Refuse, reader, refused


# ---- The reader against WeaverWeb's worked example, before it judges anything. ----
ex = open(EXAMPLE, "rb").read()
check("the worked example is WeaverWeb's at e0575e65, by digest", hashlib.sha256(ex).hexdigest() == EXAMPLE_SHA)
check("the reader takes the worked example", not refused(ex))
exo = json.loads(ex)


def mutated(f):
    o = json.loads(ex); f(o); return json.dumps(o).encode()


check("the reader refuses a member given twice", refused(ex.replace(b'"stage": "drives",', b'"stage": "drives", "stage": "drives",')))
check("the reader refuses a fractional integer", refused(ex.replace(b'"logical_cpus": 16', b'"logical_cpus": 16.0')))
check("the reader refuses an exponent", refused(ex.replace(b'"logical_cpus": 16', b'"logical_cpus": 1.6e1')))
check("the reader refuses a missing member", refused(mutated(lambda o: o["kernel"].pop("release"))))
check("the reader refuses flags out of order", refused(mutated(lambda o: o["cpu"]["flags"].reverse())))
check("the reader refuses a second newline after the object", refused(ex.rstrip(b"\n") + b"\n\n"))
check("the reader refuses an unmounted drive with a label", refused(mutated(lambda o: o["drives"][0].update(mounted=False))))
check("the reader refuses ordinals out of index", refused(mutated(lambda o: o["cards"][1].update(ordinal=0))))
check("the reader keeps and ignores an unknown member", not refused(mutated(lambda o: o["cpu"].update(extra=[1, 2]))))

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
        }
        self.answers.update(over)

    def __call__(self, argv, timeout=60):
        self.calls.append(list(argv))
        key = argv[0] + (":" + argv[4] if argv[0] == "findmnt" else "")
        return self.answers[key]


def run_survey(root, stage="pairing", **over):
    r = Runner(**over)
    try:
        return ws.survey(stage, root=root, runner=r, now=NOW), r, None
    except ws.Fail as e:
        return None, r, str(e)


# 1. Two cards, loaded, at pairing: nothing mounted.
root = machine_root("two-cards")
body, r, why = run_survey(root)
check("two cards at pairing: a report (%s)" % why, body is not None)
o = json.loads(body) if body else {}
check("the reader takes it", body is not None and not refused(body))
check("format first, then stage and taken", list(o)[:3] == ["format", "stage", "taken"])
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
check("the drives at pairing: role, mount point and mounted false alone",
      o.get("drives") == [{"role": "state", "mount_point": "/mnt/weaver/state", "mounted": False},
                          {"role": "models", "mount_point": "/mnt/weaver/models", "mounted": False}])
check("findmnt asked the format's columns at each mount point",
      [c for c in r.calls if c[0] == "findmnt"] == [["findmnt", "-J", "-b", "--mountpoint", mp, "-o", "FSTYPE,LABEL,OPTIONS,SIZE,AVAIL"]
                                                    for mp in ("/mnt/weaver/state", "/mnt/weaver/models")])
check("no command beyond the five", {c[0] for c in r.calls} <= {"systemd-detect-virt", "modinfo", "ldconfig", "nvidia-smi", "findmnt"})

# 2. The same machine at drives: both mounted.
body, r, why = run_survey(root, "drives", **{"findmnt:/mnt/weaver/state": (0, FINDMNT_STATE), "findmnt:/mnt/weaver/models": (0, FINDMNT_MODELS)})
o = json.loads(body) if body else {}
check("at drives: a report the reader takes (%s)" % why, body is not None and not refused(body))
check("the state drive: label, ext4, sizes and the project quota",
      o.get("drives", [{}])[0] == {"role": "state", "mount_point": "/mnt/weaver/state", "mounted": True, "label": "weaver-state",
                                   "filesystem": "ext4", "bytes": 16729894912, "free_bytes": 15832121344, "project_quota": True})
check("the models drive: no label read as empty, sizes given as strings read as integers, no project quota",
      o.get("drives", [{}, {}])[1] == {"role": "models", "mount_point": "/mnt/weaver/models", "mounted": True, "label": "",
                                       "filesystem": "ext4", "bytes": 84230021120, "free_bytes": 79831982080, "project_quota": False})

# 3. No card, the module not loaded: the cardless rehearsal's machine.
root3 = machine_root("cardless", loaded=False, nvidia_fns=())
body, r, why = run_survey(root3)
o = json.loads(body) if body else {}
check("no card: a report the reader takes (%s)" % why, body is not None and not refused(body))
check("no card: cards empty, the driver read from the module file, not loaded",
      o.get("cards") == [] and o.get("driver") == {"version": "620.70.06", "loaded": False})
check("no card: modinfo asked the format's question; nvidia-smi never run",
      ["modinfo", "-F", "version", "nvidia"] in r.calls and not any(c[0] == "nvidia-smi" for c in r.calls))

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
    ("seventeen cards", machine_root("seventeen", nvidia_fns=["0000:%02x:00.0" % (0x10 + n) for n in range(17)]),
     {"nvidia-smi": (0, "".join("00000000:%02X:00.0, NVIDIA RTX A5000, 24564\n" % (0x10 + n) for n in range(17)))}, "over 16"),
]
for what, rt, over, cause in cases:
    stage = "drives" if "findmnt" in "".join(over) else "pairing"
    body, r, why = run_survey(rt, stage, **over)
    check("%s: no report, and the cause names it (%s)" % (what, why), body is None and why is not None and cause in why)
check("the stage must be pairing or drives", run_survey(root, "pair")[0] is None)

# 6. The script as written: it opens nothing for writing, no socket, no temporary file.
import ast
tree = ast.parse(open(SURVEY).read())
imported = {a.name for n in ast.walk(tree) if isinstance(n, (ast.Import, ast.ImportFrom)) for a in n.names} \
    | {n.module for n in ast.walk(tree) if isinstance(n, ast.ImportFrom)}
check("the survey imports %s alone: no socket, no temporary file" % sorted(imported),
      imported <= {"hashlib", "json", "os", "re", "subprocess", "sys", "time"})
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
p = subprocess.run([sys.executable, "-I", SURVEY, "--stage", "drives"], capture_output=True, timeout=60)
check("for real, with no stamp: exit 1, nothing on standard output, the stamp named (%r)" % p.stderr[:120],
      p.returncode == 1 and p.stdout == b"" and b"the stamp" in p.stderr)
p = subprocess.run([sys.executable, "-I", SURVEY], capture_output=True, timeout=60)
check("no stage: exit 2, nothing on standard output", p.returncode == 2 and p.stdout == b"")
p = subprocess.run([sys.executable, "-I", SURVEY, "--stage", "later"], capture_output=True, timeout=60)
check("an unknown stage: exit 2, nothing on standard output", p.returncode == 2 and p.stdout == b"")

print("== %d passed, %d failed (in %s)" % (passed, failed, T))
if failed:
    sys.exit(1)
if os.path.dirname(T) == tempfile.gettempdir() and re.fullmatch(r"t-weaver-survey\.[A-Za-z0-9_]{8}", os.path.basename(T)):
    shutil.rmtree(T)
