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

/// Define a composite const encodeable that combines multiple encodeables.
#[macro_export]
macro_rules! ansi_composite {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident = [
            $($command:path),* $(,)?
        ];
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $vis struct $name;

        impl $crate::encode::AnsiEncode for $name {
            const ENCODED_LEN: ::core::option::Option<usize> = {
                // Sum all ENCODED_LEN values, yielding None if any is None
                const TERMS: &[::core::option::Option<usize>] = &[
                    $( <$command as $crate::encode::AnsiEncode>::ENCODED_LEN ),*
                ];

                let mut sum = 0;
                let mut i = 0;
                while i < TERMS.len() {
                    match TERMS[i] {
                        ::core::option::Option::Some(len) => sum += len,
                        ::core::option::Option::None => {sum = 0; break},
                    }
                    i += 1;
                }
                if sum == 0 {
                    ::core::option::Option::None
                } else {
                    ::core::option::Option::Some(sum)
                }
            };

            #[inline]
            fn encode_ansi_into<W: ::std::io::Write + ?::std::marker::Sized>(
                &self,
                buf: &mut W
            ) -> Result<usize, $crate::encode::EncodeError> {
                // Use a stack-allocated buffer for const-length commands if available
                match <Self as $crate::encode::AnsiEncode>::ENCODED_LEN {
                    ::core::option::Option::Some(len) => {
                        let mut stack_buf = ::std::vec![0u8; len];
                        let mut offset = 0;

                        $(
                            offset += $command.encode_ansi_into(&mut &mut stack_buf[offset..])?;
                        )*

                        buf.write_all(&stack_buf[..offset])
                            .map_err($crate::encode::EncodeError::IOError)?;
                        Ok(offset)
                    }
                    ::core::option::Option::None => {
                        // Dynamic encoding path
                        let mut total = 0;
                        $(
                            total += $command.encode_ansi_into(buf)?;
                        )*
                        Ok(total)
                    }
                }
            }
        }
    };
}
