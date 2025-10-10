//! Macros for framing VT ANSI escape sequences.

#![warn(clippy::pedantic)]

pub mod encode;
pub use encode::{ConstEncodedLen, Encode, ConstEncode};

/// Format a string while prepending a ANSI control sequence introducer
/// (`"\x1b["`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! csi {
    () => { "\x1B[" };
    ($fmt:literal) => { concat!("\x1B[", $fmt) };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1B[", $fmt), $($args),+)
    };
}

/// Write a CSI sequence to a buffer without heap allocation.
///
/// When called with only a string literal, this uses `write_const_str_into!`
/// for compile-time efficiency. When called with format arguments, this uses
/// `write_into!` to format directly into the buffer.
#[macro_export]
macro_rules! write_csi {
    ($buf:expr, $fmt:literal) => {
        $crate::write_const_str_into!($buf, concat!("\x1B[", $fmt))
    };
    ($buf:expr, $fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::write_into!($buf, concat!("\x1B[", $fmt), $($args),+)
    };
}

/// Format a string while prepending a xterm Operating System Commands (OSC)
/// introducer (`"\x1b]"`) and appending a string terminator (`"\x1b\\"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! osc {
    () => { "\x1B]\x1B\\" };
    ($fmt:literal) => { concat!("\x1B]", $fmt, "\x1B\\") };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1B]", $fmt, "\x1B\\"), $($args),+)
    };
}

/// Write an OSC sequence to a buffer without heap allocation.
///
/// When called with only a string literal, this uses `write_const_str_into!`
/// for compile-time efficiency. When called with format arguments, this uses
/// `write_into!` to format directly into the buffer.
#[macro_export]
macro_rules! write_osc {
    ($buf:expr, $fmt:literal) => {
        $crate::write_const_str_into!($buf, concat!("\x1B]", $fmt, "\x1B\\"))
    };
    ($buf:expr, $fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::write_into!($buf, concat!("\x1B]", $fmt, "\x1B\\"), $($args),+)
    };
}

/// Format a string while prepending a Single Shift 2 (SS2) introducer
/// (`"\x1bN"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! ss2 {
    () => { "\x1BN" };
    ($fmt:literal) => { concat!("\x1BN", $fmt) };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1BN", $fmt), $($args),+)
    };
}

/// Write an SS2 sequence to a buffer without heap allocation.
///
/// When called with only a string literal, this uses `write_const_str_into!`
/// for compile-time efficiency. When called with format arguments, this uses
/// `write_into!` to format directly into the buffer.
#[macro_export]
macro_rules! write_ss2 {
    ($buf:expr, $fmt:literal) => {
        $crate::write_const_str_into!($buf, concat!("\x1BN", $fmt))
    };
    ($buf:expr, $fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::write_into!($buf, concat!("\x1BN", $fmt), $($args),+)
    };
}

/// Format a string while prepending a Single Shift 3 (SS3) introducer
/// (`"\x1bO"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! ss3 {
    () => { "\x1BO" };
    ($fmt:literal) => { concat!("\x1BO", $fmt) };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1BO", $fmt), $($args),+)
    };
}

/// Write an SS3 sequence to a buffer without heap allocation.
///
/// When called with only a string literal, this uses `write_const_str_into!`
/// for compile-time efficiency. When called with format arguments, this uses
/// `write_into!` to format directly into the buffer.
#[macro_export]
macro_rules! write_ss3 {
    ($buf:expr, $fmt:literal) => {
        $crate::write_const_str_into!($buf, concat!("\x1BO", $fmt))
    };
    ($buf:expr, $fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::write_into!($buf, concat!("\x1BO", $fmt), $($args),+)
    };
}

/// Format a string while prepending a Device Control String (DCS)
/// introducer (`"\x1bP"`) and appending a string terminator (`"\x1b\\"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! dcs {
    () => { "\x1BP\x1B\\" };
    ($fmt:literal) => { concat!("\x1BP", $fmt, "\x1B\\") };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1BP", $fmt, "\x1B\\"), $($args),+)
    };
}

/// Write a DCS sequence to a buffer without heap allocation.
///
/// When called with only a string literal, this uses `write_const_str_into!`
/// for compile-time efficiency. When called with format arguments, this uses
/// `write_into!` to format directly into the buffer.
#[macro_export]
macro_rules! write_dcs {
    ($buf:expr, $fmt:literal) => {
        $crate::write_const_str_into!($buf, concat!("\x1BP", $fmt, "\x1B\\"))
    };
    ($buf:expr, $fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::write_into!($buf, concat!("\x1BP", $fmt, "\x1B\\"), $($args),+)
    };
}

/// Format a string while prepending a Privacy Message (PM) introducer
/// (`"\x1b^"`) and appending a string terminator (`"\x1b\\"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! pm {
    () => { "\x1B^\x1B\\" };
    ($fmt:literal) => { concat!("\x1B^", $fmt, "\x1B\\") };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1B^", $fmt, "\x1B\\"), $($args),+)
    };
}

/// Write a PM sequence to a buffer without heap allocation.
///
/// When called with only a string literal, this uses `write_const_str_into!`
/// for compile-time efficiency. When called with format arguments, this uses
/// `write_into!` to format directly into the buffer.
#[macro_export]
macro_rules! write_pm {
    ($buf:expr, $fmt:literal) => {
        $crate::write_const_str_into!($buf, concat!("\x1B^", $fmt, "\x1B\\"))
    };
    ($buf:expr, $fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::write_into!($buf, concat!("\x1B^", $fmt, "\x1B\\"), $($args),+)
    };
}

/// Format a string while prepending an Application Program Command (APC)
/// introducer (`"\x1b_"`) and appending a string terminator (`"\x1b\\"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! apc {
    () => { "\x1B_\x1B\\" };
    ($fmt:literal) => { concat!("\x1B_", $fmt, "\x1B\\") };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1B_", $fmt, "\x1B\\"), $($args),+)
    };
}

/// Write an APC sequence to a buffer without heap allocation.
///
/// When called with only a string literal, this uses `write_const_str_into!`
/// for compile-time efficiency. When called with format arguments, this uses
/// `write_into!` to format directly into the buffer.
#[macro_export]
macro_rules! write_apc {
    ($buf:expr, $fmt:literal) => {
        $crate::write_const_str_into!($buf, concat!("\x1B_", $fmt, "\x1B\\"))
    };
    ($buf:expr, $fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::write_into!($buf, concat!("\x1B_", $fmt, "\x1B\\"), $($args),+)
    };
}

/// Format a string while prepending an escape character (`"\x1b"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! esc {
    () => { "\x1B" };
    ($fmt:literal) => { concat!("\x1B", $fmt) };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1B", $fmt), $($args),+)
    };
}

/// Write an ESC sequence to a buffer without heap allocation.
///
/// When called with only a string literal, this uses `write_const_str_into!`
/// for compile-time efficiency. When called with format arguments, this uses
/// `write_into!` to format directly into the buffer.
#[macro_export]
macro_rules! write_esc {
    ($buf:expr, $fmt:literal) => {
        $crate::write_const_str_into!($buf, concat!("\x1B", $fmt))
    };
    ($buf:expr, $fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::write_into!($buf, concat!("\x1B", $fmt), $($args),+)
    };
}

/// Format a string while prepending an escape character (`"\x1b"`) and
/// appending a string terminator (`"\x1b\\"`).
///
/// When called with only a string literal, this uses `concat!` for
/// compile-time efficiency. When called with format arguments, this uses
/// `format!` for runtime formatting.
#[macro_export]
macro_rules! escst {
    () => { "\x1B\x1B\\" };
    ($fmt:literal) => { concat!("\x1B", $fmt, "\x1B\\") };
    ($fmt:literal, $($args:expr),+ $(,)?) => {
        format!(concat!("\x1B", $fmt, "\x1B\\"), $($args),+)
    };
}

/// Write an ESC...ST sequence to a buffer without heap allocation.
///
/// When called with only a string literal, this uses `write_const_str_into!`
/// for compile-time efficiency. When called with format arguments, this uses
/// `write_into!` to format directly into the buffer.
#[macro_export]
macro_rules! write_escst {
    ($buf:expr, $fmt:literal) => {
        $crate::write_const_str_into!($buf, concat!("\x1B", $fmt, "\x1B\\"))
    };
    ($buf:expr, $fmt:literal, $($args:expr),+ $(,)?) => {
        $crate::write_into!($buf, concat!("\x1B", $fmt, "\x1B\\"), $($args),+)
    };
}
