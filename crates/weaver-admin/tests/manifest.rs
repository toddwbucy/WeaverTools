//! conforms: admin-one-floor-link-types-config
//! conforms: admin-no-direct-traits-line
//! conforms: admin-no-runtime-no-bus-no-logging
//! conforms: admin-no-library-surface
//!
//! The manifest assertions of `weaver-admin-Spec` sections 1 and 10. These
//! reach the crate from outside without needing its modules, which is what
//! lets them live in an integration test while every other test in this crate
//! sits inside it: the no-library rule means nothing else can.

use std::process::Command;

fn resolved_tree() -> String {
    let out = Command::new(env!("CARGO"))
        .args([
            "tree",
            "-p",
            "weaver-admin",
            "--edges",
            "normal",
            "--prefix",
            "none",
        ])
        .output()
        .expect("cargo tree runs");
    assert!(
        out.status.success(),
        "cargo tree failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("utf8")
}

fn crate_names(tree: &str) -> Vec<String> {
    tree.lines()
        .filter_map(|l| l.split_whitespace().next())
        .map(|s| s.to_string())
        .collect()
}

/// The internal dependency is exactly `weaver-types`, and `weaver-traits`
/// arrives transitively rather than by a line of this crate's own - the
/// charter's declared non-link as a checkable absence.
#[test]
fn one_floor_link_and_no_direct_traits_line() {
    let manifest = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("manifest");
    // **The section is bounded at the next header.** Splitting on
    // `[dependencies]` alone runs past it into `[features]` and
    // `[dev-dependencies]`, so the assertions below could pass while the
    // property they name is false - a test that cannot fail.
    let after = manifest
        .split("\n[dependencies]\n")
        .nth(1)
        .expect("a dependency section");
    let deps = after.split("\n[").next().expect("a bounded section");
    assert!(deps.contains("weaver-types"), "the one floor link is named");
    assert!(
        !deps.contains("weaver-traits"),
        "weaver-traits is not a direct line: it arrives through weaver-types"
    );
    assert!(
        deps.contains("features = [\"config\"]"),
        "taken with the config feature, admin being the crate that parses the file"
    );
    // ...and the tree confirms it arrives all the same.
    let names = crate_names(&resolved_tree());
    assert!(
        names.iter().any(|n| n == "weaver-traits"),
        "transitively present"
    );
    assert!(
        names.iter().any(|n| n == "serde_yaml_ng"),
        "the parser is linked"
    );
}

/// No async runtime, no bus crate, no logging crate: the surface's traffic is
/// operator-paced, the init system is reached by its command-line interface,
/// and the operations log is this crate's own writer.
#[test]
fn no_runtime_no_bus_no_logging() {
    let names = crate_names(&resolved_tree());
    for banned in [
        "tokio",
        "async-std",
        "smol",
        "futures",
        "async-trait",
        "dbus",
        "zbus",
        "systemd",
        "tracing",
        "tracing-subscriber",
        "log",
        "env_logger",
    ] {
        assert!(
            !names.iter().any(|n| n == banned),
            "{banned} in the resolved tree"
        );
    }
}

/// One binary and no library surface: nothing links admin, so a library target
/// would be an API for a consumer the topology forbids. The manifest declares
/// the binary explicitly and no `src/lib.rs` exists for Cargo to find by
/// convention - which is the half a manifest read cannot see on its own, and
/// why the Spec tags this claim review.
#[test]
fn one_binary_and_no_library_surface() {
    let manifest = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("manifest");
    assert!(manifest.contains("[[bin]]"), "the binary is declared");
    assert!(!manifest.contains("[lib]"), "no library section");
    assert!(
        !std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs")).exists(),
        "no src/lib.rs for Cargo to find by convention"
    );
}
