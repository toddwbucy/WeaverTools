//! conforms: harness-loop-mints-no-port
//! conforms: harness-extension-seam-at-loaded-and-idle
//! conforms: harness-turn-authors-the-model-events
//! conforms: harness-stop-polled-during-the-stream
//!
//! Loop 1's seat and the decode surface it composes, per `weaver-harness-Spec`
//! sections 6 and 6.1. The loop itself is the builder's, written at the worker
//! composition root and compiled into the worker binary. What this crate holds
//! is the seat and the granted surface the loop composes against.
//!
//! **The extension seam is crossed at loaded-and-idle itself**: loop 0 hands a
//! standing interior to whatever loop 1 the binary carries, and takes it back
//! at the stop and at the leave, the bracket discipline being loop 0's for
//! every loop alike. The interior is handed as a borrow, so a loop holds the
//! surface for exactly as long as loop 0 lends it and cannot outlive the run.
//!
//! **The blade is structural.** A port is a type this crate owns with a
//! constructor no consumer can reach, so a loop composes against the granted
//! surface or does not compile. There is no call by which a loop mints a port:
//! [`Ports`] has private fields and a crate-private constructor, and a loop
//! that needs a port this surface does not offer is a capability change
//! entering through the front door as a charter and contract edit.
//!
//! **The turn is loop 0's machinery, granted.** Loop 1 supplies the delta and
//! receives the outcome, and loop 0 drives the append-and-generate exchange,
//! consumes the stream, reads the close, and authors the record. Loop 1
//! authors nothing, which is the sole-writer property held where the turn runs.

use weaver_trace::{Kind, ModelOutput, Payload, Recorder, Subsystem, TurnClose};
use weaver_traits::{ContentBlock, Message, Role};
use weaver_types::{TokenAnswer, TokenDirective, TokenRefusal, TurnKey};

use crate::assembly::Prompt;
use crate::authorship::Author;
use crate::channel::{CoordinationListener, DecodeChannel, OrganChannel};

/// The granted surface handed to loop 1 at loaded-and-idle, borrowing the
/// standing interior for the seat's lifetime. Its fields are private and its
/// only constructor is crate-private, which is the blade.
pub struct Ports<'a> {
    decode: &'a DecodeChannel,
    author: &'a Author,
    recorder: &'a mut Recorder,
    turn_ordinal: &'a mut u64,
    assembled: Option<Prompt>,
    /// The coordination listener, waited against beside the decode channel
    /// while a generation streams, per Spec 6.1: the stop is heard
    /// mid-stream by `poll`. Private, so the seat carries the ear without
    /// granting the loop a port.
    coordination: &'a CoordinationListener,
    /// The verb connection being served, shared with loop 0's own wait: a
    /// dial accepted mid-stream lands here and is handed back when the
    /// turn returns. `None` for a seat granted outside the serve loop,
    /// which then streams without the ear.
    pending: Option<&'a mut Option<OrganChannel>>,
}

/// Why a turn did not complete. A refusal the seam typed is the session
/// declining the ask, and a fault below the exchange is the worker gone or the
/// octets unreadable, per the decode contract's closure rule.
#[derive(Debug)]
pub enum TurnError {
    /// The decode seam typed a refusal to the append.
    Refused(TokenRefusal),
    /// The channel faulted or answered something the turn cannot read.
    ChannelLost,
    /// A message loop 1 supplied is not licensed for its role, so no event
    /// authors and the turn does not run.
    Unlicensed,
}

/// What a completed turn produced for loop 1: the emission verbatim and how it
/// ended. The record holds more, but this is what the reasoning loop reads to
/// decide its next move.
#[derive(Debug, Clone)]
pub struct TurnOutcome {
    pub emission: String,
    pub stopped: bool,
    /// The turn was aborted by the operator's stop rather than running to
    /// its own close: the bracket closed with the directive's reason and
    /// the partial stands. A model-side stop is not this, per the close's
    /// own distinction.
    pub aborted: bool,
}

impl<'a> Ports<'a> {
    /// Crate-private: no consumer can reach it, which makes the blade a
    /// compile property. Loop 0 calls it at loaded-and-idle, lending the
    /// interior across the extension seam.
    pub(crate) fn grant(
        decode: &'a DecodeChannel,
        author: &'a Author,
        recorder: &'a mut Recorder,
        turn_ordinal: &'a mut u64,
        assembled: Option<Prompt>,
        coordination: &'a CoordinationListener,
        pending: Option<&'a mut Option<OrganChannel>>,
    ) -> Self {
        Ports {
            decode,
            author,
            recorder,
            turn_ordinal,
            assembled,
            coordination,
            pending,
        }
    }

    /// The prompt loop 0 assembled from the working structure, a read loop 1
    /// has over the granted surface.
    pub fn assembled(&self) -> Option<&Prompt> {
        self.assembled.as_ref()
    }

    /// **Run one turn**, per `weaver-harness-Spec` section 6.1. Loop 1 supplies
    /// the delta as canonical messages, and loop 0 drives the exchange and
    /// authors the record: the turn bracket, the delta as message events, the
    /// three model events across the boundary, and the assistant's turn as a
    /// message event. Loop 1 authors nothing.
    ///
    /// The seam is append-only, so only the delta crosses, the SPU holding the
    /// resident session from the open at enter. The model events splice what
    /// the SPU rendered, the request and the measurement carried opaque and the
    /// output shaped from the emission and finish this crate consumes.
    pub fn turn(&mut self, delta: Vec<Message>) -> Result<TurnOutcome, TurnError> {
        *self.turn_ordinal += 1;
        let turn = TurnKey(format!("t-{}", self.turn_ordinal));

        // The bracket opens. A failure to open it leaves no bracket to close,
        // so it returns before there is anything to unwind.
        self.author
            .author(
                self.recorder,
                Kind::TurnStarted,
                Subsystem::Harness,
                Some(&turn),
                None,
            )
            .map_err(|_| TurnError::ChannelLost)?;

        // **Every exit past the open closes the bracket.** A turn that opened
        // and then lost the channel or met a refusal must not leave a
        // `turn.started` without its `turn.closed`, which a consumer pairing
        // the bracket would read as a turn that never ended. The body runs to
        // its own close on success, and a failure closes with the fault reason
        // before the error returns.
        match self.run_turn(&turn, delta) {
            Ok(outcome) => Ok(outcome),
            Err(error) => {
                // **A failed close is itself a failure the caller must learn.**
                // The recorder can be broken when the turn fails, so authoring
                // the fault close can fail too, and swallowing that would leave
                // a `turn.started` with no `turn.closed` and no error naming it.
                // The original error is returned only when the close lands, and
                // a close that cannot author returns `ChannelLost` because the
                // record is now untrustworthy whatever the first cause was.
                match self.author.author(
                    self.recorder,
                    Kind::TurnClosed,
                    Subsystem::Harness,
                    Some(&turn),
                    Some(Payload::TurnClosed(TurnClose::Stopped {
                        reason: weaver_trace::StopReason::Fault,
                    })),
                ) {
                    Ok(_) => Err(error),
                    Err(_) => Err(TurnError::ChannelLost),
                }
            }
        }
    }

    /// The turn's body, run inside the bracket [`turn`] opens and closes. It
    /// authors the delta, drives the exchange, and authors the answer, and
    /// every error it returns is closed by the caller.
    fn run_turn(&mut self, turn: &TurnKey, delta: Vec<Message>) -> Result<TurnOutcome, TurnError> {
        // The delta is authored as the turn's user messages, before the
        // exchange so the record reads the ask before the answer.
        for message in &delta {
            self.author
                .author_message(self.recorder, message, turn)
                .map_err(|_| TurnError::Unlicensed)?
                .map_err(|_| TurnError::ChannelLost)?;
        }

        // Append and generate. The delta crosses, the SPU appends it at the
        // resident end, and the stream returns each token before the close.
        self.decode
            .send_directive(&TokenDirective::AppendAndGenerate {
                turn: turn.clone(),
                delta,
                tunable: std::collections::BTreeMap::new(),
            })
            .map_err(|_| TurnError::ChannelLost)?;

        // Consume the stream to the close, hearing the stop while it runs,
        // per Spec 6.1: `poll` sleeps against the decode channel, the
        // coordination listener, and the verb connection if one stands, and
        // wakes on the first ready. A stop dialed mid-stream cancels the
        // turn at the seam, the outstanding generation answering with its
        // partial marked stopped and the tokens already streamed standing
        // in the close. A dialer that connects and then sends nothing holds
        // only its own connection, never the streaming turn.
        let mut stop: Option<weaver_types::ExchangeId> = None;
        let generation = loop {
            match self.stream_wake()? {
                StreamWake::Decode => match self
                    .decode
                    .recv_reply()
                    .map_err(|_| TurnError::ChannelLost)?
                {
                    crate::channel::DecodeReply::Answer(TokenAnswer::Token { .. }) => continue,
                    crate::channel::DecodeReply::Answer(TokenAnswer::Generated(generation)) => {
                        break generation;
                    }
                    crate::channel::DecodeReply::Answer(TokenAnswer::AtRest) => continue,
                    crate::channel::DecodeReply::Refusal(refusal) => {
                        return Err(TurnError::Refused(refusal));
                    }
                    crate::channel::DecodeReply::Answer(_) => return Err(TurnError::ChannelLost),
                },
                StreamWake::Dial => {
                    let Some(slot) = self.pending.as_deref_mut() else {
                        continue;
                    };
                    match self.coordination.accept_root() {
                        Ok(connection) => *slot = Some(connection),
                        // A refused peer never reaches an exchange, and the
                        // stream continues.
                        Err(crate::failure::ChannelFault::WrongPeer { .. }) => {}
                        Err(_) => return Err(TurnError::ChannelLost),
                    }
                }
                StreamWake::Directive => {
                    let Some(slot) = self.pending.as_deref_mut() else {
                        continue;
                    };
                    let Some(connection) = slot.as_ref() else {
                        continue;
                    };
                    match connection.recv() {
                        Ok(envelope) => {
                            let exchange = envelope.exchange.clone();
                            match envelope.payload {
                                weaver_types::Payload::Directive(
                                    weaver_types::LifecycleDirective::Stop,
                                ) if stop.is_none() => {
                                    // The cancel crosses at once, and the
                                    // stream is consumed to the close it
                                    // will produce, the answer owed after
                                    // the record, not here.
                                    self.decode
                                        .send_directive(&TokenDirective::Cancel {
                                            turn: turn.clone(),
                                        })
                                        .map_err(|_| TurnError::ChannelLost)?;
                                    stop = Some(exchange);
                                }
                                // A second stop, a leave, and everything
                                // else are out of order for a turn in
                                // flight, refused and not queued.
                                weaver_types::Payload::Directive(
                                    weaver_types::LifecycleDirective::Leave,
                                ) => {
                                    let _ = connection.send(&weaver_types::OrganEnvelope {
                                        exchange,
                                        position: weaver_types::Position::Close,
                                        payload: weaver_types::Payload::Refusal(
                                            weaver_types::LifecycleRefusal::ActivityNotAtRest,
                                        ),
                                    });
                                }
                                weaver_types::Payload::Directive(_) => {
                                    let _ = connection.send(&weaver_types::OrganEnvelope {
                                        exchange,
                                        position: weaver_types::Position::Close,
                                        payload: weaver_types::Payload::Refusal(
                                            weaver_types::LifecycleRefusal::OutOfOrder,
                                        ),
                                    });
                                }
                                _ => return Err(TurnError::ChannelLost),
                            }
                        }
                        Err(crate::failure::ChannelFault::Closed) => {
                            // The dialer left, and the stream continues.
                            *slot = None;
                        }
                        Err(_) => return Err(TurnError::ChannelLost),
                    }
                }
            }
        };

        // The three model events author across the boundary, subsystem SPU
        // because the SPU produced them, each spliced or shaped by the custody
        // model: the request and the measurement carried opaque, the output
        // shaped from the emission and finish.
        self.author
            .author(
                self.recorder,
                Kind::ModelRequest,
                Subsystem::Spu,
                Some(turn),
                Some(Payload::ModelRequest(generation.request)),
            )
            .map_err(|_| TurnError::ChannelLost)?;
        let stopped = matches!(generation.finish, weaver_types::Finish::Stopped);
        self.author
            .author(
                self.recorder,
                Kind::ModelOutput,
                Subsystem::Spu,
                Some(turn),
                Some(Payload::ModelOutput(ModelOutput {
                    emission: generation.emission.clone(),
                    finish: if stopped {
                        weaver_trace::Finish::Stopped
                    } else {
                        weaver_trace::Finish::Completed
                    },
                })),
            )
            .map_err(|_| TurnError::ChannelLost)?;
        self.author
            .author(
                self.recorder,
                Kind::ModelMeasurement,
                Subsystem::Spu,
                Some(turn),
                Some(Payload::ModelMeasurement(generation.measurement)),
            )
            .map_err(|_| TurnError::ChannelLost)?;

        // The assistant's turn enters the record as a message event, the
        // canonical parse beside the verbatim the output holds. A plain
        // response is text, and family-aware parsing arrives with the tool
        // workflow that gives the emission structure worth extracting.
        let assistant = Message {
            role: Role::Assistant,
            content: vec![ContentBlock::Text {
                text: generation.emission.clone(),
            }],
        };
        self.author
            .author_message(self.recorder, &assistant, turn)
            .map_err(|_| TurnError::Unlicensed)?
            .map_err(|_| TurnError::ChannelLost)?;

        // **The close names what ended the turn.** A model-side stop, the
        // generation reaching capacity or its own stop token, is a completed
        // turn whose truncation is recorded in the output's finish and the
        // bracket closes clean. A turn the operator's stop cancelled closes
        // with the directive's reason, the partial standing. The cancel
        // losing the race to a natural completion is the first case: the
        // turn completed, and the stop is answered at rest.
        let aborted = stop.is_some() && stopped;
        self.author
            .author(
                self.recorder,
                Kind::TurnClosed,
                Subsystem::Harness,
                Some(turn),
                Some(Payload::TurnClosed(if aborted {
                    TurnClose::Stopped {
                        reason: weaver_trace::StopReason::Directive,
                    }
                } else {
                    TurnClose::Clean
                })),
            )
            .map_err(|_| TurnError::ChannelLost)?;

        // **Announce after record**: the stop's answer follows the close it
        // reports, carrying the turn's fate, aborted or completed-at-rest,
        // both truthful at the moment of answering.
        if let Some(exchange) = stop
            && let Some(slot) = self.pending.as_deref_mut()
            && let Some(connection) = slot.as_ref()
        {
            let answer = if aborted {
                weaver_types::LifecycleAnswer::TurnAborted { turn: turn.clone() }
            } else {
                weaver_types::LifecycleAnswer::AtRest
            };
            let _ = connection.send(&weaver_types::OrganEnvelope {
                exchange,
                position: weaver_types::Position::Close,
                payload: weaver_types::Payload::Answer(answer),
            });
        }

        Ok(TurnOutcome {
            emission: generation.emission,
            stopped,
            aborted,
        })
    }

    /// One wake from the streaming wait, per Spec 6.1: the decode channel
    /// first, then the verb connection if one stands, then the listener
    /// while none does, the same serial discipline as the idle wait's.
    fn stream_wake(&self) -> Result<StreamWake, TurnError> {
        use std::os::fd::AsFd;

        use nix::poll::{PollFd, PollFlags, PollTimeout, poll};
        loop {
            let mut fds: Vec<PollFd<'_>> = Vec::with_capacity(3);
            let mut wakes: Vec<StreamWake> = Vec::with_capacity(3);
            fds.push(PollFd::new(self.decode.as_fd(), PollFlags::POLLIN));
            wakes.push(StreamWake::Decode);
            match self.pending.as_ref().map(|slot| slot.as_ref()) {
                Some(Some(connection)) => {
                    fds.push(PollFd::new(connection.as_fd(), PollFlags::POLLIN));
                    wakes.push(StreamWake::Directive);
                }
                Some(None) => {
                    fds.push(PollFd::new(self.coordination.as_fd(), PollFlags::POLLIN));
                    wakes.push(StreamWake::Dial);
                }
                // A seat granted outside the serve loop streams without the
                // ear, the slot being the loop's own.
                None => {}
            }
            match poll(&mut fds, PollTimeout::NONE) {
                Ok(_) => {}
                Err(nix::errno::Errno::EINTR) => continue,
                Err(_) => return Err(TurnError::ChannelLost),
            }
            let woken = PollFlags::POLLIN | PollFlags::POLLHUP | PollFlags::POLLERR;
            for (fd, wake) in fds.iter().zip(&wakes) {
                let revents = fd.revents().unwrap_or(PollFlags::empty());
                if revents.contains(PollFlags::POLLNVAL) {
                    return Err(TurnError::ChannelLost);
                }
                if revents.intersects(woken) {
                    return Ok(*wake);
                }
            }
        }
    }
}

/// What the streaming wait woke on.
#[derive(Clone, Copy)]
enum StreamWake {
    Decode,
    Dial,
    Directive,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::fd::{AsRawFd, OwnedFd};

    use nix::sys::socket::{AddressFamily, MsgFlags, SockFlag, SockType, recv, send, socketpair};
    use weaver_trace::{Recorder, RunOrdinal, SessionRef};
    use weaver_types::{Finish, Generation, SessionId};

    /// **A turn runs, and loop 0 authors the whole bracket.** Loop 1 supplies
    /// the delta and receives the outcome, and the record carries the turn's
    /// user message, the three model events, the assistant's turn, and the
    /// close, in that order. The decode peer is scripted here rather than a
    /// real SPU: it streams two tokens and then the generation whole, which is
    /// what the turn method consumes and authors.
    #[test]
    fn a_turn_authors_the_whole_bracket() {
        let (near, far) = socketpair(
            AddressFamily::Unix,
            SockType::SeqPacket,
            None,
            SockFlag::SOCK_CLOEXEC,
        )
        .expect("socketpair");
        let decode = crate::channel::decode_from_owned(near);

        // The scripted decode peer: read the append, stream two tokens, then
        // answer with the generation whole, the request and measurement the
        // SPU-rendered blobs the turn splices.
        let peer = std::thread::spawn(move || {
            let mut buf = vec![0u8; 65536];
            let n = recv(far.as_raw_fd(), &mut buf, MsgFlags::empty()).expect("recv append");
            let directive: weaver_types::TokenDirective =
                serde_json::from_slice(&buf[..n]).expect("the append parses");
            assert!(
                matches!(
                    directive,
                    weaver_types::TokenDirective::AppendAndGenerate { .. }
                ),
                "the turn sends append-and-generate"
            );
            let send_answer = |answer: &weaver_types::TokenAnswer| {
                let bytes = serde_json::to_vec(answer).expect("answer renders");
                send(far.as_raw_fd(), &bytes, MsgFlags::empty()).expect("send answer");
            };
            send_answer(&weaver_types::TokenAnswer::Token {
                token: 9707,
                piece: "one".into(),
            });
            send_answer(&weaver_types::TokenAnswer::Token {
                token: 1879,
                piece: " word".into(),
            });
            let request = serde_json::value::RawValue::from_string(
                r#"{"rendered":"<|im_start|>user\nsay a word<|im_end|>\n","template":"qwen2","sampling":{"temperature":0.7}}"#
                    .to_string(),
            )
            .unwrap();
            let measurement = serde_json::value::RawValue::from_string(
                r#"{"model":"qwen2.5","weights_hash":"sha256:abc","input_tokens":[1,2],"output_tokens":[9707,1879],"blocks":[{"label":"turn-delta","start":0,"end":10}],"timings":{"prefill_ns":"10","decode_ns":"20"}}"#
                    .to_string(),
            )
            .unwrap();
            send_answer(&weaver_types::TokenAnswer::Generated(Generation {
                emission: "one word".into(),
                finish: Finish::Completed,
                request,
                measurement,
            }));
        });

        let session = SessionId("s-1".into());
        let sink = tempfile();
        let mut recorder = Recorder::receive(sink, RunOrdinal(0), SessionRef(session.0.clone()))
            .expect("recorder");
        let author = Author::new(&session, 0);
        author
            .author(&mut recorder, Kind::Load, Subsystem::Harness, None, None)
            .expect("load");
        let mut turn_ordinal = 0u64;

        let listener = test_listener();
        let outcome = {
            let mut ports = Ports::grant(
                &decode,
                &author,
                &mut recorder,
                &mut turn_ordinal,
                None,
                &listener,
                None,
            );
            let delta = vec![Message {
                role: Role::User,
                content: vec![ContentBlock::Text {
                    text: "say a word".into(),
                }],
            }];
            ports.turn(delta).expect("the turn completes")
        };
        peer.join().expect("the decode peer finishes");

        assert_eq!(outcome.emission, "one word", "loop 1 receives the emission");
        assert!(!outcome.stopped, "a clean generation is not stopped");

        // The bracket, in order: the turn's user message, the three model
        // events, the assistant's turn, and the close.
        let kinds: Vec<Kind> = recorder
            .structure()
            .iter()
            .filter(|r| r.turn.is_some())
            .map(|r| r.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                Kind::TurnStarted,
                Kind::MessageUser,
                Kind::ModelRequest,
                Kind::ModelOutput,
                Kind::ModelMeasurement,
                Kind::MessageAssistant,
                Kind::TurnClosed,
            ],
            "the whole bracket authored in order"
        );

        // The model events splice what the peer rendered: the request carries
        // the template, the measurement the weights hash, both opaque.
        let line_of = |kind: Kind| {
            recorder
                .structure()
                .by_kind(kind)
                .next()
                .expect("the event authored")
                .line
                .to_string()
        };
        assert!(
            line_of(Kind::ModelRequest).contains("\"template\":\"qwen2\""),
            "the request splice carries what the SPU rendered"
        );
        assert!(
            line_of(Kind::ModelMeasurement).contains("sha256:abc"),
            "the measurement splice carries what the SPU rendered"
        );
    }

    /// **The stop is heard mid-stream, per Spec 6.1.** The streaming wait
    /// spans the decode channel and the verb connection at once, the stop
    /// cancels the turn at the seam, the partial stands in the close with
    /// the directive's reason, and the stop is answered with the turn's
    /// fate after the record. Perturbation: collapse the streaming wait to
    /// the decode channel alone and the cancel this scripted peer waits
    /// for never arrives.
    #[test]
    fn a_stop_dialed_mid_stream_cancels_the_turn() {
        let (near, far) = socketpair(
            AddressFamily::Unix,
            SockType::SeqPacket,
            None,
            SockFlag::SOCK_CLOEXEC,
        )
        .expect("socketpair");
        let decode = crate::channel::decode_from_owned(near);
        let listener = test_listener();
        let (verb_end, admin_end) = crate::channel::OrganChannel::pair().expect("pair");
        let admin = admin_end.into_channel();

        // The operator's stop is already on the connection when the turn
        // begins, the way the poll's readiness would deliver it mid-stream.
        admin
            .send(&weaver_types::OrganEnvelope {
                exchange: weaver_types::ExchangeId {
                    opener: weaver_types::Opener::Admin,
                    ordinal: 9,
                },
                position: weaver_types::Position::Open,
                payload: weaver_types::Payload::Directive(weaver_types::LifecycleDirective::Stop),
            })
            .expect("the stop sends");

        let peer = std::thread::spawn(move || {
            let mut buf = vec![0u8; 65536];
            let n = recv(far.as_raw_fd(), &mut buf, MsgFlags::empty()).expect("recv append");
            let directive: weaver_types::TokenDirective =
                serde_json::from_slice(&buf[..n]).expect("the append parses");
            assert!(matches!(
                directive,
                weaver_types::TokenDirective::AppendAndGenerate { .. }
            ));
            // The cancel arrives because the wait heard the stop.
            let n = recv(far.as_raw_fd(), &mut buf, MsgFlags::empty()).expect("recv cancel");
            let directive: weaver_types::TokenDirective =
                serde_json::from_slice(&buf[..n]).expect("the cancel parses");
            assert!(
                matches!(directive, weaver_types::TokenDirective::Cancel { .. }),
                "the stop cancels at the seam"
            );
            let send_answer = |answer: &weaver_types::TokenAnswer| {
                let bytes = serde_json::to_vec(answer).expect("answer renders");
                send(far.as_raw_fd(), &bytes, MsgFlags::empty()).expect("send answer");
            };
            send_answer(&weaver_types::TokenAnswer::Token {
                token: 7,
                piece: "par".into(),
            });
            let request = serde_json::value::RawValue::from_string(
                r#"{"rendered":"r","template":"t","sampling":{}}"#.to_string(),
            )
            .unwrap();
            let measurement = serde_json::value::RawValue::from_string(
                r#"{"model":"m","weights_hash":"h","input_tokens":[1],"output_tokens":[7],"blocks":[],"timings":{"prefill_ns":"1","decode_ns":"2"}}"#
                    .to_string(),
            )
            .unwrap();
            send_answer(&weaver_types::TokenAnswer::Generated(Generation {
                emission: "par".into(),
                finish: Finish::Stopped,
                request,
                measurement,
            }));
        });

        let session = SessionId("s-2".into());
        let sink = tempfile();
        let mut recorder = Recorder::receive(sink, RunOrdinal(0), SessionRef(session.0.clone()))
            .expect("recorder");
        let author = Author::new(&session, 0);
        author
            .author(&mut recorder, Kind::Load, Subsystem::Harness, None, None)
            .expect("load");
        let mut turn_ordinal = 0u64;
        let mut slot = Some(verb_end);

        let outcome = {
            let mut ports = Ports::grant(
                &decode,
                &author,
                &mut recorder,
                &mut turn_ordinal,
                None,
                &listener,
                Some(&mut slot),
            );
            let delta = vec![Message {
                role: Role::User,
                content: vec![ContentBlock::Text {
                    text: "stop me".into(),
                }],
            }];
            ports.turn(delta).expect("the aborted turn still returns")
        };
        peer.join().expect("the decode peer finishes");

        assert!(outcome.aborted, "the directive aborted the turn");
        assert_eq!(outcome.emission, "par", "the partial stands");

        // Announce after record: the answer carries the turn's fate.
        match admin.recv().expect("the stop's answer").payload {
            weaver_types::Payload::Answer(weaver_types::LifecycleAnswer::TurnAborted { turn }) => {
                assert_eq!(turn.0, "t-1");
            }
            other => panic!("the stop answers the turn's fate, got {other:?}"),
        }

        // The close names the directive, the partial standing before it.
        let close = recorder
            .structure()
            .by_kind(Kind::TurnClosed)
            .next()
            .expect("the close authored")
            .line
            .to_string();
        assert!(close.contains("stopped"), "{close}");
    }

    fn test_listener() -> crate::channel::CoordinationListener {
        let dir = std::env::temp_dir().join(format!(
            "weaver-engine-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let path = dir.join("c.sock");
        std::fs::remove_file(&path).ok();
        crate::channel::bind_coordination(&path).expect("bind")
    }

    fn tempfile() -> OwnedFd {
        let path = std::env::temp_dir().join(format!(
            "weaver-harness-turn-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let file = std::fs::File::create(&path).expect("sink");
        std::fs::remove_file(&path).ok();
        OwnedFd::from(file)
    }
}
