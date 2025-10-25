//! Implementation of `ToAnsi` for structs.
//!
//! This module generates implementations of the `ToAnsi` trait for structs
//! with named or unnamed (tuple) fields. It supports two encoding formats:
//!
//! 1. **Key-Value format** (default for named fields) - fields are encoded as
//!    `key=value` pairs separated by a delimiter. Optional fields with
//!    `Option<T>` type are omitted if `None`.
//! 2. **Value format** (default for tuple structs) - fields are encoded as
//!    values in declaration order, separated by a delimiter. Supports optional
//!    trailing fields with `Option<T>` type. Trailing `None` values are
//!    omitted from the output.
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

/// Generate the implementation of `ToAnsi` for a struct.
///
/// This function generates encoding code based on the struct's format
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
            "ToAnsi cannot be derived for unit structs",
        )),
    }
}

/// Generate implementation for structs with key=value format.
///
/// This function creates a `ToAnsi` implementation that encodes the struct
/// as `key=value` pairs separated by the delimiter.
fn generate_keyvalue_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    field_names: &[syn::Ident],
    delimiter: &str,
) -> TokenStream {
    let field_pairs = field_names.iter().map(|name| {
        let name_str = name.to_string();
        quote! {
            (
                #name_str,
                <_ as ::vtenc::encode::ToAnsi>::to_ansi(&self.#name).to_string()
            )
        }
    });

    let delimiter_lit = syn::LitStr::new(delimiter, proc_macro2::Span::call_site());

    quote! {
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl #impl_generics ::vtenc::encode::ToAnsi for #name #ty_generics #where_clause {
            #[inline]
            fn to_ansi(&self) -> impl ::vtenc::encode::AnsiEncode {
                let pairs: ::std::vec::Vec<(&str, ::std::string::String)> = ::std::vec![
                    #(#field_pairs),*
                ];
                ::vtenc::encode_keyvalue_pairs(&pairs, #delimiter_lit)
            }
        }
    }
}

/// Generate implementation for tuple structs.
///
/// This function creates a `ToAnsi` implementation that encodes the tuple
/// struct as values in field order, separated by the delimiter. Supports
/// optional trailing fields (fields with `Option<T>` type). Trailing `None`
/// values are omitted from the output.
fn generate_tuple_value_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    field_types: &[&syn::Type],
    delimiter: &str,
) -> TokenStream {
    let field_count = field_types.len();

    // Detect which fields are Option types
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

    let field_encodings = (0..field_count).map(|idx| {
        let field_idx = syn::Index::from(idx);
        let (is_option, _) = &field_info[idx];
        
        if *is_option {
            // For Option fields, check if Some or None
            quote! {
                match &self.#field_idx {
                    ::core::option::Option::Some(value) => {
                        ::core::option::Option::Some(
                            ::std::string::String::from_utf8(
                                <_ as ::vtenc::encode::AnsiEncode>::encode_ansi(value).unwrap()
                            ).unwrap()
                        )
                    },
                    ::core::option::Option::None => ::core::option::Option::None,
                }
            }
        } else {
            // For non-Option fields, always wrap in Some
            quote! {
                ::core::option::Option::Some(
                    ::std::string::String::from_utf8(
                        <_ as ::vtenc::encode::AnsiEncode>::encode_ansi(&self.#field_idx).unwrap()
                    ).unwrap()
                )
            }
        }
    });

    let delimiter_lit = syn::LitStr::new(delimiter, proc_macro2::Span::call_site());

    quote! {
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl #impl_generics ::vtenc::encode::ToAnsi for #name #ty_generics #where_clause {
            #[inline]
            fn to_ansi(&self) -> impl ::vtenc::encode::AnsiEncode {
                let parts: ::std::vec::Vec<::core::option::Option<::std::string::String>> = ::std::vec![
                    #(#field_encodings),*
                ];
                ::vtenc::encode_delimited_values_with_optional(&parts, #delimiter_lit)
            }
        }
    }
}

/// Generate implementation for named structs with value format.
///
/// This function creates a `ToAnsi` implementation that encodes the struct
/// as values in field declaration order, separated by the delimiter. Supports
/// optional trailing fields (fields with `Option<T>` type). Trailing `None`
/// values are omitted from the output.
fn generate_named_value_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    field_names: &[syn::Ident],
    field_types: &[&syn::Type],
    delimiter: &str,
) -> TokenStream {
    // Detect which fields are Option types
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

    let field_encodings = field_names.iter().zip(field_info.iter()).map(|(name, (is_option, _))| {
        if *is_option {
            // For Option fields, check if Some or None
            quote! {
                match &self.#name {
                    ::core::option::Option::Some(value) => {
                        ::core::option::Option::Some(
                            ::std::string::String::from_utf8(
                                <_ as ::vtenc::encode::AnsiEncode>::encode_ansi(value).unwrap()
                            ).unwrap()
                        )
                    },
                    ::core::option::Option::None => ::core::option::Option::None,
                }
            }
        } else {
            // For non-Option fields, always wrap in Some
            quote! {
                ::core::option::Option::Some(
                    ::std::string::String::from_utf8(
                        <_ as ::vtenc::encode::AnsiEncode>::encode_ansi(&self.#name).unwrap()
                    ).unwrap()
                )
            }
        }
    });

    let delimiter_lit = syn::LitStr::new(delimiter, proc_macro2::Span::call_site());

    quote! {
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl #impl_generics ::vtenc::encode::ToAnsi for #name #ty_generics #where_clause {
            #[inline]
            fn to_ansi(&self) -> impl ::vtenc::encode::AnsiEncode {
                let parts: ::std::vec::Vec<::core::option::Option<::std::string::String>> = ::std::vec![
                    #(#field_encodings),*
                ];
                ::vtenc::encode_delimited_values_with_optional(&parts, #delimiter_lit)
            }
        }
    }
}
