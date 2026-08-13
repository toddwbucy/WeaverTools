//! conforms: spu-reduction-in-place-at-the-tap
//! conforms: spu-tap-failure-is-a-fault
//!
//! Residual readout, per `weaver-spu-Spec` section 7.
//!
//! **The reduction happens at the tap, before anything leaves the device.**
//! Apex section 3 step 6 has the activations reduced in place and the reduction
//! returning by the same path as the generation, so no per-layer tensor crosses
//! the seam and the volume the seam carries is the reduction's. That is stated
//! here as a type rather than as a discipline: [`Tap::observe`] is handed a
//! layer's activations by reference and answers nothing, accumulating into the
//! reduction it owns, and the only thing that leaves is [`Reduction`]. There is
//! no path on this seam that returns a per-layer tensor, so a caller cannot
//! carry one off the device by mistake.
//!
//! **One shape, whichever backend produced it.** The native path uses the
//! candle fork's `forward_with_intermediates` and the GGUF path the ggml
//! scheduler's eval callback the llama-cpp fork exposes. Both drive this seam
//! and both answer [`Reduction`], so a consumer reads one shape and cannot tell
//! which engine produced it. **Neither tap is stood up**, because neither
//! backend is: `decoder::backend::for_container` refuses for both containers
//! today, and the Spec is explicit that standing the GGUF tap up is code this
//! program writes rather than salvage it inherits. What is here is the seam
//! they will drive and the reduction they will fill.
//!
//! **A tap that fails while elected is a fault, not an absence.** Per charter
//! section 13.10: an elected observability that silently stopped observing is a
//! record that reads as a run without readout rather than a run whose readout
//! broke. [`TapOutcome`] carries no case for a quiet nothing, which is what
//! makes the distinction unrepresentable rather than merely discouraged.

use crate::family::Declaration;

/// Whether the operator elected residual readout for this load.
///
/// A newtype rather than a bare `bool` because it travels beside other flags on
/// the admit path, and two booleans in a row is the argument order nobody
/// remembers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadoutElection(pub bool);

impl ReadoutElection {
    pub fn elected(&self) -> bool {
        self.0
    }
}

/// What a tap answers with, per layer.
///
/// **There is no case for a quiet absence.** A tap either observed or it
/// faulted, and the caller cannot record "nothing happened" without saying
/// which. That is the whole of section 7's fault-not-absence claim expressed as
/// a type.
#[derive(Debug, Clone, PartialEq)]
pub enum TapOutcome {
    /// The layer was observed and folded into the reduction.
    Observed,
    /// The tap failed while elected. Carries what failed, because a fault the
    /// record cannot name is a fault the operator cannot act on.
    Faulted { detail: String },
}

/// Why a readout election could not be honored.
#[derive(Debug, Clone, PartialEq)]
pub enum ReadoutRefusal {
    /// The election was made and this family's engine cannot tap.
    ///
    /// **Refused at admit rather than at the first turn**, per the charter's
    /// fail-cheap-or-lie-expensive rule. A load that succeeded and then failed
    /// on the first turn has already cost the operator the load.
    NotTappable { family: &'static str },
}

/// The reduction a tap accumulates, and the only thing that leaves the device.
///
/// **In place.** The tap folds each layer into the running figures and never
/// retains the layer, so the memory this holds is a function of the layer count
/// rather than of the activation width, and nothing here can be widened into a
/// per-layer tensor without changing the type.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Reduction {
    /// One figure per layer observed, in layer order.
    per_layer_norm: Vec<f32>,
}

impl Reduction {
    pub fn new() -> Self {
        Reduction::default()
    }

    /// Fold one layer in, keeping the figure and dropping the activations.
    ///
    /// The caller's slice is borrowed and not retained, which is what "in
    /// place" means here: this function is the only way into the reduction and
    /// it keeps a scalar per layer.
    pub fn fold(&mut self, activations: &[f32]) {
        let sum_of_squares: f32 = activations.iter().map(|value| value * value).sum();
        self.per_layer_norm.push(sum_of_squares.sqrt());
    }

    pub fn layers(&self) -> usize {
        self.per_layer_norm.len()
    }

    pub fn per_layer_norm(&self) -> &[f32] {
        &self.per_layer_norm
    }
}

/// The seam both taps drive.
///
/// **`observe` answers no tensor.** It is handed a borrowed layer and folds it,
/// so the per-layer activations have no route off the device through this
/// trait. `finish` yields the reduction, which is the volume the seam carries.
pub trait Tap {
    /// Fold one layer's activations in, at the tap.
    fn observe(&mut self, layer: usize, activations: &[f32]) -> TapOutcome;

    /// Hand back the reduction and end the tap.
    fn finish(self: Box<Self>) -> Reduction;
}

/// Judge a readout election against what the family declares, at admit.
///
/// **This runs on the admit path and nowhere else.** Moving it later is what
/// section 10's watch perturbs: the load then succeeds and the first turn
/// fails, which is the expensive lie the charter's rule forbids.
pub fn judge(election: ReadoutElection, declaration: &Declaration) -> Result<(), ReadoutRefusal> {
    if election.elected() && !declaration.taps_readout {
        return Err(ReadoutRefusal::NotTappable {
            family: declaration.family,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::backend::FlushMechanism;

    const TAPPABLE: Declaration = Declaration {
        family: "tappable",
        shard_widths: &[1],
        template: "{message}",
        generation_opener: "",
        flush: FlushMechanism::TruncateToPosition,
        taps_readout: true,
    };

    const UNTAPPABLE: Declaration = Declaration {
        family: "untappable",
        shard_widths: &[1],
        template: "{message}",
        generation_opener: "",
        flush: FlushMechanism::TruncateToPosition,
        taps_readout: false,
    };

    /// **An election a family cannot honor refuses at admit**, naming the
    /// family, rather than loading and failing at the first turn.
    ///
    /// Perturbation: section 10's watch is the check moving off the admit path,
    /// under which the load succeeds and the first turn fails. Watched at the
    /// admit path in `residency.rs`, where the ordering is observable; what
    /// this test buys is the judgment itself.
    #[test]
    fn an_election_the_family_cannot_honor_is_refused() {
        assert_eq!(
            judge(ReadoutElection(true), &UNTAPPABLE),
            Err(ReadoutRefusal::NotTappable {
                family: "untappable"
            })
        );
        assert_eq!(judge(ReadoutElection(true), &TAPPABLE), Ok(()));
    }

    /// **No shipped family advertises a tap, because nothing implements one.**
    ///
    /// This is a tripwire rather than a preference. `Tap` has no implementation
    /// in this crate, so a family declaring `taps_readout: true` would be
    /// admitted under an election nothing could honor. The day a backend stands
    /// its tap up, that family's declaration flips and this test is edited in
    /// the same act, which is what makes the flip deliberate rather than
    /// incidental.
    #[test]
    fn no_shipped_family_advertises_a_tap_it_cannot_perform() {
        for declaration in crate::family::REGISTRY {
            assert!(
                !declaration.taps_readout,
                "{} advertises a tap, and no Tap implementation exists",
                declaration.family
            );
        }
    }

    /// **The refusal is reachable against the shipped registry**, not only
    /// against a fixture.
    ///
    /// A judgment whose refusing branch only ever fires for a test-local
    /// declaration proves the function and says nothing about the binary. This
    /// walks every family this binary actually carries.
    #[test]
    fn an_elected_readout_refuses_against_every_shipped_family() {
        for declaration in crate::family::REGISTRY {
            assert_eq!(
                judge(ReadoutElection(true), declaration),
                Err(ReadoutRefusal::NotTappable {
                    family: declaration.family
                }),
                "{} cannot honor an election today",
                declaration.family
            );
            assert_eq!(
                judge(ReadoutElection(false), declaration),
                Ok(()),
                "{} serves an unelected load",
                declaration.family
            );
        }
    }

    /// **No election, no refusal.** A family that cannot tap serves every load
    /// that did not ask it to, which is what keeps the capability from becoming
    /// a requirement.
    #[test]
    fn an_unelected_load_is_served_by_an_untappable_family() {
        assert_eq!(judge(ReadoutElection(false), &UNTAPPABLE), Ok(()));
    }

    /// **The reduction keeps a figure per layer and never the layer.** What
    /// leaves is a function of the layer count, not the activation width.
    #[test]
    fn the_reduction_holds_a_figure_per_layer_rather_than_the_layers() {
        let mut reduction = Reduction::new();
        // Two layers, wildly different widths. What comes out is two numbers.
        reduction.fold(&[3.0, 4.0]);
        reduction.fold(&vec![1.0; 4096]);

        assert_eq!(reduction.layers(), 2);
        assert_eq!(reduction.per_layer_norm().len(), 2);
        assert!((reduction.per_layer_norm()[0] - 5.0).abs() < 1e-5);
        assert!((reduction.per_layer_norm()[1] - 64.0).abs() < 1e-3);
    }

    /// **A tap outcome cannot say "nothing happened".** The enum carries two
    /// cases and a failure has to name itself, so an elected readout that
    /// stopped observing cannot be recorded as a run without readout.
    #[test]
    fn a_failed_tap_names_itself_rather_than_reporting_an_absence() {
        let faulted = TapOutcome::Faulted {
            detail: "the eval callback returned no tensor".into(),
        };
        match faulted {
            TapOutcome::Faulted { detail } => assert!(!detail.is_empty()),
            TapOutcome::Observed => panic!("this arm is the only other one"),
        }
    }
}
