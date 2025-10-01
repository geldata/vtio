use core::fmt;

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

pub trait Encode {
    /// Encode this value into the provided buffer.
    ///
    /// # Errors
    ///
    /// Return an error if the buffer is too small to hold the encoded value.
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError>;
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

/// `write!`-like macro that targets a `&mut [u8]`.
#[macro_export]
macro_rules! write_into {
    ($buf:expr, $($arg:tt)*) => {{
        $crate::encode::write_fmt_into($buf, core::format_args!($($arg)*))
    }};
}
