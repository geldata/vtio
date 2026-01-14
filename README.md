# vtio

**Portable and Efficient Terminal I/O in Rust**

`vtio` is a Rust library for building terminal emulators and terminal multiplexers.

## Overview

`vtio` is specifically designed to allow for low-level access to terminal input and output, which facilitates implementation of terminal multiplexers significantly better than other terminal libraries. This design philosophy makes `vtio` ideal for applications that need fine-grained control over terminal I/O operations.

## Architecture

The workspace consists of two main crates that provide a clean separation of concerns:

- **`vtansi`**: Core ANSI escape sequence framing, traits, and derive macros
- **`vtio`**: High-level terminal input parser and event definitions

### Trie-Based Parsing with Extensible Event Registry

The core parsing infrastructure uses a custom byte trie implementation (`ByteTrie`) optimized specifically for ANSI escape sequence matching:

- O(1) lookup per byte via 128-element direct-indexed array (7-bit encoding)
- Cache-friendly Vec-based node storage (~256 bytes per node)
- Lightweight cursor (16 bytes: trie reference + u16 index)
- Automatic trie construction of all known events via `linkme` distributed slices

The primary motivation for this architecture is **maintainability and extensibility**. Unlike match-based approaches where all events must be defined directly within the crate, the registry-based design allows defining custom events in downstream crates while still benefiting from the generic parsing infrastructure.

### Derive Macros for Declarative Event Definitions

The `vtansi_derive` crate provides procedural macros that eliminate boilerplate for ANSI sequence encoding/decoding:

- `FromAnsi` / `ToAnsi`: Parameter encoding/decoding for primitives, enums, and structs with support for key-value and vector formats
- `AnsiInput` / `AnsiOutput`: Complete control sequence definitions with automatic trait implementation and compile-time validation
- Support for optional fields, default variants, custom delimiters, parameter multiplexing, and transparent newtypes

Events are defined declaratively with attributes specifying their ANSI sequence structure, enabling both parsing and encoding from a single type definition.

## Features

- **Low-level terminal input parsing**: Direct access to terminal input events with minimal abstraction
- **Streaming push parser**: Efficient, zero-copy parsing of terminal input sequences
- **Comprehensive event types**: Support for keyboard, mouse, focus, paste, and resize events
- **Modern keyboard protocol support**: Full support for the [Kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/)
- **Mouse event handling**: Complete mouse event capture including drag, scroll, and movement
- **Bracketed paste mode**: Proper handling of pasted content
- **Event encoding**: Bidirectional conversion via `AnsiEncode` trait
- **Optional serde support**: Serialization and deserialization of events when needed

### Comprehensive VT Event Coverage

The `vtio` crate provides extensive coverage of terminal events:

- **Keyboard**: Full Kitty keyboard protocol, legacy encoding compatibility, key release events
- **Mouse**: X10, SGR, SGR-Pixel, and UTF-8 extended coordinate encoding
- **Cursor**: Movement, positioning, style, visibility, save/restore state
- **Terminal Queries**: Device Status Reports, Device Attributes, mode queries
- **Screen Operations**: Erase, scroll regions, insert/delete, alternate buffer
- **Window Management**: Size, position, state, title queries, focus tracking
- **Character Sets**: G0-G3 designation, single shifts (SS2/SS3)
- **Shell Integration**: OSC 7, OSC 133, iTerm2-specific sequences

## Usage

Add `vtio` to your `Cargo.toml`:

```toml
[dependencies]
vtio = { path = "path/to/vtio/crates/vtio" }
```

### Basic Example

```rust
use vtio::{TerminalInputParser, TerminalInputEvent};

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
use vtio::encode::Encode;
use vtio::event::{
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
5. **Extensibility**: Registry-based architecture allows custom event definitions in downstream crates

## Parser Comparison

This section compares `vtio` with other **terminal input parsers**—libraries that parse keyboard, mouse, and other input events from the terminal.

> **Note**: Libraries like `vte`, `vtparse`, and `ansi-parser` are **output parsers** designed for terminal emulators to interpret escape sequences sent _to_ the terminal (cursor movement, colors, etc.). They are not compared here as they serve a different purpose.

### Benchmark Results (MB/s, higher is better)

| Library         | ASCII | Unicode | CSI Sequences |
| --------------- | ----: | ------: | ------------: |
| **vtio**        |   772 |     634 |           249 |
| **termwiz**     |     2 |       2 |             2 |
| **crossterm**\* |    28 |      28 |            28 |
| **termion**     |    13 |      17 |            25 |

_\* crossterm benchmark includes PTY I/O overhead as it doesn't expose a public parser API_

## Credits

This project builds upon the work of others:

- **[crossterm](https://github.com/crossterm-rs/crossterm)**: Inspiration for the overall design, plus key code definitions in `vtio::event::keyboard`.
- **[Terminal Guide](https://github.com/terminalguide/terminalguide)**: An invaluable reference for documentation of terminal escape sequences.

## License

Licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
