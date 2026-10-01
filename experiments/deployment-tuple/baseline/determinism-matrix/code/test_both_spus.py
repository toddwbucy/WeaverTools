"""The provenance readers a run with both SPUs relies on, each held by a test that fails
when its rule is perturbed: the weights of a safetensors directory, the serving device of
an engine that prints no llama.cpp lines, the engine libraries of a Rust SPU that links
CUDA and of a Python SPU, the interpreter a Python SPU runs on, and the SPU a load served.
Run with `python3 test_both_spus.py` or under pytest.
"""
import contextlib
import hashlib
import io
import json
import os
import shutil
import subprocess
import sys
import tempfile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import confirm_cells as base  # noqa: E402

CFG = {"agent": "alpha"}


def model_directory(root, files=None):
    """A safetensors export as the operator's plain copy holds it."""
    os.makedirs(root, exist_ok=True)
    for name, data in (files or {"model.safetensors": b"weights", "config.json": b"{}",
                                 "tokenizer.json": b"{}"}).items():
        os.makedirs(os.path.dirname(os.path.join(root, name)) or root, exist_ok=True)
        with open(os.path.join(root, name), "wb") as fh:
            fh.write(data)
    return root


def test_a_directory_artifact_is_read_whole():
    """python-spu-Spec section 5: a directory artifact is a digest over its sorted
    relative paths and each file's sha256. The same files in another order of
    creation read the same; a byte changed, a file renamed or a file added reads
    otherwise. Perturbation: hash the paths' sha256 alone without the names, and the
    rename reads the same."""
    with tempfile.TemporaryDirectory() as tmp:
        first = model_directory(os.path.join(tmp, "a"))
        second = model_directory(os.path.join(tmp, "b"), {"tokenizer.json": b"{}", "config.json": b"{}",
                                                          "model.safetensors": b"weights"})
        one = base.weights(first)(CFG)["artifact"]
        assert base.is_reading({"artifact": one}), one
        assert one["directory"] == 3, one
        assert one["sha256"] == base.weights(second)(CFG)["artifact"]["sha256"]
        with open(os.path.join(second, "model.safetensors"), "ab") as fh:
            fh.write(b"!")
        assert base.weights(second)(CFG)["artifact"]["sha256"] != one["sha256"]
        third = model_directory(os.path.join(tmp, "c"))
        os.rename(os.path.join(third, "config.json"), os.path.join(third, "config2.json"))
        assert base.weights(third)(CFG)["artifact"]["sha256"] != one["sha256"]
        fourth = model_directory(os.path.join(tmp, "d"))
        model_directory(fourth, {"sub/extra.json": b"[]"})
        assert base.weights(fourth)(CFG)["artifact"]["directory"] == 4


def test_a_link_in_or_at_a_directory_artifact_refuses():
    """A symbolic link anywhere in the artifact, or at its own path, is unreadable and
    never followed, since the bytes a link names are not the artifact's. Perturbation:
    follow links in the walk, and the linked file reads as part of the digest."""
    with tempfile.TemporaryDirectory() as tmp:
        outside = os.path.join(tmp, "outside.json")
        with open(outside, "wb") as fh:
            fh.write(b"{}")
        linked = model_directory(os.path.join(tmp, "linked"))
        os.symlink(outside, os.path.join(linked, "extra.json"))
        reading = base.weights(linked)(CFG)
        assert not base.is_reading(reading) and "symbolic link" in reading["artifact"]["unreadable"], reading
        linked_dir = model_directory(os.path.join(tmp, "linked-dir"))
        os.symlink(tmp, os.path.join(linked_dir, "elsewhere"))
        reading = base.weights(linked_dir)(CFG)
        assert not base.is_reading(reading), reading
        plain = model_directory(os.path.join(tmp, "plain"))
        os.symlink(plain, os.path.join(tmp, "alias"))
        reading = base.weights(os.path.join(tmp, "alias"))(CFG)
        assert not base.is_reading(reading) and "symbolic link" in reading["artifact"]["unreadable"], reading


def test_a_directory_the_walk_cannot_read_refuses():
    """A subdirectory the reader may not list is unreadable, never skipped as absent.
    Perturbation: walk without the raising `onerror`, and the shut directory reads as
    though it held nothing."""
    if os.geteuid() == 0:
        print("skip test_a_directory_the_walk_cannot_read_refuses: no mode denies root")
        return
    with tempfile.TemporaryDirectory() as tmp:
        root = model_directory(os.path.join(tmp, "m"), {"model.safetensors": b"w", "shut/x.json": b"{}"})
        os.chmod(os.path.join(root, "shut"), 0)
        try:
            reading = base.weights(root)(CFG)
            assert not base.is_reading(reading), reading
        finally:
            os.chmod(os.path.join(root, "shut"), 0o755)



# The digest script is python-spu's own, in the agent's repository beside this one in
# the suite workshop's layout; WEAVER_AGENTS_DIR names it where the checkouts sit
# elsewhere. The real script, so the reader is held to the line it actually prints.
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "..", ".."))
AGENTS = os.environ.get("WEAVER_AGENTS_DIR", os.path.join(REPO, "..", "WeaverAgents"))
TREE_DIGEST = os.path.join(AGENTS, "python-spu", "scripts", "tree_digest.py")


def python_spu(tmp, installed=0, interpreter_exit=0):
    """A Python SPU as python-spu installs one: a prefix whose `bin` holds the
    interpreter (here a shell script running this Python), a zipapp whose first line
    names it, and a repository carrying the lock, the real `tree_digest.py` and an
    `installed_set.py` answering `installed`. Answers the config and the SPU."""
    prefix = os.path.join(tmp, "prefix")
    os.makedirs(os.path.join(prefix, "bin"))
    os.makedirs(os.path.join(prefix, "lib", "torch"))
    interpreter = os.path.join(prefix, "bin", "python3.14")
    with open(interpreter, "w") as fh:
        fh.write(f"#!/bin/sh\n[ {interpreter_exit} -eq 0 ] || exit {interpreter_exit}\n"
                 f"exec {sys.executable} \"$@\"\n")
    os.chmod(interpreter, 0o755)
    with open(os.path.join(prefix, "lib", "torch", "libcublas.so.12"), "wb") as fh:
        fh.write(b"kernels")
    spu = os.path.join(prefix, "python-spu.pyz")
    with open(spu, "wb") as fh:
        fh.write(b"#!" + interpreter.encode() + b"\nPK\x03\x04zip")
    repo = os.path.join(tmp, "repo")
    scripts = os.path.join(repo, "python-spu", "scripts")
    os.makedirs(scripts)
    shutil.copy(TREE_DIGEST, scripts)
    with open(os.path.join(scripts, "installed_set.py"), "w") as fh:
        fh.write(f"import sys\nprint('torch: 2.9.1 installed, lock pins 2.9.0') if {installed} else None\n"
                 f"sys.exit({installed})\n")
    with open(os.path.join(repo, "python-spu", "requirements.lock"), "w") as fh:
        fh.write("torch==2.9.0 \\\n    --hash=sha256:00\n")
    return {"agent": "alpha", "repo": repo}, (spu, "admin config spu-binary")


def lock_sha(cfg):
    return base._sha256(os.path.join(cfg["repo"], "python-spu", "requirements.lock"))


def test_a_python_spu_reads_its_zipapp_lock_and_prefix():
    """python-spu-Spec sections 2, 5 and 8: a Python SPU's engine libraries are the
    zipapp, the lock its prefix is held to and the prefix's digest, and a library
    changed in the prefix reads otherwise. Perturbation: drop the Python branch from
    `engine_libraries`, and `ldd` on a script names no library, unreadable."""
    with tempfile.TemporaryDirectory() as tmp:
        cfg, spu = python_spu(tmp)
        one = base.engine_libraries(cfg, spu)
        assert base.is_reading(one), one
        assert set(one) == {"zipapp", "requirements.lock", "prefix"}, one
        assert one["zipapp"]["sha256"] == base._sha256(spu[0])
        assert one["requirements.lock"]["sha256"] == lock_sha(cfg)
        assert one["prefix"]["path"] == os.path.join(tmp, "prefix")
        with open(os.path.join(tmp, "prefix", "lib", "torch", "libcublas.so.12"), "ab") as fh:
            fh.write(b"!")
        two = base.engine_libraries(cfg, spu)
        assert base.close_hashes(two)["prefix"] != base.close_hashes(one)["prefix"], (one, two)


def test_a_lock_the_prefix_does_not_hold_is_unreadable():
    """The lock's hash is recorded only where the installed set is the lock's, since a
    lock states what should be installed and not what is. Perturbation: drop the exit
    check on `installed_set.py`, and a prefix holding another torch records the lock."""
    with tempfile.TemporaryDirectory() as tmp:
        cfg, spu = python_spu(tmp, installed=1)
        reading = base.engine_libraries(cfg, spu)
        assert not base.is_reading(reading), reading
        assert "not held to the lock" in reading["requirements.lock"]["unreadable"], reading
        assert "2.9.1" in reading["requirements.lock"]["unreadable"], reading


def test_a_prefix_digest_is_of_the_prefix_and_never_of_nothing():
    """The digest is `tree_digest.py`'s line for the prefix asked, and the script's
    refusal of an empty or unreadable tree is unreadable here. Perturbations: drop the
    check that the line names the prefix, and a script answering for another tree is
    recorded; drop the exit check, and an empty prefix records an empty digest."""
    with tempfile.TemporaryDirectory() as tmp:
        cfg, spu = python_spu(tmp)
        script = os.path.join(cfg["repo"], "python-spu", "scripts", "tree_digest.py")
        with open(script, "w") as fh:
            fh.write(f"print('{'a' * 64}  /elsewhere')\n")
        reading = base.engine_libraries(cfg, spu)
        assert not base.is_reading(reading) and "cannot hold" in reading["prefix"]["unreadable"], reading
        shutil.copy(TREE_DIGEST, script)
        files = base._python_spu_files(cfg)
        empty = os.path.join(tmp, "empty")
        os.mkdir(empty)
        reading = base._prefix_digest(files, empty)
        assert reading["sha256"] is None and "holds no entries" in reading["unreadable"], reading


def test_a_zipapp_first_line_is_held_to_one_bare_absolute_path():
    """The prefix the readers digest is derived from the interpreter the first line
    names, so a line of any other form than the build writes is unreadable: arguments,
    a relative path, an interpreter outside a `bin`, a path with `..`, no line end.
    Perturbation: take the line's first word, and `#!/p/bin/python3 -I` reads as the
    interpreter `/p/bin/python3`."""
    with tempfile.TemporaryDirectory() as tmp:
        cfg, (spu, how) = python_spu(tmp)
        for head in (b"#!/p/bin/python3 -I\n", b"#!python3\n", b"#!/p/python3\n",
                     b"#!/p/x/../bin/python3\n", b"#!/p/bin/python3", b"#!\n"):
            with open(spu, "wb") as fh:
                fh.write(head)
            reading = base.engine_libraries(cfg, (spu, how))
            assert set(reading) == {"unreadable"}, (head, reading)
            runtime = base._spu_runtime(cfg, spu)
            assert set(runtime) == {"unreadable"}, (head, runtime)


def test_a_rust_spu_records_the_cuda_libraries_it_links():
    """A Rust SPU built with `cuda` links the CUDA runtime, cuBLAS, cuBLASLt, cuRAND and
    the driver, `DT_NEEDED` all, and its matmuls are cuBLAS's, so each is read beside
    llama.cpp's. A library that is not the engine's, libc, is not. Perturbation: restore
    the llama.cpp-only pattern, and the CUDA libraries are absent from the reading."""
    with tempfile.TemporaryDirectory() as tmp:
        names = ("libggml-cuda.so.0", "libllama.so.0", "libcudart.so.13", "libcublas.so.13",
                 "libcublasLt.so.13", "libcurand.so.10", "libcuda.so.1", "libc.so.6")
        ldd = ""
        for name in names:
            with open(os.path.join(tmp, name), "wb") as fh:
                fh.write(name.encode())
            ldd += f"\t{name} => {os.path.join(tmp, name)} (0x00007f0000000000)\n"
        saved_sh, saved_stat = base.sh, base._stat_spu
        try:
            base._stat_spu = lambda p: None
            base.sh = lambda args, **kw: subprocess.CompletedProcess(args, 0, ldd, "")
            reading = base.engine_libraries({}, ("/spu", "admin config spu-binary"))
        finally:
            base.sh, base._stat_spu = saved_sh, saved_stat
        assert base.is_reading(reading), reading
        assert set(reading) == set(names) - {"libc.so.6"}, reading


def test_the_toolchain_names_what_runs_the_spu():
    """python-spu-Spec section 5: the toolchain reads per implementation, `rustc` for
    the Rust SPU and the interpreter's version with the lock's hash for a Python one,
    and an interpreter that does not answer is unreadable rather than the `rustc`
    reading standing alone. Perturbation: drop the `spu` entry from `toolchain`, and
    neither the interpreter nor the failure is in the reading."""
    rust = subprocess.CompletedProcess([], 0, "rustc stub\n", "")
    real = base.sh

    def sh(args, **kw):
        return rust if args[0] in ("rustc", "rustup") else real(args, **kw)

    with tempfile.TemporaryDirectory() as tmp:
        cfg, spu = python_spu(tmp)
        base.sh = sh
        try:
            tools = base.toolchain(cfg, spu)
            assert base.is_reading(tools), tools
            assert tools["spu"]["kind"] == "python", tools
            assert tools["spu"]["version"] == sys.version, tools
            assert tools["spu"]["lock_sha256"] == lock_sha(cfg), tools
            saved = base._stat_spu
            base._stat_spu = lambda p: None
            try:
                assert base.toolchain(cfg, spu)["spu"] == {"kind": "binary"}
            finally:
                base._stat_spu = saved
        finally:
            base.sh = real
    with tempfile.TemporaryDirectory() as tmp:
        cfg, spu = python_spu(tmp, interpreter_exit=3)
        base.sh = sh
        try:
            tools = base.toolchain(cfg, spu)
        finally:
            base.sh = real
        assert not base.is_reading(tools), tools


def test_the_matrix_reads_the_toolchain_for_the_spu_it_resolved_at_both_ends():
    """The toolchain is read for the SPU the open and the close each resolved once for
    every collector. Perturbation: restore the close's `base.toolchain` without the
    SPU, and the close reads it for no SPU."""
    from test_recorded_seed import Reloading, run_main
    seen = []

    def toolchain(cfg, spu=None):
        seen.append(spu)
        return {"rustc": "stub"}

    with contextlib.redirect_stdout(io.StringIO()):
        run_main(Reloading(), stack={"toolchain": toolchain})
    assert len(seen) == 2 and None not in seen, seen



INVOCATION = "a" * 32
GPUS = ("GPU-aaaa, NVIDIA RTX A6000, 00000000:01:00.0\n"
        "GPU-bbbb, NVIDIA RTX A6000, 00000000:41:00.0\n"
        "GPU-cccc, NVIDIA RTX 2000 Ada Generation, 00000000:86:00.0\n")


def driver(tmp, apps, invocations=None, gpus=GPUS, journal=()):
    """The box as `unit_devices` reads it: a cgroup hierarchy whose unit holds pid 100
    and, a group beneath it, pid 200; systemctl naming the unit's invocation (each call
    the next of `invocations`) and group; nvidia-smi listing `gpus` and the compute
    processes `apps`; the journal holding `journal`'s engine lines."""
    group = os.path.join(tmp, "system.slice", "worker.service")
    os.makedirs(os.path.join(group, "spu"))
    with open(os.path.join(group, "cgroup.procs"), "w") as fh:
        fh.write("100\n")
    with open(os.path.join(group, "spu", "cgroup.procs"), "w") as fh:
        fh.write("200\n")
    reads = iter(invocations or [INVOCATION] * 8)

    def sh(args, **kw):
        if args[:2] == ["systemctl", "show"] and "InvocationID" in args:
            return subprocess.CompletedProcess(args, 0, next(reads) + "\n", "")
        if args[:2] == ["systemctl", "show"] and "ControlGroup" in args:
            return subprocess.CompletedProcess(args, 0, "/system.slice/worker.service\n", "")
        if args[0] == "nvidia-smi" and args[1].startswith("--query-gpu"):
            return subprocess.CompletedProcess(args, 0, gpus, "")
        if args[0] == "nvidia-smi":
            return subprocess.CompletedProcess(args, 0, apps, "")
        if args[0] == "journalctl" and "-g" in args:
            # journalctl prints nothing where its pattern matches nothing.
            return subprocess.CompletedProcess(args, 0 if journal else 1,
                                               "".join(line + "\n" for line in journal), "")
        if args[0] == "journalctl":
            return subprocess.CompletedProcess(args, 0, "a line\n", "")
        raise AssertionError(f"unexpected command {args}")
    return sh


@contextlib.contextmanager
def box(tmp, apps, **kw):
    saved = base.sh, base.CGROUP_ROOT
    base.sh, base.CGROUP_ROOT = driver(tmp, apps, **kw), tmp
    try:
        yield
    finally:
        base.sh, base.CGROUP_ROOT = saved


B = {"uuid": "GPU-bbbb", "name": "NVIDIA RTX A6000", "pci_bus_id": "0000:41:00.0"}


def test_the_serving_device_is_the_card_the_units_processes_hold():
    """The binding is read from the driver: the cards `nvidia-smi` lists a process of the
    unit's control group holding, the groups beneath it included, and no other
    process's. Perturbations: read the unit's own group alone, and the SPU forked into
    a group beneath it is missed; drop the pid filter, and another user's card is read
    as this load's."""
    apps = "200, GPU-bbbb, 00000000:41:00.0\n999, GPU-aaaa, 00000000:01:00.0\n"
    with tempfile.TemporaryDirectory() as tmp, box(tmp, apps):
        assert base.unit_devices(CFG, INVOCATION) == {"devices": [B]}


def test_a_restart_across_the_device_read_is_unreadable():
    """The read is bound to one invocation, held before and after: a unit restarted while
    its processes were listed lists another load's processes. Perturbation: drop the
    after-read, and the restart reads as the load's binding."""
    apps = "200, GPU-bbbb, 00000000:41:00.0\n"
    with tempfile.TemporaryDirectory() as tmp, box(tmp, apps, invocations=[INVOCATION, "b" * 32]):
        reading = base.unit_devices(CFG, INVOCATION)
    assert set(reading) == {"unreadable"} and "after" in reading["unreadable"], reading


def test_no_card_held_or_a_driver_that_does_not_read_is_unreadable():
    """No process of the unit on a card, a row the reader cannot read, a process on a
    card `--query-gpu` does not list at that bus, and a driver that does not answer are
    each unreadable, never an empty binding. Perturbation: return the empty binding
    where no process of the unit holds a card, and the first case reads as a reading."""
    for apps, gpus, said in (("999, GPU-aaaa, 00000000:01:00.0\n", GPUS, "no process of the unit"),
                             ("200, GPU-bbbb\n", GPUS, "cannot read"),
                             ("200, GPU-bbbb, 00000000:01:00.0\n", GPUS, "does not list there"),
                             ("200, GPU-dddd, 00000000:41:00.0\n", GPUS, "does not list there"),
                             ("200, GPU-bbbb, 00000000:41:00.0\n",
                              GPUS + "GPU-bbbb, NVIDIA RTX A6000, 00000000:41:00.0\n", "twice")):
        with tempfile.TemporaryDirectory() as tmp, box(tmp, apps, gpus=gpus):
            reading = base.unit_devices(CFG, INVOCATION)
        assert set(reading) == {"unreadable"} and said in reading["unreadable"], (apps, reading)
    with tempfile.TemporaryDirectory() as tmp, box(tmp, ""):
        saved = base.sh
        base.sh = lambda args, **kw: (subprocess.CompletedProcess(args, 9, "", "NVML gone")
                                      if args[0] == "nvidia-smi" else saved(args, **kw))
        try:
            reading = base.unit_devices(CFG, INVOCATION)
        finally:
            base.sh = saved
    assert "exit 9: NVML gone" in reading["unreadable"], reading


def test_the_engines_own_lines_cross_check_the_driver():
    """Where the engine prints its device lines they must name the cards the unit's
    processes hold, the driver's eight-digit domain read as the engine's four; where it
    prints none, as the native backend and python-spu do, the driver's read stands and
    the journal's empty read is recorded beside it. Perturbation: drop the comparison,
    and a journal naming another card is held as this load's."""
    from test_round_five import BOUNDARY, COMPLETE, device_line
    apps = "200, GPU-bbbb, 00000000:41:00.0\n"
    agreeing = [BOUNDARY, device_line(1, "0000:41:00.0"), COMPLETE]
    with tempfile.TemporaryDirectory() as tmp, box(tmp, apps, journal=agreeing):
        seen, invocation = base.load_devices(CFG, 2, 0)
    assert seen["devices"] == [B] and seen["journal"]["complete"], seen
    other = [BOUNDARY, device_line(0, "0000:01:00.0"), COMPLETE]
    with tempfile.TemporaryDirectory() as tmp, box(tmp, apps, journal=other):
        seen, invocation = base.load_devices(CFG, 2, 0)
    assert set(seen) == {"unreadable"} and "0000:01:00.0" in seen["unreadable"], seen
    with tempfile.TemporaryDirectory() as tmp, box(tmp, apps):
        seen, invocation = base.load_devices(CFG, 2, 0)
    assert seen["devices"] == [B] and seen["journal"]["devices"] == [], seen


TESTS = [v for k, v in sorted(globals().items()) if k.startswith("test_")]

if __name__ == "__main__":
    for test in TESTS:
        test()
        print(f"ok {test.__name__}")
