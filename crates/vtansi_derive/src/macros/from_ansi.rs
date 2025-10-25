//! Implementation of the `FromAnsi` derive macro.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::helpers::{
    find_default_variant, non_enum_error, DefaultVariant, HasTypeProperties,
};

/// Generate the implementation of `TryFromAnsi` for an enum.
///
/// This function handles both primitive repr enums and string-based enums,
/// with optional default variant support for capturing unrecognized values.
pub fn from_ansi_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Check if this is an enum
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
                #[automatically_derived]
                impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
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
                #[automatically_derived]
                impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
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
                #[automatically_derived]
                impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
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
                #[automatically_derived]
                impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
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
                #[automatically_derived]
                impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
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
                #[automatically_derived]
                impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
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