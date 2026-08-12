//! conforms: harness-loop-mints-no-port
//! conforms: harness-extension-seam-at-loaded-and-idle
//! conforms: harness-turn-authors-the-model-events
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
use crate::channel::DecodeChannel;

/// The granted surface handed to loop 1 at loaded-and-idle, borrowing the
/// standing interior for the seat's lifetime. Its fields are private and its
/// only constructor is crate-private, which is the blade.
pub struct Ports<'a> {
    decode: &'a DecodeChannel,
    author: &'a Author,
    recorder: &'a mut Recorder,
    turn_ordinal: &'a mut u64,
    assembled: Option<Prompt>,
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
    ) -> Self {
        Ports {
            decode,
            author,
            recorder,
            turn_ordinal,
            assembled,
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

        // The bracket opens, then the delta is authored as the turn's user
        // messages, before the exchange so the record reads the ask before the
        // answer.
        self.author
            .author(
                self.recorder,
                Kind::TurnStarted,
                Subsystem::Harness,
                Some(&turn),
                None,
            )
            .map_err(|_| TurnError::ChannelLost)?;
        for message in &delta {
            self.author
                .author_message(self.recorder, message, &turn)
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

        // Consume the stream to the close. The intermediate tokens are the
        // stream loop 1 could surface as it grows, and the close carries the
        // generation whole, which is what the record needs.
        let generation = loop {
            match self
                .decode
                .recv_answer()
                .map_err(|_| TurnError::ChannelLost)?
            {
                TokenAnswer::Token { .. } => continue,
                TokenAnswer::Generated(generation) => break generation,
                TokenAnswer::AtRest => continue,
                other => {
                    let _ = other;
                    return Err(TurnError::ChannelLost);
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
                Some(&turn),
                Some(Payload::ModelRequest(generation.request)),
            )
            .map_err(|_| TurnError::ChannelLost)?;
        let stopped = matches!(generation.finish, weaver_types::Finish::Stopped);
        self.author
            .author(
                self.recorder,
                Kind::ModelOutput,
                Subsystem::Spu,
                Some(&turn),
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
                Some(&turn),
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
            .author_message(self.recorder, &assistant, &turn)
            .map_err(|_| TurnError::Unlicensed)?
            .map_err(|_| TurnError::ChannelLost)?;

        // The bracket closes clean, the turn having completed without a stop.
        self.author
            .author(
                self.recorder,
                Kind::TurnClosed,
                Subsystem::Harness,
                Some(&turn),
                Some(Payload::TurnClosed(TurnClose::Clean)),
            )
            .map_err(|_| TurnError::ChannelLost)?;

        Ok(TurnOutcome {
            emission: generation.emission,
            stopped,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

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

        let outcome = {
            let mut ports = Ports::grant(&decode, &author, &mut recorder, &mut turn_ordinal, None);
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

    fn tempfile() -> OwnedFd {
        let path = std::env::temp_dir().join(format!(
            "weaver-harness-turn-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let file = std::fs::File::create(&path).expect("sink");
        std::fs::remove_file(&path).ok();
        // SAFETY: File owns the descriptor and into_raw_fd transfers it.
        unsafe { OwnedFd::from_raw_fd(std::os::fd::IntoRawFd::into_raw_fd(file)) }
    }
}
