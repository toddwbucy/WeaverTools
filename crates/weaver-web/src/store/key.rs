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

/// A plan's identity, spelled `pl-` and sixteen hex per section 2.
///
/// An arm's identity does not resolve to a plan's, and the compiler says so
/// rather than the database:
///
/// ```compile_fail
/// use weaver_web::store::{ArmId, Store};
/// # async fn f(store: Store, arm: ArmId) {
/// store.plan(&arm).await.unwrap();
/// # }
/// ```
///
/// The same call with a `PlanId` compiles, which is the pin's other half:
///
/// ```no_run
/// use weaver_web::store::{PlanId, Store};
/// # async fn f(store: Store, plan: PlanId) {
/// store.plan(&plan).await.unwrap();
/// # }
/// ```
///
/// **The kind is in the type and not only in the bytes.** The identities of
/// section 2's authored rows are all text, so a bare `String` lets an arm's
/// identity be handed where a plan's is owed: the schema refuses it, but not
/// until a round trip, and a read so addressed answers `None`, which every
/// caller reads as "no such plan" rather than as "wrong kind of key". The
/// newtype is what makes section 2's claim - that a key of the wrong kind is
/// refused at the boundary - true of the boundary a caller actually meets.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlanId(pub String);

impl std::fmt::Display for PlanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// An arm's identity, spelled `ar-` and sixteen hex per section 2.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArmId(pub String);

impl std::fmt::Display for ArmId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// A run's identity as the record spells it.
///
/// **It carries no prefix and that is the convention rather than an
/// exception to it**, per section 2: a run is a row this crate received, so
/// its identity is the record's spelling and never one this store invented.
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
