//! conforms: internal-no-dependencies
//! conforms: internal-one-library-target
//!
//! The manifest assertions of `weaver-internal-Spec` section 1: the resolved
//! dependency set is empty, the manifest form of the charter's pure bar, and
//! the crate declares exactly one library target and no other kind.

use std::io::Write;
use std::process::{Command, Stdio};

/// **The dependency set is empty.** Read from the lockfile's view of this
/// package rather than the manifest's text, so a dependency arriving by any
/// route is what the instrument sees.
#[test]
fn the_dependency_set_is_empty() {
    let out = Command::new(env!("CARGO"))
        .args([
            "tree",
            "-p",
            "weaver-internal",
            "--edges",
            "normal,build",
            "--prefix",
            "none",
            // This inner cargo may not write the lock as a side effect of
            // answering. It does not prove the lock was in step, the outer
            // `cargo test` having resolved before this binary was spawned.
            // `process/gates/lock.sh` is where that is bought, ahead of the
            // suite, per issue #551's third ask.
            "--locked",
            "--offline",
        ])
        .output()
        .expect("cargo tree runs");
    assert!(
        out.status.success(),
        "cargo tree failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let tree = String::from_utf8(out.stdout).expect("utf8");
    let dependencies: Vec<&str> = tree
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("weaver-internal"))
        .collect();
    assert!(
        dependencies.is_empty(),
        "a pure member names no dependency: {dependencies:?}"
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
assert len(packages) == 1, "metadata must name exactly one weaver-internal package"
targets = sorted((t["name"], t["kind"], t["crate_types"])
                 for t in packages[0]["targets"])
expected = [("manifest", ["test"], ["bin"]),
            ("weaver_internal", ["lib"], ["lib"])]
assert targets == expected, f"expected library plus manifest test only; got {targets!r}"
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
