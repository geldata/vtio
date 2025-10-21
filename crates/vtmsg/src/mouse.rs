//! Mouse mode control commands.

use vtenc::{ConstEncode, Encode, EncodeError, csi, write_csi};

use crate::terminal_mode;

/// Define a composite command that combines multiple sub-commands.
///
/// This macro generates a struct and its `Encode` implementation that
/// sequentially encodes multiple commands.
macro_rules! composite_command {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident = [
            $($command:expr),* $(,)?
        ];
    ) => {
        $(#[$meta])*
        $vis struct $name;

        impl Encode for $name {
            #[inline]
            fn encode<W: std::io::Write>(
                &mut self,
                buf: &mut W
            ) -> Result<usize, EncodeError> {
                let mut total = 0;
                $(
                    total += $command.encode(buf)?;
                )*
                Ok(total)
            }
        }
    };
}

terminal_mode!(
    #[doc = "Normal mouse tracking (send X & Y on button press/release)."]
    MouseTracking,
    1000
);

terminal_mode!(
    #[doc = "Button-event tracking (report button motion/dragging)."]
    MouseButtonEventTracking,
    1002
);

terminal_mode!(
    #[doc = "Any-event tracking (report all motion events)."]
    MouseAnyEventTracking,
    1003
);

terminal_mode!(
    #[doc = "RXVT mouse mode (extended coordinates >223, less preferred)."]
    MouseRxvtMode,
    1015
);

terminal_mode!(
    #[doc = "SGR mouse mode (extended coordinates >223, preferred)."]
    MouseSgrMode,
    1006
);

composite_command! {
    /// A command that enables mouse event capture.
    ///
    /// This command enables all mouse tracking modes and coordinate encoding
    /// formats for comprehensive mouse support.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct EnableMouseCapture = [
        EnableMouseTracking,
        EnableMouseButtonEventTracking,
        EnableMouseAnyEventTracking,
        EnableMouseRxvtMode,
        EnableMouseSgrMode,
    ];
}

composite_command! {
    /// A command that disables mouse event capture.
    ///
    /// This command disables all mouse tracking modes and coordinate encoding
    /// formats. The modes are disabled in reverse order of enablement.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DisableMouseCapture = [
        DisableMouseSgrMode,
        DisableMouseRxvtMode,
        DisableMouseAnyEventTracking,
        DisableMouseButtonEventTracking,
        DisableMouseTracking,
    ];
}
