//! Channel reads: lists, pages of the log, and the projection types
//! the templates render.

use crate::store::Store;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Channel {
    pub id: i64,
    pub name: String,
    pub topic: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// A channel event joined with its author's names, ready to render.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct EventView {
    pub id: i64,
    pub channel_id: i64,
    pub ts: DateTime<Utc>,
    pub kind: String,
    pub body: Option<String>,
    pub run_label: Option<String>,
    pub turn_label: Option<String>,
    pub close_kind: Option<String>,
    pub author_name: Option<String>,
    pub author_display: Option<String>,
    pub author_kind: Option<String>,
}

pub async fn list(store: &Store) -> anyhow::Result<Vec<Channel>> {
    Ok(sqlx::query_as::<_, Channel>(
        "SELECT id, name, topic, created_at FROM channels ORDER BY name",
    )
    .fetch_all(&store.pool)
    .await?)
}

pub async fn by_name(store: &Store, name: &str) -> anyhow::Result<Option<Channel>> {
    Ok(sqlx::query_as::<_, Channel>(
        "SELECT id, name, topic, created_at FROM channels WHERE name = $1",
    )
    .bind(name)
    .fetch_optional(&store.pool)
    .await?)
}

const EVENT_VIEW_COLUMNS: &str =
    "e.id, e.channel_id, e.ts, e.kind, e.body, e.run_label, e.turn_label, e.close_kind, \
     p.name AS author_name, p.display AS author_display, p.kind AS author_kind";

pub async fn events_after(
    store: &Store,
    channel_id: i64,
    after_id: i64,
    limit: i64,
) -> anyhow::Result<Vec<EventView>> {
    Ok(sqlx::query_as::<_, EventView>(sqlx::AssertSqlSafe(format!(
        "SELECT {EVENT_VIEW_COLUMNS} FROM channel_events e \
         LEFT JOIN participants p ON p.id = e.participant_id \
         WHERE e.channel_id = $1 AND e.id > $2 ORDER BY e.id LIMIT $3"
    )))
    .bind(channel_id)
    .bind(after_id)
    .bind(limit)
    .fetch_all(&store.pool)
    .await?)
}

pub async fn event_view(store: &Store, event_id: i64) -> anyhow::Result<Option<EventView>> {
    Ok(sqlx::query_as::<_, EventView>(sqlx::AssertSqlSafe(format!(
        "SELECT {EVENT_VIEW_COLUMNS} FROM channel_events e \
         LEFT JOIN participants p ON p.id = e.participant_id \
         WHERE e.id = $1"
    )))
    .bind(event_id)
    .fetch_optional(&store.pool)
    .await?)
}

/// Messages since a participant's last close in a channel - the prompt
/// window of Spec section 7.
pub async fn messages_since_last_close(
    store: &Store,
    channel_id: i64,
    participant_id: i64,
) -> anyhow::Result<Vec<EventView>> {
    Ok(sqlx::query_as::<_, EventView>(sqlx::AssertSqlSafe(format!(
        "SELECT {EVENT_VIEW_COLUMNS} FROM channel_events e \
         LEFT JOIN participants p ON p.id = e.participant_id \
         WHERE e.channel_id = $1 AND e.kind = 'message' AND e.id > COALESCE(( \
             SELECT max(id) FROM channel_events \
             WHERE channel_id = $1 AND participant_id = $2 AND kind = 'close'), 0) \
         ORDER BY e.id"
    )))
    .bind(channel_id)
    .bind(participant_id)
    .fetch_all(&store.pool)
    .await?)
}
