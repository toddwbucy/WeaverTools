//! The connector: runs on the agents' own box as the operator's uid
//! and holds every box-bound reach - gate sockets, verb invocation,
//! trace sinks, the load-state observable, the declaration read. It
//! renders nothing, stores nothing, and only dials out (PRD section
//! 3): the server's address is the one line that changes when the
//! presentation stack moves.

use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use weaver_web::config::ConnectorConfig;
use weaver_web::wire;

#[derive(Parser)]
#[command(
    name = "weaver-web-connector",
    about = "The box-side half of weaver-web: the link's connector."
)]
struct Args {
    /// Path to the connector's TOML configuration file.
    #[arg(long, default_value = "/etc/weaver-web/connector.toml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "weaver_web=info".into()),
        )
        .init();

    let args = Args::parse();
    let cfg = Arc::new(ConnectorConfig::load(&args.config)?);
    tracing::info!(
        "config loaded: {} agent(s), dialing {}",
        cfg.agents.len(),
        cfg.server
    );

    tokio::select! {
        _ = wire::connector_run(cfg) => {}
        _ = shutdown_signal() => {}
    }
    Ok(())
}

async fn shutdown_signal() {
    use tokio::signal::unix::{signal, SignalKind};
    let mut term = signal(SignalKind::terminate()).expect("SIGTERM handler");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = term.recv() => {}
    }
    tracing::info!("shutdown signal received, stopping");
}
