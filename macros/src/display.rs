use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

use macro_functions::fast_impl;

pub fn expand(item: TokenStream) -> TokenStream {
    let struct_: ItemStruct = parse_macro_input!(item);

    let ItemStruct {
        generics, ident, ..
    } = struct_;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics std::fmt::Display for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(self, f)
            }
        }
    }
    .into()
}
