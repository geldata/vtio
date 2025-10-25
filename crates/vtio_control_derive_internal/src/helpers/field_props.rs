//! Field-level properties and metadata extraction.
//!
//! This module provides utilities for extracting metadata from field-level
//! attributes and analyzing field types. The extracted properties are used
//! to determine how to encode and decode individual fields in escape
//! sequences.

use syn::{Field, Type};

use super::metadata::FieldExt;

/// Trait for extracting field properties from AST nodes.
///
/// This trait provides a uniform interface for extracting field-level
/// metadata from field AST nodes.
pub trait HasFieldProperties {
    /// Extract field properties from this field.
    ///
    /// This method processes the field's attributes and type information
    /// and returns a `FieldProperties` struct containing all relevant
    /// metadata for code generation.
    ///
    /// # Errors
    ///
    /// Return an error if the attributes cannot be parsed or contain
    /// invalid values.
    fn get_field_properties(&self) -> syn::Result<FieldProperties>;
}

/// Properties extracted from a field's attributes and type.
///
/// This struct holds metadata parsed from field attributes and type
/// information. It is used during code generation to determine how to
/// handle each field.
#[derive(Clone)]
pub struct FieldProperties {
    /// The field's identifier (name).
    pub ident: syn::Ident,

    /// The field's type.
    pub ty: Type,

    /// Whether the field is an Option<T> type.
    pub is_optional: bool,

    /// The inner type if this is Option<T>, otherwise the type itself.
    pub inner_ty: Type,

    /// Optional positional parameter index from #[paramidx(...)].
    pub param_idx: Option<usize>,
}

impl HasFieldProperties for Field {
    fn get_field_properties(&self) -> syn::Result<FieldProperties> {
        let ident = self
            .ident
            .clone()
            .ok_or_else(|| {
                syn::Error::new_spanned(
                    self,
                    "field must have an identifier (named fields only)",
                )
            })?;

        let ty = self.ty.clone();
        let is_optional = is_option_type(&ty);
        let inner_ty = if is_optional {
            extract_option_inner_type(&ty)
                .cloned()
                .unwrap_or_else(|| ty.clone())
        } else {
            ty.clone()
        };

        let param_idx = self.get_paramidx()?;

        Ok(FieldProperties {
            ident,
            ty,
            is_optional,
            inner_ty,
            param_idx,
        })
    }
}

/// Check if a type is Option<T>.
///
/// Detect optional fields for conditional encoding.
pub fn is_option_type(ty: &Type) -> bool {
    extract_option_inner_type(ty).is_some()
}

/// Extract the inner type `T` from `Option<T>` (cloning variant).
///
/// Returns the type itself if not an `Option`.
pub fn extract_option_inner_type_cloned(ty: &Type) -> Type {
    extract_option_inner_type(ty)
        .cloned()
        .unwrap_or_else(|| ty.clone())
}

/// Extract the inner type `T` from `Option<T>`.
///
/// Returns `None` if the type is not an `Option`.
#[allow(clippy::collapsible_if)]
pub fn extract_option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if type_path.qself.is_some() {
            return None;
        }

        let path = &type_path.path;

        // Check for Option with generic argument
        let last_segment = path.segments.last()?;

        // Check if the last segment is "Option"
        if last_segment.ident != "Option" {
            return None;
        }

        // Check that the path is either just "Option" or
        // "std/core::option::Option"
        let is_valid_path = if path.segments.len() == 1 {
            true
        } else if path.segments.len() == 3 {
            let segs: Vec<_> =
                path.segments.iter().map(|s| s.ident.to_string()).collect();
            (segs[0] == "std" || segs[0] == "core") && segs[1] == "option"
        } else {
            false
        };

        if !is_valid_path {
            return None;
        }

        // Extract the inner type from the angle brackets
        if let syn::PathArguments::AngleBracketed(args) =
            &last_segment.arguments
        {
            if args.args.len() == 1 {
                if let syn::GenericArgument::Type(inner_ty) = &args.args[0] {
                    return Some(inner_ty);
                }
            }
        }
    }

    None
}

/// Check if a type is a unit type `()`.
///
/// Helper to detect unit type fields that should be skipped during
/// encoding.
pub fn is_unit_type(ty: &Type) -> bool {
    matches!(ty, Type::Tuple(tuple) if tuple.elems.is_empty())
}