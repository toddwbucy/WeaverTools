//! conforms: state-distillate-lands-whole
//! conforms: state-indexes-built-at-load
//! conforms: state-serve-restricts-to-the-session
//!
//! The custody, per `weaver-state-Spec` section 3: sqlite behind the seam,
//! never reached as a file, the distillate landing whole or not at all.

/// The election as the seam's opener carries it: the elected kinds, each
/// with its payload key paths, empty meaning the envelope alone. The
/// default election is the envelope of every kind and nothing more, per
/// `weaver-trace-PRD` section 11, which this shape spells as an empty map
/// with `all_kinds` standing.
#[derive(Debug, Clone, PartialEq)]
pub struct Election {
    /// Every kind crosses with its envelope. The default, always true
    /// today: a kind-restricted election arrives with the operator's
    /// payload-key elections, and nothing here guesses at its shape.
    pub all_kinds: bool,
    /// Payload key paths per kind, on top of the envelope.
    pub keys: Vec<(String, Vec<String>)>,
}

impl Default for Election {
    fn default() -> Self {
        Election {
            all_kinds: true,
            keys: Vec::new(),
        }
    }
}

/// One distilled event, parsed from the seam's frame: the envelope whole,
/// the elected pairs beside it.
#[derive(Debug, Clone, PartialEq)]
pub struct Distillate {
    pub session: String,
    pub run: String,
    pub turn: Option<String>,
    pub kind: String,
    pub sequence: i64,
    pub pairs: Vec<(String, String)>,
}

/// What custody refuses. The set is small because the charter is: a
/// custodian that answered richly would be growing a voice the serve
/// direction has not given it.
#[derive(Debug)]
pub enum CustodyFault {
    /// The store could not open or the schema could not stand.
    StoreUnavailable(String),
    /// A distillate failed to land. The transaction rolled back whole.
    LandingFailed(String),
}

/// **The store is a port**, per `weaver-state-Spec` section 3 and the ruling of
/// 2026-09-04: every engine implements the whole of it, the ingest and the
/// serve speak to this and never to an engine, and this is the one place a
/// query language is spelled, each engine in its own dialect.
pub trait Store {
    /// Build the elected keys' indexes, read from the seam's opener.
    fn index_election(&mut self, election: &Election) -> Result<(), CustodyFault>;
    /// Land one distillate whole or not at all.
    fn land(&mut self, distillate: &Distillate) -> Result<(), CustodyFault>;
    /// Retire the session's prior holdings and stand the election's indexes,
    /// in one transaction, per the preload door's contract.
    fn retire_and_index(&mut self, session: &str, election: &Election) -> Result<(), CustodyFault>;
    /// Every event of the session with its pairs, in landing order.
    fn replay(&self, session: &str) -> Result<Vec<RecalledEvent>, CustodyFault>;
    /// How many events the store holds, every session counted.
    fn held(&self) -> Result<i64, CustodyFault>;
    /// The session's shape: its runs in order, each with its kinds counted.
    fn shape(&self, session: &str) -> Result<Vec<RunShape>, CustodyFault>;
    /// The session's message events, bounded to the last `last_turns` turns
    /// where a bound is given.
    fn recall(
        &self,
        session: &str,
        last_turns: Option<u64>,
    ) -> Result<Vec<RecalledEvent>, CustodyFault>;
    /// The boundary as the engine states it, per the contract's `grants`
    /// ask of 2026-09-04: an ordered list of lines the engine renders from
    /// its own catalog, spelled so two readings compare and no more.
    fn grants(&self) -> Result<Vec<String>, CustodyFault>;
}

/// One run's shape, the answer's material: the run reference and the held
/// event counts by kind.
#[derive(Debug, Clone, PartialEq)]
pub struct RunShape {
    pub run: String,
    pub kinds: Vec<(String, i64)>,
}

/// An ask as the seam's closed vocabulary spells it: three names, per the
/// contract's section 2 as amended 2026-08-24, and a frame carrying any
/// other ask name is malformed and answers nothing.
#[derive(Debug, Clone, PartialEq)]
pub enum Ask {
    /// The session's shape: runs in first-seen order, counts by kind.
    Shape,
    /// The conversation as custody holds it, bounded to the most recent
    /// turns where a bound is given.
    Recall { last_turns: Option<u64> },
    /// Every held event of the declared session, whole, in landing order.
    /// Carries no members: what a replay reads is the session, and the
    /// four message kinds `recall` serves are less than it needs.
    Replay,
    /// The boundary as the store states it, read at the enter and the
    /// leave, per the contract as of 2026-09-04. Carries no members.
    Grants,
}

/// Parse a seam frame as an ask, or nothing where it is not one.
pub fn parse_ask(frame: &str) -> Option<Ask> {
    let value: serde_json::Value = serde_json::from_str(frame).ok()?;
    let ask = value.get("ask")?;
    if ask.get("shape").is_some() {
        return Some(Ask::Shape);
    }
    if ask.get("replay").is_some() {
        return Some(Ask::Replay);
    }
    if ask.get("grants").is_some() {
        return Some(Ask::Grants);
    }
    let recall = ask.get("recall")?;
    let last_turns = match recall.get("last-turns") {
        None => None,
        Some(bound) => Some(bound.as_u64()?),
    };
    Some(Ask::Recall { last_turns })
}

/// Whether a seam frame is the shape ask, kept for the standing tests: the
/// dispatch reads [`parse_ask`].
pub fn is_shape_ask(frame: &str) -> bool {
    matches!(parse_ask(frame), Some(Ask::Shape))
}

/// One recalled event, the recall answer's material: the envelope's facts
/// and the elected pairs as custody kept them.
#[derive(Debug, Clone, PartialEq)]
pub struct RecalledEvent {
    pub session: String,
    pub run: String,
    pub turn: Option<String>,
    pub kind: String,
    pub sequence: i64,
    pub pairs: Vec<(String, String)>,
}

/// Render the recall answer as the contract's frame: each event in the
/// distillate's own shape, envelope and pairs, because custody serves what
/// it kept in the form it kept it.
pub fn render_recall_answer(events: &[RecalledEvent]) -> String {
    let rendered = rendered_events(events);
    let mut frame = serde_json::json!({"answer": {"recall": {"events": rendered}}}).to_string();
    frame.push('\n');
    frame
}

/// One event's rendering, envelope and pairs, shared by the recall and the
/// replay answers because both serve an event as the distillate's own
/// shape and a second rendering would be a second spelling of one form.
fn rendered_events(events: &[RecalledEvent]) -> Vec<serde_json::Value> {
    events
        .iter()
        .map(|event| {
            let mut envelope = serde_json::Map::new();
            envelope.insert("session".into(), event.session.clone().into());
            envelope.insert("run".into(), event.run.clone().into());
            if let Some(turn) = &event.turn {
                envelope.insert("turn".into(), turn.clone().into());
            }
            envelope.insert("kind".into(), event.kind.clone().into());
            envelope.insert("sequence".into(), event.sequence.to_string().into());
            let pairs: serde_json::Map<String, serde_json::Value> = event
                .pairs
                .iter()
                .map(|(key, value)| {
                    let parsed = serde_json::from_str(value)
                        .unwrap_or_else(|_| serde_json::Value::String(value.clone()));
                    (key.clone(), parsed)
                })
                .collect();
            serde_json::json!({"envelope": envelope, "pairs": pairs})
        })
        .collect()
}

/// Render the replay answer as the contract's frame: every event whole, in
/// landing order, each as the distillate's own shape. The answer names the
/// ask it answers, which is what pairs it without a correlation member, per
/// `weaver-harness-state-contract` section 2.
pub fn render_replay_answer(events: &[RecalledEvent]) -> String {
    let rendered = rendered_events(events);
    let mut frame = serde_json::json!({"answer": {"replay": {"events": rendered}}}).to_string();
    frame.push('\n');
    frame
}

/// Render the shape answer as the contract's frame, one answer frame on
/// the channel, the runs in the order the query gave them.
/// The grants answer: the surface's lines in the engine's order, per the
/// contract's answer shape `{"answer":{"grants":{"surface":[...]}}}`.
pub fn render_grants_answer(surface: &[String]) -> String {
    let mut frame = serde_json::json!({"answer": {"grants": {"surface": surface}}}).to_string();
    frame.push('\n');
    frame
}

pub fn render_shape_answer(runs: &[RunShape]) -> String {
    let entries: Vec<serde_json::Value> = runs
        .iter()
        .map(|shape| {
            let kinds: serde_json::Map<String, serde_json::Value> = shape
                .kinds
                .iter()
                .map(|(kind, count)| (kind.clone(), serde_json::Value::from(*count)))
                .collect();
            serde_json::json!({"run": shape.run, "kinds": kinds})
        })
        .collect();
    let mut frame = serde_json::json!({"answer": {"shape": {"runs": entries}}}).to_string();
    frame.push('\n');
    frame
}

/// Build the election's partial indexes on whatever holds the connection,
/// the store itself or an open transaction, so the preload path can run the
/// build inside the retirement's transaction while the first door's opener
/// runs it bare. The index name is the key path itself, hex-encoded, so a
/// name can only ever stand for one predicate: a positional name would let
/// a later load's differing election fall silently under `IF NOT EXISTS` on
/// an earlier load's name. The key is a bound-in literal within the WHERE,
/// quoted through sqlite's own quoting to keep a hostile key path from
/// becoming SQL.
pub fn parse_distillate(frame: &str) -> Option<Distillate> {
    use serde_json::value::RawValue;
    let top: std::collections::BTreeMap<&str, &RawValue> = serde_json::from_str(frame).ok()?;
    let envelope: serde_json::Value = serde_json::from_str(top.get("envelope")?.get()).ok()?;
    // The pair values land as the raw text that crossed, never re-rendered,
    // because the distillate is a projection of the canonical form and a
    // reshaping here would break that on the last step.
    let pairs = match top.get("pairs") {
        Some(raw) => {
            serde_json::from_str::<std::collections::BTreeMap<String, &RawValue>>(raw.get())
                .ok()?
                .into_iter()
                .map(|(key, value)| (key, value.get().to_string()))
                .collect()
        }
        None => Vec::new(),
    };
    Some(Distillate {
        session: envelope.get("session")?.as_str()?.to_string(),
        run: envelope.get("run")?.as_str()?.to_string(),
        turn: envelope
            .get("turn")
            .and_then(|t| t.as_str())
            .map(str::to_string),
        kind: envelope.get("kind")?.as_str()?.to_string(),
        // The canonical form spells the sequence as a string and the
        // distillate carries that spelling, so the conversion to the row's
        // integer happens here, at the landing, and a spelling that does
        // not convert refuses the frame whole.
        sequence: envelope.get("sequence")?.as_str()?.parse().ok()?,
        pairs,
    })
}
