//! The diagnostic replay, per `diagnostic-replay-loop` and
//! `weaver-harness-Spec` section 6.2's second criterion: the seat granted
//! once at the run's opening, on the run itself as the work, the operator's
//! sealed preload what arrived owed an answer and the certification the
//! answer it is owed. The loop composes what the seat grants - the replay
//! ask, the decode surface, the record - and it mints no port.
//!
//! **This act is the null replay**: no reader pass, the certification of
//! the charter's step 2 alone. The recorded path re-feeds, the recomputed
//! token identifiers match the recorded ones exactly, integers, or the
//! certification fails naming the first divergent position. Every outcome
//! lands in the diagnostic-trace and nothing is answered outward, there
//! being no one on this seam to answer.

use weaver_diagnostic::{
    AbandonReason, Divergence, Kind, ModelId, Payload, ReplayClosed, ReplayIdentity,
    ReplayOpened, ReplayOutcome, TemplateId, TokenId, WeightsHash,
};
use weaver_types::TurnKey;

use crate::engine::{Ports, TurnError};
use crate::state::Recalled;

/// How long the replay ask waits on the seal, milliseconds. The operator
/// sequences the driver after the load, per the walk's three acts, so this
/// bound spans a human running a program rather than a socket answering,
/// and an expiry is the abandoned outcome rather than a fault.
const REPLAY_ASK_BOUND_MS: u64 = 600_000;

/// One recorded generation, paired from the holdings: the request's members
/// beside the measurement's, in the landing order the pairing rule fixes.
struct SourceGeneration {
    rendered: String,
    template: String,
    sampling: serde_json::Value,
    input_tokens: Vec<u32>,
    output_tokens: Vec<u32>,
    model: String,
    weights_hash: String,
}

/// The walk, whole: ask, group, establish identity, re-feed, compare,
/// close. Returns only on a channel-level failure the service cannot
/// survive: every outcome of the replay itself is the record's.
pub(crate) fn drive(
    seat: &mut Ports<'_>,
    reader_elected: bool,
    declared_artifact: &str,
) -> Result<(), TurnError> {
    // **The bracket opens first**, per `weaver-diagnostic-Spec` section 4:
    // a diagnostic-trace opens every bracket with `replay.opened`, which is
    // how a reader tells the two records apart, so it precedes even the
    // ask whose absence would abandon the pass.
    seat.author_replay(
        Kind::ReplayOpened,
        Payload::ReplayOpened(ReplayOpened { reader_elected }),
    )?;

    let Some(events) = seat.replay(REPLAY_ASK_BOUND_MS) else {
        // A leg that is down, an answer that is malformed, and a bound that
        // expired on a seal that never came are one outcome, per the loop
        // document's failure terms: no answer is no answer, and the account
        // is the record's.
        return close(
            seat,
            ReplayOutcome::Abandoned {
                reason: AbandonReason::ReplayAskUnanswered,
            },
        );
    };

    let turns = match group(&events) {
        Ok(turns) => turns,
        Err(detail) => return refuse_identity(seat, detail),
    };

    // **Input identity, from the answered holdings and the declared
    // binding**, per the charter's step 1: what is about to feed is what
    // the record says was fed, established before any forward pass. The
    // items the null replay's claim requires are each required here, and a
    // record missing one fails now, which is completeness being
    // claim-relative. The state claim's own item, the tee's election, is
    // not required: the null replay rests on the recorded identifiers
    // alone, per the charter's step 1, and this pass claims nothing about
    // the state.
    let first = turns
        .first()
        .and_then(|(_, generations)| generations.first());
    let Some(first) = first else {
        return refuse_identity(seat, "the holdings pair no generation".to_string());
    };
    if first.model != declared_artifact {
        return refuse_identity(
            seat,
            format!(
                "the record replays {} and the load declared {}",
                first.model, declared_artifact
            ),
        );
    }
    let identity = ReplayIdentity {
        replayed_session: seat.session_name(),
        model: ModelId(first.model.clone()),
        weights_hash: WeightsHash(first.weights_hash.clone()),
        template: TemplateId(first.template.clone()),
    };
    seat.author_replay(Kind::ReplayIdentity, Payload::ReplayIdentity(identity))?;

    // **The walk, by turn, by generation, in landing order.** Each re-feed
    // computes the draws the source would have computed, and the recorded
    // token appends whatever the draw said, so every later position stays
    // comparable behind a divergence, but the certification itself fails at
    // the first divergent position, per the charter's step 2, and the pass
    // closes naming it.
    for (turn_key, generations) in &turns {
        let turn = TurnKey(turn_key.clone());
        seat.replay_turn_started(&turn)?;
        for source in generations {
            let refed = match seat.refeed(&turn, source.rendered.clone(), source.output_tokens.clone())
            {
                Ok(generation) => generation,
                Err(TurnError::ChannelLost) => return Err(TurnError::ChannelLost),
                // A seam refusal or fault mid-replay ends the pass where it
                // stopped: the refusal is already authored inside the
                // bracket, the bracket closes, and no `replay.closed`
                // follows - the not-ended outcome is the absence of one,
                // per `weaver-diagnostic-Spec` section 3.
                Err(_) => {
                    let _ = seat.replay_turn_closed(&turn);
                    return Ok(());
                }
            };
            match compare(source, &refed) {
                Comparison::Matches => {}
                Comparison::Diverged(divergence) => {
                    seat.replay_turn_closed(&turn)?;
                    return close(seat, ReplayOutcome::Diverged { divergence });
                }
                Comparison::IdentityBroken(detail) => {
                    seat.replay_turn_closed(&turn)?;
                    return refuse_identity(seat, detail);
                }
            }
        }
        seat.replay_turn_closed(&turn)?;
    }

    close(seat, ReplayOutcome::Certified)
}

fn close(seat: &mut Ports<'_>, outcome: ReplayOutcome) -> Result<(), TurnError> {
    seat.author_replay(Kind::ReplayClosed, Payload::ReplayClosed(ReplayClosed { outcome }))
}

fn refuse_identity(seat: &mut Ports<'_>, detail: String) -> Result<(), TurnError> {
    close(
        seat,
        ReplayOutcome::Abandoned {
            reason: AbandonReason::IdentityRefused { detail },
        },
    )
}

/// The grouping rule of the loop document's section 2: events group by run
/// and turn from their envelopes, in landing order, requests pairing to the
/// first unpaired measurement after them within one turn, and a grouping
/// the record does not determine rejects the replay before any forward
/// pass, naming what failed.
///
/// **One run is this pass's bound, named rather than silent**: the decode
/// session opened once at this run's enter, and a second recorded run
/// opened a second session the replay has no fresh context for, so
/// holdings spanning runs refuse at identity instead of replaying a
/// conversation across a boundary the source never crossed.
fn group(events: &[Recalled]) -> Result<Vec<(String, Vec<SourceGeneration>)>, String> {
    let mut run: Option<&str> = None;
    let mut order: Vec<String> = Vec::new();
    let mut pending: std::collections::BTreeMap<String, PendingRequest> = Default::default();
    let mut paired: std::collections::BTreeMap<String, Vec<SourceGeneration>> = Default::default();
    for event in events {
        match run {
            None => run = Some(&event.run),
            Some(held) if held == event.run => {}
            Some(held) => {
                return Err(format!(
                    "the holdings span runs {held} and {}, and this pass replays one",
                    event.run
                ));
            }
        }
        match event.kind.as_str() {
            "model.request" => {
                let Some(turn) = &event.turn else {
                    return Err("a model.request carries no turn".to_string());
                };
                if !order.contains(turn) {
                    order.push(turn.clone());
                } else if order.last() != Some(turn) {
                    return Err(format!(
                        "turn {turn} resumes after another turn intervened"
                    ));
                }
                if pending.contains_key(turn) {
                    return Err(format!(
                        "two model.request events stand unpaired in turn {turn}"
                    ));
                }
                pending.insert(turn.clone(), request_members(event, turn)?);
            }
            "model.measurement" => {
                let Some(turn) = &event.turn else {
                    return Err("a model.measurement carries no turn".to_string());
                };
                let Some(request) = pending.remove(turn) else {
                    return Err(format!(
                        "a model.measurement in turn {turn} has no preceding unpaired request"
                    ));
                };
                paired
                    .entry(turn.clone())
                    .or_default()
                    .push(measurement_members(event, turn, request)?);
            }
            // Turnless events - the run brackets, the seated prefix, a
            // flush, the load - inform identity and feed nothing
            // positionally, and kinds this walk does not read pass by, per
            // the versionless-schema rule.
            _ => {}
        }
    }
    if let Some((turn, _)) = pending.iter().next() {
        return Err(format!("a model.request in turn {turn} pairs with nothing"));
    }
    let mut grouped = Vec::new();
    for turn in order {
        let generations = paired.remove(&turn).unwrap_or_default();
        if generations.is_empty() {
            return Err(format!("turn {turn} pairs no generation"));
        }
        grouped.push((turn, generations));
    }
    Ok(grouped)
}

/// The request's elected members, half a [`SourceGeneration`] until its
/// measurement lands.
struct PendingRequest {
    rendered: String,
    template: String,
    sampling: serde_json::Value,
}

fn pair<'a>(event: &'a Recalled, key: &str) -> Option<&'a str> {
    event
        .pairs
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.as_str())
}

fn request_members(event: &Recalled, turn: &str) -> Result<PendingRequest, String> {
    let missing = |key: &str| format!("the model.request of turn {turn} holds no {key}");
    let rendered = pair(event, "rendered").ok_or_else(|| missing("rendered"))?;
    let rendered: String = serde_json::from_str(rendered)
        .map_err(|_| format!("the rendered form of turn {turn} does not parse"))?;
    let template = pair(event, "template").ok_or_else(|| missing("template"))?;
    let template: String = serde_json::from_str(template)
        .map_err(|_| format!("the template of turn {turn} does not parse"))?;
    let sampling = pair(event, "sampling").ok_or_else(|| missing("sampling"))?;
    let sampling: serde_json::Value = serde_json::from_str(sampling)
        .map_err(|_| format!("the sampling of turn {turn} does not parse"))?;
    Ok(PendingRequest {
        rendered,
        template,
        sampling,
    })
}

fn measurement_members(
    event: &Recalled,
    turn: &str,
    request: PendingRequest,
) -> Result<SourceGeneration, String> {
    let missing = |key: &str| format!("the model.measurement of turn {turn} holds no {key}");
    let parse_tokens = |raw: &str, key: &str| -> Result<Vec<u32>, String> {
        serde_json::from_str(raw).map_err(|_| format!("the {key} of turn {turn} do not parse"))
    };
    let input = pair(event, "input_tokens").ok_or_else(|| missing("input_tokens"))?;
    let output = pair(event, "output_tokens").ok_or_else(|| missing("output_tokens"))?;
    let model = pair(event, "model").ok_or_else(|| missing("model"))?;
    let model: String =
        serde_json::from_str(model).map_err(|_| format!("the model of turn {turn} does not parse"))?;
    let weights_hash = pair(event, "weights_hash").ok_or_else(|| missing("weights_hash"))?;
    let weights_hash: String = serde_json::from_str(weights_hash)
        .map_err(|_| format!("the weights hash of turn {turn} does not parse"))?;
    let output_tokens = parse_tokens(output, "output_tokens")?;
    if output_tokens.is_empty() {
        return Err(format!("turn {turn} records a generation with no output"));
    }
    Ok(SourceGeneration {
        rendered: request.rendered,
        template: request.template,
        sampling: request.sampling,
        input_tokens: parse_tokens(input, "input_tokens")?,
        output_tokens,
        model,
        weights_hash,
    })
}

enum Comparison {
    Matches,
    Diverged(Divergence),
    IdentityBroken(String),
}

/// One generation's certification: the re-fed answer against the recorded
/// members. The token comparison is exact, integers, and the first
/// divergent position names both identifiers. **Positions count through
/// the generation's forward**: the appended input first, then the draws,
/// so a tokenization divergence and a draw divergence land on one scale
/// and a reader can place either against the measurement's own vectors.
fn compare(source: &SourceGeneration, refed: &weaver_types::Generation) -> Comparison {
    let measurement: serde_json::Value = match serde_json::from_str(refed.measurement.get()) {
        Ok(value) => value,
        Err(_) => return Comparison::IdentityBroken("the re-fed measurement does not parse".into()),
    };
    let request: serde_json::Value = match serde_json::from_str(refed.request.get()) {
        Ok(value) => value,
        Err(_) => return Comparison::IdentityBroken("the re-fed request does not parse".into()),
    };
    let tokens = |value: &serde_json::Value, key: &str| -> Option<Vec<u32>> {
        serde_json::from_value(value.get(key)?.clone()).ok()
    };
    let Some(refed_input) = tokens(&measurement, "input_tokens") else {
        return Comparison::IdentityBroken("the re-fed measurement holds no input_tokens".into());
    };
    let Some(refed_output) = tokens(&measurement, "output_tokens") else {
        return Comparison::IdentityBroken("the re-fed measurement holds no output_tokens".into());
    };
    // The identity items whose live values only a pass can produce: the
    // weights hash and the template ride the re-fed answer, and the
    // sampling block must be the values the record says the source drew
    // under, the derived seed among them.
    if measurement.get("weights_hash")
        != Some(&serde_json::Value::String(source.weights_hash.clone()))
    {
        return Comparison::IdentityBroken(format!(
            "the loaded weights hash {:?} is not the recorded {}",
            measurement.get("weights_hash"),
            source.weights_hash
        ));
    }
    if request.get("template") != Some(&serde_json::Value::String(source.template.clone())) {
        return Comparison::IdentityBroken(format!(
            "the loaded template {:?} is not the recorded {}",
            request.get("template"),
            source.template
        ));
    }
    if request.get("sampling") != Some(&source.sampling) {
        return Comparison::IdentityBroken(format!(
            "the effective sampling {:?} is not the recorded {}",
            request.get("sampling"),
            source.sampling
        ));
    }
    // Tokenization identity: the rendered form re-tokenized must be the
    // recorded appended input, per the loop document's re-feed clause,
    // exercised rather than assumed.
    for (position, (recorded, recomputed)) in
        source.input_tokens.iter().zip(refed_input.iter()).enumerate()
    {
        if recorded != recomputed {
            return Comparison::Diverged(Divergence::TokenPath {
                position: position as u64,
                recorded: TokenId(*recorded),
                recomputed: TokenId(*recomputed),
            });
        }
    }
    if source.input_tokens.len() != refed_input.len() {
        return Comparison::Diverged(Divergence::TokenPath {
            position: source.input_tokens.len().min(refed_input.len()) as u64,
            recorded: TokenId(
                source
                    .input_tokens
                    .get(refed_input.len())
                    .copied()
                    .unwrap_or(0),
            ),
            recomputed: TokenId(refed_input.get(source.input_tokens.len()).copied().unwrap_or(0)),
        });
    }
    // **The null comparison itself**: the recomputed draws in the output
    // slots against the recorded path, exactly, integers.
    let base = source.input_tokens.len() as u64;
    for (ordinal, (recorded, recomputed)) in
        source.output_tokens.iter().zip(refed_output.iter()).enumerate()
    {
        if recorded != recomputed {
            return Comparison::Diverged(Divergence::TokenPath {
                position: base + ordinal as u64,
                recorded: TokenId(*recorded),
                recomputed: TokenId(*recomputed),
            });
        }
    }
    if source.output_tokens.len() != refed_output.len() {
        return Comparison::Diverged(Divergence::TokenPath {
            position: base + source.output_tokens.len().min(refed_output.len()) as u64,
            recorded: TokenId(
                source
                    .output_tokens
                    .get(refed_output.len())
                    .copied()
                    .unwrap_or(0),
            ),
            recomputed: TokenId(
                refed_output
                    .get(source.output_tokens.len())
                    .copied()
                    .unwrap_or(0),
            ),
        });
    }
    Comparison::Matches
}
