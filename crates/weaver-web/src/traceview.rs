//! Trace tailers (Spec section 12): per agent, backfill a bounded
//! window, then follow appends by polling. Events are tolerant JSON:
//! known envelope fields typed, everything retained raw. Rotation or
//! truncation surfaces as a discontinuity mark.

use serde::Serialize;
use std::collections::{HashMap, VecDeque};
use std::io::SeekFrom;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader};
use tokio::sync::broadcast;

const BACKFILL_BYTES: u64 = 1024 * 1024;
const RING_CAP: usize = 10_000;
const POLL_MS: u64 = 500;

#[derive(Debug, Clone, Serialize)]
pub struct TraceEvent {
    pub seq: u64,
    /// None for real events; Some(reason) for discontinuity marks the
    /// tailer itself inserted (rotation, truncation, parse failure).
    pub mark: Option<String>,
    pub run: Option<String>,
    pub turn: Option<String>,
    pub kind: Option<String>,
    pub raw: serde_json::Value,
}

pub struct Tailer {
    ring: Arc<Mutex<VecDeque<TraceEvent>>>,
    tx: broadcast::Sender<TraceEvent>,
}

#[derive(Clone)]
pub struct TraceViews {
    tailers: Arc<HashMap<String, Arc<Tailer>>>,
}

impl TraceViews {
    pub fn start(agents: Vec<(String, PathBuf)>) -> Self {
        let mut tailers = HashMap::new();
        for (name, path) in agents {
            let (tx, _) = broadcast::channel(1024);
            let tailer = Arc::new(Tailer {
                ring: Arc::new(Mutex::new(VecDeque::with_capacity(RING_CAP))),
                tx,
            });
            tokio::spawn(tail_task(path, tailer.clone()));
            tailers.insert(name, tailer);
        }
        Self { tailers: Arc::new(tailers) }
    }

    pub fn snapshot(&self, agent: &str) -> Option<Vec<TraceEvent>> {
        self.tailers
            .get(agent)
            .map(|t| t.ring.lock().unwrap().iter().cloned().collect())
    }

    pub fn subscribe(&self, agent: &str) -> Option<broadcast::Receiver<TraceEvent>> {
        self.tailers.get(agent).map(|t| t.tx.subscribe())
    }
}

fn push(tailer: &Tailer, ev: TraceEvent) {
    {
        let mut ring = tailer.ring.lock().unwrap();
        if ring.len() == RING_CAP {
            ring.pop_front();
        }
        ring.push_back(ev.clone());
    }
    let _ = tailer.tx.send(ev);
}

fn parse_line(seq: u64, line: &str) -> TraceEvent {
    match serde_json::from_str::<serde_json::Value>(line) {
        Ok(raw) => {
            let get = |k: &str| raw.get(k).and_then(|v| v.as_str()).map(str::to_owned);
            TraceEvent {
                seq,
                mark: None,
                run: get("run"),
                turn: get("turn"),
                kind: get("kind"),
                raw,
            }
        }
        Err(e) => TraceEvent {
            seq,
            mark: Some(format!("line did not parse as JSON: {e}")),
            run: None,
            turn: None,
            kind: None,
            raw: serde_json::Value::String(line.to_owned()),
        },
    }
}

async fn tail_task(path: PathBuf, tailer: Arc<Tailer>) {
    use std::os::unix::fs::MetadataExt;
    // Sequence ids stay monotonic across weaver-web restarts (a
    // reconnecting browser replays Last-Event-ID from before the
    // restart), by basing them on the epoch second at startup.
    let mut seq: u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
        * 1_000_000;
    let mut pos: u64 = 0;
    let mut opened = false;
    let mut skip_partial_first = false;
    let mut identity: Option<(u64, u64)> = None;

    loop {
        match File::open(&path).await {
            Ok(mut file) => {
                let meta = file.metadata().await.ok();
                let len = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                let ident = meta.as_ref().map(|m| (m.dev(), m.ino()));
                if !opened {
                    // Backfill window on first open.
                    pos = len.saturating_sub(BACKFILL_BYTES);
                    opened = true;
                    identity = ident;
                    if pos > 0 {
                        skip_partial_first = true;
                        seq += 1;
                        push(&tailer, TraceEvent {
                            seq,
                            mark: Some(format!("backfill starts {pos} bytes into the file")),
                            run: None, turn: None, kind: None,
                            raw: serde_json::Value::Null,
                        });
                    }
                } else if (ident.is_some() && identity.is_some() && ident != identity)
                    || len < pos
                {
                    // A different inode, or a shrink in place: either
                    // way a discontinuity, marked, never smoothed.
                    seq += 1;
                    push(&tailer, TraceEvent {
                        seq,
                        mark: Some(if ident != identity {
                            "file replaced: rotation".into()
                        } else {
                            "file shrank: truncation".into()
                        }),
                        run: None, turn: None, kind: None,
                        raw: serde_json::Value::Null,
                    });
                    pos = 0;
                    identity = ident;
                    skip_partial_first = false;
                }
                if len > pos {
                    if file.seek(SeekFrom::Start(pos)).await.is_ok() {
                        let mut reader = BufReader::new(file);
                        let mut line = String::new();
                        loop {
                            line.clear();
                            match reader.read_line(&mut line).await {
                                Ok(0) => break,
                                Ok(n) => {
                                    if !line.ends_with('\n') {
                                        // Incomplete tail: leave it for
                                        // the next poll, do not advance.
                                        break;
                                    }
                                    pos += n as u64;
                                    if skip_partial_first {
                                        // The backfill seek landed
                                        // mid-line; this fragment is
                                        // not a whole record.
                                        skip_partial_first = false;
                                        continue;
                                    }
                                    let trimmed = line.trim_end();
                                    if !trimmed.is_empty() {
                                        seq += 1;
                                        push(&tailer, parse_line(seq, trimmed));
                                    }
                                }
                                Err(_) => break,
                            }
                        }
                    }
                }
            }
            Err(_) => {
                // File absent: the agent has never run or the sink
                // moved. Keep polling; absence is not an error here.
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(POLL_MS)).await;
    }
}
