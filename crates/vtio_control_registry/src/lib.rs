use linkme::distributed_slice;

use vtio_control_base::{EscapeSequenceIntroducer, EscapeSequenceParam};

pub type Handler = fn(&[EscapeSequenceParam]);

#[derive(Copy, Clone, Debug)]
pub struct EscapeSequenceMatchEntry {
    /// Sequence name.
    pub name: &'static str,
    /// Sequence introducer kind.
    pub intro: EscapeSequenceIntroducer,
    /// A byte prefix following the introducer, this would include
    /// the private byte and any static params.
    pub prefix: &'static [u8],
    /// Final byte if fixed; otherwise 0 to indicate “any final with flags”.
    pub final_byte: u8,
    /// Handler function to call on match.
    pub handler: Handler,
}

#[distributed_slice]
pub static ESCAPE_SEQUENCE_REGISTRY: [EscapeSequenceMatchEntry] = [..];
