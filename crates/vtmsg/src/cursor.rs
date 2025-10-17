//! Cursor movement and control commands.

use vtenc::{ConstEncode, ConstEncodedLen, Encode, EncodeError, csi, esc, write_csi};

/// Move cursor to the specified position (1-indexed).
pub struct MoveTo {
    pub row: u16,
    pub col: u16,
}

impl ConstEncodedLen for MoveTo {
    // CSI (2) + max u16 digits (5) + ";" (1) + max u16 digits (5) + "H" (1) = 14
    const ENCODED_LEN: usize = 14;
}

impl Encode for MoveTo {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.row, ";", self.col, "H")
    }
}

/// Move cursor up by the specified number of lines.
pub struct MoveUp(pub u16);

impl ConstEncodedLen for MoveUp {
    // CSI (2) + max u16 digits (5) + "A" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for MoveUp {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "A")
    }
}

/// Move cursor down by the specified number of lines.
pub struct MoveDown(pub u16);

impl ConstEncodedLen for MoveDown {
    // CSI (2) + max u16 digits (5) + "B" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for MoveDown {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "B")
    }
}

/// Move cursor left by the specified number of columns.
pub struct MoveLeft(pub u16);

impl ConstEncodedLen for MoveLeft {
    // CSI (2) + max u16 digits (5) + "D" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for MoveLeft {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "D")
    }
}

/// Move cursor right by the specified number of columns.
pub struct MoveRight(pub u16);

impl ConstEncodedLen for MoveRight {
    // CSI (2) + max u16 digits (5) + "C" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for MoveRight {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "C")
    }
}

/// Move cursor to the beginning of the line N lines down.
pub struct MoveToNextLine(pub u16);

impl ConstEncodedLen for MoveToNextLine {
    // CSI (2) + max u16 digits (5) + "E" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for MoveToNextLine {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "E")
    }
}

/// Move cursor to the beginning of the line N lines up.
pub struct MoveToPreviousLine(pub u16);

impl ConstEncodedLen for MoveToPreviousLine {
    // CSI (2) + max u16 digits (5) + "F" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for MoveToPreviousLine {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "F")
    }
}

/// Move cursor to the specified column on the current line.
pub struct MoveToColumn(pub u16);

impl ConstEncodedLen for MoveToColumn {
    // CSI (2) + max u16 digits (5) + "G" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for MoveToColumn {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; self.0, "G")
    }
}

/// Hide the cursor.
pub struct HideCursor;

impl ConstEncode for HideCursor {
    const STR: &'static str = csi!("?25l");
}

/// Show the cursor.
pub struct ShowCursor;

impl ConstEncode for ShowCursor {
    const STR: &'static str = csi!("?25h");
}

/// Enable cursor blinking.
pub struct EnableCursorBlinking;

impl ConstEncode for EnableCursorBlinking {
    const STR: &'static str = csi!("?12h");
}

/// Disable cursor blinking.
pub struct DisableCursorBlinking;

impl ConstEncode for DisableCursorBlinking {
    const STR: &'static str = csi!("?12l");
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

impl ConstEncodedLen for SetCursorShape {
    // CSI (2) + max single digit (1) + " q" (2) = 5
    const ENCODED_LEN: usize = 5;
}

impl Encode for SetCursorShape {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        let code = match self.0 {
            CursorShape::Default => 0,
            CursorShape::BlinkingBlock => 1,
            CursorShape::SteadyBlock => 2,
            CursorShape::BlinkingUnderline => 3,
            CursorShape::SteadyUnderline => 4,
            CursorShape::BlinkingBar => 5,
            CursorShape::SteadyBar => 6,
        };
        write_csi!(buf; code, " q")
    }
}

/// Save cursor position (DECSC).
pub struct SaveCursorPosition;

impl ConstEncode for SaveCursorPosition {
    // DECSC: ESC 7 (not a CSI sequence)
    const STR: &'static str = esc!("7");
}

/// Restore cursor position (DECRC).
pub struct RestoreCursorPosition;

impl ConstEncode for RestoreCursorPosition {
    // DECRC: ESC 8 (not a CSI sequence)
    const STR: &'static str = esc!("8");
}
