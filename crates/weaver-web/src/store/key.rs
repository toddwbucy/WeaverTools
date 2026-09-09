//! conforms: web-position-is-addressed-by-run-turn-position
//!
//! The address of a position, per `weaver-web-Spec` section 2.1: the run,
//! the turn, and the position, and the composite of the three is the key.
//!
//! **The pin is that the reads take this type and nothing looser.** A read
//! addressed by three bare values could be handed a turn and a position
//! without a run, which answers one line per run rather than one, because
//! turn keys repeat across a serving record's runs. A read that takes
//! `&PositionKey` cannot be handed less than the whole address, and the
//! compiler holds that rather than a test.
//!
//! **`position` is the resident length at the draw and not an ordinal within
//! the turn**, per section 2.1: the first generated token of a sixty-token
//! prompt is position sixty. The type carries the number as the record
//! spells it and converts nothing, the conversion between ordinal and
//! position being the surface's to make and to make once, per section 6.

use serde::{Deserialize, Serialize};

/// A run's identity as the record spells it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RunId(pub String);

/// A turn's key within its run, repeated across a serving record's runs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TurnId(pub String);

/// The composite address of one position.
///
/// ```
/// use weaver_web::store::{PositionKey, RunId, TurnId};
///
/// let key = PositionKey {
///     run: RunId("2026-09-08T05:06:48.865Z-karl-646bf4eb0582c253".into()),
///     turn: TurnId("t-1".into()),
///     position: 154,
/// };
/// assert_eq!(key.position, 154);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PositionKey {
    pub run: RunId,
    pub turn: TurnId,
    pub position: i32,
}
