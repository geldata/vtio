//! Type-level properties and metadata extraction.

use std::default::Default;
use syn::{DeriveInput, Ident};

use super::repr_type::extract_repr_type;

/// Trait for extracting vtansi type properties from AST nodes.
pub trait HasTypeProperties {
    /// Extract vtansi properties from this type.
    fn get_type_properties(&self) -> syn::Result<TypeProperties>;
}

/// Properties extracted from an enum's type-level attributes.
#[derive(Clone, Default)]
pub struct TypeProperties {
    /// The primitive representation type from `#[repr(...)]`, if present.
    pub repr_type: Option<Ident>,
}

impl HasTypeProperties for DeriveInput {
    fn get_type_properties(&self) -> syn::Result<TypeProperties> {
        let repr_type = extract_repr_type(self);

        Ok(TypeProperties { repr_type })
    }
}