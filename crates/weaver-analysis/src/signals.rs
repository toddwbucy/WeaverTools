//! conforms: analysis-summary-reports-the-record-session
//! conforms: analysis-summary-reports-the-record-digest
//! conforms: analysis-summary-reports-the-prefix-length
//! conforms: analysis-summary-reports-the-run-and-its-conditions
//! conforms: analysis-signals-keep-absence
//! conforms: analysis-summary-reports-residency
//! conforms: analysis-summary-reports-the-record-identity
//!
//! The per-position signals, read as a series, per `weaver-analysis-Spec`
//! section 5 as of 2026-09-05.
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
//! one: it rides the same drain the lens rides, holds one generation per
//! run in flight, and shares nothing with the lens but the road.

use crate::deposit::{CodeIdentity, Deposit};
use crate::record::{Event, value_at};
use crate::stream::{Reader, Step};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

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

/// Per-generation facts, with run facts repeated only when known.
/// Raw values serialize directly, without an intermediate JSON Value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GenerationSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perplexity: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resident: Option<u64>,
    pub output_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights_hash: Option<String>,
    pub run: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_sampling: Option<Box<RawValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_depth: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage: Option<Box<RawValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_identity: Option<CodeIdentity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verdict: Option<Verdict>,
}

/// Reserved until the task-close event in #523 exists. This reader emits none.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verdict {
    pub predicate: String,
    pub ratio: f64,
    pub denominator: u64,
}

impl GenerationSummary {
    /// Validate an emission before a consumer keys a row by its run.
    pub fn read(emission: &str) -> Result<Self, String> {
        let members: BTreeMap<String, Box<RawValue>> =
            serde_json::from_str(emission).map_err(|e| format!("invalid summary: {e}"))?;
        if !members.contains_key("run") {
            return Err(
                "summary from an emitter older than 2026-09-09: run identity absent".into(),
            );
        }
        serde_json::from_str(emission).map_err(|e| format!("invalid summary: {e}"))
    }
}

/// The series, and the generation-level figures beside it.
#[derive(Debug, Clone, Default)]
pub struct Series {
    pub points: Vec<Point>,
    /// One entry per measured generation, in landing order.
    pub generations: Vec<GenerationSummary>,
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

/// Signals retain summaries and points, but never the drained record lines.
#[derive(Debug, Default)]
pub struct Signals {
    pub series: Series,
    pub opening: Option<String>,
    pub outcome: Option<String>,
    diagnostic_run: Option<String>,
    runs: BTreeMap<String, Run>,
    deposit: Deposit,
}

#[derive(Debug, Default)]
struct Run {
    loaded: bool,
    closed: bool,
    hash: Sha256,
    raw_only: bool,
    session: Option<String>,
    depth: Option<u32>,
    lineage: Option<Box<RawValue>>,
    stack: Option<Box<RawValue>>,
    first_started: bool,
    pending: Option<Generation>,
    prefix: Option<u64>,
    entries: Vec<usize>,
    sampling: Option<serde_json::Value>,
    weights: Option<String>,
}

#[derive(Debug)]
struct Generation {
    turn: Option<String>,
    first: bool,
    closing: Option<u64>,
    sampling: Option<Box<RawValue>>,
}

// This reader admits pre-session records; the reconstruction parser continues
// to require its full envelope. No missing session is manufactured for it.
#[derive(Deserialize)]
struct SignalEvent {
    #[serde(rename = "sequence")]
    _sequence: String,
    session: Option<String>,
    run: String,
    turn: Option<String>,
    kind: String,
    payload: Option<Box<RawValue>>,
}

impl Run {
    fn begin(&mut self, turn: Option<String>) {
        self.pending = Some(Generation {
            turn,
            first: !self.first_started,
            closing: None,
            sampling: None,
        });
        self.first_started = true;
    }
}

impl Signals {
    pub fn with_deposit(deposit: Deposit) -> Self {
        Self {
            deposit,
            ..Self::default()
        }
    }

    pub fn diagnostic(&self) -> bool {
        self.opening.as_deref() == Some("replay.opened")
    }

    pub fn licensed(&self) -> bool {
        !self.diagnostic() || self.outcome.as_deref() == Some("certified")
    }

    fn observe(&mut self, event: SignalEvent, raw: Option<&str>) -> Step {
        if self.opening.is_none() {
            self.opening = Some(event.kind.clone());
            if event.kind == "replay.opened" {
                self.diagnostic_run = Some(event.run.clone());
            }
        }
        let run = self.runs.entry(event.run.clone()).or_insert_with(|| Run {
            raw_only: true,
            ..Run::default()
        });
        if run.closed {
            return Step::Refuse(format!("run {} has an event after unload", event.run));
        }
        if let Some(line) = raw {
            run.hash.update(line.as_bytes());
        } else {
            run.raw_only = false;
        }
        if let Some(session) = &event.session {
            if run.session.as_ref().is_some_and(|held| held != session) {
                return Step::Refuse(format!("run {} disagrees on session", event.run));
            }
            run.session = Some(session.clone());
        }
        let payload = event.payload.as_deref();
        let member = |key: &str| payload.and_then(|p| value_at(p, key));
        if event.kind == "replay.closed" {
            if self.diagnostic_run.as_deref() != Some(event.run.as_str()) {
                return Step::Continue;
            }
            self.outcome = member("outcome.kind").and_then(|v| serde_json::from_str(v.get()).ok());
            return Step::Done;
        }
        match event.kind.as_str() {
            "load" => {
                if run.loaded || run.first_started {
                    return Step::Refuse(format!("run {} has a late or repeated load", event.run));
                }
                run.loaded = true;
                run.depth = member("field").and_then(|v| serde_json::from_str(v.get()).ok());
                run.lineage = member("lineage")
                    .filter(|v| v.get() != "null")
                    .map(RawValue::to_owned);
                run.stack = member("stack")
                    .filter(|v| v.get() != "null")
                    .map(RawValue::to_owned);
            }
            "unload" => {
                run.closed = true;
                if run.loaded {
                    let digest = run
                        .raw_only
                        .then(|| format!("{:x}", run.hash.clone().finalize()));
                    for index in &run.entries {
                        self.series.generations[*index].digest = digest.clone();
                        self.series.generations[*index].prefix_length = run.prefix;
                    }
                }
            }
            "model.request" => {
                run.begin(event.turn);
                let sampling = member("sampling").filter(|v| v.get() != "null");
                if let Some(raw) = sampling {
                    let Ok(mut declared) = serde_json::from_str::<
                        serde_json::Map<String, serde_json::Value>,
                    >(raw.get()) else {
                        return Step::Refuse(format!("run {} has unreadable sampling", event.run));
                    };
                    declared.remove("generation_seed");
                    let declared = serde_json::Value::Object(declared);
                    if run.sampling.as_ref().is_some_and(|held| held != &declared) {
                        return Step::Refuse(format!(
                            "run {} disagrees on declared sampling",
                            event.run
                        ));
                    }
                    run.sampling = Some(declared);
                }
                run.pending.as_mut().unwrap().sampling = sampling.map(RawValue::to_owned);
            }
            "model.output" => {
                if run.pending.as_ref().is_none_or(|g| g.turn != event.turn) {
                    run.begin(event.turn);
                }
                run.pending.as_mut().unwrap().closing =
                    member("resident").and_then(|v| serde_json::from_str(v.get()).ok());
            }
            "model.measurement" => {
                if run.pending.as_ref().is_none_or(|g| g.turn != event.turn) {
                    run.begin(event.turn.clone());
                }
                let generation = run.pending.take().unwrap();
                let Some(tokens) = member("output_tokens")
                    .and_then(|v| serde_json::from_str::<Vec<u32>>(v.get()).ok())
                else {
                    return Step::Continue;
                };
                let inputs = member("input_tokens")
                    .and_then(|v| serde_json::from_str::<Vec<u32>>(v.get()).ok());
                if generation.first && run.loaded {
                    run.prefix = generation.closing.and_then(|r| {
                        r.checked_sub(tokens.len() as u64)?
                            .checked_sub(1)?
                            .checked_sub(inputs.as_ref()?.len() as u64)
                    });
                }
                let weights: Option<String> =
                    member("weights_hash").and_then(|v| serde_json::from_str(v.get()).ok());
                if let Some(weights) = &weights {
                    if run.weights.as_ref().is_some_and(|held| held != weights) {
                        return Step::Refuse(format!(
                            "run {} disagrees on weights hash",
                            event.run
                        ));
                    }
                    run.weights = Some(weights.clone());
                }
                let entropies: Option<Vec<f32>> =
                    member("entropies").and_then(|v| serde_json::from_str(v.get()).ok());
                let surprisals: Option<Vec<f32>> =
                    member("surprisals").and_then(|v| serde_json::from_str(v.get()).ok());
                for (ordinal, token) in tokens.iter().enumerate() {
                    self.series.points.push(Point {
                        turn: event.turn.clone(),
                        ordinal,
                        token: *token,
                        entropy: entropies.as_ref().and_then(|v| v.get(ordinal).copied()),
                        surprisal: surprisals.as_ref().and_then(|v| v.get(ordinal).copied()),
                    });
                }
                run.entries.push(self.series.generations.len());
                self.series.generations.push(GenerationSummary {
                    turn: event.turn,
                    perplexity: member("perplexity")
                        .and_then(|v| serde_json::from_str(v.get()).ok()),
                    resident: generation.closing,
                    output_count: tokens.len(),
                    weights_hash: weights,
                    run: event.run,
                    session: event.session,
                    effective_sampling: generation.sampling,
                    field_depth: run.depth,
                    lineage: run.lineage.clone(),
                    device_model: self.deposit.device_model.clone(),
                    code_identity: self.deposit.code_identity(run.stack.clone()),
                    ..GenerationSummary::default()
                });
            }
            _ => {}
        }
        Step::Continue
    }
}

impl Reader for Signals {
    fn event(&mut self, event: &Event) -> Step {
        self.observe(
            SignalEvent {
                _sequence: event.envelope.sequence.clone(),
                session: Some(event.envelope.session.clone()),
                run: event.envelope.run.clone(),
                turn: event.envelope.turn.clone(),
                kind: event.envelope.kind.clone(),
                payload: event.payload.clone(),
            },
            None,
        )
    }

    fn line(&mut self, line: &str) -> Step {
        match serde_json::from_str(line) {
            Ok(event) => self.observe(event, Some(line)),
            Err(_) => Step::Continue,
        }
    }
}
