#![feature(proc_macro_diagnostic)]

use proc_macro::{ TokenStream, TokenTree, Delimiter };
use proc_macro2::Group;
use syn::{
    parse::Parse,
    Visibility,
    Ident,
    Token,
    spanned::Spanned,
    parse_macro_input,
    FnArg,
    Receiver,
    punctuated::Punctuated,
    Attribute,
    Block,
};
use quote::quote;

extern crate proc_macro;

struct BuilderImpl {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub signature: syn::Signature,
    pub expr: syn::Expr,
    pub field: syn::Ident,
}

impl Parse for BuilderImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let mut signature = input.parse::<syn::Signature>()?;
        let ident = signature.ident.to_string();
        let field = ident.rsplit("_").next().unwrap();

        let expr;
        // fn set_bar(bar: u32);
        if input.parse::<Token![;]>().is_ok() {
            let arg = signature.inputs.first().expect("Expected at least one argument");
            assert!(signature.inputs.len() == 1);

            match arg {
                FnArg::Typed(arg) => {
                    let ident = match &*arg.pat {
                        syn::Pat::Ident(ident) => &ident.ident,
                        _ => panic!("Expected identifier for the name of the argument"),
                    };
                    expr = syn::parse2(quote! { self.#ident = #ident; })?;
                }
                _ => panic!("Expected typed arg, including `self` is unecessary!"),
            }
        } else {
            // fn set_bar(bar: u32) => bar + 1;
            input.parse::<Token!(=>)>()?;
            expr = input.parse::<syn::Expr>()?;
        }

        signature.inputs.insert(0, syn::parse_str("mut self")?);
        signature.output = syn::parse_str("Self")?;

        Ok(BuilderImpl { attrs, vis, signature, expr, field: syn::parse_str(field)? })
    }
}

// fn error<T>(span: proc_macro2::Span, msg: &str) -> syn::Result<T> {
//     Err(syn::Error::new(span, msg))
// }

struct Impl {
    struct_name: Ident,
    group: Group,
}

impl Parse for Impl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![impl]>()?;
        let struct_name = input.parse::<Ident>()?;
        let group = input.parse::<Group>()?;
        Ok(Impl { struct_name, group })
    }
}

struct BuilderImpls {
    impls: Punctuated<BuilderImpl, Token![;]>,
}

impl Parse for BuilderImpls {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let impls = input.parse_terminated(|p| p.parse::<BuilderImpl>())?;
        Ok(Self { impls })
    }
}

#[proc_macro_attribute]
pub fn builder_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let i = parse_macro_input!(item as Impl);
    let name = i.struct_name;
    let stream = i.group.stream();
    let builder_impls = syn::parse2::<BuilderImpls>(stream);
    let builder_impls = match builder_impls {
        Ok(builder_impls) => builder_impls,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };

    let mut out = proc_macro2::TokenStream::new();

    for builder_impl in builder_impls.impls {
        let BuilderImpl { attrs, vis, signature, expr, field } = builder_impl;
        out.extend(
            quote! {
            #(#attrs)*
            #vis #signature {
                self.#field = #expr
                self
            }
        }
        );
    }

    (quote! {
        impl #name {
            #out
        }
    }).into()
}