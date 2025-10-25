//! Implementation of the `FromAnsi` derive macro.
//!
//! This module generates implementations of the `TryFromAnsi` trait for both
//! enums and structs.
//!
//! # Enum Support
//!
//! For enums, it supports two parsing strategies:
//!
//! 1. **Primitive representation** - for enums with `#[repr(u8)]` and similar
//!    attributes, parsing from integer values
//! 2. **String-based** - for enums without repr, parsing from string values
//!    via `TryFrom<&str>`
//!
//! Both strategies support optional default variants that can either return a
//! constant value or capture the unparsed input when parsing fails.
//!
//! # Struct Support
//!
//! For structs, see the `struct_from_ansi` module.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::helpers::{
    find_default_variant, non_enum_error, DefaultVariant, HasTypeProperties,
};

use super::struct_from_ansi;

/// Generate the implementation of `TryFromAnsi` for an enum or struct.
///
/// This function dispatches to the appropriate generator based on whether the
/// input is an enum or a struct.
///
/// # Errors
///
/// Return an error if:
/// - The input is neither an enum nor a struct with named fields
/// - The attributes cannot be parsed
/// - The configuration is invalid
pub fn from_ansi_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    match &ast.data {
        Data::Enum(_) => generate_enum_impl(ast),
        Data::Struct(data) => match &data.fields {
            Fields::Named(_) | Fields::Unnamed(_) => struct_from_ansi::generate_struct_impl(ast),
            Fields::Unit => Err(syn::Error::new_spanned(
                ast,
                "FromAnsi cannot be derived for unit structs",
            )),
        },
        Data::Union(_) => Err(syn::Error::new_spanned(
            ast,
            "FromAnsi cannot be derived for unions",
        )),
    }
}

/// Generate the implementation of `TryFromAnsi` for an enum.
///
/// This function orchestrates the code generation process by:
/// 1. Extracting type-level properties (e.g., repr type)
/// 2. Finding the default variant, if any
/// 3. Delegating to the appropriate generation function based on the repr
///    type
///
/// # Errors
///
/// Return an error if:
/// - The attributes cannot be parsed
/// - The default variant is invalid
fn generate_enum_impl(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let Data::Enum(enum_data) = &ast.data else {
        return Err(non_enum_error());
    };

    // Extract type-level properties
    let type_properties = ast.get_type_properties()?;

    // Find default variant if any
    let default_variant = find_default_variant(enum_data)?;

    let expanded = if let Some(repr_type) = type_properties.repr_type {
        // Generate implementation using the primitive representation
        generate_repr_impl(
            name,
            &impl_generics,
            &ty_generics,
            where_clause,
            &repr_type,
            default_variant,
        )
    } else {
        // Generate implementation using TryFrom<&str>
        generate_string_impl(
            name,
            &impl_generics,
            &ty_generics,
            where_clause,
            default_variant,
        )
    };

    Ok(expanded)
}

/// Generate implementation for enums with primitive repr.
///
/// This function creates a `TryFromAnsi` implementation that:
/// 1. Parses the input bytes as the primitive repr type
/// 2. Attempts to convert the number to the enum using `TryFrom`
/// 3. If a default variant is present, uses it on conversion failure
/// 4. Otherwise, returns an error on conversion failure
///
/// The implementation differs based on whether a default variant is present
/// and whether it's a unit or capturing variant.
fn generate_repr_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    repr_type: &syn::Ident,
    default_variant: Option<DefaultVariant>,
) -> TokenStream {
    match default_variant {
        Some(DefaultVariant::Unit(default_var)) => {
            quote! {
                #[allow(clippy::use_self)]
                #[automatically_derived]
                impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
                    #[inline]
                    fn try_from_ansi(bytes: &[u8]) -> ::core::result::Result<Self, ::vtenc::parse::ParseError> {
                        use ::core::convert::TryFrom;

                        // Parse as the repr type
                        let num = <#repr_type as ::vtenc::parse::TryFromAnsi>::try_from_ansi(bytes)?;

                        // Convert to enum using TryFrom, or use default on
                        // failure
                        ::core::result::Result::Ok(Self::try_from(num).unwrap_or(Self::#default_var))
                    }
                }
            }
        }
        Some(DefaultVariant::Capturing(default_var)) => {
            quote! {
                #[allow(clippy::use_self)]
                #[automatically_derived]
                impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
                    #[inline]
                    fn try_from_ansi(bytes: &[u8]) -> ::core::result::Result<Self, ::vtenc::parse::ParseError> {
                        use ::core::convert::TryFrom;

                        // Parse as the repr type
                        let num = <#repr_type as ::vtenc::parse::TryFromAnsi>::try_from_ansi(bytes)?;

                        // Convert to enum using TryFrom, or capture value in
                        // default on failure
                        ::core::result::Result::Ok(
                            Self::try_from(num).unwrap_or_else(|_| Self::#default_var(num.into()))
                        )
                    }
                }
            }
        }
        None => {
            quote! {
                #[allow(clippy::use_self)]
                #[automatically_derived]
                impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
                    #[inline]
                    fn try_from_ansi(bytes: &[u8]) -> ::core::result::Result<Self, ::vtenc::parse::ParseError> {
                        use ::core::convert::TryFrom;

                        // Parse as the repr type
                        let num = <#repr_type as ::vtenc::parse::TryFromAnsi>::try_from_ansi(bytes)?;

                        // Convert to enum using TryFrom
                        Self::try_from(num).map_err(|_| {
                            ::vtenc::parse::ParseError::InvalidValue(
                                ::std::format!("invalid enum discriminant: {}", num)
                            )
                        })
                    }
                }
            }
        }
    }
}

/// Generate implementation for string-based enums.
///
/// This function creates a `TryFromAnsi` implementation that:
/// 1. Parses the input bytes as a UTF-8 string slice
/// 2. Attempts to convert the string to the enum using `TryFrom<&str>`
/// 3. If a default variant is present, uses it on conversion failure
/// 4. Otherwise, returns an error on conversion failure
///
/// The implementation differs based on whether a default variant is present
/// and whether it's a unit or capturing variant.
fn generate_string_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    default_variant: Option<DefaultVariant>,
) -> TokenStream {
    match default_variant {
        Some(DefaultVariant::Unit(default_var)) => {
            quote! {
                #[allow(clippy::use_self)]
                #[automatically_derived]
                impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
                    #[inline]
                    fn try_from_ansi(bytes: &[u8]) -> ::core::result::Result<Self, ::vtenc::parse::ParseError> {
                        use ::core::convert::TryFrom;

                        // Parse as &str
                        let s = <&str as ::vtenc::parse::TryFromAnsi>::try_from_ansi(bytes)?;

                        // Convert to enum using TryFrom<&str>, or use default
                        // on failure
                        ::core::result::Result::Ok(Self::try_from(s).unwrap_or(Self::#default_var))
                    }
                }
            }
        }
        Some(DefaultVariant::Capturing(default_var)) => {
            quote! {
                #[allow(clippy::use_self)]
                #[automatically_derived]
                impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
                    #[inline]
                    fn try_from_ansi(bytes: &[u8]) -> ::core::result::Result<Self, ::vtenc::parse::ParseError> {
                        use ::core::convert::TryFrom;

                        // Parse as &str
                        let s = <&str as ::vtenc::parse::TryFromAnsi>::try_from_ansi(bytes)?;

                        // Convert to enum using TryFrom<&str>, or capture
                        // value in default on failure
                        ::core::result::Result::Ok(
                            Self::try_from(s).unwrap_or_else(|_| Self::#default_var(s.into()))
                        )
                    }
                }
            }
        }
        None => {
            quote! {
                #[allow(clippy::use_self)]
                #[automatically_derived]
                impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
                    #[inline]
                    fn try_from_ansi(bytes: &[u8]) -> ::core::result::Result<Self, ::vtenc::parse::ParseError> {
                        use ::core::convert::TryFrom;

                        // Parse as &str
                        let s = <&str as ::vtenc::parse::TryFromAnsi>::try_from_ansi(bytes)?;

                        // Convert to enum using TryFrom<&str>
                        Self::try_from(s).map_err(|_| {
                            ::vtenc::parse::ParseError::InvalidValue(
                                ::std::format!("invalid enum value: {}", s)
                            )
                        })
                    }
                }
            }
        }
    }
}