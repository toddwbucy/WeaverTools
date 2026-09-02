//! weaver-web: the WeaverTools suite's frontend, two binaries in one
//! crate (Spec section 2). The server (`weaver-web`) presents HTTP
//! and holds everything that is not box-bound. The connector
//! (`weaver-web-connector`) runs on the agents' box, holds the
//! box-bound reaches, and dials the server (PRD section 3).

pub mod adapters;
pub mod channel;
pub mod config;
pub mod lifecycle;
pub mod queue;
pub mod registry;
pub mod repro;
pub mod router;
pub mod store;
pub mod traceview;
pub mod web;
pub mod wire;
