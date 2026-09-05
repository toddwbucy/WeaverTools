//! A position's field, read from the record, per `weaver-analysis-Spec`
//! section 5 as of 2026-09-04.
//!
//! **The record already carries what else had mass at a position.** Under
//! the field election a `model.field` event stands at every generated
//! position the generation retained, per `weaver-trace-Spec` section 3:
//! the position as the resident length at the draw, the ranked candidates
//! with their probabilities, and the rank the draw landed on. So this
//! reader adds no reading of its own. It rides the class's drain, keeps
//! the one event whose turn and position match the address it was asked
//! for, and drops every other event as it lands, so a record of twenty
//! thousand positions costs the read one position.
//!
//! **It gates on no certified close.** The field is the record's own fact
//! about a position and not a reading taken over a replay, so a serving
//! record and a diagnostic record answer alike, and the diagnostic
//! bracket's close ends the read as it ends every reader's.

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use crate::record::{Event, value_at};
use crate::stream::{Reader, Step};

/// Where a read is pointed: a turn key and a position as the record's
/// `model.field` payload spells `position`, which is the resident length
/// at the draw rather than an ordinal within the generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub turn: String,
    pub position: u64,
}

impl Address {
    /// `<turn>:<position>`, the position a decimal. **Anything else
    /// refuses by name** rather than reading a turn the caller did not
    /// spell: a bare number, an empty turn, or a position that is not a
    /// number each answer the text that was given.
    pub fn parse(text: &str) -> Result<Address, String> {
        let Some((turn, position)) = text.rsplit_once(':') else {
            return Err(format!(
                "the position is <turn>:<position>, and {text:?} carries no colon"
            ));
        };
        if turn.is_empty() {
            return Err(format!(
                "the position is <turn>:<position>, and {text:?} names no turn"
            ));
        }
        match position.parse::<u64>() {
            Ok(position) => Ok(Address {
                turn: turn.to_string(),
                position,
            }),
            Err(_) => Err(format!(
                "the position is <turn>:<position>, and {position:?} is not a position"
            )),
        }
    }
}

/// One position's field, as the record spelled it. The ranked list is the
/// record's own bytes, spliced rather than parsed and re-rendered, so a
/// probability crosses this crate exactly as the SPU wrote it.
#[derive(Debug, Clone, Serialize)]
pub struct Answer {
    pub run: String,
    pub turn: String,
    pub position: u64,
    /// The rank the draw landed on among the ranked candidates, or the
    /// depth itself where the draw fell past what was reported.
    pub realized: u32,
    /// The token drawn at this position: the candidate at the realized
    /// rank where that rank is within the list, otherwise the turn's
    /// measurement's `output_tokens` entry at this field's ordinal within
    /// its generation. **Absent where neither answers**, never invented.
    pub drawn: Option<u32>,
    pub ranked: Box<RawValue>,
}

/// The one member of a candidate this reader reads for itself, to place
/// the realized rank. The probability is never parsed here.
#[derive(Deserialize)]
struct Peek {
    token: u32,
}

/// The reader: one address, one pending answer at most, and a counter for
/// the generation in flight. Nothing else is held.
pub struct FieldReader<'a> {
    address: Address,
    /// The run the caller named, where one was. Named, the read ends when
    /// that run's position has answered.
    run: Option<String>,
    /// What the caller does with each answer as it completes.
    emit: &'a mut dyn FnMut(&Answer),
    /// Whether any `model.field` event landed at all, which is how a
    /// missing position is told from a missing election.
    pub seen_field: bool,
    /// The depth the record's `load` elected, where a `load` event carried
    /// one: the record's own statement of its posture, per
    /// `weaver-spu-PRD` section 13.11.
    pub elected_depth: Option<u32>,
    /// How many answers have completed.
    pub answered: usize,
    /// A match whose draw fell past the reported depth, held with its
    /// ordinal within the generation until that generation's measurement
    /// names the token.
    pending: Option<(Answer, usize)>,
    /// The generation in flight: its run and turn, and how many fields
    /// have landed since its last measurement. Fields pair with the drawn
    /// tokens one for one in landing order, the stop token being neither
    /// retained nor ranked.
    counter: Option<(String, Option<String>, usize)>,
    opening: Option<String>,
    /// The run of a diagnostic record's opened bracket, whose close ends
    /// the read.
    bracket_run: Option<String>,
}

impl<'a> FieldReader<'a> {
    pub fn new(
        address: Address,
        run: Option<String>,
        emit: &'a mut dyn FnMut(&Answer),
    ) -> FieldReader<'a> {
        FieldReader {
            address,
            run,
            emit,
            seen_field: false,
            elected_depth: None,
            answered: 0,
            pending: None,
            counter: None,
            opening: None,
            bracket_run: None,
        }
    }

    /// Whether this record opened as a diagnostic one.
    pub fn diagnostic(&self) -> bool {
        self.opening.as_deref() == Some("replay.opened")
    }

    /// Emit the pending answer with whatever token it has. Called at the
    /// stream's end and at the bracket's close, where the measurement that
    /// would have named the token is not coming.
    pub fn finish(&mut self) {
        if let Some((answer, _)) = self.pending.take() {
            (self.emit)(&answer);
            self.answered += 1;
        }
    }

    fn emit_now(&mut self, answer: Answer) {
        (self.emit)(&answer);
        self.answered += 1;
    }

    /// Whether the read is complete: a named run has answered.
    fn done(&self) -> bool {
        self.run.is_some() && self.answered > 0 && self.pending.is_none()
    }
}

impl Reader for FieldReader<'_> {
    fn event(&mut self, event: &Event) -> Step {
        if self.opening.is_none() {
            self.opening = Some(event.envelope.kind.clone());
            if event.envelope.kind == "replay.opened" {
                self.bracket_run = Some(event.envelope.run.clone());
            }
        }
        let run = &event.envelope.run;
        let turn = &event.envelope.turn;
        match event.envelope.kind.as_str() {
            "load" => {
                if let Some(depth) = event
                    .payload
                    .as_deref()
                    .and_then(|p| value_at(p, "field"))
                    .and_then(|raw| raw.get().parse::<u32>().ok())
                {
                    self.elected_depth = Some(depth);
                }
            }
            "replay.closed" => {
                if self.bracket_run.as_deref() == Some(run.as_str()) {
                    self.finish();
                    return Step::Done;
                }
            }
            "model.field" => {
                self.seen_field = true;
                // The counter follows the generation in flight: a field
                // from another run or turn starts a new count.
                let ordinal = match &mut self.counter {
                    Some((r, t, n)) if r == run && t == turn => {
                        let ordinal = *n;
                        *n += 1;
                        ordinal
                    }
                    _ => {
                        self.counter = Some((run.clone(), turn.clone(), 1));
                        0
                    }
                };
                if self.run.as_ref().is_some_and(|wanted| wanted != run) {
                    return Step::Continue;
                }
                if turn.as_deref() != Some(self.address.turn.as_str()) {
                    return Step::Continue;
                }
                let Some(payload) = event.payload.as_deref() else {
                    return Step::Continue;
                };
                let position =
                    value_at(payload, "position").and_then(|r| r.get().parse::<u64>().ok());
                if position != Some(self.address.position) {
                    return Step::Continue;
                }
                let (Some(ranked), Some(realized)) = (
                    value_at(payload, "ranked"),
                    value_at(payload, "realized").and_then(|r| r.get().parse::<u32>().ok()),
                ) else {
                    return Step::Continue;
                };
                // The realized rank places the draw among the candidates
                // where it is within them. The list is peeked for its
                // tokens and never re-rendered.
                let peeked: Vec<Peek> = serde_json::from_str(ranked.get()).unwrap_or_default();
                let drawn = peeked.get(realized as usize).map(|c| c.token);
                let answer = Answer {
                    run: run.clone(),
                    turn: self.address.turn.clone(),
                    position: self.address.position,
                    realized,
                    drawn,
                    ranked: ranked.to_owned(),
                };
                // A second match before the first paired leaves the first
                // with what it has rather than losing it.
                self.finish();
                if drawn.is_some() {
                    self.emit_now(answer);
                    if self.done() {
                        return Step::Done;
                    }
                } else {
                    self.pending = Some((answer, ordinal));
                }
            }
            "model.measurement" => {
                let in_flight = self
                    .counter
                    .as_ref()
                    .is_some_and(|(r, t, _)| r == run && t == turn);
                if in_flight {
                    // The generation closed: the next field starts a new
                    // count.
                    self.counter = None;
                }
                let pending_here = self.pending.as_ref().is_some_and(|(a, _)| {
                    &a.run == run && Some(a.turn.as_str()) == turn.as_deref()
                });
                if pending_here {
                    let (mut answer, ordinal) = self.pending.take().expect("checked");
                    answer.drawn = event
                        .payload
                        .as_deref()
                        .and_then(|p| value_at(p, "output_tokens"))
                        .and_then(|r| serde_json::from_str::<Vec<u32>>(r.get()).ok())
                        .and_then(|out| out.get(ordinal).copied());
                    self.emit_now(answer);
                    if self.done() {
                        return Step::Done;
                    }
                }
            }
            _ => {}
        }
        Step::Continue
    }
}
