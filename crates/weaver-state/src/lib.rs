//! conforms: state-custody-without-policy
//!
//! The custodian, per `weaver-state-PRD`: stores what it is handed,
//! organizes what it stores, serves what it is asked for, and does nothing
//! else. The management of state as it concerns context for the decoder
//! belongs to the harness and its control loops, and nothing in this crate
//! holds an opinion about why it was asked.
//!
//! The ingest half stands. The serve half is chartered and unshaped, per
//! the charter's cell: its first asker is the context-injection control
//! loop, and the surface is elected in that loop's act against real asks.

pub mod store;

pub use store::{CustodyFault, Distillate, Election, Store, parse_distillate};
