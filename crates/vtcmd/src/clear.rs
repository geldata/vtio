//! Screen clearing commands.

use vtansi::encode::{Encode, EncodeError};
use vtansi::{csi, write_const_str_into};

/// Clear the entire screen.
pub struct ClearAll;

impl Encode for ClearAll {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!("2J"))
    }
}

/// Clear from cursor to end of screen.
pub struct ClearFromCursorDown;

impl Encode for ClearFromCursorDown {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!("J"))
    }
}

/// Clear from cursor to beginning of screen.
pub struct ClearFromCursorUp;

impl Encode for ClearFromCursorUp {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!("1J"))
    }
}

/// Clear the current line.
pub struct ClearLine;

impl Encode for ClearLine {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!("2K"))
    }
}

/// Clear from cursor to end of line.
pub struct ClearUntilNewLine;

impl Encode for ClearUntilNewLine {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!("K"))
    }
}

/// Purge scrollback buffer (extended command).
pub struct ClearScrollback;

impl Encode for ClearScrollback {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!("3J"))
    }
}
