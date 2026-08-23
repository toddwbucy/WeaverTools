//! conforms: spu-fixture-surface-pinned
//!
//! The fixture-surface assertion of `weaver-spu-Spec` section 1.2, checked the
//! way this workspace's other manifest tests check theirs: by reading the build
//! rather than by trusting a comment.
//!
//! **This crate had no manifest test until #288, and it is the crate with the
//! feature gates.** Every other crate in the workspace carries one. The absence
//! is why the Spec's table could have been written and then drifted with nobody
//! finding out.
//!
//! **What it pins and what it deliberately does not.** It pins that every
//! artifact variable a test target reads appears in the table, and that every
//! variable the table names is read by a target. It does not pin the vocabulary
//! roster: `markers.rs` asks each registry entry for its own artifact under
//! `WEAVER_VOCAB_<FAMILY>`, so the set grows with the registry and a fixed list
//! would be stale on the next family. The shape is checked instead.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

/// The table of `weaver-spu-Spec` section 1.2, less the per-family vocabulary
/// set, which is checked by shape below.
///
/// **Transcribed from the Spec rather than derived from the tests.** A list
/// generated from the sources would agree with them whatever the document said,
/// which is the drift this test exists to catch.
const DOCUMENTED: &[&str] = &[
    "WEAVER_ARTIFACT_GEMMA4",
    "WEAVER_ARTIFACT_MISTRAL_SMALL",
    "WEAVER_ARTIFACT_PHI4",
    "WEAVER_ARTIFACT_PHI4_MINI",
    "WEAVER_ARTIFACT_QWEN25_32B",
    "WEAVER_ARTIFACT_QWEN25_SAFETENSORS",
    "WEAVER_ARTIFACT_READOUT",
    "WEAVER_ARTIFACT_SMOLLM2",
    "WEAVER_ARTIFACT_SPLIT",
    "WEAVER_ARTIFACT_TWO_CARD",
    "WEAVER_MEASURE_PACE",
    "WEAVER_TEST_GGUF",
];

/// The prefix `markers.rs` builds its per-family names from.
const VOCAB_PREFIX: &str = "WEAVER_VOCAB_";

/// Every `WEAVER_*` name appearing as a string literal in this crate's test
/// sources.
///
/// Read off disk rather than compiled in, because a `cfg`-gated target is not
/// compiled on a host build and its variables would vanish from a compiled
/// inventory exactly when the gate is what hid them. That is the failure this
/// whole issue is about, and it would be reproduced here.
///
/// **The quote is what makes this a name rather than prose.** An earlier form
/// searched the bare text and read a doc comment's `WEAVER_VOCAB_<FAMILY>` as a
/// variable no test asks the environment for. Requiring the surrounding quote
/// leaves comments, documentation, and diagnostic prose out without a parser.
///
/// **Anchoring to `env::var` call sites was considered and rejected.** Two
/// access shapes exist here: `loaded.rs` and its neighbours call
/// `env::var_os` directly, while `markers.rs` and `selection.rs` carry the name
/// as a field of a fixture table and look it up elsewhere. A scan anchored to
/// the call site would miss eighteen of the twenty-one names, which is the
/// under-reporting this test exists to prevent. A real token parse would reach
/// both, and it would put a parser in the dependency set of a crate whose Spec
/// section 1.1 argues every dependency it takes.
fn variables_in_tests() -> BTreeSet<String> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let mut found = BTreeSet::new();
    for entry in fs::read_dir(&dir).expect("the tests directory reads") {
        let path = entry.expect("the entry reads").path();
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        // **This file is not a subject of its own scan.** It names variables in
        // its roster, its prefix constant, and its own perturbation note, and a
        // scanner that read them would report the document's table as a finding
        // against itself. Watched doing exactly that before this line existed.
        if path.file_name().is_some_and(|n| n == "manifest.rs") {
            continue;
        }
        let source = fs::read_to_string(&path).expect("the source reads");
        let bytes = source.as_bytes();
        let mut at = 0;
        // A name is `"WEAVER_..."`, quoted. Both access shapes in this crate
        // put it in a string literal: a direct `env::var_os` argument, and a
        // fixture table's field. Prose carries it in backticks or bare, and is
        // left out by the opening quote alone.
        while let Some(offset) = source[at..].find("\"WEAVER_") {
            let start = at + offset + 1;
            let mut end = start;
            while end < bytes.len()
                && (bytes[end].is_ascii_uppercase()
                    || bytes[end].is_ascii_digit()
                    || bytes[end] == b'_')
            {
                end += 1;
            }
            // The literal must close where the name ends, or the match is a
            // longer string that merely begins with the prefix.
            if bytes.get(end) == Some(&b'"') {
                found.insert(source[start..end].to_string());
            }
            at = end;
        }
    }
    found
}

/// **Every variable a test reads is in the Spec's table, and every entry in the
/// table is read.**
///
/// Perturbation: add a `WEAVER_ARTIFACT_ANYTHING` to any test in this crate and
/// this fails naming it, until section 1.2 carries it. Delete a row from
/// `DOCUMENTED` and it fails the other way.
#[test]
fn the_fixture_surface_matches_the_documented_table() {
    let found = variables_in_tests();
    let documented: BTreeSet<String> =
        DOCUMENTED.iter().map(|s| s.to_string()).collect();

    // The per-family vocabulary names are the set the Spec checks by shape, so
    // they are held out of the roster comparison and checked below.
    let concrete: BTreeSet<String> = found
        .iter()
        .filter(|v| !v.starts_with(VOCAB_PREFIX))
        .cloned()
        .collect();

    let undocumented: Vec<&String> = concrete.difference(&documented).collect();
    assert!(
        undocumented.is_empty(),
        "these are read by a test and absent from weaver-spu-Spec section 1.2: \
         {undocumented:?}"
    );
    let unread: Vec<&String> = documented.difference(&concrete).collect();
    assert!(
        unread.is_empty(),
        "these are in section 1.2's table and read by no test: {unread:?}"
    );
}

/// **The vocabulary set is per family and is checked by shape.**
///
/// The Spec declines to list the roster because it grows with the registry.
/// What it does claim is that the names are built from one prefix, so a test
/// reaching for a vocabulary artifact under some other spelling is a drift the
/// table cannot describe.
#[test]
fn the_vocabulary_variables_share_one_prefix() {
    let vocab: Vec<String> = variables_in_tests()
        .into_iter()
        .filter(|v| v.starts_with(VOCAB_PREFIX))
        .collect();
    assert!(
        !vocab.is_empty(),
        "no per-family vocabulary variable found, so the shape claim reads \
         nothing"
    );
    for name in &vocab {
        let family = &name[VOCAB_PREFIX.len()..];
        assert!(
            !family.is_empty(),
            "{name} carries the prefix and names no family"
        );
    }
}
