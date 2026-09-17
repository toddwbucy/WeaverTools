//! conforms: spu-nothing-laid-in-for-later-operations
//!
//! The decode submodule, per `weaver-spu-Spec` section 4.
//!
//! **One submodule exists**, and a later operation type is a sibling of this
//! directory in its own process rather than a variant inside it. Nothing here
//! is laid in for one: adding an operation type is adding a directory beside
//! this one, which is the reversibility test passing on this crate's own
//! future.
//!
//! The family library sits above this submodule and the device beneath it. The
//! seam between is `backend.rs`, and the loop that drives it is `session.rs`.

pub mod backend;
pub mod session;

/// The GGUF backend. Its residency half is real: llama.cpp holds the model
/// and dropping the residency frees the device. Its decode half, the
/// [`backend::Backend`] implementation, is the turn path's act, so
/// `backend::for_container` still refuses both containers.
#[cfg(feature = "gguf")]
pub mod gguf;

/// The GGUF path's residual tap, per `weaver-spu-Spec` section 7. Its own
/// module because it is the readout's one unsafe surface: a C callback the
/// ggml scheduler invokes from inside the graph walk, which nothing else in
/// this crate needs and nothing else should have to read around.
#[cfg(feature = "gguf")]
pub mod gguf_tap;

/// The candle-native backend, the second peer of the seam. Its residency
/// half loads safetensors onto the admitted device through the pinned candle
/// fork, and its decode half implements the five primitives over a session's
/// clone of the resident model. One family and one device this stage, per
/// the module's own header.
#[cfg(feature = "cuda")]
pub mod native;

/// The native pair forward, the two-device half of the width election. The
/// scheme is the salvaged `forward_tp2`'s and the code is candle's ops, per
/// the module's own header.
#[cfg(feature = "cuda")]
pub mod native_pair;

/// The long-context rotary property, watched as arithmetic.
///
/// **The claim.** A rotary table entry is `cos(position * inv_freq[j])`. The
/// pinned candle fork carries that whole angle in fp32 and casts only the
/// cosine to the model dtype, per the precision note in
/// `candle-transformers/src/models/qwen2.rs`. Stock upstream casts `inv_freq`
/// and the position vector to the model dtype first and takes the cosine of a
/// bf16 angle, which is upstream pull request 3520, still open and unmerged.
/// bf16 carries eight significant bits, so an integer position stops being
/// exact above 256 and the spacing is 64 across the range 8192 to 16384. On the
/// `j = 0` lane, where `inv_freq` is 1.0 whatever `rope_theta` is, the angle in
/// radians is the position itself, so the stock ordering takes the cosine of a
/// neighbouring representable integer and the answer is uncorrelated with the
/// one asked for. Measured here at position 15962, which rounds to 15936:
/// `cos(15962) = -0.908` against `cos(15936) = -0.268`.
///
/// **What this holds.** The two orderings, reproduced over `half::bf16`, which
/// is the type candle's own BF16 dtype is built on. The test fails when the
/// construction under watch takes the stock ordering and passes when it takes
/// the fork's, and it asserts the gap first so that its tolerance is shown to
/// discriminate rather than assumed to.
///
/// **What this does not hold, and it is the larger half.** It holds the
/// arithmetic rather than the type. `RotaryEmbedding` and its `new` are private
/// to `candle_transformers::models::qwen2`, so nothing outside that crate can
/// call the constructor this property lives in, and reaching it would mean
/// widening the fork rather than watching it. So a repin, a rebase, or a swap
/// to stock candle that reverted the ordering would leave this test green. The
/// same is true of the second copy of this arithmetic in
/// `crate::decoder::native_pair`, which builds its own tables inside the admit
/// and is not reachable from a test either. What the test does buy is that the
/// claim itself is written down and executable, so the ordering cannot be
/// argued about from memory, and step 2 of issue 639, which vendors the model
/// file into this crate, makes the shipped construction directly callable and
/// turns this into a watch on the real path.
#[cfg(test)]
mod rotary_precision {
    use half::bf16;

    /// Qwen2's head dimension and rotary base at the sizes this crate serves.
    /// Neither figure decides the property: `inv_freq[0]` is `theta.powf(0.0)`
    /// inverted, which is 1.0 for every base and every head dimension.
    const HEAD_DIM: usize = 128;
    const ROPE_THETA: f64 = 1_000_000.0;

    /// A position inside the window a served model offers and far outside what
    /// any other test in this crate reaches, the highest of those being a
    /// 256-token generation on the native path.
    const POSITION: usize = 15_962;

    fn inv_freq(head_dim: usize, theta: f64) -> Vec<f32> {
        (0..head_dim)
            .step_by(2)
            .map(|i| 1f32 / theta.powf(i as f64 / head_dim as f64) as f32)
            .collect()
    }

    /// The construction under watch, in the pinned fork's ordering: the angle
    /// is carried in fp32 and the cast lands on the cosine.
    fn cos_entry(position: usize, inv_freq: f32) -> bf16 {
        bf16::from_f32((position as f32 * inv_freq).cos())
    }

    /// Stock upstream's ordering, here so the test can show the two answers
    /// differ at this position rather than resting on a tolerance nobody
    /// checked. The inner product is a rank-one outer product over a
    /// one-element inner dimension, so no accumulation order is being modelled
    /// away.
    fn cos_entry_stock_ordering(position: usize, inv_freq: f32) -> bf16 {
        let angle = bf16::from_f32(
            bf16::from_f32(position as f32).to_f32() * bf16::from_f32(inv_freq).to_f32(),
        );
        bf16::from_f32(angle.to_f32().cos())
    }

    #[test]
    fn the_rotary_angle_survives_a_long_context_position() {
        let inv = inv_freq(HEAD_DIM, ROPE_THETA);
        assert_eq!(
            inv[0], 1.0,
            "the j = 0 lane is the one the property lives on"
        );

        let truth = (POSITION as f64 * inv[0] as f64).cos();
        let stock = cos_entry_stock_ordering(POSITION, inv[0]).to_f64();
        let watched = cos_entry(POSITION, inv[0]).to_f64();

        assert!(
            (stock - truth).abs() > 0.5,
            "the two orderings must disagree at position {POSITION} or this test \
             discriminates nothing: stock {stock}, true {truth}"
        );
        assert!(
            (watched - truth).abs() < 0.01,
            "the rotary angle is built in a precision that cannot hold position \
             {POSITION}: got {watched}, expected {truth}"
        );
    }
}

/// The same property read through candle's own operators at the pinned
/// revision, on the host device, so that a repin changing `matmul`, `cos`, or
/// `to_dtype` numerics is caught alongside a repin changing the ordering. It
/// rides the `cuda` gate because that gate is what declares candle at all, and
/// it touches no card: every tensor here is built on `Device::Cpu`. The
/// construction mirrors the fork's `RotaryEmbedding::new` call for call, and
/// the caveat above stands unchanged - this is the arithmetic, not the type.
#[cfg(all(test, feature = "cuda"))]
mod rotary_precision_through_candle {
    use candle_core::{DType, Device, IndexOp, Tensor};

    const HEAD_DIM: usize = 128;
    const ROPE_THETA: f64 = 1_000_000.0;
    const MAX_SEQ_LEN: usize = 16_384;
    const POSITION: usize = 15_962;

    fn inv_freq_tensor(dev: &Device) -> candle_core::Result<Tensor> {
        let inv: Vec<f32> = (0..HEAD_DIM)
            .step_by(2)
            .map(|i| 1f32 / ROPE_THETA.powf(i as f64 / HEAD_DIM as f64) as f32)
            .collect();
        let len = inv.len();
        Tensor::from_vec(inv, (1, len), dev)
    }

    fn positions(dev: &Device) -> candle_core::Result<Tensor> {
        Tensor::arange(0u32, MAX_SEQ_LEN as u32, dev)?
            .to_dtype(DType::F32)?
            .reshape((MAX_SEQ_LEN, 1))
    }

    /// The construction under watch, in the pinned fork's ordering.
    fn cos_table(dev: &Device) -> candle_core::Result<Tensor> {
        let freqs = positions(dev)?.matmul(&inv_freq_tensor(dev)?)?;
        freqs.cos()?.to_dtype(DType::BF16)
    }

    /// Stock upstream's ordering, for the discriminating half, modelled rather
    /// than mirrored. candle refuses `matmul` on BF16 on the host backend, so
    /// the operands are rounded to bf16 and widened again before the product
    /// and the product is rounded before the cosine. For this shape that is
    /// the same value: the table is a rank-one outer product over a
    /// one-element inner dimension, so there is one multiply and no
    /// accumulation whose order could differ.
    fn cos_table_stock_ordering(dev: &Device) -> candle_core::Result<Tensor> {
        let inv = inv_freq_tensor(dev)?
            .to_dtype(DType::BF16)?
            .to_dtype(DType::F32)?;
        let t = positions(dev)?
            .to_dtype(DType::BF16)?
            .to_dtype(DType::F32)?;
        t.matmul(&inv)?.to_dtype(DType::BF16)?.cos()
    }

    fn entry(table: &Tensor) -> candle_core::Result<f64> {
        table
            .i((POSITION, 0))?
            .to_dtype(DType::F64)?
            .to_scalar::<f64>()
    }

    #[test]
    fn candles_rotary_table_holds_its_angle_at_long_context() -> candle_core::Result<()> {
        let dev = Device::Cpu;
        let truth = (POSITION as f64).cos();
        let stock = entry(&cos_table_stock_ordering(&dev)?)?;
        let watched = entry(&cos_table(&dev)?)?;

        assert!(
            (stock - truth).abs() > 0.5,
            "the two orderings must disagree at position {POSITION} or this test \
             discriminates nothing: stock {stock}, true {truth}"
        );
        assert!(
            (watched - truth).abs() < 0.01,
            "the rotary table is built in a precision that cannot hold position \
             {POSITION}: got {watched}, expected {truth}"
        );
        Ok(())
    }
}
