#![warn(clippy::pedantic)]

mod c0;
mod char;
mod csi;
pub mod encode;
mod esc;
pub mod event;
mod macros;
pub mod parser;
mod ss;
pub use encode::Encode;
pub use event::{
    KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, KeyboardEnhancementFlags,
    MouseButton, MouseEvent, MouseEventKind, TerminalInputEvent, TerminalInputEventOwned,
    TerseDisplay,
};
pub use parser::TerminalInputParser;
pub use vt_push_parser::event::{VTEvent, VTOwnedEvent};
