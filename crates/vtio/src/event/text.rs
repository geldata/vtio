//! Plain text event types.

use std::io::Write;

/// Plain text event wrapping a `&str`.
///
/// This event is emitted for raw text output that has been validated as UTF-8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlainText<'a>(pub &'a str);

better_any::tid! {PlainText<'a>}

impl vtansi::AnsiEncode for PlainText<'_> {
    #[inline]
    fn encode_ansi_into<W: Write + ?Sized>(
        &self,
        sink: &mut W,
    ) -> Result<usize, vtansi::EncodeError> {
        sink.write_all(self.0.as_bytes())
            .map_err(vtansi::EncodeError::IOError)?;
        Ok(self.0.len())
    }
}

impl<'a> vtansi::AnsiEvent<'a> for PlainText<'a> {
    #[inline]
    fn ansi_control_kind(&self) -> Option<vtansi::AnsiControlFunctionKind> {
        None
    }

    #[inline]
    fn ansi_direction(&self) -> vtansi::AnsiControlDirection {
        vtansi::AnsiControlDirection::Output
    }

    vtansi::impl_ansi_event_encode!();
}
