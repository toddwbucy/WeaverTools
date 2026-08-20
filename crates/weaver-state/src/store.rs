//! conforms: state-distillate-lands-whole
//! conforms: state-indexes-built-at-load
//! conforms: state-serve-restricts-to-the-session
//!
//! The custody, per `weaver-state-Spec` section 3: sqlite behind the seam,
//! never reached as a file, the distillate landing whole or not at all.

use std::path::Path;

use rusqlite::Connection;

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

/// The store: one sqlite file in the member's territory.
pub struct Store {
    connection: Connection,
}

impl Store {
    /// Open or create the store and stand the schema, per the Spec: the
    /// event and field tables, and the envelope's standing indexes. The
    /// election's own indexes arrive with [`Store::index_election`], read
    /// from the seam's opener.
    pub fn open(path: &Path) -> Result<Store, CustodyFault> {
        let connection =
            Connection::open(path).map_err(|e| CustodyFault::StoreUnavailable(e.to_string()))?;
        // Durability yields to speed, per the Spec's election: the
        // derivative is rebuildable from the record and the session never
        // depends on it.
        connection
            .execute_batch(
                "PRAGMA journal_mode = MEMORY;
                 PRAGMA synchronous = OFF;
                 CREATE TABLE IF NOT EXISTS event (
                     id       INTEGER PRIMARY KEY,
                     session  TEXT NOT NULL,
                     run      TEXT NOT NULL,
                     turn     TEXT,
                     kind     TEXT NOT NULL,
                     sequence INTEGER NOT NULL
                 );
                 CREATE TABLE IF NOT EXISTS field (
                     event_id INTEGER NOT NULL REFERENCES event(id),
                     key      TEXT NOT NULL,
                     value    TEXT NOT NULL
                 );
                 CREATE INDEX IF NOT EXISTS event_run_turn ON event (run, turn);
                 CREATE INDEX IF NOT EXISTS event_kind_sequence ON event (kind, sequence);",
            )
            .map_err(|e| CustodyFault::StoreUnavailable(e.to_string()))?;
        Ok(Store { connection })
    }

    /// Build the election's indexes, at load and never mid-serve, per the
    /// Spec: one partial index per elected key path, so extension within a
    /// session is rows accumulating under standing indexes.
    pub fn index_election(&mut self, election: &Election) -> Result<(), CustodyFault> {
        for (_kind, keys) in &election.keys {
            for key in keys {
                // The index name is the key path itself, hex-encoded, so a
                // name can only ever stand for one predicate: a positional
                // name would let a later load's differing election fall
                // silently under `IF NOT EXISTS` on an earlier load's name.
                // The key is a bound-in literal within the WHERE, quoted
                // through sqlite's own quoting to keep a hostile key path
                // from becoming SQL.
                use std::fmt::Write;
                let mut name = String::with_capacity(key.len() * 2);
                for byte in key.bytes() {
                    let _ = write!(name, "{byte:02x}");
                }
                let statement = format!(
                    "CREATE INDEX IF NOT EXISTS field_elected_{name} ON field (key, value) WHERE key = {}",
                    quoted(key)
                );
                self.connection
                    .execute(&statement, [])
                    .map_err(|e| CustodyFault::StoreUnavailable(e.to_string()))?;
            }
        }
        Ok(())
    }

    /// Land one distillate, whole or not at all, per the Spec: the event
    /// row and its field rows in one transaction that rolls back entire on
    /// any failure, because a distillate held in part would be an
    /// attributable envelope over missing pairs.
    pub fn land(&mut self, distillate: &Distillate) -> Result<(), CustodyFault> {
        let transaction = self
            .connection
            .transaction()
            .map_err(|e| CustodyFault::LandingFailed(e.to_string()))?;
        // The two inserts ride cached statements: one prepare per schema
        // for the store's life rather than one per event, with the
        // transaction boundary unchanged.
        {
            let mut insert_event = transaction
                .prepare_cached(
                    "INSERT INTO event (session, run, turn, kind, sequence)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                )
                .map_err(|e| CustodyFault::LandingFailed(e.to_string()))?;
            insert_event
                .execute(rusqlite::params![
                    distillate.session,
                    distillate.run,
                    distillate.turn,
                    distillate.kind,
                    distillate.sequence
                ])
                .map_err(|e| CustodyFault::LandingFailed(e.to_string()))?;
        }
        let event_id = transaction.last_insert_rowid();
        {
            let mut insert_field = transaction
                .prepare_cached("INSERT INTO field (event_id, key, value) VALUES (?1, ?2, ?3)")
                .map_err(|e| CustodyFault::LandingFailed(e.to_string()))?;
            for (key, value) in &distillate.pairs {
                insert_field
                    .execute(rusqlite::params![event_id, key, value])
                    .map_err(|e| CustodyFault::LandingFailed(e.to_string()))?;
            }
        }
        transaction
            .commit()
            .map_err(|e| CustodyFault::LandingFailed(e.to_string()))
    }

    /// How many events stand, a custody fact the tests read.
    pub fn held(&self) -> Result<i64, CustodyFault> {
        self.connection
            .query_row("SELECT COUNT(*) FROM event", [], |row| row.get(0))
            .map_err(|e| CustodyFault::StoreUnavailable(e.to_string()))
    }

    /// The shape ask's query, per `weaver-state-Spec` section 4: the runs
    /// in first-landed order, the `id` column being custody's own order
    /// key, each carrying its kinds and their counts as the envelope
    /// spelled them. An organized envelope fact carrying no judgment about
    /// what any count means to a turn, per the three-way division.
    pub fn shape(&self, session: &str) -> Result<Vec<RunShape>, CustodyFault> {
        let fault = |e: rusqlite::Error| CustodyFault::StoreUnavailable(e.to_string());
        let mut runs_query = self
            .connection
            .prepare_cached(
                "SELECT run FROM event WHERE session = ?1
                 GROUP BY run ORDER BY MIN(id)",
            )
            .map_err(fault)?;
        let runs: Vec<String> = runs_query
            .query_map([session], |row| row.get(0))
            .map_err(fault)?
            .collect::<Result<_, _>>()
            .map_err(fault)?;
        let mut kinds_query = self
            .connection
            .prepare_cached(
                "SELECT kind, COUNT(*) FROM event WHERE session = ?1 AND run = ?2
                 GROUP BY kind ORDER BY kind",
            )
            .map_err(fault)?;
        let mut shaped = Vec::with_capacity(runs.len());
        for run in runs {
            let kinds: Vec<(String, i64)> = kinds_query
                .query_map([session, run.as_str()], |row| Ok((row.get(0)?, row.get(1)?)))
                .map_err(fault)?
                .collect::<Result<_, _>>()
                .map_err(fault)?;
            shaped.push(RunShape { run, kinds });
        }
        Ok(shaped)
    }
}

/// One run's shape, the answer's material: the run reference and the held
/// event counts by kind.
#[derive(Debug, Clone, PartialEq)]
pub struct RunShape {
    pub run: String,
    pub kinds: Vec<(String, i64)>,
}

/// An ask as the seam's closed vocabulary spells it: two names, per the
/// contract's section 2, and a frame carrying any other ask name is
/// malformed and answers nothing.
#[derive(Debug, Clone, PartialEq)]
pub enum Ask {
    /// The session's shape: runs in first-seen order, counts by kind.
    Shape,
    /// The conversation as custody holds it, bounded to the most recent
    /// turns where a bound is given.
    Recall { last_turns: Option<u64> },
}

/// Parse a seam frame as an ask, or nothing where it is not one.
pub fn parse_ask(frame: &str) -> Option<Ask> {
    let value: serde_json::Value = serde_json::from_str(frame).ok()?;
    let ask = value.get("ask")?;
    if ask.get("shape").is_some() {
        return Some(Ask::Shape);
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

impl Store {
    /// The recall query, per `weaver-state-Spec` section 4: the event rows
    /// of the four message kinds with their field pairs, ordered by the
    /// `id` column, bounded where asked to the distinct session, run, and
    /// turn triples of the most recent turns by id, the rows outside them
    /// left unread. The bound keys the whole turn identity because a turn
    /// label recurs across runs, and a label alone would recall an older
    /// run's turn beside its namesake.
    pub fn recall(
        &self,
        session: &str,
        last_turns: Option<u64>,
    ) -> Result<Vec<RecalledEvent>, CustodyFault> {
        let fault = |e: rusqlite::Error| CustodyFault::StoreUnavailable(e.to_string());
        let bound: Option<Vec<(String, String, String)>> = match last_turns {
            None => None,
            Some(count) => {
                let mut turns_query = self
                    .connection
                    .prepare_cached(
                        "SELECT session, run, turn FROM (
                             SELECT session, run, turn, MAX(id) AS last FROM event
                             WHERE turn IS NOT NULL AND session = ?1
                             GROUP BY session, run, turn
                         ) ORDER BY last DESC LIMIT ?2",
                    )
                    .map_err(fault)?;
                let turns: Vec<(String, String, String)> = turns_query
                    .query_map(rusqlite::params![session, count as i64], |row| {
                        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
                    })
                    .map_err(fault)?
                    .collect::<Result<_, _>>()
                    .map_err(fault)?;
                Some(turns)
            }
        };
        let mut events_query = self
            .connection
            .prepare_cached(
                "SELECT id, session, run, turn, kind, sequence FROM event
                 WHERE session = ?1
                   AND kind IN ('message.system', 'message.user',
                                'message.assistant', 'message.tool_result')
                 ORDER BY id",
            )
            .map_err(fault)?;
        let rows: Vec<(i64, String, String, Option<String>, String, i64)> = events_query
            .query_map([session], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            })
            .map_err(fault)?
            .collect::<Result<_, _>>()
            .map_err(fault)?;
        let mut pairs_query = self
            .connection
            .prepare_cached("SELECT key, value FROM field WHERE event_id = ?1")
            .map_err(fault)?;
        let mut recalled = Vec::new();
        for (id, session, run, turn, kind, sequence) in rows {
            if let (Some(kept), Some(turn_ref)) = (&bound, &turn)
                && !kept
                    .iter()
                    .any(|(s, r, t)| s == &session && r == &run && t == turn_ref)
            {
                continue;
            }
            if bound.is_some() && turn.is_none() {
                continue;
            }
            let pairs: Vec<(String, String)> = pairs_query
                .query_map([id], |row| Ok((row.get(0)?, row.get(1)?)))
                .map_err(fault)?
                .collect::<Result<_, _>>()
                .map_err(fault)?;
            recalled.push(RecalledEvent {
                session,
                run,
                turn,
                kind,
                sequence,
                pairs,
            });
        }
        Ok(recalled)
    }
}

/// Render the recall answer as the contract's frame: each event in the
/// distillate's own shape, envelope and pairs, because custody serves what
/// it kept in the form it kept it.
pub fn render_recall_answer(events: &[RecalledEvent]) -> String {
    let rendered: Vec<serde_json::Value> = events
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
        .collect();
    let mut frame = serde_json::json!({"answer": {"recall": {"events": rendered}}}).to_string();
    frame.push('\n');
    frame
}

/// Render the shape answer as the contract's frame, one answer frame on
/// the channel, the runs in the order the query gave them.
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
    let mut frame =
        serde_json::json!({"answer": {"shape": {"runs": entries}}}).to_string();
    frame.push('\n');
    frame
}

/// A string as a single-quoted SQL literal, sqlite's own doubling rule.
fn quoted(text: &str) -> String {
    format!("'{}'", text.replace('\'', "''"))
}

/// Parse one seam frame into a distillate. The envelope is demanded whole,
/// per the contract: an unattributable distillate is the sender's defect
/// and is dropped by the caller on `None`.
pub fn parse_distillate(frame: &str) -> Option<Distillate> {
    use serde_json::value::RawValue;
    let top: std::collections::BTreeMap<&str, &RawValue> = serde_json::from_str(frame).ok()?;
    let envelope: serde_json::Value = serde_json::from_str(top.get("envelope")?.get()).ok()?;
    // The pair values land as the raw text that crossed, never re-rendered,
    // because the distillate is a projection of the canonical form and a
    // reshaping here would break that on the last step.
    let pairs = match top.get("pairs") {
        Some(raw) => serde_json::from_str::<std::collections::BTreeMap<String, &RawValue>>(
            raw.get(),
        )
        .ok()?
        .into_iter()
        .map(|(key, value)| (key, value.get().to_string()))
        .collect(),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "weaver-state-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        dir.join("state.sql")
    }

    /// The landing is atomic: a good distillate lands whole, and the store
    /// reopened from disk still holds it, which is the persistence the
    /// charter rules for runs within a session.
    #[test]
    fn a_distillate_lands_whole_and_survives_reopen() {
        let path = scratch();
        let _ = std::fs::remove_file(&path);
        let mut store = Store::open(&path).expect("opens");
        store
            .index_election(&Election::default())
            .expect("default election indexes");
        let distillate = Distillate {
            session: "alpha-1".into(),
            run: "2026-08-18T19:03:31.198Z-alpha-7d53a936e".into(),
            turn: Some("t-1".into()),
            kind: "turn.started".into(),
            sequence: 4,
            pairs: vec![("payload.close".into(), "\"clean\"".into())],
        };
        store.land(&distillate).expect("lands");
        assert_eq!(store.held().expect("held"), 1);
        drop(store);
        let store = Store::open(&path).expect("reopens");
        assert_eq!(
            store.held().expect("held"),
            1,
            "holdings survive the process, per the charter"
        );
        let _ = std::fs::remove_file(&path);
    }

    /// A later load's differing election builds its own index rather than
    /// falling silently under an earlier load's name, which is what a
    /// positional index name would allow under `IF NOT EXISTS`.
    #[test]
    fn a_changed_election_builds_its_own_indexes() {
        let path = scratch();
        let _ = std::fs::remove_file(&path);
        let mut store = Store::open(&path).expect("opens");
        let elect = |key: &str| Election {
            all_kinds: true,
            keys: vec![("turn.closed".into(), vec![key.into()])],
        };
        store.index_election(&elect("close")).expect("first");
        store.index_election(&elect("tokens")).expect("second");
        let elected: i64 = store
            .connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type = 'index' AND name LIKE 'field_elected_%'",
                [],
                |row| row.get(0),
            )
            .expect("counts");
        assert_eq!(elected, 2, "each key path owns its index");
        let _ = std::fs::remove_file(&path);
    }

    fn landed(session: &str, run: &str, kind: &str, sequence: i64) -> Distillate {
        Distillate {
            session: session.into(),
            run: run.into(),
            turn: None,
            kind: kind.into(),
            sequence,
            pairs: Vec::new(),
        }
    }

    /// **Custody answers within its session and not across it**, per
    /// `weaver-state-Spec` section 4 and `weaver-state-PRD` section 4's
    /// boundary. A store file holding more than one session is the normal
    /// case: sessions outlive runs and the file outlives sessions, so both
    /// serve queries bound to the session the opener declared.
    ///
    /// The defect this pins was invisible in exactly the way that matters.
    /// Unbounded, both queries answered over every session the file held
    /// and every answer looked well formed - a shape ask reporting a
    /// lifetime's runs as this session's, and a recall reaching a fact the
    /// operator believed a session cut had retired.
    ///
    /// Perturbation: drop either `WHERE session` and this fails, the older
    /// session's run and message appearing in the newer session's answers.
    #[test]
    fn the_answers_stay_inside_the_running_session() {
        let path = scratch();
        let _ = std::fs::remove_file(&path);
        let mut store = Store::open(&path).expect("opens");
        store
            .index_election(&Election::default())
            .expect("indexes");

        // An earlier session's holdings, still on disk where a session cut
        // left them, and a message it may not serve into the new session.
        store.land(&landed("old", "r-old", "load", 0)).expect("lands");
        let mut stale = landed("old", "r-old", "message.user", 1);
        stale.turn = Some("t-1".into());
        stale.pairs = vec![("payload.content".into(), "\"the vault code\"".into())];
        store.land(&stale).expect("lands");

        store.land(&landed("new", "r-new", "load", 0)).expect("lands");
        let mut fresh = landed("new", "r-new", "message.user", 1);
        fresh.turn = Some("t-1".into());
        fresh.pairs = vec![("payload.content".into(), "\"hello\"".into())];
        store.land(&fresh).expect("lands");

        assert_eq!(store.held().expect("held"), 4, "the file holds both");

        let shape = store.shape("new").expect("shapes");
        assert_eq!(
            shape.len(),
            1,
            "the shape holds the running session's runs alone: {shape:?}"
        );
        assert_eq!(shape[0].run, "r-new");

        let recalled = store.recall("new", None).expect("recalls");
        assert_eq!(
            recalled.len(),
            1,
            "the recall reads the running session's messages alone: {recalled:?}"
        );
        assert_eq!(recalled[0].run, "r-new");
        assert!(
            !recalled[0].pairs.iter().any(|(_, v)| v.contains("vault")),
            "and never the retired session's content"
        );

        // The older session is not destroyed, only unreachable: removal is
        // section 6's open question, deliberately not this act's.
        let old_shape = store.shape("old").expect("shapes");
        assert_eq!(old_shape.len(), 1, "the older session's rows stand");
    }

    /// The shape holds the runs in first-landed order by the id column,
    /// interleaved landings included, each with its counts by kind, and
    /// the answer frame renders the contract's spelling.
    #[test]
    fn the_shape_orders_runs_by_first_landing() {
        let path = scratch();
        let _ = std::fs::remove_file(&path);
        let mut store = Store::open(&path).expect("opens");
        for (run, kind, sequence) in [
            ("r-1", "load", 0),
            ("r-1", "turn.closed", 1),
            ("r-2", "load", 0),
            ("r-1", "turn.closed", 2),
            ("r-2", "turn.closed", 1),
        ] {
            store.land(&landed("s", run, kind, sequence)).expect("lands");
        }
        let shape = store.shape("s").expect("shapes");
        assert_eq!(shape.len(), 2);
        assert_eq!(shape[0].run, "r-1", "first landed leads");
        assert_eq!(
            shape[0].kinds,
            vec![("load".to_string(), 1), ("turn.closed".to_string(), 2)]
        );
        assert_eq!(shape[1].run, "r-2");
        let frame = render_shape_answer(&shape);
        assert!(frame.starts_with(r#"{"answer":{"shape":{"runs":["#), "{frame}");
        assert!(frame.ends_with("}\n"), "{frame}");
        let _ = std::fs::remove_file(&path);
    }

    /// The answered-against clause, in time: an ask sees every landing
    /// before it and nothing after, because the shape reads the holdings
    /// at its own position in the stream.
    #[test]
    fn an_ask_sees_the_holdings_at_its_position_and_no_more() {
        let path = scratch();
        let _ = std::fs::remove_file(&path);
        let mut store = Store::open(&path).expect("opens");
        store.land(&landed("s", "r-1", "load", 0)).expect("lands");
        let before = store.shape("s").expect("shapes");
        assert_eq!(before[0].kinds, vec![("load".to_string(), 1)]);
        store
            .land(&landed("s", "r-1", "turn.closed", 1))
            .expect("lands");
        let after = store.shape("s").expect("shapes");
        assert_eq!(
            after[0].kinds,
            vec![("load".to_string(), 1), ("turn.closed".to_string(), 1)]
        );
        assert_eq!(before[0].kinds.len(), 1, "the earlier answer never grew");
        let _ = std::fs::remove_file(&path);
    }

    /// The ask vocabulary is closed at two names: both are recognized,
    /// the recall's optional bound parses, and every other frame is not
    /// an ask at all.
    #[test]
    fn the_ask_vocabulary_is_closed() {
        assert_eq!(parse_ask(r#"{"ask":{"shape":{}}}"#), Some(Ask::Shape));
        assert_eq!(
            parse_ask(r#"{"ask":{"recall":{}}}"#),
            Some(Ask::Recall { last_turns: None })
        );
        assert_eq!(
            parse_ask(r#"{"ask":{"recall":{"last-turns":3}}}"#),
            Some(Ask::Recall {
                last_turns: Some(3)
            })
        );
        for not_an_ask in [
            r#"{"ask":{"summarize":{}}}"#,
            r#"{"ask":{"recall":{"last-turns":-3}}}"#,
            r#"{"ask":{"recall":{"last-turns":"three"}}}"#,
            r#"{"ask":{"recall":{"last-turns":2.5}}}"#,
            r#"{"envelope":{}}"#,
            "not json",
        ] {
            assert!(parse_ask(not_an_ask).is_none(), "{not_an_ask}");
        }
    }

    /// The bounded recall keys its turns by session, run, and turn
    /// together: a turn label recurring across runs names two different
    /// turns, and the bound must not recall the older run's events beside
    /// its namesake's.
    #[test]
    fn a_bounded_recall_keeps_colliding_turn_labels_apart() {
        let path = scratch();
        let _ = std::fs::remove_file(&path);
        let mut store = Store::open(&path).expect("opens");
        let message = |run: &str, turn: &str, text: &str, sequence: i64| Distillate {
            session: "s".into(),
            run: run.into(),
            turn: Some(turn.into()),
            kind: "message.user".into(),
            sequence,
            pairs: vec![("content".into(), format!("\"{text}\""))],
        };
        for landing in [
            message("r-1", "t-1", "old one", 1),
            message("r-1", "t-2", "old two", 2),
            message("r-2", "t-1", "new one", 1),
            message("r-2", "t-2", "new two", 2),
        ] {
            store.land(&landing).expect("lands");
        }
        let bounded = store.recall("s", Some(2)).expect("recalls");
        let quoted: Vec<&str> = bounded
            .iter()
            .map(|event| event.pairs[0].1.as_str())
            .collect();
        assert_eq!(
            quoted,
            vec!["\"new one\"", "\"new two\""],
            "the bound keeps the newer run's turns and no namesakes"
        );
        let whole = store.recall("s", None).expect("recalls");
        assert_eq!(whole.len(), 4, "the unbounded recall reads every message");
        let _ = std::fs::remove_file(&path);
    }

    /// The parse demands the envelope whole: a frame missing any envelope
    /// member is nobody's row.
    #[test]
    fn an_unattributable_frame_is_refused() {
        assert!(parse_distillate(r#"{"envelope":{"session":"s","run":"r","kind":"load","sequence":"0"}}"#).is_some());
        for missing in [
            r#"{"envelope":{"run":"r","kind":"load","sequence":"0"}}"#,
            r#"{"envelope":{"session":"s","kind":"load","sequence":"0"}}"#,
            r#"{"envelope":{"session":"s","run":"r","sequence":"0"}}"#,
            r#"{"envelope":{"session":"s","run":"r","kind":"load"}}"#,
            r#"{"pairs":{}}"#,
            "not json",
        ] {
            assert!(parse_distillate(missing).is_none(), "{missing} must refuse");
        }
    }
}
