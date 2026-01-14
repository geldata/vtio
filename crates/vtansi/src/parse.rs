//! Parsing helpers and traits for data encoded in ANSI control sequences.

use core::fmt;

/// Error type for ANSI parsing operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The input has the wrong length.
    WrongLen { expected: usize, got: usize },
    /// The input contains an invalid number.
    InvalidNum(String),
    /// The input is empty but a value was expected.
    Empty,
    /// The input contains an invalid string.
    InvalidString(String),
    /// The input contains an invalid value.
    InvalidValue(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::WrongLen { expected, got } => {
                write!(f, "wrong length: expected {expected}, got {got}")
            }
            ParseError::Empty => write!(f, "empty input"),
            ParseError::InvalidString(msg) => {
                write!(f, "invalid string: {msg}")
            }
            ParseError::InvalidNum(msg) => write!(f, "invalid number: {msg}"),
            ParseError::InvalidValue(msg) => write!(f, "invalid value: {msg}"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Parse a value from an ANSI byte slice with error handling.
///
/// This is the primary trait for parsing ANSI-encoded values and
/// control function sequences.
pub trait TryFromAnsi<'a>: Sized {
    /// Parse a value from an ANSI byte slice.
    ///
    /// # Errors
    ///
    /// Return an error if the byte slice is invalid for this type.
    fn try_from_ansi(bytes: &'a [u8]) -> Result<Self, ParseError>;
}

/// Parse a value from an iterator of ANSI parameter byte slices.
///
/// This trait is used for flattened fields that consume multiple parameters
/// from the iterator. The implementation should consume as many parameters
/// as needed and leave the iterator positioned after the last consumed parameter.
pub trait TryFromAnsiIter<'a>: Sized {
    /// Parse a value from an iterator of ANSI parameter byte slices.
    ///
    /// # Errors
    ///
    /// Return an error if the parameters are invalid for this type.
    fn try_from_ansi_iter<I>(iter: &mut I) -> Result<Self, ParseError>
    where
        I: Iterator<Item = &'a [u8]>;
}

impl TryFromAnsi<'_> for bool {
    fn try_from_ansi(bytes: &[u8]) -> Result<Self, ParseError> {
        match bytes.len() {
            1 if bytes[0] == b'1' => Ok(true),
            1 if bytes[0] == b'0' => Ok(false),
            n => Err(ParseError::WrongLen {
                expected: 1,
                got: n,
            }),
        }
    }
}

impl TryFromAnsi<'_> for char {
    fn try_from_ansi(bytes: &[u8]) -> Result<Self, ParseError> {
        match bytes.len() {
            1 => Ok(char::from(bytes[0])),
            n => Err(ParseError::InvalidValue(format!(
                "invalid ASCII character: expected exactly one byte, got {n}"
            ))),
        }
    }
}

impl<'a> TryFromAnsi<'a> for &'a str {
    fn try_from_ansi(bytes: &'a [u8]) -> Result<Self, ParseError> {
        str::from_utf8(bytes)
            .map_err(|e| ParseError::InvalidString(e.to_string()))
    }
}

impl TryFromAnsi<'_> for String {
    fn try_from_ansi(bytes: &[u8]) -> Result<Self, ParseError> {
        str::from_utf8(bytes)
            .map_err(|e| ParseError::InvalidString(e.to_string()))
            .map(String::from)
    }
}

// Macro to implement `TryFromAnsi` for numeric types
macro_rules! impl_try_from_ansi_numeric {
    ($($t:ty),+ $(,)?) => {
        $(
            impl TryFromAnsi<'_> for $t {
                fn try_from_ansi(bytes: &[u8]) -> Result<Self, ParseError> {
                    atoi_simd::parse::<$t>(bytes).map_err(
                        |e| ParseError::InvalidNum(e.to_string()),
                    )
                }
            }
        )+
    };
}

impl_try_from_ansi_numeric! {
    u8, i8, u16, i16, u32, i32, u64, i64, usize, isize
}

/// Helper function for parsing key=value pairs from a delimited bytestring.
///
/// This function returns an iterator that yields parsed `key=value` pairs.
/// It's used by derived `FromAnsi` implementations for structs with key=value
/// format.
///
/// # Examples
///
/// ```
/// use vtansi::parse::parse_keyvalue_pairs;
/// let pairs: Result<Vec<_>, _> = parse_keyvalue_pairs(b"a=1;b=2", b';').collect();
/// ```
#[inline]
pub fn parse_keyvalue_pairs(
    bytes: &[u8],
    delimiter: u8,
) -> impl Iterator<Item = Result<(&[u8], &[u8]), ParseError>> + '_ {
    parse_keyvalue_pairs_from_iter(bytes.split(move |&b| b == delimiter))
}

#[inline]
pub fn parse_keyvalue_pairs_from_slice<'a>(
    pairs: &'a [&'a [u8]],
) -> impl Iterator<Item = Result<(&'a [u8], &'a [u8]), ParseError>> + 'a {
    parse_keyvalue_pairs_from_iter(pairs.iter().copied())
}

#[inline]
pub fn parse_keyvalue_pairs_from_iter<'a>(
    pairs: impl Iterator<Item = &'a [u8]> + 'a,
) -> impl Iterator<Item = Result<(&'a [u8], &'a [u8]), ParseError>> + 'a {
    pairs.map(|pair| {
        let mut parts = pair.splitn(2, |&b| b == b'=');
        let key = parts.next().ok_or_else(|| {
            ParseError::InvalidValue(format!(
                "invalid key=value pair: {pair:?}"
            ))
        })?;
        let value = parts.next().ok_or_else(|| {
            ParseError::InvalidValue(format!(
                "invalid key=value pair: {pair:?}"
            ))
        })?;
        Ok((key, value))
    })
}
