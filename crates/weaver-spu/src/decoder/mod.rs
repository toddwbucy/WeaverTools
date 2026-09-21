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

/// The GGUF backend. llama.cpp holds the resident model and the engine
/// implements the decode primitives. [`backend::for_container`] accepts GGUF
/// when this feature is built, and dropping the residency frees the device.
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
/// clone of the resident model. Qwen2 serves on one device through the fork
/// and on a pair through [`native_pair`].
#[cfg(feature = "cuda")]
pub mod native;

/// The native pair forward, the two-device half of the width election. The
/// scheme is the salvaged `forward_tp2`'s and the code is candle's ops, per
/// the module's own header.
#[cfg(feature = "cuda")]
pub mod native_pair;

/// The inverse frequencies used by the pair's rotary table construction.
#[cfg(any(test, feature = "cuda"))]
fn rotary_inverse_frequencies(head_dim: usize, theta: f64) -> Vec<f32> {
    (0..head_dim)
        .step_by(2)
        .map(|i| 1f32 / theta.powf(i as f64 / head_dim as f64) as f32)
        .collect()
}

/// Host arithmetic diagnostics for both rotary functions. The angle stays
/// fp32 until after sine or cosine, matching the pinned qwen2 fork's fix
/// proposed in upstream candle PR 3520. BF16-first ordering rounds integer
/// positions above 256: 15962 rounds to 15936, whose cosine is -0.267950,
/// or -0.267578 after rounding the output to BF16.
///
/// These scalar diagnostics do not call Candle's private RotaryEmbedding
/// constructor. Changing only that constructor's ordering leaves them green.
/// The CUDA-gated tests below watch this crate's production pair builder.
#[cfg(test)]
mod rotary_precision {
    use half::bf16;

    // One frequency lane suffices to expose position rounding. The builder
    // uses its ordinary head_dim / 2 shape without allocating 64 unused lanes.
    pub(super) const HEAD_DIM: usize = 2;
    pub(super) const ROPE_THETA: f64 = 1_000_000.0;
    // Include the first inexact BF16 position, the 1024/4096 fixture edges,
    // and a larger context position available to artifacts with that capacity.
    pub(super) const POSITIONS: [usize; 4] = [257, 1023, 4095, 15_962];

    /// Scalar diagnostic in cosine/sine order, casting only after the trig operation.
    pub(super) fn entries(position: usize, inv: f32) -> [bf16; 2] {
        let angle = position as f32 * inv;
        [bf16::from_f32(angle.cos()), bf16::from_f32(angle.sin())]
    }

    /// BF16-first reference, including rounding the product before either trig operation.
    pub(super) fn stock_entries(position: usize, inv: f32) -> [bf16; 2] {
        let angle =
            bf16::from_f32(bf16::from_f32(position as f32).to_f32() * bf16::from_f32(inv).to_f32())
                .to_f32();
        [bf16::from_f32(angle.cos()), bf16::from_f32(angle.sin())]
    }

    /// Require each reference to discriminate before testing against the fp64 angle.
    pub(super) fn check(position: usize, inv: f32, watched: [f64; 2], stock: [f64; 2]) {
        let angle = position as f64 * inv as f64;
        for (name, actual, rounded, truth) in [
            ("cosine", watched[0], stock[0], angle.cos()),
            ("sine", watched[1], stock[1], angle.sin()),
        ] {
            // Each gap must exceed forty times the accuracy tolerance.
            assert!(
                (rounded - truth).abs() > 0.4,
                "{name} does not discriminate at {position}: stock {rounded}, true {truth}"
            );
            assert!(
                (actual - truth).abs() < 0.01,
                "{name} lost the rotary angle at {position}: got {actual}, expected {truth}"
            );
        }
    }

    /// Exercise both trig functions at the first rounding boundary and larger positions.
    #[test]
    fn the_rotary_angle_survives_context_boundaries() {
        let inv = super::rotary_inverse_frequencies(HEAD_DIM, ROPE_THETA);
        assert_eq!(
            inv[0], 1.0,
            "the zero frequency lane has unit inverse frequency"
        );
        for position in POSITIONS {
            check(
                position,
                inv[0],
                entries(position, inv[0]).map(bf16::to_f64),
                stock_entries(position, inv[0]).map(bf16::to_f64),
            );
        }
    }
}

/// Watch the production pair builder through Candle operators on Device::Cpu.
/// Both sine and cosine feed ShardedModel::load and attend's rope operation.
/// This detects changes to that builder or Candle operator numerics, but not
/// changes confined to the fork's private single-device RotaryEmbedding.
/// The cuda feature requires a CUDA toolkit at build time even though these
/// tests use only the CPU. They provide no device-execution evidence.
#[cfg(all(test, feature = "cuda"))]
mod rotary_precision_through_candle {
    use super::rotary_precision::{HEAD_DIM, POSITIONS, ROPE_THETA, check, stock_entries};
    use candle_core::{DType, Device, IndexOp, Tensor};

    /// Widen one stored BF16 entry without changing the value being measured.
    fn entry(table: &Tensor, position: usize) -> candle_core::Result<f64> {
        table
            .i((position, 0))?
            .to_dtype(DType::F64)?
            .to_scalar::<f64>()
    }

    /// Read the pair loader's production tables and cross-check both stock references.
    #[test]
    fn the_production_rotary_tables_hold_their_angles() -> candle_core::Result<()> {
        let dev = Device::Cpu;
        let max = POSITIONS[POSITIONS.len() - 1] + 1;
        let inv = super::rotary_inverse_frequencies(HEAD_DIM, ROPE_THETA);
        assert_eq!(
            inv[0], 1.0,
            "the truth uses the builder's unit inverse frequency"
        );
        let (cos, sin) = super::native_pair::rotary_tables(HEAD_DIM, ROPE_THETA, max, &dev, &dev)?;
        assert_eq!(cos.dims(), &[max, HEAD_DIM / 2]);
        assert_eq!(sin.dims(), cos.dims());
        assert_eq!(cos.dtype(), DType::BF16);
        assert_eq!(sin.dtype(), DType::BF16);

        // CPU matmul does not support BF16. Round each operand and the
        // product through BF16 to model that ordering for a one-term product.
        let inv = Tensor::from_vec(inv, (1, HEAD_DIM / 2), &dev)?
            .to_dtype(DType::BF16)?
            .to_dtype(DType::F32)?;
        let positions = Tensor::arange(0u32, max as u32, &dev)?
            .to_dtype(DType::BF16)?
            .to_dtype(DType::F32)?
            .reshape((max, 1))?;
        let rounded = positions.matmul(&inv)?.to_dtype(DType::BF16)?;
        let stock_cos = rounded.cos()?;
        let stock_sin = rounded.sin()?;
        for position in POSITIONS {
            let stock = [entry(&stock_cos, position)?, entry(&stock_sin, position)?];
            assert_eq!(
                stock,
                stock_entries(position, 1.0).map(half::bf16::to_f64),
                "Candle and half agree on both BF16-first outputs at {position}"
            );
            check(
                position,
                1.0,
                [entry(&cos, position)?, entry(&sin, position)?],
                stock,
            );
        }
        Ok(())
    }
}
