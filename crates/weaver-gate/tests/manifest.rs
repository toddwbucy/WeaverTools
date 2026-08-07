//! conforms: gate-one-binary
//! conforms: gate-floor-link-types-without-config
//! conforms: gate-no-runtime-no-logging-no-yaml
//!
//! The manifest assertions of `weaver-gate-Spec` sections 1 and 6. Gate H2
//! reads the internal edge against the graph; these are the instrument for the
//! external half H2 does not check, plus the featureless take, which is a
//! `Cargo.toml` fact of its own rather than an edge.

use std::process::Command;

/// One helper for both depths, so a flag change cannot land in one copy and
/// leave the two tests asserting about different resolutions.
fn resolved_tree(edges: &str, depth: Option<&str>) -> String {
    let mut args = vec![
        "tree",
        "-p",
        "weaver-gate",
        "--edges",
        edges,
        "--prefix",
        "none",
        // The lock file is the subject: a resolution that fetched or updated
        // would be asserting about a tree this repository does not record.
        "--locked",
        "--offline",
    ];
    if let Some(depth) = depth {
        args.push("--depth");
        args.push(depth);
    }
    let out = Command::new(env!("CARGO"))
        .args(&args)
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

fn manifest() -> String {
    std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("the manifest reads")
}

/// **The internal set is exactly one floor link, and no organ appears.** A
/// crate this one answers to is reached over a socket, so no organ is in the
/// resolved tree at any depth.
///
/// `weaver-traits` is in the tree and is not a defect: it arrives transitively
/// under `weaver-types`, which is the floor linking its own sibling. What the
/// charter's floor-link set forbids is a *direct* line, which the test below
/// reads from the manifest, because a transitive arrival is the floor's shape
/// rather than this crate's draw.
#[test]
fn no_organ_is_in_the_resolved_tree() {
    let names = crate_names(&resolved_tree("normal", None));
    let mut internal: Vec<_> = names
        .iter()
        .filter(|n| n.starts_with("weaver-") && *n != "weaver-gate")
        .cloned()
        .collect();
    internal.sort();
    internal.dedup();
    assert_eq!(
        internal,
        vec!["weaver-traits", "weaver-types"],
        "the floor, reached directly and transitively, and no organ"
    );
    // Read against the whole resolved tree rather than the filtered set above,
    // which the assertion has already fixed: a loop over `internal` could
    // never fail and would report a check nobody ran.
    for organ in ["weaver-harness", "weaver-spu", "weaver-admin", "weaver-trace"] {
        assert!(
            !names.iter().any(|n| n == organ),
            "{organ} is a crate this one reaches over a socket, never a link"
        );
    }
}

/// **No direct `weaver-traits` line exists**, matching the charter's
/// floor-link set: nothing here draws a trait by name, so nothing declares one.
///
/// Perturbation: add a `weaver-traits` line to the dependency section and this
/// test fails. Watched under exactly that addition.
#[test]
fn the_direct_dependency_set_carries_no_traits_line() {
    // Structural first: the resolved direct set, which catches a renamed or
    // table-form declaration a text scan reads straight past.
    let direct = crate_names(&resolved_tree("normal", Some("1")));
    assert!(
        !direct.iter().any(|n| n == "weaver-traits"),
        "weaver-traits is a direct dependency, and the floor-link set is \
         weaver-types alone: found {direct:?}"
    );
    // And textually, because the manifest is what a reader edits.
    let manifest = manifest();
    assert!(
        !manifest
            .lines()
            .any(|l| l.trim_start().starts_with("weaver-traits")),
        "the manifest declares a weaver-traits line"
    );
}

/// **`weaver-types` is taken without its `config` feature.** This crate reads
/// no configuration file, the gate instruction arriving over the seam instead,
/// so no parser enters a process whose whole argument is that it holds little.
///
/// Perturbation: drop `default-features = false` from the dependency line and
/// this test fails. Watched under exactly that removal.
#[test]
fn the_floor_link_is_taken_without_config() {
    let manifest = manifest();
    let line = manifest
        .lines()
        .find(|l| l.trim_start().starts_with("weaver-types"))
        .expect("the floor link is declared");
    assert!(
        line.contains("default-features = false"),
        "the config feature would put a parser in this process, got: {line}"
    );
}

/// **One binary.** The gate is its own executable and nothing links it, so the
/// manifest declares exactly one `[[bin]]`.
///
/// The `[lib]` beside it is not an API for a consumer: it exists so the Spec's
/// bind-shape pins execute, cargo running doctests for lib targets only.
#[test]
fn the_manifest_declares_one_binary() {
    let manifest = manifest();
    assert_eq!(
        manifest.matches("[[bin]]").count(),
        1,
        "one binary, forked and exec'd by the harness"
    );
}

/// **No async runtime, no logging crate, no YAML implementation.** The
/// lifecycle traffic is two exchanges and the client traffic is deferred, so
/// nothing here needs an executor, and this crate writes no account of
/// anything: a logging crate would be a second author's first step.
///
/// Perturbation: add `tokio` or `tracing` to the manifest and this test fails.
/// Watched under exactly that addition.
#[test]
fn the_resolved_tree_carries_no_runtime_no_logging_no_yaml() {
    let names = crate_names(&resolved_tree("all", None));
    for forbidden in [
        "tokio",
        "async-std",
        "smol",
        "tracing",
        "log",
        "env_logger",
        "slog",
        "serde_yaml",
        "yaml-rust",
        "serde_yml",
    ] {
        assert!(
            !names.iter().any(|n| n == forbidden),
            "{forbidden} is in the resolved tree, and this crate takes none of its kind"
        );
    }
}
