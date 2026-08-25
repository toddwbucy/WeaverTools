//! Participant reads, and reconciliation of announced agents and
//! configured providers into the registry. Agents arrive with the
//! link's hello (roster-by-hello, Spec section 16), providers with
//! the server's own config at startup.

use crate::config::ProviderConfig;
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
    // A static string needs no safety assertion - the one site of the
    // seven that interpolated nothing now interpolates nothing visibly.
    Ok(sqlx::query_as::<_, Participant>(
        "SELECT p.id, p.name, p.display, p.kind, p.adapter, p.respond, p.role \
         FROM participants p JOIN members m ON m.participant_id = p.id \
         WHERE m.channel_id = $1 ORDER BY p.name",
    )
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

/// Ensure every agent a hello announced has a participant row.
/// Humans are created on first visit, not here.
pub async fn reconcile_agents(store: &Store, names: &[String]) -> anyhow::Result<()> {
    for name in names {
        store
            .create_participant(name, name, "agent", Some(name))
            .await?;
    }
    Ok(())
}

/// Ensure every configured provider has a participant row.
pub async fn reconcile_providers(
    store: &Store,
    providers: &[ProviderConfig],
) -> anyhow::Result<()> {
    for p in providers {
        store
            .create_participant(&p.name, &p.name, "model", Some(&p.name))
            .await?;
    }
    Ok(())
}
