//! Type-level properties and metadata extraction.
//!
//! This module provides utilities for extracting metadata from type-level
//! attributes on enums, such as `#[repr(...)]`. The extracted properties
//! are used to determine how to generate trait implementations.

use std::default::Default;
use syn::{DeriveInput, Ident};

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

/// Properties extracted from an enum's type-level attributes.
///
/// This struct holds metadata parsed from attributes like `#[repr(...)]` on
/// the enum declaration. It is used during code generation to determine the
/// appropriate implementation strategy.
#[derive(Clone, Default)]
pub struct TypeProperties {
    /// The primitive representation type from `#[repr(...)]`, if present.
    ///
    /// When an enum has a `#[repr(u8)]` or similar attribute, this field
    /// contains the primitive type identifier. This determines whether the
    /// enum uses integer-based or string-based conversion.
    pub repr_type: Option<Ident>,
}

impl HasTypeProperties for DeriveInput {
    fn get_type_properties(&self) -> syn::Result<TypeProperties> {
        let repr_type = extract_repr_type(self);

        Ok(TypeProperties { repr_type })
    }
}