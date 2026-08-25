//! The link (Spec section 16): NDJSON frames over one TCP connection
//! the connector dials. Five services - turn, verb, trace, status,
//! declaration - ask and answer correlated by id where the shape is
//! ask-answer, the trace streaming unasked. Link loss is marked, never
//! smoothed: pending asks fail typed, and the server inserts
//! discontinuity marks into every trace view.

use crate::adapters::gate::{GateAdapter, GateClose, GateError};
use crate::config::ConnectorConfig;
use crate::lifecycle::{self, VerbOutcome};
use crate::traceview::{self, TraceEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot, RwLock};

/// Frames the server sends the connector: the ask half of the
/// ask-answer services.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "svc", rename_all = "snake_case")]
pub enum ToConnector {
    Turn { id: u64, agent: String, text: String },
    Verb { id: u64, agent: String, verb: String },
    Status { id: u64 },
    Declaration { id: u64, agent: String },
}

/// The gate adapter's error, carried over the link with its typing
/// intact (Spec section 16: section 6's variants verbatim in kind).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGateError {
    pub kind: String,
    pub message: String,
}

impl From<&GateError> for WireGateError {
    fn from(e: &GateError) -> Self {
        let kind = match e {
            GateError::Unloaded => "unloaded",
            GateError::LineTooLong(_) => "line_too_long",
            GateError::DeliveryLost(_) => "delivery_lost",
            GateError::BadClose(_) => "bad_close",
            GateError::CloseTooLong(_) => "close_too_long",
        };
        Self { kind: kind.into(), message: e.to_string() }
    }
}

/// Frames the connector sends the server: the hello, the answers, and
/// the unasked trace stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "svc", rename_all = "snake_case")]
pub enum ToServer {
    /// First frame on every connection: the box's agent roster by
    /// name. Paths stay on the box (Spec section 16).
    Hello { agents: Vec<String> },
    Turn {
        id: u64,
        close: Option<GateClose>,
        error: Option<WireGateError>,
    },
    Verb {
        id: u64,
        outcome: Option<VerbOutcome>,
        error: Option<String>,
    },
    Status { id: u64, agents: HashMap<String, bool> },
    Declaration { id: u64, path: String, content: String },
    Trace { agent: String, event: TraceEvent },
}

/// What a turn ask can fail with on the server side: the gate's own
/// typed error relayed, or the link itself being down.
#[derive(Debug, Clone)]
pub enum TurnError {
    Gate(WireGateError),
    LinkDown,
}

impl std::fmt::Display for TurnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TurnError::Gate(e) => write!(f, "{}", e.message),
            TurnError::LinkDown => {
                write!(f, "agent unreachable: the link to the agents' box is down")
            }
        }
    }
}

/// How long the server waits on the small asks (status, declaration)
/// before reporting the link unresponsive. Turns and verbs carry no
/// server-side deadline: the gate serializes turns and a queued turn
/// legitimately waits (Spec section 6), and the verb's 300 s ceiling
/// is the connector's (Spec section 11).
const SMALL_ASK_TIMEOUT_SECS: u64 = 10;

/// Events the link surfaces to the composition root.
#[derive(Debug, Clone)]
pub enum LinkEvent {
    /// A connector said hello: the roster, fresh on every connection.
    Hello(Vec<String>),
    /// A trace event or mark arrived for an agent.
    Trace { agent: String, event: TraceEvent },
    /// The connection dropped. Pending asks have already failed.
    Down,
}

struct LinkInner {
    /// Write path to the live connection's writer task. None while
    /// the link is down.
    tx: Mutex<Option<mpsc::Sender<ToConnector>>>,
    pending: Mutex<HashMap<u64, oneshot::Sender<ToServer>>>,
    next_id: AtomicU64,
    /// The latest hello's roster. Survives a link drop so the
    /// surfaces keep naming the agents they knew, honestly marked
    /// unreachable rather than vanished.
    roster: RwLock<Vec<String>>,
}

/// The server's handle on the link: asks, the roster, and liveness.
#[derive(Clone)]
pub struct Link {
    inner: Arc<LinkInner>,
}

impl Link {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(LinkInner {
                tx: Mutex::new(None),
                pending: Mutex::new(HashMap::new()),
                next_id: AtomicU64::new(1),
                roster: RwLock::new(Vec::new()),
            }),
        }
    }

    pub fn is_up(&self) -> bool {
        self.inner.tx.lock().unwrap().is_some()
    }

    pub async fn roster(&self) -> Vec<String> {
        self.inner.roster.read().await.clone()
    }

    pub async fn has_agent(&self, name: &str) -> bool {
        self.inner.roster.read().await.iter().any(|a| a == name)
    }

    async fn ask(
        &self,
        make: impl FnOnce(u64) -> ToConnector,
    ) -> Result<ToServer, ()> {
        let id = self.inner.next_id.fetch_add(1, Ordering::Relaxed);
        let (reply_tx, reply_rx) = oneshot::channel();
        let sender = {
            let tx = self.inner.tx.lock().unwrap();
            match tx.as_ref() {
                Some(s) => s.clone(),
                None => return Err(()),
            }
        };
        self.inner.pending.lock().unwrap().insert(id, reply_tx);
        if sender.send(make(id)).await.is_err() {
            self.inner.pending.lock().unwrap().remove(&id);
            return Err(());
        }
        // A dropped connection fails every pending ask (fail_pending),
        // so this await ends when the answer or the disconnect does.
        reply_rx.await.map_err(|_| ())
    }

    /// One turn across the link: the connector dials the gate per
    /// turn (Spec section 6) and answers with the close or the typed
    /// error.
    pub async fn turn(&self, agent: &str, text: &str) -> Result<GateClose, TurnError> {
        let (agent, text) = (agent.to_owned(), text.to_owned());
        match self.ask(|id| ToConnector::Turn { id, agent, text }).await {
            Ok(ToServer::Turn { close: Some(c), .. }) => Ok(c),
            Ok(ToServer::Turn { error: Some(e), .. }) => Err(TurnError::Gate(e)),
            _ => Err(TurnError::LinkDown),
        }
    }

    /// One verb invocation across the link (Spec section 11).
    pub async fn verb(&self, agent: &str, verb: &str) -> anyhow::Result<VerbOutcome> {
        let (agent, verb) = (agent.to_owned(), verb.to_owned());
        match self.ask(|id| ToConnector::Verb { id, agent, verb }).await {
            Ok(ToServer::Verb { outcome: Some(o), .. }) => Ok(o),
            Ok(ToServer::Verb { error: Some(e), .. }) => anyhow::bail!("{e}"),
            _ => anyhow::bail!("the link to the agents' box is down"),
        }
    }

    /// The load-state observable per agent: socket existence, the
    /// inference the UI labels (PRD 4.2). None when the link is down
    /// or unresponsive, which the caller renders as unreachable.
    pub async fn status(&self) -> Option<HashMap<String, bool>> {
        let fut = self.ask(|id| ToConnector::Status { id });
        match tokio::time::timeout(
            std::time::Duration::from_secs(SMALL_ASK_TIMEOUT_SECS),
            fut,
        )
        .await
        {
            Ok(Ok(ToServer::Status { agents, .. })) => Some(agents),
            _ => None,
        }
    }

    /// The agent declaration, read on the box: (path, content).
    pub async fn declaration(&self, agent: &str) -> Option<(String, String)> {
        let agent = agent.to_owned();
        let fut = self.ask(|id| ToConnector::Declaration { id, agent });
        match tokio::time::timeout(
            std::time::Duration::from_secs(SMALL_ASK_TIMEOUT_SECS),
            fut,
        )
        .await
        {
            Ok(Ok(ToServer::Declaration { path, content, .. })) => Some((path, content)),
            _ => None,
        }
    }

    fn fail_pending(&self) {
        // Dropping the senders fails every waiting ask.
        self.inner.pending.lock().unwrap().clear();
    }
}

impl Default for Link {
    fn default() -> Self {
        Self::new()
    }
}

/// The server's accept loop. One connector at a time in v1: a new
/// connection replaces the old, which self-heals a half-open drop the
/// server has not yet noticed.
pub async fn serve(link: Link, listener: TcpListener, events: mpsc::Sender<LinkEvent>) {
    loop {
        let (stream, peer) = match listener.accept().await {
            Ok(x) => x,
            Err(e) => {
                tracing::error!("link accept failed: {e}");
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
        };
        tracing::info!("link connected from {peer}");
        // Tear down any prior connection's write path first.
        if link.inner.tx.lock().unwrap().take().is_some() {
            link.fail_pending();
            let _ = events.send(LinkEvent::Down).await;
        }
        serve_connection(&link, stream, &events).await;
        tracing::info!("link from {peer} closed");
        *link.inner.tx.lock().unwrap() = None;
        link.fail_pending();
        let _ = events.send(LinkEvent::Down).await;
    }
}

async fn serve_connection(link: &Link, stream: TcpStream, events: &mpsc::Sender<LinkEvent>) {
    let (read_half, mut write_half) = stream.into_split();
    let (tx, mut rx) = mpsc::channel::<ToConnector>(64);
    let writer = tokio::spawn(async move {
        while let Some(frame) = rx.recv().await {
            let Ok(mut line) = serde_json::to_string(&frame) else { continue };
            line.push('\n');
            if write_half.write_all(line.as_bytes()).await.is_err() {
                return;
            }
        }
    });

    let mut reader = BufReader::new(read_half).lines();
    let mut said_hello = false;
    while let Ok(Some(line)) = reader.next_line().await {
        let frame: ToServer = match serde_json::from_str(&line) {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("link frame did not parse, closing: {e}");
                break;
            }
        };
        // The first frame must be the hello (Spec section 16); only a
        // greeted connection gets the write path.
        if !said_hello {
            match frame {
                ToServer::Hello { agents } => {
                    said_hello = true;
                    *link.inner.roster.write().await = agents.clone();
                    *link.inner.tx.lock().unwrap() = Some(tx.clone());
                    let _ = events.send(LinkEvent::Hello(agents)).await;
                    continue;
                }
                _ => {
                    tracing::error!("link spoke before hello, closing");
                    break;
                }
            }
        }
        match frame {
            ToServer::Hello { agents } => {
                // A repeated hello refreshes the roster.
                *link.inner.roster.write().await = agents.clone();
                let _ = events.send(LinkEvent::Hello(agents)).await;
            }
            ToServer::Trace { agent, event } => {
                let _ = events.send(LinkEvent::Trace { agent, event }).await;
            }
            ToServer::Turn { id, .. }
            | ToServer::Verb { id, .. }
            | ToServer::Status { id, .. }
            | ToServer::Declaration { id, .. } => {
                if let Some(reply) = link.inner.pending.lock().unwrap().remove(&id) {
                    let _ = reply.send(frame);
                }
            }
        }
    }
    writer.abort();
}

/// The connector's whole life: dial the server, say hello, tail the
/// traces, answer asks, and redial with backoff when the link drops.
/// Reconnection re-runs the hello and a fresh trace backfill (Spec
/// section 16).
pub async fn connector_run(cfg: Arc<ConnectorConfig>) {
    let gates: Arc<HashMap<String, GateAdapter>> = Arc::new(
        cfg.agents
            .iter()
            .map(|a| (a.name.clone(), GateAdapter::new(&a.gate)))
            .collect(),
    );
    let mut backoff = 1u64;
    loop {
        match TcpStream::connect(&cfg.server).await {
            Ok(stream) => {
                tracing::info!("link established to {}", cfg.server);
                backoff = 1;
                connector_connection(&cfg, gates.clone(), stream).await;
                tracing::warn!("link to {} lost", cfg.server);
            }
            Err(e) => {
                tracing::warn!("link dial to {} failed: {e}", cfg.server);
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(backoff)).await;
        backoff = (backoff * 2).min(30);
    }
}

async fn connector_connection(
    cfg: &ConnectorConfig,
    gates: Arc<HashMap<String, GateAdapter>>,
    stream: TcpStream,
) {
    let (read_half, mut write_half) = stream.into_split();
    let (tx, mut rx) = mpsc::channel::<ToServer>(256);

    let writer = tokio::spawn(async move {
        while let Some(frame) = rx.recv().await {
            let Ok(mut line) = serde_json::to_string(&frame) else { continue };
            line.push('\n');
            if write_half.write_all(line.as_bytes()).await.is_err() {
                return;
            }
        }
    });

    let hello = ToServer::Hello {
        agents: cfg.agents.iter().map(|a| a.name.clone()).collect(),
    };
    if tx.send(hello).await.is_err() {
        writer.abort();
        return;
    }

    // Fresh tailers per connection: fresh backfill, per Spec section
    // 16. The server marks the reconnect, so the re-read is bracketed.
    let mut tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();
    for a in &cfg.agents {
        let (agent, path, tx) = (a.name.clone(), a.trace.clone(), tx.clone());
        tasks.push(tokio::spawn(async move {
            let (ev_tx, mut ev_rx) = mpsc::channel::<TraceEvent>(256);
            tokio::spawn(traceview::tail_task(path, ev_tx));
            while let Some(event) = ev_rx.recv().await {
                if tx
                    .send(ToServer::Trace { agent: agent.clone(), event })
                    .await
                    .is_err()
                {
                    return;
                }
            }
        }));
    }

    let mut reader = BufReader::new(read_half).lines();
    while let Ok(Some(line)) = reader.next_line().await {
        let frame: ToConnector = match serde_json::from_str(&line) {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("link ask did not parse, closing: {e}");
                break;
            }
        };
        // Every ask runs concurrently: a turn holds the gate for
        // minutes while status must answer now. Answers serialize
        // through the writer's channel.
        let (gates, cfg_decl, tx) =
            (gates.clone(), cfg.agent_declarations.clone(), tx.clone());
        let (admin_bin, admin_config) =
            (cfg.admin_bin.clone(), cfg.admin_config.clone());
        tasks.push(tokio::spawn(async move {
            let answer = match frame {
                ToConnector::Turn { id, agent, text } => match gates.get(&agent) {
                    Some(gate) => match gate.turn(&text).await {
                        Ok(close) => ToServer::Turn { id, close: Some(close), error: None },
                        Err(e) => ToServer::Turn {
                            id,
                            close: None,
                            error: Some(WireGateError::from(&e)),
                        },
                    },
                    None => ToServer::Turn {
                        id,
                        close: None,
                        error: Some(WireGateError {
                            kind: "no_such_agent".into(),
                            message: format!("no agent '{agent}' on this box"),
                        }),
                    },
                },
                ToConnector::Verb { id, agent, verb } => {
                    match lifecycle::run_verb(&verb, &agent, &admin_bin, &admin_config).await
                    {
                        Ok(outcome) => ToServer::Verb { id, outcome: Some(outcome), error: None },
                        Err(e) => ToServer::Verb { id, outcome: None, error: Some(e.to_string()) },
                    }
                }
                ToConnector::Status { id } => ToServer::Status {
                    id,
                    agents: gates
                        .iter()
                        .map(|(name, gate)| (name.clone(), gate.socket_exists()))
                        .collect(),
                },
                ToConnector::Declaration { id, agent } => {
                    let path = cfg_decl.join(format!("{agent}.yaml"));
                    let content = match tokio::fs::read_to_string(&path).await {
                        Ok(c) => c,
                        Err(e) => format!("could not read the declaration: {e}"),
                    };
                    ToServer::Declaration {
                        id,
                        path: path.display().to_string(),
                        content,
                    }
                }
            };
            let _ = tx.send(answer).await;
        }));
    }

    // The connection is gone: everything serving it dies with it. An
    // in-flight turn's gate connection drops, and the record holds the
    // close (Spec section 6).
    for t in &tasks {
        t.abort();
    }
    writer.abort();
}
