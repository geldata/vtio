//! Type analysis utilities for extracting information from Rust types.
//!
//! This module provides helpers for analyzing type structures, such as
//! extracting the inner type from generic containers like `Vec<T>`.

use syn::{GenericArgument, PathArguments, Type};

/// Extract the inner type from a `Vec<T>` type.
///
/// Return `Some(&T)` if the type is `Vec<T>`, otherwise return `None`.
///
/// # Examples
///
/// ```ignore
/// let ty: Type = syn::parse_quote!(Vec<u8>);
/// let inner = extract_vec_inner_type(&ty);
/// assert!(inner.is_some());
/// ```
pub fn extract_vec_inner_type(ty: &Type) -> Option<&Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };

    // Check if this is a simple path (not qualified)
    if type_path.qself.is_some() {
        return None;
    }

    // Get the last segment of the path
    let segment = type_path.path.segments.last()?;

    // Check if the segment is named "Vec"
    if segment.ident != "Vec" {
        return None;
    }

    // Extract the generic argument
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };

    // Get the first generic argument
    if args.args.len() != 1 {
        return None;
    }

    let GenericArgument::Type(inner_ty) = args.args.first()? else {
        return None;
    };

    Some(inner_ty)
}

/// Check if a type is a `Vec<T>` for any T.
///
/// Return `true` if the type is `Vec<T>`, otherwise return `false`.
#[inline]
pub fn is_vec_type(ty: &Type) -> bool {
    extract_vec_inner_type(ty).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_extract_vec_inner_type() {
        let ty: Type = parse_quote!(Vec<u8>);
        let inner = extract_vec_inner_type(&ty);
        assert!(inner.is_some());

        let inner_ty = inner.unwrap();
        if let Type::Path(path) = inner_ty {
            assert_eq!(path.path.segments.first().unwrap().ident, "u8");
        } else {
            panic!("Expected Type::Path");
        }
    }

    #[test]
    fn test_extract_vec_inner_type_complex() {
        let ty: Type = parse_quote!(Vec<Option<String>>);
        let inner = extract_vec_inner_type(&ty);
        assert!(inner.is_some());
    }

    #[test]
    fn test_extract_vec_inner_type_not_vec() {
        let ty: Type = parse_quote!(u8);
        let inner = extract_vec_inner_type(&ty);
        assert!(inner.is_none());

        let ty: Type = parse_quote!(Option<u8>);
        let inner = extract_vec_inner_type(&ty);
        assert!(inner.is_none());
    }

    #[test]
    fn test_is_vec_type() {
        let ty: Type = parse_quote!(Vec<u8>);
        assert!(is_vec_type(&ty));

        let ty: Type = parse_quote!(u8);
        assert!(!is_vec_type(&ty));

        let ty: Type = parse_quote!(Option<u8>);
        assert!(!is_vec_type(&ty));
    }
}
