/// Concatenate string literals while prepending a ANSI control sequence introducer (`"\x1b["`)
#[macro_export]
#[doc(hidden)]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

/// Concatenate string literals while prepending a xterm Operating System Commands (OSC)
/// introducer (`"\x1b]"`) and appending a BEL (`"\x07"`).
#[macro_export]
#[doc(hidden)]
macro_rules! osc {
    ($( $l:expr ),*) => { concat!("\x1B]", $( $l ),*, "\x1B\\") };
}
