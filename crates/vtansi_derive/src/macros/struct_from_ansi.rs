//! Implementation of `FromAnsi` for structs.
//!
//! This module generates implementations of the `TryFromAnsi` trait for
//! structs with named or unnamed (tuple) fields. It supports two parsing
//! formats:
//!
//! 1. **Key-Value format** (default for named fields) - fields are parsed as
//!    `key=value` pairs separated by a delimiter. Fields can appear in any
//!    order. Fields with `Option<T>` type are automatically optional.
//! 2. **Value format** (default for tuple structs) - fields are parsed as
//!    values in declaration order, separated by a delimiter. Supports optional
//!    trailing fields with `Option<T>` type. Optional fields must appear after
//!    all required fields.
//!
//! The format and delimiter can be configured using `#[vtansi(format = "...")]`
//! and `#[vtansi(delimiter = "...")]` attributes on the struct.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::helpers::{
    HasFieldProperties, HasTypeProperties, extract_option_inner_type, metadata::StructFormat,
    non_struct_error,
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
/// in any order. Fields with `Option<T>` type are optional and will be
/// `None` if not present.
fn generate_keyvalue_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    field_names: &[syn::Ident],
    field_types: &[&syn::Type],
    delimiter: &str,
) -> TokenStream {
    // Detect which fields are Option types and extract inner types
    let field_info: Vec<_> = field_types
        .iter()
        .map(|ty| {
            if let Some(inner_ty) = extract_option_inner_type(ty) {
                (true, inner_ty)
            } else {
                (false, *ty)
            }
        })
        .collect();

    let field_declarations = field_names
        .iter()
        .zip(field_types.iter())
        .zip(field_info.iter())
        .map(|((name, ty), (is_option, _))| {
            if *is_option {
                // For Option fields, initialize as None
                quote! {
                    let mut #name: #ty = ::core::option::Option::None;
                }
            } else {
                // For required fields, wrap in Option for tracking presence
                quote! {
                    let mut #name: ::core::option::Option<#ty> = ::core::option::Option::None;
                }
            }
        });

    let field_assignments = field_names
        .iter()
        .zip(field_info.iter())
        .map(|(name, (is_option, inner_ty))| {
            let name_str = name.to_string();
            if *is_option {
                // For Option fields, parse the inner type T and wrap in Some
                quote! {
                    #name_str => {
                        #name = ::core::option::Option::Some(
                            <#inner_ty as ::vtenc::parse::TryFromAnsi>::try_from_ansi(value.as_bytes())?
                        );
                    }
                }
            } else {
                quote! {
                    #name_str => {
                        #name = ::core::option::Option::Some(
                            <#inner_ty as ::vtenc::parse::TryFromAnsi>::try_from_ansi(value.as_bytes())?
                        );
                    }
                }
            }
        });

    let field_unwrapping =
        field_names
            .iter()
            .zip(field_info.iter())
            .filter_map(|(name, (is_option, _))| {
                if *is_option {
                    // Option fields don't need unwrapping - they're already optional
                    None
                } else {
                    // Required fields must be present
                    let name_str = name.to_string();
                    Some(quote! {
                        let #name = #name.ok_or_else(|| {
                            ::vtenc::parse::ParseError::InvalidValue(
                                ::std::format!("missing field: {}", #name_str)
                            )
                        })?;
                    })
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
/// Supports optional trailing fields (fields with `Option<T>` type).
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

    // Detect which fields are Option types and validate ordering
    let field_info: Vec<_> = field_types
        .iter()
        .map(|ty| {
            if let Some(inner_ty) = extract_option_inner_type(ty) {
                (true, inner_ty)
            } else {
                (false, *ty)
            }
        })
        .collect();

    // Find the index of the first optional field
    let first_optional_idx = field_info.iter().position(|(is_opt, _)| *is_opt);

    // Validate that all optional fields trail non-optional fields
    if let Some(first_opt_idx) = first_optional_idx {
        for (idx, (is_opt, _)) in field_info.iter().enumerate() {
            if idx > first_opt_idx && !is_opt {
                // Found a non-optional field after an optional field
                let opt_field = &field_names[first_opt_idx];
                let req_field = &field_names[idx];
                return syn::Error::new_spanned(
                    req_field,
                    format!(
                        "non-optional field '{}' cannot appear after optional field '{}'",
                        req_field, opt_field
                    ),
                )
                .to_compile_error();
            }
        }
    }

    let required_count = first_optional_idx.unwrap_or(field_count);

    let field_parsing = field_names
        .iter()
        .zip(field_types.iter())
        .zip(field_info.iter())
        .enumerate()
        .map(|(idx, ((name, ty), (is_option, inner_ty)))| {
            if *is_option {
                // Optional field: parse if present, otherwise None
                quote! {
                    let #name: #ty = match __iter.next() {
                        ::core::option::Option::Some(__part) if !__part.trim().is_empty() => {
                            ::core::option::Option::Some(
                                <#inner_ty as ::vtenc::parse::TryFromAnsi>::try_from_ansi(
                                    __part.trim().as_bytes()
                                )?
                            )
                        }
                        _ => ::core::option::Option::None,
                    };
                }
            } else {
                // Required field: must be present
                quote! {
                    let #name = match __iter.next() {
                        ::core::option::Option::Some(__part) => {
                            <#ty as ::vtenc::parse::TryFromAnsi>::try_from_ansi(
                                __part.trim().as_bytes()
                            )?
                        }
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                ::vtenc::parse::ParseError::InvalidValue(
                                    ::std::format!("expected at least {} fields, got {}", #required_count, #idx)
                                )
                            );
                        }
                    };
                }
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

                // Create iterator over fields - no allocation
                let mut __iter = s.split(#delimiter_lit);
                
                // Parse each field in order
                #(#field_parsing)*

                // Check for too many fields
                if __iter.next().is_some() {
                    return ::core::result::Result::Err(
                        ::vtenc::parse::ParseError::InvalidValue(
                            ::std::format!("expected at most {} fields", #field_count)
                        )
                    );
                }

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
/// Supports optional trailing fields (fields with `Option<T>` type).
fn generate_tuple_value_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    field_types: &[&syn::Type],
    delimiter: &str,
) -> TokenStream {
    let field_count = field_types.len();

    // Detect which fields are Option types and validate ordering
    let field_info: Vec<_> = field_types
        .iter()
        .map(|ty| {
            if let Some(inner_ty) = extract_option_inner_type(ty) {
                (true, inner_ty)
            } else {
                (false, *ty)
            }
        })
        .collect();

    // Find the index of the first optional field
    let first_optional_idx = field_info.iter().position(|(is_opt, _)| *is_opt);

    // Validate that all optional fields trail non-optional fields
    if let Some(first_opt_idx) = first_optional_idx {
        for (idx, (is_opt, _)) in field_info.iter().enumerate() {
            if idx > first_opt_idx && !is_opt {
                // Found a non-optional field after an optional field
                return syn::Error::new_spanned(
                    name,
                    format!(
                        "non-optional field at position {} cannot appear after optional field at position {}",
                        idx, first_opt_idx
                    ),
                )
                .to_compile_error();
            }
        }
    }

    let required_count = first_optional_idx.unwrap_or(field_count);

    let field_parsing = field_types
        .iter()
        .zip(field_info.iter())
        .enumerate()
        .map(|(idx, (ty, (is_option, inner_ty)))| {
            let field_name =
                syn::Ident::new(&format!("field_{}", idx), proc_macro2::Span::call_site());
            if *is_option {
                // Optional field: parse if present, otherwise None
                quote! {
                    let #field_name: #ty = match __iter.next() {
                        ::core::option::Option::Some(__part) if !__part.trim().is_empty() => {
                            ::core::option::Option::Some(
                                <#inner_ty as ::vtenc::parse::TryFromAnsi>::try_from_ansi(
                                    __part.trim().as_bytes()
                                )?
                            )
                        }
                        _ => ::core::option::Option::None,
                    };
                }
            } else {
                // Required field: must be present
                quote! {
                    let #field_name = match __iter.next() {
                        ::core::option::Option::Some(__part) => {
                            <#ty as ::vtenc::parse::TryFromAnsi>::try_from_ansi(
                                __part.trim().as_bytes()
                            )?
                        }
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                ::vtenc::parse::ParseError::InvalidValue(
                                    ::std::format!("expected at least {} fields, got {}", #required_count, #idx)
                                )
                            );
                        }
                    };
                }
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

                // Create iterator over fields - no allocation
                let mut __iter = s.split(#delimiter_lit);

                // Parse each field in order
                #(#field_parsing)*

                // Check for too many fields
                if __iter.next().is_some() {
                    return ::core::result::Result::Err(
                        ::vtenc::parse::ParseError::InvalidValue(
                            ::std::format!("expected at most {} fields", #field_count)
                        )
                    );
                }

                ::core::result::Result::Ok(Self(
                    #(#field_construction),*
                ))
            }
        }
    }
}
