//! The native backend against a real safetensors artifact, end to end:
//! resolve, header, admit, session, generation. The GGUF peer's suite is
//! `loaded.rs` and this file mirrors its discipline: real artifact, skipped
//! where the workshop lacks it, failing where an operator named one that is
//! not there.
#![cfg(all(feature = "gguf", feature = "cuda"))]

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

fn cuda_present() -> bool {
    cudarc::driver::CudaContext::new(0).is_ok()
}
