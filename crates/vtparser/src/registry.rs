use linkme::distributed_slice;
use smallvec::SmallVec;

#[derive(Clone, Debug, Default)]
pub struct EscapeSequenceParam(SmallVec<[u8; 32]>);

impl EscapeSequenceParam {
    #[inline]
    #[must_use]
    pub const fn from_smallvec(value: SmallVec<[u8; 32]>) -> Self {
        Self(value)
    }

    pub fn first(&self) -> Option<&u8> {
        self.0.first()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl std::ops::Deref for EscapeSequenceParam {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SmallVec<[u8; 32]>> for EscapeSequenceParam {
    fn from(value: SmallVec<[u8; 32]>) -> Self {
        Self(value)
    }
}

pub type EscapeSequenceParams = SmallVec<[EscapeSequenceParam; 8]>;
pub(crate) type Intermediate = [u8; 2];

#[derive(Copy, Clone, Debug, derive_more::TryFrom)]
#[repr(u8)]
#[try_from(repr)]
pub enum EscapeSequenceIntroducer {
    /// Control Sequence Introducer (CSI)
    ///
    /// The Control Sequence Introducer (CSI) is used to introduce control
    /// sequences, which are commands or functions that modify the behavior of
    /// the terminal. CSI sequences typically start with an escape character
    /// (\x1B) followed by a left square bracket ([) and are used for tasks
    /// such as cursor movement, text formatting, color changes, and more.
    CSI = b'[',

    /// Operating System Command (OSC)
    ///
    /// The Operating System Command (OSC) is used to send commands directly to
    /// the terminal emulator or operating system. OSC sequences typically
    /// start with an escape character (\x1B) followed by a right square
    /// bracket (]), and they are often used for tasks like setting the
    /// terminal window title, changing the terminal's icon, or sending
    /// notifications to the user.
    OSC = b']',

    /// Single Shift 2 (SS2)
    ///
    /// The Single Shift 2 (SS2) sequence is used to switch between different
    /// character sets in the terminal. SS2 sequences typically start with an
    /// escape character (\x1B) followed by the letter 'N'. They are used in
    /// internationalization scenarios where different character sets are
    /// needed.
    SS2 = b'N',

    /// Single Shift 3 (SS3)
    ///
    /// The Single Shift 3 (SS3) sequence is used to switch between different
    /// character sets in the terminal. SS3 sequences typically start with an
    /// escape character (\x1B) followed by the letter 'O'. They are used in
    /// internationalization scenarios where different character sets are
    /// needed.
    SS3 = b'O',

    /// Device Control String (DCS)
    ///
    /// The Device Control String (DCS) is similar to the OSC sequence but is
    /// used for more advanced device control. DCS sequences typically start
    /// with an escape character (\x1B) followed by the letter 'P', and they
    /// allow for more complex interactions with the terminal hardware or
    /// emulator.
    DCS = b'P',

    /// Privacy Message (PM)
    ///
    /// The Privacy Message (PM) sequence is similar to the OSC and DCS
    /// sequences but serves different purposes. PM sequences typically start
    /// with an escape character (\x1B) followed by the caret (^), and they are
    /// used for various communication and control tasks, including passing
    /// data between applications and the terminal emulator.
    PM = b'^',

    /// Application Program Command (APC)
    ///
    /// The Application Program Command (APC) sequence is similar to the OSC
    /// and DCS sequences but serves different purposes. APC sequences
    /// typically start with an escape character (\x1B) followed by the
    /// underscore (_), and they are used for various communication and control
    /// tasks, including passing data between applications and the terminal
    /// emulator.
    APC = b'_',

    /// String Terminator (ST)
    ///
    /// The String Terminator (ST) is used to indicate the end of an escape
    /// sequence. ST sequences typically start with an escape character (\x1B)
    /// followed by a backslash (\), and they signal the end of the escape
    /// sequence.
    ST = b'\\',

    /// DECKPAM (DEC Keypad Application Mode) Escape Sequence.
    ///
    /// This escape sequence is used to enable the application keypad mode in a
    /// DEC VT220 terminal. When application keypad mode is enabled, certain
    /// keys on the keypad (such as function keys) send special escape
    /// sequences instead of their regular ASCII characters. For example, the
    /// Page Up key may send the sequence for Page Up instead of its regular
    /// ASCII character.
    ///
    /// See <https://vt100.net/docs/vt220-rm/chapter4.html> for details.
    DECKPAM = b'=',

    /// DECKPNM (DEC Keypad Numeric Mode) Escape Sequence.
    ///
    /// This escape sequence is used to disable the application keypad mode in
    /// a DEC VT220 terminal and switch back to the normal keypad mode where
    /// keys send their regular ASCII characters.
    ///
    /// See <https://vt100.net/docs/vt220-rm/chapter4.html> for details.
    DECKPNM = b'>',
}

pub trait EscapeSequence {
    const INTRO: EscapeSequenceIntroducer;
    const PRIVATE: Option<u8>;
    const PARAMS: EscapeSequenceParams;
    const INTERMEDIATE: Intermediate;
    const FINAL: u8;
}

pub type Handler = fn(&[EscapeSequenceParam]);

#[derive(Copy, Clone, Debug)]
pub struct EscapeSequenceMatchEntry {
    /// Sequence name.
    pub name: &'static str,
    /// Sequence introducer kind.
    pub intro: EscapeSequenceIntroducer,
    /// A byte prefix following the introducer, this would include
    /// the private byte and any static params.
    pub prefix: &'static [u8],
    /// Final byte if fixed; otherwise 0 to indicate “any final with flags”.
    pub final_byte: u8,
    /// Handler function to call on match.
    pub handler: Handler,
}

#[distributed_slice]
pub static ESCAPE_SEQUENCE_REGISTRY: [EscapeSequenceMatchEntry] = [..];

impl From<EscapeSequenceParam> for bool {
    fn from(param: EscapeSequenceParam) -> Self {
        param.first().is_some_and(|&v| v != 0)
    }
}

impl From<&EscapeSequenceParam> for bool {
    fn from(param: &EscapeSequenceParam) -> Self {
        param.first().is_some_and(|&v| v != 0)
    }
}

impl From<EscapeSequenceParam> for String {
    fn from(param: EscapeSequenceParam) -> Self {
        String::from_utf8_lossy(&param).into_owned()
    }
}

impl From<&EscapeSequenceParam> for String {
    fn from(param: &EscapeSequenceParam) -> Self {
        String::from_utf8_lossy(param).into_owned()
    }
}

impl From<EscapeSequenceParam> for char {
    fn from(param: EscapeSequenceParam) -> Self {
        param.first().copied().map_or('\0', |b| b as char)
    }
}

impl From<&EscapeSequenceParam> for char {
    fn from(param: &EscapeSequenceParam) -> Self {
        param.first().copied().map_or('\0', |b| b as char)
    }
}

// Macro to implement From<EscapeSequenceParam> for numeric types
macro_rules! impl_from_param_numeric {
    ($($t:ty),+ $(,)?) => {
        $(
            impl From<EscapeSequenceParam> for $t {
                #[allow(clippy::cast_lossless, clippy::cast_possible_wrap)]
                fn from(param: EscapeSequenceParam) -> Self {
                    param.first().copied().unwrap_or(0) as $t
                }
            }

            impl From<&EscapeSequenceParam> for $t {
                #[allow(clippy::cast_lossless, clippy::cast_possible_wrap)]
                fn from(param: &EscapeSequenceParam) -> Self {
                    param.first().copied().unwrap_or(0) as $t
                }
            }
        )+
    };
}

impl_from_param_numeric! {
    u8, i8, u16, i16, u32, i32, u64, i64, usize, isize
}
