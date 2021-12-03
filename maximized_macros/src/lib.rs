#![feature(proc_macro_quote)]

use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::iter;
use syn::DeriveInput;

#[proc_macro_derive(Maximized)]
pub fn maximized(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    maximized_inner(&ast)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn maximized_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

    let (maximized_body, nbytes_body) = match &ast.data {
        syn::Data::Struct(v) => (
            fill_fields(quote! {#name}, &v.fields),
            sum_fields(&v.fields),
        ),
        syn::Data::Enum(v) => {
            let mbodies = v.variants.iter().map(|variant| {
                let vident = &variant.ident;
                fill_fields(quote! {#name::#vident}, &variant.fields)
            });
            let nbytess = v.variants.iter().map(|variant| sum_fields(&variant.fields));
            (
                quote! {maximized::maximum(vec![#(#mbodies),*])},
                quote! {1 + vec![#(#nbytess),*].into_iter().max().unwrap()},
            )
        }
        syn::Data::Union(_) => {
            return Err(syn::Error::new(
                Span::call_site(),
                "This macro doesn't support unions.",
            ))
        }
    };
    Ok(quote! {
        impl #impl_generics ::maximized::Maximized for #name #ty_generics #where_clause {
            fn maximized() -> Self {
                #maximized_body
            }
            fn compute_size() -> usize {
                #nbytes_body
            }
        }
    })
}

fn fill_fields(name: TokenStream, fields: &syn::Fields) -> TokenStream {
    match fields {
        syn::Fields::Named(fields) => {
            let params = fields.named.iter().map(|x| {
                let field_name = x.ident.as_ref().unwrap();
                quote! {#field_name: Maximized::maximized()}
            });
            quote! {#name{#(#params),*}}
        }
        syn::Fields::Unnamed(fields) => {
            let params = iter::repeat(quote! {Maximized::maximized()}).take(fields.unnamed.len());
            quote! {#name(#(#params),*)}
        }
        syn::Fields::Unit => quote! {#name},
    }
}

fn sum_fields(fields: &syn::Fields) -> TokenStream {
    if fields.len() == 0 {
        quote! {0}
    } else {
        let nbytess = fields.iter().map(|x| {
            let field_type = &x.ty;
            quote! {<#field_type as Maximized>::compute_size()}
        });
        quote! {#(#nbytess)+*}
    }
}
