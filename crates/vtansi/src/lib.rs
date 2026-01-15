#![warn(clippy::pedantic)]

pub mod macros;

pub mod debug;
pub mod encode;
pub mod parse;

#[cfg(feature = "bitflags")]
pub mod bitflags;

pub mod byte_trie;
pub mod registry;

pub use encode::write_byte_into;
pub use encode::write_bytes_into;
pub use encode::write_int;
pub use encode::write_str_into;
pub use encode::{
    AnsiEncode, AnsiFinalByte, AnsiMuxEncode, EncodeError, RawByte,
    StaticAnsiEncode,
};
pub use encode::{
    encode_delimited_values, encode_delimited_values_with_optional,
    encode_keyvalue_pairs,
};

pub use parse::parse_keyvalue_pairs;
pub use parse::{ParseError, TryFromAnsi, TryFromAnsiIter};

pub use debug::{TerseDebug, TerseDisplay};

#[cfg(feature = "derive")]
pub mod derive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnsiControlDirection {
    /// Terminal to host input (e.g key events)
    Input,
    /// Host to terminal output (i.e render sequences, reports etc.)
    Output,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AnsiControlFunctionKind {
    /// C0 Control Character (C0)
    C0,

    /// Raw byte (0x00-0x7F)
    Byte,

    /// Control Sequence Introducer (CSI)
    ///
    /// The Control Sequence Introducer (CSI) is used to introduce control
    /// sequences, which are commands or functions that modify the behavior of
    /// the terminal. CSI sequences typically start with an escape character
    /// (\x1B) followed by a left square bracket ([) and are used for tasks
    /// such as cursor movement, text formatting, color changes, and more.
    Csi,

    /// Operating System Command (OSC)
    ///
    /// The Operating System Command (OSC) is used to send commands directly to
    /// the terminal emulator or operating system. OSC sequences typically
    /// start with an escape character (\x1B) followed by a right square
    /// bracket (]), and they are often used for tasks like setting the
    /// terminal window title, changing the terminal's icon, or sending
    /// notifications to the user.
    Osc,

    /// Device Control String (DCS)
    ///
    /// The Device Control String (DCS) is similar to the OSC sequence but is
    /// used for more advanced device control. DCS sequences typically start
    /// with an escape character (\x1B) followed by the letter 'P', and they
    /// allow for more complex interactions with the terminal hardware or
    /// emulator.
    Dcs,

    /// Other escape sequence (unterminated)
    Esc,

    /// ESC ... ST - Escape sequence terminated with ST (`ESC \`)
    /// This serves as a catch-all with less-common and less-defined
    /// sequences such as APC, PM and SOS.
    EscSt,

    /// Single Shift 3 (SS3)
    ///
    /// The Single Shift 3 (SS3) sequence is used for input parsing,
    /// particularly for application cursor keys and function keys.
    /// SS3 sequences start with an escape character (\x1B) followed
    /// by the letter 'O'.
    Ss3,
}

pub trait AnsiEvent<'a>: better_any::Tid<'a> {
    fn ansi_control_kind(&self) -> Option<AnsiControlFunctionKind>;
    fn ansi_direction(&self) -> AnsiControlDirection;

    /// Encode this event as an ANSI control sequence into the provided writer.
    ///
    /// # Errors
    ///
    /// Returns an error if the encoding fails or if the writer returns an error.
    fn encode_ansi_into(
        &self,
        sink: &mut dyn std::io::Write,
    ) -> Result<usize, EncodeError>;

    /// Encode this event as an ANSI control sequence and return the resulting bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the encoding fails.
    fn encode_ansi(&self) -> Result<Vec<u8>, EncodeError> {
        let mut v = Vec::new();
        self.encode_ansi_into(&mut v)?;
        Ok(v)
    }

    /// Format this event in a terse, human-readable format.
    ///
    /// # Errors
    ///
    /// Returns an error if formatting fails.
    fn terse_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl std::fmt::Debug for dyn AnsiEvent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.terse_fmt(f)
    }
}

/// Blanket implementation helper: implement `AnsiEvent::encode_ansi_into` by
/// delegating to `AnsiEncode::encode_ansi_into`.
///
/// Use this macro in your `AnsiEvent` implementation when your type also
/// implements `AnsiEncode`:
///
/// ```ignore
/// impl<'a> AnsiEvent<'a> for MyType {
///     fn ansi_control_kind(&self) -> Option<AnsiControlFunctionKind> { ... }
///     fn ansi_direction(&self) -> AnsiControlDirection { ... }
///     vtansi::impl_ansi_event_encode!();
/// }
/// ```
#[macro_export]
macro_rules! impl_ansi_event_encode {
    () => {
        #[inline]
        fn encode_ansi_into(
            &self,
            sink: &mut dyn ::std::io::Write,
        ) -> ::core::result::Result<usize, $crate::EncodeError> {
            <Self as $crate::AnsiEncode>::encode_ansi_into(self, sink)
        }
    };
}

/// Blanket implementation helper: implement `AnsiEvent::terse_fmt` by
/// delegating to `TerseDisplay::terse_fmt`.
///
/// Use this macro in your `AnsiEvent` implementation when your type also
/// implements `TerseDisplay`:
///
/// ```ignore
/// impl<'a> AnsiEvent<'a> for MyType {
///     fn ansi_control_kind(&self) -> Option<AnsiControlFunctionKind> { ... }
///     fn ansi_direction(&self) -> AnsiControlDirection { ... }
///     vtansi::impl_ansi_event_encode!();
///     vtansi::impl_ansi_event_terse_fmt!();
/// }
/// ```
#[macro_export]
macro_rules! impl_ansi_event_terse_fmt {
    () => {
        #[inline]
        fn terse_fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::fmt::Result {
            <Self as $crate::TerseDisplay>::terse_fmt(self, f)
        }
    };
}

better_any::tid! { impl<'a> TidAble<'a> for &'a dyn AnsiEvent<'a> }
better_any::tid! { impl<'a> TidAble<'a> for dyn AnsiEvent<'a> + 'a }

#[doc(hidden)]
pub mod __private {
    pub use itoa;
    pub use paste;

    pub use better_any;

    pub use linkme;

    #[cfg(feature = "bitflags")]
    pub use bitflags;
}
