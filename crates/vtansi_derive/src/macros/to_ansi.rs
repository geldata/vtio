//! Implementation of the `ToAnsi` derive macro.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::helpers::{non_enum_error, HasTypeProperties};

/// Generate the implementation of `ToAnsi` for an enum.
///
/// This function handles both primitive repr enums (converting to their
/// discriminant) and string-based enums (using `AsRef<str>`).
pub fn to_ansi_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Check if this is an enum
    let Data::Enum(_) = &ast.data else {
        return Err(non_enum_error());
    };

    // Extract type-level properties
    let type_properties = ast.get_type_properties()?;

    let expanded = if let Some(repr_type) = type_properties.repr_type {
        // Generate implementation using the primitive representation
        generate_repr_impl(
            name,
            &impl_generics,
            &ty_generics,
            where_clause,
            &repr_type,
        )
    } else {
        // Generate implementation using AsRef<str>
        generate_string_impl(name, &impl_generics, &ty_generics, where_clause)
    };

    Ok(expanded)
}

/// Generate implementation for enums with primitive repr.
fn generate_repr_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    repr_type: &syn::Ident,
) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl #impl_generics ::vtenc::encode::ToAnsi for #name #ty_generics #where_clause {
            fn to_ansi(&self) -> impl ::vtenc::encode::AnsiEncode {
                // Convert enum to its repr type
                *self as #repr_type
            }
        }
    }
}

/// Generate implementation for string-based enums.
fn generate_string_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl #impl_generics ::vtenc::encode::ToAnsi for #name #ty_generics #where_clause {
            fn to_ansi(&self) -> impl ::vtenc::encode::AnsiEncode {
                // Convert to string slice using AsRef<str>
                <Self as ::core::convert::AsRef<str>>::as_ref(self)
            }
        }
    }
}