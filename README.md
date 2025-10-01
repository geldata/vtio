# vtio

**Portable and Efficient Terminal I/O in Rust**

`vtio` is a Rust library for building terminal emulators and terminal multiplexers.

## Overview

While `vtio` borrows heavily from [`crossterm`](https://github.com/crossterm-rs/crossterm), it is specifically designed to allow for low-level access to terminal input, which facilitates implementation of terminal multiplexers significantly better than crossterm. This design philosophy makes `vtio` ideal for applications that need fine-grained control over terminal I/O operations.

Under the hood, `vtio` uses [`vt-push-parser`](https://github.com/elprans/vt-push-parser) as the underlying low-level terminal protocol parser, providing robust and efficient parsing of VT sequences.

## Features

- **Low-level terminal input parsing**: Direct access to terminal input events with minimal abstraction
- **Streaming push parser**: Efficient, zero-copy parsing of terminal input sequences
- **Comprehensive event types**: Support for keyboard, mouse, focus, paste, and resize events
- **Modern keyboard protocol support**: Full support for the [Kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/)
- **Mouse event handling**: Complete mouse event capture including drag, scroll, and movement
- **Bracketed paste mode**: Proper handling of pasted content
- **Event encoding**: Efficient encoding of terminal control sequences into buffers
- **Optional serde support**: Serialization and deserialization of events when needed

## Project Structure

This is a Cargo workspace containing:

- **`vtinput`**: The main terminal input parsing and event handling library

## Usage

Add `vtinput` to your `Cargo.toml`:

```toml
[dependencies]
vtinput = { path = "path/to/vtio/crates/vtinput" }
```

### Basic Example

```rust
use vtinput::{TerminalInputParser, TerminalInputEvent};

let mut parser = TerminalInputParser::new();
let input = b"\x1b[A"; // Up arrow key

parser.feed_with(input, |event| {
    match event {
        TerminalInputEvent::Key(key_event) => {
            println!("Key pressed: {:?}", key_event);
        }
        TerminalInputEvent::Mouse(mouse_event) => {
            println!("Mouse event: {:?}", mouse_event);
        }
        _ => {}
    }
});
```

### Enabling Terminal Features

```rust
use vtinput::encode::Encode;
use vtinput::event::{
    EnableMouseCapture,
    EnableBracketedPaste,
    PushKeyboardEnhancementFlags,
    KeyboardEnhancementFlags,
};

let mut buf = [0u8; 64];

// Enable mouse capture
let mut cmd = EnableMouseCapture;
let len = cmd.encode(&mut buf).unwrap();
// Write buf[..len] to terminal

// Enable bracketed paste
let mut cmd = EnableBracketedPaste;
let len = cmd.encode(&mut buf).unwrap();
// Write buf[..len] to terminal

// Enable keyboard enhancement flags
let mut cmd = PushKeyboardEnhancementFlags(
    KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
);
let len = cmd.encode(&mut buf).unwrap();
// Write buf[..len] to terminal
```

## Design Philosophy

Unlike higher-level terminal libraries, `vtio` prioritizes:

1. **Low-level control**: Direct access to parsed terminal events without unnecessary abstraction
2. **Performance**: Zero-copy parsing where possible, efficient buffer handling
3. **Flexibility**: Designed for building terminal multiplexers and emulators, not just TUIs
4. **Correctness**: Comprehensive support for modern terminal protocols and edge cases

## License

Licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
