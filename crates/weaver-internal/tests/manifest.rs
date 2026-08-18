//! conforms: internal-no-dependencies
//! conforms: internal-one-library-target
//!
//! The manifest assertions of `weaver-internal-Spec` section 1: the resolved
//! dependency set is empty, the manifest form of the charter's pure bar, and
//! the crate declares exactly one library target and no other kind.

use std::process::Command;

/// **The dependency set is empty.** Read from the lockfile's view of this
/// package rather than the manifest's text, so a dependency arriving by any
/// route is what the instrument sees.
#[test]
fn the_dependency_set_is_empty() {
    let out = Command::new("cargo")
        .args([
            "tree",
            "-p",
            "weaver-internal",
            "--edges",
            "normal,build",
            "--prefix",
            "none",
            "--locked",
            "--offline",
        ])
        .output()
        .expect("cargo tree runs");
    assert!(out.status.success(), "cargo tree answers");
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

/// **Exactly one library target and no target of any other kind.** The
/// manifest text and the source tree are both read: a `[[bin]]` section and
/// a `src/main.rs` are two routes to a process, and a member holds none.
#[test]
fn the_one_target_is_a_library() {
    let manifest = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("the manifest reads");
    for forbidden in ["[[bin]]", "[[bench]]", "[[example]]"] {
        assert!(
            !manifest.contains(forbidden),
            "the manifest declares a {forbidden} target"
        );
    }
    assert!(
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs")).exists(),
        "the library target exists"
    );
    assert!(
        !std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/main.rs")).exists(),
        "no implicit binary target exists"
    );
}
