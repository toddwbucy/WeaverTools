//! Your loop lives here.
//!
//! This is `dev_loop`, the developer's directory, per `weaver-harness-Spec`
//! sections 1 and 6: any directory carrying the `dev_` prefix is yours to
//! edit, and no directory without it is. The loop below is the operator's
//! first control loop, occupying the exact location the bare decode loop
//! held, so the example and the extension point remain the same place.
//!
//! The crossing is the one function loop 0 calls: the granted seat and the
//! parsed request come in, the turn's outcome goes back, and that is the
//! whole of the traffic across this boundary. The seat is the only surface
//! you hold, and a loop that needs a port the seat does not offer is a
//! charter change entering through the front door, not an import.

use weaver_harness::{Ports, TurnError, TurnOutcome};
use weaver_traits::{ContentBlock, Message, Role};

/// The system prompt, the loop's own content, per the operator's direction
/// of 2026-08-18: contained with the control loop's code rather than the
/// agent declaration, injected as the first contribution, and thereby
/// recorded, distilled, and held like everything else the model ever
/// received. In the field vocabulary it is the first narrowing, applied
/// once per decode session.
///
/// **Edit this constant to shape your agent.** It rides as a user-role
/// message because the floor's `Role` carries no system variant today,
/// which is the standing practice of every declaration-carried identity so
/// far, relocated. The true system slot arrives with the `Role::System`
/// floor act and this constant moves roles without moving homes.
const SYSTEM_PROMPT: &str = "\
You are a careful assistant. Answer from what you know, say plainly when \
you do not know, and keep answers as short as the question allows.";

/// The first control loop, loop 1: the system prompt once per session's
/// standing, then one turn per request.
///
/// The first-turn test reads the seat's assembly: an empty message history
/// means no turn has run against this decode session, so the prompt leads
/// the delta and the narrowing base is established before the first
/// request's content. Every later turn finds history standing and
/// contributes only itself. A fresh run of the same session re-establishes
/// the base, which is what a fresh decode session needs: the resident
/// prefix died with the last process, and the record shows each run's
/// narrowing from its own first event.
pub fn drive(seat: &mut Ports<'_>, text: &str) -> Result<TurnOutcome, TurnError> {
    let first_turn = seat
        .assembled()
        .is_none_or(|prompt| prompt.messages.is_empty());
    let mut delta = Vec::new();
    if first_turn {
        delta.push(Message {
            role: Role::User,
            content: vec![ContentBlock::Text {
                text: SYSTEM_PROMPT.to_string(),
            }],
        });
    }
    delta.push(Message {
        role: Role::User,
        content: vec![ContentBlock::Text {
            text: text.to_string(),
        }],
    });
    seat.turn(delta)
}
