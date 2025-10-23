#![warn(clippy::pedantic)]

pub mod registry;

pub use registry::{
    ESCAPE_SEQUENCE_REGISTRY, EscapeSequence, EscapeSequenceIntroducer, EscapeSequenceMatchEntry,
    EscapeSequenceParam, EscapeSequenceParams,
};
