//! Code generation builders for escape sequences.
//!
//! This module provides builder types for constructing token streams for
//! escape sequence implementations. These builders encapsulate common
//! patterns in code generation and provide a fluent API for building
//! complex output.

use proc_macro2::TokenStream;
use quote::quote;

/// Builder for generating write operation token streams.
///
/// Consolidates repetitive write operation generation patterns for
/// encoding escape sequences at runtime.
pub struct WriteOperationBuilder {
    operations: Vec<TokenStream>,
    sink: syn::Ident,
    counter: syn::Ident,
}

impl WriteOperationBuilder {
    /// Create a new write operation builder.
    pub fn new(sink: &syn::Ident, counter: &syn::Ident) -> Self {
        Self {
            operations: Vec::new(),
            sink: sink.clone(),
            counter: counter.clone(),
        }
    }

    /// Add a byte slice write operation.
    pub fn write_bytes(&mut self, s: &[u8]) -> &mut Self {
        let counter = &self.counter;
        let sink = &self.sink;
        let s = syn::LitByteStr::new(s, proc_macro2::Span::mixed_site());
        self.operations.push(quote! {
            #counter += ::vtansi::encode::write_bytes_into(#sink, #s)?;
        });
        self
    }

    /// Add a byte write operation.
    pub fn write_byte(&mut self, ch: u8) -> &mut Self {
        let counter = &self.counter;
        let sink = &self.sink;
        self.operations.push(quote! {
            #counter += ::vtansi::encode::write_byte_into(#sink, #ch)?;
        });
        self
    }

    pub fn write_byte_expr(
        &mut self,
        ch: proc_macro2::TokenStream,
    ) -> &mut Self {
        let counter = &self.counter;
        let sink = &self.sink;
        self.operations.push(quote! {
            #counter += ::vtansi::encode::write_byte_into(#sink, #ch)?;
        });
        self
    }

    /// Build the final token stream.
    pub fn build(self) -> Vec<TokenStream> {
        self.operations
    }
}
