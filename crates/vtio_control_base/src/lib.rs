mod bytevec;
mod esc;

pub use crate::esc::{
    EscapeSequenceIntermediates, EscapeSequenceIntroducer, EscapeSequenceParam,
    EscapeSequenceParams, EscapeSequence,
};

pub use vtenc::{
    ConstEncode, ConstEncodedLen, Encode, EncodeError, IntoSeq, WriteSeq, const_composite,
    write_apc, write_bytes_into, write_csi, write_dcs, write_esc, write_escst, write_int,
    write_str_into,
};

/// Trait for VT control sequences that have multiple possible final bytes.
///
/// When a sequence is declared with multiple final bytes in its
/// `#[vtctl(..., finalbyte = ['M', 'm'])]` attribute, it must implement
/// this trait to determine which final byte to use during encoding.
///
/// For parsing, all specified final bytes are registered and recognized.
pub trait DynamicFinalByte {
    /// Return the final byte to use for encoding this sequence.
    fn final_byte(&self) -> u8;
}
