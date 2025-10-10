#![warn(clippy::pedantic)]

pub use vtansi::encode::{self, Encode};
pub use vtansi::{csi, osc};

mod c0;
mod char;
mod csi;
mod esc;
pub mod event;
pub mod parser;
mod ss;
pub use event::{
    KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, KeyboardEnhancementFlags,
    MouseButton, MouseEvent, MouseEventKind, TerminalInputEvent, TerminalInputEventOwned,
    TerseDisplay,
};
pub use parser::TerminalInputParser;
pub use vt_push_parser::event::{VTEvent, VTOwnedEvent};
