//! conforms: analysis-seal-ends-the-preload
//! conforms: analysis-one-preload-per-run
//! conforms: analysis-dials-as-invoked
//!
//! The seam's sender, per `weaver-analysis-Spec` section 4: three things in
//! one order - the election opens the channel, a distillate per elected
//! event follows in sequence order, and the seal ends it - and the seam is
//! owed nothing back. This crate asks nothing on this seam and reads
//! nothing from it.
//!
//! **One preload per standing of this driver, by structure.** Opening
//! consumes the sink into a sender and sealing consumes the sender, so a
//! second opener on a channel that carried one is not expressible: a retry
//! is a new run of this crate rather than a second preload inside one. It
//! dials under whatever identity it was invoked with and mints none, the
//! credential's rightness judged at the far end by the door.

use std::io::Write;

use crate::project::Distillate;

/// The standing preload: the opener has crossed and distillates may follow.
/// Constructed only by [`open`], which consumes the sink, and consumed by
/// [`Sender::seal`].
pub struct Sender<W: Write> {
    sink: W,
}

/// Open the preload: the election crosses whole as the channel's first
/// traffic, declaring the replayed session under its own name so the
/// holdings answer to the name the loop asks against.
pub fn open<W: Write>(mut sink: W, session: &str) -> std::io::Result<Sender<W>> {
    sink.write_all(crate::project::render_opener(session).as_bytes())?;
    Ok(Sender { sink })
}

impl<W: Write> Sender<W> {
    /// One distillate, in the record's order. Takes the projection's own
    /// type and never a raw event: the sender's shape is the compile-time
    /// half of the no-writer claim.
    pub fn send(&mut self, distillate: &Distillate) -> std::io::Result<()> {
        self.sink.write_all(distillate.frame().as_bytes())
    }

    /// The seal: one empty JSON object on its own line, `{}` canonically,
    /// and this crate writes that spelling. A blank line is framing residue
    /// and not a seal, so the parked replay ask would never answer over
    /// one. Consumes the sender, which is the one-preload owing met by
    /// structure.
    pub fn seal(mut self) -> std::io::Result<()> {
        self.sink.write_all(b"{}\n")?;
        self.sink.flush()
    }
}
