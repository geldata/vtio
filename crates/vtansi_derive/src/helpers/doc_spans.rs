//! Span collection and IDE documentation import generation.
//!
//! This module provides utilities for collecting attribute keyword spans and
//! generating documentation anchors that enable rust-analyzer hover support.
//!
//! # How It Works
//!
//! When a user writes `#[vtansi(csi, finalbyte = 'H')]`, we want hovering over
//! `csi` or `finalbyte` to show documentation. This is achieved by:
//!
//! 1. Collecting the original identifier tokens from attributes during parsing
//! 2. Generating anchor references with the original spans:
//!    - For sequence types (csi, osc, etc.): `let _ = ::vtansi::derive::csi;`
//!    - For arguments: `let _ = ::vtansi::derive::CSI_ARGS.finalbyte;`
//!
//! Because rust-analyzer uses span mapping between the macro call site and
//! expansion, hovering the keyword in the attribute effectively hovers the
//! generated reference, showing the documentation.

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::DeriveInput;
use syn::spanned::Spanned;

use super::metadata::{
    ControlKeyword, DeriveInputExt, FieldExt, FieldMeta, TypeMeta, VariantExt,
    VariantMeta,
};

/// Information about a sequence type for doc generation.
#[derive(Clone, Copy)]
struct SeqTypeInfo {
    /// The function name (e.g., "csi")
    func_name: &'static str,
    /// The args constant name (e.g., "CSI_ARGS")
    args_const: &'static str,
}

impl SeqTypeInfo {
    const CSI: Self = Self {
        func_name: "csi",
        args_const: "CSI_ARGS",
    };
    const OSC: Self = Self {
        func_name: "osc",
        args_const: "OSC_ARGS",
    };
    const DCS: Self = Self {
        func_name: "dcs",
        args_const: "DCS_ARGS",
    };
    const ESC: Self = Self {
        func_name: "esc",
        args_const: "ESC_ARGS",
    };
    const ESCST: Self = Self {
        func_name: "escst",
        args_const: "ESCST_ARGS",
    };
    const SS3: Self = Self {
        func_name: "ss3",
        args_const: "SS3_ARGS",
    };
    const C0: Self = Self {
        func_name: "c0",
        args_const: "C0_ARGS",
    };
    const BYTE: Self = Self {
        func_name: "byte",
        args_const: "BYTE_ARGS",
    };
}

/// A doc entry representing either a sequence type or a standalone const.
enum DocEntry {
    /// The `vtansi` attribute name itself: `let _ = ::vtansi::derive::vtansi;`
    AttrName { span: Span },
    /// A sequence type function reference: `let _ = ::vtansi::derive::csi;`
    SeqType { span: Span, info: SeqTypeInfo },
    /// A sequence argument field access: `let _ = ::vtansi::derive::CSI_ARGS.finalbyte;`
    SeqArg {
        span: Span,
        args_const: &'static str,
        field_name: &'static str,
    },
    /// A standalone const reference: `let _ = ::vtansi::derive::finalbyte;`
    Const {
        span: Span,
        const_name: &'static str,
    },
}

/// Collected spans from parsed attributes for documentation generation.
#[derive(Default)]
pub struct DocSpans {
    /// The current sequence type context (for argument field access).
    current_seq: Option<SeqTypeInfo>,
    /// Collected doc entries.
    entries: Vec<DocEntry>,
}

impl DocSpans {
    /// Create a new empty span collector.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current sequence type context.
    fn set_seq_type(&mut self, info: SeqTypeInfo) {
        self.current_seq = Some(info);
    }

    /// Add the `vtansi` attribute name reference.
    fn add_attr_name(&mut self, span: Span) {
        self.entries.push(DocEntry::AttrName { span });
    }

    /// Add a sequence type function reference.
    fn add_seq_type(&mut self, span: Span, info: SeqTypeInfo) {
        self.set_seq_type(info);
        self.entries.push(DocEntry::SeqType { span, info });
    }

    /// Add an argument that belongs to the current sequence type.
    fn add_seq_arg(&mut self, span: Span, field_name: &'static str) {
        if let Some(info) = self.current_seq {
            self.entries.push(DocEntry::SeqArg {
                span,
                args_const: info.args_const,
                field_name,
            });
        } else {
            // Fall back to standalone const if no sequence context
            self.add_const(span, field_name);
        }
    }

    /// Add a standalone const reference.
    fn add_const(&mut self, span: Span, const_name: &'static str) {
        self.entries.push(DocEntry::Const { span, const_name });
    }

    /// Collect `vtansi` attribute path spans from a list of attributes.
    fn collect_attr_name_spans(&mut self, attrs: &[syn::Attribute]) {
        for attr in attrs {
            if attr.path().is_ident("vtansi") {
                // Get the span of the `vtansi` identifier in the attribute path
                if let Some(ident) = attr.path().get_ident() {
                    self.add_attr_name(ident.span());
                }
            }
        }
    }

    /// Collect all documentation spans from a derive input.
    pub fn collect_from_derive_input(&mut self, input: &DeriveInput) {
        // Collect vtansi attribute name spans from type-level attributes
        self.collect_attr_name_spans(&input.attrs);

        // Collect type-level attribute spans
        if let Ok(metas) = input.get_type_metadata() {
            for meta in metas {
                self.collect_from_type_meta(&meta);
            }
        }

        // Collect field/variant spans based on data type
        match &input.data {
            syn::Data::Struct(data) => {
                self.collect_from_fields(&data.fields);
            }
            syn::Data::Enum(data) => {
                for variant in &data.variants {
                    // Collect vtansi attribute name spans from variant attributes
                    self.collect_attr_name_spans(&variant.attrs);
                    // Collect variant-level spans
                    if let Ok(metas) = variant.get_metadata() {
                        for meta in metas {
                            self.collect_from_variant_meta(&meta);
                        }
                    }
                    // Collect field spans within variant
                    self.collect_from_fields(&variant.fields);
                }
            }
            syn::Data::Union(data) => {
                for field in &data.fields.named {
                    self.collect_from_field(field);
                }
            }
        }
    }

    /// Collect spans from struct/variant fields.
    fn collect_from_fields(&mut self, fields: &syn::Fields) {
        match fields {
            syn::Fields::Named(named) => {
                for field in &named.named {
                    self.collect_from_field(field);
                }
            }
            syn::Fields::Unnamed(unnamed) => {
                for field in &unnamed.unnamed {
                    self.collect_from_field(field);
                }
            }
            syn::Fields::Unit => {}
        }
    }

    /// Collect spans from a single field.
    fn collect_from_field(&mut self, field: &syn::Field) {
        // Collect vtansi attribute name spans from field attributes
        self.collect_attr_name_spans(&field.attrs);

        if let Ok(metas) = field.get_metadata() {
            for meta in metas {
                self.collect_from_field_meta(&meta);
            }
        }
    }

    /// Collect spans from a type-level metadata item.
    fn collect_from_type_meta(&mut self, meta: &TypeMeta) {
        match meta {
            TypeMeta::Kind { kw, .. } => {
                // Sequence type - use function reference
                let (span, info) = match kw {
                    ControlKeyword::Byte(k) => (k.span(), SeqTypeInfo::BYTE),
                    ControlKeyword::C0(k) => (k.span(), SeqTypeInfo::C0),
                    ControlKeyword::Csi(k) => (k.span(), SeqTypeInfo::CSI),
                    ControlKeyword::Dcs(k) => (k.span(), SeqTypeInfo::DCS),
                    ControlKeyword::Esc(k) => (k.span(), SeqTypeInfo::ESC),
                    ControlKeyword::EscSt(k) => (k.span(), SeqTypeInfo::ESCST),
                    ControlKeyword::Osc(k) => (k.span(), SeqTypeInfo::OSC),
                    ControlKeyword::Ss3(k) => (k.span(), SeqTypeInfo::SS3),
                };
                self.add_seq_type(span, info);
            }
            // Arguments that can be part of a sequence
            TypeMeta::FinalByte { kw, .. } => {
                self.add_seq_arg(kw.span(), "finalbyte");
            }
            TypeMeta::Private { kw, .. } => {
                self.add_seq_arg(kw.span(), "private");
            }
            TypeMeta::Params { kw, .. } => {
                self.add_seq_arg(kw.span(), "params");
            }
            TypeMeta::Intermediate { kw, .. } => {
                self.add_seq_arg(kw.span(), "intermediate");
            }
            TypeMeta::Data { kw, .. } => {
                self.add_seq_arg(kw.span(), "data");
            }
            TypeMeta::Number { kw, .. } => {
                self.add_seq_arg(kw.span(), "number");
            }
            TypeMeta::DataDelimiter { kw, .. } => {
                self.add_seq_arg(kw.span(), "data_delimiter");
            }
            TypeMeta::Code { kw, .. } => {
                self.add_seq_arg(kw.span(), "code");
            }
            // Standalone attributes (not sequence-specific)
            TypeMeta::Format { kw, .. } => {
                self.add_const(kw.span(), "format");
            }
            TypeMeta::Delimiter { kw, .. } => {
                self.add_const(kw.span(), "delimiter");
            }
            TypeMeta::Transparent { kw } => {
                self.add_const(kw.span(), "transparent");
            }
            TypeMeta::EncodedLen { kw, .. } => {
                self.add_const(kw.span(), "encoded_len");
            }
            TypeMeta::FieldLocation { kw, .. } => {
                self.add_const(kw.span(), "locate_all");
            }
            TypeMeta::Into { kw, .. } => {
                self.add_const(kw.span(), "into");
            }
            TypeMeta::AliasOf { kw, .. } => {
                self.add_const(kw.span(), "alias_of");
            }
        }
    }

    /// Collect spans from a field-level metadata item.
    fn collect_from_field_meta(&mut self, meta: &FieldMeta) {
        match meta {
            FieldMeta::Skip { kw } => {
                self.add_const(kw.span(), "skip");
            }
            FieldMeta::Mux { kw, .. } => {
                self.add_const(kw.span(), "muxwith");
            }
            FieldMeta::Location { kw, .. } => {
                self.add_const(kw.span(), "locate");
            }
            FieldMeta::Flatten { kw } => {
                self.add_const(kw.span(), "flatten");
            }
        }
    }

    /// Collect spans from a variant-level metadata item.
    fn collect_from_variant_meta(&mut self, meta: &VariantMeta) {
        match meta {
            VariantMeta::Default { kw } => {
                self.add_const(kw.span(), "default");
            }
        }
    }

    /// Generate the documentation anchor references as a TokenStream.
    #[must_use]
    pub fn generate_imports(&self) -> TokenStream {
        let refs = self.entries.iter().map(|entry| {
            match entry {
                DocEntry::AttrName { span } => {
                    // Generate: let _ = ::vtansi::derive::vtansi;
                    let vtansi_ident = Ident::new("vtansi", *span);
                    quote! {
                        let _ = ::vtansi::derive::#vtansi_ident;
                    }
                }
                DocEntry::SeqType { span, info } => {
                    // Generate: let _ = ::vtansi::derive::csi;
                    let func_ident = Ident::new(info.func_name, *span);
                    quote! {
                        let _ = ::vtansi::derive::#func_ident;
                    }
                }
                DocEntry::SeqArg {
                    span,
                    args_const,
                    field_name,
                } => {
                    // Generate: let _ = ::vtansi::derive::CSI_ARGS.finalbyte;
                    let args_ident = Ident::new(args_const, Span::call_site());
                    let field_ident = Ident::new(field_name, *span);
                    quote! {
                        let _ = ::vtansi::derive::#args_ident.#field_ident;
                    }
                }
                DocEntry::Const { span, const_name } => {
                    // Generate: let _ = ::vtansi::derive::finalbyte;
                    let const_ident = Ident::new(const_name, *span);
                    quote! {
                        let _ = ::vtansi::derive::#const_ident;
                    }
                }
            }
        });

        quote! {
            const _: () = {
                #(#refs)*
            };
        }
    }

    /// Check if any spans have been collected.
    #[must_use]
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get the number of collected spans.
    #[must_use]
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

/// Collect documentation spans from a derive input and generate imports.
///
/// This is a convenience function that creates a `DocSpans` collector,
/// collects all spans from the input, and generates the import tokens.
#[must_use]
pub fn generate_doc_imports(input: &DeriveInput) -> TokenStream {
    let mut spans = DocSpans::new();
    spans.collect_from_derive_input(input);
    spans.generate_imports()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_spans_empty() {
        let spans = DocSpans::new();
        assert!(spans.is_empty());
        assert_eq!(spans.len(), 0);
    }

    #[test]
    fn test_doc_spans_add_const() {
        let mut spans = DocSpans::new();
        spans.add_const(Span::call_site(), "csi");
        spans.add_const(Span::call_site(), "finalbyte");
        assert!(!spans.is_empty());
        assert_eq!(spans.len(), 2);
    }

    #[test]
    fn test_generate_imports() {
        let mut spans = DocSpans::new();
        spans.add_const(Span::call_site(), "csi");
        let imports = spans.generate_imports();
        // Just verify it produces some tokens
        assert!(!imports.is_empty());
    }
}
