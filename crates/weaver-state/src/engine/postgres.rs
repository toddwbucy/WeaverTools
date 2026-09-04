//! The service engine, per `weaver-state-Spec` section 3 and the ruling of
//! 2026-09-04: one database per agent, reached over the store's unix socket
//! under the member's own account, the store's peer authentication mapping
//! that account to the role the binding declares. Behind the `postgres`
//! feature. The same two-table shape as the embedded engine, in this engine's
//! dialect, and the same port, whole.

use std::cell::{RefCell, RefMut};

use postgres::{Client, GenericClient, NoTls};

use crate::store::{CustodyFault, Distillate, Election, RecalledEvent, RunShape, Store};

/// The service engine. The port's asks take `&self` and the wire is a
/// stream that needs `&mut`, so the client sits behind a cell: one member
/// holds one connection and serves one ask at a time, so the cell is never
/// contended, and a contended borrow would be a defect worth the panic.
pub struct Postgres {
    client: RefCell<Client>,
}

impl Postgres {
    /// Connect over the store's socket directory as the member's account,
    /// under the declared role and database, and stand the schema: the event
    /// and field tables and the envelope's standing indexes. The election's
    /// own indexes arrive with [`Store::index_election`].
    pub fn open(socket_dir: &str, database: &str, role: &str) -> Result<Postgres, CustodyFault> {
        let mut client = postgres::Config::new()
            .host_path(socket_dir)
            .user(role)
            .dbname(database)
            .connect(NoTls)
            .map_err(unavailable)?;
        client
            .batch_execute(
                "CREATE TABLE IF NOT EXISTS event (
                     id       BIGSERIAL PRIMARY KEY,
                     session  TEXT NOT NULL,
                     run      TEXT NOT NULL,
                     turn     TEXT,
                     kind     TEXT NOT NULL,
                     sequence BIGINT NOT NULL
                 );
                 CREATE TABLE IF NOT EXISTS field (
                     event_id BIGINT NOT NULL REFERENCES event(id),
                     key      TEXT NOT NULL,
                     value    TEXT NOT NULL
                 );
                 CREATE INDEX IF NOT EXISTS event_run_turn ON event (run, turn);
                 CREATE INDEX IF NOT EXISTS event_kind_sequence ON event (kind, sequence);",
            )
            .map_err(unavailable)?;
        Ok(Postgres {
            client: RefCell::new(client),
        })
    }

    fn client(&self) -> RefMut<'_, Client> {
        self.client.borrow_mut()
    }
}

fn unavailable(e: postgres::Error) -> CustodyFault {
    CustodyFault::StoreUnavailable(e.to_string())
}

fn landing(e: postgres::Error) -> CustodyFault {
    CustodyFault::LandingFailed(e.to_string())
}

/// The elected keys' partial indexes, named by the key's hex so any elected
/// key names a legal identifier, as the embedded engine names them.
fn build_indexes(
    executor: &mut impl GenericClient,
    election: &Election,
) -> Result<(), CustodyFault> {
    use std::fmt::Write;
    for (_kind, keys) in &election.keys {
        for key in keys {
            let mut name = String::with_capacity(key.len() * 2);
            for byte in key.bytes() {
                let _ = write!(name, "{byte:02x}");
            }
            let statement = format!(
                "CREATE INDEX IF NOT EXISTS field_elected_{name} ON field (key, value) WHERE key = {}",
                quoted(key)
            );
            executor.batch_execute(&statement).map_err(unavailable)?;
        }
    }
    Ok(())
}

fn quoted(text: &str) -> String {
    format!("'{}'", text.replace('\'', "''"))
}

/// The events of one query with their pairs, in the query's order.
fn with_pairs(
    client: &mut Client,
    rows: Vec<postgres::Row>,
) -> Result<Vec<RecalledEvent>, CustodyFault> {
    // One read for every event's pairs rather than one per event: the
    // answer is a session's worth of rows and a round trip per event would
    // charge the ask for its own length. Grouped by event on this side, the
    // pairs of one event kept in the order the store returns them, which
    // the answer renders as a map and so does not depend on.
    let ids: Vec<i64> = rows.iter().map(|row| row.get(0)).collect();
    let mut pairs_by_event: std::collections::HashMap<i64, Vec<(String, String)>> =
        std::collections::HashMap::with_capacity(ids.len());
    for pair in client
        .query(
            "SELECT event_id, key, value FROM field WHERE event_id = ANY($1) ORDER BY event_id",
            &[&ids],
        )
        .map_err(unavailable)?
    {
        let event_id: i64 = pair.get(0);
        pairs_by_event
            .entry(event_id)
            .or_default()
            .push((pair.get(1), pair.get(2)));
    }
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let id: i64 = row.get(0);
        out.push(RecalledEvent {
            session: row.get(1),
            run: row.get(2),
            turn: row.get(3),
            kind: row.get(4),
            sequence: row.get(5),
            pairs: pairs_by_event.remove(&id).unwrap_or_default(),
        });
    }
    Ok(out)
}

const MESSAGE_KINDS: &str =
    "('message.system', 'message.user', 'message.assistant', 'message.tool_result')";

impl Store for Postgres {
    fn index_election(&mut self, election: &Election) -> Result<(), CustodyFault> {
        build_indexes(self.client.get_mut(), election)
    }

    fn land(&mut self, distillate: &Distillate) -> Result<(), CustodyFault> {
        let client = self.client.get_mut();
        let mut transaction = client.transaction().map_err(landing)?;
        let row = transaction
            .query_one(
                "INSERT INTO event (session, run, turn, kind, sequence)
                 VALUES ($1, $2, $3, $4, $5) RETURNING id",
                &[
                    &distillate.session,
                    &distillate.run,
                    &distillate.turn,
                    &distillate.kind,
                    &distillate.sequence,
                ],
            )
            .map_err(landing)?;
        let event_id: i64 = row.get(0);
        for (key, value) in &distillate.pairs {
            transaction
                .execute(
                    "INSERT INTO field (event_id, key, value) VALUES ($1, $2, $3)",
                    &[&event_id, key, value],
                )
                .map_err(landing)?;
        }
        transaction.commit().map_err(landing)
    }

    fn retire_and_index(&mut self, session: &str, election: &Election) -> Result<(), CustodyFault> {
        let client = self.client.get_mut();
        let mut transaction = client.transaction().map_err(landing)?;
        transaction
            .execute(
                "DELETE FROM field WHERE event_id IN (SELECT id FROM event WHERE session = $1)",
                &[&session],
            )
            .map_err(landing)?;
        transaction
            .execute("DELETE FROM event WHERE session = $1", &[&session])
            .map_err(landing)?;
        build_indexes(&mut transaction, election)?;
        transaction.commit().map_err(landing)
    }

    fn replay(&self, session: &str) -> Result<Vec<RecalledEvent>, CustodyFault> {
        let mut client = self.client();
        let rows = client
            .query(
                "SELECT id, session, run, turn, kind, sequence FROM event
                 WHERE session = $1 ORDER BY id",
                &[&session],
            )
            .map_err(unavailable)?;
        with_pairs(&mut client, rows)
    }

    fn held(&self) -> Result<i64, CustodyFault> {
        let row = self
            .client()
            .query_one("SELECT COUNT(*) FROM event", &[])
            .map_err(unavailable)?;
        Ok(row.get(0))
    }

    fn shape(&self, session: &str) -> Result<Vec<RunShape>, CustodyFault> {
        let mut client = self.client();
        let runs: Vec<String> = client
            .query(
                "SELECT run FROM event WHERE session = $1 GROUP BY run ORDER BY MIN(id)",
                &[&session],
            )
            .map_err(unavailable)?
            .into_iter()
            .map(|r| r.get(0))
            .collect();
        let mut shaped = Vec::with_capacity(runs.len());
        for run in runs {
            let kinds: Vec<(String, i64)> = client
                .query(
                    "SELECT kind, COUNT(*) FROM event WHERE session = $1 AND run = $2
                     GROUP BY kind ORDER BY kind",
                    &[&session, &run],
                )
                .map_err(unavailable)?
                .into_iter()
                .map(|r| (r.get(0), r.get(1)))
                .collect();
            shaped.push(RunShape { run, kinds });
        }
        Ok(shaped)
    }

    fn recall(
        &self,
        session: &str,
        last_turns: Option<u64>,
    ) -> Result<Vec<RecalledEvent>, CustodyFault> {
        let mut client = self.client();
        let rows = match last_turns {
            None => client
                .query(
                    &format!(
                        "SELECT id, session, run, turn, kind, sequence FROM event
                         WHERE session = $1 AND kind IN {MESSAGE_KINDS} ORDER BY id"
                    ),
                    &[&session],
                )
                .map_err(unavailable)?,
            Some(count) => client
                .query(
                    &format!(
                        "SELECT e.id, e.session, e.run, e.turn, e.kind, e.sequence FROM event e
                         JOIN (
                             SELECT run, turn, MAX(id) AS last FROM event
                             WHERE session = $1 AND turn IS NOT NULL
                             GROUP BY run, turn ORDER BY last DESC LIMIT $2
                         ) t ON e.run = t.run AND e.turn = t.turn
                         WHERE e.session = $1 AND e.kind IN {MESSAGE_KINDS} ORDER BY e.id"
                    ),
                    &[&session, &(count as i64)],
                )
                .map_err(unavailable)?,
        };
        with_pairs(&mut client, rows)
    }

    /// Under the service engine the boundary is the catalog's: the role's
    /// attributes, its memberships, the database's access list, and the
    /// table grants the role holds, each one line, ordered by the store so
    /// two readings compare as text.
    fn grants(&self) -> Result<Vec<String>, CustodyFault> {
        let rows = self
            .client()
            .query(
                "SELECT line FROM (
                     SELECT 'role ' || rolname || ' super=' || rolsuper::text
                         || ' createrole=' || rolcreaterole::text
                         || ' createdb=' || rolcreatedb::text AS line
                     FROM pg_roles WHERE rolname = current_user
                     UNION ALL
                     SELECT 'member ' || r.rolname FROM pg_auth_members m
                     JOIN pg_roles r ON r.oid = m.roleid
                     WHERE m.member = (SELECT oid FROM pg_roles WHERE rolname = current_user)
                     UNION ALL
                     SELECT 'database ' || datname || ' acl=' || COALESCE(datacl::text, '')
                     FROM pg_database WHERE datname = current_database()
                     UNION ALL
                     SELECT 'table ' || table_schema || '.' || table_name || ' ' || privilege_type
                     FROM information_schema.role_table_grants
                     WHERE grantee = current_user
                 ) surface ORDER BY line",
                &[],
            )
            .map_err(unavailable)?;
        Ok(rows.into_iter().map(|r| r.get(0)).collect())
    }

    fn identity(&self, session: &str) -> Result<Vec<RecalledEvent>, CustodyFault> {
        let mut client = self.client();
        let rows = client
            .query(
                "SELECT id, session, run, turn, kind, sequence FROM event
                 WHERE session = $1 AND kind = 'message.system' AND turn IS NULL
                   AND run = (SELECT run FROM event
                              WHERE session = $1 AND kind = 'message.system'
                                AND turn IS NULL
                              ORDER BY id DESC LIMIT 1)
                 ORDER BY id",
                &[&session],
            )
            .map_err(unavailable)?;
        with_pairs(&mut client, rows)
    }
}
