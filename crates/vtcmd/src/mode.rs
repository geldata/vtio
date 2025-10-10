//! Mode control commands.

use vtansi::encode::{Encode, EncodeError};
use vtansi::{write_csi, write_esc};

/// Enable bracketed paste mode.
pub struct EnableBracketedPaste;

impl Encode for EnableBracketedPaste {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "?2004h")
    }
}

/// Disable bracketed paste mode.
pub struct DisableBracketedPaste;

impl Encode for DisableBracketedPaste {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "?2004l")
    }
}

/// Enable focus reporting.
pub struct EnableFocusReporting;

impl Encode for EnableFocusReporting {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "?1004h")
    }
}

/// Disable focus reporting.
pub struct DisableFocusReporting;

impl Encode for DisableFocusReporting {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "?1004l")
    }
}

/// Enable application keypad mode (DECKPAM).
pub struct EnableApplicationKeypad;

impl Encode for EnableApplicationKeypad {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        // DECKPAM: ESC = (not a CSI sequence)
        write_esc!(buf, "=")
    }
}

/// Disable application keypad mode (DECKPNM).
pub struct DisableApplicationKeypad;

impl Encode for DisableApplicationKeypad {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        // DECKPNM: ESC > (not a CSI sequence)
        write_esc!(buf, ">")
    }
}

/// Begin synchronized update.
pub struct BeginSynchronizedUpdate;

impl Encode for BeginSynchronizedUpdate {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "?2026h")
    }
}

/// End synchronized update.
pub struct EndSynchronizedUpdate;

impl Encode for EndSynchronizedUpdate {
    #[inline]
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        write_csi!(buf, "?2026l")
    }
}
