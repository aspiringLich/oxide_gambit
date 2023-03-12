#![feature(proc_macro_diagnostic)]

use proc_macro::{ TokenStream, TokenTree, Delimiter };
use syn::{
    parse::Parse,
    Visibility,
    Ident,
    Field,
    Expr,
    Token,
    spanned::Spanned,
    parse_macro_input,
};

extern crate proc_macro;

struct BuilderImplInner {
    pub fields: Vec<Field>,
    pub set: Vec<Expr>,
}

impl Parse for BuilderImplInner {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        
    }
}

struct BuilderImpl {
    visibility: Visibility,
    pub name: Ident,
    pub fields: Vec<Field>,
    pub set: Vec<Expr>,
}

impl Parse for BuilderImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let visibility: Visibility = input.parse()?;
        input.parse::<Token![fn]>()?;
        let name = input.parse::<Ident>()?;
        let group = input.parse::<proc_macro2::Group>()?;

        let inner: BuilderImplInner = input.parse_terminated()?;
        Ok(Self {
            visibility,
            name,
            fields: inner.fields,
            set: inner.set,
        })
    }
}

fn error<T>(span: proc_macro2::Span, msg: &str) -> syn::Result<T> {
    Err(syn::Error::new(span, msg))
}

#[proc_macro_attribute]
pub fn builder_impl(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream
) -> proc_macro::TokenStream {
    let mut iter = item.into_iter();

    let impl_ = iter.next();
    let struct_ = iter.next();
    let group = iter.next();

    let error = |span: proc_macro::Span, msg| {
        span.error(msg).emit();
        proc_macro::TokenStream::new()
    };

    if let (Some(impl_), Some(struct_), Some(group)) = (impl_, struct_, group) {
        let impl_ = match impl_ {
            TokenTree::Ident(ident) => ident,
            _ => {
                return error(impl_.span(), "Expected impl keyword");
            }
        };
        let struct_ = match struct_ {
            TokenTree::Ident(ident) => ident,
            _ => {
                return error(struct_.span(), "Missing struct name after impl");
            }
        };
        let group = match group {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => group,
            _ => {
                return error(group.span(), "Missing brace delimited group");
            }
        };

        builder_impls = syn::parse(group.stream()).unwrap();
    } else {
        panic!("Input stream too short");
    }

    panic!()
}