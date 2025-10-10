//! Mode control commands.

use vtansi::csi;
use vtansi::encode::StaticEncode;
use vtansi::esc;

/// Enable bracketed paste mode.
pub struct EnableBracketedPaste;

impl StaticEncode for EnableBracketedPaste {
    const STR: &'static str = csi!("?2004h");
}

/// Disable bracketed paste mode.
pub struct DisableBracketedPaste;

impl StaticEncode for DisableBracketedPaste {
    const STR: &'static str = csi!("?2004l");
}

/// Enable focus reporting.
pub struct EnableFocusReporting;

impl StaticEncode for EnableFocusReporting {
    const STR: &'static str = csi!("?1004h");
}

/// Disable focus reporting.
pub struct DisableFocusReporting;

impl StaticEncode for DisableFocusReporting {
    const STR: &'static str = csi!("?1004l");
}

/// Enable application keypad mode (DECKPAM).
pub struct EnableApplicationKeypad;

impl StaticEncode for EnableApplicationKeypad {
    const STR: &'static str = esc!("=");
}

/// Disable application keypad mode (DECKPNM).
pub struct DisableApplicationKeypad;

impl StaticEncode for DisableApplicationKeypad {
    const STR: &'static str = esc!(">");
}

/// Begin synchronized update.
pub struct BeginSynchronizedUpdate;

impl StaticEncode for BeginSynchronizedUpdate {
    const STR: &'static str = csi!("?2026h");
}

/// End synchronized update.
pub struct EndSynchronizedUpdate;

impl StaticEncode for EndSynchronizedUpdate {
    const STR: &'static str = csi!("?2026l");
}
