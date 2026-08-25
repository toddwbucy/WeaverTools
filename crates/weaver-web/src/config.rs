//! TOML configuration, one file per process (Spec section 3). Box
//! facts live in the box's config: the agent roster with its socket
//! and sink paths is the connector's declaration, announced to the
//! server in the link's hello, never entered twice.

use serde::Deserialize;
use std::path::{Path, PathBuf};

/// The server's config, default `/etc/weaver-web/config.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub listen: String,
    /// Where the server listens for the connector's dial. Loopback by
    /// default, so any exposure is the operator's explicit widening
    /// (Spec section 16).
    #[serde(default = "default_link_listen")]
    pub link_listen: String,
    pub database: String,
    /// Participant names holding the admin role. v1 role assignment is
    /// the operator's declaration; IAM later changes how a session
    /// proves it is a participant, not where roles live.
    #[serde(default)]
    pub admins: Vec<String>,
    /// How many consecutive agent-to-agent hops the router serves after
    /// the last human message in a channel, before it pauses the volley
    /// visibly. The hello-loop counter, added 2026-08-20 after the first
    /// open volley greeted itself in circles: coordination stays open,
    /// and a human word resets the budget.
    #[serde(default = "default_agent_hop_budget")]
    pub agent_hop_budget: u32,
    #[serde(default)]
    pub providers: Vec<ProviderConfig>,
}

/// The connector's config, default `/etc/weaver-web/connector.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectorConfig {
    /// The server's link address, the one line that changes when the
    /// presentation stack moves to another box (PRD section 3).
    #[serde(default = "default_server")]
    pub server: String,
    /// Directory of agent declarations, served read-only to the admin
    /// surface over the link. The files are the operator's own
    /// deployment config.
    #[serde(default = "default_agent_declarations")]
    pub agent_declarations: PathBuf,
    #[serde(default)]
    pub agents: Vec<AgentConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub gate: PathBuf,
    pub trace: PathBuf,
}

// Read by the upstream adapter once it is implemented (Spec section
// 10). Providers are the server's business alone: nothing
// upstream-facing touches the agents' box.
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub api: String,
    pub model: String,
    pub key_env: String,
}

fn default_link_listen() -> String {
    "127.0.0.1:8081".into()
}

fn default_server() -> String {
    "127.0.0.1:8081".into()
}

fn default_agent_declarations() -> PathBuf {
    PathBuf::from("/etc/weaver/agents")
}

fn default_agent_hop_budget() -> u32 {
    8
}

fn load_toml<T: serde::de::DeserializeOwned>(path: &Path) -> anyhow::Result<T> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("reading config {}: {e}", path.display()))?;
    Ok(toml::from_str(&raw)?)
}

impl ServerConfig {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        load_toml(path)
    }
}

impl ConnectorConfig {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        load_toml(path)
    }
}
