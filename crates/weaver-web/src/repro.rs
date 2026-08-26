//! The confirm job (Spec section 17): pull a run from the record,
//! drive its turns back through the gate on a fresh load, compare
//! field by field, and report. One job at a time, in memory, nothing
//! touching the channel store - a confirm is the operator asking the
//! record a question, not conversation.

use crate::wire::Link;
use serde_json::Value;
use std::sync::{Arc, Mutex};

/// How long the job waits for the reloaded agent's socket to stand.
const LOAD_WAIT_SECS: u64 = 120;

/// The compared fields, the lab report's list mechanized. Each check
/// is a named equality over the two runs' payloads.
const CHECKS: [&str; 7] = [
    "rendered prompt",
    "generation seed",
    "effective knobs",
    "emission bytes",
    "finish and resident",
    "input token ids",
    "entropies float-exact",
];

#[derive(Debug, Clone)]
pub struct TurnReport {
    pub turn: String,
    pub checks: Vec<(String, bool)>,
    pub reproduced: bool,
    pub source_ms: Option<i64>,
    pub replay_ms: Option<i64>,
    pub tokens_in: usize,
    pub tokens_out: usize,
    pub preview: String,
}

#[derive(Debug, Clone)]
pub struct Report {
    pub agent: String,
    pub source_run: String,
    pub replay_run: String,
    pub turns: Vec<TurnReport>,
    pub reproduced: bool,
}

#[derive(Default)]
struct State {
    running: bool,
    agent: String,
    log: Vec<String>,
    report: Option<Report>,
}

/// The page's read: is a job running, what has it said, what did the
/// last one conclude.
#[derive(Clone, Default)]
pub struct Snapshot {
    pub running: bool,
    pub agent: String,
    pub log: Vec<String>,
    pub report: Option<Report>,
}

#[derive(Clone, Default)]
pub struct Repro {
    inner: Arc<Mutex<State>>,
}

impl Repro {
    pub fn snapshot(&self) -> Snapshot {
        let s = self.inner.lock().unwrap();
        Snapshot {
            running: s.running,
            agent: s.agent.clone(),
            log: s.log.clone(),
            report: s.report.clone(),
        }
    }

    /// Start a confirm if none is running. The refusal is the page's
    /// to render.
    pub fn start(&self, link: Link, agent: String, run: String) -> Result<(), String> {
        {
            let mut s = self.inner.lock().unwrap();
            if s.running {
                return Err(format!(
                    "a confirm of '{}' is already running - one at a time",
                    s.agent
                ));
            }
            s.running = true;
            s.agent = agent.clone();
            s.log = vec![format!("confirm of run {run} on agent {agent}")];
            s.report = None;
        }
        let this = self.clone();
        tokio::spawn(async move {
            let outcome = job(&this, &link, &agent, &run).await;
            let mut s = this.inner.lock().unwrap();
            if let Err(e) = outcome {
                s.log.push(format!("ended without a verdict: {e}"));
            }
            s.running = false;
        });
        Ok(())
    }

    fn log(&self, line: impl Into<String>) {
        self.inner.lock().unwrap().log.push(line.into());
    }
}

/// One recorded turn, cut from the source run's events.
struct SourceTurn {
    turn: String,
    text: String,
    request: Value,
    output: Value,
    measurement: Value,
    whole_ms: Option<i64>,
}

/// Group a run's events into turns, in first-appearance order. The
/// request text is the turn's last `message.user` event, identity
/// messages preceding the request in render order (Spec section 17).
fn cut_turns(events: &[Value]) -> Vec<SourceTurn> {
    let mut order: Vec<String> = Vec::new();
    let mut by_turn: std::collections::HashMap<String, Vec<&Value>> =
        std::collections::HashMap::new();
    for e in events {
        let Some(turn) = e.get("turn").and_then(|t| t.as_str()) else { continue };
        by_turn
            .entry(turn.to_owned())
            .or_insert_with(|| {
                order.push(turn.to_owned());
                Vec::new()
            })
            .push(e);
    }
    let mut turns = Vec::new();
    for t in order {
        let evs = &by_turn[&t];
        let find = |kind: &str| {
            evs.iter()
                .find(|e| e.get("kind").and_then(|k| k.as_str()) == Some(kind))
                .and_then(|e| e.get("payload"))
                .cloned()
        };
        let text = evs
            .iter()
            .filter(|e| e.get("kind").and_then(|k| k.as_str()) == Some("message.user"))
            .filter_map(|e| {
                e.pointer("/payload/content/0/text").and_then(|t| t.as_str())
            })
            .next_back()
            .map(str::to_owned);
        let wall = |kind: &str| {
            evs.iter()
                .find(|e| e.get("kind").and_then(|k| k.as_str()) == Some(kind))
                .and_then(|e| e.get("wall_ms"))
                .and_then(|w| w.as_i64())
        };
        let (Some(text), Some(request), Some(output), Some(measurement)) =
            (text, find("model.request"), find("model.output"), find("model.measurement"))
        else {
            continue; // an unfinished or refused turn has nothing to confirm
        };
        turns.push(SourceTurn {
            turn: t,
            text,
            request,
            output,
            measurement,
            whole_ms: match (wall("turn.started"), wall("turn.closed")) {
                (Some(a), Some(b)) => Some(b - a),
                _ => None,
            },
        });
    }
    turns
}

fn compare(source: &SourceTurn, replay: &SourceTurn) -> Vec<(String, bool)> {
    let eq = |a: &Value, b: &Value, ptr: &str| a.pointer(ptr) == b.pointer(ptr);
    let r = (&source.request, &replay.request);
    let o = (&source.output, &replay.output);
    let m = (&source.measurement, &replay.measurement);
    let results = [
        eq(r.0, r.1, "/rendered"),
        eq(r.0, r.1, "/sampling/generation_seed"),
        eq(r.0, r.1, "/sampling"),
        eq(o.0, o.1, "/emission"),
        eq(o.0, o.1, "/finish") && eq(o.0, o.1, "/resident"),
        eq(m.0, m.1, "/input_tokens"),
        eq(m.0, m.1, "/entropies"),
    ];
    CHECKS
        .iter()
        .zip(results)
        .map(|(name, ok)| ((*name).to_owned(), ok))
        .collect()
}

async fn job(repro: &Repro, link: &Link, agent: &str, run: &str) -> Result<(), String> {
    // 1. The source run, from the record.
    let (events, truncated) = link
        .trace_run(agent, run)
        .await
        .ok_or("the record read failed - link down or unresponsive")?;
    if truncated {
        return Err("the run exceeds the read cap - not confirmable whole".into());
    }
    let source_turns = cut_turns(&events);
    if source_turns.is_empty() {
        return Err("the run holds no completed turns".into());
    }
    repro.log(format!(
        "source run read: {} events, {} completed turn(s)",
        events.len(),
        source_turns.len()
    ));

    // 2. Fresh load, by the verbs, each answer rendered as it came.
    for verb in ["unload", "load"] {
        let outcome = link
            .verb(agent, verb)
            .await
            .map_err(|e| format!("{verb} not run: {e}"))?;
        let answer = outcome
            .answer
            .as_ref()
            .map(|a| a.to_string())
            .or(outcome.raw_stdout.clone())
            .unwrap_or_default();
        repro.log(format!("{verb}: {answer}"));
        if verb == "load" && !answer.contains("\"state\"") {
            return Err(format!("the load did not answer a state: {answer}"));
        }
    }
    let mut waited = 0;
    loop {
        if let Some(status) = link.status().await {
            if status.get(agent).copied().unwrap_or(false) {
                break;
            }
        }
        waited += 1;
        if waited > LOAD_WAIT_SECS {
            return Err(format!("gate socket absent {LOAD_WAIT_SECS}s after load"));
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    repro.log("agent reloaded, gate standing".to_owned());

    // 3. Reissue each turn byte-exact, in order. Every close must be
    // answered and every close must name the same fresh run - a
    // refusal, a missing label, or a split across runs is the job
    // ending with that fact, never a comparison against the wrong
    // record.
    let mut runs_seen: Vec<String> = Vec::new();
    for st in &source_turns {
        let close = link
            .turn(agent, &st.text)
            .await
            .map_err(|e| format!("reissue of {} refused: {e}", st.turn))?;
        repro.log(format!(
            "reissued {} -> close {} on {}",
            st.turn,
            close.kind,
            close.run.as_deref().unwrap_or("?")
        ));
        if close.kind != "answered" {
            return Err(format!("reissue of {} closed {}", st.turn, close.kind));
        }
        match close.run {
            Some(r) => {
                if !runs_seen.contains(&r) {
                    runs_seen.push(r);
                }
            }
            None => return Err(format!("reissue of {} named no run", st.turn)),
        }
    }
    let replay_run = match runs_seen.as_slice() {
        [one] if one != run => one.clone(),
        [one] => {
            return Err(format!("reissues landed in the source run {one} itself"))
        }
        many => {
            return Err(format!("reissues split across runs: {}", many.join(", ")))
        }
    };

    // 4. The replay run, from the record, and the comparison.
    let (replay_events, _) = link
        .trace_run(agent, &replay_run)
        .await
        .ok_or("the replay record read failed")?;
    let replay_turns = cut_turns(&replay_events);
    let mut turns = Vec::new();
    let mut all = true;
    // A replay run with turns the source never had is an interleave
    // (a channel turn landing mid-confirm) and fails the verdict even
    // when every source turn happens to match.
    if replay_turns.len() != source_turns.len() {
        repro.log(format!(
            "turn count differs: source {} replay {} - interleaved traffic",
            source_turns.len(),
            replay_turns.len()
        ));
        all = false;
    }
    for st in &source_turns {
        let rt = replay_turns.iter().find(|r| r.turn == st.turn);
        let (checks, replay_ms) = match rt {
            Some(rt) => (compare(st, rt), rt.whole_ms),
            // A missing turn diverged by ordinal - a mid-confirm
            // channel turn or a refusal - and fails every check.
            None => (
                CHECKS.iter().map(|n| ((*n).to_owned(), false)).collect(),
                None,
            ),
        };
        let reproduced = checks.iter().all(|(_, ok)| *ok);
        all &= reproduced;
        turns.push(TurnReport {
            turn: st.turn.clone(),
            reproduced,
            checks,
            source_ms: st.whole_ms,
            replay_ms,
            tokens_in: st
                .measurement
                .pointer("/input_tokens")
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .unwrap_or(0),
            tokens_out: st
                .measurement
                .pointer("/entropies")
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .unwrap_or(0),
            preview: st
                .output
                .pointer("/emission")
                .and_then(|e| e.as_str())
                .unwrap_or("")
                .chars()
                .take(80)
                .collect(),
        });
    }
    repro.log(format!(
        "verdict: {}",
        if all { "REPRODUCED" } else { "NOT REPRODUCED" }
    ));
    repro.inner.lock().unwrap().report = Some(Report {
        agent: agent.to_owned(),
        source_run: run.to_owned(),
        replay_run,
        turns,
        reproduced: all,
    });
    Ok(())
}
