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

use weaver_harness::{Ports, SessionShape, TurnError, TurnOutcome};
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
    // The context injection, per the loop's charter as the state seam's
    // first asker: on a run's first turn the loop consults the session's
    // shape and narrows the field with what the session already holds. The
    // ask costs one bounded exchange and a missing answer costs nothing
    // but the line, the leg being optional by presence.
    let continuity = if first_turn {
        seat.session_shape().and_then(|shape| continuity_line(&shape))
    } else {
        None
    };
    seat.turn(contribution(first_turn, continuity.as_deref(), text))
}

/// The loop's judgment over the served shape, and the place the three-way
/// division puts it: the custodian counted kinds without opinion, and
/// deciding that `turn.closed` counts a completed turn is this loop's
/// business. The shape's last run is the run now open - its load event
/// landed before any turn, per the tee attaching ahead of the run's
/// opening - so the earlier runs are the session's past, and a session
/// with no past injects nothing.
fn continuity_line(shape: &SessionShape) -> Option<String> {
    let prior = shape.runs.len().checked_sub(1)?;
    if prior == 0 {
        return None;
    }
    let turns: u64 = shape.runs[..prior]
        .iter()
        .map(|run| {
            run.kinds
                .iter()
                .find(|(kind, _)| kind == "turn.closed")
                .map_or(0, |(_, count)| *count)
        })
        .sum();
    let runs_word = if prior == 1 { "run" } else { "runs" };
    let turns_word = if turns == 1 { "turn" } else { "turns" };
    Some(format!(
        "Continuity: this session held {prior} earlier {runs_word} with {turns} \
         completed {turns_word} before this one. The conversation content of \
         those runs is not in your context."
    ))
}

/// The turn's delta from the loop's one judgment: the prompt leads a first
/// turn and no other, the continuity line riding beneath it where the
/// session holds a past. Pure, because the seat cannot be minted outside
/// loop zero - the E0624 blade of `weaver-harness-Spec` section 8 - so
/// what this loop can pin is its own contribution, and the seat's half is
/// loop zero's suite's to hold.
fn contribution(first_turn: bool, continuity: Option<&str>, text: &str) -> Vec<Message> {
    let mut delta = Vec::new();
    if first_turn {
        let opening = match continuity {
            Some(line) => format!("{SYSTEM_PROMPT}\n\n{line}"),
            None => SYSTEM_PROMPT.to_string(),
        };
        delta.push(Message {
            role: Role::User,
            content: vec![ContentBlock::Text { text: opening }],
        });
    }
    delta.push(Message {
        role: Role::User,
        content: vec![ContentBlock::Text {
            text: text.to_string(),
        }],
    });
    delta
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(delta: &[Message]) -> Vec<&str> {
        delta
            .iter()
            .flat_map(|message| {
                message.content.iter().map(|block| match block {
                    ContentBlock::Text { text } => text.as_str(),
                    _ => "",
                })
            })
            .collect()
    }

    /// The one-time injection contract: a first turn leads with the prompt,
    /// in order, and every later turn contributes the request alone.
    #[test]
    fn the_prompt_leads_the_first_turn_and_no_other() {
        let first = contribution(true, None, "hello");
        assert_eq!(texts(&first), vec![SYSTEM_PROMPT, "hello"]);
        assert!(first.iter().all(|m| m.role == Role::User));

        let later = contribution(false, None, "and again");
        assert_eq!(texts(&later), vec!["and again"]);
    }

    /// The continuity line rides beneath the prompt in the same opening
    /// message, so the injection stays one message and one narrowing.
    #[test]
    fn continuity_rides_beneath_the_prompt() {
        let first = contribution(true, Some("Continuity: held."), "hello");
        assert_eq!(first.len(), 2);
        let opening = &texts(&first)[0];
        assert!(opening.starts_with(SYSTEM_PROMPT), "{opening}");
        assert!(opening.ends_with("Continuity: held."), "{opening}");
    }

    /// The loop's judgment over the served shape: a session with no past
    /// injects nothing, the current run being the shape's last, and the
    /// earlier runs sum their closed turns.
    #[test]
    fn the_continuity_line_counts_the_past_and_only_the_past() {
        let run = |name: &str, closed: u64| weaver_harness::RunShape {
            run: name.to_string(),
            kinds: vec![
                ("load".to_string(), 1),
                ("turn.closed".to_string(), closed),
            ],
        };
        let alone = SessionShape {
            runs: vec![run("r-1", 0)],
        };
        assert!(continuity_line(&alone).is_none(), "no past, no line");
        let storied = SessionShape {
            runs: vec![run("r-1", 3), run("r-2", 1), run("r-3", 0)],
        };
        let line = continuity_line(&storied).expect("a past injects");
        assert!(line.contains("2 earlier runs"), "{line}");
        assert!(line.contains("4 completed turns"), "{line}");
    }
}
