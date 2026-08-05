//! conforms: trace-sequence-gapless
//! conforms: trace-admission-precedes-fan-out
//! conforms: trace-queue-depth-not-config
//! conforms: trace-boundary-derived-not-stored
//! conforms: trace-whole-events-only
//! conforms: trace-pressure-reported-not-authored
//! conforms: trace-no-path-taking-constructor
//! conforms: trace-receive-site-takes-no-flag
//! conforms: trace-holds-no-recording-level
//! conforms: trace-receive-shape-pinned-by-doctest
//!
//! The emit path, the writer, and the commit boundary, per `weaver-trace-Spec`
//! sections 5 through 8. The recorder is the crate's principal type and owns
//! both sinks; the writer is a bounded queue and one thread draining to the
//! descriptor the harness was handed. Open takes descriptors and offers no
//! path-taking constructor, and this crate holds no recording level, because
//! none exists: everything produced is recorded.

use std::fs::File;
use std::io::Write;
use std::os::fd::OwnedFd;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;

use crate::canonical::{Sequence, render};
use crate::event::{Event, Kind, Line, Payload, RunOrdinal, SessionRef};
use crate::failure::{Failure, FieldName, SubmitRefusal, WriteError};
use crate::structure::{Record, WorkingStructure};

/// The queue's depth. A construction parameter of the deployment rather than a
/// configuration field, per `weaver-trace-Spec` section 6 - the composition
/// root supplies it when that root exists, and the receive shape's pin holds
/// the constructor to its three declared arguments until then. Generous
/// against loop 0's traffic.
pub const QUEUE_DEPTH: usize = 1024;

/// The depth at which pressure is reported rather than the depth of the queue
/// itself. An open election per section 11, settled by a measurement against a
/// real sink at a real rate; the interim value keeps the report ahead of the
/// queue filling.
pub const HIGH_WATER_MARK: usize = 768;

/// The interrogable state of the stream side, derived rather than stored:
/// committed is the highest sequence handed to the sink, admitted is the
/// highest assigned, queued is the difference, and a failed write is the last
/// error. Nothing about it survives the process.
#[derive(Debug, Clone, PartialEq)]
pub struct Boundary {
    pub committed: Option<Sequence>,
    pub admitted: Option<Sequence>,
    pub queued: usize,
    pub last_error: Option<WriteError>,
}

struct WriterShared {
    state: Mutex<WriterState>,
    drained: Condvar,
    queued: AtomicUsize,
}

struct WriterState {
    committed: Option<Sequence>,
    last_error: Option<WriteError>,
    failed: Option<(Sequence, WriteError)>,
}

/// The stream writer: one thread, one descriptor, whole events only. A short
/// write is retried to completion rather than reported as success, which
/// `write_all` carries.
type QueueItem = (Sequence, Line);

struct Writer {
    queue: SyncSender<QueueItem>,
    shared: Arc<WriterShared>,
    thread: Option<JoinHandle<()>>,
}

impl Writer {
    fn start(sink: OwnedFd) -> Writer {
        let (queue, receiver): (SyncSender<QueueItem>, Receiver<QueueItem>) =
            sync_channel(QUEUE_DEPTH);
        let shared = Arc::new(WriterShared {
            state: Mutex::new(WriterState {
                committed: None,
                last_error: None,
                failed: None,
            }),
            drained: Condvar::new(),
            queued: AtomicUsize::new(0),
        });
        let thread_shared = Arc::clone(&shared);
        let mut file = File::from(sink);
        let thread = std::thread::spawn(move || {
            while let Ok((sequence, line)) = receiver.recv() {
                // A write failure is terminal: committed never advances past
                // the first failed sequence, and every later queued record is
                // discarded with its accounting kept consistent, so a drain
                // returns rather than hanging on a dead sink.
                let already_failed = thread_shared
                    .state
                    .lock()
                    .expect("writer state")
                    .failed
                    .is_some();
                if !already_failed {
                    let outcome = file.write_all(line.as_bytes());
                    let mut state = thread_shared.state.lock().expect("writer state");
                    match outcome {
                        Ok(()) => state.committed = Some(sequence),
                        Err(e) => {
                            let err = WriteError::from(&e);
                            state.last_error = Some(err.clone());
                            state.failed = Some((sequence, err));
                        }
                    }
                }
                thread_shared.queued.fetch_sub(1, Ordering::SeqCst);
                thread_shared.drained.notify_all();
            }
        });
        Writer {
            queue,
            shared,
            thread: Some(thread),
        }
    }

    fn enqueue(&self, sequence: Sequence, line: Line) {
        self.shared.queued.fetch_add(1, Ordering::SeqCst);
        // A full queue blocks here until the writer frees space: the
        // deployment's loss bound expressed as backpressure rather than loss,
        // reached only after the high-water report warned the harness. After a
        // terminal write failure the writer discards, so the block cannot
        // outlive a dead sink. A send failure means the writer thread is
        // gone, surfaced through the boundary's last error.
        if self.queue.send((sequence, line)).is_err() {
            self.shared.queued.fetch_sub(1, Ordering::SeqCst);
            let mut state = self.shared.state.lock().expect("writer state");
            state.last_error = Some(WriteError {
                os_error: None,
                message: "writer thread terminated".to_string(),
            });
        }
    }

    fn queued(&self) -> usize {
        self.shared.queued.load(Ordering::SeqCst)
    }

    /// The failure a later submission surfaces: the first failed write, named
    /// with its record, or a dead writer thread.
    fn standing_failure(&self) -> Option<Failure> {
        let state = self.shared.state.lock().expect("writer state");
        if let Some((sequence, source)) = &state.failed {
            return Some(Failure::CommitFailed {
                sequence: *sequence,
                source: source.clone(),
            });
        }
        state
            .last_error
            .as_ref()
            .map(|err| Failure::WriteTargetUnusable {
                source: err.clone(),
            })
    }

    fn wait_drained(&self) -> Result<(), Failure> {
        let mut state = self.shared.state.lock().expect("writer state");
        while self.shared.queued.load(Ordering::SeqCst) > 0 {
            state = self.shared.drained.wait(state).expect("writer state");
        }
        match &state.failed {
            Some((sequence, source)) => Err(Failure::CommitFailed {
                sequence: *sequence,
                source: source.clone(),
            }),
            None => match &state.last_error {
                Some(err) => Err(Failure::WriteTargetUnusable {
                    source: err.clone(),
                }),
                None => Ok(()),
            },
        }
    }

    fn boundary(&self, admitted: Option<Sequence>) -> Boundary {
        let state = self.shared.state.lock().expect("writer state");
        Boundary {
            committed: state.committed,
            admitted,
            queued: self.shared.queued.load(Ordering::SeqCst),
            last_error: state.last_error.clone(),
        }
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        // Closing the channel ends the thread's loop; the join keeps the
        // descriptor's writes ordered before the process moves on.
        let (closed, _) = sync_channel::<QueueItem>(1);
        let _ = std::mem::replace(&mut self.queue, closed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// The recorder: the crate's principal type, owning the working structure and
/// the writer, per `weaver-trace-Spec` section 5. The emit path fans out to
/// both in one act, so one item holds both, and nothing else in the crate can
/// append to the structure or enqueue to the writer.
pub struct Recorder {
    session: SessionRef,
    run: RunOrdinal,
    next: u64,
    structure: WorkingStructure,
    writer: Writer,
}

impl Recorder {
    /// The crate's one constructor and its one receive site. It accepts an
    /// `OwnedFd` and nothing that could name a path, and it takes no flag
    /// argument: close-on-exec is supplied by the harness at its own receive,
    /// and append-only rides the open file description from admin's open.
    pub fn receive(sink: OwnedFd, run: RunOrdinal, session: SessionRef) -> Result<Self, Failure> {
        Ok(Recorder {
            session,
            run,
            next: 0,
            structure: WorkingStructure::new(),
            writer: Writer::start(sink),
        })
    }

    /// The four steps of the charter's emit path, in order, always: admit or
    /// refuse, assign the sequence, render once, fan out. The structure lands
    /// first and the return is the acknowledgment; a refusal has no effect on
    /// either sink, so the two cannot disagree about what was admitted.
    ///
    /// The sequence member of the submitted envelope is assigned here - step
    /// two is the recorder's - and the submitted value is overwritten rather
    /// than trusted, since a gapless run-scoped order cannot be authored by
    /// the caller.
    ///
    /// An `Err(CommitPressure)` has exactly one meaning: the event was
    /// admitted, appended, and queued, and the queue has crossed the
    /// high-water mark while the sink remains writable. The harness authors a
    /// `fault` event in response; this crate authors no event and holds no
    /// event kind. At the queue's full depth a submission blocks instead,
    /// which is the deployment's loss bound expressed as backpressure - the
    /// high-water report warned first, and a terminal write failure frees the
    /// block by discarding.
    pub fn submit(&mut self, mut event: Event) -> Result<Sequence, Failure> {
        admit(&event, &self.session, self.run)?;
        if let Some(failure) = self.writer.standing_failure() {
            return Err(failure);
        }
        let sequence = Sequence(self.next);
        event.envelope.sequence = sequence;
        let line = render(&event)?;
        self.next += 1;
        self.structure.append(Record {
            sequence,
            kind: event.envelope.kind,
            turn: event.envelope.turn.clone(),
            line: Line::clone(&line),
        });
        self.writer.enqueue(sequence, line);
        let queued = self.writer.queued();
        if queued > HIGH_WATER_MARK {
            return Err(Failure::CommitPressure { queued });
        }
        Ok(sequence)
    }

    pub fn structure(&self) -> &WorkingStructure {
        &self.structure
    }

    pub fn boundary(&self) -> Boundary {
        let admitted = self.next.checked_sub(1).map(Sequence);
        self.writer.boundary(admitted)
    }

    /// Empties the queue and returns. At unload the harness calls this before
    /// answering left, so a left answer means everything admitted reached the
    /// sink. At process death nothing drains and the tail is forfeited, which
    /// this crate cannot report because the process reporting it died.
    pub fn drain(&mut self) -> Result<(), Failure> {
        self.writer.wait_drained()
    }
}

/// Admission, step one: the envelope's identity fields bind the event by
/// construction, the required fields are present for the kind, the kind is
/// known by type, and the payload is well-formed octets by `RawValue`
/// construction. What admission may judge is bounded: the interior of a
/// message is the harness's, and a recorder that parsed one would have taken a
/// judgment the charter denies it.
fn admit(event: &Event, session: &SessionRef, run: RunOrdinal) -> Result<(), Failure> {
    let refuse = |reason| Err(Failure::RefusedOnSubmit { reason });
    if event.envelope.session != *session || event.envelope.run != run {
        // A submission bound to another run's identity is missing this run's:
        // the closest case the contract's vocabulary expresses.
        return refuse(SubmitRefusal::RequiredFieldAbsent {
            field: FieldName("session".to_string()),
        });
    }
    let kind = event.envelope.kind;
    if turn_required(kind) && event.envelope.turn.is_none() {
        return refuse(SubmitRefusal::RequiredFieldAbsent {
            field: FieldName("turn".to_string()),
        });
    }
    if turn_forbidden(kind) && event.envelope.turn.is_some() {
        // A run-level event carrying a turn would be a join key the work never
        // held, a false attribution, refused as a malformed submission - the
        // closest case the contract's vocabulary expresses.
        return refuse(SubmitRefusal::PayloadMalformed);
    }
    if !pairing_licensed(kind, event.payload.as_ref()) {
        return match event.payload {
            None => refuse(SubmitRefusal::RequiredFieldAbsent {
                field: FieldName("payload".to_string()),
            }),
            Some(_) => refuse(SubmitRefusal::PayloadMalformed),
        };
    }
    Ok(())
}

/// A run-level event belongs to no turn and refuses one; a turn-level kind
/// carries one, never inferred. `fault` may occur inside or outside a turn,
/// so its option stays the caller's fact.
fn turn_forbidden(kind: Kind) -> bool {
    matches!(kind, Kind::Load | Kind::Unload | Kind::SessionClosed)
}

fn turn_required(kind: Kind) -> bool {
    match kind {
        Kind::Load | Kind::Unload | Kind::SessionClosed | Kind::Fault => false,
        Kind::TurnStarted
        | Kind::TurnClosed
        | Kind::MessageUser
        | Kind::MessageAssistant
        | Kind::MessageToolResult
        | Kind::ToolCallStarted
        | Kind::ToolCallCompleted
        | Kind::ModelRequest
        | Kind::ModelOutput
        | Kind::ModelMeasurement => true,
    }
}

/// The total kind-to-payload mapping, fourteen kinds and eight dispositions,
/// enforced here because the untagged payload leaves serde unable to.
fn pairing_licensed(kind: Kind, payload: Option<&Payload>) -> bool {
    matches!(
        (kind, payload),
        (
            Kind::Load | Kind::Unload | Kind::SessionClosed | Kind::TurnStarted,
            None
        ) | (
            Kind::MessageUser | Kind::MessageAssistant | Kind::MessageToolResult,
            Some(Payload::Message(_))
        ) | (Kind::TurnClosed, Some(Payload::TurnClosed(_)))
            | (Kind::Fault, Some(Payload::Fault(_)))
            | (Kind::ModelRequest, Some(Payload::ModelRequest(_)))
            | (Kind::ModelOutput, Some(Payload::ModelOutput(_)))
            | (Kind::ModelMeasurement, Some(Payload::ModelMeasurement(_)))
            | (
                Kind::ToolCallStarted | Kind::ToolCallCompleted,
                Some(Payload::Deferred(_))
            )
    )
}
