//! Window control commands.

use vtio_control_derive::VTControl;
use vtio_control_base::EscapeSequenceParam;
use vtenc::ToAnsi;

/// Title stack target.
///
/// Specifies which title(s) to push or pop from the stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TitleStackTarget {
    /// Icon name and window title.
    #[default]
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

impl ToAnsi for TitleStackTarget {
    fn to_ansi(&self) -> impl vtenc::AnsiEncode {
        self.as_u8()
    }
}

impl From<u8> for TitleStackTarget {
    fn from(value: u8) -> Self {
        match value {
            0 => TitleStackTarget::Both,
            1 => TitleStackTarget::IconName,
            2 => TitleStackTarget::WindowTitle,
            n => TitleStackTarget::Other(n),
        }
    }
}

impl From<EscapeSequenceParam> for TitleStackTarget {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

impl From<&EscapeSequenceParam> for TitleStackTarget {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

/// Maximization mode.
///
/// Specifies how to maximize the terminal window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MaximizeMode {
    /// Restore (un-maximize) the window.
    #[default]
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

impl ToAnsi for MaximizeMode {
    fn to_ansi(&self) -> impl vtenc::AnsiEncode {
        self.as_u8()
    }
}

impl From<u8> for MaximizeMode {
    fn from(value: u8) -> Self {
        match value {
            0 => MaximizeMode::Restore,
            1 => MaximizeMode::Maximize,
            2 => MaximizeMode::MaximizeVertically,
            3 => MaximizeMode::MaximizeHorizontally,
            n => MaximizeMode::Other(n),
        }
    }
}

impl From<EscapeSequenceParam> for MaximizeMode {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

impl From<&EscapeSequenceParam> for MaximizeMode {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

/// Coordinate system for window position reporting.
///
/// Specifies the coordinate system to use when reporting window position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PositionCoordinateSystem {
    /// Relative to display.
    #[default]
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

impl ToAnsi for PositionCoordinateSystem {
    fn to_ansi(&self) -> impl vtenc::AnsiEncode {
        self.as_u8()
    }
}

impl From<u8> for PositionCoordinateSystem {
    fn from(value: u8) -> Self {
        match value {
            2 => PositionCoordinateSystem::Display,
            3 => PositionCoordinateSystem::AccessibleDesktop,
            n => PositionCoordinateSystem::Other(n),
        }
    }
}

impl From<EscapeSequenceParam> for PositionCoordinateSystem {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

impl From<&EscapeSequenceParam> for PositionCoordinateSystem {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

/// Size selector for window size reporting.
///
/// Specifies what size to report when requesting window size in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SizeSelector {
    /// Window size.
    #[default]
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

impl ToAnsi for SizeSelector {
    fn to_ansi(&self) -> impl vtenc::AnsiEncode {
        self.as_u8()
    }
}

impl From<u8> for SizeSelector {
    fn from(value: u8) -> Self {
        match value {
            2 => SizeSelector::Window,
            3 => SizeSelector::AccessibleDesktop,
            n => SizeSelector::Other(n),
        }
    }
}

impl From<EscapeSequenceParam> for SizeSelector {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

impl From<&EscapeSequenceParam> for SizeSelector {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

/// Set terminal window title and icon name.
///
/// Set both the window title and icon name to the same string.
///
/// See <https://terminalguide.namepad.de/seq/osc-0/> for
/// terminal support specifics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "0")]
pub struct SetTitleAndIconName<'a> {
    pub title: &'a str,
}

/// Set terminal window title.
///
/// Set the window title.
///
/// See <https://terminalguide.namepad.de/seq/osc-2/> for
/// terminal support specifics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "2")]
pub struct SetTitle<'a> {
    pub title: &'a str,
}

/// Set icon name.
///
/// Set the icon name for the terminal window.
///
/// See <https://terminalguide.namepad.de/seq/osc-1/> for
/// terminal support specifics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "1")]
pub struct SetIconName<'a> {
    pub name: &'a str,
}

/// Get terminal window title.
///
/// Request the current window title. The terminal responds with
/// `OSC 2 ; title ST` or `OSC l title ST`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-21/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["21"], finalbyte = 't')]
pub struct GetTitle;

/// Get icon name.
///
/// Request the current icon name. The terminal responds with
/// `OSC 1 ; name ST` or `OSC L name ST`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-20/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["20"], finalbyte = 't')]
pub struct GetIconName;

/// Push terminal title onto stack.
///
/// Push the current title onto an internal stack. The optional
/// parameter specifies which title to push.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-22/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["22"], finalbyte = 't')]
pub struct PushTitle {
    /// Which title to push.
    #[vtctl(positional)]
    pub which: Option<TitleStackTarget>,
}

/// Pop terminal title from stack.
///
/// Pop a title from the internal stack and set it as the current title.
/// The optional parameter specifies which title to pop.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-23/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["23"], finalbyte = 't')]
pub struct PopTitle {
    /// Which title to pop.
    #[vtctl(positional)]
    pub which: Option<TitleStackTarget>,
}

/// Restore terminal window.
///
/// Restore (de-iconify) the terminal window.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-1/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["1"], finalbyte = 't')]
pub struct RestoreWindow;

/// Minimize terminal window.
///
/// Minimize (iconify) the terminal window.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-2/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["2"], finalbyte = 't')]
pub struct MinimizeWindow;

/// Raise terminal window.
///
/// Raise the terminal window to the front of the stacking order.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-5/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["5"], finalbyte = 't')]
pub struct RaiseWindow;

/// Lower terminal window.
///
/// Lower the terminal window to the back of the stacking order.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-6/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["6"], finalbyte = 't')]
pub struct LowerWindow;

/// Refresh terminal window.
///
/// Refresh (redraw) the terminal window.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-7/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["7"], finalbyte = 't')]
pub struct RefreshWindow;

/// Set terminal window position.
///
/// Move the terminal window to the specified position in pixels,
/// relative to the upper-left corner of the screen.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-3/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["3"], finalbyte = 't')]
pub struct SetWindowPosition {
    /// X coordinate in pixels.
    pub x: u16,
    /// Y coordinate in pixels.
    pub y: u16,
}

/// Set terminal window size in pixels.
///
/// Resize the terminal window to the specified size in pixels.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-4/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["4"], finalbyte = 't')]
pub struct SetWindowSizePixels {
    /// Height in pixels.
    pub height: u16,
    /// Width in pixels.
    pub width: u16,
}

/// Set terminal size in character cells.
///
/// Resize the terminal window to the specified size in character cells
/// (rows and columns).
///
/// See <https://terminalguide.namepad.de/seq/csi_st-8/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["8"], finalbyte = 't')]
pub struct SetSize {
    /// Number of rows.
    pub rows: u16,
    /// Number of columns.
    pub cols: u16,
}

/// Maximize terminal window.
///
/// Maximize the terminal window. The parameter specifies the
/// maximization mode.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-9/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["9"], finalbyte = 't')]
pub struct MaximizeWindow {
    /// Maximization mode.
    pub mode: MaximizeMode,
}

/// Maximize terminal window (alternate form).
///
/// Alternate sequence for maximizing the terminal window.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-10/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["10"], finalbyte = 't')]
pub struct MaximizeWindowAlt {
    /// Maximization mode.
    pub mode: MaximizeMode,
}

/// Report terminal window state.
///
/// Request whether the terminal window is iconified or not.
/// The terminal responds with `CSI 1 t` if not iconified or
/// `CSI 2 t` if iconified.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-11/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["11"], finalbyte = 't')]
pub struct ReportWindowState;

/// Report terminal window position.
///
/// Request the terminal window position in pixels.
/// The terminal responds with `CSI 3 ; x ; y t`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-13/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["13"], finalbyte = 't')]
pub struct ReportWindowPosition {
    /// Coordinate system selector.
    #[vtctl(positional)]
    pub mode: Option<PositionCoordinateSystem>,
}

/// Report terminal window size in pixels.
///
/// Request the terminal window size in pixels.
/// The terminal responds with `CSI 4 ; height ; width t`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-14/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["14"], finalbyte = 't')]
pub struct ReportWindowSizePixels {
    /// Size selector.
    #[vtctl(positional)]
    pub mode: Option<SizeSelector>,
}

/// Report screen size in pixels.
///
/// Request the screen size in pixels.
/// The terminal responds with `CSI 5 ; height ; width t`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-15/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["15"], finalbyte = 't')]
pub struct ReportScreenSizePixels;

/// Report cell size in pixels.
///
/// Request the character cell size in pixels.
/// The terminal responds with `CSI 6 ; height ; width t`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-16/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["16"], finalbyte = 't')]
pub struct ReportCellSizePixels;

/// Report terminal size in character cells.
///
/// Request the terminal size in character cells (rows and columns).
/// The terminal responds with `CSI 8 ; rows ; cols t`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-18/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["18"], finalbyte = 't')]
pub struct ReportSize;

/// Report screen size in character cells.
///
/// Request the screen size in character cells.
/// The terminal responds with `CSI 9 ; rows ; cols t`.
///
/// See <https://terminalguide.namepad.de/seq/csi_st-19/> for
/// terminal support specifics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["19"], finalbyte = 't')]
pub struct ReportScreenSize;

/// Window state.
///
/// Indicate whether the window is iconified or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WindowState {
    /// Window is not iconified (normal state).
    #[default]
    NotIconified,
    /// Window is iconified (minimized).
    Iconified,
}

impl ToAnsi for WindowState {
    fn to_ansi(&self) -> impl vtenc::AnsiEncode {
        match self {
            WindowState::NotIconified => 1u8,
            WindowState::Iconified => 2u8,
        }
    }
}

impl From<u8> for WindowState {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::NotIconified,
            2 => Self::Iconified,
            _ => Self::default(),
        }
    }
}

impl From<EscapeSequenceParam> for WindowState {
    fn from(param: EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

impl From<&EscapeSequenceParam> for WindowState {
    fn from(param: &EscapeSequenceParam) -> Self {
        Self::from(param.first())
    }
}

/// Window state report.
///
/// Response to [`ReportWindowState`] request.
/// Report whether the window is iconified or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, finalbyte = 't')]
pub struct WindowStateReport {
    pub state: WindowState,
}

/// Window position report.
///
/// Response to [`ReportWindowPosition`] request.
/// Report the window position in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["3"], finalbyte = 't')]
pub struct WindowPositionReport {
    /// X coordinate in pixels.
    pub x: u16,
    /// Y coordinate in pixels.
    pub y: u16,
}

/// Window size in pixels report.
///
/// Response to [`ReportWindowSizePixels`] request.
/// Report the window size in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["4"], finalbyte = 't')]
pub struct WindowSizePixelsReport {
    /// Height in pixels.
    pub height: u16,
    /// Width in pixels.
    pub width: u16,
}

/// Screen size in pixels report.
///
/// Response to [`ReportScreenSizePixels`] request.
/// Report the screen size in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["5"], finalbyte = 't')]
pub struct ScreenSizePixelsReport {
    /// Height in pixels.
    pub height: u16,
    /// Width in pixels.
    pub width: u16,
}

/// Cell size in pixels report.
///
/// Response to [`ReportCellSizePixels`] request.
/// Report the character cell size in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["6"], finalbyte = 't')]
pub struct CellSizePixelsReport {
    /// Cell height in pixels.
    pub height: u16,
    /// Cell width in pixels.
    pub width: u16,
}

/// Terminal size report.
///
/// Response to [`ReportSize`] request.
/// Report the terminal size in character cells (rows and columns).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["8"], finalbyte = 't')]
pub struct SizeReport {
    /// Number of rows.
    pub rows: u16,
    /// Number of columns.
    pub cols: u16,
}

/// Screen size report.
///
/// Response to [`ReportScreenSize`] request.
/// Report the screen size in character cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(csi, params = ["9"], finalbyte = 't')]
pub struct ScreenSizeReport {
    /// Number of rows.
    pub rows: u16,
    /// Number of columns.
    pub cols: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use vtio_control_base::{StaticAnsiEncode, AnsiEncode2};

    #[test]
    fn test_set_title_and_icon_name() {
        let mut cmd = SetTitleAndIconName {
            title: "Test Title",
        };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(
            String::from_utf8(buf).unwrap(),
            "\x1b]0;Test Title\x1b\\"
        );
    }

    #[test]
    fn test_set_title() {
        let mut cmd = SetTitle {
            title: "Window Title",
        };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(
            String::from_utf8(buf).unwrap(),
            "\x1b]2;Window Title\x1b\\"
        );
    }

    #[test]
    fn test_set_icon_name() {
        let mut cmd = SetIconName { name: "Icon" };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b]1;Icon\x1b\\");
    }

    #[test]
    fn test_get_title() {
        assert_eq!(GetTitle::STR, "\x1b[21t");
    }

    #[test]
    fn test_get_icon_name() {
        assert_eq!(GetIconName::STR, "\x1b[20t");
    }

    #[test]
    fn test_push_title_without_which() {
        let mut cmd = PushTitle { which: None };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[22t");
    }

    #[test]
    fn test_push_title_with_which() {
        let mut cmd = PushTitle {
            which: Some(TitleStackTarget::WindowTitle),
        };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[22;2t");
    }

    #[test]
    fn test_pop_title_without_which() {
        let mut cmd = PopTitle { which: None };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[23t");
    }

    #[test]
    fn test_pop_title_with_which() {
        let mut cmd = PopTitle {
            which: Some(TitleStackTarget::IconName),
        };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[23;1t");
    }

    #[test]
    fn test_restore_window() {
        assert_eq!(RestoreWindow::STR, "\x1b[1t");
    }

    #[test]
    fn test_minimize_window() {
        assert_eq!(MinimizeWindow::STR, "\x1b[2t");
    }

    #[test]
    fn test_raise_window() {
        assert_eq!(RaiseWindow::STR, "\x1b[5t");
    }

    #[test]
    fn test_lower_window() {
        assert_eq!(LowerWindow::STR, "\x1b[6t");
    }

    #[test]
    fn test_refresh_window() {
        assert_eq!(RefreshWindow::STR, "\x1b[7t");
    }

    #[test]
    fn test_set_window_position() {
        let mut cmd = SetWindowPosition { x: 100, y: 200 };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[3;100;200t");
    }

    #[test]
    fn test_set_window_size_pixels() {
        let mut cmd = SetWindowSizePixels {
            height: 600,
            width: 800,
        };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[4;600;800t");
    }

    #[test]
    fn test_set_size() {
        let mut cmd = SetSize { rows: 24, cols: 80 };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[8;24;80t");
    }

    #[test]
    fn test_maximize_window() {
        let mut cmd = MaximizeWindow {
            mode: MaximizeMode::Maximize,
        };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[9;1t");
    }

    #[test]
    fn test_maximize_window_alt() {
        let mut cmd = MaximizeWindowAlt {
            mode: MaximizeMode::MaximizeVertically,
        };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[10;2t");
    }

    #[test]
    fn test_report_window_state() {
        assert_eq!(ReportWindowState::STR, "\x1b[11t");
    }

    #[test]
    fn test_report_window_position_without_mode() {
        let mut cmd = ReportWindowPosition { mode: None };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[13t");
    }

    #[test]
    fn test_report_window_position_with_mode() {
        let mut cmd = ReportWindowPosition {
            mode: Some(PositionCoordinateSystem::AccessibleDesktop),
        };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[13;3t");
    }

    #[test]
    fn test_report_window_size_pixels() {
        let mut cmd = ReportWindowSizePixels {
            mode: Some(SizeSelector::Window),
        };
        let mut buf = Vec::new();
        cmd.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[14;2t");
    }

    #[test]
    fn test_report_screen_size_pixels() {
        assert_eq!(ReportScreenSizePixels::STR, "\x1b[15t");
    }

    #[test]
    fn test_report_cell_size_pixels() {
        assert_eq!(ReportCellSizePixels::STR, "\x1b[16t");
    }

    #[test]
    fn test_report_size() {
        assert_eq!(ReportSize::STR, "\x1b[18t");
    }

    #[test]
    fn test_report_screen_size() {
        assert_eq!(ReportScreenSize::STR, "\x1b[19t");
    }

    #[test]
    fn test_window_state_report_not_iconified() {
        let mut report = WindowStateReport {
            state: WindowState::NotIconified,
        };
        let mut buf = Vec::new();
        report.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[1t");
    }

    #[test]
    fn test_window_state_report_iconified() {
        let mut report = WindowStateReport {
            state: WindowState::Iconified,
        };
        let mut buf = Vec::new();
        report.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[2t");
    }

    #[test]
    fn test_window_position_report() {
        let mut report = WindowPositionReport { x: 50, y: 100 };
        let mut buf = Vec::new();
        report.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[3;50;100t");
    }

    #[test]
    fn test_window_size_pixels_report() {
        let mut report = WindowSizePixelsReport {
            height: 768,
            width: 1024,
        };
        let mut buf = Vec::new();
        report.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[4;768;1024t");
    }

    #[test]
    fn test_screen_size_pixels_report() {
        let mut report = ScreenSizePixelsReport {
            height: 1080,
            width: 1920,
        };
        let mut buf = Vec::new();
        report.encode(&mut buf).unwrap();
        assert_eq!(
String::from_utf8(buf).unwrap(), "\x1b[5;1080;1920t");
    }

    #[test]
    fn test_cell_size_pixels_report() {
        let mut report = CellSizePixelsReport {
            height: 16,
            width: 8,
        };
        let mut buf = Vec::new();
        report.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[6;16;8t");
    }

    #[test]
    fn test_size_report() {
        let mut report = SizeReport { rows: 30, cols: 120 };
        let mut buf = Vec::new();
        report.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[8;30;120t");
    }

    #[test]
    fn test_screen_size_report() {
        let mut report = ScreenSizeReport {
            rows: 40,
            cols: 160,
        };
        let mut buf = Vec::new();
        report.encode(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b[9;40;160t");
    }
}
