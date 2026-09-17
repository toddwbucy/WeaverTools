//! conforms: state-store-is-a-port
//! conforms: state-distillate-lands-whole
//! conforms: state-indexes-built-at-load
//!
//! The service engine, per `weaver-state-Spec` section 3 and the ruling of
//! 2026-09-04: one database per agent, reached over the store's unix socket
//! under the member's own account, the store's peer authentication mapping
//! that account to the role the binding declares. Behind the `postgres`
//! feature. The same two-table shape as the embedded engine, in this engine's
//! dialect, and the same port, whole.
//!
//! One of the section's claims this file does not cite, and it does not
//! return here on this reading: a citation is earned by holding an instrument
//! or by a reading that holds, and this engine holds neither for it.
//! `state-serve-restricts-to-the-session` is tagged `perturbation` and a
//! perturbation claim is bought by a test, and no unit here constructs a
//! `Postgres`, so the session predicate could leave `shape` and every device
//! would still answer green.
//!
//! `state-indexes-built-at-load` is cited above and its instrument is the
//! suite at the foot of this file, which buys what the Spec's section 5
//! names for that claim: the statement a build would issue carries a name
//! derived from the key path rather than from the key's position, so a later
//! load's differing election cannot fall under an earlier load's name
//! through `CREATE INDEX IF NOT EXISTS`. **The suite compiles only under
//! `--features postgres`**, the crate defaulting to sqlite, so a run without
//! that flag runs none of it and answers nothing about this engine.
//!
//! Three halves of the claim stand on a reading rather than on a run, and
//! section 5 carries all three. The timing is reached by no test in either
//! engine. The catalog is reached by no test here, no unit constructing a
//! `Postgres`, so what the suite reads is the statement rather than the index
//! the store then holds. The width the statements are measured against is
//! the store's own, read at open, and no test stands a server to watch that
//! reading.
//!
//! The naming answers `weaver-state-Spec` section 3's election and the
//! encoding is this act's under it, per that clause. The store truncates an
//! identifier past its stated width rather than refusing it, which is the
//! path issue #618 measured to a collision, so a name is made whole here or
//! the election is refused.

use std::cell::{RefCell, RefMut};

use postgres::{Client, GenericClient, NoTls};

use crate::store::{CustodyFault, Distillate, Election, RecalledEvent, RunShape, Store};

/// The service engine. The port's asks take `&self` and the wire is a
/// stream that needs `&mut`, so the client sits behind a cell: one member
/// holds one connection and serves one ask at a time, so the cell is never
/// contended, and a contended borrow would be a defect worth the panic.
pub struct Postgres {
    client: RefCell<Client>,
    /// The width this store holds an identifier to, read from the store at
    /// open. It is a build-time constant of the server rather than of this
    /// crate, so assuming it is the assumption issue #618 was made of.
    identifier_limit: usize,
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
        // **The ceiling is asked for rather than assumed.** A server built
        // with another `NAMEDATALEN` truncates at another width, and a name
        // measured against the wrong number is the silent collision again.
        let stated: String = client
            .query_one("SHOW max_identifier_length", &[])
            .map_err(unavailable)?
            .get(0);
        let identifier_limit = stated.trim().parse::<usize>().map_err(|_| {
            CustodyFault::StoreUnavailable(format!(
                "the store states its identifier width as {stated:?}, which is not a number, \
                 so no elected index name can be measured"
            ))
        })?;
        Ok(Postgres {
            client: RefCell::new(client),
            identifier_limit,
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

/// The prefix every elected index's name carries, and the reason it is not
/// the `field_elected_` the embedded engine still uses. The pre-#618 names
/// were that prefix and hex, so their space is `field_elected_[0-9a-f]+`, and
/// the encoding below passes a hex digit through as itself: elected path `ab`
/// named `field_elected_6162` under the old scheme and elected path `6162`
/// names the same string under the new one. A store carried across the two
/// elections would find the name taken, `IF NOT EXISTS` would return quietly,
/// and the standing index would carry the other path's predicate, which is
/// issue #618 raised from its own repair. A prefix the old space cannot reach
/// keeps the two disjoint. Indexes the old scheme built are not dropped here,
/// and what a load owes a name its election does not carry is an open
/// question this act names rather than answers.
const ELECTED_PREFIX: &str = "field_key_";

/// One elected key path's index name, or the refusal `weaver-state-Spec`
/// section 3 asks for where the store's identifier limit cannot hold it.
/// `limit` is the store's own `max_identifier_length`, read at open.
///
/// The encoding keeps a lowercase letter or a digit as itself and writes
/// every other byte as an underscore and two hex digits, so an underscore
/// never appears but as an escape and one name reads back to one key path.
/// Two properties come of that. The name is derived from the path rather
/// than from the key's position in the election, which is what keeps a later
/// load's differing election from falling under an earlier load's name. And
/// the name carries nothing the store folds, where passing an uppercase byte
/// through would let two paths differing only in case name one index, an
/// unquoted identifier being folded to lowercase before it is stored.
///
/// The budget is uneven and the refusal message says so: the prefix spends
/// ten, a byte inside `[a-z0-9]` spends one, and every other byte, the dots
/// of a key path among them, spends three.
fn elected_index_name(key: &str, limit: usize) -> Result<String, CustodyFault> {
    use std::fmt::Write;
    let mut name = String::with_capacity(ELECTED_PREFIX.len() + key.len() * 3);
    name.push_str(ELECTED_PREFIX);
    for byte in key.bytes() {
        match byte {
            b'a'..=b'z' | b'0'..=b'9' => name.push(char::from(byte)),
            _ => {
                let _ = write!(name, "_{byte:02x}");
            }
        }
    }
    if name.len() > limit {
        return Err(CustodyFault::StoreUnavailable(format!(
            "the elected key path {key:?} of {} bytes names an index of {} bytes and this \
             store holds an identifier to {limit}, the name spending {} on its prefix, one \
             on a byte inside a-z0-9 and three on every other, so the election is refused \
             rather than built in the part that fits",
            key.len(),
            name.len(),
            ELECTED_PREFIX.len()
        )));
    }
    Ok(name)
}

/// The statements one election's partial indexes are built by, every name
/// made before the first statement is issued.
///
/// The naming is its own pass because the refusal is the whole election's. A
/// name refused in the middle of the build would leave the indexes named
/// before it standing, and a subset of the election is the silent loss the
/// refusal exists to prevent, read from the other end. An execution failure
/// would leave the same subset, so both callers run the statements inside a
/// transaction and this engine's DDL rolls back with it.
///
/// A path elected under two kinds states itself once. The name and the
/// predicate are functions of the path alone, so the second statement would
/// be byte-identical and a round trip spent on an `IF NOT EXISTS` no-op.
///
/// The key is a bound-in literal within the WHERE, single quotes doubled.
/// That is what keeps an elected path carrying a quote from closing the
/// literal, and it is sufficient while the store's `standard_conforming_strings`
/// is on, which is its default and which this code does not read.
fn elected_index_statements(
    election: &Election,
    limit: usize,
) -> Result<Vec<String>, CustodyFault> {
    let paths: usize = election.keys.iter().map(|(_, keys)| keys.len()).sum();
    let mut statements = Vec::with_capacity(paths);
    let mut named = std::collections::HashSet::with_capacity(paths);
    for (_kind, keys) in &election.keys {
        for key in keys {
            let name = elected_index_name(key, limit)?;
            if !named.insert(name.clone()) {
                continue;
            }
            statements.push(format!(
                "CREATE INDEX IF NOT EXISTS {name} ON field (key, value) WHERE key = {}",
                quoted(key)
            ));
        }
    }
    Ok(statements)
}

/// The elected keys' partial indexes, one per elected key path, built at
/// load and never mid-serve, on whatever holds the connection: both callers
/// hand this an open transaction, so a failure part way through leaves no
/// part of the election standing.
fn build_indexes(
    executor: &mut impl GenericClient,
    election: &Election,
    limit: usize,
) -> Result<(), CustodyFault> {
    for statement in elected_index_statements(election, limit)? {
        executor.batch_execute(&statement).map_err(unavailable)?;
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
        // **The build is one transaction.** The naming pass refuses an
        // election it cannot name whole, and this is the other half of the
        // same property: a statement failing on a lock, a permission or the
        // disk would otherwise leave the indexes issued before it standing
        // and the election half built, with the same silence. The other
        // door is inside the retirement's transaction already.
        let limit = self.identifier_limit;
        let client = self.client.get_mut();
        let mut transaction = client.transaction().map_err(unavailable)?;
        build_indexes(&mut transaction, election, limit)?;
        transaction.commit().map_err(unavailable)
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
        let limit = self.identifier_limit;
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
        build_indexes(&mut transaction, election, limit)?;
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

/// The naming suite, which is this engine's instrument for
/// `state-indexes-built-at-load` and reaches the store not at all: what a
/// name is made of, what statement carries it and when a name is refused are
/// properties of the derivation, so they are watched where the derivation is
/// and no unit here stands a server to watch them. **The suite compiles only
/// under `--features postgres`**, sqlite being the crate's default, so a run
/// without that flag runs none of it.
#[cfg(test)]
mod tests {
    use super::*;

    /// The width this store states, which the suite pins explicitly so a
    /// refusal is measured against a number rather than against whatever the
    /// box the suite runs on happens to have been built with.
    const STATED: usize = 63;

    /// An election carrying the given key paths under one kind.
    fn elect(keys: &[&str]) -> Election {
        Election {
            all_kinds: true,
            keys: vec![(
                "message.assistant".into(),
                keys.iter().map(|key| (*key).to_string()).collect(),
            )],
        }
    }

    /// **The statement carries the name the path derives**, which is the
    /// property issue #618 is about and the one a test over the derivation
    /// alone does not reach: a build emitting a positional name while still
    /// calling the deriving function passes every other test in this suite.
    /// The whole statement is pinned, so the encoding, the name in the
    /// clause and the predicate move together or this fails.
    #[test]
    fn the_statement_carries_the_name_the_path_derives() {
        let built = elected_index_statements(&elect(&["message.assistant.content.text"]), STATED)
            .expect("a nameable election builds");
        assert_eq!(built.len(), 1, "one statement per elected key path");
        assert_eq!(
            built[0],
            "CREATE INDEX IF NOT EXISTS field_key_message_2eassistant_2econtent_2etext \
             ON field (key, value) WHERE key = 'message.assistant.content.text'",
            "the statement names the index from the path and filters to it"
        );
    }

    /// Issue #618's own case, which the hex encoding collided: two elected
    /// paths of thirty bytes sharing their first twenty-six, each owning its
    /// name here and each name short enough that the store stores it whole.
    #[test]
    fn two_elected_paths_never_share_a_name() {
        let text =
            elected_index_name("message.assistant.content.text", STATED).expect("names the path");
        let kind =
            elected_index_name("message.assistant.content.type", STATED).expect("names the path");
        assert_ne!(text, kind, "each elected path owns its index name");
        assert!(
            text.len() <= STATED && kind.len() <= STATED,
            "a name the store would truncate is a name two paths can share"
        );
    }

    /// The store folds an unquoted identifier to lowercase before it stores
    /// it, so a path's uppercase byte owes an escape rather than the byte.
    #[test]
    fn two_paths_differing_only_in_case_name_two_indexes() {
        let lower = elected_index_name("turn.closed", STATED).expect("names the path");
        let upper = elected_index_name("turn.Closed", STATED).expect("names the path");
        assert_ne!(
            lower, upper,
            "case is part of the path and part of the name"
        );
        assert_eq!(
            upper,
            upper.to_lowercase(),
            "the store folds nothing this name carries"
        );
    }

    /// A path the identifier limit cannot hold is refused with a named
    /// fault, rather than standing an index under a name the store truncated.
    #[test]
    fn a_name_that_cannot_be_made_is_a_refusal() {
        let long = "message.assistant.content.text.rendered.for.the.operator";
        match elected_index_name(long, STATED) {
            Err(CustodyFault::StoreUnavailable(said)) => {
                assert!(said.contains(long), "the fault names the path it refused");
                assert!(
                    said.contains("63"),
                    "the fault names the width it was measured against"
                );
            }
            other => panic!("a path over the limit is a refusal, and this answered {other:?}"),
        }
    }

    /// **The limit is the store's and not this file's.** A path a wide store
    /// names is refused by a narrow one, so a server built with another
    /// `NAMEDATALEN` refuses rather than truncating behind the name.
    #[test]
    fn the_refusal_is_measured_against_the_width_the_store_states() {
        let path = "message.assistant.content.text";
        assert!(
            elected_index_name(path, STATED).is_ok(),
            "the path fits the width this store states"
        );
        assert!(
            elected_index_name(path, 40).is_err(),
            "the same path is refused by a store that states a narrower width"
        );
    }

    /// The refusal is the election's and not one key's: an election carrying
    /// a path the limit cannot hold builds none of its indexes rather than
    /// the subset that fits.
    #[test]
    fn an_election_that_cannot_be_named_builds_nothing() {
        let held = "message.assistant.content.text";
        let over = "message.assistant.content.text.rendered.for.the.operator";
        assert!(
            elected_index_statements(&elect(&[held, over]), STATED).is_err(),
            "an election holding a path the limit cannot name is refused whole"
        );
    }

    /// A path elected under two kinds states itself once. The name and the
    /// predicate are the path's, so the second statement would be identical
    /// and the round trip would buy an `IF NOT EXISTS` no-op.
    #[test]
    fn a_path_elected_under_two_kinds_is_stated_once() {
        let election = Election {
            all_kinds: true,
            keys: vec![
                ("message.assistant".into(), vec!["content.text".into()]),
                ("message.user".into(), vec!["content.text".into()]),
            ],
        };
        let built = elected_index_statements(&election, STATED).expect("builds");
        assert_eq!(built.len(), 1, "one statement per elected key path");
    }

    /// A quote in an elected path is doubled inside the predicate rather
    /// than closing the literal. This is what `quoted` guarantees, and it
    /// guarantees it while the store's `standard_conforming_strings` is on.
    #[test]
    fn a_quote_in_a_path_is_doubled_in_the_predicate() {
        let built = elected_index_statements(&elect(&["a'b"]), STATED).expect("builds");
        assert_eq!(built.len(), 1);
        assert!(
            built[0].ends_with("WHERE key = 'a''b'"),
            "the predicate carries the path with its quote doubled, and this reads {}",
            built[0]
        );
    }
}
