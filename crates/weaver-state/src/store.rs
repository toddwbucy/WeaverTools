//! conforms: state-distillate-lands-whole
//! conforms: state-indexes-built-at-load
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
        for (index, (_kind, keys)) in election.keys.iter().enumerate() {
            for (key_index, key) in keys.iter().enumerate() {
                // The index name is positional and the key is a bound-in
                // literal within the WHERE, quoted through sqlite's own
                // quoting to keep a hostile key path from becoming SQL.
                let name = format!("field_elected_{index}_{key_index}");
                let statement = format!(
                    "CREATE INDEX IF NOT EXISTS {name} ON field (key, value) WHERE key = {}",
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
        transaction
            .execute(
                "INSERT INTO event (session, run, turn, kind, sequence)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![
                    distillate.session,
                    distillate.run,
                    distillate.turn,
                    distillate.kind,
                    distillate.sequence
                ],
            )
            .map_err(|e| CustodyFault::LandingFailed(e.to_string()))?;
        let event_id = transaction.last_insert_rowid();
        for (key, value) in &distillate.pairs {
            transaction
                .execute(
                    "INSERT INTO field (event_id, key, value) VALUES (?1, ?2, ?3)",
                    rusqlite::params![event_id, key, value],
                )
                .map_err(|e| CustodyFault::LandingFailed(e.to_string()))?;
        }
        transaction
            .commit()
            .map_err(|e| CustodyFault::LandingFailed(e.to_string()))
    }

    /// How many events stand, a custody fact the tests read. Not a serve
    /// surface: the serve direction's shape waits for its asker, and this
    /// answers no question about content.
    pub fn held(&self) -> Result<i64, CustodyFault> {
        self.connection
            .query_row("SELECT COUNT(*) FROM event", [], |row| row.get(0))
            .map_err(|e| CustodyFault::StoreUnavailable(e.to_string()))
    }
}

/// A string as a single-quoted SQL literal, sqlite's own doubling rule.
fn quoted(text: &str) -> String {
    format!("'{}'", text.replace('\'', "''"))
}

/// Parse one seam frame into a distillate. The envelope is demanded whole,
/// per the contract: an unattributable distillate is the sender's defect
/// and is dropped by the caller on `None`.
pub fn parse_distillate(frame: &str) -> Option<Distillate> {
    let value: serde_json::Value = serde_json::from_str(frame).ok()?;
    let envelope = value.get("envelope")?;
    let pairs = value
        .get("pairs")
        .and_then(|p| p.as_object())
        .map(|object| {
            object
                .iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect()
        })
        .unwrap_or_default();
    Some(Distillate {
        session: envelope.get("session")?.as_str()?.to_string(),
        run: envelope.get("run")?.as_str()?.to_string(),
        turn: envelope
            .get("turn")
            .and_then(|t| t.as_str())
            .map(str::to_string),
        kind: envelope.get("kind")?.as_str()?.to_string(),
        sequence: envelope.get("sequence")?.as_i64()?,
        pairs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("weaver-state-{}", std::process::id()));
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

    /// The parse demands the envelope whole: a frame missing any envelope
    /// member is nobody's row.
    #[test]
    fn an_unattributable_frame_is_refused() {
        assert!(parse_distillate(r#"{"envelope":{"session":"s","run":"r","kind":"load","sequence":0}}"#).is_some());
        for missing in [
            r#"{"envelope":{"run":"r","kind":"load","sequence":0}}"#,
            r#"{"envelope":{"session":"s","kind":"load","sequence":0}}"#,
            r#"{"envelope":{"session":"s","run":"r","sequence":0}}"#,
            r#"{"envelope":{"session":"s","run":"r","kind":"load"}}"#,
            r#"{"pairs":{}}"#,
            "not json",
        ] {
            assert!(parse_distillate(missing).is_none(), "{missing} must refuse");
        }
    }
}
