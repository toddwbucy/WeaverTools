//! The instrument's surfaces, one module per surface of the charter's
//! section 3, per `weaver-web-Spec` section 1.
//!
//! **These stand apart from `web/`, which is the conversation half.** That
//! module and its layout retire with the modules the register at
//! `docs/project/inventory-weaver-web-code.md` names, and a surface written
//! against its `base.html` would retire with it. So this tree carries its
//! own layout and its own routes, and the retirement lifts `web/` out
//! without reaching in here.
//!
//! **None of them writes the recorded half**, per Spec section 6: a position
//! and a run land by the ingest of section 3.1 alone.

pub mod gate;
pub mod record;

use axum::Router;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::get;

use crate::store::Store;

/// Every surface this tree serves, mounted on the store and nothing else.
///
/// **The state is the store because a surface that renders what is kept
/// reads the store and nothing else**, per Spec section 6. A surface that
/// holds a seam takes it as its own argument rather than widening this.
pub fn routes() -> Router<Store> {
    Router::new()
        .merge(record::routes())
        .route("/assets/surfaces/instrument.css", get(stylesheet))
}

/// **This tree serves its own stylesheet.** The conversation half's is
/// served by `web/`'s router, which retires with that half, so a surface
/// linking it would lose its styling the day the register's retirement
/// lands - the coupling this tree's own layout exists to avoid, which
/// duplicating the layout alone would not have broken.
async fn stylesheet() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        include_str!("../../assets/surfaces/instrument.css"),
    )
}
