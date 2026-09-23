#!/usr/bin/env python3
"""Deploy plan boundaries, using subprocess fixtures (no real sudo or build).

Run: python3 deploy/test_plans.py
The command doubles record every privileged/build invocation. They never
forward sudo, systemctl, package-manager or Cargo calls to the host.
"""

import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


DEPLOY = Path(__file__).resolve().parent
DOUBLE = r'''#!/usr/bin/env python3
import json, os, pathlib, sys
name = pathlib.Path(sys.argv[0]).name
args = sys.argv[1:]
with open(os.environ["CALLS"], "a") as log:
    log.write(json.dumps([name, *args]) + "\n")
if name == "getent":
    sys.exit(0 if os.environ.get("COLLISION") == args[-1] else 2)
if name == "git":
    if args[0] == "rev-parse": print("abcdef0")
    elif args[0] == "branch": print("fixture-branch")
elif name == "hostname": print("fixture-box")
elif name == "nvidia-smi": print("fixture-driver")
elif name == "pacman": print("cccl 3.3.4-1")
elif name == "cargo":
    target = pathlib.Path(os.environ["CARGO_TARGET_DIR"])
    if args[0] == "metadata": print(json.dumps({"target_directory": str(target)}))
    elif args[0] == "test":
        print("test result: ok. 1 passed; 0 failed; 0 ignored")
    elif args[0] == "build":
        if os.environ.get("BUILD_FAIL"):
            print("fixture build refusal", file=sys.stderr)
            sys.exit(42)
        target.joinpath("release").mkdir(parents=True)
        for member in ("pyworker", "worker", "weaver-admin", "weaver-gate", "weaver-spu", "weaver-state"):
            target.joinpath("release", member).write_text("fixture artifact " + member)
    else: sys.exit(99)
elif name == "sudo":
    if os.environ.get("ALLOW_APPLY_CHECKS"):
        if args == ["-v"]: sys.exit(1 if os.environ.get("SUDO_FAIL") else 0)
        elif args[:2] == ["-n", "cat"]:
            if args[2].endswith("allow-list") and os.environ.get("READ_FAIL") == "allow-list": sys.exit(2)
            print(pathlib.Path(args[2]).read_text(), end="")
        elif args[:2] == ["-n", "grep"]:
            sys.exit(2 if os.environ.get("READ_FAIL") == "allow-list" else 1)
        elif args[:2] == ["-n", "systemctl"]:
            sys.exit(2 if os.environ.get("READ_FAIL") == args[2] else 0)
        elif "psql" in args:
            query = args[-1]
            if os.environ.get("READ_FAIL") and os.environ["READ_FAIL"] in query: sys.exit(2)
            if os.environ.get("EMPTY_PATH") and os.environ["EMPTY_PATH"] in query: sys.exit(0)
            if "pg_roles" in query and os.environ.get("ROLE_COLLISION"): print("1")
            elif "show hba_file" in query: print("/fixture/pg_hba.conf")
            elif "show ident_file" in query: print("/fixture/pg_ident.conf")
        else: sys.exit(99)
    else: sys.exit(99)
elif name == "mktemp":
    if os.environ.get("ALLOW_APPLY_CHECKS"):
        probe = pathlib.Path(os.environ["PROBE"])
        probe.mkdir()
        print(probe)
    else: sys.exit(99)
elif name == "setfacl": sys.exit(1)
else: sys.exit(99)
'''


class PlanTests(unittest.TestCase):
    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory(prefix="weaver-deploy-test-")
        self.addCleanup(self.scratch.cleanup)
        self.root = Path(self.scratch.name)
        self.repo = self.root / "repo"
        shutil.copytree(DEPLOY, self.repo / "deploy")
        self.config = self.root / "config"
        self.config.mkdir()
        self.agents = self.root / "agents"
        self.agents.mkdir()
        (self.config / "agent-config-directory").write_text(str(self.agents))
        (self.config / "worker-binary").write_text(str(self.root / "installed" / "pyworker"))
        (self.config / "allow-list").write_text("existing\n")
        (self.agents / "existing.yaml").write_text("state-store:\n  engine: none\n")
        self.artifact = self.root / "model.gguf"
        self.artifact.touch()
        self.bin = self.root / "bin"
        self.bin.mkdir()
        for name in ("sudo", "systemctl", "psql", "mktemp", "setfacl", "getent", "git", "cargo", "hostname", "nvidia-smi", "pacman"):
            command = self.bin / name
            command.write_text(DOUBLE)
            command.chmod(0o755)
        self.log = self.root / "calls"
        self.env = {**os.environ, "PATH": str(self.bin) + os.pathsep + os.environ["PATH"],
                    "WEAVER_ADMIN_CONFIG": str(self.config), "CALLS": str(self.log),
                    "CARGO_TARGET_DIR": str(self.root / 'target with "quotes"'),
                    "USER": "fixture-no-home", "PROBE": str(self.root / "probe")}
        for name in ("BASH_ENV", "SUDO_USER", "COLLISION", "ALLOW_APPLY_CHECKS", "ROLE_COLLISION", "BUILD_FAIL", "SUDO_FAIL", "READ_FAIL", "EMPTY_PATH"):
            self.env.pop(name, None)

    def run_script(self, name, *args):
        return subprocess.run(["bash", str(self.repo / "deploy" / name), *args],
                              env=self.env, text=True, capture_output=True, timeout=20)

    def calls(self):
        return [json.loads(line) for line in self.log.read_text().splitlines()] if self.log.exists() else []

    def create(self, *args):
        return self.run_script("create-agent.sh", "m1", "--artifact", str(self.artifact), *args)

    def assert_unprivileged(self):
        forbidden = {"sudo", "systemctl", "psql", "mktemp", "setfacl"}
        self.assertFalse([c for c in self.calls() if c[0] in forbidden], self.calls())

    def test_agent_plan_defers_privileged_checks_without_writes(self):
        before = {p: p.read_bytes() for p in self.root.rglob("*") if p.is_file()}
        result = self.create()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(str(self.agents / "m1.yaml"), result.stdout)
        self.assertIn("PENDING --apply", result.stdout)
        self.assertNotIn("nothing of this agent exists", result.stdout)
        self.assert_unprivileged()
        after = {p: p.read_bytes() for p in self.root.rglob("*") if p.is_file() and p != self.log}
        self.assertEqual(before, after)

    def test_agent_plan_refuses_missing_or_empty_configuration(self):
        field = self.config / "agent-config-directory"
        for value in (None, ""):
            with self.subTest(value=value):
                if value is None: field.unlink()
                else: field.write_text(value)
                result = self.create()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("agent-config-directory", result.stderr)
        self.assert_unprivileged()

    def test_agent_plan_refuses_missing_allow_list(self):
        (self.config / "allow-list").unlink()
        result = self.create()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("cannot read", result.stderr)
        self.assert_unprivileged()

    def test_agent_plan_refuses_visible_collisions(self):
        for collision in ("account", "declaration", "allow-list"):
            with self.subTest(collision=collision):
                self.env.pop("COLLISION", None)
                (self.agents / "m1.yaml").unlink(missing_ok=True)
                (self.config / "allow-list").write_text("existing\n")
                if collision == "account": self.env["COLLISION"] = "weaver-m1-state"
                elif collision == "declaration": (self.agents / "m1.yaml").touch()
                else: (self.config / "allow-list").write_text("m1\n")
                result = self.create()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("already exists" if collision != "allow-list" else "already in", result.stderr)
        self.assert_unprivileged()

    def test_apply_still_refuses_catalogue_collision_before_creation(self):
        self.env.update(ALLOW_APPLY_CHECKS="1", ROLE_COLLISION="1")
        result = self.create("--apply")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("role weaver_m1 already exists", result.stderr)
        self.assertFalse(any("useradd" in c for c in self.calls()))
        self.assertFalse(any(c[0] == "mktemp" for c in self.calls()))

    def test_apply_still_probes_acl_before_creation(self):
        self.env["ALLOW_APPLY_CHECKS"] = "1"
        result = self.create("--apply")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("refuses access entries", result.stderr)
        self.assertTrue(any(c[0] == "setfacl" for c in self.calls()))
        self.assertFalse(any("useradd" in c for c in self.calls()))
        self.assertFalse(Path(self.env["PROBE"]).exists())

    def test_apply_requires_sudo_before_any_other_privileged_call(self):
        self.env.update(ALLOW_APPLY_CHECKS="1", SUDO_FAIL="1")
        result = self.create("--apply")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("--apply needs sudo", result.stderr)
        self.assertEqual([c for c in self.calls() if c[0] == "sudo"], [["sudo", "-v"]])

    def test_apply_read_failures_refuse_before_creating_accounts(self):
        for fault, cause in (("allow-list", "allow-list"), ("pg_roles", "role catalog"),
                             ("pg_database", "database catalog"), ("hba_file", "hba_file"),
                             ("ident_file", "ident_file"), ("start", "start PostgreSQL"),
                             ("is-active", "confirm PostgreSQL")):
            with self.subTest(fault=fault):
                self.log.unlink(missing_ok=True)
                self.env.update(ALLOW_APPLY_CHECKS="1", READ_FAIL=fault)
                result = self.create("--apply")
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(cause, result.stderr)
                self.assertFalse(any("useradd" in c for c in self.calls()))
                self.assertFalse(any(c[0] == "mktemp" for c in self.calls()))

    def test_apply_empty_authentication_paths_refuse_before_creation(self):
        for path in ("hba_file", "ident_file"):
            with self.subTest(path=path):
                self.log.unlink(missing_ok=True)
                self.env.update(ALLOW_APPLY_CHECKS="1", EMPTY_PATH=path)
                result = self.create("--apply")
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("empty " + path, result.stderr)
                self.assertFalse(any("useradd" in c for c in self.calls()))
                self.assertFalse(any(c[0] == "mktemp" for c in self.calls()))

    def test_stack_plan_excludes_web_and_compares_all_members(self):
        result = self.run_script("update-stack.sh")
        self.assertEqual(result.returncode, 0, result.stderr)
        build = next(c for c in self.calls() if c[:2] == ["cargo", "build"])
        self.assertIn("--locked", build)
        self.assertIn("--workspace", build)
        self.assertIn("--exclude", build)
        self.assertEqual(build[build.index("--exclude") + 1], "weaver-web")
        self.assertIn("weaver-spu/cuda", build[-1])
        self.assertEqual(result.stdout.count("NEW"), 6)
        self.assertIn("plan only. rerun with --install", result.stdout)
        self.assert_unprivileged()
        cargo_actions = [c[1] for c in self.calls() if c[0] == "cargo"]
        self.assertEqual(cargo_actions, ["metadata", "test", "build"])

    def test_stack_build_failure_cannot_claim_a_plan(self):
        self.env["BUILD_FAIL"] = "1"
        result = self.run_script("update-stack.sh")
        self.assertEqual(result.returncode, 42, result.stderr)
        self.assertIn("fixture build refusal", result.stderr)
        self.assertNotIn("== plan", result.stdout)
        self.assertNotIn("box is current", result.stdout)
        self.assert_unprivileged()


if __name__ == "__main__":
    unittest.main()
