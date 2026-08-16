//! conforms: spu-marker-promotion
//!
//! The inbound direction of `weaver-spu-Spec` section 5, bought as section 10
//! asks: every control marker a family renders tokenizes to exactly one token.
//!
//! **A marker that degrades to sub-word text is structure the model reads as
//! prose.** The renderer emits its markers as literal text, and that rendering
//! is faithful only where the tokenizer promotes each literal back to the one
//! control token it stands for. `str_to_token` calls `llama_tokenize` with
//! `parse_special` set, which promotes a literal **iff** the GGUF's tokenizer
//! marks that string special, so the property is a fact about the pairing of a
//! family's renderer with a family's tokenizer and not about either alone.
//!
//! **The load is vocab-only and reaches no device.** `with_vocab_only` reads
//! the header and the tokenizer and stops, so this needs the `gguf` feature and
//! no GPU, which is what the feature comment in `Cargo.toml` says that build
//! exists for. The archived tree's version of this test loaded full weights
//! onto `gpu_id: 0`; nothing here needs weights, and the salvage improves on
//! that rather than carrying it across.
//!
//! **Only the rendered set is asserted.** Promotion is the inbound claim, and a
//! family's parsed markers are matched as text against an emission this crate
//! never tokenizes. Where a parsed marker does not promote, the family module
//! says so at its declaration rather than this test asserting a property the
//! corpus does not claim.
#![cfg(feature = "gguf")]

use std::path::{Path, PathBuf};

use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel};

use weaver_spu::family::{gpt_oss, llama, qwen2};

/// A family, its rendered markers, and where a tokenizer for it is found.
///
/// The paths are defaults an operator overrides with `WEAVER_VOCAB_<FAMILY>`,
/// because a workshop's model layout is not a fact this crate should hold.
struct Probe {
    family: &'static str,
    env: &'static str,
    default_path: &'static str,
    rendered: &'static [&'static str],
}

const PROBES: &[Probe] = &[
    Probe {
        family: "llama",
        env: "WEAVER_VOCAB_LLAMA",
        default_path: "/fastpool/scratch/llama.cpp/models/ggml-vocab-llama-bpe.gguf",
        rendered: llama::RENDERED_MARKERS,
    },
    Probe {
        family: "qwen2",
        env: "WEAVER_VOCAB_QWEN2",
        default_path: "/opt/weaver/models/qwen2.5-0.5b-instruct-q6_k.gguf",
        rendered: qwen2::RENDERED_MARKERS,
    },
    Probe {
        // **Qwen3 is a separate registry key served by the qwen2 module**, so
        // its tokenizer is probed separately. Two keys citing one module is a
        // claim about their markers agreeing, and a claim is worth measuring.
        family: "qwen3",
        env: "WEAVER_VOCAB_QWEN3",
        default_path: "/opt/weaver/models/h-dist/Qwen3-8B-Q5_K_M.gguf",
        rendered: qwen2::RENDERED_MARKERS,
    },
    Probe {
        // **Qwen3.5 declares its own architecture and renders the same ChatML
        // scaffolding**, so it is probed separately for the reason qwen3 is:
        // two keys citing one module is a claim about their markers agreeing,
        // and a claim is worth measuring. The artifacts on this workshop are
        // community quantizations of Qwen3.6, which declare `qwen35`.
        family: "qwen35",
        env: "WEAVER_VOCAB_QWEN35",
        default_path: "/bulk-store/models/DavidAU--Qwen3.6-40B-Claude-4.6-Opus-Deckard-Heretic-Uncensored-Thinking-NEO-CODE-Di-IMatrix-MAX-GGUF/Qwen3.6-40B-Deck-Opus-NEO-CODE-HERE-2T-OT-IQ2_M.gguf",
        rendered: qwen2::RENDERED_MARKERS,
    },
    Probe {
        // The sparse sibling declares its own architecture again, so the same
        // claim is measured against its own tokenizer rather than assumed from
        // the dense one.
        family: "qwen35moe",
        env: "WEAVER_VOCAB_QWEN35MOE",
        default_path: "/bulk-store/books/models/unsloth--Qwen3.6-35B-A3B-GGUF/Qwen3.6-35B-A3B-UD-IQ1_M.gguf",
        rendered: qwen2::RENDERED_MARKERS,
    },
    Probe {
        family: "gpt-oss",
        env: "WEAVER_VOCAB_GPT_OSS",
        default_path: "/opt/weaver/models/h-dist/gpt-oss-20b-mxfp4.gguf",
        rendered: gpt_oss::RENDERED_MARKERS,
    },
];

impl Probe {
    /// The vocab to probe, and whether an operator named it.
    ///
    /// **The two are not the same absence.** A default path that is not there
    /// is a workshop without that family's artifact, which leaves the family
    /// unverified and is reported. An overridden path that is not there is an
    /// operator who asked for a vocabulary and got no measurement, which must
    /// not pass quietly.
    fn path(&self) -> (PathBuf, bool) {
        match std::env::var_os(self.env) {
            Some(value) => (PathBuf::from(value), true),
            None => (PathBuf::from(self.default_path), false),
        }
    }
}

/// A string shaped like a marker that no family declares.
///
/// **The control that keeps the assertion from being vacuous.** A tokenizer
/// that returned one token for every input would satisfy the promotion check
/// without promoting anything, so each vocab is also asked for a lookalike it
/// cannot know, and that must come back as more than one token. Without this
/// the test would pass against a tokenizer that had lost its special set
/// entirely.
const LOOKALIKE: &str = "<|not_a_marker_any_family_declares|>";

fn vocab_only(backend: &LlamaBackend, path: &Path) -> Result<LlamaModel, String> {
    let params = LlamaModelParams::default()
        .with_vocab_only(true)
        .with_n_gpu_layers(0);
    LlamaModel::load_from_file(backend, path, &params).map_err(|error| error.to_string())
}

/// **Every marker a family renders promotes to exactly one token.**
///
/// The failures are collected across every family before the assertion fires,
/// so one bad marker does not hide the rest. What a reader gets on a failure is
/// the family, the marker, and the token ids it degraded into.
///
/// Perturbation: add a marker to any family's `RENDERED_MARKERS` that its GGUF
/// does not mark special, and this fails naming it. Watched by adding
/// `<|python_tag|>` to llama's rendered set, which the reachable 3.0-era
/// tokenizer splits into six tokens.
#[test]
fn every_rendered_marker_promotes_to_one_token() {
    let backend = LlamaBackend::init().expect("the llama backend initialises");

    let mut failures: Vec<String> = Vec::new();
    let mut absent: Vec<String> = Vec::new();
    let mut probed = 0usize;

    for probe in PROBES {
        let (path, overridden) = probe.path();
        // An operator who named a vocab and whose path is not a regular file
        // gets a failure rather than a skip: the ask was explicit, so silence
        // would report a measurement that never ran.
        assert!(
            !overridden || path.is_file(),
            "{} is set to {} which is not a regular file, so {} would have been \
             skipped and this test would have passed without probing it",
            probe.env,
            path.display(),
            probe.family
        );
        if !path.exists() {
            absent.push(format!("{} (no vocab at {})", probe.family, path.display()));
            continue;
        }
        let model = match vocab_only(&backend, &path) {
            Ok(model) => model,
            Err(error) => {
                failures.push(format!("{}: vocab-only load failed: {error}", probe.family));
                continue;
            }
        };
        probed += 1;

        for marker in probe.rendered {
            match model.str_to_token(marker, AddBos::Never) {
                Ok(tokens) if tokens.len() == 1 => {}
                Ok(tokens) => failures.push(format!(
                    "{}: {marker} degraded to {} tokens {:?}",
                    probe.family,
                    tokens.len(),
                    tokens.iter().map(|t| t.0).collect::<Vec<_>>()
                )),
                Err(error) => failures.push(format!("{}: {marker}: {error}", probe.family)),
            }
        }

        // The control. A tokenizer answering one token for everything would
        // satisfy every assertion above without promoting anything.
        match model.str_to_token(LOOKALIKE, AddBos::Never) {
            Ok(tokens) if tokens.len() > 1 => {}
            Ok(tokens) => failures.push(format!(
                "{}: the lookalike promoted to {} token(s), so this vocab \
                 promotes indiscriminately and the assertions above prove nothing",
                probe.family,
                tokens.len()
            )),
            Err(error) => failures.push(format!("{}: lookalike: {error}", probe.family)),
        }
    }

    // **A run that probed nothing is a failure, not a pass.** The archived
    // tree's version returned early when no model was present, which reads as
    // green on a machine that tested nothing.
    //
    // The failures are reported here as well as below, because a vocab that
    // exists and will not load leaves `probed` at zero while `absent` stays
    // empty. Naming only the absent set would then report that nothing was
    // found when what happened is that everything failed to open.
    assert!(
        probed > 0,
        "no family vocab was reachable, so this test asserted nothing. Set one of \
         {}.\n  absent: {absent:?}\n  failed to load: {failures:?}",
        PROBES
            .iter()
            .map(|probe| probe.env)
            .collect::<Vec<_>>()
            .join(", ")
    );

    assert!(
        failures.is_empty(),
        "rendered markers that did not promote to a single token, which means the \
         literal-text rendering degrades to sub-word prose for them:\n  {}",
        failures.join("\n  ")
    );

    if !absent.is_empty() {
        // Named rather than silent: a family with no reachable vocab is
        // unverified here, and the run says which.
        eprintln!("marker promotion unverified for: {}", absent.join(", "));
    }
}
