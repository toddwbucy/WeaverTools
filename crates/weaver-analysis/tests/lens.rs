//! conforms: analysis-control-gates-the-reading
//! conforms: analysis-captures-compare-exactly
//! conforms: analysis-threaded-head-is-bit-identical
//!
//! The reading's watches, per `weaver-analysis-Spec` sections 5 and 6. The
//! fixtures are the real records of 2026-09-01: the certified column
//! replay and its repeat, whose differencing measured the vector bar.

use weaver_analysis::{Capture, Comparison, LensRefusal, compare, manifest_path_for, parse_record};

const CERTIFIED: &str = include_str!("fixtures/columns-a.ndjson");
const REPEAT: &str = include_str!("fixtures/columns-b.ndjson");

/// **Two captures of one run compare exactly**, which is certification
/// step 3's own check: within one device model the bar is exact, per
/// `weaver-diagnostic-PRD` section 4 as measured.
///
/// Perturbation: compare with `!=` relaxed to an epsilon and a
/// deliberately altered value passes. Watched under exactly that change,
/// by the divergence case below.
#[test]
fn two_captures_of_one_run_are_identical() {
    let a = Capture::of(&parse_record(CERTIFIED));
    let b = Capture::of(&parse_record(REPEAT));
    assert!(!a.columns.is_empty(), "the fixture holds columns");
    match compare(&a, &b) {
        Comparison::Identical { positions, values } => {
            assert_eq!(positions, a.columns.len());
            assert!(values > 0, "the verdict rests on values");
        }
        other => panic!("the repeat is identical: {other:?}"),
    }
}

/// **One bit of difference diverges, and the comparison names where.**
/// The value is moved by a single representable step, which is the whole
/// of the exactness claim: a tolerance wide enough to admit arithmetic
/// noise would admit this too, and the bar the measurement bought is
/// exact.
///
/// Perturbation: relax the comparison to a relative epsilon - the shape a
/// tolerance-bearing implementation takes - and this fails, the one-bit
/// difference passing as identical. Watched under exactly that change.
#[test]
fn one_bit_of_difference_diverges_naming_the_site() {
    let a = Capture::of(&parse_record(CERTIFIED));
    let mut b = Capture::of(&parse_record(REPEAT));
    let key = b.columns.keys().next().cloned().expect("a column");
    let held = b.columns[&key][0][0];
    b.columns.get_mut(&key).unwrap()[0][0] = f32::from_bits(held.to_bits() ^ 1);
    match compare(&a, &b) {
        Comparison::Diverged {
            position, layer, ..
        } => {
            assert_eq!(position, key.1);
            assert_eq!(layer, 0);
        }
        other => panic!("a changed value diverges: {other:?}"),
    }
}

/// **Cardinality is checked and never truncated**: a ragged or empty
/// comparison refuses rather than verdicting over what happens to align.
#[test]
fn ragged_and_empty_comparisons_refuse() {
    let a = Capture::of(&parse_record(CERTIFIED));
    let mut short = Capture::of(&parse_record(REPEAT));
    let key = short.columns.keys().next().cloned().expect("a column");
    short.columns.get_mut(&key).unwrap().pop();
    assert!(
        matches!(compare(&a, &short), Comparison::Incomparable { .. }),
        "a dropped layer refuses"
    );

    let mut narrow = Capture::of(&parse_record(REPEAT));
    narrow.columns.get_mut(&key).unwrap()[0].pop();
    assert!(
        matches!(compare(&a, &narrow), Comparison::Incomparable { .. }),
        "a narrowed layer refuses"
    );

    let mut hollow = Capture::of(&parse_record(REPEAT));
    for column in hollow.columns.values_mut() {
        column.clear();
    }
    assert!(
        matches!(compare(&a, &hollow), Comparison::Incomparable { .. }),
        "an empty column refuses rather than verdicting over no evidence"
    );

    let empty = Capture::default();
    assert!(
        matches!(compare(&a, &empty), Comparison::Incomparable { .. }),
        "an empty capture refuses"
    );
}

/// **Signed zeros and NaNs are bits, not values.** `0.0` and `-0.0`
/// compare equal in arithmetic and differ in bytes, and a `NaN` compares
/// unequal to its own bit pattern - so an arithmetic comparison would
/// admit one difference and invent another, over a bar the measurement
/// took across bytes.
///
/// Perturbation: compare with `!=` on the values and this fails both ways,
/// the signed zeros passing as identical and the equal NaNs diverging.
/// Watched under exactly that change.
#[test]
fn signed_zeros_differ_and_equal_nans_do_not() {
    let a = Capture::of(&parse_record(CERTIFIED));
    let key = a.columns.keys().next().cloned().expect("a column");

    let mut positive = a.clone_shallow();
    let mut negative = a.clone_shallow();
    positive.columns.get_mut(&key).unwrap()[0][0] = 0.0_f32;
    negative.columns.get_mut(&key).unwrap()[0][0] = -0.0_f32;
    assert!(
        matches!(compare(&positive, &negative), Comparison::Diverged { .. }),
        "a signed zero is a difference in the bytes"
    );

    let mut left = a.clone_shallow();
    let mut right = a.clone_shallow();
    let nan = f32::from_bits(0x7fc0_0001);
    left.columns.get_mut(&key).unwrap()[0][0] = nan;
    right.columns.get_mut(&key).unwrap()[0][0] = nan;
    assert!(
        matches!(compare(&left, &right), Comparison::Identical { .. }),
        "one NaN's bits equal their own"
    );
}

/// **A capture pairs by turn, not by position alone**: positions repeat
/// across brackets, so a column belongs to its own turn's measurement.
#[test]
fn columns_pair_by_turn() {
    let capture = Capture::of(&parse_record(CERTIFIED));
    let turns: std::collections::BTreeSet<_> =
        capture.columns.keys().map(|(t, _)| t.clone()).collect();
    assert!(turns.len() > 1, "the fixture spans turns");
    for key in capture.paired() {
        assert!(capture.drawn.contains_key(&key), "every read column drew");
    }
    let doubled = format!("{CERTIFIED}{CERTIFIED}");
    let twice = Capture::of(&parse_record(&doubled));
    assert_eq!(
        twice.columns.len(),
        capture.columns.len(),
        "one bracket's positions do not multiply with a repeated record"
    );
}

/// **The manifest is derived from the lens it identifies**, tag and all,
/// and a manifest naming another lens refuses.
#[test]
fn the_manifest_is_the_lens_it_names() {
    use std::path::Path;
    assert_eq!(
        manifest_path_for(Path::new("/x/y/jacobian_lens_m-bf16.safetensors")),
        Path::new("/x/y/lens-manifest.json")
    );
    assert_eq!(
        manifest_path_for(Path::new("/x/y/jacobian_lens_m-bf16-1000p.safetensors")),
        Path::new("/x/y/lens-manifest-1000p.json")
    );

    let dir = std::env::temp_dir().join(format!("weaver-analysis-lens-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch");
    let lens = dir.join("jacobian_lens_m-bf16.safetensors");
    std::fs::write(&lens, b"").expect("lens");
    let write = |manifest: &str| std::fs::write(dir.join("lens-manifest.json"), manifest);

    write(r#"{"lens":"another.safetensors","fitted_for":{"model":"/m","model_safetensors_sha256":"0000000000000000000000000000000000000000000000000000000000000000"},"lens_shape":{"d_model":896,"source_layers":[0,1]}}"#).expect("w");
    assert!(matches!(
        weaver_analysis::read_manifest(&lens),
        Err(LensRefusal::ManifestNamesAnotherLens { .. })
    ));

    write(r#"{"lens":"jacobian_lens_m-bf16.safetensors","fitted_for":{"model":"/m","model_safetensors_sha256":"nope"},"lens_shape":{"d_model":896,"source_layers":[0,1]}}"#).expect("w");
    assert!(matches!(
        weaver_analysis::read_manifest(&lens),
        Err(LensRefusal::DigestMalformed { .. })
    ));

    write(r#"{"lens":"jacobian_lens_m-bf16.safetensors","fitted_for":{"model":"/m","model_safetensors_sha256":"0000000000000000000000000000000000000000000000000000000000000000"},"lens_shape":{"d_model":896,"source_layers":[1,0]}}"#).expect("w");
    assert!(matches!(
        weaver_analysis::read_manifest(&lens),
        Err(LensRefusal::LayerSetUnsorted { .. })
    ));

    write(r#"{"lens":"jacobian_lens_m-bf16.safetensors","fitted_for":{"model":"/m"},"lens_shape":{"d_model":896,"source_layers":[0]}}"#).expect("w");
    assert!(
        matches!(
            weaver_analysis::read_manifest(&lens),
            Err(LensRefusal::ManifestUnreadable { .. })
        ),
        "a missing member is malformed rather than read past"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// **An epsilon that parses is not yet one that norms.** Zero and the
/// negatives put a non-positive quantity under the reciprocal square root,
/// and an infinity or a NaN carries through every logit, so each refuses
/// rather than reaching the readout.
///
/// Perturbation: accept any value that parses and this fails on the first
/// case. Watched under exactly that change.
#[test]
fn the_epsilon_must_be_finite_and_positive() {
    for bad in [
        "0", "0.0", "-0.0", "-1e-6", "nan", "NaN", "inf", "-inf", "infinity", "x",
    ] {
        assert!(
            weaver_analysis::rms_epsilon(bad).is_none(),
            "{bad:?} is not an epsilon"
        );
    }
    assert_eq!(weaver_analysis::rms_epsilon("1e-6"), Some(1e-6));
    assert_eq!(weaver_analysis::rms_epsilon("0.000001"), Some(0.000001));
}

/// **The digest is the standard's**, checked against a known vector so a
/// wrong identity check cannot pass silently.
#[test]
fn the_digest_is_the_standard() {
    assert_eq!(
        weaver_analysis::sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(
        weaver_analysis::sha256_hex(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

/// **The head across the cores is the head on one core, to the bit**, per
/// Spec section 5: the rows are split across scoped threads and each row's
/// sum runs in one order, so no logit moves. A vocabulary wider than the
/// box's thread count, so the split is a real one, and values the rows
/// cannot all share, so a row landing under the wrong token would show.
///
/// Perturbation: swap the first two chunks' row ranges, which are equal in
/// size and so stay in bounds, and it fails naming the rows that moved
/// under their tokens. Reversing every chunk, or offsetting each by one,
/// runs the uneven last chunk past the head and fails on the range check
/// instead of on this claim.
#[test]
fn the_threaded_head_is_the_single_thread_head_to_the_bit() {
    let scratch = std::env::temp_dir().join(format!("weaver-analysis-head-{}", std::process::id()));
    std::fs::create_dir_all(&scratch).expect("scratch");
    let (vocabulary, width) = (301usize, 8usize);
    // A fixed generator, so the fixture is the same on every run and no
    // two rows agree.
    let mut state = 0x9E37_79B9u32;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        (state as f32 / u32::MAX as f32) * 2.0 - 1.0
    };
    let head: Vec<f32> = (0..vocabulary * width).map(|_| next()).collect();
    let norm: Vec<f32> = (0..width).map(|_| next()).collect();
    let residual: Vec<f32> = (0..width).map(|_| next()).collect();
    let bytes =
        |values: &[f32]| -> Vec<u8> { values.iter().flat_map(|v| v.to_le_bytes()).collect() };
    let (head_bytes, norm_bytes) = (bytes(&head), bytes(&norm));
    let views = vec![
        (
            "lm_head.weight",
            safetensors::tensor::TensorView::new(
                safetensors::Dtype::F32,
                vec![vocabulary, width],
                &head_bytes,
            )
            .expect("view"),
        ),
        (
            "model.norm.weight",
            safetensors::tensor::TensorView::new(safetensors::Dtype::F32, vec![width], &norm_bytes)
                .expect("view"),
        ),
    ];
    let weights = scratch.join("m.safetensors");
    std::fs::write(
        &weights,
        safetensors::serialize(views, None).expect("serialize"),
    )
    .expect("write");

    let unembedding = weaver_analysis::Unembedding::open(&weights, 1e-6).expect("weights");
    // **The split is forced, not left to the box**: eight workers over 301
    // rows is seven chunks of 38 and a last of 35, a real partition with an
    // uneven remainder whatever `available_parallelism` reports here. Seven
    // divided 301 exactly, which the review caught.
    let threaded = unembedding
        .logits_with_workers(&residual, 8)
        .expect("width");
    let normalized = unembedding.normalized(&residual).expect("width");
    let mut single = vec![0.0f32; vocabulary];
    unembedding
        .logits_rows(&normalized, 0, &mut single)
        .expect("every row fits the head");
    assert_eq!(threaded.len(), vocabulary);
    let differing = threaded
        .iter()
        .zip(&single)
        .enumerate()
        .filter(|(_, (a, b))| a.to_bits() != b.to_bits())
        .map(|(token, _)| token)
        .collect::<Vec<_>>();
    assert!(
        differing.is_empty(),
        "rows that moved under the split: {differing:?}"
    );
    // The box's own split agrees too, judged after the named assertion so
    // a perturbation fails on the claim the watch exists for.
    assert_eq!(unembedding.logits(&residual).expect("width"), threaded);
    // And a range past the head, or a residual of the wrong width, refuses
    // rather than summing a prefix.
    let mut past = vec![0.0f32; 2];
    assert!(
        unembedding
            .logits_rows(&normalized, vocabulary - 1, &mut past)
            .is_none()
    );
    assert!(
        unembedding
            .logits_rows(&normalized[..width - 1], 0, &mut single)
            .is_none()
    );
    let _ = std::fs::remove_dir_all(&scratch);
}
