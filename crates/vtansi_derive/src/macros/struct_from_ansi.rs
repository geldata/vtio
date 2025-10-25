//! Implementation of `FromAnsi` for structs.
//!
//! This module generates implementations of the `TryFromAnsi` trait for
//! structs with named or unnamed (tuple) fields. It supports two parsing
//! formats:
//!
//! 1. **Key-Value format** (default for named fields) - fields are parsed as
//!    `key=value` pairs separated by a delimiter. Fields can appear in any
//!    order.
//! 2. **Value format** (default for tuple structs) - fields are parsed as
//!    values in declaration order, separated by a delimiter.
//!
//! The format and delimiter can be configured using `#[vtansi(format = "...")]`
//! and `#[vtansi(delimiter = "...")]` attributes on the struct.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::helpers::{
    metadata::StructFormat, non_struct_error, HasFieldProperties, HasTypeProperties,
};

/// Generate the implementation of `TryFromAnsi` for a struct.
///
/// This function generates parsing code based on the struct's format
/// attribute (`key=value` or `value`) and delimiter. For tuple structs,
/// it automatically uses `value` format.
///
/// # Errors
///
/// Return an error if:
/// - The input is not a struct with named or unnamed fields
/// - The attributes cannot be parsed
/// - Fields have invalid configurations
/// - `key=value` format is used with tuple structs
pub fn generate_struct_impl(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let Data::Struct(data) = &ast.data else {
        return Err(non_struct_error());
    };

    // Extract type-level properties
    let type_properties = ast.get_type_properties()?;
    let format = type_properties.format;
    let delimiter = type_properties.delimiter.value();

    match &data.fields {
        Fields::Named(fields) => {
            // Collect non-skipped fields
            let mut field_names = Vec::new();
            let mut field_types = Vec::new();

            for field in &fields.named {
                let field_props = field.get_field_properties()?;
                if field_props.skip.is_some() {
                    continue;
                }

                let field_name = field.ident.as_ref().unwrap();
                field_names.push(field_name.clone());
                field_types.push(&field.ty);
            }

            let expanded = match format {
                StructFormat::KeyValue => generate_keyvalue_impl(
                    name,
                    &impl_generics,
                    &ty_generics,
                    where_clause,
                    &field_names,
                    &field_types,
                    &delimiter,
                ),
                StructFormat::Value => generate_named_value_impl(
                    name,
                    &impl_generics,
                    &ty_generics,
                    where_clause,
                    &field_names,
                    &field_types,
                    &delimiter,
                ),
            };

            Ok(expanded)
        }
        Fields::Unnamed(fields) => {
            // Tuple structs must use value format
            if format == StructFormat::KeyValue {
                return Err(syn::Error::new_spanned(
                    ast,
                    "Tuple structs cannot use key=value format. Use \
                     #[vtansi(format = \"value\")] or omit the format attribute",
                ));
            }

            // Collect field types (no names, no skip support)
            let field_types: Vec<_> = fields.unnamed.iter().map(|f| &f.ty).collect();

            let expanded = generate_tuple_value_impl(
                name,
                &impl_generics,
                &ty_generics,
                where_clause,
                &field_types,
                &delimiter,
            );

            Ok(expanded)
        }
        Fields::Unit => Err(syn::Error::new_spanned(
            ast,
            "FromAnsi cannot be derived for unit structs",
        )),
    }
}

/// Generate implementation for structs with key=value format.
///
/// This function creates a `TryFromAnsi` implementation that parses the
/// input as `key=value` pairs separated by the delimiter. Fields can appear
/// in any order.
fn generate_keyvalue_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    field_names: &[syn::Ident],
    field_types: &[&syn::Type],
    delimiter: &str,
) -> TokenStream {
    let field_declarations = field_names.iter().zip(field_types.iter()).map(|(name, ty)| {
        quote! {
            let mut #name: ::core::option::Option<#ty> = ::core::option::Option::None;
        }
    });

    let field_assignments = field_names.iter().zip(field_types.iter()).map(|(name, ty)| {
        let name_str = name.to_string();
        quote! {
            #name_str => {
                #name = ::core::option::Option::Some(
                    <#ty as ::vtenc::parse::TryFromAnsi>::try_from_ansi(value.as_bytes())?
                );
            }
        }
    });

    let field_unwrapping = field_names.iter().map(|name| {
        let name_str = name.to_string();
        quote! {
            let #name = #name.ok_or_else(|| {
                ::vtenc::parse::ParseError::InvalidValue(
                    ::std::format!("missing field: {}", #name_str)
                )
            })?;
        }
    });

    let field_construction = field_names.iter().map(|name| {
        quote! { #name }
    });

    let delimiter_lit = syn::LitStr::new(delimiter, proc_macro2::Span::call_site());

    quote! {
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
            #[inline]
            fn try_from_ansi(bytes: &[u8]) -> ::core::result::Result<Self, ::vtenc::parse::ParseError> {
                // Parse as UTF-8 string
                let s = <&str as ::vtenc::parse::TryFromAnsi>::try_from_ansi(bytes)?;

                // Initialize all fields as None
                #(#field_declarations)*

                // Parse key=value pairs using helper function
                for pair_result in ::vtenc::parse_keyvalue_pairs(s, #delimiter_lit) {
                    let (key, value) = pair_result?;
                    match key {
                        #(#field_assignments)*
                        _ => {
                            return ::core::result::Result::Err(
                                ::vtenc::parse::ParseError::InvalidValue(
                                    ::std::format!("unknown field: {}", key)
                                )
                            );
                        }
                    }
                }

                // Unwrap all fields or return error for missing ones
                #(#field_unwrapping)*

                ::core::result::Result::Ok(Self {
                    #(#field_construction),*
                })
            }
        }
    }
}

/// Generate implementation for named structs with value format.
///
/// This function creates a `TryFromAnsi` implementation that parses the
/// input as values in field declaration order, separated by the delimiter.
fn generate_named_value_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    field_names: &[syn::Ident],
    field_types: &[&syn::Type],
    delimiter: &str,
) -> TokenStream {
    let field_count = field_names.len();

    let field_parsing = field_names
        .iter()
        .zip(field_types.iter())
        .enumerate()
        .map(|(idx, (name, ty))| {
            quote! {
                let #name = if #idx < parts.len() {
                    <#ty as ::vtenc::parse::TryFromAnsi>::try_from_ansi(
                        parts[#idx].trim().as_bytes()
                    )?
                } else {
                    return ::core::result::Result::Err(
                        ::vtenc::parse::ParseError::InvalidValue(
                            ::std::format!("expected {} fields, got {}", #field_count, parts.len())
                        )
                    );
                };
            }
        });

    let field_construction = field_names.iter().map(|name| {
        quote! { #name }
    });

    let delimiter_lit = syn::LitStr::new(delimiter, proc_macro2::Span::call_site());

    quote! {
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
            #[inline]
            fn try_from_ansi(bytes: &[u8]) -> ::core::result::Result<Self, ::vtenc::parse::ParseError> {
                // Parse as UTF-8 string
                let s = <&str as ::vtenc::parse::TryFromAnsi>::try_from_ansi(bytes)?;

                // Parse delimited values using helper function
                let parts = ::vtenc::parse_delimited_values(s, #delimiter_lit, #field_count)?;

                // Parse each field in order
                #(#field_parsing)*

                ::core::result::Result::Ok(Self {
                    #(#field_construction),*
                })
            }
        }
    }
}

/// Generate implementation for tuple structs.
///
/// This function creates a `TryFromAnsi` implementation that parses the
/// input as values in field declaration order, separated by the delimiter.
/// Tuple struct fields are accessed by position (0, 1, 2, etc.).
fn generate_tuple_value_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    field_types: &[&syn::Type],
    delimiter: &str,
) -> TokenStream {
    let field_count = field_types.len();

    let field_parsing = field_types.iter().enumerate().map(|(idx, ty)| {
        let field_name = syn::Ident::new(&format!("field_{}", idx), proc_macro2::Span::call_site());
        quote! {
            let #field_name = if #idx < parts.len() {
                <#ty as ::vtenc::parse::TryFromAnsi>::try_from_ansi(
                    parts[#idx].trim().as_bytes()
                )?
            } else {
                return ::core::result::Result::Err(
                    ::vtenc::parse::ParseError::InvalidValue(
                        ::std::format!("expected {} fields, got {}", #field_count, parts.len())
                    )
                );
            };
        }
    });

    let field_construction = (0..field_count).map(|idx| {
        let field_name = syn::Ident::new(&format!("field_{}", idx), proc_macro2::Span::call_site());
        quote! { #field_name }
    });

    let delimiter_lit = syn::LitStr::new(delimiter, proc_macro2::Span::call_site());

    quote! {
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl #impl_generics ::vtenc::parse::TryFromAnsi<'_> for #name #ty_generics #where_clause {
            #[inline]
            fn try_from_ansi(bytes: &[u8]) -> ::core::result::Result<Self, ::vtenc::parse::ParseError> {
                // Parse as UTF-8 string
                let s = <&str as ::vtenc::parse::TryFromAnsi>::try_from_ansi(bytes)?;

                // Parse delimited values using helper function
                let parts = ::vtenc::parse_delimited_values(s, #delimiter_lit, #field_count)?;

                // Parse each field in order
                #(#field_parsing)*

                ::core::result::Result::Ok(Self(
                    #(#field_construction),*
                ))
            }
        }
    }
}