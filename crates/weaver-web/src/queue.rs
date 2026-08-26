//! Per-agent queue: single-flight, batch-on-drain (Spec section 8).
//! One worker per agent is the single-flight rule made structural.
//! Workers reach the agent through the link's turn service, and
//! agents register dynamically as hellos announce them (Spec
//! section 16).

use crate::channel;
use crate::store::{NewEvent, Store};
use crate::wire::Link;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

/// Bound on pending invocations per agent. Batch-on-drain empties the
/// queue every turn, so hitting this means the agent is badly behind;
/// refusing loudly beats unbounded memory.
const QUEUE_CAP: usize = 256;

/// One pending invocation of an agent in a channel.
#[derive(Debug, Clone)]
pub struct Invocation {
    pub channel_id: i64,
    pub agent_participant_id: i64,
    pub agent_name: String,
}

/// A readable snapshot per agent: queue depth and the channel of the
/// in-flight turn, rendered in the channel view (PRD 4.1).
#[derive(Debug, Clone, Default)]
pub struct AgentState {
    pub depth: usize,
    pub in_flight: Option<i64>, // channel_id of the running turn
}

struct AgentQueue {
    tx: mpsc::Sender<Invocation>,
    state: Arc<Mutex<AgentState>>,
}

#[derive(Clone)]
pub struct Queues {
    agents: Arc<Mutex<HashMap<String, AgentQueue>>>,
    store: Store,
    link: Link,
    hop_budget: u32,
}

impl Queues {
    pub fn new(store: Store, link: Link, hop_budget: u32) -> Self {
        Self {
            agents: Arc::new(Mutex::new(HashMap::new())),
            store,
            link,
            hop_budget,
        }
    }

    /// Start a worker for an agent the roster announced. Idempotent,
    /// and workers are never torn down: a queue over a departed agent
    /// answers with the link's typed refusal rather than vanishing.
    pub fn ensure_agent(&self, name: &str) {
        let mut agents = self.agents.lock().unwrap();
        if agents.contains_key(name) {
            return;
        }
        let (tx, rx) = mpsc::channel(QUEUE_CAP);
        let state = Arc::new(Mutex::new(AgentState::default()));
        tokio::spawn(worker(self.clone(), rx, state.clone()));
        agents.insert(name.to_owned(), AgentQueue { tx, state });
    }

    pub fn enqueue(&self, inv: Invocation) -> anyhow::Result<()> {
        // A restart with the agent's box down leaves a known,
        // mentionable agent with no queue; stand one up so the
        // refusal comes typed from the link rather than from here.
        self.ensure_agent(&inv.agent_name);
        let agents = self.agents.lock().unwrap();
        let q = agents
            .get(&inv.agent_name)
            .ok_or_else(|| anyhow::anyhow!("no queue for agent '{}'", inv.agent_name))?;
        q.tx.try_send(inv).map_err(|e| match e {
            mpsc::error::TrySendError::Full(_) => {
                anyhow::anyhow!("agent queue is full ({QUEUE_CAP} pending)")
            }
            mpsc::error::TrySendError::Closed(_) => anyhow::anyhow!("agent worker is gone"),
        })?;
        // Depth counts only invocations the queue accepted.
        q.state.lock().unwrap().depth += 1;
        Ok(())
    }

    pub fn state(&self, agent_name: &str) -> Option<AgentState> {
        self.agents
            .lock()
            .unwrap()
            .get(agent_name)
            .map(|q| q.state.lock().unwrap().clone())
    }
}

async fn worker(
    queues: Queues,
    mut rx: mpsc::Receiver<Invocation>,
    state: Arc<Mutex<AgentState>>,
) {
    while let Some(first) = rx.recv().await {
        // Batch-on-drain: collect everything pending. Invocations for
        // other channels than the first stay batched per channel.
        let mut batch = vec![first];
        while let Ok(inv) = rx.try_recv() {
            batch.push(inv);
        }
        {
            let mut s = state.lock().unwrap();
            s.depth = s.depth.saturating_sub(batch.len());
        }
        // Group by channel; one turn per channel with pending mentions.
        let mut by_channel: HashMap<i64, Invocation> = HashMap::new();
        for inv in batch {
            by_channel.insert(inv.channel_id, inv);
        }
        for (channel_id, inv) in by_channel {
            state.lock().unwrap().in_flight = Some(channel_id);
            if let Err(e) = run_turn(&queues, &inv).await {
                tracing::error!(agent = %inv.agent_name, "turn handling failed: {e}");
                // Best-effort terminating event so an opened turn is
                // never left dangling in the log.
                let _ = queues
                    .store
                    .append(NewEvent {
                        channel_id,
                        participant_id: Some(inv.agent_participant_id),
                        kind: "app-error".into(),
                        body: Some(format!("turn handling failed: {e}")),
                        run_label: None,
                        turn_label: None,
                        close_kind: None,
                    })
                    .await;
            }
            state.lock().unwrap().in_flight = None;
        }
    }
}

async fn run_turn(queues: &Queues, inv: &Invocation) -> anyhow::Result<()> {
    let store = &queues.store;
    let Some(context) =
        build_context(store, inv.channel_id, inv.agent_participant_id, &inv.agent_name).await?
    else {
        return Ok(());
    };

    store
        .append(NewEvent {
            channel_id: inv.channel_id,
            participant_id: Some(inv.agent_participant_id),
            kind: "turn-open".into(),
            body: Some(inv.agent_name.clone()),
            run_label: None,
            turn_label: None,
            close_kind: None,
        })
        .await?;

    match queues.link.turn(&inv.agent_name, &context).await {
        Ok(close) => {
            let body = close
                .text
                .clone()
                .unwrap_or_else(|| close.raw.to_string());
            let event = store
                .append(NewEvent {
                    channel_id: inv.channel_id,
                    participant_id: Some(inv.agent_participant_id),
                    kind: "close".into(),
                    body: Some(body),
                    run_label: close.run,
                    turn_label: close.turn,
                    close_kind: Some(close.kind),
                })
                .await?;
            // The answer routes like any other message, minus its author:
            // agents coordinate by mentioning each other, per the
            // coordination ruling, and a volley ends when a message
            // carries no mention.
            crate::router::on_agent_message(
                store,
                queues,
                &event,
                inv.agent_participant_id,
                queues.hop_budget,
            )
            .await?;
        }
        Err(e) => {
            // Every gate failure lands as a typed app-error; the
            // variants carry their distinction in the message, link
            // loss included (Spec section 16).
            store
                .append(NewEvent {
                    channel_id: inv.channel_id,
                    participant_id: Some(inv.agent_participant_id),
                    kind: "app-error".into(),
                    body: Some(e.to_string()),
                    run_label: None,
                    turn_label: None,
                    close_kind: None,
                })
                .await?;
        }
    }
    Ok(())
}

/// Prompt serialization per Spec section 7: speaker-labeled messages
/// since the agent's last close, truncated oldest-first to the bound,
/// the final message always kept.
async fn build_context(
    store: &Store,
    channel_id: i64,
    agent_participant_id: i64,
    agent_name: &str,
) -> anyhow::Result<Option<String>> {
    let msgs =
        channel::messages_since_last_close(store, channel_id, agent_participant_id).await?;
    // **An empty window does not invoke.** Batching and in-flight turns
    // race: a mention enqueues the agent, the agent's current turn closes
    // after that mention landed, and the queued invocation's window is
    // then empty - its justification already consumed. A turn on an empty
    // prompt is a model improvising into a void (the 2026-08-20 traces
    // show 31K-character thinking sprees answering nothing), so a stale
    // invocation is dropped rather than served.
    if msgs.is_empty() {
        return Ok(None);
    }
    let header = format!(
        "Turn context. You are {agent_name}. Messages since your last turn:\n"
    );
    let mut lines: Vec<String> = msgs
        .iter()
        .map(|m| {
            format!(
                "{}: {}",
                m.author_name.as_deref().unwrap_or("unknown"),
                m.body.as_deref().unwrap_or("")
            )
        })
        .collect();

    // The bound the gate enforces is the serialized line's, JSON
    // escaping included, so the trim measures exactly what the
    // adapter will send rather than estimating around it - a refused
    // over-bound line is a dropped turn where a trimmed one serves.
    const BOUND: usize = crate::adapters::gate::LINE_BOUND;
    let serialized_len = |ls: &[String]| -> usize {
        #[derive(serde::Serialize)]
        struct Request<'a> {
            text: &'a str,
        }
        let text = format!("{header}{}", ls.join("\n"));
        serde_json::to_string(&Request { text: &text })
            .map(|s| s.len())
            .unwrap_or(usize::MAX)
    };
    while lines.len() > 1 && serialized_len(&lines) > BOUND {
        lines.remove(0);
    }
    // One message can exceed the bound alone. It truncates on a char
    // boundary with the marker counted inside the bound, so the
    // mention that justified the turn still arrives, marked rather
    // than refused (Spec section 7).
    const MARKER: &str = " ...[truncated to the line bound]";
    if serialized_len(&lines) > BOUND {
        let mut only = lines.pop().unwrap_or_default();
        loop {
            let candidate = format!("{only}{MARKER}");
            let len = serialized_len(std::slice::from_ref(&candidate));
            if len <= BOUND || only.is_empty() {
                only = candidate;
                break;
            }
            let keep = only.chars().count().saturating_sub((len - BOUND).max(1));
            only = only.chars().take(keep).collect();
        }
        lines.push(only);
    }
    Ok(Some(format!("{header}{}", lines.join("\n"))))
}
