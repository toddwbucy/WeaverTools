//! conforms: spu-log-sum-exp-stable
//! conforms: spu-absent-not-empty-vector
//! conforms: spu-absent-shape-pinned-by-doctest
//! conforms: spu-prompt-partition-covers-exactly
//! conforms: spu-nothing-retained-after-the-answer
//!
//! Measurement production, per `weaver-spu-Spec` section 6.
//!
//! **The signals are computed where the distribution is.** Entropy over the
//! logits at each step and the surprisal of the token that was drawn, both in
//! bits. The distribution exists inside the generation loop and nowhere else,
//! so the arithmetic lives here and the loop in [`crate::decoder::session`]
//! calls it before the sampler runs.
//!
//! **The math is log-sum-exp stable and the reason is arithmetic rather than
//! taste.** A naive exponential overflows `f32` above a logit around 88, which
//! the archived tree records as happening routinely on production
//! vocabularies, so the stable form is the only correct one at the scale this
//! runs at. Everything here goes through [`log_sum_exp`], which subtracts the
//! maximum before exponentiating and therefore never evaluates `exp` on a
//! positive argument.
//!
//! **The absence takes a type instead of a fixture.** A vector that was not
//! produced is `None`, never an empty vector, and the shape makes the
//! distinction unrepresentable rather than merely unproduced: the payload is a
//! [`NonEmpty`], which is a head and a tail and cannot be built empty. The
//! doctest below reads the field's type, so a change to `Vec<f32>` stops the
//! build rather than reintroducing the ambiguity quietly.
//!
//! ```
//! use weaver_spu::measurement::{NonEmpty, Signals};
//! // Absent is a case of the option, not a length of the vector.
//! let nothing = Signals::absent();
//! assert!(nothing.entropy_bits.is_none());
//!
//! // And what is present cannot be empty: there is no constructor that
//! // yields a zero-length payload.
//! let present: Option<NonEmpty<f32>> = NonEmpty::from_vec(vec![1.5, 2.0]);
//! assert_eq!(present.expect("two values").len(), 2);
//! assert!(NonEmpty::from_vec(Vec::<f32>::new()).is_none());
//! ```
//!
//! ```compile_fail
//! use weaver_spu::measurement::Signals;
//! // The field is not a bare vector, so an empty one cannot stand for absence.
//! let signals = Signals {
//!     entropy_bits: Some(Vec::<f32>::new()),
//!     surprisal_bits: None,
//! };
//! ```

use std::f32::consts::LN_2;

/// A vector that cannot be empty, as a head and a tail.
///
/// **This is what makes "absent" and "measured nothing" different states.** An
/// `Option<NonEmpty<T>>` has no representation for a present-but-empty vector,
/// so the trace's absence carries a meaning no length can imitate.
#[derive(Debug, Clone, PartialEq)]
pub struct NonEmpty<T> {
    head: T,
    tail: Vec<T>,
}

impl<T> NonEmpty<T> {
    /// One value, which is the smallest a non-empty vector can be.
    pub fn new(head: T) -> Self {
        NonEmpty {
            head,
            tail: Vec::new(),
        }
    }

    /// Build from a vector, answering `None` where it was empty. This is the
    /// only path from a possibly-empty collection, so the emptiness is decided
    /// once, here, rather than at each reader.
    pub fn from_vec(mut values: Vec<T>) -> Option<Self> {
        if values.is_empty() {
            return None;
        }
        let head = values.remove(0);
        Some(NonEmpty { head, tail: values })
    }

    pub fn push(&mut self, value: T) {
        self.tail.push(value);
    }

    /// Always at least one.
    pub fn len(&self) -> usize {
        1 + self.tail.len()
    }

    /// Never true. Present so a reader reaching for the usual pair finds the
    /// answer stated rather than the method missing.
    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn first(&self) -> &T {
        &self.head
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.head).chain(self.tail.iter())
    }
}

/// The two signal vectors, positionally paired with the tokens drawn.
///
/// **This crate produces no empty vector to mean nothing was measured.** It
/// produces no vector at all, and the trace's own `skip_serializing_if` carries
/// the absence to the record. The serialization half is `weaver-trace-Spec`'s;
/// this is the production half.
#[derive(Debug, Clone, PartialEq)]
pub struct Signals {
    /// Entropy of the distribution at each step, in bits.
    pub entropy_bits: Option<NonEmpty<f32>>,
    /// Surprisal of the token drawn at each step, in bits.
    pub surprisal_bits: Option<NonEmpty<f32>>,
}

impl Signals {
    /// Nothing was measured. Distinct from having measured an empty run.
    pub fn absent() -> Self {
        Signals {
            entropy_bits: None,
            surprisal_bits: None,
        }
    }

    /// How many steps were measured, zero where absent.
    pub fn steps(&self) -> usize {
        self.entropy_bits.as_ref().map_or(0, NonEmpty::len)
    }
}

/// Accumulates one generation's signals, then answers with the pair.
///
/// **Nothing is retained after the answer**, per Spec section 6 and charter
/// section 3's no-state rule. [`Self::finish`] consumes the accumulator, so a
/// second generation cannot reach the first's measurements through it: there is
/// no accumulator left to reach.
#[derive(Debug)]
pub struct Accumulator {
    entropy: Vec<f32>,
    surprisal: Vec<f32>,
    /// **Cleared the moment a retained token could not be measured.** The
    /// vectors are positionally paired with the tokens the generation answers
    /// with, so a gap is not a shorter vector: it is a vector whose every
    /// entry past the gap names the wrong token. An incomplete run reports
    /// absent instead, which is a fact a reader can act on.
    complete: bool,
}

impl Default for Accumulator {
    fn default() -> Self {
        Accumulator {
            entropy: Vec::new(),
            surprisal: Vec::new(),
            complete: true,
        }
    }
}

impl Accumulator {
    pub fn new() -> Self {
        Accumulator::default()
    }

    /// Record one step, from the distribution the sampler has not yet seen.
    pub fn record(&mut self, entropy_bits: f32, surprisal_bits: f32) {
        self.entropy.push(entropy_bits);
        self.surprisal.push(surprisal_bits);
    }

    /// Abandon the measurement: a retained token could not be measured, so the
    /// pairing is broken and the whole run reports absent.
    ///
    /// **Not a shorter vector.** Dropping one entry silently shifts every
    /// later one onto the wrong token, which is a misattribution a consumer
    /// cannot detect. Absence is detectable.
    pub fn abandon(&mut self) {
        self.complete = false;
    }

    /// Answer with the pair and consume the accumulator.
    ///
    /// A generation that produced no token answers absent on both, which is
    /// the case `NonEmpty::from_vec` turns into `None` rather than into a
    /// zero-length vector. A generation that lost a step answers absent too,
    /// for the pairing reason above.
    pub fn finish(self) -> Signals {
        if !self.complete {
            return Signals::absent();
        }
        Signals {
            entropy_bits: NonEmpty::from_vec(self.entropy),
            surprisal_bits: NonEmpty::from_vec(self.surprisal),
        }
    }
}

/// The stable normaliser, as the pair everything here works from: the maximum
/// logit and the log-sum-exp of the logits shifted by it.
///
/// **Nothing downstream ever touches a raw logit again**, and that is the whole
/// of the stability. Working from an unshifted `lse` and subtracting raw logits
/// from it looks equivalent and is not: at large magnitudes `lse - logit`
/// cancels to zero and `probability * logit` overflows to infinity, so two
/// equal logits at `f32::MAX` report no entropy and no surprise where the
/// answer is one bit each. Measured before this shape was chosen.
///
/// `None` where the maximum is not finite, which is the empty slice and the
/// all-infinite one, and `None` where the sum is not a positive finite
/// number.
///
/// **The second guard is not implied by the first.** `f32::max` answers the
/// other operand when one is `NaN`, so a vector carrying a `NaN` beside real
/// logits has a finite maximum and a `NaN` sum, and `ln` of that is `NaN`.
/// Without the guard every reading taken from it is `NaN`: an entropy, a
/// surprisal, and every probability of the field. **Absent is the honest
/// answer and `NaN` is not**, per the absent-rather-than-zero rule this
/// module keeps - the caller abandons the run's signals rather than
/// recording a number that means nothing. A healthy forward pass produces no
/// such vector, which is why this guards a degraded device rather than an
/// ordinary path.
fn shifted_normaliser(logits: &[f32]) -> Option<(f32, f32)> {
    let max = logits.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    if !max.is_finite() {
        return None;
    }
    // Every argument to `exp` is at most zero, so nothing overflows however
    // large the logits are, and the maximum's own term is exactly one, so a
    // sum over real logits is at least one and never zero.
    let sum: f32 = logits.iter().map(|logit| (logit - max).exp()).sum();
    if !sum.is_finite() || sum <= 0.0 {
        return None;
    }
    Some((max, sum.ln()))
}

/// The stable log-sum-exp of a logit vector.
///
/// **Subtracting the maximum is the whole of the stability.** Every argument to
/// `exp` is then at most zero, so nothing overflows however large the logits
/// are, and the shift is added back afterwards. A naive `ln(sum(exp(l)))`
/// overflows `f32` above roughly 88, which is a value production vocabularies
/// reach.
///
/// An empty slice answers negative infinity, which is the identity for this
/// operation, though no caller here supplies one.
pub fn log_sum_exp(logits: &[f32]) -> f32 {
    match shifted_normaliser(logits) {
        Some((max, shifted)) => max + shifted,
        None => logits.iter().copied().fold(f32::NEG_INFINITY, f32::max),
    }
}

/// Entropy of the distribution the logits describe, in bits.
///
/// Computed entirely in shifted space, so no probability is materialised
/// against a raw logit and nothing overflows on the way.
pub fn entropy_bits(logits: &[f32]) -> f32 {
    let Some((max, shifted_lse)) = shifted_normaliser(logits) else {
        return 0.0;
    };
    // H = -sum(p * ln p), with ln p taken in shifted space so it is a small
    // normalised number rather than the difference of two large ones.
    let mut nats = 0.0f32;
    for &logit in logits {
        let log_probability = (logit - max) - shifted_lse;
        nats -= log_probability.exp() * log_probability;
    }
    (nats / LN_2).max(0.0)
}

/// The probability field at one position: the `depth` most probable
/// candidates in descending order with their probabilities, and the rank
/// the drawn token landed on.
///
/// Per `weaver-spu-Spec` section 7.5, computed from the same logits the
/// entropy and the surprisal are computed from, at the same site and
/// before the sampler consumes the distribution.
///
/// **A partial selection rather than a sort.** The depth is small against
/// a vocabulary in the hundreds of thousands, so the whole vector is
/// scanned once to find the depth's worth of leaders and only those are
/// ordered. Sorting the vocabulary would cost the run for a result the
/// caller discards.
///
/// **Probabilities rather than logits**, in shifted space for the reason
/// the entropy is: a consumer re-normalising per position would repeat
/// arithmetic already done over the whole vocabulary, and a raw logit
/// tells a reader nothing without the normaliser beside it.
///
/// `None` where the distribution carries no finite normaliser or the drawn
/// index is outside it, which is the absent-rather-than-zero rule the
/// signals follow.
pub fn field(logits: &[f32], drawn: usize, depth: usize) -> Option<(Vec<(u32, f32)>, u32)> {
    let (max, shifted_lse) = shifted_normaliser(logits)?;
    if drawn >= logits.len() || depth == 0 {
        return None;
    }
    let probability = |logit: f32| ((logit - max) - shifted_lse).exp();

    // One pass for the leaders, holding at most `depth` of them. The list
    // stays ordered as it fills, so the insert is a walk over a short
    // vector rather than a sort of a long one.
    // **The capacity is the depth or the vocabulary, whichever is smaller.**
    // A vector can hold no more leaders than there are logits, and the depth
    // is the operator's declared number with a floor at the sampling cutoff
    // and no ceiling anywhere, so a declaration's stray digit would
    // otherwise reserve that many slots before the first comparison. The
    // depth itself stays whole below, where an out-of-depth draw is reported
    // against the number that was declared rather than the number reserved.
    let mut leaders: Vec<(u32, f32)> = Vec::with_capacity(depth.min(logits.len()) + 1);
    for (index, &logit) in logits.iter().enumerate() {
        if leaders.len() == depth && logit <= leaders[depth - 1].1 {
            continue;
        }
        let at = leaders
            .iter()
            .position(|(_, held)| logit > *held)
            .unwrap_or(leaders.len());
        leaders.insert(at, (index as u32, logit));
        leaders.truncate(depth);
    }

    // The realized rank is the drawn token's place among the leaders. A
    // draw outside them is reported as the depth itself, which reads as
    // "past what was reported" and never as a rank that was observed.
    let realized = leaders
        .iter()
        .position(|(token, _)| *token as usize == drawn)
        .unwrap_or(depth) as u32;
    let ranked = leaders
        .into_iter()
        .map(|(token, logit)| (token, probability(logit)))
        .collect();
    Some((ranked, realized))
}

/// Surprisal of one token under the distribution, in bits.
///
/// `-log2 p` for the drawn token, taken in shifted space for the same reason
/// the entropy is: an unshifted `lse - logit` cancels at large magnitudes and
/// answers no surprise for a token that carried some.
pub fn surprisal_bits(logits: &[f32], index: usize) -> Option<f32> {
    let logit = *logits.get(index)?;
    let (max, shifted_lse) = shifted_normaliser(logits)?;
    Some(((shifted_lse - (logit - max)) / LN_2).max(0.0))
}

/// Why a partition was rejected.
#[derive(Debug, Clone, PartialEq)]
pub enum PartitionDefect {
    /// The offsets do not reach the end of the rendered text, so a consumer
    /// joining text to tokens would have a tail to guess at.
    ShortOfEnd { last: usize, text_len: usize },
    /// The offsets run backwards somewhere, so they do not partition anything.
    NotAscending { at: usize },
    /// No offsets at all for a non-empty text.
    Empty,
}

/// The prompt's block partition, from the tokenizer's offsets.
///
/// **The invariant is that the last offset equals the rendered text's length**,
/// which is what makes the partition cover the prompt exactly. The salvaged
/// `tokenize_with_offsets` mechanic carries it, and it is checked here rather
/// than assumed, because a consumer joining text to tokens with a gap at the
/// end has no way to tell a short partition from a short prompt.
#[derive(Debug, Clone, PartialEq)]
pub struct PromptPartition {
    offsets: Vec<usize>,
    text_len: usize,
}

impl PromptPartition {
    /// Build and check. The offsets are the end position of each token in the
    /// rendered text.
    pub fn new(offsets: Vec<usize>, text_len: usize) -> Result<Self, PartitionDefect> {
        let Some(&last) = offsets.last() else {
            return if text_len == 0 {
                Ok(PromptPartition {
                    offsets,
                    text_len: 0,
                })
            } else {
                Err(PartitionDefect::Empty)
            };
        };
        for window in offsets.windows(2).enumerate() {
            let (at, pair) = window;
            if pair[1] < pair[0] {
                return Err(PartitionDefect::NotAscending { at: at + 1 });
            }
        }
        if last != text_len {
            return Err(PartitionDefect::ShortOfEnd { last, text_len });
        }
        Ok(PromptPartition { offsets, text_len })
    }

    pub fn offsets(&self) -> &[usize] {
        &self.offsets
    }

    pub fn text_len(&self) -> usize {
        self.text_len
    }

    /// How many blocks the partition holds.
    pub fn blocks(&self) -> usize {
        self.offsets.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The stable form survives logits a naive exponential cannot.**
    ///
    /// At 100 a naive `exp` is already infinite in `f32`, so `ln(sum(exp(l)))`
    /// answers infinity and every signal downstream is a NaN. The stable form
    /// answers a finite number, which is what section 6 says it must.
    #[test]
    fn a_logit_past_the_overflow_threshold_still_measures() {
        // Above the roughly-88 threshold section 6 names.
        let logits = [100.0f32, 99.0, 98.0];

        assert!(
            (100.0f32).exp().is_infinite(),
            "the naive form really does overflow here, which is why this matters"
        );

        let lse = log_sum_exp(&logits);
        assert!(
            lse.is_finite(),
            "the stable normaliser is finite, got {lse}"
        );

        let entropy = entropy_bits(&logits);
        assert!(entropy.is_finite() && entropy > 0.0, "got {entropy}");

        let surprisal = surprisal_bits(&logits, 0).expect("a finite surprisal");
        assert!(surprisal.is_finite(), "got {surprisal}");
    }

    /// The arithmetic is right, not merely finite. A uniform distribution over
    /// four outcomes has entropy of exactly two bits, and each outcome carries
    /// two bits of surprisal.
    #[test]
    fn a_uniform_distribution_measures_two_bits_over_four_outcomes() {
        let logits = [1.0f32; 4];
        assert!((entropy_bits(&logits) - 2.0).abs() < 1e-4);
        assert!((surprisal_bits(&logits, 2).expect("present") - 2.0).abs() < 1e-4);
    }

    /// **Two equal logits carry one bit however large they are.** The magnitude
    /// is a property of the distribution's position, not of its shape, so the
    /// answer at `f32::MAX` is the answer at 1.0.
    ///
    /// This is the regression. The earlier form took `lse` in unshifted space
    /// and then computed `probability * logit` and `lse - logit`: the first
    /// overflows to infinity at `f32::MAX` and the second cancels to zero, so
    /// both signals reported nothing where they should report a bit. Measured
    /// at `0` entropy and `0` surprisal before the fix.
    #[test]
    fn equal_logits_carry_one_bit_at_any_magnitude() {
        for logits in [
            vec![f32::MAX, f32::MAX],
            vec![1e30f32, 1e30],
            vec![100.0f32, 100.0],
            vec![1.0f32, 1.0],
        ] {
            let entropy = entropy_bits(&logits);
            let surprisal = surprisal_bits(&logits, 0).expect("a finite surprisal");
            assert!(
                (entropy - 1.0).abs() < 1e-4,
                "entropy at {:?} is one bit, got {entropy}",
                logits[0]
            );
            assert!(
                (surprisal - 1.0).abs() < 1e-4,
                "surprisal at {:?} is one bit, got {surprisal}",
                logits[0]
            );
        }
    }

    /// A distribution with all its mass in one place carries no entropy, and
    /// the token that takes the mass carries no surprise.
    #[test]
    fn a_certain_distribution_measures_nothing() {
        let logits = [80.0f32, -80.0, -80.0];
        assert!(entropy_bits(&logits) < 1e-3);
        assert!(surprisal_bits(&logits, 0).expect("present") < 1e-3);
    }

    /// **Absent is not empty.** A generation that produced no token answers
    /// `None` on both vectors rather than a zero-length one.
    ///
    /// Perturbation: change `Accumulator::finish` to wrap the vectors in
    /// `Some` unconditionally and this fails, because a run that measured
    /// nothing then reports having measured an empty run. Watched under
    /// exactly that change.
    #[test]
    fn a_generation_that_measured_nothing_is_absent_rather_than_empty() {
        let signals = Accumulator::new().finish();
        assert_eq!(signals.entropy_bits, None);
        assert_eq!(signals.surprisal_bits, None);
        assert_eq!(signals.steps(), 0);

        let mut one = Accumulator::new();
        one.record(1.0, 2.0);
        let measured = one.finish();
        assert_eq!(measured.steps(), 1, "and one step is present, not absent");
    }

    /// **The partition covers the prompt exactly**, the last offset reaching
    /// the rendered text's end.
    ///
    /// Perturbation: drop the `last != text_len` check from
    /// `PromptPartition::new` and this fails, the short partition being
    /// accepted. Watched under exactly that removal.
    #[test]
    fn a_partition_short_of_the_end_is_refused() {
        let text_len = 11;
        assert!(PromptPartition::new(vec![4, 8, 11], text_len).is_ok());

        assert_eq!(
            PromptPartition::new(vec![4, 8, 10], text_len),
            Err(PartitionDefect::ShortOfEnd {
                last: 10,
                text_len: 11
            }),
            "a gap at the end is a consumer's guess, so it is refused here"
        );

        assert_eq!(
            PromptPartition::new(vec![4, 2, 11], text_len),
            Err(PartitionDefect::NotAscending { at: 1 })
        );
    }

    /// **The field ranks by probability and names the realized rank**, per
    /// `weaver-spu-Spec` section 7.5. A uniform vector plus one peak has a
    /// known order, so the ranking is checkable without depending on the
    /// arithmetic the entropy tests already buy.
    ///
    /// Perturbation: rank ascending instead of descending and the leader
    /// assertion fails; drop the realized lookup and the rank does.
    #[test]
    fn the_field_ranks_and_names_the_draw() {
        let mut logits = vec![0.0f32; 32];
        logits[7] = 10.0;
        logits[3] = 5.0;
        logits[19] = 1.0;
        let (ranked, realized) = field(&logits, 3, 4).expect("a field");
        assert_eq!(ranked.len(), 4, "the depth's worth of candidates");
        assert_eq!(ranked[0].0, 7, "the peak leads");
        assert_eq!(ranked[1].0, 3, "then the second");
        assert_eq!(ranked[2].0, 19, "then the third");
        assert!(
            ranked[0].1 > ranked[1].1 && ranked[1].1 > ranked[2].1,
            "descending probability: {ranked:?}"
        );
        assert_eq!(realized, 1, "the drawn token's own rank");
        let total: f32 = ranked.iter().map(|(_, p)| p).sum();
        assert!(
            total > 0.0 && total < 1.0 + 1e-4,
            "probabilities, not logits: {total}"
        );
    }

    /// **A draw outside the reported depth reads as past it**, never as a
    /// rank that was observed. The alternative, reporting rank zero or
    /// omitting the member, would put a token at the front of a field it
    /// was not in.
    #[test]
    fn a_draw_below_the_depth_reads_as_past_it() {
        let mut logits = vec![0.0f32; 32];
        for (at, value) in [(7, 10.0), (3, 5.0), (19, 1.0)] {
            logits[at] = value;
        }
        let (_, realized) = field(&logits, 11, 3).expect("a field");
        assert_eq!(realized, 3, "the depth itself, meaning past the report");
    }

    /// **A `NaN` among real logits is an absence, not a `NaN` reading.**
    /// `f32::max` skips it in the fold, so the maximum is finite while the
    /// sum is not, and every reading taken from that sum would be `NaN`
    /// without the normaliser's second guard.
    ///
    /// Perturbation: drop the `sum.is_finite()` guard and the surprisal
    /// answers `Some(NaN)` and the field answers `NaN` probabilities, both
    /// of which a consumer records as though they meant something.
    #[test]
    fn a_broken_distribution_is_absent_rather_than_nan() {
        let logits = vec![1.0f32, f32::NAN, 3.0, 2.0];
        assert!(
            logits.iter().copied().fold(f32::NEG_INFINITY, f32::max).is_finite(),
            "the fixture's maximum is finite, which is what makes it the case"
        );
        assert!(surprisal_bits(&logits, 2).is_none(), "no surprisal");
        assert!(field(&logits, 2, 2).is_none(), "no field");
        assert_eq!(entropy_bits(&logits), 0.0, "and the entropy reports none");
    }

    /// **A depth past the vocabulary answers the vocabulary**, without
    /// reserving the declared number of slots. The depth carries a floor at
    /// the sampling cutoff and no ceiling, so this is the shape a stray
    /// digit in a declaration takes, and it must be an ordinary answer
    /// rather than an allocation.
    #[test]
    fn a_depth_past_the_vocabulary_answers_the_vocabulary() {
        let logits = vec![0.0f32, 3.0, 1.0, 2.0];
        let (ranked, realized) = field(&logits, 1, 1_000_000).expect("a field");
        assert_eq!(ranked.len(), logits.len(), "no more leaders than logits");
        assert_eq!(ranked[0].0, 1, "still ranked by probability");
        assert_eq!(realized, 0, "and the draw keeps its rank");
    }

    /// A depth of zero and an index outside the vector are absences rather
    /// than empty fields, the rule the signals follow.
    #[test]
    fn the_field_is_absent_rather_than_empty() {
        let logits = vec![0.0f32, 1.0, 2.0];
        assert!(field(&logits, 0, 0).is_none(), "no depth, no field");
        assert!(field(&logits, 9, 2).is_none(), "no such draw, no field");
    }
}
