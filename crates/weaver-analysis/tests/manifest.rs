//! conforms: analysis-no-internal-dependency
//! conforms: analysis-no-runtime-no-socket-crate
//!
//! The manifest assertions of `weaver-analysis-Spec` section 1: no
//! `weaver-*` dependency at all - this crate stands outside the agent and
//! linking any interior crate would make an outside consumer a compile-time
//! dependent of the interior - and no async runtime and no socket crate in
//! the resolved tree, the standard library's own client being the whole of
//! what one dialed Unix socket needs.

use std::process::Command;

fn resolved_tree() -> String {
    let out = Command::new(env!("CARGO"))
        .args([
            "tree",
            "-p",
            "weaver-analysis",
            "--edges",
            "normal",
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
