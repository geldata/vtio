# vtio

**Terminal Input Parser and Event Handler**

`vtio` is a Rust library for parsing terminal input sequences into structured events. It provides a streaming push parser that efficiently converts raw terminal input bytes into high-level events such as key presses, mouse actions, and terminal mode responses.

## Features

- **Streaming push parser**: Efficient, callback-driven parsing of terminal input
- **Comprehensive keyboard support**: Full support for the [Kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/) including modifier keys, key release events, and enhancement flags
- **Mouse event handling**: Complete mouse event capture including button press/release, drag, scroll, and motion tracking
- **Terminal mode queries**: Parse responses to DECRQM (Request Mode) and other terminal capability queries
- **Bracketed paste mode**: Proper handling of pasted content with escape sequence preservation
- **Event encoding**: Encode events back into ANSI sequences via the `vtansi::AnsiEncode` trait
- **Optional serde support**: Enable the `serde` feature for serialization/deserialization of events

## Installation

Add `vtio` to your `Cargo.toml`:

```toml
[dependencies]
vtio = "0.1"
```

With optional serde support:

```toml
[dependencies]
vtio = { version = "0.1", features = ["serde"] }
```

## Usage

### Basic Parsing

The parser emits events as `&dyn vtansi::AnsiEvent` trait objects. Use `downcast_ref` from the `AnyEvent` trait to match on concrete event types:

```rust
use vtio::{TerminalInputParser, AnyEvent};
use vtio::event::{KeyEvent, MouseEvent};

let mut parser = TerminalInputParser::new();

// Parse an up arrow key sequence
let input = b"\x1b[A";
parser.feed_with(input, &mut |event| {
    if let Some(key_event) = event.downcast_ref::<KeyEvent>() {
        println!("Key pressed: {:?}", key_event.code);
    } else if let Some(mouse_event) = event.downcast_ref::<MouseEvent>() {
        println!("Mouse event: {:?}", mouse_event);
    }
});
```

### One-shot Buffer Decoding

```rust
use vtio::{TerminalInputParser, AnyEvent};
use vtio::event::KeyEvent;

let input = b"\x1b[A\x1b[B"; // Up arrow, Down arrow

TerminalInputParser::decode_buffer(input, &mut |event| {
    if let Some(key_event) = event.downcast_ref::<KeyEvent>() {
        println!("Key: {:?}", key_event.code);
    }
});
```

### Handling Idle State

For sequences like the Escape key that could be the start of a longer sequence, call `idle()` after a timeout to flush pending events:

```rust
use vtio::{TerminalInputParser, AnyEvent};
use vtio::event::KeyEvent;

let mut parser = TerminalInputParser::new();

// User presses Escape key
parser.feed_with(b"\x1b", &mut |event| {
    // No event emitted yet - could be start of escape sequence
});

// After a timeout with no more input, flush the escape as a key event
parser.idle(&mut |event| {
    if let Some(key_event) = event.downcast_ref::<KeyEvent>() {
        println!("Escape key pressed");
    }
});
```

## Event Types

The parser emits events that implement `vtansi::AnsiEvent`. Use `downcast_ref::<T>()` to check for specific event types. Common event types include:

- **Keyboard events**: `KeyEvent`, `KeyboardEnhancementFlagsResponse`, `PushKeyboardEnhancementFlags`, `PopKeyboardEnhancementFlags`
- **Mouse events**: `MouseEvent` with various tracking modes
- **Terminal responses**: Mode query responses, cursor position reports, device attributes
- **Paste events**: `BracketedPasteStart`, `BracketedPasteEnd`, `BracketedPasteData`

## Examples

Run the interactive terminal input debugger:

```bash
cargo run --example vtev -p vtio
```

This launches a full-screen TUI that displays:

- Decoded key and mouse events
- Raw input bytes
- Terminal mode states
- Event history log

## Architecture

`vtio` is built on top of:

- [`vt-push-parser`](https://github.com/anthropics/vt-push-parser): Low-level VT sequence tokenizer
- [`vtansi`](../vtansi): ANSI event traits and encoding utilities

The `TerminalInputParser` wraps the low-level parser and transforms raw VT events into semantic terminal input events.

## License

Licensed under the MIT license. See [LICENSE](../../LICENSE) for details.
