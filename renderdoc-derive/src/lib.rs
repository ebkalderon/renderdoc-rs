//! Codegen crate for `renderdoc-rs` which generates trait implementation boilerplate.

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use syn::{Attribute, DeriveInput, Meta, MetaList, NestedMeta};

/// Generates API boilerplate for the `renderdoc` crate.
///
/// # Details
///
/// This macro expects a tuple struct of the form:
///
/// ```rust,ignore
/// use renderdoc::ApiVersion;
///
/// struct Foo<T: ApiVersion>(T::Entry);
/// ```
///
/// Given the data structure above, this macro generates the following implementations:
///
/// * `From` conversions downgrading a newer API to a compatible older API.
/// * Implementations of the `RenderDocV###` trait for each `Foo<V###>`.
#[proc_macro_derive(RenderDoc, attributes(renderdoc_convert))]
pub fn renderdoc(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_renderdoc(&ast)
}

/// Generates RenderDoc API implementations locked by versions through traits.
fn impl_renderdoc(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let apis = build_api_list(&ast.attrs);

    let from_impls = gen_from_impls(&name, &apis, TokenStream2::new());
    let rd_impls = gen_renderdoc_impls(&name, &apis, TokenStream2::new());

    let expanded = quote! {
        #from_impls
        #rd_impls
    };

    expanded.into()
}

/// Reads the `renderdoc_convert` list attribute and returns a list of API versions to implement.
///
/// Each API version in this list is a unique identifier of the form `V100`, `V110`, `V120`, etc.
fn build_api_list(attrs: &[Attribute]) -> Vec<Ident> {
    let meta = attrs
        .iter()
        .flat_map(|attr| attr.interpret_meta())
        .find(|meta| meta.name() == "renderdoc_convert")
        .expect("Missing required attribute `#[renderdoc_convert(...)]`");

    let mut apis: Vec<Ident> = match meta {
        Meta::List(MetaList { ref nested, .. }) => nested
            .iter()
            .cloned()
            .flat_map(|elem| match elem {
                NestedMeta::Meta(Meta::Word(ident)) => Some(ident),
                _ => None,
            }).collect(),
        _ => panic!("Expected list attribute `#[renderdoc_convert(...)]`"),
    };

    apis.sort();
    apis
}

/// Generates `From` implementations that permit downgrading of API versions.
///
/// This function takes a list of API versions sorted in ascending order and recursively generates
/// `From` implementations for them. For instance, given the following three API versions
/// `[V100, V110, V200]`, these trait implementations will be generated:
///
/// ```rust,ignore
/// // V200 -> V110, V100
///
/// impl From<#name<V200>> for #name<V110> {
///     fn from(newer: #name<V200>) -> Self {
///         // ...
///     }
/// }
///
/// impl From<#name<V200>> for #name<V100> {
///     fn from(newer: #name<V200>) -> Self {
///         // ...
///     }
/// }
///
/// // V110 -> V100
///
/// impl From<#name<V110>> for #name<V100> {
///     fn from(newer: #name<V200>) -> Self {
///         // ...
///     }
/// }
///
/// // V100 -> ()
/// ```
fn gen_from_impls(name: &Ident, apis: &[Ident], tokens: TokenStream2) -> TokenStream2 {
    if apis.len() <= 1 {
        return tokens;
    }

    let last_idx = apis.len() - 1;
    let newer = &apis[last_idx];
    let impls: TokenStream2 = apis[0..last_idx]
        .iter()
        .map(|older| {
            quote! {
                impl From<#name<#newer>> for #name<#older> {
                    fn from(newer: #name<#newer>) -> Self {
                        #name(newer.0.into())
                    }
                }
            }
        }).collect();

    gen_from_impls(
        name,
        &apis[0..last_idx],
        tokens.into_iter().chain(impls).collect(),
    )
}

/// Generates `RenderDocV###` implementations for statically typing the RenderDoc API.
///
/// This function takes a list of API versions sorted in ascending order and recursively generates
/// `RenderDocV###` implementations for them. For instance, given the following three API versions
/// `[V100, V110, V200]`, these trait implementations will be generated:
///
/// ```rust,ignore
/// // V200 -> Trait methods from V200 down.
///
/// impl RenderDocV200 for #name<V200> {
///     // ...
/// }
///
/// impl RenderDocV110 for #name<V200> {
///     // ...
/// }
///
/// impl RenderDocV100 for #name<V200> {
///     // ...
/// }
///
/// // V110 -> Trait methods from V110 down.
///
/// impl RenderDocV110 for #name<V110> {
///     // ...
/// }
///
/// impl RenderDocV100 for #name<V110> {
///     // ...
/// }
///
/// // V100 -> Trait methods from V100 down.
///
/// impl RenderDocV100 for #name<V100> {
///     // ...
/// }
/// ```
fn gen_renderdoc_impls(name: &Ident, apis: &[Ident], tokens: TokenStream2) -> TokenStream2 {
    if apis.len() == 0 {
        return tokens;
    }

    let last_idx = apis.len() - 1;
    let version = &apis[last_idx];
    let impls: TokenStream2 = apis[0..=last_idx]
        .iter()
        .map(|api| {
            let span = Span::call_site();

            let trait_name = Ident::new(&format!("RenderDoc{}", api), span.clone());
            let method = Ident::new(
                &format!("entry_{}", api.to_string().to_lowercase()),
                span.clone(),
            );
            let ret_val = Ident::new(&format!("Entry{}", api), span.clone());

            quote! {
                impl ::api::#trait_name for #name<#version> {
                    unsafe fn #method(&self) -> &::entry::#ret_val {
                        &self.0
                    }
                }
            }
        }).collect();

    gen_renderdoc_impls(
        name,
        &apis[0..last_idx],
        tokens.into_iter().chain(impls).collect(),
    )
}
