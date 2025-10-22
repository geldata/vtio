#![warn(clippy::pedantic)]

pub mod registry;

pub use registry::{
    EscapeSequence,
    EscapeSequenceIntroducer,
    EscapeSequenceMatchEntry,
    EscapeSequenceParam,
    EscapeSequenceParams,
    ESCAPE_SEQUENCE_REGISTRY,
};
