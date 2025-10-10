use core::fmt;
use std::io;

use tinyvec::tiny_vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeError {
    BufferOverflow(usize),
}

/// Copy a UTF-8 string into the provided buffer,
/// returning the number of bytes written.
///
/// # Errors
///
/// Return an error if the buffer is too small to hold the string.
#[inline]
pub fn write_str_into(buf: &mut [u8], s: &str) -> Result<usize, EncodeError> {
    let len = s.len();
    if buf.len() < len {
        return Err(EncodeError::BufferOverflow(len));
    }
    buf[..len].copy_from_slice(s.as_bytes());
    Ok(len)
}

pub trait EncodedLen {
    /// Return the upper bound of the buffer size needed for `encode`.
    ///
    /// This value guarantees that `encode` will succeed with a buffer of
    /// this size. It does not have to be exact and should be computed as
    /// quickly as possible.
    fn encoded_buf_len(&self) -> usize;
}

pub trait Write {
    /// Write encoded bytes to the provided writer.
    ///
    /// # Errors
    ///
    /// Return an error if encoding fails or if writing to the writer fails.
    fn write<W: io::Write>(&mut self, writer: &mut W) -> io::Result<usize>;
}

pub trait Encode {
    /// Encode this value into the provided buffer.
    ///
    /// # Errors
    ///
    /// Return an error if the buffer is too small to hold the encoded value.
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError>;
}

impl<T: Encode+EncodedLen> Write for T {
    /// Write encoded bytes to the provided writer.
    ///
    /// This implementation uses a temporary buffer to encode the value and then
    /// writes it to the writer.
    ///
    /// # Errors
    ///
    /// Return an error if encoding fails or if writing to the writer fails.
    fn write<W: io::Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        let mut buf = tiny_vec!([u8; 64]);
        let len = self
            .encode(&mut buf)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{e:?}")))?;
        writer.write_all(&buf[..len])?;
        Ok(len)
    }
}

/// Trait for types with a compile-time known encoded length.
///
/// This trait provides a constant upper bound for the buffer size needed to
/// encode a value. Types implementing this trait can have their buffer
/// requirements determined at compile time.
///
/// Types implementing this trait automatically get a default implementation
/// of [`Encode::encoded_buf_len`] that returns this constant value.
pub trait ConstEncodedLen {
    /// The maximum number of bytes needed to encode this type.
    ///
    /// This value represents an upper bound that guarantees `encode` will
    /// succeed with a buffer of this size.
    const ENCODED_LEN: usize;
}

impl<T: ConstEncodedLen> EncodedLen for T {
    #[inline]
    fn encoded_buf_len(&self) -> usize {
        Self::ENCODED_LEN
    }
}

/// Trait for types that encode to a static byte sequence.
///
/// This trait is for types that always encode to the same constant string,
/// such as terminal control sequences without parameters. Types implementing
/// this trait automatically get an `Encode` implementation via a blanket impl.
pub trait ConstEncode {
    /// The static string this type encodes to.
    const STR: &'static str;
}

impl<T: ConstEncode> ConstEncodedLen for T {
    const ENCODED_LEN: usize = T::STR.len();
}

impl<T: ConstEncode> Encode for T {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_str_into(buf, Self::STR)
    }
}

impl<T: ConstEncode> Write for T {
    /// Write encoded bytes to the provided writer.
    ///
    /// This method uses a temporary buffer to encode the value and then
    /// writes it to the writer.
    ///
    /// # Errors
    ///
    /// Return an error if encoding fails or if writing to the writer fails.
    #[inline]
    fn write<W: io::Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(Self::STR.as_bytes())?;
        Ok(Self::STR.len())
    }
}

/// Internal adapter that writes `&str` chunks into a byte slice.
struct SliceFmt<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl fmt::Write for SliceFmt<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let need = s.len();
        let end = self.pos.checked_add(need).ok_or(fmt::Error)?;
        if end > self.buf.len() {
            return Err(fmt::Error);
        }
        self.buf[self.pos..end].copy_from_slice(s.as_bytes());
        self.pos = end;
        Ok(())
    }
}

/// Write formatted arguments into the provided buffer.
///
/// # Errors
///
/// Return an error if the buffer is too small to hold the formatted string.
#[inline]
pub fn write_fmt_into(buf: &mut [u8], args: fmt::Arguments<'_>) -> Result<usize, EncodeError> {
    let mut w = SliceFmt { buf, pos: 0 };
    fmt::write(&mut w, args).map_err(|_| EncodeError::BufferOverflow(0))?;
    Ok(w.pos)
}

/// Copy a UTF-8 string literal into a buffer at compile time.
///
/// This is a zero-cost macro version of [`write_str_into`] that inlines
/// the copy operation directly, avoiding function call overhead. When used
/// with compile-time string literals (such as those from [`csi!`],
/// [`esc!`], etc.), the length is known at compile time.
///
/// [`csi!`]: crate::csi
/// [`esc!`]: crate::esc
#[macro_export]
macro_rules! write_const_str_into {
    ($buf:expr, $s:expr) => {{
        const S: &str = $s;
        const LEN: usize = S.len();
        let buf = $buf;
        if buf.len() < LEN {
            Err($crate::encode::EncodeError::BufferOverflow(LEN))
        } else {
            buf[..LEN].copy_from_slice(S.as_bytes());
            Ok(LEN)
        }
    }};
}

/// `write!`-like macro that targets a `&mut [u8]`.
#[macro_export]
macro_rules! write_into {
    ($buf:expr, $($arg:tt)*) => {{
        $crate::encode::write_fmt_into($buf, core::format_args!($($arg)*))
    }};
}
