use std::iter::{Enumerate, Map};

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Fields, FieldsNamed, FieldsUnnamed, ItemStruct};

use macro_functions::fast_impl;

pub fn expand(input: TokenStream) -> TokenStream {
    let struct_ = &parse_macro_input!(input);

    let ItemStruct {
        fields,
        ident,
        generics,
        ..
    } = struct_;

    let new = match fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            let args = named.iter();
            let struct_body = named.iter().map(|field| field.ident.clone().unwrap());
            quote! {
                pub fn new(#(#args),*) -> Self {
                    Self {
                        #(#struct_body),*
                    }
                }
            }
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let (arg, k): (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::Ident>) = unnamed.iter().enumerate().map(|(i, arg)| {
                let ident = format_ident!("arg_{}", i + 1);
                (
                    quote! {
                        #ident: #arg
                    },
                    ident,
                )
            }).unzip();

            let body = if unnamed.len() > 1 {
                quote! {
                    #(#k),*
                }
            } else {
                quote! {
                    arg_1
                }
            };

            quote! {
                pub fn new(#(#arg),*) -> Self {
                    Self(#body)
                }
            }
        }
        _ => {
            panic!()
        }
    };
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    fast_impl(struct_, new).into()
}
