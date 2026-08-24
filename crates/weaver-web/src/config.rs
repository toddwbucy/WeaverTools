use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub listen: String,
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
    /// Directory of agent declarations, shown read-only on the admin
    /// surface. The files are the operator's own deployment config.
    #[serde(default = "default_agent_declarations")]
    pub agent_declarations: PathBuf,
    #[serde(default)]
    pub agents: Vec<AgentConfig>,
    #[serde(default)]
    pub providers: Vec<ProviderConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub gate: PathBuf,
    pub trace: PathBuf,
}

// Read by the upstream adapter once it is implemented (Spec section 10).
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub api: String,
    pub model: String,
    pub key_env: String,
}

fn default_agent_declarations() -> PathBuf {
    PathBuf::from("/etc/weaver/agents")
}

impl Config {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("reading config {}: {e}", path.display()))?;
        let cfg: Config = toml::from_str(&raw)?;
        Ok(cfg)
    }

    pub fn agent(&self, name: &str) -> Option<&AgentConfig> {
        self.agents.iter().find(|a| a.name == name)
    }
}

fn default_agent_hop_budget() -> u32 {
    8
}
