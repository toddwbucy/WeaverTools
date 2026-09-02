//! The record's events, drained rather than held, per
//! `weaver-analysis-Spec` section 5.
//!
//! **The drain is the class's rather than the lens's.**
//! `diagnostic-replay-loop` names the diagnostic loop a class with an
//! interchangeable reader, so this module is one drain over a record's
//! events and a reader trait above it: a reader consumes events as they
//! land and holds only what its own reading needs. The lens is the first
//! reader and sets no precedent the next must break.
//!
//! A file is a stream that ends, so nothing here is a pipe-only path:
//! every reading takes this road and the sink's shape decides only where
//! the bytes come from.

use std::io::BufRead;

use crate::record::Event;

/// What a reader answers to a drained event.
#[derive(Debug, Clone, PartialEq)]
pub enum Step {
    /// Keep draining.
    Continue,
    /// Stop reading: the reader has what it needs, and the rest of the
    /// stream is someone else's business.
    Done,
    /// The reader refuses, naming why. The drain stops and carries it.
    Refuse(String),
}

/// A reader over a drained record. One method, called per event in
/// landing order, so a reader that holds nothing holds nothing.
pub trait Reader {
    fn event(&mut self, event: &Event) -> Step;
}

/// Why a drain ended.
#[derive(Debug, Clone, PartialEq)]
pub enum Drained {
    /// The stream ended.
    Exhausted,
    /// A reader answered `Done`.
    Stopped,
    /// A reader refused, or the stream failed under it.
    Refused(String),
}

/// Drain a record's lines through a reader. **A malformed line is skipped
/// rather than fatal**, per section 2's reader rules: what a reader does
/// not know it does not read, and a line that is not an event is exactly
/// that.
pub fn drain<R: BufRead>(source: R, reader: &mut dyn Reader) -> Drained {
    for line in source.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => return Drained::Refused(format!("the stream failed: {error}")),
        };
        let Some(event) = Event::parse(&line) else {
            continue;
        };
        match reader.event(&event) {
            Step::Continue => {}
            Step::Done => return Drained::Stopped,
            Step::Refuse(why) => return Drained::Refused(why),
        }
    }
    Drained::Exhausted
}

/// Open a record for draining: a path, or `-` for the standard input. A
/// FIFO opens here exactly as a file does, the blocking open being what
/// pairs the reader with the writer.
pub fn open(path: &str) -> std::io::Result<Box<dyn BufRead>> {
    if path == "-" {
        Ok(Box::new(std::io::BufReader::new(std::io::stdin())))
    } else {
        Ok(Box::new(std::io::BufReader::new(std::fs::File::open(path)?)))
    }
}
