//! conforms: web-nothing-is-computed-at-read-time-unless-the-query-is-recorded
//!
//! The four reads of `weaver-web-Spec` section 4, each an index hit over
//! the schema of section 2, and none of them deriving a value: what a read
//! returns was stored at ingest or authored, per section 2.7, and a reader
//! that wants a derived value asks the recorded query of section 2.6.
//!
//! **A read returns rows and not readings.** Position one is the click, the
//! range is the timeline, the tuple is the label on every reading, and the
//! sweep is one experiment's value set with each value's run. What the
//! surface draws from them is the surface's, and the conversion between an
//! ordinal and a position is the surface's too, per section 6.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use super::Store;
use super::experiment::{Arm, ExperimentState, StagedExperiment, Sweep};
use super::key::{PositionKey, RunId, TurnId};

/// Read one: one position's alternatives, per section 2.1. This is
/// `weaver-analysis field` served from the store rather than re-derived.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternatives {
    pub key: PositionKey,
    pub token_id: i64,
    pub token_text: String,
    /// Rides every generation unconditionally, per section 6.
    pub entropy: f64,
    /// Rides only where its election stands. Absent is absent and never zero.
    pub surprisal: Option<f64>,
    /// The ranked candidates with their mass, at the depth the declaration's
    /// field election kept.
    pub alternatives: serde_json::Value,
    /// A rank and not a token, per section 2.1.
    pub realized: i32,
    pub residual: Option<Vec<u8>>,
}

/// Read two: one point of a contiguous range, per section 2.1. The timeline
/// and the transcript.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionPoint {
    pub position: i32,
    pub token_id: i64,
    pub token_text: String,
    pub entropy: f64,
    pub surprisal: Option<f64>,
}

/// Read three: the run's row, per section 2.2. The tuple is what identifies
/// the conditions, and lineage and the signature stand beside it outside
/// tuple equality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunTuple {
    pub run: RunId,
    pub record_identity: String,
    /// The record spells the seed as an unsigned 64-bit value the suite's
    /// fixtures carry above the signed maximum, so it crosses as text and
    /// the schema holds it as `NUMERIC(20,0)`.
    pub seed: Option<String>,
    pub sampler: serde_json::Value,
    pub device: String,
    pub compute_precision: String,
    pub engine: serde_json::Value,
    pub batching: serde_json::Value,
    pub field_depth: Option<i32>,
    pub task_source: Option<String>,
    pub task_identity: Option<String>,
    pub boundary_set: serde_json::Value,
    pub forced_position: Option<i32>,
    pub forced_token: Option<String>,
    /// Lineage, outside the compound.
    pub parent_run: Option<RunId>,
    pub branch_position: Option<i32>,
    /// Lineage, outside the compound: the first position this run's token
    /// path left its parent's, derived at ingest. `None` where the paths
    /// never parted or where there is no parent.
    pub parting_position: Option<i32>,
    /// The emission's signature, outside the compound. Its representation is
    /// section 10's open election.
    pub signature: Option<serde_json::Value>,
    pub ingested_at: DateTime<Utc>,
}

impl Store {
    /// **Read one.** One position's alternatives, addressed by the whole key.
    pub async fn alternatives_at(&self, key: &PositionKey) -> anyhow::Result<Option<Alternatives>> {
        let row = sqlx::query(
            "SELECT token_id, token_text, entropy, surprisal, alternatives, realized, residual \
             FROM position WHERE run_id = $1 AND turn = $2 AND position = $3",
        )
        .bind(&key.run.0)
        .bind(&key.turn.0)
        .bind(key.position)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| Alternatives {
            key: key.clone(),
            token_id: r.get("token_id"),
            token_text: r.get("token_text"),
            entropy: r.get("entropy"),
            surprisal: r.get("surprisal"),
            alternatives: r.get("alternatives"),
            realized: r.get("realized"),
            residual: r.get("residual"),
        }))
    }

    /// **Read two.** A contiguous range of positions within one turn,
    /// inclusive at both ends, in position order.
    pub async fn range(
        &self,
        run: &RunId,
        turn: &TurnId,
        from: i32,
        to: i32,
    ) -> anyhow::Result<Vec<PositionPoint>> {
        let rows = sqlx::query(
            "SELECT position, token_id, token_text, entropy, surprisal \
             FROM position WHERE run_id = $1 AND turn = $2 \
             AND position BETWEEN $3 AND $4 ORDER BY position",
        )
        .bind(&run.0)
        .bind(&turn.0)
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| PositionPoint {
                position: r.get("position"),
                token_id: r.get("token_id"),
                token_text: r.get("token_text"),
                entropy: r.get("entropy"),
                surprisal: r.get("surprisal"),
            })
            .collect())
    }

    /// **Read three.** The run's row.
    pub async fn tuple(&self, run: &RunId) -> anyhow::Result<Option<RunTuple>> {
        let row = sqlx::query(
            "SELECT run_id, record_identity, seed::text AS seed, sampler, device, \
             compute_precision, engine, batching, field_depth, task_source, \
             task_identity, boundary_set, forced_position, forced_token, \
             parent_run_id, branch_position, parting_position, signature, ingested_at \
             FROM run WHERE run_id = $1",
        )
        .bind(&run.0)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(run_tuple_from_row))
    }

    /// **Read four.** One experiment's value set, each value with its run
    /// where one exists. `None` where no such experiment stands, and
    /// `Some` with no arms where the experiment is not a sweep.
    pub async fn sweep(&self, experiment_id: i64) -> anyhow::Result<Option<Sweep>> {
        let Some(experiment) = self.experiment(experiment_id).await? else {
            return Ok(None);
        };
        let (Some(member), Some(values)) = (
            experiment.swept_member.clone(),
            experiment.swept_values.clone(),
        ) else {
            return Ok(Some(Sweep {
                experiment,
                member: String::new(),
                arms: Vec::new(),
            }));
        };

        // The runs this experiment produced, each with the value it was
        // produced under. One query, the association being explicit in the
        // schema rather than recovered by matching tuples against the set.
        let produced = sqlx::query(
            "SELECT ser.swept_value, r.run_id, r.record_identity, r.seed::text AS seed, \
             r.sampler, r.device, r.compute_precision, r.engine, r.batching, \
             r.field_depth, r.task_source, r.task_identity, r.boundary_set, \
             r.forced_position, r.forced_token, r.parent_run_id, r.branch_position, \
             r.parting_position, r.signature, r.ingested_at \
             FROM staged_experiment_run ser JOIN run r ON r.run_id = ser.run_id \
             WHERE ser.experiment_id = $1",
        )
        .bind(experiment_id)
        .fetch_all(&self.pool)
        .await?;

        let mut by_value: Vec<(serde_json::Value, RunTuple)> = produced
            .into_iter()
            .map(|r| {
                let v: Option<serde_json::Value> = r.get("swept_value");
                (v.unwrap_or(serde_json::Value::Null), run_tuple_from_row(r))
            })
            .collect();

        // The unit is the value. Every value in the frozen set returns,
        // with its run where one was produced under it, so an arm that
        // never ran keeps its place.
        let arms = values
            .into_iter()
            .map(|value| {
                let run = by_value
                    .iter()
                    .position(|(v, _)| *v == value)
                    .map(|i| by_value.remove(i).1);
                Arm { value, run }
            })
            .collect();

        Ok(Some(Sweep {
            experiment,
            member,
            arms,
        }))
    }

    /// The experiment's own row, by id. Not one of the four reads on its
    /// own: read four calls it, and the Experiments list of the charter's
    /// section 3.6 reads through the same column set.
    pub async fn experiment(&self, experiment_id: i64) -> anyhow::Result<Option<StagedExperiment>> {
        let row = sqlx::query(
            "SELECT experiment_id, state, state_changed_at, parent_run_id, branch_position, \
             forced_token, parent_declaration_id, diff_at_load, diff_at_turn, question, \
             swept_member, swept_values, author, version \
             FROM staged_experiment WHERE experiment_id = $1",
        )
        .bind(experiment_id)
        .fetch_optional(&self.pool)
        .await?;
        row.map(|r| {
            let state_text: String = r.get("state");
            let state = ExperimentState::parse(&state_text)
                .ok_or_else(|| anyhow::anyhow!("staged_experiment {experiment_id} holds a state the five do not name: {state_text}"))?;
            let swept_values: Option<serde_json::Value> = r.get("swept_values");
            let swept_values = match swept_values {
                Some(serde_json::Value::Array(items)) => Some(items),
                Some(_) => anyhow::bail!("staged_experiment {experiment_id} holds swept_values that is not an array"),
                None => None,
            };
            Ok(StagedExperiment {
                experiment_id: r.get("experiment_id"),
                state,
                state_changed_at: r.get("state_changed_at"),
                parent_run: r.get::<Option<String>, _>("parent_run_id").map(RunId),
                branch_position: r.get("branch_position"),
                forced_token: r.get("forced_token"),
                parent_declaration_id: r.get("parent_declaration_id"),
                diff_at_load: r.get("diff_at_load"),
                diff_at_turn: r.get("diff_at_turn"),
                question: r.get("question"),
                swept_member: r.get("swept_member"),
                swept_values,
                author: r.get("author"),
                version: r.get("version"),
            })
        })
        .transpose()
    }
}

fn run_tuple_from_row(r: sqlx::postgres::PgRow) -> RunTuple {
    RunTuple {
        run: RunId(r.get("run_id")),
        record_identity: r.get("record_identity"),
        seed: r.get("seed"),
        sampler: r.get("sampler"),
        device: r.get("device"),
        compute_precision: r.get("compute_precision"),
        engine: r.get("engine"),
        batching: r.get("batching"),
        field_depth: r.get("field_depth"),
        task_source: r.get("task_source"),
        task_identity: r.get("task_identity"),
        boundary_set: r.get("boundary_set"),
        forced_position: r.get("forced_position"),
        forced_token: r.get("forced_token"),
        parent_run: r.get::<Option<String>, _>("parent_run_id").map(RunId),
        branch_position: r.get("branch_position"),
        parting_position: r.get("parting_position"),
        signature: r.get("signature"),
        ingested_at: r.get("ingested_at"),
    }
}

#[cfg(test)]
mod tests {
    //! These run against a live PostgreSQL named by `DATABASE_URL`, because
    //! a read over a schema is tested against the schema or not at all.
    //! Without the variable they pass by not running and say so, so a box
    //! with no database still passes the workspace.

    use super::*;
    use crate::store::Registered;
    use serde_json::json;

    async fn store() -> Option<Store> {
        let Ok(url) = std::env::var("DATABASE_URL") else {
            eprintln!(
                "skipped: DATABASE_URL is not set, and a read over a schema is tested against one"
            );
            return None;
        };
        Some(Store::connect(&url).await.expect("connect and migrate"))
    }

    async fn seed_run(s: &Store, run_id: &str, parent: Option<&str>, parting: Option<i32>) {
        sqlx::query(
            "INSERT INTO run (run_id, record_identity, seed, sampler, device, compute_precision, \
             engine, batching, boundary_set, parent_run_id, branch_position, parting_position, signature) \
             VALUES ($1, 'REC', 14458752852352082704, '{}', 'cuda:0', 'bf16', '{}', '{}', '{}', $2, $3, $4, $5) \
             ON CONFLICT (run_id) DO NOTHING",
        )
        .bind(run_id)
        .bind(parent)
        .bind(parent.map(|_| 22))
        .bind(parting)
        .bind(json!({"shingles": [1, 2, 3]}))
        .execute(&s.pool)
        .await
        .expect("seed run");
    }

    #[tokio::test]
    async fn read_one_is_addressed_by_the_whole_key_and_absence_is_none() {
        let Some(s) = store().await else { return };
        seed_run(&s, "r-read1", None, None).await;
        sqlx::query(
            "INSERT INTO position (run_id, turn, position, token_id, token_text, entropy, surprisal, alternatives, realized) \
             VALUES ('r-read1', 't-1', 154, 19026, 'Okay', 0.0024, NULL, '[{\"token\":19026,\"probability\":0.9998}]', 0) \
             ON CONFLICT DO NOTHING",
        )
        .execute(&s.pool)
        .await
        .unwrap();

        let key = PositionKey {
            run: RunId("r-read1".into()),
            turn: TurnId("t-1".into()),
            position: 154,
        };
        let got = s
            .alternatives_at(&key)
            .await
            .unwrap()
            .expect("the row stands");
        assert_eq!(got.token_id, 19026);
        assert_eq!(got.realized, 0);
        // Absent-not-empty: surprisal rode no election here and reads as None, never as zero.
        assert_eq!(got.surprisal, None);

        let elsewhere = PositionKey {
            position: 155,
            ..key
        };
        assert!(s.alternatives_at(&elsewhere).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn read_two_is_ordered_and_inclusive() {
        let Some(s) = store().await else { return };
        seed_run(&s, "r-read2", None, None).await;
        for (p, e) in [(200, 0.5), (202, 0.7), (201, 0.6), (203, 0.8)] {
            sqlx::query(
                "INSERT INTO position (run_id, turn, position, token_id, token_text, entropy, alternatives, realized) \
                 VALUES ('r-read2', 't-1', $1, 1, 'x', $2, '[]', 0) ON CONFLICT DO NOTHING",
            )
            .bind(p)
            .bind(e)
            .execute(&s.pool)
            .await
            .unwrap();
        }
        let pts = s
            .range(&RunId("r-read2".into()), &TurnId("t-1".into()), 201, 203)
            .await
            .unwrap();
        assert_eq!(
            pts.iter().map(|p| p.position).collect::<Vec<_>>(),
            vec![201, 202, 203]
        );
        assert_eq!(pts[0].entropy, 0.6);
    }

    #[tokio::test]
    async fn read_three_carries_the_seed_the_record_spells() {
        let Some(s) = store().await else { return };
        seed_run(&s, "r-read3", None, None).await;
        let t = s
            .tuple(&RunId("r-read3".into()))
            .await
            .unwrap()
            .expect("the row stands");
        // The suite's fixture seed is above i64::MAX and crosses as text unchanged.
        assert_eq!(t.seed.as_deref(), Some("14458752852352082704"));
        assert_eq!(t.parent_run, None);
        assert_eq!(t.parting_position, None);
    }

    #[tokio::test]
    async fn read_four_returns_every_value_and_an_unrun_arm_keeps_its_place() {
        let Some(s) = store().await else { return };
        seed_run(&s, "r-sweep-parent", None, None).await;
        seed_run(&s, "r-sweep-arm-7", Some("r-sweep-parent"), Some(149)).await;
        seed_run(&s, "r-sweep-arm-1000003", Some("r-sweep-parent"), None).await;

        let id: i64 = sqlx::query_scalar(
            "INSERT INTO staged_experiment (state, parent_run_id, branch_position, question, author, \
             swept_member, swept_values) \
             VALUES ('registered', 'r-sweep-parent', 22, 'does the seed reach the sampler', 'todd', \
             'seed', '[7, 1000003, 123456789]') RETURNING experiment_id",
        )
        .fetch_one(&s.pool)
        .await
        .unwrap();
        for (run, v) in [("r-sweep-arm-7", 7), ("r-sweep-arm-1000003", 1000003)] {
            sqlx::query("INSERT INTO staged_experiment_run (experiment_id, run_id, swept_value) VALUES ($1, $2, $3)")
                .bind(id)
                .bind(run)
                .bind(json!(v))
                .execute(&s.pool)
                .await
                .unwrap();
        }

        let sweep = s.sweep(id).await.unwrap().expect("the experiment stands");
        assert_eq!(sweep.member, "seed");
        // The unit is the value: three values return, two with runs and one without.
        assert_eq!(sweep.arms.len(), 3);
        let by_value: Vec<(i64, bool)> = sweep
            .arms
            .iter()
            .map(|a| (a.value.as_i64().unwrap(), a.run.is_some()))
            .collect();
        assert_eq!(
            by_value,
            vec![(7, true), (1000003, true), (123456789, false)]
        );
        // The arm that parted carries where, and the one that reproduced its parent carries None.
        assert_eq!(
            sweep.arms[0].run.as_ref().unwrap().parting_position,
            Some(149)
        );
        assert_eq!(sweep.arms[1].run.as_ref().unwrap().parting_position, None);

        // The pin: a registered row wraps, and a draft is refused.
        assert!(Registered::new(sweep.experiment.clone()).is_ok());
        let mut draft = sweep.experiment.clone();
        draft.state = ExperimentState::Draft;
        assert!(Registered::new(draft).is_err());
    }

    #[tokio::test]
    async fn an_experiment_that_is_not_a_sweep_has_no_arms() {
        let Some(s) = store().await else { return };
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO staged_experiment (state, question) VALUES ('draft', 'a point, not a sweep') \
             RETURNING experiment_id",
        )
        .fetch_one(&s.pool)
        .await
        .unwrap();
        let sweep = s.sweep(id).await.unwrap().unwrap();
        assert!(sweep.arms.is_empty());
        assert_eq!(sweep.experiment.state, ExperimentState::Draft);
        assert!(s.sweep(i64::MAX).await.unwrap().is_none());
    }
}
