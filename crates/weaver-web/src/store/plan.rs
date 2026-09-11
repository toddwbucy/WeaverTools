//! conforms: web-nothing-is-computed-at-read-time-unless-the-query-is-recorded
//!
//! **The sixth read of `weaver-web-Spec` section 4: one plan whole.**
//!
//! Section 4 named this read owed rather than discovered - "section 2.9's
//! plan is rendered by the matrix with its columns and their entries, and
//! none of these five returns it, so that read is owed at the act that
//! gives the plan its schema." This is that act, and the read lands with
//! the schema rather than after the surface that wants it, which is the
//! rule section 4 states about itself working a third time.
//!
//! **It derives nothing.** A column's status is not computed here: section
//! 5.1 has the column take the five states of the staged experiment it
//! became, so the status is that row's `state` read across the reference,
//! and a column whose reference is null has not been registered - which the
//! null itself records rather than a sixth word this document would have to
//! name.
//!
//! **A column is not a run.** Section 2.9 has a column reach its runs
//! through its staged experiment and never directly, so nothing here joins
//! a column to `run`.

use serde::Serialize;
use sqlx::Row as _;

use super::Store;

/// What an operator is still composing: one parent run, and the columns
/// each of which becomes at most one staged experiment.
#[derive(Debug, Clone, Serialize)]
pub struct Plan {
    pub plan: i64,
    /// The run every column branches from, per section 2.9.
    pub parent_run: String,
    /// Per section 3.2, and null where the store could not name one. **It
    /// never means the operator.**
    pub author: Option<String>,
    pub version: i64,
    pub columns: Vec<Column>,
}

/// One column of a plan, with the disposition of every member it names.
#[derive(Debug, Clone, Serialize)]
pub struct Column {
    pub key: String,
    /// The staged experiment this column became, where it has been
    /// registered.
    pub experiment: Option<i64>,
    /// **The column's status is the staged experiment's state**, per
    /// section 5.1, and absent where the column has not been registered.
    /// This document names no sixth vocabulary for the matrix to render.
    pub status: Option<String>,
    pub entries: Vec<Entry>,
}

/// One member of the tuple, as this column disposes of it.
#[derive(Debug, Clone, Serialize)]
pub struct Entry {
    pub member: String,
    pub disposition: Disposition,
}

/// **Held or freed, and the value travels with the disposition.** Section
/// 2.9 has the value where a member is held and the value set where it is
/// freed, and the two are never both present, so they are arms of one thing
/// rather than two nullable members a reader has to correlate.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case", tag = "disposition")]
pub enum Disposition {
    Held { value: serde_json::Value },
    Freed { values: Vec<serde_json::Value> },
}

/// The plan, its columns in the order they were authored, and each column's
/// entries. `None` where no plan carries this identity.
///
/// **Three statements and each an index hit**: the plan by its key, the
/// columns by the plan's, and the entries by the plan's. It is one read of
/// one plan rather than three reads, and it is stated as three because the
/// alternative is one join that returns the plan's row once per entry and a
/// caller that rebuilds the shape anyway.
const SELECT_PLAN: &str =
    "SELECT plan_id, parent_run_id, author, version FROM plan WHERE plan_id = $1";

const SELECT_COLUMNS: &str = "SELECT c.column_key, c.experiment_id, e.state \
     FROM plan_column c LEFT JOIN staged_experiment e ON e.experiment_id = c.experiment_id \
     WHERE c.plan_id = $1 ORDER BY c.column_key";

const SELECT_ENTRIES: &str = "SELECT column_key, member, disposition, held_value, freed_values \
     FROM plan_entry WHERE plan_id = $1 ORDER BY column_key, member";

impl Store {
    /// Read one plan whole, per section 4's sixth read.
    pub async fn plan(&self, plan: i64) -> anyhow::Result<Option<Plan>> {
        let Some(row) = sqlx::query(SELECT_PLAN)
            .bind(plan)
            .fetch_optional(&self.pool)
            .await?
        else {
            return Ok(None);
        };

        let mut columns: Vec<Column> = sqlx::query(SELECT_COLUMNS)
            .bind(plan)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|r| Column {
                key: r.get("column_key"),
                experiment: r.get("experiment_id"),
                status: r.get("state"),
                entries: Vec::new(),
            })
            .collect();

        for r in sqlx::query(SELECT_ENTRIES)
            .bind(plan)
            .fetch_all(&self.pool)
            .await?
        {
            let key: String = r.get("column_key");
            let disposition: String = r.get("disposition");
            // The schema's own check holds that a held entry carries a value
            // and a freed one carries a set, so an arm that found neither
            // would be a row the store should not be able to hold. It is an
            // error here rather than a default, a default being the
            // absent-not-empty failure moved into the reader.
            let disposition = match disposition.as_str() {
                "held" => Disposition::Held {
                    value: r
                        .try_get::<Option<serde_json::Value>, _>("held_value")?
                        .ok_or_else(|| anyhow::anyhow!("a held entry with no value: {key}"))?,
                },
                "freed" => {
                    let values = r
                        .try_get::<Option<serde_json::Value>, _>("freed_values")?
                        .ok_or_else(|| anyhow::anyhow!("a freed entry with no set: {key}"))?;
                    Disposition::Freed {
                        values: match values {
                            serde_json::Value::Array(v) => v,
                            other => anyhow::bail!("a freed entry's set is not an array: {other}"),
                        },
                    }
                }
                other => anyhow::bail!("a disposition the schema does not admit: {other}"),
            };
            if let Some(column) = columns.iter_mut().find(|c| c.key == key) {
                column.entries.push(Entry {
                    member: r.get("member"),
                    disposition,
                });
            }
        }

        Ok(Some(Plan {
            plan: row.get("plan_id"),
            parent_run: row.get("parent_run_id"),
            author: row.get("author"),
            version: row.get("version"),
            columns,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::read::tests::store;

    /// Seed a plan whose parent is a run of this invocation's own, and hand
    /// back the plan's key. **The rows are this run's alone**: the watches
    /// below count columns and entries, so anything else carrying the same
    /// plan would be counted as though this read returned it.
    async fn a_plan(s: &Store, tag: &str) -> i64 {
        sqlx::query(
            "INSERT INTO run (run_id, record_identity, sampler, boundary_set) \
             VALUES ($1, 'PLAN-REC', '{}', '[]')",
        )
        .bind(tag)
        .execute(&s.pool)
        .await
        .unwrap();
        sqlx::query_scalar(
            "INSERT INTO plan (parent_run_id, author) VALUES ($1, $2) RETURNING plan_id",
        )
        .bind(tag)
        .bind("todd")
        .fetch_one(&s.pool)
        .await
        .unwrap()
    }

    fn tag(prefix: &str) -> String {
        format!(
            "{prefix}-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        )
    }

    /// The sixth read returns a plan with its columns and their entries, and
    /// **a column's status is the staged experiment's state or is absent**,
    /// per section 5.1.
    ///
    /// conforms: web-nothing-is-computed-at-read-time-unless-the-query-is-recorded
    #[tokio::test]
    async fn read_six_returns_the_plan_with_its_columns_and_their_entries() {
        let Some(s) = store().await else { return };
        let tag = tag("plan");
        let plan = a_plan(&s, &tag).await;

        // One column registered, one not. The registered one is what makes
        // the status a read across the reference rather than a member.
        let experiment: i64 = sqlx::query_scalar(
            "INSERT INTO staged_experiment (state, question, parent_run_id) \
             VALUES ('registered', 'does the sampler carry it?', $1) RETURNING experiment_id",
        )
        .bind(&tag)
        .fetch_one(&s.pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO plan_column (plan_id, column_key, experiment_id) \
             VALUES ($1, 'arm-a', $2), ($1, 'arm-b', NULL)",
        )
        .bind(plan)
        .bind(experiment)
        .execute(&s.pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO plan_entry (plan_id, column_key, member, disposition, held_value, freed_values) \
             VALUES ($1, 'arm-a', 'sampler', 'freed', NULL, '[{\"top_p\": 0.9}, {\"top_p\": 0.95}]'), \
                    ($1, 'arm-a', 'device', 'held', '\"cuda:0\"', NULL), \
                    ($1, 'arm-b', 'device', 'held', '\"cuda:1\"', NULL)",
        )
        .bind(plan)
        .execute(&s.pool)
        .await
        .unwrap();

        let read = s.plan(plan).await.unwrap().expect("the plan is returned");
        assert_eq!(read.parent_run, tag, "the run every column branches from");
        assert_eq!(read.author.as_deref(), Some("todd"));
        assert_eq!(read.columns.len(), 2, "both columns, registered or not");

        let a = &read.columns[0];
        assert_eq!(a.key, "arm-a");
        assert_eq!(a.experiment, Some(experiment));
        // Perturbation: read the status from the column's own row and there
        // is nothing to read, the column having no state of its own. Drop
        // the join and this is None while the column is registered.
        assert_eq!(
            a.status.as_deref(),
            Some("registered"),
            "the column's status is the experiment's state"
        );
        assert_eq!(a.entries.len(), 2, "both members this column names");

        let b = &read.columns[1];
        assert_eq!(b.key, "arm-b");
        assert_eq!(b.experiment, None);
        // **A null reference records that the column was never registered**,
        // which is why no sixth word is named for it.
        assert_eq!(b.status, None, "an unregistered column has no status");

        // The value travels with the disposition rather than beside it.
        let freed = a
            .entries
            .iter()
            .find(|e| e.member == "sampler")
            .expect("the freed member");
        match &freed.disposition {
            Disposition::Freed { values } => {
                assert_eq!(values.len(), 2, "the set registration would freeze")
            }
            other => panic!("the sampler is freed, not {other:?}"),
        }
        let held = a
            .entries
            .iter()
            .find(|e| e.member == "device")
            .expect("the held member");
        match &held.disposition {
            Disposition::Held { value } => assert_eq!(value, "cuda:0"),
            other => panic!("the device is held, not {other:?}"),
        }

        assert!(
            s.plan(-1).await.unwrap().is_none(),
            "a plan nobody authored is absent rather than empty"
        );
    }

    /// **A column frees at most one member**, per section 2.9, section 5.4
    /// having a sweep name one member and its value set.
    ///
    /// Perturbation: drop `plan_column_frees_at_most_one_member` and a
    /// column frees two, which is a sweep of a set the sweep's own row
    /// cannot carry.
    #[tokio::test]
    async fn a_column_frees_at_most_one_member() {
        let Some(s) = store().await else { return };
        let tag = tag("free");
        let plan = a_plan(&s, &tag).await;
        sqlx::query("INSERT INTO plan_column (plan_id, column_key) VALUES ($1, 'arm')")
            .bind(plan)
            .execute(&s.pool)
            .await
            .unwrap();
        let free = |member: &'static str| {
            let pool = s.pool.clone();
            async move {
                sqlx::query(
                    "INSERT INTO plan_entry (plan_id, column_key, member, disposition, freed_values) \
                     VALUES ($1, 'arm', $2, 'freed', '[1, 2]')",
                )
                .bind(plan)
                .bind(member)
                .execute(&pool)
                .await
            }
        };
        free("sampler").await.expect("the first freed member lands");
        let second = free("device").await;
        assert!(second.is_err(), "the second is refused: {second:?}");

        // A column holding the rest is the ordinary case and is not touched
        // by the bound: the index is partial on the freed disposition.
        sqlx::query(
            "INSERT INTO plan_entry (plan_id, column_key, member, disposition, held_value) \
             VALUES ($1, 'arm', 'device', 'held', '\"cuda:0\"'), \
                    ($1, 'arm', 'engine', 'held', '{}')",
        )
        .bind(plan)
        .execute(&s.pool)
        .await
        .expect("held members are unbounded");
    }

    /// **A column registers at most once**, per section 2.9, the reference
    /// being what holds the claim: two columns cannot reach one staged
    /// experiment.
    ///
    /// This is the schema's half of `web-a-column-registers-at-most-once`.
    /// **The whole of that assertion is the registration write**, which
    /// section 5.1 has set the reference only where it was null and in the
    /// same transaction as the staged experiment, and which lands with
    /// Stage. Section 9's perturbation - register a plan twice - needs that
    /// write to exist before it can be run, so what is watched here is the
    /// constraint the write will lean on and not the write.
    #[tokio::test]
    async fn two_columns_cannot_claim_one_staged_experiment() {
        let Some(s) = store().await else { return };
        let tag = tag("claim");
        let plan = a_plan(&s, &tag).await;
        let experiment: i64 = sqlx::query_scalar(
            "INSERT INTO staged_experiment (state, question, parent_run_id) \
             VALUES ('registered', 'twice?', $1) RETURNING experiment_id",
        )
        .bind(&tag)
        .fetch_one(&s.pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO plan_column (plan_id, column_key, experiment_id) VALUES ($1, 'a', $2)",
        )
        .bind(plan)
        .bind(experiment)
        .execute(&s.pool)
        .await
        .expect("the first column claims it");
        let second = sqlx::query(
            "INSERT INTO plan_column (plan_id, column_key, experiment_id) VALUES ($1, 'b', $2)",
        )
        .bind(plan)
        .bind(experiment)
        .execute(&s.pool)
        .await;
        assert!(second.is_err(), "the second is refused: {second:?}");

        // Two unregistered columns are the ordinary case: the uniqueness is
        // over the reference and nulls do not collide.
        sqlx::query("INSERT INTO plan_column (plan_id, column_key) VALUES ($1, 'c'), ($1, 'd')")
            .bind(plan)
            .execute(&s.pool)
            .await
            .expect("unregistered columns do not collide");
    }

    /// **An entry states the value its disposition names**, per section 2.9,
    /// so a held entry with no value and a freed entry with no set are rows
    /// the store cannot hold.
    ///
    /// Perturbation: drop the check and the read above meets an entry whose
    /// disposition says held and whose value is absent, which is the
    /// absent-not-empty failure moved into the store.
    #[tokio::test]
    async fn an_entry_states_the_value_its_disposition_names() {
        let Some(s) = store().await else { return };
        let tag = tag("state");
        let plan = a_plan(&s, &tag).await;
        sqlx::query("INSERT INTO plan_column (plan_id, column_key) VALUES ($1, 'arm')")
            .bind(plan)
            .execute(&s.pool)
            .await
            .unwrap();
        // **Five statements and not one built from a string.** sqlx
        // refuses a dynamic query for the reason this crate agrees with,
        // and a watch that reached for `AssertSqlSafe` to say five things
        // would be spending that refusal on its own convenience.
        for (member, refused) in [
            (
                "held-with-no-value",
                sqlx::query(
                    "INSERT INTO plan_entry (plan_id, column_key, member, disposition) \
                     VALUES ($1, 'arm', $2, 'held')",
                ),
            ),
            (
                "freed-with-no-set",
                sqlx::query(
                    "INSERT INTO plan_entry (plan_id, column_key, member, disposition) \
                     VALUES ($1, 'arm', $2, 'freed')",
                ),
            ),
            (
                "held-carrying-a-set",
                sqlx::query(
                    "INSERT INTO plan_entry (plan_id, column_key, member, disposition, \
                     held_value, freed_values) VALUES ($1, 'arm', $2, 'held', '1', '[1]')",
                ),
            ),
            (
                "freed-whose-set-is-not-an-array",
                sqlx::query(
                    "INSERT INTO plan_entry (plan_id, column_key, member, disposition, \
                     freed_values) VALUES ($1, 'arm', $2, 'freed', '1')",
                ),
            ),
            (
                "a-disposition-of-its-own",
                sqlx::query(
                    "INSERT INTO plan_entry (plan_id, column_key, member, disposition, \
                     held_value) VALUES ($1, 'arm', $2, 'moved', '1')",
                ),
            ),
        ] {
            let outcome = refused.bind(plan).bind(member).execute(&s.pool).await;
            assert!(outcome.is_err(), "{member} is refused: {outcome:?}");
        }
    }
}
