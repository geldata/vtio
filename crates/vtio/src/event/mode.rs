//! Definitions and helpers for terminal modes.

/// Represents state of terminal mode as reported in `DECRPM` responses.
///
/// See <https://vt100.net/docs/vt510-rm/DECRPM.html> for more information.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    num_enum::IntoPrimitive,
    num_enum::TryFromPrimitive,
    vtansi::derive::ToAnsi,
    vtansi::derive::FromAnsi,
)]
#[repr(u8)]
pub enum TerminalModeState {
    NotRecognized = 0,
    Set = 1,
    Reset = 2,
    PermanentlySet = 3,
    PermanentlyReset = 4,
}

/// Generate terminal mode control sequences.
///
/// This macro generates four control sequence structs for a terminal mode:
/// - `Enable{Name}`: CSI sequence with 'h' final byte to enable the mode
/// - `Disable{Name}`: CSI sequence with 'l' final byte to disable the mode
/// - `Request{Name}`: CSI sequence with '$' intermediate and 'p' final byte
///   to request the mode state
/// - `{Name}`: CSI sequence with '$' intermediate and 'y' final byte
///   representing the mode state response (with `enabled: bool` field)
///
/// # Syntax
///
/// ```ignore
/// terminal_mode!(ModeName, params = ["param_value"]);
/// terminal_mode!(ModeName, private = '?', params = ["param_value"]);
/// ```
///
/// # Parameters
///
/// - `ModeName`: The base identifier for the mode
/// - `private`: Optional private parameter character (e.g., '?')
/// - `params`: Required parameter array for the CSI sequence
///
/// # Example
///
/// ```ignore
/// terminal_mode!(RelativeCursorOriginMode, private = '?', params = ["6"]);
/// ```
///
/// This generates:
/// - `EnableRelativeCursorOriginMode` → CSI ? 6 h
/// - `DisableRelativeCursorOriginMode` → CSI ? 6 l
/// - `RequestRelativeCursorOriginMode` → CSI ? 6 $ p
/// - `RelativeCursorOriginMode` → CSI ? $ y (with `state` field)
#[macro_export]
macro_rules! terminal_mode {
    ($(#[$meta:meta])* $base_name:ident, private = $private:literal, params = [$($params:literal),* $(,)?]) => {
        $crate::__private::paste::paste! {
            $(#[$meta])*
            #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, ::vtansi::derive::AnsiInput)]
            #[vtansi(csi, private = $private, params = [$($params),*], intermediate = "$", finalbyte = 'y')]
            pub struct [<$base_name>] {
                pub state: $crate::event::mode::TerminalModeState,
            }

            #[doc = concat!("Enable [`", stringify!($base_name), "`].")]
            #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, ::vtansi::derive::AnsiOutput)]
            #[vtansi(csi, private = $private, params = [$($params),*], finalbyte = 'h')]
            pub struct [<Enable $base_name>];

            #[doc = concat!("Disable [`", stringify!($base_name), "`].")]
            #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, ::vtansi::derive::AnsiOutput)]
            #[vtansi(csi, private = $private, params = [$($params),*], finalbyte = 'l')]
            pub struct [<Disable $base_name>];

            #[doc = concat!("Query state of [`", stringify!($base_name), "`].")]
            #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, ::vtansi::derive::AnsiOutput)]
            #[vtansi(csi, private = $private, params = [$($params),*], intermediate = "$", finalbyte = 'p')]
            pub struct [<Request $base_name>];
        }
    };

    ($(#[$meta:meta])* $base_name:ident, params = [$($params:literal),* $(,)?]) => {
        $crate::__private::paste::paste! {
            $(#[$meta])*
            #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, ::vtansi::derive::AnsiInput)]
            #[vtansi(csi, params = [$($params),*], intermediate = "$", finalbyte = 'y')]
            pub struct [<$base_name>] {
                pub state: $crate::event::mode::TerminalModeState,
            }

            #[doc = concat!("Enable [`", stringify!($base_name), "`].")]
            #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, ::vtansi::derive::AnsiOutput)]
            #[vtansi(csi, params = [$($params),*], finalbyte = 'h')]
            pub struct [<Enable $base_name>];

            #[doc = concat!("Disable [`", stringify!($base_name), "`].")]
            #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, ::vtansi::derive::AnsiOutput)]
            #[vtansi(csi, params = [$($params),*], finalbyte = 'l')]
            pub struct [<Disable $base_name>];

            #[doc = concat!("Query state of [`", stringify!($base_name), "`].")]
            #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, ::vtansi::derive::AnsiOutput)]
            #[vtansi(csi, params = [$($params),*], intermediate = "$", finalbyte = 'p')]
            pub struct [<Request $base_name>];
        }
    };

    // With flag specifier (private mode)
    ($(#[$meta:meta])* $base_name:ident, private = $private:literal, params = [$($params:literal),* $(,)?], flag = $flag:expr) => {
        $crate::terminal_mode!($(#[$meta])* $base_name, private = $private, params = [$($params),*]);

        impl $crate::event::keyboard::AsModeFlag for $base_name {
            fn as_mode_flag(&self) -> $crate::event::keyboard::KeyboardModeFlags {
                if self.state == $crate::event::mode::TerminalModeState::Set
                    || self.state == $crate::event::mode::TerminalModeState::PermanentlySet
                {
                    $flag
                } else {
                    $crate::event::keyboard::KeyboardModeFlags::empty()
                }
            }
        }
    };

    // With flag specifier (non-private mode)
    ($(#[$meta:meta])* $base_name:ident, params = [$($params:literal),* $(,)?], flag = $flag:expr) => {
        $crate::terminal_mode!($(#[$meta])* $base_name, params = [$($params),*]);

        impl $crate::event::keyboard::AsModeFlag for $base_name {
            fn as_mode_flag(&self) -> $crate::event::keyboard::KeyboardModeFlags {
                if self.state == $crate::event::mode::TerminalModeState::Set
                    || self.state == $crate::event::mode::TerminalModeState::PermanentlySet
                {
                    $flag
                } else {
                    $crate::event::keyboard::KeyboardModeFlags::empty()
                }
            }
        }
    };
}
