//! Implementation of `ToAnsi` for structs.
//!
//! This module generates implementations of the `ToAnsi` trait for structs
//! with named or unnamed (tuple) fields. It supports two encoding formats:
//!
//! 1. **Key-Value format** (default for named fields) - fields are encoded as
//!    `key=value` pairs separated by a delimiter.
//! 2. **Value format** (default for tuple structs) - fields are encoded as
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

            for field in &fields.named {
                let field_props = field.get_field_properties()?;
                if field_props.skip.is_some() {
                    continue;
                }

                let field_name = field.ident.as_ref().unwrap();
                field_names.push(field_name.clone());
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

            let field_count = fields.unnamed.len();

            let expanded = generate_tuple_value_impl(
                name,
                &impl_generics,
                &ty_generics,
                where_clause,
                field_count,
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
/// struct as values in field order, separated by the delimiter.
fn generate_tuple_value_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    field_count: usize,
    delimiter: &str,
) -> TokenStream {
    let field_encodings = (0..field_count).map(|idx| {
        let field_idx = syn::Index::from(idx);
        quote! {
            <_ as ::vtenc::encode::ToAnsi>::to_ansi(&self.#field_idx).to_string()
        }
    });

    let delimiter_lit = syn::LitStr::new(delimiter, proc_macro2::Span::call_site());

    quote! {
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl #impl_generics ::vtenc::encode::ToAnsi for #name #ty_generics #where_clause {
            #[inline]
            fn to_ansi(&self) -> impl ::vtenc::encode::AnsiEncode {
                let parts: ::std::vec::Vec<::std::string::String> = ::std::vec![
                    #(#field_encodings),*
                ];
                ::vtenc::encode_delimited_values(&parts, #delimiter_lit)
            }
        }
    }
}

/// Generate implementation for named structs with value format.
///
/// This function creates a `ToAnsi` implementation that encodes the struct
/// as values in field declaration order, separated by the delimiter.
fn generate_named_value_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    field_names: &[syn::Ident],
    delimiter: &str,
) -> TokenStream {
    let field_encodings = field_names.iter().map(|name| {
        quote! {
            <_ as ::vtenc::encode::ToAnsi>::to_ansi(&self.#name).to_string()
        }
    });

    let delimiter_lit = syn::LitStr::new(delimiter, proc_macro2::Span::call_site());

    quote! {
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl #impl_generics ::vtenc::encode::ToAnsi for #name #ty_generics #where_clause {
            #[inline]
            fn to_ansi(&self) -> impl ::vtenc::encode::AnsiEncode {
                let parts: ::std::vec::Vec<::std::string::String> = ::std::vec![
                    #(#field_encodings),*
                ];
                ::vtenc::encode_delimited_values(&parts, #delimiter_lit)
            }
        }
    }
}