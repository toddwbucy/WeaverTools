//! The native backend against a real safetensors artifact, end to end:
//! resolve, header, admit, session, generation. The GGUF peer's suite is
//! `loaded.rs` and this file mirrors its discipline: real artifact, skipped
//! where the workshop lacks it, failing where an operator named one that is
//! not there.
#![cfg(feature = "cuda")]

use std::path::PathBuf;

use weaver_spu::artifact;
use weaver_spu::decoder::session::{NeverCancels, StopCondition};
use weaver_spu::family::FamilyName;
use weaver_spu::readout::ReadoutElection;
use weaver_spu::residency::{Headroom, Residency};
use weaver_spu::sampling::EffectiveKnobs;
use weaver_types::{ArtifactRef, DeviceOrdinal, ModelBinding};

/// The safetensors artifact, a directory the resolution walks into. The
/// override follows the per-fixture pattern of `selection.rs`: unset and
/// absent skips, named and absent fails.
fn artifact_dir() -> Option<PathBuf> {
    match std::env::var_os("WEAVER_ARTIFACT_QWEN25_SAFETENSORS") {
        Some(named) => {
            let path = PathBuf::from(named);
            assert!(
                path.is_dir(),
                "WEAVER_ARTIFACT_QWEN25_SAFETENSORS names {}, which is not a directory",
                path.display()
            );
            Some(path)
        }
        None => {
            let path = PathBuf::from("/bulk-store/models/Qwen--Qwen2.5-0.5B-Instruct");
            path.is_dir().then_some(path)
        }
    }
}

/// The header reads the sidecars: a stock export's `__metadata__` carries
/// only its format, so the family and the template arrive from `config.json`
/// and `tokenizer_config.json` beside the weights.
#[test]
fn the_header_reads_the_sidecars() {
    let Some(dir) = artifact_dir() else {
        eprintln!("SKIP the_header_reads_the_sidecars: no safetensors artifact");
        return;
    };
    let resolved = artifact::resolve(&ArtifactRef(dir.to_string_lossy().into_owned()))
        .expect("a directory artifact resolves to its container");
    let mut pinned = artifact::pin(&resolved).expect("the container pins");
    let header = artifact::read_header(&mut pinned).expect("the header reads");
    assert_eq!(header.family, FamilyName("qwen2".into()));
    assert_eq!(header.hidden_size, Some(896));
    assert_eq!(header.layer_count, Some(24));
    assert!(
        header
            .chat_template
            .as_deref()
            .is_some_and(|t| t.contains("<|im_start|>")),
        "the template arrives from tokenizer_config.json"
    );
}

/// One full pass: admit onto the device, open a session, decode a rendered
/// prompt, and generate. What this proves is the seam, not the model: the
/// native engine serves the same five primitives the GGUF peer serves, under
/// the same session loop.
#[test]
fn a_safetensors_artifact_generates_through_the_native_engine() {
    let Some(dir) = artifact_dir() else {
        eprintln!("SKIP a_safetensors_artifact_generates: no safetensors artifact");
        return;
    };
    if !cuda_present() {
        eprintln!("SKIP a_safetensors_artifact_generates: no CUDA device");
        return;
    }

    let mut residency = Residency::new();
    let binding = ModelBinding {
        artifact: ArtifactRef(dir.to_string_lossy().into_owned()),
        devices: vec![DeviceOrdinal(0)],
    };
    let resident = residency
        .admit(&binding, Headroom(64 * 1024 * 1024), ReadoutElection(false))
        .expect("the admit succeeds against a real artifact");

    // The prompt through the family's own rendering, tokenized against the
    // artifact's own vocabulary.
    let prompt =
        "<|im_start|>user\nReply with exactly one word: hello<|im_end|>\n<|im_start|>assistant\n";
    let prefix = resident.tokenize(prompt).expect("the prompt tokenizes");
    assert!(!prefix.is_empty(), "a rendered prompt yields tokens");

    let knobs = EffectiveKnobs {
        temperature: 0.0,
        top_k: 1,
        top_p: 1.0,
        repetition_penalty: 1.0,
        repetition_window: 0,
        seed: 11,
    };
    let mut session = resident
        .open_session(&knobs, 512)
        .expect("a session opens over the native residency");
    session.open(&prefix).expect("the prefix decodes");

    // `<|im_end|>` is the family's turn close, promoted against this
    // vocabulary at a known id; asserting the promotion here keeps the stop
    // set honest rather than assumed.
    let close = resident
        .tokenize("<|im_end|>")
        .expect("the close tokenizes");
    let [terminator] = close.as_slice() else {
        panic!("the turn close promotes to one token, got {close:?}");
    };

    let generated = session
        .append_and_generate(
            &[],
            &StopCondition {
                stop_tokens: vec![*terminator],
                terminator: *terminator,
                max_tokens: 16,
            },
            &mut NeverCancels,
            &mut |_token| {},
        )
        .expect("the generation completes");

    assert!(
        !generated.tokens.is_empty(),
        "the model generated at least one token"
    );
    let text = resident
        .detokenize(&generated.tokens)
        .expect("the emission detokenizes");
    assert!(
        !text.trim().is_empty(),
        "the emission renders to text: {text:?}"
    );
    eprintln!("native emission: {text:?}");
}

/// The pair serves the same artifact the single card serves, and greedy
/// decoding answers the same first tokens: the sharded forward is the same
/// model cut in half, so the strongest cheap assertion is agreement with the
/// whole. Reduction order differs between the two, so low-order logit bits
/// may differ where two candidates tie; the first token is asserted and the
/// rest is reported.
#[test]
fn the_pair_agrees_with_the_single_card() {
    let Some(dir) = artifact_dir() else {
        eprintln!("SKIP the_pair_agrees: no safetensors artifact");
        return;
    };
    if cudarc::driver::CudaContext::new(1).is_err() {
        eprintln!("SKIP the_pair_agrees: fewer than two CUDA devices");
        return;
    }
    let knobs = EffectiveKnobs {
        temperature: 0.0,
        top_k: 1,
        top_p: 1.0,
        repetition_penalty: 1.0,
        repetition_window: 0,
        seed: 11,
    };
    let prompt =
        "<|im_start|>user\nReply with exactly one word: hello<|im_end|>\n<|im_start|>assistant\n";

    let mut emissions = Vec::new();
    for devices in [
        vec![DeviceOrdinal(0)],
        vec![DeviceOrdinal(0), DeviceOrdinal(1)],
    ] {
        let width = devices.len();
        let mut residency = Residency::new();
        let binding = ModelBinding {
            artifact: ArtifactRef(dir.to_string_lossy().into_owned()),
            devices,
        };
        let resident = residency
            .admit(&binding, Headroom(64 * 1024 * 1024), ReadoutElection(false))
            .unwrap_or_else(|refusal| panic!("the admit at width {width}: {refusal:?}"));
        let prefix = resident.tokenize(prompt).expect("tokenizes");
        let mut session = resident.open_session(&knobs, 512).expect("session opens");
        session.open(&prefix).expect("prefix decodes");
        let close = resident.tokenize("<|im_end|>").expect("close tokenizes");
        let [terminator] = close.as_slice() else {
            panic!("the turn close promotes to one token, got {close:?}");
        };
        let generated = session
            .append_and_generate(
                &[],
                &StopCondition {
                    stop_tokens: close.clone(),
                    terminator: *terminator,
                    max_tokens: 8,
                },
                &mut NeverCancels,
                &mut |_| {},
            )
            .expect("generates");
        let text = resident.detokenize(&generated.tokens).expect("detokenizes");
        eprintln!(
            "width {width} emission: {text:?} tokens: {:?}",
            generated.tokens
        );
        emissions.push(generated.tokens.clone());
    }
    // Greedy at temperature zero over the same weights: the full sequences
    // agree, measured, and the whole sequence is the pin. A future flake
    // here would mean two candidates tied close enough for reduction order
    // to pick differently, and that is a fact to meet with evidence in hand
    // rather than pre-excused by a weaker assertion.
    assert!(
        !emissions[0].is_empty() && !emissions[1].is_empty(),
        "both widths generated"
    );
    assert_eq!(
        emissions[0], emissions[1],
        "the pair's greedy sequence agrees with the single card's"
    );
}

/// The model the pair exists for: larger than any single card, sharded
/// safetensors, served across both devices through the native engine. The
/// deliberate mirror of `two_card.rs`'s split-GGUF proof in the second
/// container.
#[test]
fn a_large_sharded_safetensors_serves_across_the_pair() {
    let overridden = std::env::var_os("WEAVER_ARTIFACT_QWEN25_32B").is_some();
    let dir = match std::env::var_os("WEAVER_ARTIFACT_QWEN25_32B") {
        Some(named) => {
            let path = PathBuf::from(named);
            assert!(
                path.is_dir(),
                "WEAVER_ARTIFACT_QWEN25_32B names {}, which is not a directory",
                path.display()
            );
            path
        }
        None => {
            let path = PathBuf::from("/bulk-store/models/Qwen--Qwen2.5-32B-Instruct");
            if !path.is_dir() || !path.join("model-00001-of-00017.safetensors").is_file() {
                eprintln!("SKIP a_large_sharded_safetensors: artifact absent or incomplete");
                return;
            }
            path
        }
    };
    if cudarc::driver::CudaContext::new(1).is_err() {
        eprintln!("SKIP a_large_sharded_safetensors: fewer than two CUDA devices");
        return;
    }

    // The premise, asserted the way the GGUF split test asserts it: either
    // card alone refuses for room.
    for ordinal in [0u32, 1] {
        let mut probe = Residency::new();
        let single = ModelBinding {
            artifact: ArtifactRef(dir.to_string_lossy().into_owned()),
            devices: vec![DeviceOrdinal(ordinal)],
        };
        let refused = probe.admit(
            &single,
            Headroom(2 * 1024 * 1024 * 1024),
            ReadoutElection(false),
        );
        assert!(
            refused.is_err(),
            "the 32B fits no single card, so ordinal {ordinal} alone refuses"
        );
    }

    let mut residency = Residency::new();
    let binding = ModelBinding {
        artifact: ArtifactRef(dir.to_string_lossy().into_owned()),
        devices: vec![DeviceOrdinal(0), DeviceOrdinal(1)],
    };
    let resident = residency
        .admit(
            &binding,
            Headroom(2 * 1024 * 1024 * 1024),
            ReadoutElection(false),
        )
        .expect("the 32B admits across the pair");

    let prompt =
        "<|im_start|>user\nReply with exactly one word: hello<|im_end|>\n<|im_start|>assistant\n";
    let prefix = resident.tokenize(prompt).expect("tokenizes");
    let knobs = EffectiveKnobs {
        temperature: 0.0,
        top_k: 1,
        top_p: 1.0,
        repetition_penalty: 1.0,
        repetition_window: 0,
        seed: 11,
    };
    let mut session = resident.open_session(&knobs, 1024).expect("session opens");
    let opened = std::time::Instant::now();
    session.open(&prefix).expect("the prefix decodes");
    let close = resident.tokenize("<|im_end|>").expect("close tokenizes");
    let [terminator] = close.as_slice() else {
        panic!("the turn close promotes to one token, got {close:?}");
    };
    let generated = session
        .append_and_generate(
            &[],
            &StopCondition {
                stop_tokens: close.clone(),
                terminator: *terminator,
                max_tokens: 16,
            },
            &mut NeverCancels,
            &mut |_| {},
        )
        .expect("the 32B generates");
    let secs = opened.elapsed().as_secs_f64();
    let text = resident.detokenize(&generated.tokens).expect("detokenizes");
    eprintln!(
        "32B pair emission: {text:?} ({} tokens, {secs:.1}s with prefill)",
        generated.tokens.len()
    );
    assert!(!generated.tokens.is_empty());
    // **The known fixture's answer is pinned exactly**: greedy at zero
    // temperature over fixed weights is deterministic, and the measured
    // emission is the one word the instruction asks for, in one token. An
    // operator's override may name a model whose obedience this test never
    // promised, so the content pin holds for the fixture the test knows and
    // the serving assertions above hold for any.
    if !overridden {
        assert_eq!(generated.tokens.len(), 1, "one word, one token");
        assert_eq!(text.trim(), "hello", "the instruction is followed exactly");
    }
}

fn cuda_present() -> bool {
    cudarc::driver::CudaContext::new(0).is_ok()
}
