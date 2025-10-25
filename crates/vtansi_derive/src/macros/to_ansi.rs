//! Implementation of the `ToAnsi` derive macro.
//!
//! This module generates implementations of the `ToAnsi` trait for enums. It
//! supports two encoding strategies:
//!
//! 1. **Primitive representation** - for enums with `#[repr(u8)]` and similar
//!    attributes, encoding as integer values
//! 2. **String-based** - for enums without repr, encoding as string values
//!    via `AsRef<str>`
//!
//! The generated implementations are optimized with `#[inline]` attributes
//! for better performance.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::helpers::{non_enum_error, HasTypeProperties};

/// Generate the implementation of `ToAnsi` for an enum.
///
/// This function orchestrates the code generation process by:
/// 1. Validating that the input is an enum
/// 2. Extracting type-level properties (e.g., repr type)
/// 3. Delegating to the appropriate generation function based on the repr
///    type
///
/// # Errors
///
/// Return an error if:
/// - The input is not an enum
/// - The attributes cannot be parsed
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
///
/// This function creates a `ToAnsi` implementation that converts the enum to
/// its primitive representation type (e.g., `u8`, `i32`) using a cast. The
/// resulting number is then encoded via its `AnsiEncode` implementation.
///
/// The generated code is marked with `#[inline]` to encourage the compiler
/// to optimize away the intermediate conversion.
fn generate_repr_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    repr_type: &syn::Ident,
) -> TokenStream {
    quote! {
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl #impl_generics ::vtenc::encode::ToAnsi for #name #ty_generics #where_clause {
            #[inline]
            fn to_ansi(&self) -> impl ::vtenc::encode::AnsiEncode {
                // Convert enum to its repr type
                *self as #repr_type
            }
        }
    }
}

/// Generate implementation for string-based enums.
///
/// This function creates a `ToAnsi` implementation that converts the enum to
/// a string slice using the `AsRef<str>` trait. The enum must implement this
/// trait for the generated code to compile.
///
/// The generated code is marked with `#[inline]` to encourage the compiler
/// to optimize the conversion.
fn generate_string_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
) -> TokenStream {
    quote! {
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl #impl_generics ::vtenc::encode::ToAnsi for #name #ty_generics #where_clause {
            #[inline]
            fn to_ansi(&self) -> impl ::vtenc::encode::AnsiEncode {
                // Convert to string slice using AsRef<str>
                <Self as ::core::convert::AsRef<str>>::as_ref(self)
            }
        }
    }
}