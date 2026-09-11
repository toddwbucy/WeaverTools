//! conforms: web-a-chip-filters-only-on-an-indexed-column
//!
//! Record, per the charter's section 3.6: **every run and branch with the
//! tuple that produced it.** A reading without its tuple is a reading of an
//! unnamed compound, and a registry that cannot hold a failure is a
//! marketing surface, so this surface shows what ran and says what it does
//! not know rather than omitting it.
//!
//! **It reads the store and nothing else**, per `weaver-web-Spec` section 6,
//! through the fifth read of section 4 and no other query. It writes
//! nothing: a run lands by the ingest of section 3.1 alone.
//!
//! **A chip is a query rather than a location.** Clearing one widens the
//! list where the operator stands and nothing navigates, so what is not
//! being seen is always visible, per the charter's section 3.6. Every chip
//! this surface offers names a column section 2.7 indexes, which is the
//! assertion this file's header cites: a chip on an unindexed column would
//! be a sequential scan offered as though it were cheap.

use askama::Template;
use axum::Router;
use axum::extract::{Query, State};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use serde::Deserialize;

use crate::store::{Chip, Cursor, RunId, RunTuple, Store};

/// How many runs a page holds. **A list is walked rather than quoted**, per
/// section 4's fifth read, so this is a bound on the page and not on what
/// the filter admits.
const PAGE: u32 = 50;

pub fn routes() -> Router<Store> {
    Router::new().route("/record", get(record))
}

/// The query this surface takes, which is its whole state: a chip and a
/// place in the walk. **There is no state that is a location**, per section
/// 6, so a link carrying no chip is the widest list rather than a different
/// page.
#[derive(Debug, Default, Deserialize)]
pub struct Ask {
    /// The chip's kind, one of `artifact`, `session` or `branches`. An
    /// unknown kind is no chip rather than an error, because a chip is a
    /// query and a query nobody wrote is the widest list.
    chip: Option<String>,
    /// The chip's value.
    of: Option<String>,
    /// The page's key, the ingest's order and the run's identity together,
    /// spelled as this surface last handed it out.
    after_at: Option<String>,
    after_run: Option<String>,
}

impl Ask {
    fn chip(&self) -> Option<Chip> {
        let of = self.of.as_deref()?;
        match self.chip.as_deref()? {
            "artifact" => Some(Chip::RecordIdentity(of.to_string())),
            "session" => Some(Chip::Session(of.to_string())),
            "branches" => Some(Chip::Branches(RunId(of.to_string()))),
            _ => None,
        }
    }

    fn cursor(&self) -> Option<Cursor> {
        let at = self.after_at.as_deref()?;
        let run = self.after_run.as_deref()?;
        Some(Cursor {
            ingested_at: at.parse().ok()?,
            run: RunId(run.to_string()),
        })
    }
}

/// One row as the surface draws it. **Every member says what it is or says
/// it is absent**, per Spec section 6: a member the record did not carry
/// renders as absent and never as a zero or an empty string, which is the
/// absent-not-empty rule at the view.
pub struct Row {
    pub run: String,
    pub record_identity: String,
    pub device: String,
    pub engine: String,
    pub seed: String,
    pub session: String,
    pub parent: Option<String>,
    pub branch_position: Option<i32>,
    pub parting_position: Option<i32>,
    pub ingested_at: String,
}

/// The word a surface uses where the record carried nothing. **It is a word
/// and not a blank**, so a reader can tell a member nobody recorded from one
/// this surface failed to draw.
const ABSENT: &str = "absent";

fn absent_or(value: Option<&str>) -> String {
    value
        .map(str::to_owned)
        .unwrap_or_else(|| ABSENT.to_string())
}

impl From<RunTuple> for Row {
    fn from(t: RunTuple) -> Self {
        Row {
            run: t.run.0,
            record_identity: if t.record_identity.is_empty() {
                // Spec 2.3: the sentinel is the empty string and means a
                // hash the SPU could not compute. It is a fact of the
                // record rather than a missing member, so it is named.
                "the hash failed".to_string()
            } else {
                t.record_identity
            },
            device: absent_or(t.device.as_deref()),
            engine: t
                .engine
                .as_ref()
                .map(|_| "recorded".to_string())
                .unwrap_or_else(|| ABSENT.to_string()),
            seed: absent_or(t.seed.as_deref()),
            session: absent_or(t.record_session.as_deref()),
            parent: t.parent_run.map(|p| p.0),
            branch_position: t.branch_position,
            parting_position: t.parting_position,
            ingested_at: t.ingested_at.to_rfc3339(),
        }
    }
}

#[derive(Template)]
#[template(path = "record.html")]
struct RecordPage {
    here: &'static str,
    rows: Vec<Row>,
    /// The chip in force, as a word a reader can see and a link can clear.
    chip_kind: Option<String>,
    chip_of: Option<String>,
    /// Where the next page resumes, absent at the end of what the filter
    /// admits.
    next_at: Option<String>,
    next_run: Option<String>,
}

async fn record(State(store): State<Store>, Query(ask): Query<Ask>) -> Result<Response, Failure> {
    let chip = ask.chip();
    let page = store
        .runs(chip.as_ref(), PAGE, ask.cursor().as_ref())
        .await?;
    let next = page.next;
    let html = RecordPage {
        here: "record",
        rows: page.runs.into_iter().map(Row::from).collect(),
        chip_kind: ask.chip.clone().filter(|_| chip.is_some()),
        chip_of: ask.of.clone().filter(|_| chip.is_some()),
        next_at: next.as_ref().map(|c| c.ingested_at.to_rfc3339()),
        next_run: next.map(|c| c.run.0),
    }
    .render()
    .map_err(|e| Failure(e.into()))?;
    Ok(Html(html).into_response())
}

/// A refusal this surface met. **The chain goes to the log and the response
/// stays generic**, so a query's text, a path, or an upstream's detail never
/// reaches a browser, which is the posture `web/mod.rs` already holds for
/// the half that retires.
pub struct Failure(anyhow::Error);

impl<E: Into<anyhow::Error>> From<E> for Failure {
    fn from(error: E) -> Self {
        Failure(error.into())
    }
}

impl IntoResponse for Failure {
    fn into_response(self) -> Response {
        tracing::error!(error = ?self.0, "record surface refused");
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "the record could not be read",
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    /// The surface answers on its route with the runs the store holds, and
    /// **says absent where the record carried nothing** rather than drawing
    /// a blank, which is Spec section 6's rule at the view.
    ///
    /// This asks the router rather than the handler, so the route, the
    /// query's shape, the read and the template are all under the watch.
    ///
    /// conforms: web-a-chip-filters-only-on-an-indexed-column
    #[tokio::test]
    async fn record_renders_the_runs_and_names_what_is_absent() {
        let Some(store) = crate::store::read::tests::store().await else {
            return;
        };
        sqlx::query(
            "INSERT INTO run (run_id, record_identity, sampler, boundary_set, \
             record_session, device) \
             VALUES ('surf-a', 'SURF-REC', '{}', '[]', 'sess-surf', 'rtx-a6000') \
             ON CONFLICT (run_id) DO NOTHING",
        )
        .execute(&store.pool)
        .await
        .unwrap();

        let response = routes()
            .with_state(store.clone())
            .oneshot(
                Request::builder()
                    .uri("/record")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 1 << 20)
            .await
            .unwrap();
        let html = String::from_utf8(body.to_vec()).unwrap();

        assert!(html.contains("surf-a"), "the run is drawn: {html:.400}");
        assert!(html.contains("rtx-a6000"), "its device is drawn");
        // The seed was never recorded for this run, so the surface says so.
        assert!(html.contains("absent"), "an absent member is named");
        assert!(
            html.contains("every run the store holds"),
            "the widest list says it is the widest, so what is not seen is visible"
        );

        // A chip narrows, and one that admits nothing says so rather than
        // erroring or falling back to the widest list.
        let empty = routes()
            .with_state(store)
            .oneshot(
                Request::builder()
                    .uri("/record?chip=session&of=sess-nobody")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(empty.status(), StatusCode::OK);
        let body = axum::body::to_bytes(empty.into_body(), 1 << 20)
            .await
            .unwrap();
        let html = String::from_utf8(body.to_vec()).unwrap();
        assert!(!html.contains("surf-a"), "the chip narrowed the list");
        assert!(
            html.contains("No run answers this"),
            "and says nothing answers"
        );
    }
}
