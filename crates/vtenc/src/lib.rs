//! Macros for framing VT ANSI escape sequences.

#![warn(clippy::pedantic)]

pub mod encode;
pub use encode::write_bytes_into;
pub use encode::write_int;
pub use encode::write_str_into;
pub use encode::{ConstEncode, ConstEncodedLen, Encode, EncodeError, WriteSeq};

/// Format a string while prepending a ANSI control sequence introducer
/// (`"\x1b["`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! csi {
    ($fmt:literal) => { concat!("\x1B[", $fmt) };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1B[", $fmt), $($args),+)
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
            __total += $crate::encode::WriteSeq::write_seq(&($item), $buf)?;
        )*
        Ok(__total)
    }};
}

/// Format a string while prepending a xterm Operating System Commands (OSC)
/// introducer (`"\x1b]"`) and appending a string terminator (`"\x1b\\"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! osc {
    ($fmt:literal) => { concat!("\x1B]", $fmt, "\x1B\\") };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1B]", $fmt, "\x1B\\"), $($args),+)
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
            __total += $crate::encode::WriteSeq::write_seq(&($item), $buf)?;
        )*
        __total += $crate::encode::write_str_into($buf, "\x1B\\")?;
        Ok(__total)
    }};
}

/// Format a string while prepending a Single Shift 2 (SS2) introducer
/// (`"\x1bN"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! ss2 {
    ($fmt:literal) => { concat!("\x1BN", $fmt) };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1BN", $fmt), $($args),+)
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
            __total += $crate::encode::WriteSeq::write_seq(&($item), $buf)?;
        )*
        Ok(__total)
    }};
}

/// Format a string while prepending a Single Shift 3 (SS3) introducer
/// (`"\x1bO"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! ss3 {
    ($fmt:literal) => { concat!("\x1BO", $fmt) };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1BO", $fmt), $($args),+)
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
            __total += $crate::encode::WriteSeq::write_seq(&($item), $buf)?;
        )*
        Ok(__total)
    }};
}

/// Format a string while prepending a Device Control String (DCS)
/// introducer (`"\x1bP"`) and appending a string terminator (`"\x1b\\"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! dcs {
    ($fmt:literal) => { concat!("\x1BP", $fmt, "\x1B\\") };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1BP", $fmt, "\x1B\\"), $($args),+)
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
            __total += $crate::encode::WriteSeq::write_seq(&($item), $buf)?;
        )*
        __total += $crate::encode::write_str_into($buf, "\x1B\\")?;
        Ok(__total)
    }};
}

/// Format a string while prepending a Privacy Message (PM) introducer
/// (`"\x1b^"`) and appending a string terminator (`"\x1b\\"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! pm {
    ($fmt:literal) => { concat!("\x1B^", $fmt, "\x1B\\") };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1B^", $fmt, "\x1B\\"), $($args),+)
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
            __total += $crate::encode::WriteSeq::write_seq(&($item), $buf)?;
        )*
        __total += $crate::encode::write_str_into($buf, "\x1B\\")?;
        Ok(__total)
    }};
}

/// Format a string while prepending an Application Program Command (APC)
/// introducer (`"\x1b_"`) and appending a string terminator (`"\x1b\\"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! apc {
    ($fmt:literal) => { concat!("\x1B_", $fmt, "\x1B\\") };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1B_", $fmt, "\x1B\\"), $($args),+)
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
            __total += $crate::encode::WriteSeq::write_seq(&($item), $buf)?;
        )*
        __total += $crate::encode::write_str_into($buf, "\x1B\\")?;
        Ok(__total)
    }};
}

/// Format a string while prepending an escape character (`"\x1b"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! esc {
    ($fmt:literal) => { concat!("\x1B", $fmt) };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1B", $fmt), $($args),+)
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
            __total += $crate::encode::WriteSeq::write_seq(&($item), $buf)?;
        )*
        Ok(__total)
    }};
}

/// Format a string while prepending an escape character (`"\x1b"`) and
/// appending a string terminator (`"\x1b\\"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! escst {
    ($fmt:literal) => { concat!("\x1B", $fmt, "\x1B\\") };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1B", $fmt, "\x1B\\"), $($args),+)
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
            __total += $crate::encode::WriteSeq::write_seq(&($item), $buf)?;
        )*
        __total += $crate::encode::write_str_into($buf, "\x1B\\")?;
        Ok(__total)
    }};
}
