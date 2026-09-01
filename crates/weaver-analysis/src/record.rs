//! conforms: analysis-parse-skips-the-unknown
//! conforms: analysis-derives-no-absent-member
//!
//! The parse of a serving record, per `weaver-analysis-Spec` section 2: the
//! read types are this crate's own and share no code with the writer, which
//! is the boundary working as intended. `weaver-trace-Spec` section 3 is
//! authoritative for every shape spelled here, per G5.
//!
//! **Envelope-first and payload-lazy.** Every line yields its envelope
//! eagerly, that being what the projection groups and orders by, and a
//! payload is held as raw text until a key path is read out of it, which is
//! what lets an elected value cross this crate byte-identical to what the
//! record spelled.
//!
//! **A kind this crate does not know is skipped, and a payload member it
//! does not know is ignored.** The kind is held as the string the record
//! spelled rather than an enum, so an unknown kind cannot fail a parse and
//! cannot decide a grouping: the walk runs on run and turn from the
//! envelope, which no unrecognised record can move. **A member a record
//! does not carry is absent and is never derived from the members beside
//! it**: this module offers a raw path read and nothing that computes one
//! member from another.

use serde::Deserialize;
use serde_json::value::RawValue;

/// The envelope's five walked members, spelled as the record spells them.
/// The sequence stays the decimal string the canonical form elects, parsed
/// to a number only where an order is compared, so what crosses onward is
/// the record's own spelling.
#[derive(Debug, Clone, Deserialize)]
pub struct Envelope {
    pub session: String,
    pub run: String,
    #[serde(default)]
    pub turn: Option<String>,
    pub sequence: String,
    pub kind: String,
    #[serde(default)]
    pub subsystem: String,
}

/// One record line: the envelope eager, the payload raw.
#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(flatten)]
    pub envelope: Envelope,
    #[serde(default)]
    pub payload: Option<Box<RawValue>>,
}

impl Event {
    /// The line parsed, or `None` for a line that is not an event: a
    /// malformed line answers nothing rather than failing the record, the
    /// reader rule's own posture toward what it does not know.
    pub fn parse(line: &str) -> Option<Event> {
        serde_json::from_str(line).ok()
    }

    /// The sequence as a number, for order comparisons alone. The record's
    /// spelling is what crosses onward.
    pub fn ordinal(&self) -> Option<u64> {
        self.envelope.sequence.parse().ok()
    }
}

/// Walk a dotted key path through raw payload JSON, returning the raw text
/// at the leaf. Each step reparses one object's keys and nothing else, so
/// the value that crosses is byte-identical to the record's spelling. A
/// path the payload does not hold is absent, never defaulted: a miss costs
/// nothing and derives nothing.
pub fn value_at<'a>(payload: &'a RawValue, path: &str) -> Option<&'a RawValue> {
    let mut cursor = payload;
    for segment in path.split('.') {
        let object: std::collections::BTreeMap<std::borrow::Cow<'_, str>, &RawValue> =
            serde_json::from_str(cursor.get()).ok()?;
        cursor = object.get(segment)?;
    }
    Some(cursor)
}

/// Every event of a record, in landing order, malformed lines skipped.
pub fn parse_record(text: &str) -> Vec<Event> {
    text.lines().filter_map(Event::parse).collect()
}
