//! The HTTP surface (Spec section 13). Two sub-surfaces mirror the
//! framework's two external boundaries and the two roles the PRD
//! names: `user` is the gate surface (channels, messages), `admin` is
//! the operator surface (lifecycle verbs, trace views), gated on the
//! participant's role. The browser is a display engine (PRD 3).

pub mod admin;
pub mod user;

use crate::adapters::gate::GateAdapter;
use crate::channel::EventView;
use crate::config::Config;
use crate::queue::Queues;
use crate::registry::{self, Participant};
use crate::store::Store;
use crate::traceview::TraceViews;
use askama::Template;
use axum::extract::Path;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub cfg: Arc<Config>,
    pub store: Store,
    pub queues: Queues,
    pub traces: TraceViews,
    pub gates: Arc<HashMap<String, GateAdapter>>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(user::routes())
        .nest("/admin", admin::routes())
        .route("/assets/{file}", get(asset))
        .with_state(state)
}

// ---------- errors ----------

pub struct AppError(pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // The full chain goes to the log; the response stays generic so
        // SQL, filesystem, and upstream detail never reach a browser.
        tracing::error!("request failed: {:#}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, "internal error; see the server log").into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(e: E) -> Self {
        AppError(e.into())
    }
}

pub type AppResult<T> = Result<T, AppError>;

// ---------- sessions ----------

fn session_token(headers: &HeaderMap) -> Option<String> {
    let cookies = headers.get(header::COOKIE)?.to_str().ok()?;
    cookies.split(';').find_map(|c| {
        let (k, v) = c.trim().split_once('=')?;
        (k == "ww_session").then(|| v.to_string())
    })
}

pub async fn session_participant(
    state: &AppState,
    headers: &HeaderMap,
) -> anyhow::Result<Option<Participant>> {
    let Some(token) = session_token(headers) else { return Ok(None) };
    let pid: Option<i64> = sqlx::query_scalar(
        "SELECT participant_id FROM sessions WHERE token = $1 AND closed_at IS NULL",
    )
    .bind(&token)
    .fetch_optional(&state.store.pool)
    .await?
    .flatten();
    match pid {
        Some(id) => registry::by_id(&state.store, id).await,
        None => Ok(None),
    }
}

// ---------- shared view helpers ----------

pub fn nav_agents(cfg: &Config) -> Vec<String> {
    cfg.agents.iter().map(|a| a.name.clone()).collect()
}

pub fn valid_handle(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

pub fn sse_cursor(headers: &HeaderMap, params: &HashMap<String, String>) -> i64 {
    // Last-Event-ID (reconnect) takes precedence over ?after= (initial).
    headers
        .get("last-event-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
        .or_else(|| params.get("after").and_then(|v| v.parse().ok()))
        .unwrap_or(0)
}

/// Markdown to HTML, server-side, with raw HTML in the source escaped
/// rather than passed through, and link/image destinations restricted
/// to safe schemes - the browser renders, never interprets
/// participant-authored markup, and never receives a javascript: or
/// data: destination.
fn markdown_to_html(src: &str) -> String {
    use pulldown_cmark::{html, Event, Options, Parser, Tag};

    fn safe_dest(dest: &str) -> bool {
        let d = dest.trim().to_ascii_lowercase();
        d.starts_with("http://")
            || d.starts_with("https://")
            || d.starts_with("mailto:")
            || d.starts_with('/')
            || d.starts_with('#')
    }

    let opts = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH;
    let parser = Parser::new_ext(src, opts).map(|ev| match ev {
        Event::Html(t) => Event::Text(t),
        Event::InlineHtml(t) => Event::Text(t),
        Event::Start(Tag::Link { link_type, dest_url, title, id }) => {
            let dest_url = if safe_dest(&dest_url) { dest_url } else { "#".into() };
            Event::Start(Tag::Link { link_type, dest_url, title, id })
        }
        Event::Start(Tag::Image { link_type, dest_url, title, id }) => {
            let dest_url = if safe_dest(&dest_url) { dest_url } else { "#".into() };
            Event::Start(Tag::Image { link_type, dest_url, title, id })
        }
        other => other,
    });
    let mut out = String::with_capacity(src.len() * 2);
    html::push_html(&mut out, parser);
    out
}

fn escape_html(src: &str) -> String {
    src.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// A message or close body, rendered: a leading <think> block folds
/// into a collapsible section, the remainder renders as markdown.
fn render_body(body: &str) -> String {
    let trimmed = body.trim_start();
    if let Some(rest) = trimmed.strip_prefix("<think>") {
        if let Some(end) = rest.find("</think>") {
            let thought = &rest[..end];
            let answer = &rest[end + "</think>".len()..];
            return format!(
                "<details class=\"think\"><summary>thinking</summary><pre>{}</pre></details>{}",
                escape_html(thought.trim()),
                markdown_to_html(answer.trim())
            );
        }
    }
    markdown_to_html(body)
}

#[derive(Template)]
#[template(path = "event.html")]
struct EventFragment {
    id: i64,
    ts: String,
    author: String,
    author_kind: String,
    kind: String,
    body: String,
    body_html: String,
    close_kind: String,
    turn_label: String,
    turn_link: String,
}

pub fn render_event(ev: &EventView) -> String {
    // The turn link points into the admin surface; the record is the
    // operator's, so a non-admin following it meets the role gate.
    let turn_link = match (&ev.run_label, &ev.turn_label, &ev.author_name) {
        (Some(_), Some(_), Some(agent)) => format!("/admin/trace/{agent}"),
        _ => String::new(),
    };
    let body = ev.body.clone().unwrap_or_default();
    let body_html = match ev.kind.as_str() {
        "message" | "close" => render_body(&body),
        _ => String::new(),
    };
    EventFragment {
        id: ev.id,
        ts: ev.ts.format("%H:%M:%S").to_string(),
        author: ev
            .author_display
            .clone()
            .or_else(|| ev.author_name.clone())
            .unwrap_or_else(|| "system".into()),
        author_kind: ev.author_kind.clone().unwrap_or_else(|| "system".into()),
        kind: ev.kind.clone(),
        body,
        body_html,
        close_kind: ev.close_kind.clone().unwrap_or_default(),
        turn_label: ev.turn_label.clone().unwrap_or_default(),
        turn_link,
    }
    .render()
    .unwrap_or_else(|e| format!("<div class=\"event\">render error: {e}</div>"))
}

// ---------- assets ----------

async fn asset(Path(file): Path<String>) -> Response {
    let (bytes, ctype): (&'static [u8], &'static str) = match file.as_str() {
        "htmx.min.js" => (include_bytes!("../../assets/htmx.min.js"), "text/javascript"),
        "sse.js" => (include_bytes!("../../assets/sse.js"), "text/javascript"),
        "style.css" => (include_bytes!("../../assets/style.css"), "text/css"),
        _ => return (StatusCode::NOT_FOUND, "no such asset").into_response(),
    };
    ([(header::CONTENT_TYPE, ctype)], bytes).into_response()
}
