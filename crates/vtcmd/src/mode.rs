//! Mode control commands.

use vtansi::csi;
use vtansi::encode::ConstEncode;
use vtansi::esc;

/// Enable bracketed paste mode.
pub struct EnableBracketedPaste;

impl ConstEncode for EnableBracketedPaste {
    const STR: &'static str = csi!("?2004h");
}

/// Disable bracketed paste mode.
pub struct DisableBracketedPaste;

impl ConstEncode for DisableBracketedPaste {
    const STR: &'static str = csi!("?2004l");
}

/// Enable focus reporting.
pub struct EnableFocusReporting;

impl ConstEncode for EnableFocusReporting {
    const STR: &'static str = csi!("?1004h");
}

/// Disable focus reporting.
pub struct DisableFocusReporting;

impl ConstEncode for DisableFocusReporting {
    const STR: &'static str = csi!("?1004l");
}

/// Enable application keypad mode (DECKPAM).
pub struct EnableApplicationKeypad;

impl ConstEncode for EnableApplicationKeypad {
    const STR: &'static str = esc!("=");
}

/// Disable application keypad mode (DECKPNM).
pub struct DisableApplicationKeypad;

impl ConstEncode for DisableApplicationKeypad {
    const STR: &'static str = esc!(">");
}

/// Begin synchronized update.
pub struct BeginSynchronizedUpdate;

impl ConstEncode for BeginSynchronizedUpdate {
    const STR: &'static str = csi!("?2026h");
}

/// End synchronized update.
pub struct EndSynchronizedUpdate;

impl ConstEncode for EndSynchronizedUpdate {
    const STR: &'static str = csi!("?2026l");
}
