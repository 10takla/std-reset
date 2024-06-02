use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, FieldsNamed, ItemStruct, FieldsUnnamed};

pub fn expand(input: TokenStream) -> TokenStream {
    let ItemStruct { fields, ident, .. } = parse_macro_input!(input);

    match fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            let args = named.iter();
            let struct_body = named.iter().map(|field| {
                field.ident.clone().unwrap()
            });
            quote! {
                impl #ident {
                    pub fn new(#(#args),*) -> Self {
                        Self {
                            #(#struct_body),*
                        }
                    }
                }
            }
            .into()
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let arg = unnamed.iter();
            let q = unnamed.iter().enumerate().map(|(i, _)| {
                quote!{
                    value.#i
                }
            });
            quote! {
                impl #ident {
                    pub fn new(value: (#(#arg),*)) -> Self {
                        Self(#(#q),*)
                    }
                }
            }
            .into()
        }
        _ => {
            panic!()
        }
    }
}
