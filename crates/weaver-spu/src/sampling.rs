//! conforms: spu-disposition-compels-election
//! conforms: spu-knob-set-includes-the-seed
//! conforms: spu-frozen-values-never-cross
//! conforms: spu-effective-values-recorded
//!
//! Sampling and the dispositions, per `weaver-spu-Spec` section 8.
//!
//! **Every knob carries a [`Disposition`], elected at the worker's composition
//! root.** A knob is `Frozen` with its value compiled into the binary, or
//! `OperatorTunable` and routed from the agent's configuration at load. The
//! machinery beneath takes a plain value and never learns which side supplied
//! it, which is what makes the two dispositions cost the same and a change
//! between them one line and a recompile.
//!
//! **A knob left without an election does not compile.** The type carries no
//! third case and no default, so a bare value where a disposition belongs is a
//! type error rather than a silent choice:
//!
//! Perturbation for the first pin: change a field to its bare type, as in
//! `pub temperature: f32`, and this stops failing, because a value with no
//! election then means something. Watched under exactly that change. A `From`
//! impl does not defeat it, since a struct field takes no implicit conversion,
//! which is why the field's own type is what carries the compulsion.
//!
//! ```compile_fail
//! use weaver_spu::sampling::{Disposition, Knobs};
//! // A value with no election. There is no `From<f32>` and no `Default`, so
//! // this cannot mean anything and does not compile.
//! let knobs = Knobs {
//!     temperature: 0.7,
//!     top_k: Disposition::OperatorTunable,
//!     top_p: Disposition::OperatorTunable,
//!     repetition_penalty: Disposition::OperatorTunable,
//!     repetition_window: Disposition::OperatorTunable,
//!     seed: Disposition::OperatorTunable,
//! };
//! ```
//!
//! ```compile_fail
//! use weaver_spu::sampling::Knobs;
//! // Nor is there a default to fall back on.
//! let knobs = Knobs::default();
//! ```
//!
//! Perturbation for the second pin: derive `Default` on [`Knobs`] and on
//! [`Disposition`], which needs a third case to default to, and this stops
//! failing. Watched under exactly that change, and the third case it forces is
//! the point: a default knob set cannot be built without inventing the
//! unelected state the type refuses to carry.
//!
//! **The knob set is a struct literal a doctest reads, and the seed is among
//! it.** Every member rides its own disposition, so a knob added or dropped
//! stops the build rather than being noticed later. Perturbation: drop the
//! `seed` field and this doctest fails on the missing initializer, which is the
//! membership check working. Watched under exactly that removal. The seed is a
//! knob for the
//! first time in this code's lineage: the archived tree carried it as a
//! hardcoded default and a determinism test and never made it configurable, and
//! a frozen seed beside a frozen sampling surface is what makes a binary's
//! declared starting field re-enterable, per apex section 8.
//!
//! ```
//! use weaver_spu::sampling::{Disposition, Knobs};
//! let knobs = Knobs {
//!     temperature: Disposition::Frozen(0.7),
//!     top_k: Disposition::Frozen(40),
//!     top_p: Disposition::Frozen(0.95),
//!     repetition_penalty: Disposition::Frozen(1.1),
//!     repetition_window: Disposition::Frozen(64),
//!     seed: Disposition::Frozen(11),
//! };
//! assert_eq!(knobs.tunable_names(), Vec::<&str>::new());
//! ```

use std::collections::BTreeMap;

/// How a knob's value is supplied, elected at the composition root.
///
/// Two cases and no third. There is no `Default` and no conversion from a bare
/// value, which is what makes the election compulsory rather than encouraged.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Disposition<T> {
    /// Compiled into the binary. The operator cannot move it and the wire never
    /// carries it.
    Frozen(T),
    /// Routed from the agent's configuration at load.
    OperatorTunable,
}

impl<T> Disposition<T> {
    pub fn is_frozen(&self) -> bool {
        matches!(self, Disposition::Frozen(_))
    }
}

/// The knob set: temperature, top-k, top-p, the repetition penalty and its
/// window, and the seed.
///
/// **Membership is what the build checks.** The doctest at the module head
/// reads this literal with every member named, so adding a knob or dropping one
/// stops that test compiling.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Knobs {
    pub temperature: Disposition<f32>,
    pub top_k: Disposition<u32>,
    pub top_p: Disposition<f32>,
    pub repetition_penalty: Disposition<f32>,
    pub repetition_window: Disposition<u32>,
    pub seed: Disposition<u64>,
}

/// What a caller must supply for the knobs this binary left tunable, keyed by
/// name. A tunable knob with no value supplied is a refused decode rather than
/// a default applied, since a default here would be an election this crate made
/// on the operator's behalf.
pub type TunableValues = BTreeMap<String, f64>;

/// The values sampling ran with, whichever side set them.
///
/// **This is what the record holds.** Reported into `model.request`'s payload,
/// so a frozen knob is as visible in the record as a tunable one and a replay
/// reads one list rather than joining two.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EffectiveKnobs {
    pub temperature: f32,
    pub top_k: u32,
    pub top_p: f32,
    pub repetition_penalty: f32,
    pub repetition_window: u32,
    pub seed: u64,
}

/// Why a resolve refused.
#[derive(Debug, Clone, PartialEq)]
pub enum KnobRefusal {
    /// A knob this binary left tunable was not supplied. Named, because an
    /// operator meeting it needs to know which one.
    Unsupplied { knob: &'static str },
}

impl Knobs {
    /// The names of the knobs this binary left tunable.
    ///
    /// **This is the whole of what crosses the token seam**, per Spec section 8
    /// and the decode contract's conformance list. A frozen knob's value is
    /// compiled in and has no reason to travel, and the list being derived from
    /// the dispositions rather than written beside them is what keeps the two
    /// from drifting.
    pub fn tunable_names(&self) -> Vec<&'static str> {
        let mut names = Vec::new();
        if !self.temperature.is_frozen() {
            names.push("temperature");
        }
        if !self.top_k.is_frozen() {
            names.push("top-k");
        }
        if !self.top_p.is_frozen() {
            names.push("top-p");
        }
        if !self.repetition_penalty.is_frozen() {
            names.push("repetition-penalty");
        }
        if !self.repetition_window.is_frozen() {
            names.push("repetition-window");
        }
        if !self.seed.is_frozen() {
            names.push("seed");
        }
        names
    }

    /// Resolve the elections against what the operator supplied.
    ///
    /// A frozen knob takes its compiled value and ignores anything supplied
    /// under its name, which is what frozen means: a supplied value for a
    /// frozen knob is not an error to report but a fact with no effect, because
    /// the wire never carried it in the first place.
    pub fn resolve(&self, supplied: &TunableValues) -> Result<EffectiveKnobs, KnobRefusal> {
        fn take<T: Copy>(
            disposition: &Disposition<T>,
            name: &'static str,
            supplied: &TunableValues,
            convert: impl Fn(f64) -> T,
        ) -> Result<T, KnobRefusal> {
            match disposition {
                Disposition::Frozen(value) => Ok(*value),
                Disposition::OperatorTunable => supplied
                    .get(name)
                    .map(|value| convert(*value))
                    .ok_or(KnobRefusal::Unsupplied { knob: name }),
            }
        }
        Ok(EffectiveKnobs {
            temperature: take(&self.temperature, "temperature", supplied, |v| v as f32)?,
            top_k: take(&self.top_k, "top-k", supplied, |v| v as u32)?,
            top_p: take(&self.top_p, "top-p", supplied, |v| v as f32)?,
            repetition_penalty: take(
                &self.repetition_penalty,
                "repetition-penalty",
                supplied,
                |v| v as f32,
            )?,
            repetition_window: take(
                &self.repetition_window,
                "repetition-window",
                supplied,
                |v| v as u32,
            )?,
            seed: take(&self.seed, "seed", supplied, |v| v as u64)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_frozen() -> Knobs {
        Knobs {
            temperature: Disposition::Frozen(0.7),
            top_k: Disposition::Frozen(40),
            top_p: Disposition::Frozen(0.95),
            repetition_penalty: Disposition::Frozen(1.1),
            repetition_window: Disposition::Frozen(64),
            seed: Disposition::Frozen(11),
        }
    }

    /// **Frozen values never cross the wire.** Only the operator-tunable
    /// remainder travels on the token seam, and the list that says which is
    /// derived from the elections rather than maintained beside them.
    ///
    /// Perturbation: make `tunable_names` push a name unconditionally for any
    /// knob, or return every name, and this test fails on the all-frozen case.
    /// Watched under exactly that change.
    #[test]
    fn a_frozen_knob_is_not_on_the_wire() {
        assert_eq!(all_frozen().tunable_names(), Vec::<&str>::new());

        let mut knobs = all_frozen();
        knobs.seed = Disposition::OperatorTunable;
        knobs.top_p = Disposition::OperatorTunable;
        assert_eq!(knobs.tunable_names(), vec!["top-p", "seed"]);
    }

    /// **The effective values are what the record holds, whichever side set
    /// them.** A frozen knob resolves to its compiled value and a tunable one
    /// to what was supplied, and the result carries all six either way, so a
    /// replay reads one list rather than joining two.
    #[test]
    fn the_effective_values_carry_every_knob_from_either_side() {
        let mut knobs = all_frozen();
        knobs.temperature = Disposition::OperatorTunable;
        let mut supplied = TunableValues::new();
        supplied.insert("temperature".into(), 0.2);

        let effective = knobs.resolve(&supplied).expect("every tunable supplied");
        assert_eq!(effective.temperature, 0.2, "the tunable came from the wire");
        assert_eq!(effective.seed, 11, "and the frozen one from the binary");
        assert_eq!(effective.top_k, 40);
    }

    /// A frozen knob ignores a value supplied under its name. Frozen means the
    /// wire never carried it, so a value arriving anyway is a fact with no
    /// effect rather than a conflict to adjudicate.
    #[test]
    fn a_supplied_value_cannot_move_a_frozen_knob() {
        let mut supplied = TunableValues::new();
        supplied.insert("seed".into(), 999.0);
        let effective = all_frozen().resolve(&supplied).expect("nothing is tunable");
        assert_eq!(effective.seed, 11, "the compiled value stands");
    }

    /// **A tunable knob with no value refuses rather than defaulting**, naming
    /// the knob. A default here would be an election this crate made on the
    /// operator's behalf, which is the thing the disposition type exists to
    /// prevent.
    ///
    /// Perturbation: give the `OperatorTunable` arm of `take` a fallback value
    /// and this test fails. Watched under exactly that change.
    #[test]
    fn a_tunable_knob_with_no_value_refuses_by_name() {
        let mut knobs = all_frozen();
        knobs.repetition_window = Disposition::OperatorTunable;
        assert_eq!(
            knobs.resolve(&TunableValues::new()),
            Err(KnobRefusal::Unsupplied {
                knob: "repetition-window"
            })
        );
    }
}
