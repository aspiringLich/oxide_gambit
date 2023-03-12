#![feature(proc_macro_diagnostic)]
#![feature(default_free_fn)]

use std::default::default;

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
    token::{ Const, Async, Unsafe, Comma, FatArrow },
    Abi,
    Generics,
    Variadic, Signature, ReturnType,
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
        let constness: Option<Const> = input.parse()?;
        let asyncness: Option<Async> = input.parse()?;
        let unsafety: Option<Unsafe> = input.parse()?;
        let _: Token![fn] = input.parse()?;
        let ident: Ident = input.parse()?;

        let ident_str = ident.to_string();
        let field = ident_str.rsplit("_").next().unwrap();
        let field: Ident = syn::parse_str(field)?;

        let generics: Generics = input.parse()?;
        let mut inputs: Punctuated<FnArg, Comma> = default();
        inputs.push(syn::parse_str("mut self")?);

        let content;
        let _ = syn::parenthesized!(content in input);

        while !content.is_empty() {
            if let Ok(arg) = content.parse::<FnArg>() {
                inputs.push(arg);
            } else if let Ok(ty) = content.parse::<syn::Type>() {
                inputs.push(
                    FnArg::Typed(syn::PatType {
                        attrs: vec![],
                        pat: Box::new(
                            syn::Pat::Ident(syn::PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: None,
                                ident: field.clone(),
                                subpat: None,
                            })
                        ),
                        colon_token: Default::default(),
                        ty: Box::new(ty),
                    })
                );
            } else {
                panic!("Expected argument or type");
            }
        }
        
        // set the expr used to set the field
        let expr;
        // fn set_bar(bar: u32);
        if input.parse::<Token![;]>().is_ok() {
            let arg = &inputs[1];
            assert!(inputs.len() == 2);

            match arg {
                FnArg::Typed(arg) => {
                    let ident = match &*arg.pat {
                        syn::Pat::Ident(ident) => &ident.ident,
                        _ => panic!("Expected identifier for the name of the argument"),
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
        
        // panic!("{:#?}", signature);
                
        Ok(BuilderImpl { attrs, vis, signature, expr, field })
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
            #vis #signature -> Self {
                self.#field = #expr;
                self
            }
        }
        );
    }

    // panic!("{:#?}", out);

    (quote! {
        impl #name {
            #out
        }
    }).into()
}