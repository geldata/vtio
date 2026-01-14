//! Field properties and metadata extraction.
//!
//! This module provides utilities for extracting structured metadata from
//! struct field attributes. The `HasFieldProperties` trait provides a
//! uniform interface for parsing `#[vtansi(...)]` attributes on fields and
//! returning a structured representation.

use syn::Field;

use crate::helpers::metadata::FieldLocation;

use super::metadata::{FieldExt, FieldMeta, kw};
use super::occurrence_error;

/// Trait for extracting vtansi field properties from AST nodes.
///
/// This trait extends `syn::Field` with a method to extract all vtansi
/// properties in a structured form. Implementations scan the field's
/// attributes, parse them, and validate their consistency.
pub trait HasFieldProperties {
    /// Extract vtansi properties from this field.
    ///
    /// This method processes all `#[vtansi(...)]` attributes on the field
    /// and returns a `FieldProperties` struct containing the parsed
    /// metadata.
    ///
    /// # Errors
    ///
    /// Return an error if:
    /// - Attributes cannot be parsed
    /// - Multiple occurrences of the same attribute are found
    /// - Attribute values are invalid
    fn get_field_properties(&self) -> syn::Result<FieldProperties>;
}

/// Properties extracted from a field's vtansi attributes.
///
/// This struct represents all the metadata that can be specified on a struct
/// field using `#[vtansi(...)]` attributes. Each field corresponds to one
/// possible attribute.
#[derive(Clone)]
pub struct FieldProperties {
    /// The field's type.
    pub ty: syn::Type,

    /// Whether the field is an Option<T> type.
    pub is_optional: bool,

    /// The inner type if this is Option<T>, otherwise the type itself.
    pub inner_ty: syn::Type,

    /// The `#[vtansi(skip)]` keyword, if present.
    ///
    /// When this is `Some`, the field will be skipped during encoding and
    /// decoding.
    pub skip: Option<kw::skip>,

    /// Optional multiplexing target from `#[vtansi(muxwith = "field")]`.
    ///
    /// When this is `Some`, the field will be multiplexed with the specified
    /// field.
    pub mux_field: Option<syn::Member>,

    /// Where the data for this field is located within the sequence.
    pub location: Option<FieldLocation>,

    /// The `#[vtansi(flatten)]` keyword, if present.
    ///
    /// When this is `Some`, the field will delegate parameter iterator parsing
    /// to its `try_from_ansi_iter` method instead of consuming a single parameter.
    pub flatten: Option<kw::flatten>,
}

impl HasFieldProperties for Field {
    fn get_field_properties(&self) -> syn::Result<FieldProperties> {
        let ty = self.ty.clone();
        let is_optional = super::extract_option_inner_type(&ty).is_some();
        let inner_ty = if is_optional {
            super::extract_option_inner_type(&ty)
                .cloned()
                .unwrap_or_else(|| ty.clone())
        } else {
            ty.clone()
        };

        let mut skip_kw: Option<kw::skip> = None;
        let mut locate_kw: Option<kw::locate> = None;
        let mut location: Option<FieldLocation> = None;
        let mut mux_kw: Option<kw::muxwith> = None;
        let mut mux_field: Option<syn::Member> = None;
        let mut flatten_kw: Option<kw::flatten> = None;

        for meta in self.get_metadata()? {
            match meta {
                FieldMeta::Skip { kw } => {
                    if let Some(fst_kw) = skip_kw {
                        return Err(occurrence_error(fst_kw, kw, "skip"));
                    }

                    skip_kw = Some(kw);
                }
                FieldMeta::Location { kw, location: loc } => {
                    if let Some(fst_kw) = locate_kw {
                        return Err(occurrence_error(fst_kw, kw, "locate"));
                    }

                    locate_kw = Some(kw);
                    location = Some(loc);
                }
                FieldMeta::Mux { kw, field } => {
                    if let Some(fst_kw) = mux_kw {
                        return Err(occurrence_error(fst_kw, kw, "mux"));
                    }

                    mux_kw = Some(kw);
                    mux_field = Some(field);
                }
                FieldMeta::Flatten { kw } => {
                    if let Some(fst_kw) = flatten_kw {
                        return Err(occurrence_error(fst_kw, kw, "flatten"));
                    }

                    flatten_kw = Some(kw);
                }
            }
        }

        Ok(FieldProperties {
            ty,
            is_optional,
            inner_ty,
            skip: skip_kw,
            location,
            mux_field,
            flatten: flatten_kw,
        })
    }
}
