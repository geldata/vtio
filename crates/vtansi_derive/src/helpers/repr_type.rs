//! Logic for extracting primitive representation types from enums.
//!
//! This module provides utilities for parsing `#[repr(...)]` attributes on
//! enums to determine if they have a primitive integer representation.

use syn::{DeriveInput, Ident};

/// Extract the primitive representation type from a `#[repr(...)]` attribute.
///
/// This function scans the attributes on a type definition looking for a
/// `#[repr(...)]` attribute with a primitive integer type. If found, it
/// validates that the type is one of the supported integer types and returns
/// the identifier.
///
/// # Supported Types
///
/// - `u8`, `i8`
/// - `u16`, `i16`
/// - `u32`, `i32`
/// - `u64`, `i64`
/// - `usize`, `isize`
///
/// # Examples
///
/// ```ignore
/// #[repr(u8)]
/// enum Color {
///     Red = 0,
///     Green = 1,
///     Blue = 2,
/// }
/// // Returns: Some(Ident("u8"))
/// ```
///
/// ```ignore
/// enum Mode {
///     Normal,
///     Insert,
/// }
/// // Returns: None
/// ```
pub fn extract_repr_type(input: &DeriveInput) -> Option<Ident> {
    input.attrs.iter().find_map(|attr| {
        if !attr.path().is_ident("repr") {
            return None;
        }

        // Parse the repr attribute to get the primitive type
        let Ok(meta) = attr.parse_args::<Ident>() else {
            return None;
        };

        let type_str = meta.to_string();
        if matches!(
            type_str.as_str(),
            "u8" | "i8"
                | "u16"
                | "i16"
                | "u32"
                | "i32"
                | "u64"
                | "i64"
                | "usize"
                | "isize"
        ) {
            Some(meta)
        } else {
            None
        }
    })
}
