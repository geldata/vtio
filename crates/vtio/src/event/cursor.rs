//! Cursor movement and control commands.

use bitflags::bitflags;
use vtio_control_derive::{terminal_mode, VTControl};
use vtio_control_base::EscapeSequenceParam;
use vtenc::{IntoSeq, WriteSeq};

/// Cursor Origin Mode (`DECOM`).
///
/// If set, the origin of the coordinate system is relative to the
/// current scroll region.
///
/// The origin is used by cursor positioning commands such as
/// [`CursorPosition`], [`CursorVerticalAbsolute`], [`CursorHorizontalAbsolute`], and
/// cursor position reports.
///
/// When this mode is set, certain sequences will force the cursor to be
/// in the scrolling region, including carriage return, next line,
/// cursor next/previous line operations.
///
/// If set, the cursor is moved to the top left of the current scroll
/// region.
///
/// See <https://terminalguide.namepad.de/mode/p6/> for
/// terminal support specifics.
terminal_mode!(RelativeCursorOriginMode, private = '?', params = ["6"]);

/// Cursor Blinking (`ATT610_BLINK`).
///
/// If set, the cursor is blinking.
///
/// This mode interacts with the blinking part of the Select Cursor Style
/// (`DECSCUSR`) setting. In xterm, this mode is synchronized with the
/// blinking part of the cursor style. In urxvt, this mode is additive to
/// the cursor style setting.
///
/// See also [`SetCursorStyle`] for a more widely supported alternative.
///
/// See <https://terminalguide.namepad.de/mode/p12/> for
/// terminal support specifics.
terminal_mode!(CursorBlinking, private = '?', params = ["12"]);

/// Cursor Visibility Mode (`DECTCEM`).
///
/// Set visibility of the cursor.
///
/// If set, the cursor is visible. If reset, the cursor is hidden.
///
/// See <https://terminalguide.namepad.de/mode/p25/> for
/// terminal support specifics.
terminal_mode!(CursorVisibility, private = '?', params = ["25"]);

/// Save cursor (`DECSC`).
///
/// Save cursor position and other state.
///
/// The primary and alternate screen have distinct save state.
///
/// The following state is saved:
///   * the state of [`RelativeCursorOriginMode`]
///     (but not its saved state for restore mode);
///   * the current attributes;
///   * If newly printed characters are protected
///     (like start protected area or select character protection attribute);
///   * the current cursor position, relative to the
///     origin set via cursor origin;
///   * pending wrap state;
///   * GL and GR character sets;
///   * G0, G1, G2, G3 character sets.
///
/// One saved state is kept per screen (main / alternative).
/// If for the current screen state was already saved it is overwritten.
///
/// The state can be restored using [`RestoreCursor`].
///
/// See <https://terminalguide.namepad.de/seq/a_esc_a7/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[esc(finalbyte = '7')]
pub struct SaveCursor;

/// Restore cursor (`DECRC`).
///
/// Restore cursor position and other state.
///
/// The primary and alternate screen have distinct save state.
///
/// The following state is restored:
///   * the state of [`RelativeCursorOriginMode`]
///     (but not its saved state for restore mode);
///   * the current attributes;
///   * If newly printed characters are protected
///     (like start protected area or select character protection attribute);
///   * the current cursor position, relative to the
///     origin set via cursor origin;
///   * pending wrap state;
///   * GL and GR character sets;
///   * G0, G1, G2, G3 character sets.
///
/// If no [`SaveCursor`] was done previously values are reset to their
/// hard reset values.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_a8/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[esc(finalbyte = '8')]
pub struct RestoreCursor;

/// Backspace (`BS`).
///
/// Move the cursor one position to the left.
///
/// If the cursor is on the left-most column, the behavior is implementation
/// dependent (may stay in place or wrap to previous line).
///
/// This unsets the pending wrap state without wrapping.
///
/// See <https://terminalguide.namepad.de/seq/c_bs/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[c0(code = 0x08)]
pub struct Backspace;

/// Horizontal Tab (`TAB`).
///
/// Move the cursor to the next tab stop.
///
/// If there are no more tab stops, the cursor is moved to the right-most
/// column.
///
/// Tab stops can be set using [`HorizontalTabSet`].
///
/// See <https://terminalguide.namepad.de/seq/c_tab/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[c0(code = 0x09)]
pub struct HorizontalTab;

/// Line Feed (`LF`).
///
/// Move the cursor to the next line.
///
/// The behavior depends on the Line Feed mode:
///   * If Line Feed mode is not set: move the cursor down one line
///     (like [`Index`])
///   * If Line Feed mode is set: move the cursor down one line and to the
///     left-most column (like [`NextLine`])
///
/// See <https://terminalguide.namepad.de/seq/c_lf/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[c0(code = 0x0A)]
pub struct LineFeed;

/// Vertical Tab (`VT`).
///
/// Move the cursor down one line (same as [`LineFeed`]).
///
/// See <https://terminalguide.namepad.de/seq/c_vt/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[c0(code = 0x0B)]
pub struct VerticalTab;

/// Form Feed (`FF`).
///
/// Move the cursor down one line (same as [`LineFeed`]).
///
/// See <https://terminalguide.namepad.de/seq/c_ff/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[c0(code = 0x0C)]
pub struct FormFeed;

/// Carriage Return (`CR`).
///
/// Move the cursor to the left-most column.
///
/// This unsets the pending wrap state without wrapping.
///
/// If left and right margin mode is set and a left margin set and the cursor
/// is on or right of the left margin column it is moved to the left margin. If
/// the cursor is left of the left margin the cursor is moved to the left-most
/// column of the screen.
///
/// If a left margin is set and [`RelativeCursorOriginMode`] is set the cursor
/// will always move to the left margin column.
///
/// See <https://terminalguide.namepad.de/seq/c_cr/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[c0(code = 0x0D)]
pub struct CarriageReturn;

/// Set Cursor Position (`CUP`).
///
/// Move cursor to the position indicated by `row` and `column`.
///
/// If `column` is 0, it is adjusted to 1. If `column` is greater than the
/// right-most column it is adjusted to the right-most column.
///
/// If `row` is 0, it is adjusted to 1. If `row` is greater than the
/// bottom-most row it is adjusted to the bottom-most row.
///
/// `column` = 1 is the left-most column. `row` = 1 is the top-most row.
///
/// This unsets the pending wrap state without wrapping.
///
/// If cursor origin mode is set the cursor row will be moved relative to the
/// top margin row and adjusted to be above or at bottom-most row in the
/// current scroll region.
///
/// If origin mode is set and left and right margin mode is set the cursor
/// will be moved relative to the left margin column and adjusted to be on or
/// left of the right margin column.
///
/// See <https://terminalguide.namepad.de/seq/csi_ch/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(finalbyte = 'H')]
pub struct CursorPosition {
    pub row: u16,
    pub col: u16,
}

/// Back Index (`DECBI`).
///
/// If the cursor is not on the left-most column of the scroll region this is
/// the same as [`CursorLeft`] with `amount = 1`.
///
/// If the cursor is on the left-most column of the scroll region and on a row
/// that is inside the scroll region, a new blank left-most column of the
/// scroll region is inserted. The previous content of the scroll region are
/// shifted to the right. The right-most column of the scroll region is
/// discarded. If the cell movement splits a multi cell character that
/// character cleared, by replacing it by spaces, keeping its attributes.
///
/// If the cursor is on the left-most column of the scroll region and on a row
/// that is outside the scroll region, nothing is changed.
///
/// The cleared space is colored according to the current SGR state.
///
/// Does not change the cursor position.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_a6/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[esc(finalbyte = '6')]
pub struct BackIndex;

/// Forward Index (`DECFI`).
///
/// If the cursor is not on the right-most column of the scroll region this is
/// the same as [`CursorRight`] with `amount = 1`.
///
/// If the cursor is on the right-most column of the scroll region and on a row
/// that is inside the scroll region, the whole left-most column of the scroll
/// region is deleted. The remaining characters are shifted to the left and
/// space from the right margin is filled with spaces. If the cell movement
/// splits a multi cell character that character is cleared, by replacing it by
/// spaces, keeping its attributes.
///
/// If the cursor is on the right-most column of the scroll region and on a row
/// that is outside the scroll region, nothing is changed.
///
/// The cleared space is colored according to the current SGR state.
///
/// Does not change the cursor position.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_a9/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[esc(finalbyte = '9')]
pub struct ForwardIndex;

/// Index (`IND`).
///
/// Move the cursor to the next line in the scrolling region,
/// possibly scrolling.
///
/// If the cursor is outside of the scrolling region:
///   * move the cursor one line down if it is not on the
///     bottom-most line of the screen.
///
/// If the cursor is inside the scrolling region:
///   * if the cursor is on the bottom-most line of the scrolling region:
///     - invoke [`ScrollUp`] with `amount=1`
///   * if the cursor is not on the bottom-most line of the scrolling region:
///     - move the cursor one line down
///
/// This unsets the pending wrap state without wrapping.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_cd/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[esc(finalbyte = 'D')]
pub struct Index;

/// Next Line (`NEL`).
///
/// Send [`CarriageReturn`] and [`Index`].
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[esc(finalbyte = 'E')]
pub struct NextLine;

/// Horizontal Tab Set (`HTS`).
///
/// Mark current column as tab stop column.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_ch/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[esc(finalbyte = 'H')]
pub struct HorizontalTabSet;

/// Reverse Index (`RI`).
///
/// Move the cursor to the previous line in the scrolling region,
/// possibly scrolling.
///
/// If the cursor is outside of the scrolling region:
///   * move the cursor one line up if it is not on the
///     top-most line of the screen.
///
/// If the cursor is inside the scrolling region:
///   * if the cursor is on the top-most line of the scrolling region:
///     - invoke [`ScrollDown`] with `amount=1`
///   * if the cursor is not on the top-most line of the scrolling region:
///     - move the cursor one line up
///
/// This unsets the pending wrap state without wrapping.
///
/// See <https://terminalguide.namepad.de/seq/a_esc_cm/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[esc(finalbyte = 'M')]
pub struct ReverseIndex;

/// Cursor Up (`CUU`).
///
/// Move cursor up by the specified `amount` of lines.
///
/// If `amount` is greater than the maximum move distance then it is
/// internally adjusted to the maximum. If `amount` is `0`, adjust it to `1`.
///
/// This unsets the pending wrap state without wrapping.
///
/// If the current scroll region is set and the cursor is on or below top-most
/// line of it then the cursor may move up only until it reaches the top-most
/// line of current scroll region.
///
/// If the current scroll region is not set or the cursor is above top-most
/// line of current scroll region it may move up until the top of the screen
/// (excluding scroll-back buffer).
///
/// See <https://terminalguide.namepad.de/seq/csi_ca/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(finalbyte = 'A')]
pub struct CursorUp(pub u16);

/// Cursor Down (`CUD`).
///
/// Move cursor down by the specified `amount` of lines.
///
/// If `amount` is greater than the maximum move distance then it is
/// internally adjusted to the maximum. This sequence will not scroll the
/// screen or scroll region. If `amount` is `0`, adjust it to `1`.
///
/// This unsets the pending wrap state without wrapping.
///
/// If the current scroll region is set and the cursor is on or above
/// bottom-most line of it then the cursor may move down only until it reaches
/// the bottom-most line of current scroll region.
///
/// If the current scroll region is not set or the cursor is below bottom-most
/// line of current scroll region it may move down until the bottom of the
/// screen.
///
/// See <https://terminalguide.namepad.de/seq/csi_cb/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(finalbyte = 'B')]
pub struct CursorDown(pub u16);

/// Cursor Left (`CUB`).
///
/// Move the cursor to the left `amount` cells.
///
/// If `amount` is 0, adjust it to 1.
///
/// This unsets the pending wrap state without wrapping.
///
/// If not both of reverse wrap mode and wraparound mode are set:
///   * Move the cursor `amount` cells left. If it would cross the left-most
///     column of the scrolling region, stop at the left-most column of the
///     scrolling region. If the cursor would move left of the left-most
///     column of the screen, move to the left most column of the screen.
///
/// Else:
///   * If the pending wrap state is set, reduce `amount` by one.
///   * If the cursor is left of the left-most column of the scrolling region:
///     - Move the cursor left `amount` of cells with the following rules:
///     - Each time the cursor is advanced past the left screen edge, continue
///       on the right-most column of the scrolling region on the line above.
///       If that would be before the top-most line of the screen resume on
///       the bottom most line of the screen (ignoring the top and bottom
///       margins of the scrolling region).
///   * If the cursor is on or right of the left-most column of the scrolling
///     region:
///     - Move the cursor left `amount` of cells with the following rules:
///     - Each time the cursor is advanced past the left-most column of the
///       scrolling region, continue on the right-most column of the scrolling
///       region on the line above. If that would be before the top-most line
///       of the screen resume on the bottom most line of the screen (ignoring
///       the top and bottom margins of the scrolling region).
///
/// See <https://terminalguide.namepad.de/seq/csi_cd/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(finalbyte = 'D')]
pub struct CursorLeft(pub u16);

/// Cursor Right (`CUF`).
///
/// Move the cursor right `amount` columns.
///
/// If `amount` is greater than the maximum move distance then it is
/// internally adjusted to the maximum. This sequence will not scroll the
/// screen or scroll region. If `amount` is 0, adjust it to 1.
///
/// This unsets the pending wrap state without wrapping.
///
/// If left and right margin mode is set and a right margin is set and the
/// cursor is on or left of the right-most column of it then the cursor may
/// move right only until it reaches the right-most column of current scroll
/// region.
///
/// If left and right margin mode is not set or a right margin is not set or
/// the cursor is right of right-most column of current scroll region it may
/// move right until the right-most column of the screen.
///
/// See <https://terminalguide.namepad.de/seq/csi_cc/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(finalbyte = 'C')]
pub struct CursorRight(pub u16);

/// Cursor Next Line (`CNL`).
///
/// Move `amount` lines down and to the beginning of the line.
///
/// If `amount` is 0, it is adjusted to 1.
///
/// This is a composition of cursor down with the given `amount` parameter
/// and carriage return.
///
/// See <https://terminalguide.namepad.de/seq/csi_ce/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(finalbyte = 'E')]
pub struct CursorNextLine(pub u16);

/// Cursor Previous Line (`CPL`).
///
/// Move `amount` lines up and to the beginning of the line.
///
/// If `amount` is 0, it is adjusted to 1.
///
/// This is a composition of cursor up with the given `amount` parameter
/// and carriage return.
///
/// See <https://terminalguide.namepad.de/seq/csi_cf/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(finalbyte = 'F')]
pub struct CursorPreviousLine(pub u16);

/// Cursor Horizontal Absolute (`CHA`).
///
/// Move the cursor to column `col` on the current line.
///
/// If `col` is 0, it is adjusted to 1. If `col` is greater than the
/// right-most column it is adjusted to the right-most column.
///
/// `col` = 1 is the left-most column.
///
/// This unsets the pending wrap state without wrapping.
///
/// See <https://terminalguide.namepad.de/seq/csi_cg/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(finalbyte = 'G')]
pub struct CursorHorizontalAbsolute(pub u16);

/// Cursor Horizontal Forward Tabulation (`CHT`).
///
/// Invoke horizontal tab `amount` times.
///
/// Move cursor to the `amount`-th next tab stop.
///
/// Repeat the following procedure `amount` times:
///
/// Move the cursor right until it reaches a column marked as tab stop
/// (that is not the column the cursor started on) or the right-most
/// column of the screen.
///
/// If cursor origin is set and after this move the cursor is right of the
/// right-most column of the scrolling region, move the cursor to the
/// right-most column of the scrolling region.
///
/// See <https://terminalguide.namepad.de/seq/csi_ci/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(finalbyte = 'I')]
pub struct CursorHorizontalForwardTab {
    pub amount: u16,
}

/// Cursor Horizontal Backward Tabulation (`CBT`).
///
/// Move cursor to the `amount`-th previous tab stop.
///
/// Repeat the following procedure `amount` times:
///
/// Move the cursor left until it reaches a column marked as tab stop
/// (that is not the column the cursor started on) or the left-most
/// column of the screen.
///
/// If cursor origin is set and after this move the cursor is left of the
/// left-most column of the scrolling region, move the cursor to the
/// left-most column of the scrolling region.
///
/// See <https://terminalguide.namepad.de/seq/csi_cz/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(finalbyte = 'Z')]
pub struct CursorHorizontalBackwardTab(pub u16);

/// Cursor Horizontal Position Relative (`HPR`).
///
/// Move cursor right by the specified `amount` of columns.
///
/// If `amount` is greater than the maximum move distance then it is
/// internally adjusted to the maximum. This sequence will not scroll the
/// screen or scroll region. If `amount` is 0, adjust it to 1.
///
/// This unsets the pending wrap state without wrapping.
///
/// If left and right margin mode is set and a right margin is set and the
/// cursor is on or left of the right-most column of it then the cursor may
/// move right only until it reaches the right-most column of current scroll
/// region.
///
/// If left and right margin mode is not set or a right margin is not set or
/// the cursor is right of right-most column of current scroll region it may
/// move right until the right-most column of the screen.
///
/// See <https://terminalguide.namepad.de/seq/csi_ca/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(finalbyte = 'a')]
pub struct CursorHorizontalRelative(pub u16);

/// Cursor Vertical Position Absolute (`VPA`).
///
/// Move the cursor to row `row` on the current column.
///
/// If `row` is 0, it is adjusted to 1. If `row` is greater than the
/// bottom-most row it is adjusted to the bottom-most row.
///
/// `row` = 1 is the top-most row.
///
/// This unsets the pending wrap state without wrapping.
///
/// If cursor origin mode is set the cursor row will be moved relative to the
/// top margin row and adjusted to be above or at bottom-most row in the
/// current scroll region.
///
/// See <https://terminalguide.namepad.de/seq/csi_cd/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(finalbyte = 'd')]
pub struct CursorVerticalAbsolute(pub u16);

/// Vertical Position Relative (`VPR`).
///
/// Move cursor down by the specified `amount` of lines.
///
/// If `amount` is greater than the maximum move distance then it is
/// internally adjusted to the maximum. This sequence will not scroll the
/// screen or scroll region. If `amount` is 0, adjust it to 1.
///
/// This unsets the pending wrap state without wrapping.
///
/// If the current scroll region is set and the cursor is on or above
/// bottom-most line of it then the cursor may move down only until it reaches
/// the bottom-most line of current scroll region.
///
/// If the current scroll region is not set or the cursor is below bottom-most
/// line of current scroll region it may move down until the bottom of the
/// screen.
///
/// See <https://terminalguide.namepad.de/seq/csi_ce/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(finalbyte = 'e')]
pub struct CursorVerticalRelative(pub u16);

/// Cursor style variants for `DECSCUSR`.
///
/// These control the visual appearance of the cursor.
///
/// See <https://terminalguide.namepad.de/seq/csi_cq/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, Default)]
#[repr(u8)]
pub enum CursorStyle {
    /// Default cursor style (usually blinking block).
    #[default]
    Default = 0,
    /// Blinking block cursor.
    BlinkingBlock = 1,
    /// Steady (non-blinking) block cursor.
    SteadyBlock = 2,
    /// Blinking underline cursor.
    BlinkingUnderline = 3,
    /// Steady underline cursor.
    SteadyUnderline = 4,
    /// Blinking bar (vertical line) cursor.
    BlinkingBar = 5,
    /// Steady bar cursor.
    SteadyBar = 6,
}

impl IntoSeq for CursorStyle {
    fn into_seq(&self) -> impl WriteSeq {
        *self as u8
    }
}

impl From<u8> for CursorStyle {
    fn from(value: u8) -> Self {
        match value {
            1 => CursorStyle::BlinkingBlock,
            2 => CursorStyle::SteadyBlock,
            3 => CursorStyle::BlinkingUnderline,
            4 => CursorStyle::SteadyUnderline,
            5 => CursorStyle::BlinkingBar,
            6 => CursorStyle::SteadyBar,
            _ => CursorStyle::Default,
        }
    }
}

impl From<EscapeSequenceParam> for CursorStyle {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

impl From<&EscapeSequenceParam> for CursorStyle {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

/// Select Cursor Style (`DECSCUSR`).
///
/// Set the cursor style (shape and blinking).
///
/// The cursor style is set using values 0-6:
///   * 0 - Default cursor style (usually blinking block)
///   * 1 - Blinking block
///   * 2 - Steady block
///   * 3 - Blinking underline
///   * 4 - Steady underline
///   * 5 - Blinking bar
///   * 6 - Steady bar
///
/// See <https://terminalguide.namepad.de/seq/csi_sq_t_space/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(intermediate = " ", finalbyte = 'q')]
pub struct SetCursorStyle {
    pub style: CursorStyle,
}

/// Request Cursor Style (`DECRQSS`).
///
/// Request the current cursor style.
///
/// The terminal will respond with a DCS sequence containing the current
/// cursor style setting.
///
/// See <https://terminalguide.namepad.de/seq/dcs-dollar-q-space-q/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[dcs(intermediate = "$", finalbyte = 'q', data = " q")]
pub struct RequestCursorStyle;

bitflags! {
    /// Flags for Linux cursor style.
    ///
    /// These flags control the cursor appearance and behavior in the Linux
    /// virtual console.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct LinuxCursorStyleFlags: u8 {
        /// Enable foreground and background change.
        ///
        /// When enabled the cursor changes the shown attributes of the
        /// cell it is on. Some drivers force size = 1 (none) internally
        /// if this is set.
        const ENABLE_FG_BG_CHANGE = 16;

        /// Ensure original background and cursor background differ.
        ///
        /// If the original and cursor background would be identical
        /// invert all background color channels (but not brightness).
        const ENSURE_BG_DIFFERS = 32;

        /// Ensure cursor foreground and background differ.
        ///
        /// If the cursor background and foreground would be identical
        /// invert all foreground color channels (but not brightness).
        const ENSURE_FG_BG_DIFFER = 64;
    }
}

/// Linux Cursor Style shape values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum LinuxCursorSize {
    /// Default (depending on driver: off, underline or block).
    #[default]
    Default = 0,
    /// No cursor.
    None = 1,
    /// Underline cursor.
    Underline = 2,
    /// Lower third cursor.
    LowerThird = 3,
    /// Lower half cursor.
    LowerHalf = 4,
    /// Two thirds cursor.
    TwoThirds = 5,
    /// Block cursor.
    Block = 6,
}

/// Wrapper type for combined Linux cursor shape and flags value.
///
/// This type combines a cursor shape with optional flags into a single
/// value for encoding in the Linux cursor style sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LinuxCursorShape(u8);

impl LinuxCursorShape {
    /// Create a shape value from size and flags.
    #[must_use]
    pub const fn new(size: LinuxCursorSize, flags: LinuxCursorStyleFlags) -> Self {
        Self((size as u8) | flags.bits())
    }

    /// Create a shape value from just a size.
    #[must_use]
    pub const fn from_size(size: LinuxCursorSize) -> Self {
        Self(size as u8)
    }

    /// Get the raw value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self.0
    }

    /// Extract the cursor size.
    #[must_use]
    pub const fn size(self) -> LinuxCursorSize {
        match self.0 & 0x0F {
            1 => LinuxCursorSize::None,
            2 => LinuxCursorSize::Underline,
            3 => LinuxCursorSize::LowerThird,
            4 => LinuxCursorSize::LowerHalf,
            5 => LinuxCursorSize::TwoThirds,
            6 => LinuxCursorSize::Block,
            _ => LinuxCursorSize::Default,
        }
    }

    /// Extract the cursor style flags.
    #[must_use]
    pub const fn flags(self) -> LinuxCursorStyleFlags {
        LinuxCursorStyleFlags::from_bits_truncate(self.0 & 0xF0)
    }
}

impl IntoSeq for LinuxCursorShape {
    fn into_seq(&self) -> impl WriteSeq {
        self.0
    }
}

impl From<u8> for LinuxCursorShape {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<EscapeSequenceParam> for LinuxCursorShape {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

impl From<&EscapeSequenceParam> for LinuxCursorShape {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

/// Linux Cursor Style.
///
/// Select Linux cursor style with fine-grained control over appearance.
///
/// This sequence allows setting the cursor shape, flags for attribute
/// changes, and XOR/OR masks for foreground and background color
/// manipulation.
///
/// The `shape` parameter combines the size (0-6) with optional
/// flags (16, 32, 64).
///
/// The `xor` and `or` parameters define changes to foreground and
/// background of the cell where the cursor is shown when the
/// `ENABLE_FG_BG_CHANGE` flag is set. Each bit controls one color channel:
///
/// | bit value |          meaning              |
/// |-----------|-------------------------------|
/// |         1 | foreground blue channel       |
/// |         2 | foreground green channel      |
/// |         4 | foreground red channel        |
/// |         8 | foreground brightness channel |
/// |        16 | background blue channel       |
/// |        32 | background green channel      |
/// |        64 | background red channel        |
/// |       128 | background brightness         |
///
/// The effective change for each bit depends on its value in both
/// parameters:
///
/// | or bit | xor bit |   change  |
/// |--------|---------|-----------|
/// |    0   |    0    | no change |
/// |    1   |    0    | enable    |
/// |    0   |    1    | toggle    |
/// |    1   |    1    | disable   |
///
/// See <https://terminalguide.namepad.de/seq/csi_sc__p/> for terminal
/// support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, VTControl)]
#[csi(intermediate = " ", finalbyte = 'q')]
pub struct LinuxCursorStyle {
    /// Combined cursor shape and flags value.
    pub shape: LinuxCursorShape,
    /// XOR mask for color channel manipulation.
    pub xor: u8,
    /// OR mask for color channel manipulation.
    pub or: u8,
}

impl LinuxCursorStyle {
    /// Create a new Linux cursor style with the specified size.
    ///
    /// Flags, xor, and or values are initialized to 0.
    #[must_use]
    pub const fn from_size(size: LinuxCursorSize) -> Self {
        Self {
            shape: LinuxCursorShape::from_size(size),
            xor: 0,
            or: 0,
        }
    }

    /// Create a new Linux cursor style with size and flags.
    ///
    /// xor and or values are initialized to 0.
    #[must_use]
    pub const fn with_flags(size: LinuxCursorSize, flags: LinuxCursorStyleFlags) -> Self {
        Self {
            shape: LinuxCursorShape::new(size, flags),
            xor: 0,
            or: 0,
        }
    }

    /// Create a new Linux cursor style with all parameters.
    #[must_use]
    pub const fn with_colors(
        size: LinuxCursorSize,
        flags: LinuxCursorStyleFlags,
        xor: u8,
        or: u8,
    ) -> Self {
        Self {
            shape: LinuxCursorShape::new(size, flags),
            xor,
            or,
        }
    }

    /// Get the cursor size.
    #[must_use]
    pub const fn size(self) -> LinuxCursorSize {
        self.shape.size()
    }

    /// Get the cursor style flags.
    #[must_use]
    pub const fn flags(self) -> LinuxCursorStyleFlags {
        self.shape.flags()
    }
}

/// Request Cursor Position Report (`CPR`).
///
/// Request the current cursor position.
///
/// The terminal replies with:
///
/// `CSI <row> ; <column> R`
///
/// If [`RelativeCursorOriginMode`] is set, the cursor position is reported
/// relative to the top left corner of the scroll area. Otherwise, it is
/// reported relative to the top left corner of the screen.
///
/// The response uses [`CursorPositionReport`].
///
/// See <https://terminalguide.namepad.de/seq/csi_sn-6/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(params = ["6"], finalbyte = 'n')]
pub struct RequestCursorPosition;

/// Cursor Position Report (`CPR`).
///
/// Response from the terminal to [`RequestCursorPosition`].
///
/// Contains the current cursor position as `row` and `col`.
///
/// The position may be relative to the scroll area if
/// [`RelativeCursorOriginMode`] is set, or relative to the screen otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[csi(finalbyte = 'R')]
pub struct CursorPositionReport {
    pub row: u16,
    pub col: u16,
}

/// Request Cursor Information Report (`DECCIR`).
///
/// Request detailed cursor information including position, attributes,
/// protection status, flags, and character set information.
///
/// The terminal replies with a DCS sequence containing:
/// - Cursor position (row and column)
/// - Current text attributes (bold, underline, blink, inverse)
/// - Character protection status
/// - Various cursor flags (origin mode, single shift, pending wrap)
/// - Character set information (GL, GR, and G0-G3 sets)
///
/// See <https://terminalguide.namepad.de/seq/csi_sw_t_dollar-1/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(params = ["1"], intermediate = "$", finalbyte = 'w')]
pub struct RequestCursorInformationReport;

bitflags! {
    /// Cursor attribute flags for cursor information report.
    ///
    /// These flags encode the currently active text attributes.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct CursorAttributes: u8 {
        /// Bold text attribute.
        const BOLD = 1;
        /// Underline text attribute.
        const UNDERLINE = 2;
        /// Blink text attribute.
        const BLINK = 4;
        /// Inverse (reverse video) text attribute.
        const INVERSE = 8;
    }
}

impl IntoSeq for CursorAttributes {
    fn into_seq(&self) -> impl WriteSeq {
        char::from(0x40 + self.bits())
    }
}

impl From<u8> for CursorAttributes {
    fn from(value: u8) -> Self {
        CursorAttributes::from_bits_truncate(value.saturating_sub(0x40))
    }
}

impl From<EscapeSequenceParam> for CursorAttributes {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

impl From<&EscapeSequenceParam> for CursorAttributes {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

bitflags! {
    /// Cursor state flags for cursor information report.
    ///
    /// These flags encode various cursor and terminal state information.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct CursorStateFlags: u8 {
        /// Cursor origin mode is set.
        const ORIGIN_MODE = 1;
        /// Single shift for G2 is active.
        const SINGLE_SHIFT_G2 = 2;
        /// Single shift for G3 is active.
        const SINGLE_SHIFT_G3 = 4;
        /// Pending wrap is set.
        const PENDING_WRAP = 8;
    }
}

impl IntoSeq for CursorStateFlags {
    fn into_seq(&self) -> impl WriteSeq {
        char::from(0x40 + self.bits())
    }
}

impl From<u8> for CursorStateFlags {
    fn from(value: u8) -> Self {
        CursorStateFlags::from_bits_truncate(value.saturating_sub(0x40))
    }
}

impl From<EscapeSequenceParam> for CursorStateFlags {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

impl From<&EscapeSequenceParam> for CursorStateFlags {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

bitflags! {
    /// Character set sizes for cursor information report (Scss).
    ///
    /// Indicates whether each G0-G3 character set has 94 or 96 characters.
    /// The base value has bit 7 set (0x40), and bits 1-4 indicate which
    /// sets have 96 characters (0 = 94 characters, 1 = 96 characters).
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct CharacterSetSizes: u8 {
        /// G0 character set has 96 characters (otherwise 94).
        const G0_96 = 0x01;
        /// G1 character set has 96 characters (otherwise 94).
        const G1_96 = 0x02;
        /// G2 character set has 96 characters (otherwise 94).
        const G2_96 = 0x04;
        /// G3 character set has 96 characters (otherwise 94).
        const G3_96 = 0x08;
    }
}

impl CharacterSetSizes {
    /// Base value with bit 7 set (required by VT510 protocol).
    const BASE: u8 = 0x40;

    /// Convert to the protocol character value.
    ///
    /// Add the base value (0x40) to encode as a character.
    #[must_use]
    pub const fn to_char(self) -> char {
        (Self::BASE | self.bits()) as char
    }

    /// Create from a protocol character value.
    ///
    /// Extract the size bits by removing the base value.
    #[must_use]
    pub const fn from_char(c: char) -> Self {
        Self::from_bits_truncate((c as u8) & !Self::BASE)
    }

    /// Create with all character sets having 94 characters.
    ///
    /// This is the most common configuration.
    #[must_use]
    pub const fn all_94() -> Self {
        Self::empty()
    }

    /// Create with all character sets having 96 characters.
    #[must_use]
    pub const fn all_96() -> Self {
        Self::from_bits_truncate(
            Self::G0_96.bits() | Self::G1_96.bits() | Self::G2_96.bits() | Self::G3_96.bits(),
        )
    }
}

impl From<char> for CharacterSetSizes {
    fn from(c: char) -> Self {
        Self::from_char(c)
    }
}

impl From<u8> for CharacterSetSizes {
    fn from(value: u8) -> Self {
        Self::from_char(value as char)
    }
}

impl IntoSeq for CharacterSetSizes {
    fn into_seq(&self) -> impl WriteSeq {
        self.to_char()
    }
}

impl From<EscapeSequenceParam> for CharacterSetSizes {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from_bits_truncate(param.first())
    }
}

impl From<&EscapeSequenceParam> for CharacterSetSizes {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from_bits_truncate(param.first())
    }
}

/// Request Cursor Information Report (`DECCIR`).
///
/// Response from the terminal to [`RequestCursorInformationReport`].
///
/// Contains detailed information about the cursor state including
/// position, attributes, protection, flags, and character set
/// configuration.
///
/// The report is encoded as a DCS sequence with the format:
/// `DCS 1 $ u Pr; Pc; Pp; Srend; Satt; Sflag; Pgl; Pgr; Scss; Sdesig ST`
///
/// See <https://vt100.net/docs/vt510-rm/DECCIR> for the VT510 specification.
/// See <https://terminalguide.namepad.de/seq/csi_sw_t_dollar-1/> for
/// terminal support specifics.
#[derive(Debug, Clone, PartialEq, Eq, VTControl)]
#[dcs(params = ["1"], intermediate = "$", finalbyte = 'u')]
pub struct CursorInformationReport {
    /// Cursor row position (Pr).
    pub row: u16,
    /// Cursor column position (Pc).
    pub col: u16,
    /// Current page number (Pp).
    pub page: u8,
    /// Current text attributes (Srend).
    ///
    /// Visual attributes such as bold, underline, blink, and reverse video.
    pub attributes: CursorAttributes,
    /// Character protection attribute (Satt).
    ///
    /// Indicates selective erase protection status.
    pub protection_char: char,
    /// Cursor state flags (Sflag).
    ///
    /// Includes origin mode, single shift settings, and autowrap pending.
    pub flags: CursorStateFlags,
    /// Character set invoked into GL (Pgl): 0-3 for G0-G3.
    pub gl: u8,
    /// Character set invoked into GR (Pgr): 0-3 for G0-G3.
    pub gr: u8,
    /// Character set sizes (Scss).
    ///
    /// Indicates whether each G0-G3 set has 94 or 96 characters.
    pub charset_sizes: CharacterSetSizes,
    /// Character set designations (Sdesig).
    ///
    /// String of intermediate and final characters indicating the character
    /// sets designated as G0 through G3.
    pub gsets: String,
}

impl CursorInformationReport {
    /// Create a new cursor information report with the specified
    /// parameters.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        row: u16,
        col: u16,
        page: u8,
        attributes: CursorAttributes,
        protected: bool,
        flags: CursorStateFlags,
        gl: u8,
        gr: u8,
        charset_sizes: CharacterSetSizes,
        gsets: String,
    ) -> Self {
        Self {
            row,
            col,
            page,
            attributes,
            protection_char: if protected { 'A' } else { '@' },
            flags,
            gl,
            gr,
            charset_sizes,
            gsets,
        }
    }

    /// Check if character protection is enabled.
    #[must_use]
    pub const fn protected(&self) -> bool {
        matches!(self.protection_char, 'A')
    }
}

/// Request Tab Stop Report (`DECTABSR`).
///
/// Request a report of the currently set tab stops.
///
/// The terminal replies with a DCS sequence containing the column
/// numbers of all set tab stops, separated by forward slashes (/).
///
/// All explicitly set tab stops and default tab stops that fit within
/// the current terminal width are reported.
///
/// See <https://terminalguide.namepad.de/seq/csi_sw_t_dollar-2/> for
/// terminal support specifics.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, VTControl)]
#[csi(params = ["2"], intermediate = "$", finalbyte = 'w')]
pub struct RequestTabStopReport;

/// Tab Stop Report (`DECTABSR`).
///
/// Response from the terminal to [`RequestTabStopReport`].
///
/// Contains the column numbers of all currently set tab stops.
///
/// The report is encoded as a DCS sequence with the format:
/// `DCS 2 $ u <data> ST` where data is tab stops separated by `/`.
///
/// Example: `DCS 2 $ u 9/17/25/33 ST`
///
/// See <https://vt100.net/docs/vt510-rm/DECTABSR> for the VT510 specification.
/// See <https://terminalguide.namepad.de/seq/csi_sw_t_dollar-2/> for
/// terminal support specifics.
#[derive(Debug, Clone, PartialEq, Eq, VTControl)]
#[dcs(params = ["2"], intermediate = "$", finalbyte = 'u')]
pub struct TabStopReport {
    /// Tab stop column positions, encoded as slash-separated string.
    pub tab_stops: TabStops,
}

/// Tab stops wrapper for encoding.
///
/// Encodes a vector of tab stop positions as a slash-separated string.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TabStops(pub Vec<u16>);

impl TabStops {
    /// Create from a vector of tab stop positions.
    #[must_use]
    pub fn new(stops: Vec<u16>) -> Self {
        Self(stops)
    }

    /// Create from a slice of tab stop positions.
    #[must_use]
    pub fn from_slice(stops: &[u16]) -> Self {
        Self(stops.to_vec())
    }
}

impl IntoSeq for TabStops {
    fn into_seq(&self) -> impl WriteSeq {
        self.0
            .iter()
            .map(u16::to_string)
            .collect::<Vec<String>>()
            .join("/")
    }
}

impl From<Vec<u16>> for TabStops {
    fn from(stops: Vec<u16>) -> Self {
        Self(stops)
    }
}

impl From<EscapeSequenceParam> for TabStops {
    fn from(param: EscapeSequenceParam) -> Self {
        // TabStops is a Vec<u16> wrapper, parse as String with slash separators
        let s = String::from_utf8_lossy(&param);
        let positions: Vec<u16> = s.split('/').filter_map(|s| s.parse().ok()).collect();
        Self(positions)
    }
}

impl From<&EscapeSequenceParam> for TabStops {
    fn from(param: &EscapeSequenceParam) -> Self {
        // TabStops is a Vec<u16> wrapper, parse as String with slash separators
        let s = String::from_utf8_lossy(param);
        let positions: Vec<u16> = s.split('/').filter_map(|s| s.parse().ok()).collect();
        Self(positions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vtenc::encode::Encode;

    #[test]
    fn test_cursor_information_report_encoding() {
        // Test encoding of DECCIR (Cursor Information Report)
        // Format: DCS 1 $ u Pr; Pc; Pp; Srend; Satt; Sflag; Pgl; Pgr; Scss; Sdesig ST
        //
        // This test creates a report matching the VT510 documentation example:
        // - Cursor at row 10, column 20, page 1
        // - No visual attributes set (Srend = '@' = 0x40)
        // - No protection attributes (Satt = '@' = 0x40)
        // - No flags set: DECOM reset, no SS2/SS3, no autowrap pending (Sflag = '@' = 0x40)
        // - G0 mapped into GL (Pgl = 0)
        // - G2 mapped into GR (Pgr = 2)
        // - All character sets have 94 characters (Scss = '@' = 0x40)
        // - Character set designations: ASCII in G0/G1, DEC Supplemental in G2/G3
        //
        // Expected output: DCS 1 $ u 10;20;1;@;@;@;0;2;@;BB%5%5 ST
        let mut report = CursorInformationReport::from_parts(
            10,                          // Pr (row)
            20,                          // Pc (column)
            1,                           // Pp (page)
            CursorAttributes::empty(),   // Srend (no visual attributes)
            false,                       // Satt (not protected = '@')
            CursorStateFlags::empty(),   // Sflag (no flags)
            0,                           // Pgl (G0 in GL)
            2,                           // Pgr (G2 in GR)
            CharacterSetSizes::all_94(), // Scss (all 94-char sets)
            "BB%5%5".to_string(),        // Sdesig (ASCII in G0/G1, DEC Supp in G2/G3)
        );

        let mut buf = Vec::new();
        let len = report.encode(&mut buf).unwrap();

        // Expected format: ESC P 1 $ u Pr; Pc; Pp; Srend; Satt; Sflag; Pgl; Pgr; Scss; Sdesig ESC \
        // ESC = 0x1B, P = 0x50, 1 = 0x31 (param), $ = 0x24, u = 0x75, \ = 0x5C
        assert_eq!(buf[0], 0x1B, "Should start with ESC");
        assert_eq!(buf[1], 0x50, "Should have P (DCS)");
        assert_eq!(buf[2], b'1', "Should have param '1'");
        assert_eq!(buf[3], 0x24, "Should have $ (intermediate)");
        assert_eq!(buf[4], 0x75, "Should have u (final byte)");

        // Check that the sequence contains the expected data
        let output = String::from_utf8_lossy(&buf);
        assert!(output.contains("10;"), "Should contain row (10)");
        assert!(output.contains(";20;"), "Should contain col (20)");
        assert!(output.contains(";1;"), "Should contain page (1)");
        assert!(
            output.contains(";@;"),
            "Should contain @ for attributes/protection/flags"
        );
        assert!(output.contains("0;2"), "Should contain gl (0) and gr (2)");
        assert!(output.contains("BB%5%5"), "Should contain gsets (BB%5%5)");

        // Check that it ends with ST (ESC \)
        assert_eq!(buf[len - 2], 0x1B, "Should end with ESC");
        assert_eq!(buf[len - 1], 0x5C, "Should end with backslash (ST)");

        // Verify the full expected format
        // DCS 1 $ u 10;20;1;@;@;@;0;2;@;BB%5%5 ST
        let expected_data = "10;20;1;@;@;@;0;2;@;BB%5%5";
        assert!(
            output.contains(expected_data),
            "Should contain expected data format, got: {output:?}",
        );

        // Verify length is reasonable
        assert!(len > 20, "Encoded length should be substantial");
    }

    #[test]
    fn test_tab_stop_report_encoding() {
        // Test encoding of DECTABSR (Tab Stop Report)
        // Format: DCS 2 $ u <data> ST where data is tab stops separated by /
        //
        // This test creates a report matching the VT510 documentation example:
        // - Tab stops at columns 9, 17, 25, 33, 41, 49, 57, 65, 73
        //
        // Expected output: DCS 2 $ u 9/17/25/33/41/49/57/65/73 ST
        let mut report = TabStopReport::new(TabStops::new(vec![9, 17, 25, 33, 41, 49, 57, 65, 73]));

        let mut buf = Vec::new();
        let len = report.encode(&mut buf).unwrap();

        // Expected format: ESC P 2 $ u <data> ESC \
        // ESC = 0x1B, P = 0x50, 2 = 0x32 (param), $ = 0x24, u = 0x75, \ = 0x5C
        assert_eq!(buf[0], 0x1B, "Should start with ESC");
        assert_eq!(buf[1], 0x50, "Should have P (DCS)");
        assert_eq!(buf[2], b'2', "Should have param '2'");
        assert_eq!(buf[3], 0x24, "Should have $ (intermediate)");
        assert_eq!(buf[4], 0x75, "Should have u (final byte)");

        // Check that the sequence contains the expected data
        let output = String::from_utf8_lossy(&buf);
        assert!(
            output.contains("9/17/25/33"),
            "Should contain tab stops separated by /"
        );
        assert!(
            output.contains("41/49/57/65/73"),
            "Should contain remaining tab stops"
        );

        // Check that it ends with ST (ESC \)
        assert_eq!(buf[len - 2], 0x1B, "Should end with ESC");
        assert_eq!(buf[len - 1], 0x5C, "Should end with backslash (ST)");

        // Verify the full expected format
        // DCS 2 $ u 9/17/25/33/41/49/57/65/73 ST
        let expected_data = "9/17/25/33/41/49/57/65/73";
        assert!(
            output.contains(expected_data),
            "Should contain expected data format, got: {output:?}",
        );

        // Verify length is reasonable
        assert!(len > 20, "Encoded length should be substantial");
    }
}
