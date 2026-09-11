//! The store: the Postgres pool, the migrations, and the reads of
//! `weaver-web-Spec` section 4 over the tables of its section 2.
//!
//! **Four of that section's five are served here.** The fifth, every run's
//! tuple filtered, is Record's and lands with that surface, and the plan's
//! is owed at the act that gives the plan its schema.
//!
//! This module is the first in the crate written to the standing Spec
//! rather than to the charter it replaced, which is why it is the first
//! whose files carry conformance headers. The register at
//! `docs/project/inventory-weaver-web-code.md` records the rest of the crate
//! as written to the retired text.
//!
//! `conversation` holds what the prior `store.rs` held for the conversation
//! half: the writer task and its commands against tables the schema no
//! longer creates. It stands so the modules the register retires still
//! build, and it goes with them.

pub mod conversation;
pub mod experiment;
pub mod key;
pub mod read;

pub use conversation::{ChannelEvent, KindConflict, NewEvent};
pub use experiment::{Arm, Experiment, ExperimentState, Registered, StagedExperiment, Sweep};
pub use key::{PositionKey, RunId, TurnId};
pub use read::{Alternatives, PositionPoint, RunTuple};

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::{broadcast, mpsc};

#[derive(Clone)]
pub struct Store {
    pub pool: PgPool,
    write_tx: mpsc::Sender<conversation::WriteCmd>,
    events_tx: broadcast::Sender<ChannelEvent>,
}

impl Store {
    /// Connect and bring the schema to the migrations' head. `sqlx` checksums
    /// each applied version against its source, so a database that ran a
    /// migration this tree no longer carries refuses here rather than
    /// running against a schema it was not written for, per the ruling at
    /// PR #499 that the schema is replaced rather than migrated.
    pub async fn connect(database_url: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(database_url)
            .await?;
        sqlx::migrate!("./migrations").run(&pool).await?;

        let (write_tx, write_rx) = mpsc::channel(256);
        let (events_tx, _) = broadcast::channel(1024);
        tokio::spawn(conversation::writer_task(
            pool.clone(),
            write_rx,
            events_tx.clone(),
        ));

        Ok(Self {
            pool,
            write_tx,
            events_tx,
        })
    }
}
