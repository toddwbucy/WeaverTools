//! The upstream model adapter (Spec section 10). Not implemented in
//! the first scaffold: the adapter seam exists so the router can name
//! it, and the first provider (Anthropic Messages) lands as its own
//! act.

use crate::config::ProviderConfig;

// The seam is declared ahead of its first implementation on purpose
// (Spec sections 10 and 15); the router names model participants today
// and ignores their mentions until this adapter is real.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UpstreamAdapter {
    pub provider: ProviderConfig,
}

#[allow(dead_code)]
impl UpstreamAdapter {
    pub fn new(provider: ProviderConfig) -> Self {
        Self { provider }
    }

    pub async fn turn(&self, _context: &str) -> anyhow::Result<String> {
        anyhow::bail!(
            "upstream provider '{}' is configured but the adapter is not yet implemented",
            self.provider.name
        )
    }
}
