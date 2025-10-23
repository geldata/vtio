#![warn(clippy::pedantic)]

mod bytevec;
pub mod params;
pub mod registry;

pub use params::{EscapeSequenceParam, EscapeSequenceParams};

pub use registry::{
    ESCAPE_SEQUENCE_REGISTRY, EscapeSequence, EscapeSequenceIntroducer, EscapeSequenceMatchEntry,
};
