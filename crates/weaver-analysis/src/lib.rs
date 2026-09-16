//! conforms: analysis-writes-no-record
//! conforms: analysis-one-preload-per-run
//!
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
//! **One preload per standing of this driver**, per `weaver-analysis-Spec`
//! section 4, whose instrument is a compile-fail pin on the shape that
//! would break it. **Both of this crate's pins are written here**, each
//! reaching the crate the way a caller reaches it, through the public
//! path rather than through a module's internals, and a citation sits
//! with its own doctest, so both citations sit here too. That is where
//! these two are written and not where a compile-fail pin has to be
//! written: rustdoc collects a doctest from any documented item, and a
//! module holding a pin of its own cites it from its own header. The
//! structure each rests on is its own module's, the seal consuming the
//! sender being `preload.rs`'s.
//!
//! ```compile_fail
//! // One preload per standing: the seal consumes the sender.
//! let mut sender = weaver_analysis::preload::open(Vec::new(), "s-1").unwrap();
//! sender.seal().unwrap();
//! sender.send(&weaver_analysis::project::project(&[])[0]);
//! ```

pub mod capture;
pub mod declare;
pub mod field;
pub mod lens;
pub mod preload;
pub mod project;
pub mod reading;
pub mod record;
pub mod signals;
pub mod stream;

pub use capture::{Capture, Comparison, Provenance, compare};
pub use declare::{AnalystInputs, DeriveRefusal, SinkKind, derive};
pub use field::{Address, Answer, FieldReader};
pub use lens::{
    Lens, LensRefusal, Manifest, Unembedding, WeightsDigest, manifest_path_for, read_manifest,
    rms_epsilon, sha256_hex, sha256_hex_of_file, shards_for, verify_weights,
};
pub use project::{
    CutRefusal, Distillate, ELECTION, cut_through, project, project_as, render_opener,
};
pub use reading::{Bracket, Gated, Outcome, RecordKind, brackets, gate, record_kind};
pub use record::{Envelope, Event, parse_record, value_at};
pub use signals::{Point, Series, Signals};
pub use stream::{Drained, Reader, Step, drain};
