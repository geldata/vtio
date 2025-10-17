//! Screen clearing commands.

use vtenc::csi;
use vtenc::encode::ConstEncode;

/// Clear the entire screen.
pub struct ClearAll;

impl ConstEncode for ClearAll {
    const STR: &'static str = csi!("2J");
}

/// Clear from cursor to end of screen.
pub struct ClearFromCursorDown;

impl ConstEncode for ClearFromCursorDown {
    const STR: &'static str = csi!("J");
}

/// Clear from cursor to beginning of screen.
pub struct ClearFromCursorUp;

impl ConstEncode for ClearFromCursorUp {
    const STR: &'static str = csi!("1J");
}

/// Clear the current line.
pub struct ClearLine;

impl ConstEncode for ClearLine {
    const STR: &'static str = csi!("2K");
}

/// Clear from cursor to end of line.
pub struct ClearUntilNewLine;

impl ConstEncode for ClearUntilNewLine {
    const STR: &'static str = csi!("K");
}

/// Purge scrollback buffer (extended command).
pub struct ClearScrollback;

impl ConstEncode for ClearScrollback {
    const STR: &'static str = csi!("3J");
}
