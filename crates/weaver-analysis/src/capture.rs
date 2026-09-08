//! conforms: analysis-captures-compare-exactly
//! conforms: analysis-compare-refuses-across-loops-and-members
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

/// What a record's `load` event says about who composed the run and
/// whether the state member stood, per `weaver-trace-Spec` section 3 as of
/// 2026-09-03: the two facts that decide whether two records are two
/// captures of one run, per `weaver-analysis-PRD` section 3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    pub binary: String,
    pub file: Option<String>,
    pub sha256: Option<String>,
    pub state_member: bool,
}

impl Provenance {
    /// The composer as a reader names it: the binary, and the file with its
    /// digest where the loop is a file.
    pub fn composer(&self) -> String {
        match (&self.file, &self.sha256) {
            (Some(file), Some(digest)) => {
                format!(
                    "{} reading {file} ({})",
                    self.binary,
                    &digest[..digest.len().min(12)]
                )
            }
            (Some(file), None) => format!("{} reading {file}", self.binary),
            _ => self.binary.clone(),
        }
    }

    /// The provenance a `load` event's `composer` names, or none where the
    /// shape is not the one `weaver-trace-Spec` section 3 spells: a binary
    /// that names nothing, or a file without its digest or a digest without
    /// its file, is a loop that cannot be known and reads as no loop named.
    pub fn of(composer: &serde_json::Value, state_member: bool) -> Option<Provenance> {
        let binary = composer.get("binary").and_then(|b| b.as_str())?;
        if binary.is_empty() {
            return None;
        }
        let file = composer
            .get("file")
            .and_then(|f| f.as_str())
            .map(str::to_string);
        let sha256 = composer
            .get("sha256")
            .and_then(|d| d.as_str())
            .map(str::to_string);
        if file.is_some() != sha256.is_some() {
            return None;
        }
        Some(Provenance {
            binary: binary.to_string(),
            file,
            sha256,
            state_member,
        })
    }

    fn same_composer(&self, other: &Provenance) -> bool {
        self.binary == other.binary && self.file == other.file && self.sha256 == other.sha256
    }
}

/// A capture's columns, the token each position drew, and what its record
/// says about the loop and the member, absent where the record predates
/// the `load` event naming them.
#[derive(Debug, Default)]
pub struct Capture {
    pub columns: BTreeMap<Key, Vec<Vec<f32>>>,
    pub drawn: BTreeMap<Key, u32>,
    pub provenance: Option<Provenance>,
}

impl Capture {
    /// A copy for a caller that needs to alter one value and compare, the
    /// tests' own use: the columns are the bulk and this clones them, so
    /// it is named for what it costs rather than derived silently.
    pub fn clone_shallow(&self) -> Capture {
        Capture {
            columns: self.columns.clone(),
            drawn: self.drawn.clone(),
            provenance: self.provenance.clone(),
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
                // **The load names the loop and the member**, per
                // `weaver-trace-Spec` section 3 as of 2026-09-03, and a
                // record naming neither is one whose loop cannot be known.
                "load" => {
                    let Some(payload) = event.payload.as_deref() else {
                        continue;
                    };
                    let composer = value_at(payload, "composer")
                        .and_then(|raw| serde_json::from_str::<serde_json::Value>(raw.get()).ok());
                    let state_member = value_at(payload, "state_member")
                        .and_then(|raw| raw.get().parse::<bool>().ok());
                    if let (Some(composer), Some(state_member)) = (composer, state_member) {
                        capture.provenance = Provenance::of(&composer, state_member);
                    }
                }
                "residual.column" => {
                    let Some(payload) = event.payload.as_deref() else {
                        continue;
                    };
                    let Some(position) =
                        value_at(payload, "position").and_then(|raw| raw.get().parse::<u64>().ok())
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
    // **The load is read before any value**, per `weaver-analysis-Spec`
    // section 5 as of 2026-09-07 and issue #381: a prompt assembled by
    // another loop diverges at the first token, and the token-path refusal
    // below would report that as two runs rather than as two loops.
    let (a, b) = match (&left.provenance, &right.provenance) {
        (Some(a), Some(b)) => (a, b),
        (None, _) => {
            return Comparison::Incomparable {
                detail: "the left record's load names no loop and no member: a record older than the fact cannot be compared".to_string(),
            };
        }
        (_, None) => {
            return Comparison::Incomparable {
                detail: "the right record's load names no loop and no member: a record older than the fact cannot be compared".to_string(),
            };
        }
    };
    if !a.same_composer(b) {
        return Comparison::Incomparable {
            detail: format!(
                "the loops differ: {} composed the left record and {} the right, so the prompts are two loops' and no disagreement below is the engine's",
                a.composer(),
                b.composer()
            ),
        };
    }
    if a.state_member != b.state_member {
        return Comparison::Incomparable {
            detail: format!(
                "the state member stood for the {} record and not the {}, so one session's past reached a prompt the other's did not",
                if a.state_member { "left" } else { "right" },
                if a.state_member { "right" } else { "left" }
            ),
        };
    }
    if left.columns.is_empty() || right.columns.is_empty() {
        return Comparison::Incomparable {
            detail: "a record holds no residual column".to_string(),
        };
    }
    if left.drawn != right.drawn {
        return Comparison::Incomparable {
            detail: "the token paths differ: these are not two replays of one run".to_string(),
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
                    detail: format!("the layer {layer} at position {} holds no values", key.1),
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

/// The capture read as the stream drains, per `weaver-analysis-Spec`
/// section 5, holding one turn at a time.
///
/// **What it holds is bounded by the turn in flight and the analyst's
/// named positions.** The control needs each position's final-layer
/// column against the token that position drew, and the drawn tokens
/// arrive with the turn's measurement after its columns, so the final
/// layers of the turn in flight are held until that measurement pairs
/// them - then the ranks are taken by the caller's own reading and the
/// columns are dropped. The trajectory's columns are held only for the
/// positions the analyst named. So a reading over a pipe costs one turn's
/// final layers and the named positions, never the record.
pub struct Streaming<'a> {
    wanted: Vec<u64>,
    /// The turn in flight: its positions in landing order, and the final
    /// layer of each, held until the measurement names their tokens.
    pending: Vec<(u64, Vec<f32>)>,
    /// Full columns for the named positions, which the trajectory reads.
    pub kept: BTreeMap<Key, Vec<Vec<f32>>>,
    /// What the caller does with each paired position as it lands: the
    /// final-layer column and the token drawn there.
    paired: &'a mut dyn FnMut(&Key, &[f32], u32),
    /// The record's own outcome, which gates whether a reading is
    /// produced at all. `None` until a `replay.closed` lands.
    pub outcome: Option<String>,
    pub opened: bool,
    /// The run the opened bracket belongs to. **A close carries its own
    /// run**, and a record may hold several brackets, so the close that
    /// ends this reading is the one whose run opened it: another run's
    /// outcome certifying these columns would be one pass vouching for
    /// another's.
    run: Option<String>,
}

impl<'a> Streaming<'a> {
    pub fn new(wanted: Vec<u64>, paired: &'a mut dyn FnMut(&Key, &[f32], u32)) -> Streaming<'a> {
        Streaming {
            wanted,
            pending: Vec::new(),
            kept: BTreeMap::new(),
            paired,
            outcome: None,
            opened: false,
            run: None,
        }
    }

    /// Whether the record's bracket closed certified, which is the only
    /// outcome a reading may be produced over.
    pub fn certified(&self) -> bool {
        self.outcome.as_deref() == Some("certified")
    }
}

impl crate::stream::Reader for Streaming<'_> {
    fn event(&mut self, event: &crate::record::Event) -> crate::stream::Step {
        use crate::stream::Step;
        let turn = event.envelope.turn.clone();
        match event.envelope.kind.as_str() {
            "replay.opened" => {
                self.opened = true;
                self.run = Some(event.envelope.run.clone());
            }
            "residual.column" => {
                let Some(payload) = event.payload.as_deref() else {
                    return Step::Continue;
                };
                let Some(position) = column_position(event) else {
                    return Step::Continue;
                };
                let Some(values) = value_at(payload, "values")
                    .and_then(|r| serde_json::from_str::<Vec<Vec<f32>>>(r.get()).ok())
                else {
                    return Step::Continue;
                };
                let Some(final_layer) = values.last().cloned() else {
                    return Step::Refuse(format!(
                        "the column at position {position} holds no layers"
                    ));
                };
                if self.wanted.contains(&position) {
                    self.kept.insert((turn, position), values);
                }
                self.pending.push((position, final_layer));
            }
            "model.measurement" => {
                let Some(payload) = event.payload.as_deref() else {
                    return Step::Continue;
                };
                let Some(out) = value_at(payload, "output_tokens")
                    .and_then(|r| serde_json::from_str::<Vec<u32>>(r.get()).ok())
                else {
                    return Step::Continue;
                };
                // The turn's columns pair with its own measurement, in the
                // draws' own order, and are dropped as they pair.
                let mut pending = std::mem::take(&mut self.pending);
                pending.sort_by_key(|(position, _)| *position);
                // **The counts agree or the reading refuses.** A zip would
                // pair a prefix and drop the rest silently, where a turn
                // whose columns and drawn tokens disagree is exactly the
                // 13.10 fault the SPU refuses on its own side.
                if pending.len() != out.len() {
                    return Step::Refuse(format!(
                        "turn {:?} holds {} columns against {} drawn tokens",
                        turn,
                        pending.len(),
                        out.len()
                    ));
                }
                for ((position, column), token) in pending.into_iter().zip(out) {
                    (self.paired)(&(turn.clone(), position), &column, token);
                }
            }
            "replay.closed" => {
                // A close from another run ends no reading of this one.
                if self.run.as_deref() != Some(event.envelope.run.as_str()) {
                    return Step::Continue;
                }
                self.outcome = event
                    .payload
                    .as_deref()
                    .and_then(|p| value_at(p, "outcome.kind"))
                    .map(|raw| raw.get().trim_matches('"').to_string());
                // **The bracket's close ends the reading, not the
                // stream's end.** A pipe's writer is the agent, which
                // holds it open for the run's whole residency, so a
                // reader waiting for end-of-stream would wait for the
                // unload - and on a socket, for longer. The pass's own
                // close is the fact that says the reading is complete.
                return Step::Done;
            }
            _ => {}
        }
        Step::Continue
    }
}

/// **The file's default spread**, per `weaver-analysis-Spec` section 5 as of
/// 2026-09-04: `count` of the record's positions in order, the first and the
/// last among them and the rest evenly spaced by index, every position where
/// the record holds `count` or fewer. Duplicates in the input collapse
/// before the spread is taken, so a position seen on two brackets counts
/// once.
pub fn spread(positions: &[u64], count: usize) -> Vec<u64> {
    let ordered: Vec<u64> = positions
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<u64>>()
        .into_iter()
        .collect();
    if count == 0 || ordered.is_empty() {
        return Vec::new();
    }
    if ordered.len() <= count || count == 1 {
        return if count == 1 {
            vec![ordered[0]]
        } else {
            ordered
        };
    }
    let last = ordered.len() - 1;
    let mut chosen: Vec<u64> = (0..count)
        .map(|i| ordered[i * last / (count - 1)])
        .collect();
    chosen.dedup();
    chosen
}

/// The position a `residual.column` event carries, or nothing where the
/// event is another kind or its payload does not spell one.
fn column_position(event: &crate::record::Event) -> Option<u64> {
    if event.envelope.kind != "residual.column" {
        return None;
    }
    let payload = event.payload.as_deref()?;
    value_at(payload, "position").and_then(|r| r.get().parse::<u64>().ok())
}

/// A reader that learns which positions a record holds columns for and
/// keeps nothing else: the first of the file's two reads under the default
/// spread.
#[derive(Default)]
pub struct Positions {
    pub held: Vec<u64>,
    /// The run the opened bracket belongs to, so the read ends at that
    /// bracket's own close and not at the file's end, the record boundary
    /// [`Streaming`] keeps: a file may hold several brackets and the spread
    /// is taken over the one the reading will read.
    run: Option<String>,
}

impl crate::stream::Reader for Positions {
    fn event(&mut self, event: &crate::record::Event) -> crate::stream::Step {
        use crate::stream::Step;
        match event.envelope.kind.as_str() {
            "replay.opened" => self.run = Some(event.envelope.run.clone()),
            "replay.closed" if self.run.as_deref() == Some(event.envelope.run.as_str()) => {
                return Step::Done;
            }
            _ => {
                if let Some(position) = column_position(event) {
                    self.held.push(position);
                }
            }
        }
        Step::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::parse_record;

    fn load(composer: &str, member: &str) -> String {
        format!(
            "{{\"session\":\"s\",\"run\":\"r\",\"sequence\":\"0\",\"kind\":\"load\",\"payload\":{{\"composer\":{composer},\"state_member\":{member}}}}}\n"
        )
    }

    /// **The provenance is the load's composer in the Spec's shape and
    /// nothing looser**, per `weaver-trace-Spec` section 3: a compiled loop
    /// is a binary alone, a file loop is a file with its digest, and an
    /// empty binary or an unpaired file or digest is a loop that cannot be
    /// known, read as no loop named so the comparison refuses it.
    ///
    /// Perturbation: accept a file without its digest and the fourth case
    /// carries a provenance. Watched under exactly that change.
    #[test]
    fn the_provenance_takes_the_specs_shape_and_nothing_looser() {
        let compiled = Capture::of(&parse_record(&load(r#"{"binary":"worker"}"#, "false")));
        assert_eq!(
            compiled.provenance,
            Some(Provenance {
                binary: "worker".into(),
                file: None,
                sha256: None,
                state_member: false
            })
        );
        let file = Capture::of(&parse_record(&load(
            r#"{"binary":"pyworker","file":"/l/a.py","sha256":"ab"}"#,
            "true",
        )));
        assert_eq!(file.provenance.as_ref().map(|p| p.state_member), Some(true));
        assert_eq!(
            file.provenance.as_ref().unwrap().composer(),
            "pyworker reading /l/a.py (ab)"
        );
        for loose in [
            r#"{"binary":""}"#,
            r#"{"binary":"pyworker","file":"/l/a.py"}"#,
            r#"{"binary":"pyworker","sha256":"ab"}"#,
            r#"{}"#,
        ] {
            let capture = Capture::of(&parse_record(&load(loose, "false")));
            assert!(
                capture.provenance.is_none(),
                "{loose} names no loop that can be known"
            );
        }
        let unnamed = Capture::of(&parse_record(&load(r#"{"binary":"worker"}"#, "null")));
        assert!(
            unnamed.provenance.is_none(),
            "a member standing that is not stated"
        );
    }
}
