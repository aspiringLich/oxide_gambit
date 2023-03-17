#![feature(proc_macro_diagnostic)]
#![feature(default_free_fn)]

use std::default::default;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse,
    punctuated::Punctuated,
    token::{Async, Comma, Const, FatArrow, Unsafe},
    Attribute, FnArg, Generics, Ident, ReturnType, Signature, Token, Visibility,
};

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
        let constness: Option<Const> = input.parse()?;
        let asyncness: Option<Async> = input.parse()?;
        let unsafety: Option<Unsafe> = input.parse()?;
        let _: Token![fn] = input.parse()?;
        let ident: Ident = input.parse()?;

        let ident_str = ident.to_string();
        let mut field = ident_str.split_once("_").map(|s| s.1).unwrap_or(&ident_str);
        if field != "with" || field != "set" {
            field = &ident_str;
        }
        let field: Ident = syn::parse_str(field)?;

        let generics: Generics = input.parse()?;
        let mut inputs: Punctuated<FnArg, Comma> = default();
        inputs.push(syn::parse_str("mut self")?);

        let content;
        let _ = syn::parenthesized!(content in input);

        while !content.is_empty() {
            // content.parse::<Ident>()?;
            if let Ok(arg) = content.parse::<FnArg>() {
                inputs.push(arg);
            } else {
                panic!("Expected argument!");
            }
        }

        // set the expr used to set the field
        let expr;
        // fn set_bar(bar: u32);
        if input.peek(Token![;]) {
            let arg = &inputs[1];
            assert!(inputs.len() == 2);

            match arg {
                FnArg::Typed(arg) => {
                    let ident = match &*arg.pat {
                        syn::Pat::Ident(ident) => &ident.ident,
                        _ => &field,
                    };
                    expr = syn::parse2(quote! { #ident })?;
                }
                _ => panic!("Expected typed arg, including `self` is unecessary!"),
            }
        } else {
            // fn set_bar(bar: u32) => bar + 1;
            input.parse::<FatArrow>()?;
            expr = input.parse::<syn::Expr>()?;
        }

        let signature = Signature {
            constness,
            asyncness,
            unsafety,
            abi: None,
            fn_token: default(),
            ident,
            generics,
            paren_token: default(),
            inputs,
            variadic: None,
            output: ReturnType::Default,
        };

        Ok(BuilderImpl {
            attrs,
            vis,
            signature,
            expr,
            field,
        })
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

#[proc_macro]
pub fn builder_impl(input: TokenStream) -> TokenStream {
    let builder_impls = syn::parse::<BuilderImpls>(input);
    let builder_impls = match builder_impls {
        Ok(builder_impls) => builder_impls,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };

    let mut out = proc_macro2::TokenStream::new();

    for builder_impl in builder_impls.impls {
        let BuilderImpl {
            attrs,
            vis,
            signature,
            expr,
            field,
        } = builder_impl;
        out.extend(quote! {
        #(#attrs)*
        #vis #signature -> Self {
            self.#field = #expr;
            self
        }});
    }

    out.into()
}
