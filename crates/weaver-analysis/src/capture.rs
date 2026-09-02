//! conforms: analysis-captures-compare-exactly
//!
//! A capture's columns and their comparison, per `weaver-analysis-Spec`
//! section 5. A capture is a certified diagnostic record kept whole, so
//! this module adds no format: it reads the record's `residual.column`
//! events and pairs them by turn.
//!
//! **The comparison is certification step 3's own check, performed where
//! both records are held.** Within one device model it is exact, per
//! `weaver-diagnostic-PRD` section 4 as measured, so two captures of one
//! source under one declaration agree value for value or the comparison
//! names the first disagreement. Cardinality is checked and never
//! truncated: an equal-and-empty comparison would be a verdict over no
//! evidence.

use std::collections::BTreeMap;

use crate::record::{Event, value_at};

/// One sampled position's columns, keyed by the turn beside the position:
/// a record holds several brackets and positions repeat across them.
pub type Key = (Option<String>, u64);

/// A capture's columns and the token each position drew.
#[derive(Debug, Default)]
pub struct Capture {
    pub columns: BTreeMap<Key, Vec<Vec<f32>>>,
    pub drawn: BTreeMap<Key, u32>,
}

impl Capture {
    /// A copy for a caller that needs to alter one value and compare, the
    /// tests' own use: the columns are the bulk and this clones them, so
    /// it is named for what it costs rather than derived silently.
    pub fn clone_shallow(&self) -> Capture {
        Capture {
            columns: self.columns.clone(),
            drawn: self.drawn.clone(),
        }
    }

    /// The columns of a diagnostic record, paired by turn and by the
    /// measurement's own order: a turn's measurement consumes exactly the
    /// positions gathered for that turn, the output order being the draws'
    /// own order - the same pairing the field's realized rank encodes. A
    /// column that pairs with no drawn token is held and not read.
    pub fn of(events: &[Event]) -> Capture {
        let mut capture = Capture::default();
        let mut pending: BTreeMap<Option<String>, Vec<u64>> = BTreeMap::new();
        for event in events {
            let turn = event.envelope.turn.clone();
            match event.envelope.kind.as_str() {
                "residual.column" => {
                    let Some(payload) = event.payload.as_deref() else {
                        continue;
                    };
                    let Some(position) = value_at(payload, "position")
                        .and_then(|raw| raw.get().parse::<u64>().ok())
                    else {
                        continue;
                    };
                    let Some(values) = value_at(payload, "values")
                        .and_then(|raw| serde_json::from_str::<Vec<Vec<f32>>>(raw.get()).ok())
                    else {
                        continue;
                    };
                    capture.columns.insert((turn.clone(), position), values);
                    pending.entry(turn).or_default().push(position);
                }
                "model.measurement" => {
                    let Some(payload) = event.payload.as_deref() else {
                        continue;
                    };
                    let Some(out) = value_at(payload, "output_tokens")
                        .and_then(|raw| serde_json::from_str::<Vec<u32>>(raw.get()).ok())
                    else {
                        continue;
                    };
                    let mut positions = pending.remove(&turn).unwrap_or_default();
                    positions.sort_unstable();
                    for (position, token) in positions.into_iter().zip(out) {
                        capture.drawn.insert((turn.clone(), position), token);
                    }
                }
                _ => {}
            }
        }
        capture
    }

    /// The positions holding both a column and the token it drew, in
    /// order: what a reading reads.
    pub fn paired(&self) -> Vec<Key> {
        self.columns
            .keys()
            .filter(|key| self.drawn.contains_key(*key))
            .cloned()
            .collect()
    }
}

/// What a comparison of two captures answers.
#[derive(Debug, Clone, PartialEq)]
pub enum Comparison {
    /// Every value equal, with the count that was compared: the evidence
    /// the verdict rests on rather than the verdict alone.
    Identical { positions: usize, values: usize },
    /// The first disagreement, named by where it sits.
    Diverged {
        turn: Option<String>,
        position: u64,
        layer: usize,
        left: f32,
        right: f32,
    },
    /// The two records are not two captures of one run, or one holds
    /// nothing to compare.
    Incomparable { detail: String },
}

/// Two captures differenced position for position. **Cardinality is
/// checked and never truncated**: differing position sets, layer counts,
/// or widths refuse rather than comparing what happens to align, and an
/// empty set refuses rather than verdicting over no evidence.
pub fn compare(left: &Capture, right: &Capture) -> Comparison {
    if left.columns.is_empty() || right.columns.is_empty() {
        return Comparison::Incomparable {
            detail: "a record holds no residual column".to_string(),
        };
    }
    if left.drawn != right.drawn {
        return Comparison::Incomparable {
            detail: "the token paths differ: these are not two replays of one run"
                .to_string(),
        };
    }
    let left_keys: Vec<&Key> = left.columns.keys().collect();
    let right_keys: Vec<&Key> = right.columns.keys().collect();
    if left_keys != right_keys {
        return Comparison::Incomparable {
            detail: "the sampled positions differ".to_string(),
        };
    }
    let mut values = 0usize;
    for (key, a) in &left.columns {
        let b = &right.columns[key];
        if a.len() != b.len() {
            return Comparison::Incomparable {
                detail: format!(
                    "the layer counts differ at position {}: {} and {}",
                    key.1,
                    a.len(),
                    b.len()
                ),
            };
        }
        if a.is_empty() {
            return Comparison::Incomparable {
                detail: format!("the column at position {} holds no layers", key.1),
            };
        }
        for (layer, (la, lb)) in a.iter().zip(b).enumerate() {
            if la.len() != lb.len() {
                return Comparison::Incomparable {
                    detail: format!(
                        "the widths differ at position {} layer {layer}: {} and {}",
                        key.1,
                        la.len(),
                        lb.len()
                    ),
                };
            }
            if la.is_empty() {
                return Comparison::Incomparable {
                    detail: format!(
                        "the layer {layer} at position {} holds no values",
                        key.1
                    ),
                };
            }
            for (x, y) in la.iter().zip(lb) {
                values += 1;
                // **The bits, not the values.** Two captures of one run are
                // the same bytes or they are not: `==` on floats calls
                // `0.0` and `-0.0` equal though their bits differ, and
                // calls a `NaN` unequal to its own bit pattern, so an
                // arithmetic comparison would admit one difference and
                // invent another. The measurement this bar rests on was
                // taken over bytes.
                if x.to_bits() != y.to_bits() {
                    return Comparison::Diverged {
                        turn: key.0.clone(),
                        position: key.1,
                        layer,
                        left: *x,
                        right: *y,
                    };
                }
            }
        }
    }
    Comparison::Identical {
        positions: left.columns.len(),
        values,
    }
}
