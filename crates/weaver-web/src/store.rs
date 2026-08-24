//! The store: Postgres pool, migrations, and the single writer task.
//!
//! Every mutation flows through the writer (Spec section 5). Appended
//! channel events are broadcast in-process for SSE fan-out.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::sync::{broadcast, mpsc, oneshot};

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ChannelEvent {
    pub id: i64,
    pub channel_id: i64,
    pub ts: DateTime<Utc>,
    pub participant_id: Option<i64>,
    pub kind: String,
    pub body: Option<String>,
    pub run_label: Option<String>,
    pub turn_label: Option<String>,
    pub close_kind: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewEvent {
    pub channel_id: i64,
    pub participant_id: Option<i64>,
    pub kind: String,
    pub body: Option<String>,
    pub run_label: Option<String>,
    pub turn_label: Option<String>,
    pub close_kind: Option<String>,
}

impl NewEvent {
    pub fn message(channel_id: i64, participant_id: i64, text: String) -> Self {
        Self {
            channel_id,
            participant_id: Some(participant_id),
            kind: "message".into(),
            body: Some(text),
            run_label: None,
            turn_label: None,
            close_kind: None,
        }
    }
}

enum WriteCmd {
    Append {
        event: NewEvent,
        reply: oneshot::Sender<Result<ChannelEvent, sqlx::Error>>,
    },
    CreateParticipant {
        name: String,
        display: String,
        kind: String,
        adapter: Option<String>,
        reply: oneshot::Sender<Result<i64, sqlx::Error>>,
    },
    CreateChannel {
        name: String,
        topic: Option<String>,
        reply: oneshot::Sender<Result<i64, sqlx::Error>>,
    },
    AddMember {
        channel_id: i64,
        participant_id: i64,
        reply: oneshot::Sender<Result<(), sqlx::Error>>,
    },
    OpenSession {
        token: String,
        participant_id: i64,
        reply: oneshot::Sender<Result<i64, sqlx::Error>>,
    },
    ReconcileRoles {
        admins: Vec<String>,
        reply: oneshot::Sender<Result<(), sqlx::Error>>,
    },
}

#[derive(Clone)]
pub struct Store {
    pub pool: PgPool,
    write_tx: mpsc::Sender<WriteCmd>,
    events_tx: broadcast::Sender<ChannelEvent>,
}

impl Store {
    pub async fn connect(database_url: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(database_url)
            .await?;
        sqlx::migrate!("./migrations").run(&pool).await?;

        let (write_tx, write_rx) = mpsc::channel(256);
        let (events_tx, _) = broadcast::channel(1024);
        tokio::spawn(writer_task(pool.clone(), write_rx, events_tx.clone()));

        Ok(Self { pool, write_tx, events_tx })
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ChannelEvent> {
        self.events_tx.subscribe()
    }

    /// How many agent answers have landed in the channel since the last
    /// human message - the hello-loop counter's reading. Counts `close`
    /// events newer than the newest human-authored `message`, or all of
    /// them where no human has spoken.
    pub async fn agent_hops_since_human(&self, channel_id: i64) -> anyhow::Result<u32> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM channel_events e              WHERE e.channel_id = $1 AND e.kind = 'close'              AND e.id > COALESCE((                 SELECT MAX(m.id) FROM channel_events m                  JOIN participants p ON p.id = m.participant_id                  WHERE m.channel_id = $1 AND m.kind = 'message'                  AND p.kind = 'human'), 0)",
        )
        .bind(channel_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(count.max(0) as u32)
    }

    async fn send<T>(
        &self,
        make: impl FnOnce(oneshot::Sender<Result<T, sqlx::Error>>) -> WriteCmd,
    ) -> anyhow::Result<T> {
        let (tx, rx) = oneshot::channel();
        self.write_tx
            .send(make(tx))
            .await
            .map_err(|_| anyhow::anyhow!("store writer is gone"))?;
        Ok(rx.await.map_err(|_| anyhow::anyhow!("store writer dropped reply"))??)
    }

    pub async fn append(&self, event: NewEvent) -> anyhow::Result<ChannelEvent> {
        self.send(|reply| WriteCmd::Append { event, reply }).await
    }

    pub async fn create_participant(
        &self,
        name: &str,
        display: &str,
        kind: &str,
        adapter: Option<&str>,
    ) -> anyhow::Result<i64> {
        let (name, display, kind) = (name.to_owned(), display.to_owned(), kind.to_owned());
        let adapter = adapter.map(|s| s.to_owned());
        self.send(|reply| WriteCmd::CreateParticipant { name, display, kind, adapter, reply })
            .await
    }

    pub async fn create_channel(&self, name: &str, topic: Option<&str>) -> anyhow::Result<i64> {
        let (name, topic) = (name.to_owned(), topic.map(|s| s.to_owned()));
        self.send(|reply| WriteCmd::CreateChannel { name, topic, reply }).await
    }

    pub async fn add_member(&self, channel_id: i64, participant_id: i64) -> anyhow::Result<()> {
        self.send(|reply| WriteCmd::AddMember { channel_id, participant_id, reply })
            .await
    }

    pub async fn open_session(&self, token: &str, participant_id: i64) -> anyhow::Result<i64> {
        let token = token.to_owned();
        self.send(|reply| WriteCmd::OpenSession { token, participant_id, reply })
            .await
    }

    /// v1 role assignment: the config's admin list is authoritative for
    /// human participants at startup (Spec section 14).
    pub async fn reconcile_roles(&self, admins: Vec<String>) -> anyhow::Result<()> {
        self.send(|reply| WriteCmd::ReconcileRoles { admins, reply }).await
    }
}

async fn writer_task(
    pool: PgPool,
    mut rx: mpsc::Receiver<WriteCmd>,
    events_tx: broadcast::Sender<ChannelEvent>,
) {
    while let Some(cmd) = rx.recv().await {
        match cmd {
            WriteCmd::Append { event, reply } => {
                let res = sqlx::query_as::<_, ChannelEvent>(
                    "INSERT INTO channel_events \
                     (channel_id, participant_id, kind, body, run_label, turn_label, close_kind) \
                     VALUES ($1, $2, $3, $4, $5, $6, $7) \
                     RETURNING id, channel_id, ts, participant_id, kind, body, \
                               run_label, turn_label, close_kind",
                )
                .bind(event.channel_id)
                .bind(event.participant_id)
                .bind(&event.kind)
                .bind(&event.body)
                .bind(&event.run_label)
                .bind(&event.turn_label)
                .bind(&event.close_kind)
                .fetch_one(&pool)
                .await;
                if let Ok(ev) = &res {
                    let _ = events_tx.send(ev.clone());
                }
                let _ = reply.send(res);
            }
            WriteCmd::CreateParticipant { name, display, kind, adapter, reply } => {
                let res = sqlx::query_scalar::<_, i64>(
                    "INSERT INTO participants (name, display, kind, adapter) \
                     VALUES ($1, $2, $3, $4) \
                     ON CONFLICT (name) DO UPDATE SET display = EXCLUDED.display \
                     RETURNING id",
                )
                .bind(&name)
                .bind(&display)
                .bind(&kind)
                .bind(&adapter)
                .fetch_one(&pool)
                .await;
                let _ = reply.send(res);
            }
            WriteCmd::CreateChannel { name, topic, reply } => {
                let res = sqlx::query_scalar::<_, i64>(
                    "INSERT INTO channels (name, topic) VALUES ($1, $2) RETURNING id",
                )
                .bind(&name)
                .bind(&topic)
                .fetch_one(&pool)
                .await;
                let _ = reply.send(res);
            }
            WriteCmd::AddMember { channel_id, participant_id, reply } => {
                let res = sqlx::query(
                    "INSERT INTO members (channel_id, participant_id) VALUES ($1, $2) \
                     ON CONFLICT DO NOTHING",
                )
                .bind(channel_id)
                .bind(participant_id)
                .execute(&pool)
                .await
                .map(|_| ());
                let _ = reply.send(res);
            }
            WriteCmd::OpenSession { token, participant_id, reply } => {
                let res = sqlx::query_scalar::<_, i64>(
                    "INSERT INTO sessions (token, participant_id) VALUES ($1, $2) RETURNING id",
                )
                .bind(&token)
                .bind(participant_id)
                .fetch_one(&pool)
                .await;
                let _ = reply.send(res);
            }
            WriteCmd::ReconcileRoles { admins, reply } => {
                // One transaction: a failure between the demote and the
                // promote must not leave the roles half-reconciled.
                let res = async {
                    let mut tx = pool.begin().await?;
                    sqlx::query("UPDATE participants SET role = 'user' WHERE role = 'admin' AND name <> ALL($1)")
                        .bind(&admins)
                        .execute(&mut *tx)
                        .await?;
                    sqlx::query("UPDATE participants SET role = 'admin' WHERE name = ANY($1)")
                        .bind(&admins)
                        .execute(&mut *tx)
                        .await?;
                    tx.commit().await?;
                    Ok(())
                }
                .await;
                let _ = reply.send(res);
            }
        }
    }
}
