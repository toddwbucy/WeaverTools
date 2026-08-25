//! The server: HTTP for browsers, the store, and the link listener
//! the connector dials (PRD section 3). Holds everything that is not
//! box-bound and reaches the box only through the link.

use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use weaver_web::config::ServerConfig;
use weaver_web::traceview::TraceViews;
use weaver_web::{queue, registry, store, web, wire};

#[derive(Parser)]
#[command(name = "weaver-web", about = "The WeaverTools suite's frontend server.")]
struct Args {
    /// Path to the server's TOML configuration file.
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
    let cfg = Arc::new(ServerConfig::load(&args.config)?);
    tracing::info!("config loaded: {} provider(s)", cfg.providers.len());

    let store = store::Store::connect(&cfg.database).await?;
    registry::reconcile_providers(&store, &cfg.providers).await?;
    store.reconcile_roles(cfg.admins.clone()).await?;
    tracing::info!("store connected, migrations applied, providers and roles reconciled");

    let link = wire::Link::new();
    let traces = TraceViews::new();
    let queues = queue::Queues::new(store.clone(), link.clone(), cfg.agent_hop_budget);

    let link_listener = tokio::net::TcpListener::bind(&cfg.link_listen).await?;
    tracing::info!("link listening on {}", cfg.link_listen);
    let (ev_tx, mut ev_rx) = tokio::sync::mpsc::channel(1024);
    tokio::spawn(wire::serve(link.clone(), link_listener, ev_tx));

    // The link event pump: hellos reconcile the registry and start
    // queues and views (roster-by-hello, Spec section 16), trace
    // frames feed the rings, and link loss marks every view.
    {
        let (store, queues, traces) = (store.clone(), queues.clone(), traces.clone());
        tokio::spawn(async move {
            let mut had_connection = false;
            while let Some(ev) = ev_rx.recv().await {
                match ev {
                    wire::LinkEvent::Hello(agents) => {
                        if let Err(e) = registry::reconcile_agents(&store, &agents).await {
                            tracing::error!("agent reconciliation failed: {e}");
                        }
                        for a in &agents {
                            queues.ensure_agent(a);
                            traces.ensure(a);
                        }
                        if had_connection {
                            traces.mark_all(
                                "link reconnected: a fresh backfill follows",
                            );
                        }
                        had_connection = true;
                        tracing::info!("hello: roster of {} agent(s)", agents.len());
                    }
                    wire::LinkEvent::Trace { agent, event } => {
                        traces.ingest(&agent, event);
                    }
                    wire::LinkEvent::Down => {
                        traces.mark_all("link to the agents' box lost");
                    }
                }
            }
        });
    }

    let state = web::AppState {
        cfg: cfg.clone(),
        store,
        queues,
        traces,
        link,
        repro: Default::default(),
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
/// named follow-up; a turn cut by shutdown lands as the link's
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
