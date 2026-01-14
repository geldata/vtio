# vtansi_derive

Procedural macros for deriving `FromAnsi` and `ToAnsi` trait implementations.

## Overview

This crate provides derive macros for the `FromAnsi` and `ToAnsi` traits from the `vtansi` crate, enabling automatic parsing and encoding of ANSI escape sequence parameters with Rust enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
vtansi = { path = "../vtansi", features = ["derive"] }
```

Or use the macro directly:

```toml
[dependencies]
vtansi_derive = { path = "../vtansi_derive" }
vtansi = { path = "../vtansi" }
```

**Note:** Both `FromAnsi` and `ToAnsi` derive macros are re-exported from `vtansi` when the `derive` feature is enabled, so you can use `#[derive(vtansi::FromAnsi, vtansi::ToAnsi)]` instead of importing from `vtansi_derive`.

## Derive Macros

### `FromAnsi`

The `FromAnsi` derive macro can be applied to enums that fall into one of two categories:

#### 1. Enums with Primitive Representations

For enums with a `#[repr(...)]` attribute specifying a primitive integer type, the macro generates an implementation that:
1. Parses the bytes as the primitive type using the existing `TryFromAnsi` implementation
2. Converts the parsed value to the enum using `TryFrom<PrimitiveType>`

##### Default Variant

You can mark one variant with `#[vtansi(default)]` to handle unrecognized values. The default variant can be either:
1. A unit variant (returns a constant value)
2. A tuple variant with one field (captures the unrecognized value)

**Unit Default Variant:**

When parsing encounters a value that doesn't match any variant, it will return the default variant instead of erroring:

```rust
use vtansi::FromAnsi;
use vtansi::parse::TryFromAnsi;

#[derive(FromAnsi)]
#[repr(u8)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
    #[vtansi(default)]
    Unknown = 255,
}

impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::Red),
            1 => Ok(Color::Green),
            2 => Ok(Color::Blue),
            255 => Ok(Color::Unknown),
            _ => Err(()),
        }
    }
}

// Valid values parse normally
assert_eq!(Color::try_from_ansi(b"0").unwrap(), Color::Red);

// Invalid values return the default
assert_eq!(Color::try_from_ansi(b"99").unwrap(), Color::Unknown);
```

**Capturing Default Variant:**

For tuple variants with one field, the unrecognized value is captured and stored in the variant. The field type must implement `From<ReprType>` for repr enums:

```rust
use vtansi::FromAnsi;
use vtansi::parse::TryFromAnsi;

#[derive(FromAnsi)]
#[repr(u8)]
enum StatusCode {
    Ok = 200,
    NotFound = 404,
    #[vtansi(default)]
    Unknown(u8),  // Captures unrecognized status codes
}

impl TryFrom<u8> for StatusCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            200 => Ok(StatusCode::Ok),
            404 => Ok(StatusCode::NotFound),
            _ => Err(()),
        }
    }
}

// Valid values parse normally
assert_eq!(StatusCode::try_from_ansi(b"200").unwrap(), StatusCode::Ok);

// Invalid values are captured
assert_eq!(StatusCode::try_from_ansi(b"500").unwrap(), StatusCode::Unknown(500));
```

```rust
use vtansi::FromAnsi;  // Re-exported from vtansi_derive
use vtansi::parse::TryFromAnsi;

#[derive(FromAnsi)]
#[repr(u8)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

// You must implement TryFrom for the repr type
impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::Red),
            1 => Ok(Color::Green),
            2 => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

// Now you can parse from ANSI bytes
assert_eq!(Color::try_from_ansi(b"0").unwrap(), Color::Red);
assert_eq!(Color::try_from_ansi(b"1").unwrap(), Color::Green);
```

**Capturing Default Variant:**

For tuple variants with one field, the unrecognized string is captured. The field type must implement `From<&str>` (e.g., `String`):

```rust
use vtansi::FromAnsi;
use vtansi::parse::TryFromAnsi;

#[derive(FromAnsi)]
enum Command {
    Quit,
    Save,
    Load,
    #[vtansi(default)]
    Custom(String),  // Captures unrecognized commands
}

impl TryFrom<&str> for Command {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "quit" => Ok(Command::Quit),
            "save" => Ok(Command::Save),
            "load" => Ok(Command::Load),
            _ => Err(format!("unknown command: {}", s)),
        }
    }
}

// Valid values parse normally
assert_eq!(Command::try_from_ansi(b"quit").unwrap(), Command::Quit);

// Invalid values are captured
assert_eq!(
    Command::try_from_ansi(b"help").unwrap(),
    Command::Custom("help".to_string())
);
```

Supported primitive types: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `usize`, `isize`

#### 2. Enums Implementing `TryFrom<&str>`

For enums without a primitive representation, the macro generates an implementation that:
1. Parses the bytes as a UTF-8 string
2. Converts the string to the enum using `TryFrom<&str>`

##### Default Variant

Similarly, string-based enums can use `#[vtansi(default)]` to handle unrecognized strings.

**Unit Default Variant:**

```rust
use vtansi::FromAnsi;
use vtansi::parse::TryFromAnsi;

#[derive(FromAnsi)]
enum TextStyle {
    Plain,
    Bold,
    Italic,
    #[vtansi(default)]
    Unknown,
}

impl TryFrom<&str> for TextStyle {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "plain" => Ok(TextStyle::Plain),
            "bold" => Ok(TextStyle::Bold),
            "italic" => Ok(TextStyle::Italic),
            _ => Err(format!("unknown text style: {}", s)),
        }
    }
}

// Valid values parse normally
assert_eq!(TextStyle::try_from_ansi(b"plain").unwrap(), TextStyle::Plain);

// Invalid values return the default
assert_eq!(TextStyle::try_from_ansi(b"underline").unwrap(), TextStyle::Unknown);
```

```rust
use vtansi::FromAnsi;  // Re-exported from vtansi_derive
use vtansi::parse::TryFromAnsi;

#[derive(FromAnsi)]
enum TextStyle {
    Plain,
    Bold,
    Italic,
}

// You must implement TryFrom<&str>
impl TryFrom<&str> for TextStyle {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "plain" => Ok(TextStyle::Plain),
            "bold" => Ok(TextStyle::Bold),
            "italic" => Ok(TextStyle::Italic),
            _ => Err(format!("unknown text style: {}", s)),
        }
    }
}

// Now you can parse from ANSI bytes
assert_eq!(TextStyle::try_from_ansi(b"plain").unwrap(), TextStyle::Plain);
assert_eq!(TextStyle::try_from_ansi(b"bold").unwrap(), TextStyle::Bold);
```

### `ToAnsi`

The `ToAnsi` derive macro can be applied to enums that fall into one of two categories:

#### 1. Enums with Primitive Representations

For enums with a `#[repr(...)]` attribute specifying a primitive integer type, the macro generates an implementation that converts the enum to its primitive representation for encoding.

```rust
use vtansi::FromAnsi;  // Re-exported from vtansi_derive
use vtansi::encode::AnsiEncode;

#[derive(ToAnsi)]
#[repr(u8)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

// Now you can encode to ANSI bytes
assert_eq!(Color::Red.encode_ansi().unwrap(), b"0");
assert_eq!(Color::Green.encode_ansi().unwrap(), b"1");
```

Supported primitive types: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `usize`, `isize`

#### 2. Enums Implementing `AsRef<str>`

For enums without a primitive representation, the macro generates an implementation that uses `AsRef<str>` to convert the enum to a string for encoding.

```rust
use vtansi::FromAnsi;  // Re-exported from vtansi_derive
use vtansi::encode::AnsiEncode;

#[derive(ToAnsi)]
enum TextStyle {
    Plain,
    Bold,
    Italic,
}

// You must implement AsRef<str>
impl AsRef<str> for TextStyle {
    fn as_ref(&self) -> &str {
        match self {
            TextStyle::Plain => "plain",
            TextStyle::Bold => "bold",
            TextStyle::Italic => "italic",
        }
    }
}

// Now you can encode to ANSI bytes
assert_eq!(TextStyle::Plain.encode_ansi().unwrap(), b"plain");
assert_eq!(TextStyle::Bold.encode_ansi().unwrap(), b"bold");
```

### Roundtrip Conversion

Both derives can be used together to enable bidirectional conversion:

```rust
use vtansi::{FromAnsi, ToAnsi};
use vtansi::parse::TryFromAnsi;
use vtansi::encode::AnsiEncode;

#[derive(Debug, PartialEq, FromAnsi, ToAnsi)]
#[repr(u8)]
enum Color {
    Red = 31,
    Green = 32,
    Blue = 33,
}

impl TryFrom<u8> for Color {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            31 => Ok(Color::Red),
            32 => Ok(Color::Green),
            33 => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

// Roundtrip: encode then decode
let color = Color::Green;
let encoded = color.encode_ansi().unwrap();
let decoded = Color::try_from_ansi(&encoded).unwrap();
assert_eq!(color, decoded);

// Roundtrip: decode then encode
let bytes = b"32";
let decoded = Color::try_from_ansi(bytes).unwrap();
let encoded = decoded.encode_ansi().unwrap();
assert_eq!(bytes, encoded.as_slice());
```

## Transparent Newtype Wrappers

The `#[vtansi(transparent)]` attribute allows creating newtype wrappers that delegate encoding/decoding to their single field.

### Basic Transparent Types

For simple newtype wrappers around types that already implement the traits:

```rust
use vtansi::{FromAnsi, ToAnsi};
use vtansi::encode::AnsiEncode;
use vtansi::parse::TryFromAnsi;

#[derive(FromAnsi, ToAnsi)]
#[vtansi(transparent)]
struct UserId(u32);

// Encodes/decodes exactly like u32
let id = UserId(42);
assert_eq!(id.encode_ansi().unwrap(), b"42");
assert_eq!(UserId::try_from_ansi(b"42").unwrap().0, 42);
```

### Transparent Vec Support

When wrapping `Vec<T>`, you **must** specify a delimiter using `#[vtansi(delimiter = <bytelit>)]`. Elements are encoded/decoded with the delimiter between them.

```rust
use vtansi::{FromAnsi, ToAnsi};
use vtansi::encode::AnsiEncode;
use vtansi::parse::TryFromAnsi;

#[derive(FromAnsi, ToAnsi)]
#[vtansi(transparent, delimiter = b';')]
struct Numbers(Vec<u32>);

// Encoding
let nums = Numbers(vec![1, 2, 3, 4, 5]);
assert_eq!(nums.encode_ansi().unwrap(), b"1;2;3;4;5");

// Decoding
let nums = Numbers::try_from_ansi(b"10;20;30").unwrap();
assert_eq!(nums.0, vec![10, 20, 30]);

// Empty vectors
let empty = Numbers(vec![]);
assert_eq!(empty.encode_ansi().unwrap(), b"");
assert_eq!(Numbers::try_from_ansi(b"").unwrap().0, Vec::<u32>::new());
```

You can use any delimiter:

```rust
#[derive(FromAnsi, ToAnsi)]
#[vtansi(transparent, delimiter = b',')]
struct CommaList(Vec<u16>);

#[derive(FromAnsi, ToAnsi)]
#[vtansi(transparent, delimiter = b':')]
struct ColonList(Vec<String>);
```

Works with named fields too:

```rust
#[derive(FromAnsi, ToAnsi)]
#[vtansi(transparent, delimiter = b';')]
struct Coordinates {
    points: Vec<u8>,
}
```

**Important:** The delimiter attribute is **required** when using `transparent` with `Vec<T>`. Omitting it will result in a compile-time error:

```rust
// This will NOT compile:
// #[derive(FromAnsi, ToAnsi)]
// #[vtansi(transparent)]  // ERROR: missing delimiter
// struct Numbers(Vec<u32>);
```

The element type `T` must implement:
- `TryFromAnsi` for `FromAnsi` derivation
- `AnsiEncode` for `ToAnsi` derivation

## Error Handling

### FromAnsi

The generated implementations return `Result<Self, ParseError>` where `ParseError` is from the `vtansi::parse` module. Possible error variants include:

- `ParseError::InvalidNum`: The bytes could not be parsed as a number (for repr enums)
- `ParseError::InvalidString`: The bytes are not valid UTF-8 (for string-based enums)
- `ParseError::InvalidValue`: The parsed value does not correspond to any enum variant (only when no default variant is specified)

**Note:**
- If a variant is marked with `#[vtansi(default)]`, parsing will never return `ParseError::InvalidValue` - unrecognized values will be converted to the default variant instead.
- Parse errors like `InvalidNum` and `InvalidString` will still be returned when the input cannot be parsed at all (e.g., non-numeric input for repr enums, invalid UTF-8 for string enums).
- For capturing default variants (tuple variants), the captured value is constructed using `.into()`, so the field type must implement the appropriate `From` trait.

### ToAnsi

The `ToAnsi` implementations are generally infallible for well-formed enums, but encoding can fail with:

- `EncodeError::BufferOverflow`: The buffer is too small to hold the encoded value (when encoding to a fixed-size buffer)
- `EncodeError::IOError`: An I/O error occurred during encoding

## Requirements

### FromAnsi

- For enums with primitive representations: must implement `TryFrom<ReprType>` where `ReprType` is the type specified in the `#[repr(...)]` attribute
- For enums without primitive representations: must implement `TryFrom<&str>`
- The enum must not have any variants with fields (unit variants only)
- At most one variant can be marked with `#[vtansi(default)]`
- Default variants must be either unit variants or tuple variants with exactly one field
- For capturing defaults (tuple variants):
  - Repr enums: the field type must implement `From<ReprType>`
  - String enums: the field type must implement `From<&str>`

### ToAnsi

- For enums with primitive representations: the `#[repr(...)]` attribute must specify a supported integer type
- For enums without primitive representations: must implement `AsRef<str>`
- The enum must not have any variants with fields (unit variants only)

## License

MIT
