//! conforms: spu-session-never-rewinds
//! conforms: spu-no-scoped-clear-surface
//! conforms: spu-overflow-refuses-sheds-nothing
//! conforms: spu-terminator-on-every-path
//! conforms: spu-cancel-bounded-by-one-token
//! conforms: spu-cancel-polled-not-signalled
//! conforms: spu-flush-mechanism-from-declaration
//!
//! The resident session and its append path, per `weaver-spu-Spec` sections 4.2
//! to 4.4.
//!
//! **Append-only, `resident_len`, and nothing ever rewinds.** The session holds
//! the resident length and no prefix cache, and every generation decodes only
//! the delta at absolute positions from that length forward. The archived
//! tree's proof is the reason: a rewind fails silently on families that keep
//! recurrent layers, because the clear returns success while the recurrent
//! state stays, and the failure surfaces later as a position error far from its
//! cause.
//!
//! **This crate calls no scoped-clear on a resident range**, which is the
//! discipline stated as an absence. There is no method here that clears a range,
//! and the pin below is that its absence is a compile error to depend on:
//!
//! ```compile_fail
//! use weaver_spu::decoder::session::Session;
//! fn pin(session: &mut Session) {
//!     // No scoped clear exists on this type, and this is what says so. The
//!     // day someone adds one, this doctest starts compiling and the pin fires.
//!     session.clear_range(0, 5);
//! }
//! ```
//!
//! The behaviour and the surface are two records with two instruments: the
//! monotonic resident length is a perturbation test and the missing method is
//! this compile-fail pin. Neither claims the other's half, because a pin says a
//! rewind cannot be written and says nothing about a turn that re-prefills
//! through the append path instead.

use crate::decoder::backend::{Backend, DecodeFault, FlushMechanism, TokenId};

/// Why a generation stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stopped {
    /// The family's stop condition was reached.
    Complete,
    /// The harness cancelled, checked at a token boundary.
    Cancelled,
    /// The session reached its capacity mid-generation.
    CapacityReached,
}

/// What one generation produced.
#[derive(Debug, Clone, PartialEq)]
pub struct Generated {
    /// The tokens sampled, terminator excluded: it is resident but it is not
    /// something the model said.
    pub tokens: Vec<TokenId>,
    /// Why the loop ended. **A cancelled generation answers with what it
    /// produced, marked stopped, after the terminator lands.**
    pub stopped: Stopped,
}

/// The family's stop condition and its terminator.
#[derive(Debug, Clone, PartialEq)]
pub struct StopCondition {
    /// Tokens that end a generation when sampled.
    pub stop_tokens: Vec<TokenId>,
    /// The token made resident after the loop on every path, so the next turn's
    /// framing begins at a boundary rather than mid-message.
    pub terminator: TokenId,
    /// A ceiling on sampled tokens, so a model that never emits a stop token
    /// still ends.
    pub max_tokens: usize,
}

/// The cancel, **polled rather than signalled**.
///
/// The decode socket is the one carrier, so the loop polls it between tokens
/// rather than waiting on it. That is why the check is a boundary check rather
/// than a signal handler: nothing asynchronous is needed to make a
/// token-boundary stop, and nothing asynchronous is introduced. A cancel is the
/// harness asking this process to stop, so it is a seam, and apex section 5.1
/// admits no seam across a process line that is not a socket.
pub trait CancelPoll {
    /// Answer without blocking. A poll that blocked would defeat the bound this
    /// exists to provide.
    fn cancelled(&mut self) -> bool;
}

/// A cancel that never fires, for callers with nothing to poll.
pub struct NeverCancels;

impl CancelPoll for NeverCancels {
    fn cancelled(&mut self) -> bool {
        false
    }
}

/// The resident session.
///
/// **`resident_len` is monotonic except across a flush.** It is private, and
/// the paths that reduce it are [`Session::flush`] and the close over a backend
/// fault, which zeroes it alongside `opened` because a closed session accounts
/// for nothing. Nothing writes it downward over a session that keeps serving,
/// which is the discipline the perturbation test watches from outside.
pub struct Session {
    backend: Box<dyn Backend>,
    resident_len: usize,
    prefix_len: usize,
    prefix: Vec<TokenId>,
    capacity: usize,
    opened: bool,
    /// Set by the close over a backend fault and never cleared. `opened` alone
    /// cannot carry this: a fresh session and a poisoned one both read as not
    /// opened, and `open` must refuse the second while serving the first,
    /// because the poisoned session's backend has been closed and a re-open
    /// would decode against a closed engine.
    poisoned: bool,
    flush_mechanism: FlushMechanism,
}

impl Session {
    pub fn new(
        backend: Box<dyn Backend>,
        capacity: usize,
        flush_mechanism: FlushMechanism,
    ) -> Self {
        Session {
            backend,
            resident_len: 0,
            prefix_len: 0,
            prefix: Vec::new(),
            capacity,
            opened: false,
            poisoned: false,
            flush_mechanism,
        }
    }

    /// What is resident now. The session's own account of itself, which the
    /// overflow refusal hands back to the harness.
    pub fn resident_len(&self) -> usize {
        self.resident_len
    }

    /// Where the identity prefix ends. **The prefix is established at open and
    /// is permanent:** no operation short of the flush reduces the length below
    /// this.
    pub fn prefix_len(&self) -> usize {
        self.prefix_len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Render the identity prefix resident, once, and record the length it
    /// produced.
    pub fn open(&mut self, prefix: &[TokenId]) -> Result<(), DecodeFault> {
        // Open is first and happens once. Without this guard a second open
        // writes the resident length back down over accumulated turns with no
        // backend truncate or reestablish beneath it, which is the silent
        // rewind the module header forbids.
        if self.opened {
            return Err(DecodeFault::AlreadyOpen);
        }
        // A poisoned session is not a fresh one: its backend is closed, so
        // there is nothing here to open over.
        if self.poisoned {
            return Err(DecodeFault::NotOpen);
        }
        if prefix.len() > self.capacity {
            return Err(DecodeFault::Overflow {
                resident: 0,
                requested: prefix.len(),
                capacity: self.capacity,
            });
        }
        self.backend.decode_at(prefix, 0)?;
        self.resident_len = prefix.len();
        self.prefix_len = prefix.len();
        self.prefix = prefix.to_vec();
        self.opened = true;
        Ok(())
    }

    /// Append the delta at the resident end and generate until the family's stop
    /// condition, the harness's cancel, or the session's capacity.
    ///
    /// **The terminator is made resident before this returns, on every path.**
    /// The archived tree's hard-won correctness note: a generation that stops at
    /// the end-of-generation marker without decoding it leaves the terminator
    /// absent, and the next turn's framing is then malformed at a boundary
    /// nobody looks at.
    pub fn append_and_generate(
        &mut self,
        delta: &[TokenId],
        stop: &StopCondition,
        cancel: &mut dyn CancelPoll,
    ) -> Result<Generated, DecodeFault> {
        if !self.opened {
            return Err(DecodeFault::NotOpen);
        }

        // **Overflow refuses and sheds nothing.** This crate evicts and compacts
        // nothing: either would be this crate deciding which part of the agent's
        // context matters, which is the harness's domain. The refusal carries
        // the session's own account so the loop decides with the fact rather
        // than without it. One slot is reserved for the terminator, because a
        // delta that fits only by displacing it would produce the malformed
        // framing the terminator discipline exists to prevent.
        let needed = delta.len() + 1;
        if self.resident_len + needed > self.capacity {
            return Err(DecodeFault::Overflow {
                resident: self.resident_len,
                requested: delta.len(),
                capacity: self.capacity,
            });
        }

        if let Err(fault) = self.backend.decode_at(delta, self.resident_len) {
            return Err(self.poison(fault));
        }
        self.resident_len += delta.len();

        let mut produced = Vec::new();
        let stopped = loop {
            // **The cancel is checked between sampled tokens**, which bounds
            // the stop by one token's decode rather than by a kernel's
            // completion. Checked before sampling, so a cancel that arrives
            // during the previous token's decode costs no further token.
            if cancel.cancelled() {
                break Stopped::Cancelled;
            }
            if produced.len() >= stop.max_tokens {
                break Stopped::Complete;
            }
            // A slot for the terminator is held back, so capacity is reached
            // one short of the true end.
            if self.resident_len + 1 >= self.capacity {
                break Stopped::CapacityReached;
            }

            let token = match self.backend.sample() {
                Ok(token) => token,
                Err(fault) => return Err(self.poison(fault)),
            };
            if stop.stop_tokens.contains(&token) {
                break Stopped::Complete;
            }
            produced.push(token);
            if let Err(fault) = self.backend.decode_at(&[token], self.resident_len) {
                return Err(self.poison(fault));
            }
            self.resident_len += 1;
        };

        // Every path above converges here. The terminator lands on the clean
        // path and the cancelled path alike, which is what makes the cancel of
        // charter section 13.5 leave a well-framed session. The engine-fault
        // paths do not reach here and do not pretend to: a backend that just
        // faulted cannot be asked to decode a terminator, so those paths close
        // the session instead, and a closed session is the honest state where a
        // well-framed one cannot be produced.
        if let Err(fault) = self
            .backend
            .decode_at(&[stop.terminator], self.resident_len)
        {
            return Err(self.poison(fault));
        }
        self.resident_len += 1;

        Ok(Generated {
            tokens: produced,
            stopped,
        })
    }

    /// Reach the flush's fixed outcome: the identity prefix resident, the
    /// accumulated turns gone.
    ///
    /// **The outcome is fixed and the mechanism is per family.** The family
    /// declares which it is and this reads the declaration rather than
    /// inferring it from a version string.
    pub fn flush(&mut self) -> Result<(), DecodeFault> {
        if !self.opened {
            return Err(DecodeFault::NotOpen);
        }
        let outcome = match self.flush_mechanism {
            FlushMechanism::TruncateToPosition => self.backend.truncate_to(self.prefix_len),
            FlushMechanism::ReestablishAndReprefill => self
                .backend
                .reestablish()
                .and_then(|()| self.backend.decode_at(&self.prefix, 0)),
        };
        // A flush that fails partway leaves the backend's state and the
        // session's account disagreeing: a reestablish that succeeded before a
        // re-prefill that failed is an empty backend under a large resident
        // length, and the next append would decode at an absolute position far
        // past the backend's real state, the position-error-far-from-cause
        // failure the append-only discipline exists to prevent. The session
        // closes instead of carrying the disagreement.
        if let Err(fault) = outcome {
            return Err(self.poison(fault));
        }
        // The one place in this type that reduces the resident length, and it
        // reduces it to the prefix rather than to an arbitrary position.
        self.resident_len = self.prefix_len;
        Ok(())
    }

    pub fn close(&mut self) {
        self.backend.close();
        self.opened = false;
    }

    /// Close the session over a backend fault and hand the fault back.
    ///
    /// After an engine fault or a failed flush the backend's state and this
    /// session's account of it can no longer be trusted to agree, so the
    /// session stops being appendable rather than carrying the disagreement
    /// forward. Every later operation answers [`DecodeFault::NotOpen`], which
    /// is the truth: no serviceable prefix stands.
    fn poison(&mut self, fault: DecodeFault) -> DecodeFault {
        self.poisoned = true;
        self.backend.close();
        self.opened = false;
        self.resident_len = 0;
        fault
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    /// What the double saw, held behind a shared handle so a test can read it
    /// back after the session has taken ownership of the backend.
    #[derive(Default)]
    struct Log {
        decoded: Vec<(Vec<TokenId>, usize)>,
        truncated: Vec<usize>,
        reestablished: usize,
        /// What `sample` returns, in order. Exhausting it yields token 0.
        script: Vec<TokenId>,
        sampled: usize,
        /// Fail the nth `decode_at` call, counting from zero.
        fail_decode_call: Option<usize>,
        /// Fail every `sample`.
        fail_sample: bool,
    }

    /// A backend that records what it was asked to do. The seam this doubles is
    /// the one Spec section 4.1 declares, so doubling it is reading the seam
    /// rather than inventing one.
    struct Recorder(Rc<RefCell<Log>>);

    impl Backend for Recorder {
        fn decode_at(&mut self, tokens: &[TokenId], position: usize) -> Result<(), DecodeFault> {
            let mut log = self.0.borrow_mut();
            let call = log.decoded.len();
            log.decoded.push((tokens.to_vec(), position));
            if log.fail_decode_call == Some(call) {
                return Err(DecodeFault::Engine {
                    detail: "scripted".into(),
                });
            }
            Ok(())
        }
        fn sample(&mut self) -> Result<TokenId, DecodeFault> {
            let mut log = self.0.borrow_mut();
            if log.fail_sample {
                return Err(DecodeFault::Engine {
                    detail: "scripted".into(),
                });
            }
            let token = log.script.get(log.sampled).copied().unwrap_or(TokenId(0));
            log.sampled += 1;
            Ok(token)
        }
        fn truncate_to(&mut self, position: usize) -> Result<(), DecodeFault> {
            self.0.borrow_mut().truncated.push(position);
            Ok(())
        }
        fn reestablish(&mut self) -> Result<(), DecodeFault> {
            self.0.borrow_mut().reestablished += 1;
            Ok(())
        }
        fn close(&mut self) {}
    }

    /// Cancels after a set number of polls, which is how a token-boundary bound
    /// is made observable without a clock.
    struct CancelsAfter {
        polls: usize,
        limit: usize,
    }

    impl CancelPoll for CancelsAfter {
        fn cancelled(&mut self) -> bool {
            let now = self.polls >= self.limit;
            self.polls += 1;
            now
        }
    }

    fn stop_at(terminator: u32) -> StopCondition {
        StopCondition {
            stop_tokens: vec![TokenId(99)],
            terminator: TokenId(terminator),
            max_tokens: 64,
        }
    }

    fn with_mechanism(
        script: Vec<TokenId>,
        capacity: usize,
        mechanism: FlushMechanism,
    ) -> (Session, Rc<RefCell<Log>>) {
        let log = Rc::new(RefCell::new(Log {
            script,
            ..Default::default()
        }));
        let mut session = Session::new(Box::new(Recorder(Rc::clone(&log))), capacity, mechanism);
        session.open(&[TokenId(1), TokenId(2)]).expect("open");
        (session, log)
    }

    fn opened(script: Vec<TokenId>, capacity: usize) -> (Session, Rc<RefCell<Log>>) {
        with_mechanism(script, capacity, FlushMechanism::TruncateToPosition)
    }

    fn last_decoded(log: &Rc<RefCell<Log>>) -> Vec<TokenId> {
        log.borrow()
            .decoded
            .last()
            .map(|(tokens, _)| tokens.clone())
            .unwrap_or_default()
    }

    /// **Every generation decodes the delta at the resident end**, never back
    /// at the prefix. This reads the position the backend was asked to decode
    /// at, which is the property itself.
    ///
    /// An earlier version of this test asserted only that `resident_len` grew
    /// across turns, and **it could not fail**: under the rewind, the second
    /// turn decodes at the prefix, runs past its exhausted script to
    /// `max_tokens`, and ends up longer than the first turn anyway. Growth was
    /// passing for non-rewinding. The position is the thing.
    ///
    /// Perturbation: decode the delta at `self.prefix_len` instead of
    /// `self.resident_len`, which is the re-prefill rewind the archived tree's
    /// proof warns about, and this test fails on the recorded position. Watched
    /// under exactly that change.
    #[test]
    fn the_delta_decodes_at_the_resident_end_never_at_the_prefix() {
        // A script that stops immediately, so generation adds a known amount
        // and cannot mask a wrong position by running long.
        let (mut session, log) = opened(vec![TokenId(99), TokenId(99)], 128);
        let mut cancel = NeverCancels;

        session
            .append_and_generate(&[TokenId(3)], &stop_at(50), &mut cancel)
            .expect("first turn");
        let after_first = session.resident_len();
        assert!(
            after_first > session.prefix_len(),
            "the first turn extended"
        );

        let before = log.borrow().decoded.len();
        session
            .append_and_generate(&[TokenId(4)], &stop_at(50), &mut cancel)
            .expect("second turn");

        let (tokens, position) = log.borrow().decoded[before].clone();
        assert_eq!(tokens, vec![TokenId(4)], "the second delta is what landed");
        assert_eq!(
            position, after_first,
            "the delta decodes at the resident end, not at the prefix"
        );
    }

    /// The resident length is monotonic across turns, which is the behavioural
    /// half beside the position check above.
    #[test]
    fn the_resident_length_never_rewinds_across_turns() {
        let (mut session, _log) = opened(vec![TokenId(99), TokenId(99)], 128);
        let mut cancel = NeverCancels;
        let mut seen = session.resident_len();
        for delta in [TokenId(3), TokenId(4), TokenId(5)] {
            session
                .append_and_generate(&[delta], &stop_at(50), &mut cancel)
                .expect("a turn");
            assert!(
                session.resident_len() > seen,
                "each turn extends, never rewinding"
            );
            seen = session.resident_len();
        }
    }

    /// **The turn terminator is made resident on the clean path.**
    ///
    /// Perturbation: move the terminator decode inside the `Stopped::Complete`
    /// arm, or delete it, and this test fails because the last decoded token is
    /// no longer the terminator. Watched under exactly that deletion.
    #[test]
    fn the_terminator_lands_on_the_clean_path() {
        let (mut session, log) = opened(vec![TokenId(7), TokenId(99)], 128);
        let mut cancel = NeverCancels;
        let generated = session
            .append_and_generate(&[TokenId(3)], &stop_at(50), &mut cancel)
            .expect("a turn");
        assert_eq!(generated.stopped, Stopped::Complete);
        assert_eq!(
            generated.tokens,
            vec![TokenId(7)],
            "99 stops, 7 is produced"
        );
        assert_eq!(last_decoded(&log), vec![TokenId(50)]);
    }

    /// **And on the cancelled path alike**, which is what makes the cancel leave
    /// a well-framed session rather than one that ends mid-message.
    ///
    /// Perturbation: return early from the `Stopped::Cancelled` break without
    /// falling through to the terminator decode and this test fails. Watched
    /// under exactly that early return.
    #[test]
    fn the_terminator_lands_on_the_cancelled_path_too() {
        let (mut session, log) = opened(vec![TokenId(7), TokenId(8), TokenId(9)], 128);
        let mut cancel = CancelsAfter { polls: 0, limit: 2 };
        let generated = session
            .append_and_generate(&[TokenId(3)], &stop_at(50), &mut cancel)
            .expect("a cancelled turn");
        assert_eq!(generated.stopped, Stopped::Cancelled);
        assert_eq!(
            last_decoded(&log),
            vec![TokenId(50)],
            "the terminator is resident even though the turn was stopped"
        );
    }

    /// **The cancel is bounded by one token.** Cancelling at the second poll
    /// yields exactly the tokens sampled before it, never more, so the stop
    /// costs one token's decode rather than a kernel's completion.
    ///
    /// Perturbation: move the cancel check to after the sample-and-decode, and
    /// this test fails because a third token is produced. Watched under exactly
    /// that move.
    #[test]
    fn the_cancel_is_bounded_by_one_token() {
        let (mut session, _log) = opened(vec![TokenId(7), TokenId(8), TokenId(9)], 128);
        let mut cancel = CancelsAfter { polls: 0, limit: 2 };
        let generated = session
            .append_and_generate(&[TokenId(3)], &stop_at(50), &mut cancel)
            .expect("a cancelled turn");
        assert_eq!(
            generated.tokens,
            vec![TokenId(7), TokenId(8)],
            "two polls passed, so exactly two tokens were produced"
        );
    }

    /// **Overflow refuses and sheds nothing**, and the refusal carries the
    /// session's own account so the harness can decide with the numbers.
    #[test]
    fn an_overflowing_delta_refuses_with_the_session_s_account() {
        let (mut session, _log) = opened(vec![], 8);
        let mut cancel = NeverCancels;
        let refusal = session.append_and_generate(
            &[
                TokenId(1),
                TokenId(2),
                TokenId(3),
                TokenId(4),
                TokenId(5),
                TokenId(6),
                TokenId(7),
            ],
            &stop_at(50),
            &mut cancel,
        );
        match refusal {
            Err(DecodeFault::Overflow {
                resident,
                requested,
                capacity,
            }) => {
                assert_eq!((resident, requested, capacity), (2, 7, 8));
            }
            other => panic!("an overflowing delta refuses, got {other:?}"),
        }
        assert_eq!(
            session.resident_len(),
            2,
            "and sheds nothing: the session is untouched"
        );
    }

    /// **The flush mechanism is read from the declaration**, not inferred. A
    /// truncating family truncates to the prefix.
    #[test]
    fn a_truncating_family_flushes_by_truncating_to_the_prefix() {
        let (mut session, log) = opened(vec![TokenId(99)], 128);
        let mut cancel = NeverCancels;
        session
            .append_and_generate(&[TokenId(3)], &stop_at(50), &mut cancel)
            .expect("a turn");
        session.flush().expect("flush");
        assert_eq!(session.resident_len(), session.prefix_len());
        assert_eq!(log.borrow().truncated.clone(), vec![2]);
    }

    /// A family that cannot roll back is re-established and re-prefilled, which
    /// is expensive and correct where the cheap path is silently wrong.
    #[test]
    fn a_non_rewinding_family_flushes_by_reestablishing() {
        let (mut session, log) = with_mechanism(
            vec![TokenId(99)],
            128,
            FlushMechanism::ReestablishAndReprefill,
        );
        session.flush().expect("flush");
        assert_eq!(session.resident_len(), 2);
        assert_eq!(log.borrow().reestablished, 1);
        assert_eq!(log.borrow().truncated.clone(), Vec::<usize>::new());
    }

    /// **Open is first and happens once.** A second open refuses rather than
    /// silently rewinding the resident length over accumulated turns.
    ///
    /// Perturbation: remove the `opened` guard from `open` and this test fails,
    /// because the second open succeeds and the length rewinds to the prefix.
    /// Watched under exactly that removal.
    #[test]
    fn a_second_open_refuses_rather_than_rewinding() {
        let (mut session, _log) = opened(vec![TokenId(99)], 128);
        let mut cancel = NeverCancels;
        session
            .append_and_generate(&[TokenId(3)], &stop_at(50), &mut cancel)
            .expect("a turn");
        let grown = session.resident_len();
        assert_eq!(
            session.open(&[TokenId(1)]),
            Err(DecodeFault::AlreadyOpen),
            "a second open refuses"
        );
        assert_eq!(session.resident_len(), grown, "and rewinds nothing");
    }

    /// **A flush that fails partway closes the session** rather than leaving a
    /// large resident length over an empty backend, where the next append would
    /// decode far past the backend's real state.
    ///
    /// Perturbation: remove the poisoning from `flush` and this test fails,
    /// because the session stays open with its pre-flush length and the append
    /// runs. Watched under exactly that removal.
    #[test]
    fn a_failed_flush_closes_the_session() {
        let (mut session, log) = with_mechanism(
            vec![TokenId(99)],
            128,
            FlushMechanism::ReestablishAndReprefill,
        );
        // The re-prefill after the reestablish is the next decode call.
        let next_call = log.borrow().decoded.len();
        log.borrow_mut().fail_decode_call = Some(next_call);
        assert!(session.flush().is_err(), "the flush reports its failure");
        let mut cancel = NeverCancels;
        assert_eq!(
            session.append_and_generate(&[TokenId(3)], &stop_at(50), &mut cancel),
            Err(DecodeFault::NotOpen),
            "the session no longer serves"
        );
    }

    /// **An engine fault mid-generation closes the session.** The terminator
    /// discipline covers the clean path and the cancelled path; a backend that
    /// just faulted cannot be asked to decode a terminator, so the fault path
    /// closes the session instead of leaving a terminator-less prefix open for
    /// the next append.
    ///
    /// Perturbation: propagate the sample fault without the poison and this
    /// test fails on the second append, which runs against the mid-message
    /// state. Watched under exactly that removal.
    #[test]
    fn an_engine_fault_mid_generation_closes_the_session() {
        let (mut session, log) = opened(vec![], 128);
        log.borrow_mut().fail_sample = true;
        let mut cancel = NeverCancels;
        assert!(
            session
                .append_and_generate(&[TokenId(3)], &stop_at(50), &mut cancel)
                .is_err(),
            "the fault surfaces"
        );
        assert_eq!(
            session.append_and_generate(&[TokenId(4)], &stop_at(50), &mut cancel),
            Err(DecodeFault::NotOpen),
            "and the session no longer serves"
        );
        assert_eq!(
            session.open(&[TokenId(1)]),
            Err(DecodeFault::NotOpen),
            "and cannot be reopened over its closed backend"
        );
    }
}
