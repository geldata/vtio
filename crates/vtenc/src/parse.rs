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
            ParseError::InvalidString(msg) => write!(f, "invalid string: {msg}"),
            ParseError::InvalidNum(msg) => write!(f, "invalid number: {msg}"),
            ParseError::InvalidValue(msg) => write!(f, "invalid value: {msg}"),
        }
    }
}

impl std::error::Error for ParseError {}
/// Parse a value from an ANSI byte slice infallibly.
///
/// This trait is for types that can always be successfully parsed from
/// a byte slice, such as types with default values or types that accept
/// any input.
pub trait FromAnsi<'a>: Sized {
    /// Parse a value from an ANSI byte slice.
    fn from_ansi(bytes: &'a [u8]) -> Self;
}

/// Parse a value from an ANSI byte slice with error handling.
///
/// This is the primary trait for parsing ANSI sequences. Types with
/// fixed-length requirements should implement `TryFromAnsiBytes` instead,
/// which can be bridged to this trait using the [`fixed_length_ansi!`](crate::fixed_length_ansi) macro.
pub trait TryFromAnsi<'a>: Sized {
    /// Parse a value from an ANSI byte slice.
    ///
    /// # Errors
    ///
    /// Return an error if the byte slice is invalid for this type.
    fn try_from_ansi(bytes: &'a [u8]) -> Result<Self, ParseError>;
}

/// Blanket implementation of `TryFromAnsi` for types that
/// define an infallible version.
impl<'a, T> TryFromAnsi<'a> for T
where
    T: FromAnsi<'a>,
{
    fn try_from_ansi(bytes: &'a [u8]) -> Result<Self, ParseError> {
        Ok(T::from_ansi(bytes))
    }
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

impl<'a> TryFromAnsi<'a> for &'a str {
    fn try_from_ansi(bytes: &'a [u8]) -> Result<Self, ParseError> {
        str::from_utf8(bytes).map_err(|e| ParseError::InvalidString(e.to_string()))
    }
}

impl<'a> TryFromAnsi<'a> for String {
    fn try_from_ansi(bytes: &'a [u8]) -> Result<Self, ParseError> {
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

/// Helper function for parsing delimited values in positional order.
///
/// This function splits the input string by the delimiter and returns a
/// vector of trimmed string slices. It's used by derived `FromAnsi`
/// implementations for structs with value format.
///
/// # Errors
///
/// Return an error if the number of parts doesn't match the expected count.
pub fn parse_delimited_values<'a>(
    s: &'a str,
    delimiter: &str,
    expected_count: usize,
) -> Result<Vec<&'a str>, ParseError> {
    let parts: Vec<&str> = s.split(delimiter).collect();
    
    if parts.len() != expected_count {
        return Err(ParseError::InvalidValue(format!(
            "expected {expected_count} fields, got {}",
            parts.len()
        )));
    }
    
    Ok(parts)
}

/// Helper function for parsing key=value pairs from a delimited string.
///
/// This function returns an iterator that yields parsed `key=value` pairs.
/// It's used by derived `FromAnsi` implementations for structs with key=value
/// format.
///
/// # Examples
///
/// ```ignore
/// let pairs: Result<Vec<_>, _> = parse_keyvalue_pairs("a=1;b=2", ";").collect();
/// ```
pub fn parse_keyvalue_pairs<'a>(
    s: &'a str,
    delimiter: &'a str,
) -> impl Iterator<Item = Result<(&'a str, &'a str), ParseError>> + 'a {
    s.split(delimiter)
        .map(str::trim)
        .filter(|pair| !pair.is_empty())
        .map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next().unwrap_or("").trim();
            let value = parts
                .next()
                .ok_or_else(|| {
                    ParseError::InvalidValue(format!("invalid key=value pair: {pair}"))
                })?
                .trim();
            Ok((key, value))
        })
}
