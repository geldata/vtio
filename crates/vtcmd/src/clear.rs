//! Screen clearing commands.

use vtansi::encode::{Encode, EncodeError};
use vtansi::write_csi;

/// Clear the entire screen.
pub struct ClearAll;

impl Encode for ClearAll {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "2J")
    }
}

/// Clear from cursor to end of screen.
pub struct ClearFromCursorDown;

impl Encode for ClearFromCursorDown {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "J")
    }
}

/// Clear from cursor to beginning of screen.
pub struct ClearFromCursorUp;

impl Encode for ClearFromCursorUp {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "1J")
    }
}

/// Clear the current line.
pub struct ClearLine;

impl Encode for ClearLine {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "2K")
    }
}

/// Clear from cursor to end of line.
pub struct ClearUntilNewLine;

impl Encode for ClearUntilNewLine {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "K")
    }
}

/// Purge scrollback buffer (extended command).
pub struct ClearScrollback;

impl Encode for ClearScrollback {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "3J")
    }
}
