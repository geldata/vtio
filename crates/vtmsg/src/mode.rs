//! Mode control commands.

use vtenc::{ConstEncode, csi, esc};

/// A command that enables [bracketed paste mode](https://en.wikipedia.org/wiki/Bracketed-paste).
///
/// The [`DisableBracketedPaste`] command does the inverse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableBracketedPaste;

impl ConstEncode for EnableBracketedPaste {
    const STR: &'static str = csi!("?2004h");
}

/// A command that disables bracketed paste mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableBracketedPaste;

impl ConstEncode for DisableBracketedPaste {
    const STR: &'static str = csi!("?2004l");
}

/// A command that enables focus event emission.
///
/// The [`DisableFocusRecording`] command does the inverse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableFocusReporting;

impl ConstEncode for EnableFocusReporting {
    const STR: &'static str = csi!("?1004h");
}

/// Disable focus reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableFocusReporting;

impl ConstEncode for DisableFocusReporting {
    const STR: &'static str = csi!("?1004l");
}

/// Enable application keypad mode (DECKPAM).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableApplicationKeypad;

impl ConstEncode for EnableApplicationKeypad {
    const STR: &'static str = esc!("=");
}

/// Disable application keypad mode (DECKPNM).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableApplicationKeypad;

impl ConstEncode for DisableApplicationKeypad {
    const STR: &'static str = esc!(">");
}

/// A command that instructs the terminal emulator to begin a
/// [synchronized update block](https://gitlab.com/gnachman/iterm2/-/wikis/synchronized-updates-spec)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BeginSynchronizedUpdate;

impl ConstEncode for BeginSynchronizedUpdate {
    const STR: &'static str = csi!("?2026h");
}

/// End synchronized update block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndSynchronizedUpdate;

impl ConstEncode for EndSynchronizedUpdate {
    const STR: &'static str = csi!("?2026l");
}



/// Generate terminal mode control structures (enable, disable, request,
/// response).
#[macro_export]
macro_rules! terminal_mode {
    // Variant with custom documentation
    ($(#[doc = $doc:expr])+ $name:ident, $code:literal) => {
        ::paste::paste! {
            $(#[doc = $doc])+
            #[doc = ""]
            #[doc = "Enable the mode."]
            pub struct [<Enable $name>];

            impl vtenc::ConstEncode for [<Enable $name>] {
                const STR: &'static str = vtenc::csi!("?", stringify!($code), "h");
            }

            $(#[doc = $doc])+
            #[doc = ""]
            #[doc = "Disable the mode."]
            pub struct [<Disable $name>];

            impl vtenc::ConstEncode for [<Disable $name>] {
                const STR: &'static str = vtenc::csi!("?", stringify!($code), "l");
            }

            $(#[doc = $doc])+
            #[doc = ""]
            #[doc = "Request the mode status."]
            pub struct [<Request $name>];

            impl vtenc::ConstEncode for [<Request $name>] {
                const STR: &'static str = vtenc::csi!("?", stringify!($code), "$p");
            }

            $(#[doc = $doc])+
            #[doc = ""]
            #[doc = "Mode status response."]
            pub struct $name(pub bool);

            impl vtenc::Encode for $name {
                #[inline]
                fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, vtenc::EncodeError> {
                    vtenc::write_csi!(buf; "?", stringify!($code), ";", u8::from(self.0), "$y")
                }
            }
        }
    };
    // Variant without custom documentation
    ($name:ident, $code:literal) => {
        ::paste::paste! {
            /// Enable the mode.
            pub struct [<Enable $name>];

            impl ConstEncode for [<Enable $name>] {
                const STR: &'static str = csi!("?", stringify!($code), "h");
            }

            /// Disable the mode.
            pub struct [<Disable $name>];

            impl ConstEncode for [<Disable $name>] {
                const STR: &'static str = csi!("?", stringify!($code), "l");
            }

            /// Request the mode status.
            pub struct [<Request $name>];

            impl ConstEncode for [<Request $name>] {
                const STR: &'static str = csi!("?", stringify!($code), "$p");
            }

            /// Mode status response.
            pub struct $name(pub bool);

            impl Encode for $name {
                #[inline]
                fn encode<W: std::io::Write>(&mut self, buf: &mut W) -> Result<usize, vtenc::EncodeError> {
                    write_csi!(buf; "?", stringify!($code), ";", u8::from(self.0), "$y")
                }
            }
        }
    };
}
