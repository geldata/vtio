# vtansi

Macros for framing VT ANSI escape sequences.

This crate provides utilities for constructing ANSI escape sequences at compile time.

## Macros

- `csi!`: Concatenate string literals while prepending a Control Sequence Introducer (`"\x1b["`)
- `osc!`: Concatenate string literals while prepending an Operating System Command introducer (`"\x1b]"`) and appending a string terminator (`"\x1b\\"`)

## Usage

```rust
use vtansi::{csi, osc};

// Create a CSI sequence to clear the screen
let clear_screen = csi!("2J");

// Create an OSC sequence to set the window title
let set_title = osc!("0;My Window Title");
```
