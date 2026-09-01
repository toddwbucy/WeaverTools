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
                // Every exit past the open closes the bracket, the serving
                // close's own rule: the record channel is the sink and may
                // still hold what the decode channel lost.
                Err(TurnError::ChannelLost) => {
                    let _ = seat.replay_turn_stopped(&turn, weaver_trace::StopReason::Fault);
                    return Err(TurnError::ChannelLost);
                }
                // A seam refusal or fault mid-replay ends the pass where it
                // stopped: the refusal is already authored inside the
                // bracket, the bracket closes stopped, and no
                // `replay.closed` follows - the not-ended outcome is the
                // absence of one, per `weaver-diagnostic-Spec` section 3.
                Err(error) => {
                    let reason = match error {
                        TurnError::Refused { .. } => weaver_trace::StopReason::Refused,
                        _ => weaver_trace::StopReason::Fault,
                    };
                    let _ = seat.replay_turn_stopped(&turn, reason);
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

#[cfg(test)]
mod tests {
    //! The pass-behavior watches of `weaver-diagnostic-Spec` section 7,
    //! bought by the loop act: the seat is granted against a scripted
    //! custodian and a scripted decode peer, and the record on disk is what
    //! each watch reads.

    use std::io::{Read, Write};
    use std::os::fd::AsRawFd;
    use std::os::fd::OwnedFd;
    use std::os::unix::net::UnixStream;

    use nix::sys::socket::{AddressFamily, MsgFlags, SockFlag, SockType, recv, send, socketpair};

    use crate::authorship::Author;
    use crate::engine::Ports;
    use crate::record::Record;
    use weaver_types::SessionId;

    fn sink() -> (OwnedFd, std::path::PathBuf) {
        let path = std::env::temp_dir().join(format!(
            "weaver-replay-{}-{:?}.ndjson",
            std::process::id(),
            std::thread::current().id()
        ));
        let file = std::fs::File::create(&path).expect("sink");
        (OwnedFd::from(file), path)
    }

    fn diagnostic_record(sink: OwnedFd) -> Record {
        Record::Diagnostic(
            weaver_diagnostic::Recorder::receive(
                sink,
                weaver_diagnostic::RunRef("r-d".into()),
                weaver_diagnostic::SessionRef("s-d".into()),
            )
            .expect("the recorder receives"),
        )
    }

    fn listener() -> crate::channel::CoordinationListener {
        let dir = std::env::temp_dir().join(format!(
            "weaver-replay-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let path = dir.join("c.sock");
        std::fs::remove_file(&path).ok();
        crate::channel::bind_coordination(&path).expect("bind")
    }

    fn record_lines(path: &std::path::Path) -> Vec<serde_json::Value> {
        let mut text = String::new();
        std::fs::File::open(path)
            .expect("the record reopens")
            .read_to_string(&mut text)
            .expect("the record reads");
        text.lines()
            .map(|line| serde_json::from_str(line).expect("a record line parses"))
            .collect()
    }

    /// One valid recorded generation, as the custodian would answer it.
    fn sealed_answer() -> String {
        concat!(
            r#"{"answer":{"replay":{"events":["#,
            r#"{"envelope":{"kind":"model.request","run":"r-1","turn":"t-1","sequence":"4"},"#,
            r#""pairs":{"rendered":"hi","template":"tmpl","sampling":{"seed":37}}},"#,
            r#"{"envelope":{"kind":"model.measurement","run":"r-1","turn":"t-1","sequence":"6"},"#,
            r#""pairs":{"input_tokens":[1,2],"output_tokens":[3],"model":"art","weights_hash":"h"}}"#,
            r#"]}}}"#,
            "\n"
        )
        .to_string()
    }

    /// A drive against scripted peers: the custodian's answer (or a closed
    /// door), the decode peer's script, and the record read back.
    fn run_drive(
        custodian: Option<String>,
        decode_script: fn(std::os::fd::OwnedFd),
    ) -> (Result<(), crate::engine::TurnError>, Vec<serde_json::Value>) {
        let (sink_fd, path) = sink();
        let mut record = diagnostic_record(sink_fd);
        let session = SessionId("s-d".into());
        let author = Author::new(&session, &weaver_types::RunId("r-d".into()));

        let (ours, theirs) = UnixStream::pair().expect("state pair");
        ours.set_nonblocking(true).expect("nonblocking");
        let mut state = crate::state::StateSeam::new(ours);
        let state_peer = std::thread::spawn(move || {
            let mut peer = theirs;
            let mut taken = [0u8; 256];
            let Some(answer) = custodian else {
                let _ = peer.read(&mut taken);
                return;
            };
            let _ = peer.read(&mut taken).expect("reads the ask");
            peer.write_all(answer.as_bytes()).expect("answers");
            // Held open until the drive finishes with it.
            let _ = peer.read(&mut taken);
        });

        let (near, far) = socketpair(
            AddressFamily::Unix,
            SockType::SeqPacket,
            None,
            SockFlag::SOCK_CLOEXEC,
        )
        .expect("decode pair");
        let decode = crate::channel::decode_from_owned(near);
        let decode_peer = std::thread::spawn(move || decode_script(far));

        let coordination = listener();
        let mut turn_ordinal = 0u64;
        let mut fullness = None;
        let mut pressure_reported = false;
        let outcome = {
            let mut ports = Ports::grant(
                &decode,
                &author,
                &mut record,
                &mut turn_ordinal,
                None,
                &coordination,
                None,
                None,
                Some(&mut state),
                None,
                &mut fullness,
                &mut pressure_reported,
            );
            super::drive(&mut ports, false, "art")
        };
        drop(decode);
        drop(state);
        decode_peer.join().expect("the decode peer finishes");
        state_peer.join().expect("the custodian finishes");
        let lines = record_lines(&path);
        std::fs::remove_file(&path).ok();
        (outcome, lines)
    }

    fn idle_decode(_far: std::os::fd::OwnedFd) {}

    /// **The record identifies itself at the open, and an absent answer
    /// invents nothing.** The custodian's door closes without answering,
    /// and the record holds exactly the bracket: `replay.opened` first, no
    /// `replay.identity` filled from defaults, and the close carrying the
    /// abandoned outcome with the ask-unanswered reason.
    ///
    /// Perturbation: drop the opened authoring from the drive and the
    /// first-event assertion fails, the record opening with its close.
    /// Watched under exactly that removal.
    ///
    /// conforms: diagnostic-record-identifies-itself-at-the-open
    /// conforms: diagnostic-no-identity-invented
    #[test]
    fn an_absent_answer_abandons_and_the_record_opens_with_the_opened_kind() {
        let (outcome, lines) = run_drive(None, idle_decode);
        assert!(outcome.is_ok(), "an abandoned pass is not a fault");
        assert_eq!(
            lines[0]["kind"], "replay.opened",
            "the bracket opens with the opening kind"
        );
        assert!(
            lines.iter().all(|l| l["kind"] != "replay.identity"),
            "no identity is invented from an answer that never arrived"
        );
        let close = lines.last().expect("the close stands");
        assert_eq!(close["kind"], "replay.closed");
        assert_eq!(close["payload"]["outcome"]["kind"], "abandoned");
        assert_eq!(
            close["payload"]["outcome"]["reason"]["kind"],
            "replay_ask_unanswered"
        );
    }

    /// **Refused holdings author no identity either**, the case that bites:
    /// a refused reading has everything an identity event would carry and
    /// must still author nothing. The custodian answers a measurement with
    /// no preceding request, the grouping refuses, and the close names it.
    ///
    /// Perturbation: author a default-filled identity on the refusal path
    /// and this fails, the refused pass carrying an identity it never
    /// established. Watched under exactly that addition.
    ///
    /// conforms: diagnostic-no-identity-invented
    #[test]
    fn refused_holdings_author_no_identity() {
        let unpaired = concat!(
            r#"{"answer":{"replay":{"events":["#,
            r#"{"envelope":{"kind":"model.measurement","run":"r-1","turn":"t-1","sequence":"6"},"#,
            r#""pairs":{"input_tokens":[1,2],"output_tokens":[3],"model":"art","weights_hash":"h"}}"#,
            r#"]}}}"#,
            "\n"
        )
        .to_string();
        let (outcome, lines) = run_drive(Some(unpaired), idle_decode);
        assert!(outcome.is_ok());
        assert!(
            lines.iter().all(|l| l["kind"] != "replay.identity"),
            "a pass that refused its holdings authors none of these"
        );
        let close = lines.last().expect("the close stands");
        assert_eq!(close["payload"]["outcome"]["kind"], "abandoned");
        assert_eq!(close["payload"]["outcome"]["reason"]["kind"], "identity_refused");
        assert!(
            close["payload"]["outcome"]["reason"]["detail"]
                .as_str()
                .is_some_and(|d| d.contains("t-1")),
            "the refusal names the turn: {close}"
        );
    }

    /// **A pass that died manufactures no outcome.** The decode peer takes
    /// the re-feed directive and closes, the drive loses the channel, and
    /// the record ends where the replay stopped: the bracket closed
    /// stopped, and no `replay.closed` at all - the fourth outcome is the
    /// absence of the event.
    ///
    /// Perturbation: author a `replay.closed` on the channel-lost arm and
    /// this fails, a death path carrying an outcome. Watched under exactly
    /// that addition.
    ///
    /// conforms: diagnostic-no-outcome-manufactured
    #[test]
    fn a_dead_seam_mid_replay_manufactures_no_outcome() {
        fn takes_and_closes(far: std::os::fd::OwnedFd) {
            let mut buf = vec![0u8; 65536];
            let _ = recv(far.as_raw_fd(), &mut buf, MsgFlags::empty()).expect("takes the re-feed");
            drop(far);
        }
        let (outcome, lines) = run_drive(Some(sealed_answer()), takes_and_closes);
        assert!(
            matches!(outcome, Err(crate::engine::TurnError::ChannelLost)),
            "the loss surfaces to end service"
        );
        assert!(
            lines.iter().any(|l| l["kind"] == "replay.identity"),
            "the identity had been established before the death"
        );
        assert!(
            lines.iter().all(|l| l["kind"] != "replay.closed"),
            "a death path authors no outcome: the absence is the fourth outcome"
        );
        let close = lines.last().expect("the bracket still closed");
        assert_eq!(close["kind"], "turn.closed", "the turn bracket closed stopped");
        assert_eq!(close["payload"]["close"], "stopped");
    }

    /// **A matching replay certifies, the bracket mirrored whole, under
    /// the run's own session.** The decode peer answers the re-feed with
    /// the recorded path recomputed, the pass closes certified, and the
    /// identity payload names the replayed session by the one name the
    /// contract gives it. The envelope's session needs no watch here: the
    /// writer binds it by construction at submit, the crossing
    /// unrepresentable, and that crate's own record carries the claim -
    /// this test's per-line read is the loop confirming what construction
    /// already holds.
    ///
    /// Perturbation: fill `replayed_session` from anything but the
    /// declared session's name in the drive and the identity assertion
    /// fails. Watched under exactly that change.
    #[test]
    fn a_matching_replay_certifies_under_the_runs_own_session() {
        fn answers_refed(far: std::os::fd::OwnedFd) {
            let mut buf = vec![0u8; 65536];
            let n = recv(far.as_raw_fd(), &mut buf, MsgFlags::empty()).expect("takes the re-feed");
            let directive: serde_json::Value =
                serde_json::from_slice(&buf[..n]).expect("the directive parses");
            assert_eq!(directive["kind"], "re_feed", "the drive crossed: {directive}");
            let refed = concat!(
                r#"{"kind":"re_fed","body":{"emission":"hi","finish":"completed","#,
                r#""content":[{"type":"text","text":"hi"}],"#,
                r#""request":{"rendered":"hi","template":"tmpl","sampling":{"seed":37}},"#,
                r#""measurement":{"input_tokens":[1,2],"output_tokens":[3],"#,
                r#""model":"art","weights_hash":"h"},"#,
                r#""resident":6,"capacity":64}}"#
            );
            send(far.as_raw_fd(), refed.as_bytes(), MsgFlags::empty()).expect("answers");
        }
        let (outcome, lines) = run_drive(Some(sealed_answer()), answers_refed);
        assert!(outcome.is_ok());
        let close = lines.last().expect("the close stands");
        assert_eq!(close["kind"], "replay.closed");
        assert_eq!(
            close["payload"]["outcome"]["kind"], "certified",
            "the matching path certifies: {close}"
        );
        for line in &lines {
            assert_eq!(
                line["session"], "s-d",
                "every envelope is the run's own session: {line}"
            );
        }
        let identity = lines
            .iter()
            .find(|l| l["kind"] == "replay.identity")
            .expect("the identity stands");
        assert_eq!(
            identity["payload"]["replayed_session"], "s-d",
            "and the identity names the replayed session"
        );
        let kinds: Vec<&str> = lines.iter().filter_map(|l| l["kind"].as_str()).collect();
        assert_eq!(
            kinds,
            vec![
                "replay.opened",
                "replay.identity",
                "turn.started",
                "model.request",
                "model.output",
                "model.measurement",
                "message.assistant",
                "turn.closed",
                "replay.closed",
            ],
            "the certified pass authors the mirrored bracket whole"
        );
    }
}
