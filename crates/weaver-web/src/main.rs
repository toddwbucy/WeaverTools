mod adapters;
mod channel;
mod config;
mod lifecycle;
mod queue;
mod registry;
mod router;
mod store;
mod traceview;
mod web;

use adapters::gate::GateAdapter;
use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "weaver-web", about = "The WeaverTools suite's frontend.")]
struct Args {
    /// Path to the TOML configuration file.
    #[arg(long, default_value = "/etc/weaver-web/config.toml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "weaver_web=info,sqlx=warn".into()),
        )
        .init();

    let args = Args::parse();
    let cfg = Arc::new(config::Config::load(&args.config)?);
    tracing::info!(
        "config loaded: {} agent(s), {} provider(s)",
        cfg.agents.len(),
        cfg.providers.len()
    );

    let store = store::Store::connect(&cfg.database).await?;
    registry::reconcile(&store, &cfg).await?;
    tracing::info!("store connected, migrations applied, registry reconciled");

    let gates: HashMap<String, GateAdapter> = cfg
        .agents
        .iter()
        .map(|a| (a.name.clone(), GateAdapter::new(&a.gate)))
        .collect();

    let queues = queue::Queues::start(
        store.clone(),
        gates.iter().map(|(n, g)| (n.clone(), g.clone())).collect(),
        cfg.agent_hop_budget,
    );

    let traces = traceview::TraceViews::start(
        cfg.agents.iter().map(|a| (a.name.clone(), a.trace.clone())).collect(),
    );

    let state = web::AppState {
        cfg: cfg.clone(),
        store,
        queues,
        traces,
        gates: Arc::new(gates),
    };

    let listener = tokio::net::TcpListener::bind(&cfg.listen).await?;
    tracing::info!("listening on {}", cfg.listen);
    axum::serve(listener, web::router(state))
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

/// SIGTERM or ctrl-c stops accepting requests and lets in-flight ones
/// finish. Draining agent queues so every turn-open gets a close is a
/// named follow-up; a turn cut by shutdown lands as the gate adapter's
/// delivery-lost error today.
async fn shutdown_signal() {
    use tokio::signal::unix::{signal, SignalKind};
    let mut term = signal(SignalKind::terminate()).expect("SIGTERM handler");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = term.recv() => {}
    }
    tracing::info!("shutdown signal received, stopping");
}
