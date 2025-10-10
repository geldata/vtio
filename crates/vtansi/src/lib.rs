//! Macros for framing VT ANSI escape sequences.

#![warn(clippy::pedantic)]

pub mod encode;
pub use encode::Encode;

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
