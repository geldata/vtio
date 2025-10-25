//! Macros for framing VT ANSI escape sequences.

#![warn(clippy::pedantic)]

pub mod encode;
pub mod parse;

pub use encode::write_bytes_into;
pub use encode::write_int;
pub use encode::write_str_into;
pub use encode::{AnsiEncode, EncodeError, StaticAnsiEncode, StaticEncodedLen, ToAnsi};
pub use encode::{
    encode_delimited_values, encode_delimited_values_with_optional, encode_keyvalue_pairs,
};

pub use parse::{FromAnsi, ParseError, TryFromAnsi};
pub use parse::{parse_delimited_values, parse_keyvalue_pairs};

#[cfg(feature = "derive")]
pub use vtansi_derive::{FromAnsi, ToAnsi};

// Note: fixed_length_ansi macro is automatically available at crate root via #[macro_export]

/// Concatenate string literals while prepending a ANSI control sequence
/// introducer (`"\x1b["`).
///
/// All arguments must be string literals that can be concatenated at
/// compile-time using `concat!`.
#[macro_export]
macro_rules! format_csi {
    ($($arg:expr),+ $(,)?) => {
        concat!("\x1B[", $($arg),+)
    };
}

/// Write a CSI sequence to a buffer without heap allocation.
///
/// Takes a semicolon-separated list of items (string literals or integers).
/// Each item is written directly without formatting overhead.
///
/// # Examples
///
/// ```ignore
/// write_csi!(buf; "H")                    // ESC[H
/// write_csi!(buf; row, ";", col, "H")     // ESC[row;colH
/// write_csi!(buf; n, "A")                 // ESC[nA
/// ```
#[macro_export]
macro_rules! write_csi {
    ($buf:expr; $($item:expr),* $(,)?) => {{
        let mut __total = 0usize;
        __total += $crate::encode::write_str_into($buf, "\x1B[")?;
        $(
            __total += $crate::encode::AnsiEncode::encode_ansi_into(&($item), $buf)?;
        )*
        Ok(__total)
    }};
}

/// Concatenate string literals while prepending a xterm Operating System
/// Commands (OSC) introducer (`"\x1b]"`) and appending a string terminator
/// (`"\x1b\\"`).
///
/// All arguments must be string literals that can be concatenated at
/// compile-time using `concat!`.
#[macro_export]
macro_rules! format_osc {
    ($($arg:expr),+ $(,)?) => {
        concat!("\x1B]", $($arg),+, "\x1B\\")
    };
}

/// Write an OSC sequence to a buffer without heap allocation.
///
/// Takes a semicolon-separated list of items (string literals or integers).
/// Each item is written directly without formatting overhead.
///
/// # Examples
///
/// ```ignore
/// write_osc!(buf; "0;", title)            // ESC]0;titleESC\
/// ```
#[macro_export]
macro_rules! write_osc {
    ($buf:expr; $($item:expr),* $(,)?) => {{
        let mut __total = 0usize;
        __total += $crate::encode::write_str_into($buf, "\x1B]")?;
        $(
            __total += $crate::encode::AnsiEncode::encode_ansi_into(&($item), $buf)?;
        )*
        __total += $crate::encode::write_str_into($buf, "\x1B\\")?;
        Ok(__total)
    }};
}

/// Concatenate string literals while prepending a Single Shift 2 (SS2)
/// introducer (`"\x1bN"`).
///
/// All arguments must be string literals that can be concatenated at
/// compile-time using `concat!`.
#[macro_export]
macro_rules! format_ss2 {
    ($($arg:expr),+ $(,)?) => {
        concat!("\x1BN", $($arg),+)
    };
}

/// Write an SS2 sequence to a buffer without heap allocation.
///
/// Takes a semicolon-separated list of items (string literals or integers).
/// Each item is written directly without formatting overhead.
#[macro_export]
macro_rules! write_ss2 {
    ($buf:expr; $($item:expr),* $(,)?) => {{
        let mut __total = 0usize;
        __total += $crate::encode::write_str_into($buf, "\x1BN")?;
        $(
            __total += $crate::encode::AnsiEncode::encode_ansi_into(&($item), $buf)?;
        )*
        Ok(__total)
    }};
}

/// Concatenate string literals while prepending a Single Shift 3 (SS3)
/// introducer (`"\x1bO"`).
///
/// All arguments must be string literals that can be concatenated at
/// compile-time using `concat!`.
#[macro_export]
macro_rules! format_ss3 {
    ($($arg:expr),+ $(,)?) => {
        concat!("\x1BO", $($arg),+)
    };
}

/// Write an SS3 sequence to a buffer without heap allocation.
///
/// Takes a semicolon-separated list of items (string literals or integers).
/// Each item is written directly without formatting overhead.
#[macro_export]
macro_rules! write_ss3 {
    ($buf:expr; $($item:expr),* $(,)?) => {{
        let mut __total = 0usize;
        __total += $crate::encode::write_str_into($buf, "\x1BO")?;
        $(
            __total += $crate::encode::AnsiEncode::encode_ansi_into(&($item), $buf)?;
        )*
        Ok(__total)
    }};
}

/// Concatenate string literals while prepending a Device Control String (DCS)
/// introducer (`"\x1bP"`) and appending a string terminator (`"\x1b\\"`).
///
/// All arguments must be string literals that can be concatenated at
/// compile-time using `concat!`.
#[macro_export]
macro_rules! format_dcs {
    ($($arg:expr),+ $(,)?) => {
        concat!("\x1BP", $($arg),+, "\x1B\\")
    };
}

/// Write a DCS sequence to a buffer without heap allocation.
///
/// Takes a semicolon-separated list of items (string literals or integers).
/// Each item is written directly without formatting overhead.
#[macro_export]
macro_rules! write_dcs {
    ($buf:expr; $($item:expr),* $(,)?) => {{
        let mut __total = 0usize;
        __total += $crate::encode::write_str_into($buf, "\x1BP")?;
        $(
            __total += $crate::encode::AnsiEncode::encode_ansi_into(&($item), $buf)?;
        )*
        __total += $crate::encode::write_str_into($buf, "\x1B\\")?;
        Ok(__total)
    }};
}

/// Concatenate string literals while prepending a Privacy Message (PM)
/// introducer (`"\x1b^"`) and appending a string terminator (`"\x1b\\"`).
///
/// All arguments must be string literals that can be concatenated at
/// compile-time using `concat!`.
#[macro_export]
macro_rules! format_pm {
    ($($arg:expr),+ $(,)?) => {
        concat!("\x1B^", $($arg),+, "\x1B\\")
    };
}

/// Write a PM sequence to a buffer without heap allocation.
///
/// Takes a semicolon-separated list of items (string literals or integers).
/// Each item is written directly without formatting overhead.
#[macro_export]
macro_rules! write_pm {
    ($buf:expr; $($item:expr),* $(,)?) => {{
        let mut __total = 0usize;
        __total += $crate::encode::write_str_into($buf, "\x1B^")?;
        $(
            __total += $crate::encode::AnsiEncode::encode_ansi_into(&($item), $buf)?;
        )*
        __total += $crate::encode::write_str_into($buf, "\x1B\\")?;
        Ok(__total)
    }};
}

/// Concatenate string literals while prepending an Application Program
/// Command (APC) introducer (`"\x1b_"`) and appending a string terminator
/// (`"\x1b\\"`).
///
/// All arguments must be string literals that can be concatenated at
/// compile-time using `concat!`.
#[macro_export]
macro_rules! format_apc {
    ($($arg:expr),+ $(,)?) => {
        concat!("\x1B_", $($arg),+, "\x1B\\")
    };
}

/// Write an APC sequence to a buffer without heap allocation.
///
/// Takes a semicolon-separated list of items (string literals or integers).
/// Each item is written directly without formatting overhead.
#[macro_export]
macro_rules! write_apc {
    ($buf:expr; $($item:expr),* $(,)?) => {{
        let mut __total = 0usize;
        __total += $crate::encode::write_str_into($buf, "\x1B_")?;
        $(
            __total += $crate::encode::AnsiEncode::encode_ansi_into(&($item), $buf)?;
        )*
        __total += $crate::encode::write_str_into($buf, "\x1B\\")?;
        Ok(__total)
    }};
}

/// Concatenate string literals while prepending an escape character
/// (`"\x1b"`).
///
/// All arguments must be string literals that can be concatenated at
/// compile-time using `concat!`.
#[macro_export]
macro_rules! format_esc {
    ($($arg:expr),+ $(,)?) => {
        concat!("\x1B", $($arg),+)
    };
}

/// Write an ESC sequence to a buffer without heap allocation.
///
/// Takes a semicolon-separated list of items (string literals or integers).
/// Each item is written directly without formatting overhead.
#[macro_export]
macro_rules! write_esc {
    ($buf:expr; $($item:expr),* $(,)?) => {{
        let mut __total = 0usize;
        __total += $crate::encode::write_str_into($buf, "\x1B")?;
        $(
            __total += $crate::encode::AnsiEncode::encode_ansi_into(&($item), $buf)?;
        )*
        Ok(__total)
    }};
}

/// Concatenate string literals while prepending an escape character
/// (`"\x1b"`) and appending a string terminator (`"\x1b\\"`).
///
/// All arguments must be string literals that can be concatenated at
/// compile-time using `concat!`.
#[macro_export]
macro_rules! format_escst {
    ($($arg:expr),+ $(,)?) => {
        concat!("\x1B", $($arg),+, "\x1B\\")
    };
}

/// Write an ESC...ST sequence to a buffer without heap allocation.
///
/// Takes a semicolon-separated list of items (string literals or integers).
/// Each item is written directly without formatting overhead.
#[macro_export]
macro_rules! write_escst {
    ($buf:expr; $($item:expr),* $(,)?) => {{
        let mut __total = 0usize;
        __total += $crate::encode::write_str_into($buf, "\x1B")?;
        $(
            __total += $crate::encode::AnsiEncode::encode_ansi_into(&($item), $buf)?;
        )*
        __total += $crate::encode::write_str_into($buf, "\x1B\\")?;
        Ok(__total)
    }};
}
