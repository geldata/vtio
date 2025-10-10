//! Cursor movement and control commands.

use vtansi::encode::{Encode, EncodeError, write_str_into};
use vtansi::{csi, esc, write_const_str_into};

/// Move cursor to the specified position (1-indexed).
pub struct MoveTo {
    pub row: u16,
    pub col: u16,
}

impl Encode for MoveTo {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_str_into(buf, &csi!("{};{}H", self.row, self.col))
    }
}

/// Move cursor up by the specified number of lines.
pub struct MoveUp(pub u16);

impl Encode for MoveUp {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        if self.0 == 0 {
            return Ok(0);
        }
        write_str_into(buf, &csi!("{}A", self.0))
    }
}

/// Move cursor down by the specified number of lines.
pub struct MoveDown(pub u16);

impl Encode for MoveDown {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        if self.0 == 0 {
            return Ok(0);
        }
        write_str_into(buf, &csi!("{}B", self.0))
    }
}

/// Move cursor left by the specified number of columns.
pub struct MoveLeft(pub u16);

impl Encode for MoveLeft {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        if self.0 == 0 {
            return Ok(0);
        }
        write_str_into(buf, &csi!("{}D", self.0))
    }
}

/// Move cursor right by the specified number of columns.
pub struct MoveRight(pub u16);

impl Encode for MoveRight {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        if self.0 == 0 {
            return Ok(0);
        }
        write_str_into(buf, &csi!("{}C", self.0))
    }
}

/// Move cursor to the beginning of the line N lines down.
pub struct MoveToNextLine(pub u16);

impl Encode for MoveToNextLine {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_str_into(buf, &csi!("{}E", self.0))
    }
}

/// Move cursor to the beginning of the line N lines up.
pub struct MoveToPreviousLine(pub u16);

impl Encode for MoveToPreviousLine {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_str_into(buf, &csi!("{}F", self.0))
    }
}

/// Move cursor to the specified column on the current line.
pub struct MoveToColumn(pub u16);

impl Encode for MoveToColumn {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_str_into(buf, &csi!("{}G", self.0))
    }
}

/// Hide the cursor.
pub struct HideCursor;

impl Encode for HideCursor {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!("?25l"))
    }
}

/// Show the cursor.
pub struct ShowCursor;

impl Encode for ShowCursor {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!("?25h"))
    }
}

/// Enable cursor blinking.
pub struct EnableCursorBlinking;

impl Encode for EnableCursorBlinking {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!("?12h"))
    }
}

/// Disable cursor blinking.
pub struct DisableCursorBlinking;

impl Encode for DisableCursorBlinking {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_const_str_into!(buf, csi!("?12l"))
    }
}

/// Cursor shape variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    /// Default cursor shape (usually blinking block).
    Default,
    /// Blinking block cursor.
    BlinkingBlock,
    /// Steady (non-blinking) block cursor.
    SteadyBlock,
    /// Blinking underline cursor.
    BlinkingUnderline,
    /// Steady underline cursor.
    SteadyUnderline,
    /// Blinking bar (vertical line) cursor.
    BlinkingBar,
    /// Steady bar cursor.
    SteadyBar,
}

/// Set cursor shape using DECSCUSR.
pub struct SetCursorShape(pub CursorShape);

impl Encode for SetCursorShape {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let code = match self.0 {
            CursorShape::Default => 0,
            CursorShape::BlinkingBlock => 1,
            CursorShape::SteadyBlock => 2,
            CursorShape::BlinkingUnderline => 3,
            CursorShape::SteadyUnderline => 4,
            CursorShape::BlinkingBar => 5,
            CursorShape::SteadyBar => 6,
        };
        write_str_into(buf, &csi!("{} q", code))
    }
}

/// Save cursor position (DECSC).
pub struct SaveCursorPosition;

impl Encode for SaveCursorPosition {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        // DECSC: ESC 7 (not a CSI sequence)
        write_const_str_into!(buf, esc!("7"))
    }
}

/// Restore cursor position (DECRC).
pub struct RestoreCursorPosition;

impl Encode for RestoreCursorPosition {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        // DECRC: ESC 8 (not a CSI sequence)
        write_const_str_into!(buf, esc!("8"))
    }
}
