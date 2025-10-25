//! Logic for extracting primitive representation types from enums.

use syn::{DeriveInput, Ident};

/// Extract the primitive representation type from a `#[repr(...)]`
/// attribute.
///
/// Returns `Some(repr_type)` if the enum has a valid primitive repr
/// attribute (u8, i8, u16, i16, u32, i32, u64, i64, usize, isize),
/// otherwise returns `None`.
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
            "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64"
                | "usize" | "isize"
        ) {
            Some(meta)
        } else {
            None
        }
    })
}