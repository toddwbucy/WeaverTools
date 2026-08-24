//! Per-agent queue: single-flight, batch-on-drain (Spec section 8).
//! One worker per agent is the single-flight rule made structural.

use crate::adapters::gate::GateAdapter;
use crate::channel;
use crate::store::{NewEvent, Store};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
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
    agents: Arc<HashMap<String, AgentQueue>>,
}

impl Queues {
    pub fn start(store: Store, adapters: Vec<(String, GateAdapter)>, hop_budget: u32) -> Self {
        // The workers need the assembled handle to route agent-authored
        // mentions (the coordination ruling of 2026-08-20), and the handle
        // needs the workers: a OnceLock breaks the cycle, set exactly once
        // below, read by every worker after its first message.
        let handle: Arc<OnceLock<Queues>> = Arc::new(OnceLock::new());
        let mut agents = HashMap::new();
        for (name, adapter) in adapters {
            let (tx, rx) = mpsc::channel(QUEUE_CAP);
            let state = Arc::new(Mutex::new(AgentState::default()));
            tokio::spawn(worker(
                store.clone(),
                adapter,
                rx,
                state.clone(),
                handle.clone(),
                hop_budget,
            ));
            agents.insert(name, AgentQueue { tx, state });
        }
        let queues = Self { agents: Arc::new(agents) };
        let _ = handle.set(queues.clone());
        queues
    }

    pub fn enqueue(&self, inv: Invocation) -> anyhow::Result<()> {
        let q = self
            .agents
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
            .get(agent_name)
            .map(|q| q.state.lock().unwrap().clone())
    }
}

async fn worker(
    store: Store,
    adapter: GateAdapter,
    mut rx: mpsc::Receiver<Invocation>,
    state: Arc<Mutex<AgentState>>,
    queues: Arc<OnceLock<Queues>>,
    hop_budget: u32,
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
            if let Err(e) = run_turn(&store, &adapter, &inv, &queues, hop_budget).await {
                tracing::error!(agent = %inv.agent_name, "turn handling failed: {e}");
                // Best-effort terminating event so an opened turn is
                // never left dangling in the log.
                let _ = store
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

async fn run_turn(
    store: &Store,
    adapter: &GateAdapter,
    inv: &Invocation,
    queues: &OnceLock<Queues>,
    hop_budget: u32,
) -> anyhow::Result<()> {
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

    match adapter.turn(&context).await {
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
            if let Some(queues) = queues.get() {
                crate::router::on_agent_message(
                    store,
                    queues,
                    &event,
                    inv.agent_participant_id,
                    hop_budget,
                )
                .await?;
            }
        }
        Err(e) => {
            // Every gate failure lands as a typed app-error; the
            // variants carry their distinction in the message.
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

    // Budget: the bound minus the JSON envelope's overhead for this
    // exact header. Serialize-and-check is authoritative in the
    // adapter; this trim just gets us under it with margin.
    const BUDGET: usize = crate::adapters::gate::LINE_BOUND - 1024;
    let total = |ls: &[String]| header.len() + ls.iter().map(|l| l.len() + 1).sum::<usize>();
    while lines.len() > 1 && total(&lines) > BUDGET {
        lines.remove(0);
    }
    Ok(Some(format!("{header}{}", lines.join("\n"))))
}
