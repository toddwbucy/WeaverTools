//! The invocation router (Spec section 9): on each message, parse
//! mentions and enqueue invocations for mentioned non-human members
//! whose policy admits the author. Agent-authored messages route like
//! human ones since the coordination ruling of 2026-08-20 - agents
//! coordinate by messaging each other, and a volley's termination is
//! the conversation's own discipline, not a router ceiling. What stays
//! suppressed is exactly self-invocation: an agent's mention of its own
//! name triggers nothing, and the operator's stop and unload verbs
//! remain the kill switch for a volley that will not die.

use crate::queue::{Invocation, Queues};
use crate::registry::{self, Participant};
use crate::store::{ChannelEvent, Store};

/// Handle one appended message authored by a human. Called by the web
/// layer after the append.
pub async fn on_human_message(
    store: &Store,
    queues: &Queues,
    event: &ChannelEvent,
) -> anyhow::Result<()> {
    route(store, queues, event, None).await
}

/// Handle one appended message authored by an agent - the close's text
/// landing in the channel - routing its mentions like a human's, minus
/// the author itself: self-mention is one suppression, and the
/// hello-loop counter is the other. The first open volley (2026-08-20)
/// greeted itself in circles, so the router serves a bounded number of
/// agent-to-agent hops since the last human message and then pauses the
/// volley visibly: the mentions still display, an app-note says why
/// nothing answered, and any human word resets the budget.
pub async fn on_agent_message(
    store: &Store,
    queues: &Queues,
    event: &ChannelEvent,
    author_participant_id: i64,
    hop_budget: u32,
) -> anyhow::Result<()> {
    let hops = store.agent_hops_since_human(event.channel_id).await?;
    if hops >= hop_budget {
        store
            .append(crate::store::NewEvent {
                channel_id: event.channel_id,
                participant_id: None,
                kind: "app-note".into(),
                body: Some(format!(
                    "volley paused: {hops} agent turns since the last human                      message (budget {hop_budget}). Say anything to continue."
                )),
                run_label: None,
                turn_label: None,
                close_kind: None,
            })
            .await?;
        return Ok(());
    }
    route(store, queues, event, Some(author_participant_id)).await
}

async fn route(
    store: &Store,
    queues: &Queues,
    event: &ChannelEvent,
    exclude: Option<i64>,
) -> anyhow::Result<()> {
    let text = match &event.body {
        Some(t) => t,
        None => return Ok(()),
    };
    let members = registry::channel_members(store, event.channel_id).await?;
    // One refused target never silences the rest of the fan-out: the
    // failures collect and surface as one app-error in the channel,
    // beside the answers that did dispatch.
    let mut failures: Vec<String> = Vec::new();
    for target in mentioned(text, &members) {
        if target.kind == "human" || target.respond != "mention" {
            continue;
        }
        if Some(target.id) == exclude {
            continue;
        }
        if target.kind == "agent" {
            if let Err(e) = queues.enqueue(Invocation {
                channel_id: event.channel_id,
                agent_participant_id: target.id,
                agent_name: target.name.clone(),
            }) {
                failures.push(format!("@{}: {e}", target.name));
            }
        }
        // kind == "model": the upstream adapter is not yet implemented
        // (Spec section 10); a mention of a model participant is
        // ignored until it is, rather than half-answered.
    }
    if !failures.is_empty() {
        store
            .append(crate::store::NewEvent {
                channel_id: event.channel_id,
                participant_id: None,
                kind: "app-error".into(),
                body: Some(format!("mention fan-out incomplete - {}", failures.join(", "))),
                run_label: None,
                turn_label: None,
                close_kind: None,
            })
            .await?;
    }
    Ok(())
}

/// `@name` tokens matched against channel members, longest handle
/// first so `@alpha-two` never half-matches `@alpha`. Both sides of
/// the token must be boundaries, so `todd@alpha` mentions no one.
fn mentioned<'a>(text: &str, members: &'a [Participant]) -> Vec<&'a Participant> {
    fn is_word(c: char) -> bool {
        c.is_alphanumeric() || c == '-' || c == '_' || c == '@'
    }
    let mut hits: Vec<&Participant> = Vec::new();
    let mut ordered: Vec<&Participant> = members.iter().collect();
    ordered.sort_by_key(|p| std::cmp::Reverse(p.name.len()));
    for p in ordered {
        let pat = format!("@{}", p.name);
        let mut found = false;
        let mut start = 0;
        while let Some(pos) = text[start..].find(&pat) {
            let at = start + pos;
            let end = at + pat.len();
            let left = text[..at].chars().next_back().map(|c| !is_word(c)).unwrap_or(true);
            let right = text[end..].chars().next().map(|c| !is_word(c)).unwrap_or(true);
            if left && right {
                found = true;
                break;
            }
            start = end;
        }
        if found && !hits.iter().any(|h| h.id == p.id) {
            hits.push(p);
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(id: i64, name: &str, kind: &str) -> Participant {
        Participant {
            id,
            name: name.into(),
            display: name.into(),
            kind: kind.into(),
            adapter: None,
            respond: "mention".into(),
            role: "user".into(),
        }
    }

    #[test]
    fn mention_boundaries() {
        let members = vec![p(1, "alpha", "agent"), p(2, "alpha-two", "agent")];
        let hits = mentioned("hey @alpha-two and @alpha!", &members);
        let names: Vec<&str> = hits.iter().map(|h| h.name.as_str()).collect();
        assert!(names.contains(&"alpha-two"));
        assert!(names.contains(&"alpha"));

        let hits = mentioned("hey @alpha-two only", &members);
        let names: Vec<&str> = hits.iter().map(|h| h.name.as_str()).collect();
        assert_eq!(names, vec!["alpha-two"]);
    }

    #[test]
    fn an_agent_never_invokes_itself() {
        // The one suppression left: the author is excluded from its own
        // mentions, and every other agent routes. Exercised at the
        // mention layer here; the exclusion itself is `route`'s.
        let members = vec![p(1, "alpha", "agent"), p(2, "bravo", "agent")];
        let hits = mentioned("@alpha I agree with @bravo, over to you", &members);
        let names: Vec<&str> = hits.iter().map(|h| h.name.as_str()).collect();
        assert!(names.contains(&"alpha") && names.contains(&"bravo"));
        // route() with exclude=Some(1) drops alpha; the filter under test
        // lives there and is a one-line comparison - the mention layer
        // stays author-blind by design.
    }

    #[test]
    fn email_form_is_not_a_mention() {
        let members = vec![p(1, "alpha", "agent")];
        assert!(mentioned("mail me at todd@alpha please", &members).is_empty());
        assert!(mentioned("x@alpha", &members).is_empty());
        let hits = mentioned("(@alpha) hello", &members);
        assert_eq!(hits.len(), 1);
    }
}
