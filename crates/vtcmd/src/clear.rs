//! Screen clearing commands.

use vtansi::csi;
use vtansi::encode::StaticEncode;

/// Clear the entire screen.
pub struct ClearAll;

impl StaticEncode for ClearAll {
    const STR: &'static str = csi!("2J");
}

/// Clear from cursor to end of screen.
pub struct ClearFromCursorDown;

impl StaticEncode for ClearFromCursorDown {
    const STR: &'static str = csi!("J");
}

/// Clear from cursor to beginning of screen.
pub struct ClearFromCursorUp;

impl StaticEncode for ClearFromCursorUp {
    const STR: &'static str = csi!("1J");
}

/// Clear the current line.
pub struct ClearLine;

impl StaticEncode for ClearLine {
    const STR: &'static str = csi!("2K");
}

/// Clear from cursor to end of line.
pub struct ClearUntilNewLine;

impl StaticEncode for ClearUntilNewLine {
    const STR: &'static str = csi!("K");
}

/// Purge scrollback buffer (extended command).
pub struct ClearScrollback;

impl StaticEncode for ClearScrollback {
    const STR: &'static str = csi!("3J");
}
