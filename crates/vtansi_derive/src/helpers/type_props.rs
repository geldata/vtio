//! Type-level properties and metadata extraction.
//!
//! This module provides utilities for extracting metadata from type-level
//! attributes on enums and structs, such as `#[repr(...)]` for enums or
//! `#[vtansi(format = "...")]` for structs. The extracted properties are
//! used to determine how to generate trait implementations.

use std::default::Default;
use syn::{Data, DeriveInput, Fields, Ident, LitStr};

use super::metadata::{DeriveInputExt, StructFormat, TypeMeta};
use super::repr_type::extract_repr_type;

/// Trait for extracting vtansi type properties from AST nodes.
///
/// This trait provides a uniform interface for extracting type-level
/// metadata from derive input AST nodes. Implementations scan the type's
/// attributes and return a structured representation of the vtansi-relevant
/// properties.
pub trait HasTypeProperties {
    /// Extract vtansi properties from this type.
    ///
    /// This method processes the type's attributes and returns a
    /// `TypeProperties` struct containing all relevant metadata for code
    /// generation.
    ///
    /// # Errors
    ///
    /// Return an error if the attributes cannot be parsed or contain
    /// invalid values.
    fn get_type_properties(&self) -> syn::Result<TypeProperties>;
}

/// Properties extracted from a type's attributes.
///
/// This struct holds metadata parsed from attributes like `#[repr(...)]` on
/// enum declarations or `#[vtansi(...)]` on struct declarations. It is used
/// during code generation to determine the appropriate implementation
/// strategy.
#[derive(Clone)]
pub struct TypeProperties {
    /// The primitive representation type from `#[repr(...)]`, if present.
    ///
    /// When an enum has a `#[repr(u8)]` or similar attribute, this field
    /// contains the primitive type identifier. This determines whether the
    /// enum uses integer-based or string-based conversion.
    pub repr_type: Option<Ident>,

    /// The struct encoding format from `#[vtansi(format = "...")]`.
    ///
    /// For structs, this determines whether fields are encoded as
    /// `key=value` pairs or as values only. Defaults to `KeyValue`.
    pub format: StructFormat,

    /// The field delimiter from `#[vtansi(delimiter = "...")]`.
    ///
    /// For structs, this determines what separator is used between fields.
    /// Defaults to `";"`.
    pub delimiter: LitStr,
}

impl Default for TypeProperties {
    fn default() -> Self {
        Self {
            repr_type: None,
            format: StructFormat::KeyValue,
            delimiter: LitStr::new(";", proc_macro2::Span::call_site()),
        }
    }
}

impl HasTypeProperties for DeriveInput {
    fn get_type_properties(&self) -> syn::Result<TypeProperties> {
        let repr_type = extract_repr_type(self);

        // Default format depends on struct type
        let default_format = match &self.data {
            Data::Struct(data) => match &data.fields {
                Fields::Unnamed(_) => StructFormat::Value,
                _ => StructFormat::KeyValue,
            },
            _ => StructFormat::KeyValue,
        };

        let mut format = default_format;
        let mut delimiter = LitStr::new(";", proc_macro2::Span::call_site());

        let mut format_kw = None;
        let mut delimiter_kw = None;

        for meta in self.get_type_metadata()? {
            match meta {
                TypeMeta::Format { kw, format: fmt } => {
                    if let Some(fst_kw) = format_kw {
                        let mut err = syn::Error::new_spanned(
                            kw,
                            "Found multiple occurrences of vtansi(format)",
                        );
                        err.combine(syn::Error::new_spanned(fst_kw, "first one here"));
                        return Err(err);
                    }

                    format_kw = Some(kw);
                    format = fmt;
                }
                TypeMeta::Delimiter {
                    kw,
                    delimiter: delim,
                } => {
                    if let Some(fst_kw) = delimiter_kw {
                        let mut err = syn::Error::new_spanned(
                            kw,
                            "Found multiple occurrences of vtansi(delimiter)",
                        );
                        err.combine(syn::Error::new_spanned(fst_kw, "first one here"));
                        return Err(err);
                    }

                    delimiter_kw = Some(kw);
                    delimiter = delim;
                }
            }
        }

        Ok(TypeProperties {
            repr_type,
            format,
            delimiter,
        })
    }
}
