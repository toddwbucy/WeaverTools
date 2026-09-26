//! conforms: analysis-no-internal-dependency
//! conforms: analysis-no-runtime-no-socket-crate
//! conforms: analysis-no-crate-depends-on-it
//!
//! The manifest assertions of `weaver-analysis-Spec` section 1: no
//! `weaver-*` dependency at all - this crate stands outside the agent and
//! linking any interior crate would make an outside consumer a compile-time
//! dependent of the interior - and no async runtime and no socket crate in
//! the resolved tree, the standard library's own client being the whole of
//! what one dialed Unix socket needs. And the other direction: no crate in
//! the workspace depends on this one, the operator's condition of 2026-09-26
//! for it staying in the repository.

use std::process::Command;

fn resolved_tree() -> String {
    let out = Command::new(env!("CARGO"))
        // Every edge kind and every target: a dev or build dependency on
        // an interior crate, or one reached only on another platform,
        // crosses the same boundary the normal edges do.
        .args([
            "tree",
            "-p",
            "weaver-analysis",
            "--edges",
            "all",
            "--all-targets",
            "--prefix",
            "none",
        ])
        .output()
        .expect("cargo tree runs");
    assert!(out.status.success());
    String::from_utf8(out.stdout).expect("utf8")
}

#[test]
fn no_weaver_dependency_at_all() {
    let names: Vec<String> = resolved_tree()
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .map(str::to_string)
        .collect();
    assert!(
        !names
            .iter()
            .any(|n| n.starts_with("weaver-") && n != "weaver-analysis"),
        "the boundary is the manifest: {names:?}"
    );
}

#[test]
fn no_runtime_and_no_socket_crate() {
    let tree = resolved_tree();
    for forbidden in ["tokio", "async-std", "smol", "mio", "socket2", "nix"] {
        assert!(
            !tree
                .lines()
                .any(|l| l.split_whitespace().next() == Some(forbidden)),
            "{forbidden} stands in the resolved tree"
        );
    }
}

/// **No crate in the workspace depends on this one, of any kind.** Read from
/// cargo's declared dependencies of every workspace member, where a normal,
/// build or dev edge, renamed, target-qualified or optional, all name the
/// package, so every route to this crate is seen and not only a manifest line
/// spelled `weaver-analysis =`. The member set is checked against every
/// `crates/*/Cargo.toml` first, so a crate standing outside the workspace
/// refuses rather than going unread.
///
/// Perturbation: add `weaver-analysis = { path = "../weaver-analysis" }` under
/// another member's `[dev-dependencies]`, or the same under a rename, and this
/// fails naming that member. Watched under exactly those changes.
#[test]
fn no_workspace_crate_depends_on_this_one() {
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
    let metadata: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("cargo metadata is json");
    let packages = metadata["packages"].as_array().expect("a package list");

    let crates_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crates directory");
    let mut on_disk: Vec<std::path::PathBuf> = std::fs::read_dir(crates_dir)
        .expect("the crates directory reads")
        .map(|entry| entry.expect("an entry reads").path().join("Cargo.toml"))
        .filter(|manifest| manifest.is_file())
        .map(|manifest| manifest.canonicalize().expect("a manifest resolves"))
        .collect();
    on_disk.sort();
    let mut members: Vec<std::path::PathBuf> = packages
        .iter()
        .map(|p| {
            std::path::PathBuf::from(p["manifest_path"].as_str().expect("a manifest path"))
                .canonicalize()
                .expect("a member manifest resolves")
        })
        .collect();
    members.sort();
    assert_eq!(
        on_disk, members,
        "every crate under crates/ is a workspace member the read below sees"
    );
    assert!(
        packages.iter().any(|p| p["name"] == "weaver-analysis"),
        "the read includes this crate, so it is the workspace's metadata"
    );

    let dependents: Vec<String> = packages
        .iter()
        .filter(|p| p["name"] != "weaver-analysis")
        .filter(|p| {
            p["dependencies"]
                .as_array()
                .expect("a dependency list")
                .iter()
                .any(|d| d["name"] == "weaver-analysis")
        })
        .map(|p| p["name"].as_str().expect("a package name").to_string())
        .collect();
    assert!(
        dependents.is_empty(),
        "weaver-analysis stays a leaf, and these depend on it: {dependents:?}"
    );
}
