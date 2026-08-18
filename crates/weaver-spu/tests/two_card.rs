//! A pair binding, admitted and served: the first exercise of the two-device
//! load path since it was written. Measured 2026-08-17: the artifact splits
//! by layer, 18.2 GiB on ordinal 0 beside 17.0 GiB on ordinal 1, and both
//! cards return to idle at release.
//!
//! **A split GGUF cannot ride this path today and the reason is the pin.**
//! llama.cpp finds `-00002-of-00002` siblings by the filename pattern, and
//! the admission hands it the descriptor's `/proc/self/fd/N`, which carries
//! no pattern, so the load nulls in under two seconds. The collision between
//! the descriptor discipline and split artifacts is filed rather than worked
//! around here, because handing the real path back would undo what the pin
//! is for.
#![cfg(all(feature = "gguf", feature = "cuda"))]

use std::path::PathBuf;

use weaver_spu::decoder::session::{NeverCancels, StopCondition};
use weaver_spu::readout::ReadoutElection;
use weaver_spu::residency::{Headroom, Residency};
use weaver_spu::sampling::EffectiveKnobs;
use weaver_types::{ArtifactRef, DeviceOrdinal, ModelBinding};

fn artifact() -> Option<PathBuf> {
    match std::env::var_os("WEAVER_ARTIFACT_TWO_CARD") {
        Some(named) => {
            let path = PathBuf::from(named);
            assert!(
                path.is_file(),
                "WEAVER_ARTIFACT_TWO_CARD names a missing file"
            );
            Some(path)
        }
        None => {
            let path = PathBuf::from(
                "/bulk-store/models/Qwen--Qwen-AgentWorld-35B-A3B-GGUF/Qwen-AgentWorld-35B-A3B-Q8_0.gguf",
            );
            path.is_file().then_some(path)
        }
    }
}

#[test]
fn a_model_larger_than_one_card_admits_across_a_pair() {
    let Some(path) = artifact() else {
        eprintln!("SKIP: no two-card artifact");
        return;
    };
    let mut residency = Residency::new();
    let binding = ModelBinding {
        artifact: ArtifactRef(path.to_string_lossy().into_owned()),
        devices: vec![DeviceOrdinal(0), DeviceOrdinal(1)],
    };
    let resident = match residency.admit(
        &binding,
        Headroom(2 * 1024 * 1024 * 1024),
        ReadoutElection(false),
    ) {
        Ok(resident) => resident,
        Err(refusal) => panic!("the pair admit: {refusal:?}"),
    };
    eprintln!("ADMITTED across two devices");

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
    let mut session = resident.open_session(&knobs, 4096).expect("session opens");
    session.open(&prefix).expect("prefix decodes");
    let close = resident.tokenize("<|im_end|>").expect("close tokenizes");
    let terminator = close[0];
    let generated = session
        .append_and_generate(
            &[],
            &StopCondition {
                stop_tokens: vec![terminator],
                terminator,
                max_tokens: 16,
            },
            &mut NeverCancels,
            &mut |_| {},
        )
        .expect("generates");
    let text = resident.detokenize(&generated.tokens).expect("detokenizes");
    eprintln!("two-card emission: {text:?}");
    assert!(!generated.tokens.is_empty());
}
