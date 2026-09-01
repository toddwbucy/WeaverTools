//! The diagnostic consumer, per `weaver-analysis-PRD`: a crate outside the
//! agent boundary that parses the operator's record, derives the diagnostic
//! declaration from it, preloads what the parse projects, and reads the
//! diagnostic-trace the run produced, gating every reading on the outcome
//! the record states. One module per obligation, re-exported here, per
//! `weaver-analysis-Spec` section 1.
//!
//! **It writes no record and nothing it produces reaches a decoder.** No
//! call constructs a trace writer - there is no writer dependency to
//! construct one from - and the preload's sender takes distillates and
//! never events:
//!
//! ```compile_fail
//! // The sender takes the projection's own type, never a raw event.
//! let event = weaver_analysis::Event::parse("{}").unwrap();
//! let mut sender = weaver_analysis::preload::open(Vec::new(), "s-1").unwrap();
//! sender.send(&event);
//! ```
//!
//! ```compile_fail
//! // One preload per standing: the seal consumes the sender.
//! let mut sender = weaver_analysis::preload::open(Vec::new(), "s-1").unwrap();
//! sender.seal().unwrap();
//! sender.send(&weaver_analysis::project::project(&[])[0]);
//! ```

pub mod declare;
pub mod preload;
pub mod project;
pub mod reading;
pub mod record;

pub use declare::{AnalystInputs, DeriveRefusal, derive};
pub use project::{Distillate, ELECTION, project, render_opener};
pub use reading::{Bracket, Gated, Outcome, RecordKind, brackets, gate, record_kind};
pub use record::{Envelope, Event, parse_record, value_at};
