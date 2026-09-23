//! conforms: internal-no-dependencies
//! conforms: internal-one-library-target
//!
//! The manifest assertions of `weaver-internal-Spec` section 1: the resolved
//! dependency set is empty, the manifest form of the charter's pure bar, and
//! the crate declares exactly one library target and no other kind.

use std::io::Write;
use std::process::{Command, Stdio};

/// **The dependency set is empty, of every kind.** Read from cargo's own
/// declared dependency list for this package, so a dependency arriving by any
/// route, normal, build or dev, target-qualified, behind a feature or under a
/// rename, is what the instrument sees, per the ruling of 2026-09-23 on #577.
/// Python's standard JSON parser keeps this crate's own edges empty.
/// Perturbations: add a dev-dependency, a target-qualified dependency, or an
/// optional dependency behind a feature; each adds a declaration.
#[test]
fn the_dependency_set_is_empty() {
    let out = Command::new(env!("CARGO"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "metadata",
            "--no-deps",
            "--format-version",
            "1",
            "--locked",
            "--offline",
        ])
        .output()
        .expect("cargo metadata runs");
    assert!(
        out.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let mut check = Command::new("python3")
        .args([
            "-c",
            r#"
import json
import sys

packages = [p for p in json.load(sys.stdin)["packages"]
            if p["name"] == "weaver-internal"]
assert len(packages) == 1, "metadata must name exactly one weaver-internal package"
declared = [(d["name"], d["kind"], d.get("target"), d.get("optional"))
            for d in packages[0]["dependencies"]]
assert declared == [], f"a pure member names no dependency of any kind; got {declared!r}"
"#,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("python3 (the census gate prerequisite) runs");
    check
        .stdin
        .take()
        .expect("parser stdin")
        .write_all(&out.stdout)
        .expect("metadata reaches the JSON parser");
    let checked = check
        .wait_with_output()
        .expect("dependency check completes");
    assert!(
        checked.status.success(),
        "cargo's declared dependency set violates the empty-set claim: {}",
        String::from_utf8_lossy(&checked.stderr)
    );
}

/// **One library and this instrument are the complete target set.** Cargo's
/// metadata sees every implicit and explicit target, including build scripts.
/// Python's standard JSON parser keeps this dependency-free crate's Cargo
/// edges empty; Python3 is already required by the repository's census gate.
/// Perturbations: add build.rs or src/bin/x.rs; either adds a forbidden target.
#[test]
fn the_one_target_is_a_library() {
    let out = Command::new(env!("CARGO"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "metadata",
            "--no-deps",
            "--format-version",
            "1",
            "--locked",
            "--offline",
        ])
        .output()
        .expect("cargo metadata runs");
    assert!(
        out.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let mut check = Command::new("python3")
        .args([
            "-c",
            r#"
import json
import sys

packages = [p for p in json.load(sys.stdin)["packages"]
            if p["name"] == "weaver-internal"]
if len(packages) != 1:
    sys.exit("metadata must name exactly one weaver-internal package")
targets = sorted((t["name"], t["kind"], t["crate_types"])
                 for t in packages[0]["targets"])
expected = [("manifest", ["test"], ["bin"]),
            ("weaver_internal", ["lib"], ["lib"])]
if targets != expected:
    sys.exit(f"expected library plus manifest test only; got {targets!r}")
"#,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("python3 (the census gate prerequisite) runs");
    check
        .stdin
        .take()
        .expect("parser stdin")
        .write_all(&out.stdout)
        .expect("metadata reaches the JSON parser");
    let checked = check.wait_with_output().expect("target check completes");
    assert!(
        checked.status.success(),
        "cargo's target set violates the one-library claim: {}",
        String::from_utf8_lossy(&checked.stderr)
    );
}
