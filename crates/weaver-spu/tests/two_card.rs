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
use std::sync::{Mutex, OnceLock};

use weaver_spu::decoder::session::{NeverCancels, StopCondition};
use weaver_spu::readout::ReadoutElection;
use weaver_spu::residency::{Headroom, Residency};
use weaver_spu::sampling::EffectiveKnobs;
use weaver_types::{ArtifactRef, DeviceOrdinal, ModelBinding};

/// One test in this binary today, and the lock stands anyway: a second
/// two-card measurement added later must not race the first for the same
/// devices, which is `loaded.rs`'s discipline carried over.
fn device_lock() -> std::sync::MutexGuard<'static, ()> {
    static DEVICE: OnceLock<Mutex<()>> = OnceLock::new();
    DEVICE
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn free_bytes(context: &cudarc::driver::CudaContext) -> u64 {
    let (free, _total) = context.mem_get_info().expect("the driver answers");
    free as u64
}

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

/// The split artifact, the class the ruling on the collision reopened: a
/// 64.6 GiB set across two files, larger than any single card this box
/// holds, admitted whole through the pinned set and the fork's explicit
/// splits door.
#[test]
fn a_split_artifact_larger_than_any_card_admits_across_the_pair() {
    let path = match std::env::var_os("WEAVER_ARTIFACT_SPLIT") {
        Some(named) => {
            let path = PathBuf::from(named);
            assert!(path.is_file(), "WEAVER_ARTIFACT_SPLIT names a missing file");
            path
        }
        None => {
            let path = PathBuf::from(
                "/bulk-store/models/unsloth--Qwen3.6-35B-A3B-GGUF/BF16/Qwen3.6-35B-A3B-BF16-00001-of-00002.gguf",
            );
            if !path.is_file() {
                eprintln!("SKIP: no split artifact");
                return;
            }
            path
        }
    };
    let (Some(card0), Some(card1)) = (
        cudarc::driver::CudaContext::new(0).ok(),
        cudarc::driver::CudaContext::new(1).ok(),
    ) else {
        eprintln!("SKIP: fewer than two CUDA devices");
        return;
    };
    let _device = device_lock();
    let before = (free_bytes(&card0), free_bytes(&card1));

    let mut residency = Residency::new();
    let binding = ModelBinding {
        artifact: ArtifactRef(path.to_string_lossy().into_owned()),
        devices: vec![DeviceOrdinal(0), DeviceOrdinal(1)],
    };
    let resident = residency
        .admit(
            &binding,
            Headroom(2 * 1024 * 1024 * 1024),
            ReadoutElection(false),
        )
        .expect("the split admits across the pair");

    // Larger than either card alone: each holds a share no one card could
    // spare beside the other's, which is the fact this artifact class is for.
    let floor = 24u64 * 1024 * 1024 * 1024;
    let held = (free_bytes(&card0), free_bytes(&card1));
    assert!(before.0 - held.0 > floor, "ordinal 0 holds a major share");
    assert!(before.1 - held.1 > floor, "ordinal 1 holds a major share");

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
            &mut |_| {},
        )
        .expect("generates");
    let text = resident.detokenize(&generated.tokens).expect("detokenizes");
    eprintln!("split emission: {text:?}");
    assert!(!generated.tokens.is_empty());

    drop(session);
    let _ = resident;
    residency.release().expect("the release succeeds");
    let after = (free_bytes(&card0), free_bytes(&card1));
    let tolerance: u64 = 1024 * 1024 * 1024;
    assert!(
        after.0 + tolerance > before.0,
        "ordinal 0 returns to baseline"
    );
    assert!(
        after.1 + tolerance > before.1,
        "ordinal 1 returns to baseline"
    );
}

#[test]
fn a_model_larger_than_one_card_admits_across_a_pair() {
    let Some(path) = artifact() else {
        eprintln!("SKIP: no two-card artifact");
        return;
    };
    let (Some(card0), Some(card1)) = (
        cudarc::driver::CudaContext::new(0).ok(),
        cudarc::driver::CudaContext::new(1).ok(),
    ) else {
        eprintln!("SKIP: fewer than two CUDA devices");
        return;
    };
    let _device = device_lock();
    let before = (free_bytes(&card0), free_bytes(&card1));

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
    // **Both cards hold a substantial share, asserted with thresholds well
    // under an even split**: the artifact is 34.4 GiB and a layer split is
    // not exactly even, so each card owing 8 GiB catches a one-card load
    // masquerading as a pair without flaking on allocator granularity.
    let floor = 8u64 * 1024 * 1024 * 1024;
    let held = (free_bytes(&card0), free_bytes(&card1));
    assert!(
        before.0 - held.0 > floor,
        "ordinal 0 holds a share: {} -> {}",
        before.0,
        held.0
    );
    assert!(
        before.1 - held.1 > floor,
        "ordinal 1 holds a share: {} -> {}",
        before.1,
        held.1
    );
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
    // The close must promote to exactly one token against this artifact's
    // vocabulary, or the override named an artifact whose stop this test
    // would misbuild.
    let close = resident.tokenize("<|im_end|>").expect("close tokenizes");
    let [terminator] = close.as_slice() else {
        panic!("the turn close promotes to one token, got {close:?}");
    };
    let terminator = *terminator;
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

    // **The release frees both cards.** The session and the resident borrow
    // the residency, so both end before it releases, and the assertion reads
    // the devices rather than trusting the drop order.
    drop(session);
    let _ = resident;
    residency.release().expect("the release succeeds");
    let after = (free_bytes(&card0), free_bytes(&card1));
    let tolerance: u64 = 1024 * 1024 * 1024;
    assert!(
        after.0 + tolerance > before.0,
        "ordinal 0 returns to baseline: {} -> {}",
        before.0,
        after.0
    );
    assert!(
        after.1 + tolerance > before.1,
        "ordinal 1 returns to baseline: {} -> {}",
        before.1,
        after.1
    );
}
