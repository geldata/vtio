//! Derive macros for `FromAnsi` and `ToAnsi` traits.
//!
//! This crate provides derive macros for the `FromAnsi` and `ToAnsi` traits
//! from the `vtenc` crate. These can be used on enums and structs for
//! parsing and encoding ANSI escape sequence parameters.
//!
//! # Features
//!
//! - **Automatic trait derivation** - simply add `#[derive(FromAnsi)]` or
//!   `#[derive(ToAnsi)]` to your enum or struct
//! - **Multiple parsing strategies** - supports integer-based (via
//!   `#[repr(...)]`), string-based, and structured field parsing
//! - **Default variants** - optionally specify a fallback variant for
//!   unrecognized enum values
//! - **Flexible struct formats** - encode/decode structs as `key=value` pairs
//!   or as positional values
//! - **Customizable delimiters** - configure field separators for structs
//! - **Zero runtime overhead** - all code is generated at compile time with
//!   `#[inline]` optimizations
//!
//! # Enum Support
//!
//! For enums, the macros support both primitive representations and
//! string-based conversions:
//!
//! ```ignore
//! #[derive(FromAnsi, ToAnsi)]
//! #[repr(u8)]
//! enum Color {
//!     Red = 0,
//!     Green = 1,
//!     Blue = 2,
//! }
//! ```
//!
//! # Struct Support
//!
//! For structs with named or unnamed (tuple) fields, the macros support two
//! formats:
//!
//! ## Key-Value Format (default)
//!
//! ```ignore
//! #[derive(FromAnsi, ToAnsi)]
//! struct Settings {
//!     width: u32,
//!     height: u32,
//! }
//! // Encodes as: "width=800;height=600"
//! ```
//!
//! Note: Key-value format is only available for named fields, not tuple
//! structs.
//!
//! ## Value Format
//!
//! ```ignore
//! #[derive(FromAnsi, ToAnsi)]
//! #[vtansi(format = "value")]
//! struct Point {
//!     x: i32,
//!     y: i32,
//! }
//! // Encodes as: "100;200"
//! ```
//!
//! ## Tuple Structs (automatic value format)
//!
//! ```ignore
//! #[derive(FromAnsi, ToAnsi)]
//! struct Point(i32, i32);
//! // Automatically uses value format
//! // Encodes as: "100;200"
//! ```
//!
//! Tuple structs automatically default to `value` format and cannot use
//! `key=value` format.
//!
//! ## Custom Delimiter
//!
//! ```ignore
//! #[derive(FromAnsi, ToAnsi)]
//! #[vtansi(delimiter = ",")]
//! struct Config {
//!     name: String,
//!     value: u32,
//! }
//! // Encodes as: "name=foo,value=42"
//! ```
//!
//! ## Optional Fields (key=value format only)
//!
//! ```ignore
//! #[derive(FromAnsi, ToAnsi)]
//! struct Settings {
//!     width: u32,
//!     height: u32,
//!     title: Option<String>,  // Optional field
//! }
//! // Parses: "width=800;height=600" (title is None)
//! // Parses: "width=800;height=600;title=MyApp" (title is Some("MyApp"))
//! ```
//!
//! Fields with `Option<T>` type are automatically optional in key=value format.
//! Missing optional fields will be set to `None` instead of causing a parse
//! error.
//!
//! ## Skipping Fields
//!
//! ```ignore
//! #[derive(FromAnsi, ToAnsi)]
//! struct Data {
//!     id: u32,
//!     #[vtansi(skip)]
//!     internal: String,
//! }
//! // Only 'id' is encoded/decoded
//! ```
//!
//! # Debug Support
//!
//! Set the `VTANSI_DEBUG` environment variable to `1` or to a specific type
//! name to print the generated code during compilation.

#![recursion_limit = "128"]
#![forbid(unsafe_code)]

extern crate proc_macro;

mod helpers;
mod macros;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::env;
use syn::DeriveInput;

fn debug_print_generated(ast: &DeriveInput, toks: &TokenStream2) {
    let debug = env::var("VTANSI_DEBUG");
    if let Ok(s) = debug {
        if s == "1" {
            println!("{}", toks);
        }

        if ast.ident == s {
            println!("{}", toks);
        }
    }
}

/// Derive macro for `FromAnsi` trait.
///
/// This macro can be applied to:
///
/// ## Enums
///
/// Enums that either:
/// 1. Have a primitive integer representation (e.g., `#[repr(u8)]`)
/// 2. Implement `std::convert::TryFrom<&str>`
///
/// ## Structs
///
/// Structs with named or unnamed (tuple) fields where each field implements
/// `TryFromAnsi`. Tuple structs automatically use `value` format.
///
/// # Attributes
///
/// - `#[vtansi(default)]` - Mark a variant as the default fallback for
///   unrecognized values. Only one variant can be marked as default.
///   The default variant can be:
///   - A unit variant (returns a constant value)
///   - A tuple variant with one field (captures the unrecognized value)
///
/// # Examples
///
/// ## Unit default variant
///
/// ```ignore
/// #[derive(FromAnsi)]
/// #[repr(u8)]
/// enum Color {
///     Red = 0,
///     Green = 1,
///     Blue = 2,
///     #[vtansi(default)]
///     Unknown = 255,
/// }
/// ```
///
/// ## Capturing default variant
///
/// ```ignore
/// #[derive(FromAnsi)]
/// #[repr(u8)]
/// enum StatusCode {
///     Ok = 200,
///     NotFound = 404,
///     #[vtansi(default)]
///     Unknown(u8),  // Captures unrecognized codes
/// }
/// ```
///
/// ## String-based enum with capturing default
///
/// ```ignore
/// #[derive(FromAnsi)]
/// enum Command {
///     Quit,
///     Save,
///     #[vtansi(default)]
///     Custom(String),  // Captures unrecognized commands
/// }
///
/// impl TryFrom<&str> for Command {
///     type Error = String;
///     fn try_from(s: &str) -> Result<Self, Self::Error> {
///         // implementation
///     }
/// }
/// ```
#[proc_macro_derive(FromAnsi, attributes(vtansi))]
pub fn derive_from_ansi(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks =
        macros::from_ansi::from_ansi_inner(&ast).unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}

/// Derive macro for `ToAnsi` trait.
///
/// This macro can be applied to:
///
/// ## Enums
///
/// Enums that either:
/// 1. Have a primitive integer representation (e.g., `#[repr(u8)]`)
/// 2. Implement `AsRef<str>`
///
/// ## Structs
///
/// Structs with named or unnamed (tuple) fields where each field implements
/// `ToAnsi`. Tuple structs automatically use `value` format.
///
/// # Examples
///
/// For enums with primitive representations:
///
/// ```ignore
/// #[derive(ToAnsi)]
/// #[repr(u8)]
/// enum Color {
///     Red = 0,
///     Green = 1,
///     Blue = 2,
/// }
/// ```
///
/// For enums implementing `AsRef<str>`:
///
/// ```ignore
/// #[derive(ToAnsi)]
/// enum Mode {
///     Normal,
///     Insert,
/// }
///
/// impl AsRef<str> for Mode {
///     fn as_ref(&self) -> &str {
///         // implementation
///     }
/// }
/// ```
#[proc_macro_derive(ToAnsi)]
pub fn derive_to_ansi(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks = macros::to_ansi::to_ansi_inner(&ast).unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}
