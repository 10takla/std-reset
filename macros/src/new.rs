use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, FieldsNamed, FieldsUnnamed, ItemStruct};

use crate::shared::fast_impl;

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
            let arg = unnamed.iter();
            let body = if unnamed.len() > 1 {
                let q = unnamed.iter().enumerate().map(|(i, _)| {
                    quote! {
                        value.#i
                    }
                });
                quote! {
                    #(#q),*

                }
            } else {
                quote! {
                    value
                }
            };
            quote! {
                pub fn new(value: (#(#arg),*)) -> Self {
                    Self(#body)
                }
            }
        }
        _ => {
            panic!()
        }
    };
    dbg!(new.to_string());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    fast_impl(struct_, new).into()
}
