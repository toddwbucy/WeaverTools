//! The per-position signals, read as a series, per the drain of section 5.
//!
//! **This reader needs no tap, no lens, and no weights.** The entropy at
//! each decode position rides every generation's measurement
//! unconditionally and the surprisal rides it where that election stands,
//! both paired position for position with the tokens drawn, per
//! `weaver-spu-Spec` section 6. So the series a reader wants - where the
//! model was uncertain, and where the token it drew surprised it - is
//! already in every record, serving and diagnostic alike, and this reader
//! only pairs and emits it.
//!
//! It is the class's second reader and exists partly to show the class is
//! one: it rides the same drain the lens rides, holds one turn at a time,
//! and shares nothing with the lens but the road.

use crate::record::{Event, value_at};
use crate::stream::{Reader, Step};

/// One position's reading: the token drawn there and what the
/// distribution said about it.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub turn: Option<String>,
    /// The position's ordinal within its generation, zero-based, which is
    /// what a series is drawn against.
    pub ordinal: usize,
    pub token: u32,
    /// The distribution's entropy in bits at this position. Absent where
    /// the generation measured none, per the absent-not-empty rule.
    pub entropy: Option<f32>,
    /// The drawn token's surprisal in bits. Absent where the election did
    /// not stand, which is the ordinary posture.
    pub surprisal: Option<f32>,
}

/// The series, and the generation-level figures beside it.
#[derive(Debug, Clone, Default)]
pub struct Series {
    pub points: Vec<Point>,
    /// One entry per generation: its turn and its perplexity where the
    /// record carries one.
    pub perplexities: Vec<(Option<String>, f32)>,
}

impl Series {
    /// The positions whose surprisal exceeds `mean + k * deviation`, the
    /// spikes a reader is looking for. **Stated as a rule rather than a
    /// threshold**: what counts as a spike depends on the series, so the
    /// caller names its `k` and this answers which positions clear it.
    /// Empty where no position carries a surprisal.
    pub fn spikes(&self, k: f32) -> Vec<&Point> {
        let held: Vec<f32> = self.points.iter().filter_map(|p| p.surprisal).collect();
        if held.len() < 2 {
            return Vec::new();
        }
        let mean = held.iter().sum::<f32>() / held.len() as f32;
        let variance =
            held.iter().map(|s| (s - mean) * (s - mean)).sum::<f32>() / held.len() as f32;
        let bar = mean + k * variance.sqrt();
        self.points
            .iter()
            .filter(|p| p.surprisal.is_some_and(|s| s > bar))
            .collect()
    }
}

/// The reader itself: one measurement at a time, nothing held between.
///
/// **It reads either record and gates only the one that has a gate.** A
/// serving record carries no bracket outcome and none is owed; a
/// diagnostic record carries one, and a series read from an uncertified
/// replay is a picture of an unknown run exactly as a readout is, per
/// `weaver-diagnostic-PRD` section 4. So this reader records which record
/// it holds and what that record's bracket said, and leaves the judgment
/// to its caller.
#[derive(Debug, Default)]
pub struct Signals {
    pub series: Series,
    /// The first event's kind, which is how a record says which record it
    /// is, per `weaver-diagnostic-Spec` section 4.
    pub opening: Option<String>,
    /// The bracket's outcome where the record carries one.
    pub outcome: Option<String>,
    run: Option<String>,
}

impl Signals {
    /// Whether this record is a diagnostic one, answered by the record.
    pub fn diagnostic(&self) -> bool {
        self.opening.as_deref() == Some("replay.opened")
    }

    /// Whether a series read from this record may be produced: a serving
    /// record has no gate, and a diagnostic one passes only where its own
    /// bracket closed certified.
    pub fn licensed(&self) -> bool {
        !self.diagnostic() || self.outcome.as_deref() == Some("certified")
    }
}

impl Reader for Signals {
    fn event(&mut self, event: &Event) -> Step {
        if self.opening.is_none() {
            self.opening = Some(event.envelope.kind.clone());
            if event.envelope.kind == "replay.opened" {
                self.run = Some(event.envelope.run.clone());
            }
        }
        if event.envelope.kind == "replay.closed" {
            if self.run.as_deref() != Some(event.envelope.run.as_str()) {
                return Step::Continue;
            }
            self.outcome = event
                .payload
                .as_deref()
                .and_then(|p| value_at(p, "outcome.kind"))
                .map(|raw| raw.get().trim_matches('"').to_string());
            // The bracket's close ends this reading too, for the same
            // reason it ends the capture's: a pipe's writer holds the
            // stream open for the run's residency.
            return Step::Done;
        }
        if event.envelope.kind != "model.measurement" {
            return Step::Continue;
        }
        let Some(payload) = event.payload.as_deref() else {
            return Step::Continue;
        };
        let read = |key: &str| -> Option<Vec<f32>> {
            value_at(payload, key).and_then(|raw| serde_json::from_str(raw.get()).ok())
        };
        let Some(tokens) = value_at(payload, "output_tokens")
            .and_then(|raw| serde_json::from_str::<Vec<u32>>(raw.get()).ok())
        else {
            return Step::Continue;
        };
        let entropies = read("entropies");
        let surprisals = read("surprisals");
        for (ordinal, token) in tokens.into_iter().enumerate() {
            self.series.points.push(Point {
                turn: event.envelope.turn.clone(),
                ordinal,
                token,
                // **Absent stays absent**: a vector shorter than the
                // tokens is not stretched and a missing one is not
                // invented, per the derive-nothing rule.
                entropy: entropies.as_ref().and_then(|v| v.get(ordinal).copied()),
                surprisal: surprisals.as_ref().and_then(|v| v.get(ordinal).copied()),
            });
        }
        if let Some(perplexity) =
            value_at(payload, "perplexity").and_then(|raw| raw.get().parse::<f32>().ok())
        {
            self.series
                .perplexities
                .push((event.envelope.turn.clone(), perplexity));
        }
        Step::Continue
    }
}
