//! conforms: trace-kind-payload-mapping-total
//!
//! **The kind set's count, read by the compiler rather than by a reader.**
//! `weaver-trace-Spec` section 3 states the mapping is total over twenty-one
//! kinds and sixteen dispositions, and records that the recount has gone wrong
//! twice, the second time silently, both halves standing at eighteen kinds
//! against a crate compiling thirteen. Until this file the count stood in prose
//! and in no instrument, and a mapping that is stale reads exactly like a
//! mapping that is total.
//!
//! **What this reaches is the kind set and not the mapping.** The array's
//! length fixes the number and the exhaustive match fixes the membership. A
//! twenty-second kind already stops the build at the lib's own matches, which
//! is section 10's compiler claim and not this file's: what this adds is that
//! the count cannot stay behind a kind the lib has absorbed, since an act that
//! answers every match and leaves the number alone stops here.
//!
//! Neither the disposition count nor the mapping's totality is reached: both
//! are properties of `pairing_licensed`, which is private to the crate and
//! outside an integration test's reach, so they stay where section 10 has them
//! and the cited record keeps its tag.

use weaver_trace::Kind;

/// Each kind's ordinal, matched exhaustively and with no wildcard arm, so a
/// kind added to the enum leaves this match non-exhaustive and a kind dropped
/// leaves an arm naming nothing. Either stops the build.
fn ordinal(kind: Kind) -> usize {
    match kind {
        Kind::Load => 0,
        Kind::Unload => 1,
        Kind::SessionClosed => 2,
        Kind::TurnStarted => 3,
        Kind::TurnClosed => 4,
        Kind::MessageSystem => 5,
        Kind::MessageUser => 6,
        Kind::MessageAssistant => 7,
        Kind::MessageToolResult => 8,
        Kind::ToolCallStarted => 9,
        Kind::ToolCallCompleted => 10,
        Kind::Fault => 11,
        Kind::Elision => 12,
        Kind::Refusal => 13,
        Kind::Flush => 14,
        Kind::ModelRequest => 15,
        Kind::ModelOutput => 16,
        Kind::ModelMeasurement => 17,
        Kind::ModelField => 18,
        Kind::ClassifyRequest => 19,
        Kind::ClassifyOutput => 20,
    }
}

/// **The kind set is twenty-one, and each of the twenty-one stands in it
/// once**, per `weaver-trace-Spec` section 3.
///
/// The array's declared length is the count and the compiler checks it. The
/// ordinals come from an exhaustive match, so membership is the enum's own and
/// not this file's reading of it. The two assertions close the one gap a
/// length and a match leave open between them, an array of the right length
/// naming one kind twice and another not at all.
#[test]
fn the_kind_set_is_twenty_one() {
    const ALL: [Kind; 21] = [
        Kind::Load,
        Kind::Unload,
        Kind::SessionClosed,
        Kind::TurnStarted,
        Kind::TurnClosed,
        Kind::MessageSystem,
        Kind::MessageUser,
        Kind::MessageAssistant,
        Kind::MessageToolResult,
        Kind::ToolCallStarted,
        Kind::ToolCallCompleted,
        Kind::Fault,
        Kind::Elision,
        Kind::Refusal,
        Kind::Flush,
        Kind::ModelRequest,
        Kind::ModelOutput,
        Kind::ModelMeasurement,
        Kind::ModelField,
        Kind::ClassifyRequest,
        Kind::ClassifyOutput,
    ];

    let mut reached = [false; 21];
    for kind in ALL {
        let at = ordinal(kind);
        assert!(!reached[at], "one kind stands in the set twice: {kind:?}");
        reached[at] = true;
    }
    assert!(
        reached.iter().all(|seen| *seen),
        "every kind the enum declares stands in the set: {reached:?}"
    );
}
