"""The matrix reads the agent's own root, per weaver-admin-Spec section 9.

Admin reads `<admin_config>/<agent>/` and nothing shared, since WeaverAgent #45,
which retired the allow-list and the SPU map. The matrix reads the same root.

Perturbations, each failing a watch here: read `spu-binary` from the base and
not the agent's root, and a flat base's file is taken; read the root of another
agent, and the run's agent reads the other's SPU; join an agent name that is not
a plain name, and a path outside the base is read; read every OSError in
`_read_admin` as absence, and an unreadable file reads as the default; decide
presence with `os.path.exists`, and the SPU behind a directory this process may
not search reads as missing; read a required file's absence as absence, and a
root admin refuses passes; drop the file-name judgment, and a root naming two
binaries under one file name passes; put the SPU map back, and a retired
`agent-spu` chooses the SPU.
"""
import os
import tempfile

import confirm_cells as g


# The seven files admin's loader requires in an agent's root
# (`load_service_config_from`), each naming what a box would. A test omits one
# by naming it with None.
REQUIRED = {"worker-binary": "/opt/weaver/bin/worker\n",
            "spu-binary": "/opt/weaver/bin/weaver-spu\n",
            "gate-binary": "/opt/weaver/bin/weaver-gate\n",
            "coordination-root": "/run\n",
            "territory": "/var/lib/weaver-agent/weaver-karl\n",
            "operator": "4242\n",
            "roles.toml": 'trace-reader = "reader"\n'}


def write_root(base, agent, files):
    """An agent's root beneath `base`, holding REQUIRED with `files` over it."""
    root = os.path.join(base, agent)
    os.makedirs(root, exist_ok=True)
    for name, text in {**REQUIRED, **files}.items():
        if text is not None:
            with open(os.path.join(root, name), "w") as f:
                f.write(text)
    return root


def resolve(files, agent="karl", base=None):
    base = base or tempfile.mkdtemp()
    write_root(base, agent, files)
    return g._resolve_spu({"admin_config": base, "agent": agent,
                           "admin_bin": "/opt/weaver/bin/weaver-admin"})


def test_the_agents_root_names_the_spu():
    assert resolve({}) == ("/opt/weaver/bin/weaver-spu", "admin config spu-binary")


def test_the_base_is_not_the_root():
    """A file at the base, the layout before #45, is not read: the run's agent
    has its own root beneath it."""
    base = tempfile.mkdtemp()
    with open(os.path.join(base, "spu-binary"), "w") as f:
        f.write("/opt/flat/weaver-spu\n")
    assert resolve({}, base=base) == ("/opt/weaver/bin/weaver-spu", "admin config spu-binary")
    path, why = g._resolve_spu({"admin_config": base, "agent": "ada",
                                "admin_bin": "/opt/weaver/bin/weaver-admin"})
    assert path is None and os.path.join(base, "ada") in why, (path, why)


def test_another_agents_root_is_not_read():
    base = tempfile.mkdtemp()
    write_root(base, "ada", {"spu-binary": "/opt/ada/weaver-spu\n"})
    assert resolve({}, base=base) == ("/opt/weaver/bin/weaver-spu", "admin config spu-binary")
    assert g._resolve_spu({"admin_config": base, "agent": "ada", "admin_bin": "/x/weaver-admin"}) == (
        "/opt/ada/weaver-spu", "admin config spu-binary")


def test_an_agent_that_is_not_a_plain_name_is_refused():
    base = tempfile.mkdtemp()
    write_root(base, "karl", {})
    for agent in ("../karl", "a/b", ".", "..", "", None):
        cfg = {"admin_config": os.path.join(base, "x"), "agent": agent, "admin_bin": "/x/weaver-admin"}
        path, why = g._resolve_spu(cfg)
        assert path is None and "not a plain name" in why, (agent, path, why)


def unreadable_forms(root, name):
    """Each way something can stand at a path and not read: a file this process
    may not read, a directory, a dangling link, and bytes that are not UTF-8."""
    path = os.path.join(root, name)
    forms = []
    if os.geteuid() != 0:
        forms.append(("mode 000", lambda: (open(path, "w").write("/opt/x\n"),
                                           os.chmod(path, 0o000))))
    forms.append(("a directory", lambda: os.mkdir(path)))
    forms.append(("a dangling link", lambda: os.symlink(os.path.join(root, "gone"), path)))
    forms.append(("not UTF-8", lambda: open(path, "wb").write(b"\xff\xfe karl\n")))
    return forms


def test_an_unreadable_value_is_unreadable_never_the_default():
    """At each required path, anything that stands and does not read is an
    unreadable resolution, never the default and never the guess beside
    admin_bin."""
    for name in REQUIRED:
        probe = tempfile.mkdtemp()
        for form, _ in unreadable_forms(probe, name):
            base = tempfile.mkdtemp()
            root = write_root(base, "karl", {name: None})
            dict(unreadable_forms(root, name))[form]()
            path, why = g._resolve_spu({"admin_config": base, "agent": "karl",
                                        "admin_bin": "/opt/weaver/bin/weaver-admin"})
            assert path is None, (name, form, path, why)
            assert name in why, (name, form, why)


def test_an_optional_file_that_stands_and_does_not_read_refuses():
    """Admin's `optional` reads a file absent only where nothing stands, so an
    optional file that stands and does not read refuses every verb, as a
    required one does. Perturbation: skip reading the optional files, and each
    form here passes."""
    for name in g._ADMIN_OPTIONAL:
        probe = tempfile.mkdtemp()
        for form, _ in unreadable_forms(probe, name):
            base = tempfile.mkdtemp()
            root = write_root(base, "karl", {})
            dict(unreadable_forms(root, name))[form]()
            path, why = g._resolve_spu({"admin_config": base, "agent": "karl",
                                        "admin_bin": "/opt/weaver/bin/weaver-admin"})
            assert path is None and name in why, (name, form, path, why)
        assert resolve({name: None})[0] == "/opt/weaver/bin/weaver-spu", name


def test_each_value_is_judged_as_admin_judges_it():
    """Admin's loader parses `operator` as a u32 and `load-bound-seconds` as a
    positive u64 that forms a deadline, and refuses a relative path in every
    key naming one. Perturbation: drop any one check, and its case passes."""
    cases = [({"operator": "ada\n"}, "operator is not a uid"),
             ({"operator": "-1\n"}, "operator is not a uid"),
             ({"operator": "4294967296\n"}, "operator is not a uid"),
             ({"load-bound-seconds": "0\n"}, "not a positive count of seconds"),
             ({"load-bound-seconds": "ten\n"}, "not a positive count of seconds"),
             ({"load-bound-seconds": f"{2**63}\n"}, "not a positive count of seconds"),
             ({"coordination-root": "run\n"}, "coordination-root names a relative path"),
             ({"territory": "var/lib/x\n"}, "territory names a relative path"),
             ({"library-path": "lib\n"}, "library-path names a relative path"),
             ({"shell-binary": "sh\n"}, "shell-binary names a relative path"),
             ({"gate-binary": "weaver-gate\n"}, "gate-binary names a relative path")]
    for change, said in cases:
        path, why = resolve(change)
        assert path is None and said in why, (change, path, why)
    for change in ({"operator": "+4242\n"}, {"load-bound-seconds": "120\n"},
                   {"library-path": "\n"}, {"headroom-bytes": "1\n"}):
        assert resolve(change)[0] == "/opt/weaver/bin/weaver-spu", change


def test_the_refusal_is_the_first_admin_meets():
    """Admin judges the file names right after the three binaries, before it
    reads `operator`, so a root with both a clash and no `operator` refuses on
    the clash. Perturbation: read every required file before the names, and
    the missing operator is named instead."""
    path, why = resolve({"spu-binary": "/opt/other/worker", "operator": None})
    assert path is None and "share the file name" in why, why


def test_a_required_file_absent_is_unreadable():
    """Admin's loader reads each of the seven as required, refusing every verb
    where one is absent, so each one's absence is an unreadable resolution, never
    the default and never the guess beside admin_bin."""
    assert set(g._ADMIN_REQUIRED) == set(REQUIRED), g._ADMIN_REQUIRED
    for name in g._ADMIN_REQUIRED:
        path, why = resolve({name: None})
        assert path is None, (name, path, why)
        assert name in why and "absent, and admin requires it" in why, (name, why)


def test_a_spu_binary_naming_no_path_is_unreadable_never_the_guess():
    """Admin launches the empty path where spu-binary names none, which is no SPU,
    so the guess beside admin_bin does not stand in for it."""
    for text in ("", "\n", "  \n"):
        path, why = resolve({"spu-binary": text})
        assert path is None and "names no path" in why, (text, path, why)


def test_a_file_admin_does_not_class_is_refused():
    """Every name `_read_admin` reads is classed as admin's loader classes it, so a
    new reader cannot read a file without saying which it is. The files #45
    retired are classed in neither."""
    for name in ("allow-list", "agent-spu", "spu-implementations", "run.marker"):
        try:
            g._read_admin(tempfile.mkdtemp(), name)
        except ValueError:
            continue
        raise AssertionError(f"an unclassed admin file was read: {name}")


def test_a_retired_map_chooses_nothing():
    """A root still carrying the retired SPU map, naming the run's agent, reads
    `spu-binary`: admin no longer reads the map, so the matrix does not."""
    files = {"spu-implementations": "python /opt/weaver/python-spu/python-spu.pyz\n",
             "agent-spu": "karl python\n", "allow-list": "ada\n"}
    assert resolve(files) == ("/opt/weaver/bin/weaver-spu", "admin config spu-binary")


def test_the_spu_behind_an_unsearchable_directory_does_not_read_as_missing():
    """`os.path.exists` answers False for a path under a directory this process
    may not search. The check reads the difference, against a real 0o000
    directory."""
    if os.geteuid() == 0:
        print("skip: root searches every directory")
        return
    root = tempfile.mkdtemp()
    walled = os.path.join(root, "walled")
    os.mkdir(walled)
    spu = os.path.join(walled, "weaver-spu")
    open(spu, "w").close()
    os.chmod(walled, 0o000)
    try:
        answer = g.engine_libraries({"admin_config": root}, spu=(spu, "test"))
    finally:
        os.chmod(walled, 0o700)
    assert "does not stat" in answer["unreadable"], answer
    gone = g.engine_libraries({"admin_config": root}, spu=(os.path.join(root, "gone"), "test"))
    assert gone["unreadable"].startswith("no SPU binary at"), gone


# Admin's section 9 check of the four binaries the stack keys by file name
# (`crates/weaver-admin/src/stack.rs`), case for case, with the message both say.
RULES = [
    ({"spu-binary": "/opt/other/worker"}, "the worker and the SPU share the file name"),
    ({"spu-binary": "/opt/other/weaver-state"}, "the state member and the SPU share the file name"),
    ({"spu-binary": "/opt/other/weaver-gate"}, "the gate and the SPU share the file name"),
    ({"spu-binary": "/opt/other/worker/"}, "the worker and the SPU share the file name"),
    ({"gate-binary": "/opt/other/worker"}, "the worker and the gate share the file name"),
    ({"gate-binary": "/opt/other/weaver-state"}, "the state member and the gate share the file name"),
    ({"worker-binary": "/opt/other/weaver-state"}, "the worker and the state member share the file name"),
]


def test_every_rule_of_admins_is_the_matrixs():
    for change, said in RULES:
        path, why = resolve(change)
        assert path is None and said in why, (change, path, why)


if __name__ == "__main__":
    for name, fn in sorted(globals().items()):
        if name.startswith("test_") and callable(fn):
            fn()
            print(f"ok  {name}")
    print("all watches held")
