# vtansi_derive

Procedural macros for deriving `FromAnsi` and `ToAnsi` trait implementations.

## Overview

This crate provides derive macros for the `FromAnsi` and `ToAnsi` traits from the `vtenc` crate, enabling automatic parsing and encoding of ANSI escape sequence parameters with Rust enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
vtenc = { path = "../vtenc", features = ["derive"] }
```

Or use the macro directly:

```toml
[dependencies]
vtansi_derive = { path = "../vtansi_derive" }
vtenc = { path = "../vtenc" }
```

**Note:** Both `FromAnsi` and `ToAnsi` derive macros are re-exported from `vtenc` when the `derive` feature is enabled, so you can use `#[derive(vtenc::FromAnsi, vtenc::ToAnsi)]` instead of importing from `vtansi_derive`.

## Derive Macros

### `FromAnsi`

The `FromAnsi` derive macro can be applied to enums that fall into one of two categories:

#### 1. Enums with Primitive Representations

For enums with a `#[repr(...)]` attribute specifying a primitive integer type, the macro generates an implementation that:
1. Parses the bytes as the primitive type using the existing `TryFromAnsi` implementation
2. Converts the parsed value to the enum using `TryFrom<PrimitiveType>`

```rust
use vtenc::FromAnsi;  // Re-exported from vtansi_derive
use vtenc::parse::TryFromAnsi;

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

Supported primitive types: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `usize`, `isize`

#### 2. Enums Implementing `TryFrom<&str>`

For enums without a primitive representation, the macro generates an implementation that:
1. Parses the bytes as a UTF-8 string
2. Converts the string to the enum using `TryFrom<&str>`

```rust
use vtenc::FromAnsi;  // Re-exported from vtansi_derive
use vtenc::parse::TryFromAnsi;

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
use vtenc::FromAnsi;  // Re-exported from vtansi_derive
use vtenc::encode::AnsiEncode;

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
use vtenc::FromAnsi;  // Re-exported from vtansi_derive
use vtenc::encode::AnsiEncode;

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
use vtenc::{FromAnsi, ToAnsi};
use vtenc::parse::TryFromAnsi;
use vtenc::encode::AnsiEncode;

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

## Error Handling

### FromAnsi

The generated implementations return `Result<Self, ParseError>` where `ParseError` is from the `vtenc::parse` module. Possible error variants include:

- `ParseError::InvalidNum`: The bytes could not be parsed as a number (for repr enums)
- `ParseError::InvalidString`: The bytes are not valid UTF-8 (for string-based enums)
- `ParseError::InvalidValue`: The parsed value does not correspond to any enum variant

### ToAnsi

The `ToAnsi` implementations are generally infallible for well-formed enums, but encoding can fail with:

- `EncodeError::BufferOverflow`: The buffer is too small to hold the encoded value (when encoding to a fixed-size buffer)
- `EncodeError::IOError`: An I/O error occurred during encoding

## Requirements

### FromAnsi

- For enums with primitive representations: must implement `TryFrom<ReprType>` where `ReprType` is the type specified in the `#[repr(...)]` attribute
- For enums without primitive representations: must implement `TryFrom<&str>`
- The enum must not have any variants with fields (unit variants only)

### ToAnsi

- For enums with primitive representations: the `#[repr(...)]` attribute must specify a supported integer type
- For enums without primitive representations: must implement `AsRef<str>`
- The enum must not have any variants with fields (unit variants only)

## License

MIT