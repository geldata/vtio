//! Window control commands.

use vtenc::{ConstEncode, ConstEncodedLen, Encode, EncodeError, csi, write_csi, write_osc};

/// Title stack target.
///
/// Specifies which title(s) to push or pop from the stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TitleStackTarget {
    /// Icon name and window title.
    Both,
    /// Icon name only.
    IconName,
    /// Window title only.
    WindowTitle,
    /// Other unsupported value.
    Other(u8),
}

impl TitleStackTarget {
    #[inline]
    fn as_u8(self) -> u8 {
        match self {
            TitleStackTarget::Both => 0,
            TitleStackTarget::IconName => 1,
            TitleStackTarget::WindowTitle => 2,
            TitleStackTarget::Other(n) => n,
        }
    }
}

/// Maximization mode.
///
/// Specifies how to maximize the terminal window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaximizeMode {
    /// Restore (un-maximize) the window.
    Restore,
    /// Maximize the window.
    Maximize,
    /// Maximize vertically only.
    MaximizeVertically,
    /// Maximize horizontally only.
    MaximizeHorizontally,
    /// Other unsupported value.
    Other(u8),
}

impl MaximizeMode {
    #[inline]
    fn as_u8(self) -> u8 {
        match self {
            MaximizeMode::Restore => 0,
            MaximizeMode::Maximize => 1,
            MaximizeMode::MaximizeVertically => 2,
            MaximizeMode::MaximizeHorizontally => 3,
            MaximizeMode::Other(n) => n,
        }
    }
}

/// Coordinate system for window position reporting.
///
/// Specifies the coordinate system to use when reporting window position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionCoordinateSystem {
    /// Relative to display.
    Display,
    /// Relative to accessible desktop area.
    AccessibleDesktop,
    /// Other unsupported value.
    Other(u8),
}

impl PositionCoordinateSystem {
    #[inline]
    fn as_u8(self) -> u8 {
        match self {
            PositionCoordinateSystem::Display => 2,
            PositionCoordinateSystem::AccessibleDesktop => 3,
            PositionCoordinateSystem::Other(n) => n,
        }
    }
}

/// Size selector for window size reporting.
///
/// Specifies what size to report when requesting window size in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SizeSelector {
    /// Window size.
    Window,
    /// Accessible desktop area.
    AccessibleDesktop,
    /// Other unsupported value.
    Other(u8),
}

impl SizeSelector {
    #[inline]
    fn as_u8(self) -> u8 {
        match self {
            SizeSelector::Window => 2,
            SizeSelector::AccessibleDesktop => 3,
            SizeSelector::Other(n) => n,
        }
    }
}

/// Set terminal window title and icon name.
///
/// Sets both the window title and icon name to the same string.
///
/// See <https://terminalguide.namepad.de/seq/osc-0/> for
/// terminal support specifics.
pub struct SetTitleAndIconName<'a>(pub &'a str);

impl Encode for SetTitleAndIconName<'_> {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        // OSC 0 ; title ST
        write_osc!(buf; "0;", self.0)
    }
}

/// Set terminal window title.
///
/// Sets the window title displayed in the title bar.
///
/// See <https://terminalguide.namepad.de/seq/osc-2/> for
/// terminal support specifics.
pub struct SetTitle<'a>(pub &'a str);

impl Encode for SetTitle<'_> {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        // OSC 2 ; title ST
        write_osc!(buf; "2;", self.0)
    }
}

/// Set icon name.
///
/// Sets the icon name (or icon title) for the terminal window.
///
/// See <https://terminalguide.namepad.de/seq/osc-1/> for
/// terminal support specifics.
pub struct SetIconName<'a>(pub &'a str);

impl Encode for SetIconName<'_> {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        // OSC 1 ; name ST
        write_osc!(buf; "1;", self.0)
    }
}

/// Get terminal window title.
///
/// Request the current window title. The terminal responds with
/// `OSC 2 ; title ST` or `OSC l title ST`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-21/> for
/// terminal support specifics.
pub struct GetTitle;

impl ConstEncode for GetTitle {
    const STR: &'static str = csi!("21t");
}

/// Get icon name.
///
/// Request the current icon name. The terminal responds with
/// `OSC 1 ; name ST` or `OSC L name ST`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-20/> for
/// terminal support specifics.
pub struct GetIconName;

impl ConstEncode for GetIconName {
    const STR: &'static str = csi!("20t");
}

/// Push terminal title onto stack.
///
/// Push the current title onto an internal stack. The optional
/// parameter specifies which title to push.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-22/> for
/// terminal support specifics.
pub struct PushTitle {
    /// Which title to push.
    pub which: Option<TitleStackTarget>,
}

impl ConstEncodedLen for PushTitle {
    // CSI (2) + "22" (2) + ";" (1) + max u8 digits (3) + "t" (1) = 9
    const ENCODED_LEN: usize = 9;
}

impl Encode for PushTitle {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        if let Some(which) = self.which {
            write_csi!(buf; "22;", which.as_u8(), "t")
        } else {
            write_csi!(buf; "22t")
        }
    }
}

/// Pop terminal title from stack.
///
/// Pop a title from the internal stack and set it as the current title.
/// The optional parameter specifies which title to pop.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-23/> for
/// terminal support specifics.
pub struct PopTitle {
    /// Which title to pop.
    pub which: Option<TitleStackTarget>,
}

impl ConstEncodedLen for PopTitle {
    // CSI (2) + "23" (2) + ";" (1) + max u8 digits (3) + "t" (1) = 9
    const ENCODED_LEN: usize = 9;
}

impl Encode for PopTitle {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        if let Some(which) = self.which {
            write_csi!(buf; "23;", which.as_u8(), "t")
        } else {
            write_csi!(buf; "23t")
        }
    }
}

/// Restore terminal window.
///
/// Restore (de-iconify) the terminal window.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-1/> for
/// terminal support specifics.
pub struct RestoreWindow;

impl ConstEncode for RestoreWindow {
    const STR: &'static str = csi!("1t");
}

/// Minimize terminal window.
///
/// Minimize (iconify) the terminal window.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-2/> for
/// terminal support specifics.
pub struct MinimizeWindow;

impl ConstEncode for MinimizeWindow {
    const STR: &'static str = csi!("2t");
}

/// Raise terminal window.
///
/// Raise the terminal window to the front of the stacking order.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-5/> for
/// terminal support specifics.
pub struct RaiseWindow;

impl ConstEncode for RaiseWindow {
    const STR: &'static str = csi!("5t");
}

/// Lower terminal window.
///
/// Lower the terminal window to the back of the stacking order.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-6/> for
/// terminal support specifics.
pub struct LowerWindow;

impl ConstEncode for LowerWindow {
    const STR: &'static str = csi!("6t");
}

/// Refresh terminal window.
///
/// Refresh (redraw) the terminal window.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-7/> for
/// terminal support specifics.
pub struct RefreshWindow;

impl ConstEncode for RefreshWindow {
    const STR: &'static str = csi!("7t");
}

/// Set terminal window position.
///
/// Move the terminal window to the specified position in pixels,
/// relative to the upper-left corner of the screen.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-3/> for
/// terminal support specifics.
pub struct SetWindowPosition {
    /// X coordinate in pixels.
    pub x: u16,
    /// Y coordinate in pixels.
    pub y: u16,
}

impl ConstEncodedLen for SetWindowPosition {
    // CSI (2) + "3" (1) + ";" (1) + max u16 digits (5) + ";" (1) +
    // max u16 digits (5) + "t" (1) = 16
    const ENCODED_LEN: usize = 16;
}

impl Encode for SetWindowPosition {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "3;", self.x, ";", self.y, "t")
    }
}

/// Set terminal window size in pixels.
///
/// Resize the terminal window to the specified size in pixels.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-4/> for
/// terminal support specifics.
pub struct SetWindowSizePixels {
    /// Height in pixels.
    pub height: u16,
    /// Width in pixels.
    pub width: u16,
}

impl ConstEncodedLen for SetWindowSizePixels {
    // CSI (2) + "4" (1) + ";" (1) + max u16 digits (5) + ";" (1) +
    // max u16 digits (5) + "t" (1) = 16
    const ENCODED_LEN: usize = 16;
}

impl Encode for SetWindowSizePixels {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "4;", self.height, ";", self.width, "t")
    }
}

/// Set terminal size in character cells.
///
/// Resize the terminal window to the specified size in character cells
/// (rows and columns).
///
/// See <https://terminalguide.namepad.de/seq/csi_st-8/> for
/// terminal support specifics.
pub struct SetSize {
    /// Number of rows.
    pub rows: u16,
    /// Number of columns.
    pub cols: u16,
}

impl ConstEncodedLen for SetSize {
    // CSI (2) + "8" (1) + ";" (1) + max u16 digits (5) + ";" (1) +
    // max u16 digits (5) + "t" (1) = 16
    const ENCODED_LEN: usize = 16;
}

impl Encode for SetSize {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "8;", self.rows, ";", self.cols, "t")
    }
}

/// Maximize terminal window.
///
/// Maximize the terminal window. The parameter specifies the
/// maximization mode.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-9/> for
/// terminal support specifics.
pub struct MaximizeWindow {
    /// Maximization mode.
    pub mode: MaximizeMode,
}

impl ConstEncodedLen for MaximizeWindow {
    // CSI (2) + "9" (1) + ";" (1) + max u8 digits (3) + "t" (1) = 8
    const ENCODED_LEN: usize = 8;
}

impl Encode for MaximizeWindow {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "9;", self.mode.as_u8(), "t")
    }
}

/// Maximize terminal window (alternate form).
///
/// Alternate sequence for maximizing the terminal window.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-10/> for
/// terminal support specifics.
pub struct MaximizeWindowAlt {
    /// Maximization mode.
    pub mode: MaximizeMode,
}

impl ConstEncodedLen for MaximizeWindowAlt {
    // CSI (2) + "10" (2) + ";" (1) + max u8 digits (3) + "t" (1) = 9
    const ENCODED_LEN: usize = 9;
}

impl Encode for MaximizeWindowAlt {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "10;", self.mode.as_u8(), "t")
    }
}

/// Report terminal window state.
///
/// Request whether the terminal window is iconified or not.
/// The terminal responds with `CSI 1 t` if not iconified or
/// `CSI 2 t` if iconified.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-11/> for
/// terminal support specifics.
pub struct ReportWindowState;

impl ConstEncode for ReportWindowState {
    const STR: &'static str = csi!("11t");
}

/// Report terminal window position.
///
/// Request the terminal window position in pixels.
/// The terminal responds with `CSI 3 ; x ; y t`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-13/> for
/// terminal support specifics.
pub struct ReportWindowPosition {
    /// Coordinate system selector.
    pub mode: Option<PositionCoordinateSystem>,
}

impl ConstEncodedLen for ReportWindowPosition {
    // CSI (2) + "13" (2) + ";" (1) + max u8 digits (3) + "t" (1) = 9
    const ENCODED_LEN: usize = 9;
}

impl Encode for ReportWindowPosition {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        if let Some(mode) = self.mode {
            write_csi!(buf; "13;", mode.as_u8(), "t")
        } else {
            write_csi!(buf; "13t")
        }
    }
}

/// Report terminal window size in pixels.
///
/// Request the terminal window size in pixels.
/// The terminal responds with `CSI 4 ; height ; width t`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-14/> for
/// terminal support specifics.
pub struct ReportWindowSizePixels {
    /// Size selector.
    pub mode: Option<SizeSelector>,
}

impl ConstEncodedLen for ReportWindowSizePixels {
    // CSI (2) + "14" (2) + ";" (1) + max u8 digits (3) + "t" (1) = 9
    const ENCODED_LEN: usize = 9;
}

impl Encode for ReportWindowSizePixels {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        if let Some(mode) = self.mode {
            write_csi!(buf; "14;", mode.as_u8(), "t")
        } else {
            write_csi!(buf; "14t")
        }
    }
}

/// Report screen size in pixels.
///
/// Request the screen size in pixels.
/// The terminal responds with `CSI 5 ; height ; width t`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-15/> for
/// terminal support specifics.
pub struct ReportScreenSizePixels;

impl ConstEncode for ReportScreenSizePixels {
    const STR: &'static str = csi!("15t");
}

/// Report cell size in pixels.
///
/// Request the character cell size in pixels.
/// The terminal responds with `CSI 6 ; height ; width t`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-16/> for
/// terminal support specifics.
pub struct ReportCellSizePixels;

impl ConstEncode for ReportCellSizePixels {
    const STR: &'static str = csi!("16t");
}

/// Report terminal size in character cells.
///
/// Request the terminal size in character cells (rows and columns).
/// The terminal responds with `CSI 8 ; rows ; cols t`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-18/> for
/// terminal support specifics.
pub struct ReportSize;

impl ConstEncode for ReportSize {
    const STR: &'static str = csi!("18t");
}

/// Report screen size in character cells.
///
/// Request the screen size in character cells.
/// The terminal responds with `CSI 9 ; rows ; cols t`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-19/> for
/// terminal support specifics.
pub struct ReportScreenSize;

impl ConstEncode for ReportScreenSize {
    const STR: &'static str = csi!("19t");
}

/// Window state report response.
///
/// Response to [`ReportWindowState`] request.
/// Indicates whether the window is iconified or not.
pub enum WindowStateReport {
    /// Window is not iconified (normal state).
    NotIconified,
    /// Window is iconified (minimized).
    Iconified,
}

impl ConstEncodedLen for WindowStateReport {
    // CSI (2) + "1" or "2" (1) + "t" (1) = 4
    const ENCODED_LEN: usize = 4;
}

impl Encode for WindowStateReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        match self {
            WindowStateReport::NotIconified => write_csi!(buf; "1t"),
            WindowStateReport::Iconified => write_csi!(buf; "2t"),
        }
    }
}

/// Window position report.
///
/// Response to [`ReportWindowPosition`] request.
/// Reports the window position in pixels.
pub struct WindowPositionReport {
    /// X coordinate in pixels.
    pub x: u16,
    /// Y coordinate in pixels.
    pub y: u16,
}

impl ConstEncodedLen for WindowPositionReport {
    // CSI (2) + "3" (1) + ";" (1) + max u16 digits (5) + ";" (1) +
    // max u16 digits (5) + "t" (1) = 16
    const ENCODED_LEN: usize = 16;
}

impl Encode for WindowPositionReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "3;", self.x, ";", self.y, "t")
    }
}

/// Window size in pixels report.
///
/// Response to [`ReportWindowSizePixels`] request.
/// Reports the window size in pixels.
pub struct WindowSizePixelsReport {
    /// Height in pixels.
    pub height: u16,
    /// Width in pixels.
    pub width: u16,
}

impl ConstEncodedLen for WindowSizePixelsReport {
    // CSI (2) + "4" (1) + ";" (1) + max u16 digits (5) + ";" (1) +
    // max u16 digits (5) + "t" (1) = 16
    const ENCODED_LEN: usize = 16;
}

impl Encode for WindowSizePixelsReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "4;", self.height, ";", self.width, "t")
    }
}

/// Screen size in pixels report.
///
/// Response to [`ReportScreenSizePixels`] request.
/// Reports the screen size in pixels.
pub struct ScreenSizePixelsReport {
    /// Height in pixels.
    pub height: u16,
    /// Width in pixels.
    pub width: u16,
}

impl ConstEncodedLen for ScreenSizePixelsReport {
    // CSI (2) + "5" (1) + ";" (1) + max u16 digits (5) + ";" (1) +
    // max u16 digits (5) + "t" (1) = 16
    const ENCODED_LEN: usize = 16;
}

impl Encode for ScreenSizePixelsReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "5;", self.height, ";", self.width, "t")
    }
}

/// Cell size in pixels report.
///
/// Response to [`ReportCellSizePixels`] request.
/// Reports the character cell size in pixels.
pub struct CellSizePixelsReport {
    /// Cell height in pixels.
    pub height: u16,
    /// Cell width in pixels.
    pub width: u16,
}

impl ConstEncodedLen for CellSizePixelsReport {
    // CSI (2) + "6" (1) + ";" (1) + max u16 digits (5) + ";" (1) +
    // max u16 digits (5) + "t" (1) = 16
    const ENCODED_LEN: usize = 16;
}

impl Encode for CellSizePixelsReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "6;", self.height, ";", self.width, "t")
    }
}

/// Terminal size report.
///
/// Response to [`ReportSize`] request.
/// Reports the terminal size in character cells (rows and columns).
pub struct SizeReport {
    /// Number of rows.
    pub rows: u16,
    /// Number of columns.
    pub cols: u16,
}

impl ConstEncodedLen for SizeReport {
    // CSI (2) + "8" (1) + ";" (1) + max u16 digits (5) + ";" (1) +
    // max u16 digits (5) + "t" (1) = 16
    const ENCODED_LEN: usize = 16;
}

impl Encode for SizeReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "8;", self.rows, ";", self.cols, "t")
    }
}

/// Screen size report.
///
/// Response to [`ReportScreenSize`] request.
/// Reports the screen size in character cells.
pub struct ScreenSizeReport {
    /// Number of rows.
    pub rows: u16,
    /// Number of columns.
    pub cols: u16,
}

impl ConstEncodedLen for ScreenSizeReport {
    // CSI (2) + "9" (1) + ";" (1) + max u16 digits (5) + ";" (1) +
    // max u16 digits (5) + "t" (1) = 16
    const ENCODED_LEN: usize = 16;
}

impl Encode for ScreenSizeReport {
    #[inline]
    fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, EncodeError> {
        write_csi!(buf; "9;", self.rows, ";", self.cols, "t")
    }
}
