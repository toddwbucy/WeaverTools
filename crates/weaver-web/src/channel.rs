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

const EVENT_VIEW_COLUMNS: &str = "e.id, e.channel_id, e.ts, e.kind, e.body, e.run_label, e.turn_label, e.close_kind, \
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

/// The window's row cap (review of #350): an agent that has never
/// closed in a channel floors at zero and would otherwise drag the
/// channel's whole history through the trim. The cap is more lines
/// than the 32 KiB line bound can carry at any plausible line size,
/// so it never costs a line the trim would have kept.
pub const WINDOW_CAP: i64 = 500;

/// The prompt window of Spec section 7: what was said since this
/// participant's last close - messages and closes alike, because an
/// agent's answer is conversation the next speaker must see. Selecting
/// only messages made agents blind to each other in a shared room. The
/// window's start already excludes this participant's own closes, and
/// the newest [`WINDOW_CAP`] rows bound the read.
pub async fn messages_since_last_close(
    store: &Store,
    channel_id: i64,
    participant_id: i64,
) -> anyhow::Result<Vec<EventView>> {
    // Two arms unioned so the cap can never evict the newest message
    // (review of #350, round three): a message followed by more than
    // a cap's worth of closes would otherwise fall out of the window,
    // and the no-message guard would then drop a justified turn as
    // stale. The first arm holds that message, the second the newest
    // rows of any kind, and UNION folds the overlap.
    let rest = WINDOW_CAP - 1;
    Ok(sqlx::query_as::<_, EventView>(sqlx::AssertSqlSafe(format!(
        "SELECT * FROM ( \
           (SELECT {EVENT_VIEW_COLUMNS} FROM channel_events e \
            LEFT JOIN participants p ON p.id = e.participant_id \
            WHERE e.channel_id = $1 AND e.kind = 'message' \
            AND e.id > COALESCE(( \
                SELECT max(id) FROM channel_events \
                WHERE channel_id = $1 AND participant_id = $2 AND kind = 'close'), 0) \
            ORDER BY e.id DESC LIMIT 1) \
           UNION \
           (SELECT {EVENT_VIEW_COLUMNS} FROM channel_events e \
            LEFT JOIN participants p ON p.id = e.participant_id \
            WHERE e.channel_id = $1 AND e.kind IN ('message', 'close') \
            AND e.id > COALESCE(( \
                SELECT max(id) FROM channel_events \
                WHERE channel_id = $1 AND participant_id = $2 AND kind = 'close'), 0) \
            ORDER BY e.id DESC LIMIT {rest}) \
         ) newest ORDER BY id"
    )))
    .bind(channel_id)
    .bind(participant_id)
    .fetch_all(&store.pool)
    .await?)
}
