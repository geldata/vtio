//! Mouse mode control commands.

use crate::terminal_mode;
use vtenc::const_composite;

//
// Mouse event modes (mutually exclusive).
//

terminal_mode!(
    #[doc = "Mouse click only tracking. Send mouse button press for left, middle, and right mouse buttons."]
    MouseX10Mode,
    "?9"
);

terminal_mode!(
    #[doc = "Mouse down+up+scroll tracking (button down+up and scroll events)."]
    MouseDownUpTrackingMode,
    "?1000"
);

terminal_mode!(
    #[doc = "Mouse highlight mode (xterm-only)."]
    MouseHighlightMode,
    "?1001"
);

terminal_mode!(
    #[doc = "Button-event tracking (report button motion/dragging)."]
    MouseClickAndDragTrackingMode,
    "?1002"
);

terminal_mode!(
    #[doc = "Any-event tracking (report all motion events)."]
    MouseAnyEventTrackingMode,
    "?1003"
);

//
// Mouse reporting format modes (mutually exclusive).
//

terminal_mode!(
    #[doc = "Mouse report format multibyte mode"]
    MouseReportMultibyteMode,
    "?1005"
);

terminal_mode!(
    #[doc = "SGR mouse mode (extended coordinates >223, preferred)."]
    MouseReportSgrMode,
    "?1006"
);

terminal_mode!(
    #[doc = "RXVT mouse mode (extended coordinates >223, less preferred)."]
    MouseReportRxvtMode,
    "?1015"
);


const_composite! {
    /// A command that enables mouse event capture.
    ///
    /// This command enables all mouse tracking modes and coordinate encoding
    /// formats for comprehensive mouse support.
    pub struct EnableMouseCapture = [
        EnableMouseDownUpTrackingMode,
        EnableMouseClickAndDragTrackingMode,
        EnableMouseAnyEventTrackingMode,
        EnableMouseReportRxvtMode,
        EnableMouseReportSgrMode,
    ];
}

const_composite! {
    /// A command that disables mouse event capture.
    ///
    /// This command disables all mouse tracking modes and coordinate encoding
    /// formats. The modes are disabled in reverse order of enablement.
    pub struct DisableMouseCapture = [
        DisableMouseReportSgrMode,
        DisableMouseReportRxvtMode,
        DisableMouseAnyEventTrackingMode,
        DisableMouseClickAndDragTrackingMode,
        DisableMouseDownUpTrackingMode,
    ];
}
