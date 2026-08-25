//! Participant reads, and startup reconciliation of configured
//! agents and providers into the registry.

use crate::config::Config;
use crate::store::Store;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Participant {
    pub id: i64,
    pub name: String,
    pub display: String,
    pub kind: String,
    pub adapter: Option<String>,
    pub respond: String,
    /// 'user' or 'admin'. The admin surface (operator boundary) is
    /// gated on it; v1 assignment is the config's admin list.
    pub role: String,
}

impl Participant {
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
}

const COLUMNS: &str = "id, name, display, kind, adapter, respond, role";

pub async fn by_name(store: &Store, name: &str) -> anyhow::Result<Option<Participant>> {
    Ok(sqlx::query_as::<_, Participant>(sqlx::AssertSqlSafe(format!(
        "SELECT {COLUMNS} FROM participants WHERE name = $1"
    )))
    .bind(name)
    .fetch_optional(&store.pool)
    .await?)
}

pub async fn by_id(store: &Store, id: i64) -> anyhow::Result<Option<Participant>> {
    Ok(sqlx::query_as::<_, Participant>(sqlx::AssertSqlSafe(format!(
        "SELECT {COLUMNS} FROM participants WHERE id = $1"
    )))
    .bind(id)
    .fetch_optional(&store.pool)
    .await?)
}

pub async fn channel_members(store: &Store, channel_id: i64) -> anyhow::Result<Vec<Participant>> {
    Ok(sqlx::query_as::<_, Participant>(sqlx::AssertSqlSafe(format!(
        "SELECT p.id, p.name, p.display, p.kind, p.adapter, p.respond, p.role \
         FROM participants p JOIN members m ON m.participant_id = p.id \
         WHERE m.channel_id = $1 ORDER BY p.name"
    )))
    .bind(channel_id)
    .fetch_all(&store.pool)
    .await?)
}

pub async fn all(store: &Store) -> anyhow::Result<Vec<Participant>> {
    Ok(sqlx::query_as::<_, Participant>(sqlx::AssertSqlSafe(format!(
        "SELECT {COLUMNS} FROM participants ORDER BY kind, name"
    )))
    .fetch_all(&store.pool)
    .await?)
}

/// Ensure every configured agent and provider has a participant row.
/// Humans are created on first visit, not here.
pub async fn reconcile(store: &Store, cfg: &Config) -> anyhow::Result<()> {
    for a in &cfg.agents {
        store
            .create_participant(&a.name, &a.name, "agent", Some(&a.name))
            .await?;
    }
    for p in &cfg.providers {
        store
            .create_participant(&p.name, &p.name, "model", Some(&p.name))
            .await?;
    }
    store.reconcile_roles(cfg.admins.clone()).await?;
    Ok(())
}
