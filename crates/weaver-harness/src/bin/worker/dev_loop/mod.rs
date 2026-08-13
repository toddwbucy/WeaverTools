//! Your loop lives here.
//!
//! This is `dev_loop`, the developer's directory, per `weaver-harness-Spec`
//! sections 1 and 6: any directory carrying the `dev_` prefix is yours to
//! edit, and no directory without it is. The bare decode loop below is the
//! worked default, occupying the exact location a replacement goes, so the
//! example and the extension point are the same place.
//!
//! The crossing is the one function loop 0 calls: the granted seat and the
//! parsed request come in, the turn's outcome goes back, and that is the
//! whole of the traffic across this boundary. The seat is the only surface
//! you hold, and a loop that needs a port the seat does not offer is a
//! charter change entering through the front door, not an import.

use weaver_harness::{Ports, TurnError, TurnOutcome};
use weaver_traits::{ContentBlock, Message, Role};

/// The bare decode loop, loop 1: one turn per request. The request's text
/// becomes the user message, the seat drives the decode, and the outcome is
/// the deliverable. Replace this body with your loop; the signature is the
/// crossing.
pub fn drive(seat: &mut Ports<'_>, text: &str) -> Result<TurnOutcome, TurnError> {
    let delta = vec![Message {
        role: Role::User,
        content: vec![ContentBlock::Text {
            text: text.to_string(),
        }],
    }];
    seat.turn(delta)
}
