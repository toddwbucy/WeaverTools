//! The user surface: everything that crosses the gate boundary.
//! Sessions, channels, messages, and the channel SSE stream. Nothing
//! here invokes a verb or reads a trace - those are the admin
//! surface's, behind its role gate.

use super::{
    nav_agents, render_event, session_participant, sse_cursor, valid_handle, AppResult, AppState,
};
use crate::channel::{self, EventView};
use crate::registry;
use crate::router as invocation_router;
use crate::store::NewEvent;
use askama::Template;
use axum::extract::{Form, Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::sse::{Event as SseEvent, KeepAlive, Sse};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::Infallible;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/session", post(open_session))
        .route("/channels", get(channels_page).post(create_channel))
        .route("/channels/{name}", get(channel_page))
        .route("/channels/{name}/messages", post(post_message))
        .route("/channels/{name}/members", post(add_member))
        .route("/channels/{name}/stream", get(channel_stream))
}

#[derive(Template)]
#[template(path = "name.html")]
struct NamePage {
    nav_agents: Vec<String>,
    who: String,
    is_admin: bool,
}

async fn index(State(state): State<AppState>, headers: HeaderMap) -> AppResult<Response> {
    if session_participant(&state, &headers).await?.is_some() {
        return Ok(Redirect::to("/channels").into_response());
    }
    let page = NamePage {
        nav_agents: nav_agents(&state).await,
        who: "anonymous".into(),
        is_admin: false,
    };
    Ok(Html(page.render()?).into_response())
}

#[derive(Deserialize)]
struct SessionForm {
    name: String,
}

async fn open_session(
    State(state): State<AppState>,
    Form(form): Form<SessionForm>,
) -> AppResult<Response> {
    if !valid_handle(&form.name) {
        return Ok((StatusCode::BAD_REQUEST, "name must be kebab-case").into_response());
    }
    if let Some(existing) = registry::by_name(&state.store, &form.name).await? {
        if existing.kind != "human" {
            return Ok((
                StatusCode::CONFLICT,
                format!("'{}' is a {} participant", form.name, existing.kind),
            )
                .into_response());
        }
    }
    let pid = state
        .store
        .create_participant(&form.name, &form.name, "human", None)
        .await?;
    // v1: roles come from the config's admin list, reapplied so a
    // first-time admin name lands with its role (Spec section 14).
    state.store.reconcile_roles(state.cfg.admins.clone()).await?;
    let token = uuid::Uuid::new_v4().to_string();
    state.store.open_session(&token, pid).await?;
    // Strict blocks any cross-site request from carrying the session,
    // which is the CSRF defense for every mutating route. The Secure
    // attribute deliberately waits for the TLS act (PRD roadmap 2):
    // v1 is plain HTTP on the LAN, and Secure would break the cookie.
    let cookie = format!("ww_session={token}; Path=/; HttpOnly; SameSite=Strict");
    Ok(([(header::SET_COOKIE, cookie)], Redirect::to("/channels")).into_response())
}

#[derive(Template)]
#[template(path = "channels.html")]
struct ChannelsPage {
    nav_agents: Vec<String>,
    who: String,
    is_admin: bool,
    channels: Vec<crate::channel::Channel>,
}

async fn channels_page(State(state): State<AppState>, headers: HeaderMap) -> AppResult<Response> {
    let Some(me) = session_participant(&state, &headers).await? else {
        return Ok(Redirect::to("/").into_response());
    };
    let page = ChannelsPage {
        nav_agents: nav_agents(&state).await,
        is_admin: me.is_admin(),
        who: me.name,
        channels: channel::list(&state.store).await?,
    };
    Ok(Html(page.render()?).into_response())
}

async fn join_with_note(
    state: &AppState,
    channel_id: i64,
    p: &crate::registry::Participant,
) -> anyhow::Result<()> {
    state.store.add_member(channel_id, p.id).await?;
    state
        .store
        .append(NewEvent {
            channel_id,
            participant_id: Some(p.id),
            kind: "member-change".into(),
            body: Some(format!("{} joined", p.name)),
            run_label: None,
            turn_label: None,
            close_kind: None,
        })
        .await?;
    Ok(())
}

// The create form carries dynamic `agent_<name>` checkboxes, so it
// deserializes as a map rather than a fixed struct.
async fn create_channel(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<HashMap<String, String>>,
) -> AppResult<Response> {
    let Some(me) = session_participant(&state, &headers).await? else {
        return Ok(Redirect::to("/").into_response());
    };
    let name = form.get("name").cloned().unwrap_or_default();
    if !valid_handle(&name) {
        return Ok((StatusCode::BAD_REQUEST, "name must be kebab-case").into_response());
    }
    let topic = form.get("topic").filter(|t| !t.is_empty()).map(|s| s.as_str());
    let cid = state.store.create_channel(&name, topic).await?;
    state.store.add_member(cid, me.id).await?;
    for key in form.keys() {
        if let Some(agent) = key.strip_prefix("agent_") {
            if let Some(p) = registry::by_name(&state.store, agent).await? {
                if p.kind == "agent" {
                    join_with_note(&state, cid, &p).await?;
                }
            }
        }
    }
    Ok(Redirect::to(&format!("/channels/{name}")).into_response())
}

struct MemberRow {
    name: String,
    kind: String,
    status: String,
}

struct AddableRow {
    name: String,
    kind: String,
}

#[derive(Template)]
#[template(path = "channel.html")]
struct ChannelPage {
    nav_agents: Vec<String>,
    who: String,
    is_admin: bool,
    channel_name: String,
    events: Vec<String>,
    members: Vec<MemberRow>,
    addable: Vec<AddableRow>,
    cursor: i64,
}

async fn channel_page(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(name): Path<String>,
) -> AppResult<Response> {
    let Some(me) = session_participant(&state, &headers).await? else {
        return Ok(Redirect::to("/").into_response());
    };
    let Some(ch) = channel::by_name(&state.store, &name).await? else {
        return Ok((StatusCode::NOT_FOUND, "no such channel").into_response());
    };
    let events = channel::events_after(&state.store, ch.id, 0, 500).await?;
    let cursor = events.last().map(|e| e.id).unwrap_or(0);
    let member_list = registry::channel_members(&state.store, ch.id).await?;
    let addable: Vec<AddableRow> = registry::all(&state.store)
        .await?
        .into_iter()
        .filter(|p| !member_list.iter().any(|m| m.id == p.id))
        .map(|p| AddableRow { name: p.name, kind: p.kind })
        .collect();
    let members = member_list
        .into_iter()
        .map(|m| {
            let status = if m.kind == "agent" {
                match state.queues.state(&m.name) {
                    Some(s) if s.in_flight.is_some() => "answering".to_string(),
                    Some(s) if s.depth > 0 => format!("{} queued", s.depth),
                    _ => String::new(),
                }
            } else {
                String::new()
            };
            MemberRow { name: m.name, kind: m.kind, status }
        })
        .collect();
    let page = ChannelPage {
        nav_agents: nav_agents(&state).await,
        is_admin: me.is_admin(),
        who: me.name,
        channel_name: ch.name,
        events: events.iter().map(render_event).collect(),
        members,
        addable,
        cursor,
    };
    Ok(Html(page.render()?).into_response())
}

#[derive(Deserialize)]
struct MessageForm {
    text: String,
}

async fn post_message(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(name): Path<String>,
    Form(form): Form<MessageForm>,
) -> AppResult<Response> {
    let Some(me) = session_participant(&state, &headers).await? else {
        return Ok(Redirect::to("/").into_response());
    };
    let Some(ch) = channel::by_name(&state.store, &name).await? else {
        return Ok((StatusCode::NOT_FOUND, "no such channel").into_response());
    };
    state.store.add_member(ch.id, me.id).await?;
    let event = state
        .store
        .append(NewEvent::message(ch.id, me.id, form.text))
        .await?;
    invocation_router::on_human_message(&state.store, &state.queues, &event).await?;

    if headers.contains_key("hx-request") {
        Ok(StatusCode::NO_CONTENT.into_response())
    } else {
        Ok(Redirect::to(&format!("/channels/{name}")).into_response())
    }
}

#[derive(Deserialize)]
struct MemberForm {
    name: String,
}

async fn add_member(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(name): Path<String>,
    Form(form): Form<MemberForm>,
) -> AppResult<Response> {
    let Some(_me) = session_participant(&state, &headers).await? else {
        return Ok(Redirect::to("/").into_response());
    };
    let Some(ch) = channel::by_name(&state.store, &name).await? else {
        return Ok((StatusCode::NOT_FOUND, "no such channel").into_response());
    };
    let Some(p) = registry::by_name(&state.store, &form.name).await? else {
        return Ok((StatusCode::NOT_FOUND, "no such participant").into_response());
    };
    join_with_note(&state, ch.id, &p).await?;
    Ok(Redirect::to(&format!("/channels/{name}")).into_response())
}

async fn channel_stream(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(name): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> AppResult<Response> {
    // Same session gate as the pages: no valid session, no stream.
    if session_participant(&state, &headers).await?.is_none() {
        return Ok((StatusCode::UNAUTHORIZED, "no session").into_response());
    }
    let Some(ch) = channel::by_name(&state.store, &name).await? else {
        return Ok((StatusCode::NOT_FOUND, "no such channel").into_response());
    };
    let want_json = headers
        .get(header::ACCEPT)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("application/json"))
        .unwrap_or(false);
    let mut cursor = sse_cursor(&headers, &params);

    let (tx, rx) = tokio::sync::mpsc::channel::<SseEvent>(64);
    let store = state.store.clone();
    let channel_id = ch.id;
    tokio::spawn(async move {
        let mut live = store.subscribe();
        match channel::events_after(&store, channel_id, cursor, 10_000).await {
            Ok(backlog) => {
                for ev in backlog {
                    cursor = ev.id;
                    if tx.send(make_channel_sse(&ev, want_json)).await.is_err() {
                        return;
                    }
                }
            }
            Err(e) => {
                tracing::error!("SSE backfill failed: {e}");
                return;
            }
        }
        loop {
            match live.recv().await {
                Ok(ev) => {
                    if ev.channel_id != channel_id || ev.id <= cursor {
                        continue;
                    }
                    cursor = ev.id;
                    let view = match channel::event_view(&store, ev.id).await {
                        Ok(Some(v)) => v,
                        _ => continue,
                    };
                    if tx.send(make_channel_sse(&view, want_json)).await.is_err() {
                        return;
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                    // The log is durable, so a lagged viewer recovers
                    // by re-reading from its cursor - no gap, no loss.
                    match channel::events_after(&store, channel_id, cursor, 10_000).await {
                        Ok(missed) => {
                            for ev in missed {
                                cursor = ev.id;
                                if tx.send(make_channel_sse(&ev, want_json)).await.is_err() {
                                    return;
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("SSE lag recovery failed: {e}");
                            return;
                        }
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => return,
            }
        }
    });

    let stream = ReceiverStream::new(rx).map(Ok::<SseEvent, Infallible>);
    Ok(Sse::new(stream).keep_alive(KeepAlive::default()).into_response())
}

fn make_channel_sse(ev: &EventView, want_json: bool) -> SseEvent {
    let base = SseEvent::default().event("channel").id(ev.id.to_string());
    if want_json {
        base.data(serde_json::to_string(ev).unwrap_or_else(|_| "{}".into()))
    } else {
        base.data(render_event(ev))
    }
}
