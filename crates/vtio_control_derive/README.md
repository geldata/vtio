# vtderive

Procedural macros for deriving VT escape sequence traits and automatically
registering them with the global escape sequence registry.

## Overview

This crate provides attribute macros for each type of escape sequence
introducer. Each macro automatically:

1. Implements the `EscapeSequence` trait for your struct
2. Registers the sequence in the global `ESCAPE_SEQUENCE_REGISTRY` using
   `linkme::distributed_slice`

## Available Macros

### `#[csi]` - Control Sequence Introducer

The most common escape sequence type, used for cursor movement, text
formatting, and more.

#### Const Sequences

For sequences with fixed parameters, use a unit struct:

```rust
use vtio_control_derive::csi;

#[csi(private = '?', params = ["6"], finalbyte = 'h')]
struct DecSetMode;

#[csi(params = ["1"], finalbyte = 'm')]
struct ResetAttributes;

#[csi(finalbyte = 'H')]
struct ClearHome;
```

These automatically implement `StaticAnsiEncode` for zero-cost encoding.

#### Variable Sequences

For sequences with runtime parameters, define a struct with fields:

```rust
use vtio_control_derive::csi;

#[csi(finalbyte = 'H')]
struct CursorPosition {
    pub row: u16,
    pub col: u16,
}

#[csi(finalbyte = 'A')]
struct CursorUp {
    pub n: u16,
}

// Use them like this:
let pos = CursorPosition { row: 10, col: 20 };
let up = CursorUp { n: 5 };
```

These implement `StaticEncodedLen` and `Encode` for efficient encoding with
parameters.

#### With Custom Handler

```rust
#[csi(finalbyte = 'J', handler = my_custom_handler)]
struct ClearScreen;

fn my_custom_handler(params: &vtparser::EscapeSequenceParams) {
    // Custom handling logic
}
```

### `#[osc]` - Operating System Command

Used for terminal window titles, notifications, and other OS-level commands.

#### Basic OSC Sequences

```rust
use vtio_control_derive::osc;

#[osc(number = "0", data = "Set Title")]
struct SetWindowTitle;
```

#### OSC with Positional Parameters

OSC (and DCS) sequences support positional parameters using the
`#[vtctl(positional)]` attribute. Positional parameters are encoded as
semicolon-separated values in the data section.

```rust
use vtio_control_derive::VTControl;

// Shell integration command end with optional exit code
#[derive(Debug, Clone, Copy, VTControl)]
#[osc(number = "133", data = "D")]
struct CommandEnd {
    #[vtctl(positional)]
    pub exit_code: Option<i32>,
}

// Usage:
let cmd_no_exit = CommandEnd { exit_code: None };
// Encodes to: OSC 133;D ST

let cmd_success = CommandEnd { exit_code: Some(0) };
// Encodes to: OSC 133;D;0 ST

let cmd_failure = CommandEnd { exit_code: Some(1) };
// Encodes to: OSC 133;D;1 ST
```

**Rules for positional parameters:**
- Mark fields with `#[vtctl(positional)]` to include them as positional
  parameters
- Optional positionals (using `Option<T>`) must come after all required
  positionals
- When encoding, `None` values are omitted along with any subsequent
  parameters
- Positional parameters are separated by semicolons
- Supported for both OSC and DCS sequences

### `#[dcs]` - Device Control String

Used for advanced device control interactions.

```rust
use vtio_control_derive::dcs;

#[dcs(finalbyte = 'q')]
struct RequestStatus;
```

### `#[ss2]` - Single Shift 2

Used for character set switching in internationalization scenarios.

```rust
use vtio_control_derive::ss2;

#[ss2(finalbyte = 'G')]
struct SingleShift2;
```

### `#[ss3]` - Single Shift 3

Used for character set switching in internationalization scenarios.

```rust
use vtio_control_derive::ss3;

#[ss3(finalbyte = 'H')]
struct SingleShift3;
```

### `#[pm]` - Privacy Message

Used for privacy-related communication between applications and terminals.

```rust
use vtio_control_derive::pm;

#[pm(finalbyte = 'p')]
struct PrivacyMessage;
```

### `#[apc]` - Application Program Command

Used for passing data between applications and the terminal emulator.

```rust
use vtio_control_derive::apc;

#[apc(finalbyte = 'a')]
struct ApplicationCommand;
```

### `#[st]` - String Terminator

Indicates the end of an escape sequence.

```rust
use vtio_control_derive::st;

#[st(finalbyte = '\\')]
struct StringTerminator;
```

### `#[deckpam]` - DEC Keypad Application Mode

Enables application keypad mode in DEC VT220 terminals.

```rust
use vtio_control_derive::deckpam;

#[deckpam(finalbyte = '=')]
struct KeypadApplicationMode;
```

### `#[deckpnm]` - DEC Keypad Numeric Mode

Disables application keypad mode in DEC VT220 terminals.

```rust
use vtio_control_derive::deckpnm;

#[deckpnm(finalbyte = '>')]
struct KeypadNumericMode;
```

## Attributes

All macros support the following attributes:

- `private` (optional): A character representing the private marker byte
  (e.g., `'?'`, `'>'`, `'='`). If omitted, no private marker is used.

- `params` (optional): An array of string literals representing fixed
  (const) sequence parameters. Each string is converted to a sequence of
  bytes. Use this only for unit structs (const sequences). Cannot be used
  with structs that have fields.

- `intermediate` (optional): A string literal (max 2 characters)
  representing intermediate bytes. If omitted, no intermediate bytes are
  used.

- `finalbyte` (required): A character representing the final byte of the
  sequence (e.g., `'h'`, `'m'`, `'H'`).

- `handler` (optional): A function identifier for a custom handler. If
  omitted, a default handler named `{struct_name}_handler` (in lowercase)
  is assumed.

### OSC/DCS Specific Attributes

For OSC and DCS sequences:

- `number` (optional): The OSC command number (e.g., `"133"` for shell
  integration)

- `data` (optional): Static data prefix for the OSC/DCS command

### Field Attributes

For fields within variable sequence structs:

- `#[vtctl(positional)]`: Marks a field as a positional parameter for
  OSC/DCS sequences. Positional parameters are encoded as semicolon-
  separated values. Optional positional parameters (using `Option<T>`)
  must come after all required positional parameters

## Struct Definition

For **const sequences**, use a unit struct:
```rust
#[csi(params = ["1"], finalbyte = 'm')]
struct ResetAttributes;
```

For **variable sequences**, define a struct with fields. The fields will be
used as parameters in the encoded sequence in the order they are declared.
Supported field types include `u8`, `u16`, `u32`, `u64`, `usize`, `i8`,
`i16`, `i32`, `i64`, `isize`, `bool`, and `char`:
```rust
#[csi(finalbyte = 'H')]
struct CursorPosition {
    pub row: u16,
    pub col: u16,
}
```

## Generated Implementation

### For Const Sequences (unit structs)

1. An `EscapeSequence` trait implementation with associated constants
2. A `StaticAnsiEncode` implementation with the static sequence string
3. Automatic `StaticEncodedLen` and `Encode` implementations via blanket
   impls
4. A registry entry in `vtparser::ESCAPE_SEQUENCE_REGISTRY`

All constants are generated in a `const` context for zero-cost abstraction.

### For Variable Sequences (structs with fields)

1. An `EscapeSequence` trait implementation with associated constants
2. A `StaticEncodedLen` implementation with the maximum encoded length
3. An efficient `Encode` implementation that writes field values as
   parameters
4. A registry entry in `vtparser::ESCAPE_SEQUENCE_REGISTRY`

The implementation:
- Uses struct fields as parameters in declaration order
- Encodes efficiently without heap allocation
- Provides compile-time length bounds for buffer allocation

## Positional Parameter Validation

The derive macro validates positional parameter ordering at compile time:

```rust
// Valid: required before optional
#[derive(VTControl)]
#[osc(number = "999", data = "TEST")]
struct ValidOrder {
    #[vtctl(positional)]
    pub required: i32,
    #[vtctl(positional)]
    pub optional: Option<i32>,
}

// Invalid: required after optional - compile error!
#[derive(VTControl)]
#[osc(number = "999", data = "TEST")]
struct InvalidOrder {
    #[vtctl(positional)]
    pub optional: Option<i32>,
    #[vtctl(positional)]
    pub required: i32,  // Error: required positional must come before optional
}
```

## Error Reporting

The macros provide detailed, helpful error messages using
`proc-macro2-diagnostics`. When attributes are used incorrectly, you'll
receive clear diagnostics with helpful hints:

```rust
// Missing required 'finalbyte' attribute
#[csi(private = '?', params = ["6"])]
struct MissingFinal;
// error: finalbyte attribute is required
// help: add finalbyte = 'X' where X is the final byte character

// Invalid private value (not a char literal)
#[csi(private = "?", finalbyte = 'h')]
struct InvalidPrivate;
// error: private must be a char literal
// help: example: private = '?'

// Intermediate value too long
#[csi(intermediate = "abc", finalbyte = 'h')]
struct IntermediateTooLong;
// error: intermediate must have at most 2 characters
// help: intermediate bytes are limited to 2 characters

// Unknown attribute
#[csi(foo = "bar", finalbyte = 'h')]
struct UnknownAttribute;
// error: unknown attribute: foo
// help: valid attributes are: private, params, intermediate, finalbyte, handler
```

## Example: Complete Usage

```rust
use vtio_control_derive::csi;
use vtparser::EscapeSequenceParams;
use vtenc::encode::Encode;

// Const sequence - unit struct with fixed params
#[csi(private = '?', params = ["1049"], finalbyte = 'h')]
struct EnableAlternateScreen;

// Variable sequence - struct with fields as params
#[csi(finalbyte = 'H')]
struct CursorPosition {
    pub row: u16,
    pub col: u16,
}

// Define handlers
fn enablealternatescreen_handler(params: &EscapeSequenceParams) {
    println!("Enabling alternate screen buffer");
}

fn cursorposition_handler(params: &EscapeSequenceParams) {
    println!("Moving cursor");
}

// Usage:
let mut buf = [0u8; 64];

// Const sequence - zero-cost
let written = EnableAlternateScreen.encode(&mut &mut buf[..]).unwrap();
assert_eq!(&buf[..written], b"\x1B[?1049h");

// Variable sequence - efficient with parameters
let mut pos = CursorPosition { row: 10, col: 20 };
let written = pos.encode(&mut &mut buf[..]).unwrap();
assert_eq!(&buf[..written], b"\x1B[10;20H");

// Both are automatically registered and will be matched by the parser
```
