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
//! declared length fixes the number, and `ordinal`'s match, exhaustive and
//! wildcard-free, fixes which kinds exist for the array to be checked against.
//! The array is written out rather than derived, so the two assertions below
//! are what close the distance between the two. A twenty-second kind already
//! stops the build at the lib's own matches, which is section 10's compiler
//! claim and not this file's: what this adds is that the count cannot stay
//! behind a kind the lib has absorbed, since an act that answers every match
//! and leaves the number alone stops here.
//!
//! Neither the disposition count nor the mapping's totality is reached, both
//! being properties of `pairing_licensed`, which is crate-private and which
//! this file cannot see from outside the crate. **That is this act's placement
//! and not a property of the crate**: `writer.rs` carries no test module today
//! and a unit test there would see the function and reach all three claims. The
//! cited record keeps its tag because this file reaches one of the three, and
//! the split that would let the tag move is issue #633 rather than an
//! assumption left in prose.

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
/// two assertions read different objects, which is why each can fail with the
/// other passing.
///
/// The first is about `ALL` alone and reaches no match: no kind is named
/// twice. Perturbation: name `Kind::ClassifyRequest` in place of
/// `Kind::ClassifyOutput` and it fails, the twenty-one entries no longer
/// naming twenty-one kinds.
///
/// The second is about `ordinal` against `ALL`: every ordinal the exhaustive
/// match produces is reached. **Marking happens without asserting**, so a
/// collision leaves a slot unset for this assertion to find rather than
/// stopping the walk where the pigeonhole would make the second unreachable.
/// Perturbation: return 19 from the `Kind::ClassifyOutput` arm of `ordinal`
/// and it fails while the first passes, `ALL` being untouched and its
/// twenty-one kinds still distinct.
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

    for (at, kind) in ALL.iter().enumerate() {
        assert!(
            !ALL[..at].contains(kind),
            "the twenty-one entries name one kind twice: {kind:?}"
        );
    }

    let mut reached = [false; ALL.len()];
    for kind in ALL {
        reached[ordinal(kind)] = true;
    }
    assert!(
        reached.iter().all(|seen| *seen),
        "every kind the enum declares stands in the set: {reached:?}"
    );
}
