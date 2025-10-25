//! Encoding utilities for ANSI control sequences.
//!
//! This module provides traits and utilities for encoding typed Rust values
//! into ANSI byte sequences.
//!
//! # Encoding
//!
//! The [`AnsiEncode`] trait is the primary interface for encoding values into
//! ANSI sequences. Types can implement this trait directly, or implement
//! [`ToAnsi`] for a more ergonomic conversion.
//!
//! For types that always encode to a constant string, implement
//! [`StaticAnsiEncode`] which provides automatic implementations of the other
//! encoding traits.
//!
//! # Parsing
//!
//! For parsing ANSI sequences into typed values, see the [`parse`](crate::parse)
//! module.

use core::fmt;
use std::io::{self, Write};

/// Write an integer to a buffer without allocation.
///
/// Uses the `itoa` crate for efficient integer-to-string conversion.
///
/// # Errors
///
/// Return an error if the buffer is too small to hold the integer.
#[inline]
pub fn write_int<W: io::Write + ?Sized>(
    sink: &mut W,
    value: impl itoa::Integer,
) -> Result<usize, EncodeError> {
    let mut buffer = itoa::Buffer::new();
    let s = buffer.format(value);
    write_str_into(sink, s)
}

pub struct CountingWriter<W> {
    inner: W,
    written: usize,
    overflow: usize,
}

impl<W: io::Write> CountingWriter<W> {
    #[inline]
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            written: 0,
            overflow: 0,
        }
    }
    #[inline]
    pub fn written(&self) -> usize {
        self.written
    }
    #[inline]
    pub fn overflow(&self) -> usize {
        self.overflow
    }
    #[inline]
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: io::Write> io::Write for CountingWriter<W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let requested = buf.len();
        let n = self.inner.write(buf)?;
        self.written += n;
        if n < requested {
            self.overflow += requested - n;
        }
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

/// Copy a byte slice into the provided sink,
/// returning the number of bytes written.
///
/// # Errors
///
/// Return an error if the buffer is too small to hold the data.
#[inline]
pub fn write_bytes_into<W: io::Write + ?Sized>(
    sink: &mut W,
    s: &[u8],
) -> Result<usize, EncodeError> {
    let mut w = CountingWriter::new(sink);
    match w.write(s) {
        Err(ref e) if e.kind() == io::ErrorKind::WriteZero => {
            Err(EncodeError::BufferOverflow(w.overflow()))
        }
        Err(e) => Err(EncodeError::IOError(e)),
        Ok(_n) if w.overflow() > 0 => Err(EncodeError::BufferOverflow(w.overflow())),
        Ok(n) => Ok(n),
    }
}

/// Copy a UTF-8 string into the provided buffer,
/// returning the number of bytes written.
///
/// # Errors
///
/// Return an error if the buffer is too small to hold the string.
#[inline]
pub fn write_str_into<W: io::Write + ?Sized>(sink: &mut W, s: &str) -> Result<usize, EncodeError> {
    write_bytes_into(sink, s.as_bytes())
}

/// Trait for types that can be efficiently written to a buffer.
///
/// This trait is implemented for string slices and integer types, allowing
/// the `write_*` macros to accept a sequence of literals and integers
/// without heap allocation or the overhead of `write_fmt`.
pub trait AnsiEncode {
    /// Write this value to the buffer.
    ///
    /// # Errors
    ///
    /// Return an error if the buffer is too small to hold the value.
    fn encode_ansi_into<W: io::Write + ?Sized>(&self, sink: &mut W) -> Result<usize, EncodeError>;

    /// Encode this value as an ANSI control sequence directly into a byte slice.
    ///
    /// # Errors
    ///
    /// Return an error if the buffer is too small to hold the encoded value.
    #[inline]
    fn encode_ansi_into_slice(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        self.encode_ansi_into(&mut &mut buf[..])
    }

    /// Encode this value as an ANSI control sequence and return the resulting bytes.
    ///
    /// # Errors
    ///
    /// Return an error if the buffer is too small to hold the encoded value.
    #[inline]
    fn encode_ansi(&self) -> Result<Vec<u8>, EncodeError> {
        let mut v: Vec<u8> = Vec::with_capacity(5);
        self.encode_ansi_into(&mut v)?;
        Ok(v)
    }
}

pub trait ToAnsi {
    fn to_ansi(&self) -> impl AnsiEncode;
}

impl<T: ToAnsi> AnsiEncode for T {
    #[inline]
    fn encode_ansi_into<W: io::Write + ?Sized>(&self, sink: &mut W) -> Result<usize, EncodeError> {
        self.to_ansi().encode_ansi_into(sink)
    }
}

impl ToAnsi for () {
    fn to_ansi(&self) -> impl AnsiEncode {
        ""
    }
}

impl AnsiEncode for &str {
    #[inline]
    fn encode_ansi_into<W: io::Write + ?Sized>(&self, sink: &mut W) -> Result<usize, EncodeError> {
        write_str_into(sink, self)
    }
}

impl AnsiEncode for String {
    #[inline]
    fn encode_ansi_into<W: io::Write + ?Sized>(&self, sink: &mut W) -> Result<usize, EncodeError> {
        write_str_into(sink, self)
    }
}

macro_rules! write_int_seq {
    ($(#[$meta:meta])* $type:ty) => {
        $(#[$meta])*
        impl $crate::encode::AnsiEncode for $type {
            #[inline]
            fn encode_ansi_into<W: io::Write + ?Sized>(&self, sink: &mut W) -> Result<usize, EncodeError> {
                write_int(sink, *self)
            }
        }

        $(#[$meta])*
        impl $crate::encode::AnsiEncode for &$type {
            #[inline]
            fn encode_ansi_into<W: io::Write + ?Sized>(&self, sink: &mut W) -> Result<usize, EncodeError> {
                write_int(sink, **self)
            }
        }

        $(#[$meta])*
        impl $crate::encode::AnsiEncode for &mut $type {
            #[inline]
            fn encode_ansi_into<W: io::Write + ?Sized>(&self, sink: &mut W) -> Result<usize, EncodeError> {
                write_int(sink, **self)
            }
        }
    };
}

write_int_seq!(u8);
write_int_seq!(u16);
write_int_seq!(u32);
write_int_seq!(u64);
write_int_seq!(usize);
write_int_seq!(i8);
write_int_seq!(i16);
write_int_seq!(i32);
write_int_seq!(i64);
write_int_seq!(isize);

impl AnsiEncode for char {
    #[inline]
    fn encode_ansi_into<W: io::Write + ?Sized>(&self, sink: &mut W) -> Result<usize, EncodeError> {
        let mut buf = [0u8; 4];
        let s = self.encode_utf8(&mut buf);
        write_str_into(sink, s)
    }
}

impl AnsiEncode for &char {
    #[inline]
    fn encode_ansi_into<W: io::Write + ?Sized>(&self, sink: &mut W) -> Result<usize, EncodeError> {
        let mut buf = [0u8; 4];
        let s = self.encode_utf8(&mut buf);
        write_str_into(sink, s)
    }
}

impl AnsiEncode for &mut char {
    #[inline]
    fn encode_ansi_into<W: io::Write + ?Sized>(&self, sink: &mut W) -> Result<usize, EncodeError> {
        let mut buf = [0u8; 4];
        let s = self.encode_utf8(&mut buf);
        write_str_into(sink, s)
    }
}

impl AnsiEncode for bool {
    #[inline]
    fn encode_ansi_into<W: io::Write + ?Sized>(&self, sink: &mut W) -> Result<usize, EncodeError> {
        write_str_into(sink, if *self { "1" } else { "0" })
    }
}

impl AnsiEncode for &bool {
    #[inline]
    fn encode_ansi_into<W: io::Write + ?Sized>(&self, sink: &mut W) -> Result<usize, EncodeError> {
        write_str_into(sink, if **self { "1" } else { "0" })
    }
}

impl AnsiEncode for &mut bool {
    #[inline]
    fn encode_ansi_into<W: io::Write + ?Sized>(&self, sink: &mut W) -> Result<usize, EncodeError> {
        write_str_into(sink, if **self { "1" } else { "0" })
    }
}

#[derive(Debug)]
pub enum EncodeError {
    BufferOverflow(usize),
    IOError(std::io::Error),
}

impl From<EncodeError> for io::Error {
    fn from(err: EncodeError) -> Self {
        match err {
            EncodeError::BufferOverflow(n) => io::Error::new(
                io::ErrorKind::WriteZero,
                format!("buffer overflow: {n} bytes could not be written"),
            ),
            EncodeError::IOError(e) => e,
        }
    }
}

impl std::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EncodeError::IOError(e) => Some(e),
            EncodeError::BufferOverflow(_) => None,
        }
    }
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodeError::BufferOverflow(n) => {
                write!(f, "buffer overflow: {n} bytes could not be written")
            }
            EncodeError::IOError(e) => write!(f, "I/O error: {e}"),
        }
    }
}

pub trait EncodedLen {
    /// Return the upper bound of the buffer size needed for `encode`.
    ///
    /// This value guarantees that `encode` will succeed with a buffer of
    /// this size. It does not have to be exact and should be computed as
    /// quickly as possible.
    fn encoded_buf_len(&self) -> usize;
}

/// Trait for types with a compile-time known encoded length.
///
/// This trait provides a constant upper bound for the buffer size needed to
/// encode a value. Types implementing this trait can have their buffer
/// requirements determined at compile time.
///
/// Types implementing this trait automatically get a default implementation
/// of [`EncodedLen::encoded_buf_len`] that returns this constant value.
pub trait StaticEncodedLen {
    /// The maximum number of bytes needed to encode this type.
    ///
    /// This value represents an upper bound that guarantees `encode` will
    /// succeed with a buffer of this size.
    const ENCODED_LEN: usize;
}

impl<T: StaticEncodedLen> EncodedLen for T {
    #[inline]
    fn encoded_buf_len(&self) -> usize {
        Self::ENCODED_LEN
    }
}

/// Trait for types that encode to a static byte sequence.
///
/// This trait is for types that always encode to the same constant string,
/// such as terminal control sequences without parameters. Types implementing
/// this trait automatically get `Encode`, `EncodedLen`, and `Write`
/// implementations via blanket impls.
pub trait StaticAnsiEncode {
    /// The static string this type encodes to.
    const STR: &'static str;
}

impl<T: StaticAnsiEncode> StaticEncodedLen for T {
    const ENCODED_LEN: usize = Self::STR.len();
}

impl<T: StaticAnsiEncode> ToAnsi for T {
    #[inline]
    fn to_ansi(&self) -> impl AnsiEncode {
        Self::STR
    }
}

/// Define a composite const encodeable that combines multiple encodeables.
#[macro_export]
macro_rules! const_composite {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident = [
            $($command:path),* $(,)?
        ];
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $vis struct $name;

        impl $crate::encode::StaticEncodedLen for $name {
            const ENCODED_LEN: usize = 0 $(+ <$command>::ENCODED_LEN)*;
        }

        impl $crate::encode::AnsiEncode for $name {
            #[inline]
            fn encode_ansi_into<W: ::std::io::Write + ?::std::marker::Sized>(
                &self,
                buf: &mut W
            ) -> Result<usize, $crate::encode::EncodeError> {
                // Use a stack-allocated buffer for const-length commands
                let mut stack_buf = [0u8; <Self as $crate::encode::StaticEncodedLen>::ENCODED_LEN];
                let mut offset = 0;

                $(
                    offset += $command.encode_ansi_into(&mut &mut stack_buf[offset..])?;
                )*

                buf.write_all(&stack_buf[..offset])
                    .map_err($crate::encode::EncodeError::IOError)?;
                Ok(offset)
            }
        }
    };
}

/// Helper function for encoding struct fields as delimited values.
///
/// This function joins the encoded values of struct fields with the specified
/// delimiter. It's used by derived `ToAnsi` implementations for structs with
/// value format.
///
/// # Examples
///
/// ```ignore
/// let parts = vec!["100".to_string(), "200".to_string()];
/// let result = encode_delimited_values(&parts, ";");
/// assert_eq!(result, "100;200");
/// ```
#[inline]
#[must_use]
pub fn encode_delimited_values(parts: &[String], delimiter: &str) -> String {
    parts.join(delimiter)
}

/// Helper function for encoding struct fields as delimited values, omitting
/// trailing None values.
///
/// This function encodes struct fields with optional trailing fields by
/// finding the last `Some` value and encoding up to that point. Trailing
/// `None` values are omitted from the output. It's used by derived `ToAnsi`
/// implementations for structs with optional fields in value format.
///
/// # Examples
///
/// ```ignore
/// let parts = vec![
///     Some("100".to_string()),
///     Some("200".to_string()),
///     None,
/// ];
/// let result = encode_delimited_values_with_optional(&parts, ";");
/// assert_eq!(result, "100;200");
/// ```
#[inline]
#[must_use]
pub fn encode_delimited_values_with_optional(parts: &[Option<String>], delimiter: &str) -> String {
    // Find the last Some value
    let last_some_idx = parts
        .iter()
        .enumerate()
        .rev()
        .find_map(|(idx, opt)| opt.as_ref().map(|_| idx));

    match last_some_idx {
        Some(idx) => {
            // Encode up to and including the last Some
            parts[..=idx]
                .iter()
                .map(|opt| opt.as_ref().map_or(String::new(), std::clone::Clone::clone))
                .collect::<Vec<_>>()
                .join(delimiter)
        }
        None => String::new(),
    }
}

/// Helper function for encoding struct fields as key=value pairs.
///
/// This function creates a string with `key=value` pairs separated by the
/// specified delimiter. It's used by derived `ToAnsi` implementations for
/// structs with key=value format.
///
/// # Examples
///
/// ```ignore
/// let pairs = vec![("width", "800"), ("height", "600")];
/// let result = encode_keyvalue_pairs(&pairs, ";");
/// assert_eq!(result, "width=800;height=600");
/// ```
#[inline]
#[must_use]
pub fn encode_keyvalue_pairs(pairs: &[(&str, String)], delimiter: &str) -> String {
    pairs
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join(delimiter)
}
