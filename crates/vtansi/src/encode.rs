use core::fmt;
use std::io;

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

/// `write!`-like macro that targets a `&mut [u8]` and returns
/// `Result<usize, EncodeError>`.
#[macro_export]
macro_rules! write_into {
    ($sink:expr, $($arg:tt)*) => {{
        use std::io::Write as _;
        let mut w = $crate::encode::CountingWriter::new($sink);
        match std::io::Write::write_fmt(&mut w, core::format_args!($($arg)*)) {
            Err(ref e) if e.kind() == std::io::ErrorKind::WriteZero => {
                Err($crate::encode::EncodeError::BufferOverflow(w.overflow()))
            },
            Err(e) => Err($crate::encode::EncodeError::IOError(e)),
            Ok(()) => Ok(w.written()),
        }
    }};
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

/// Trait for writing encoded data to an `io::Write` destination.
///
/// This trait is automatically implemented for all types that implement both
/// [`Encode`] and [`EncodedLen`]. This includes:
/// - Types implementing [`ConstEncode`] (which automatically get both traits)
/// - Types manually implementing both [`Encode`] and [`EncodedLen`]
///
/// The automatic implementation uses a temporary buffer (stack-allocated for
/// small encodings ≤256 bytes, heap-allocated for larger ones) to encode
/// the value before writing it to the destination.
///
/// # Example
///
/// ```ignore
/// use vtansi::encode::Write;
/// use std::io;
///
/// let mut cmd = SomeCommand::new();
/// let mut buffer = Vec::new();
/// cmd.write(&mut buffer)?;
/// ```
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

/// Trait for types with a compile-time known encoded length.
///
/// This trait provides a constant upper bound for the buffer size needed to
/// encode a value. Types implementing this trait can have their buffer
/// requirements determined at compile time.
///
/// Types implementing this trait automatically get a default implementation
/// of [`EncodedLen::encoded_buf_len`] that returns this constant value.
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

impl<T: Encode + ConstEncodedLen> Write for T {
    /// Write encoded bytes to the provided writer.
    ///
    /// This implementation uses a temporary buffer to encode the value and
    /// then writes it to the writer. For small encodings (≤256 bytes), a
    /// stack buffer is used; for larger encodings, heap allocation is used.
    ///
    /// # Errors
    ///
    /// Return an error if encoding fails or if writing to the writer fails.
    fn write<W: io::Write>(&mut self, writer: &mut W) -> io::Result<usize> {
        let buf_len = self.encoded_buf_len();

        // Use stack buffer for small encodings, heap for large ones
        if buf_len <= 64 {
            let mut stack_buf = [0u8; 64];
            let len = self.encode(&mut stack_buf[..buf_len]).unwrap_or_else(|_| {
                panic!(
                    "encoded_buf_len() for {:?} returned a too small value",
                    std::any::type_name::<T>(),
                )
            });
            writer.write_all(&stack_buf[..len])?;
            Ok(len)
        } else {
            let mut heap_buf = Vec::with_capacity(buf_len);
            let len = self.encode(&mut heap_buf).unwrap_or_else(|_| {
                panic!(
                    "encoded_buf_len() for {:?} returned a too small value",
                    std::any::type_name::<T>(),
                )
            });
            writer.write_all(&heap_buf[..len])?;
            Ok(len)
        }
    }
}

/// Trait for types that encode to a static byte sequence.
///
/// This trait is for types that always encode to the same constant string,
/// such as terminal control sequences without parameters. Types implementing
/// this trait automatically get `Encode`, `EncodedLen`, and `Write`
/// implementations via blanket impls.
pub trait ConstEncode {
    /// The static string this type encodes to.
    const STR: &'static str;
}

impl<T: ConstEncode> Encode for T {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_str_into(buf, Self::STR)
    }
}

impl<T: ConstEncode> ConstEncodedLen for T {
    const ENCODED_LEN: usize = Self::STR.len();
}
