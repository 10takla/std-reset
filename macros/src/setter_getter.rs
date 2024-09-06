use paste::paste;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, ItemStruct, PathSegment,
    Type,
};
use macro_functions::fast_impl;

pub fn expand(
    input: TokenStream,
    attr_prefix: &str,
    impl_: impl Fn(&Type, &Option<Ident>, Ident) -> proc_macro2::TokenStream,
) -> TokenStream {
    let struct_ = &parse_macro_input!(input);
    let ItemStruct { fields, ident, generics, .. } = struct_;
    let Fields::Named(FieldsNamed { named: fields, .. }) = fields else {
        panic!("Only works on structs with named fields");
    };

    let mut is_acc_default = true;
    let mut acc_every = vec![];

    let mut is_glob_exclude = false;
    let mut is_glob_include = false;

    fields.iter().for_each(
        |Field {
             ident, ty, attrs, ..
         }| {
            let tmp = || {
                let r = format!("{}_{{}}", attr_prefix);
                let setter_ident = format_ident!("{}_{}", attr_prefix, ident.as_ref().unwrap());
                impl_(ty, ident, setter_ident)
            };
            let [mut is_include, mut is_exclude] = [false; 2];
            attrs.into_iter().for_each(|syn::Attribute { meta, .. }| {
                if (meta.path().is_ident(&format!("include_{attr_prefix}ter"))) {
                    is_include = true
                }
                if (meta.path().is_ident(&format!("exclude_{attr_prefix}ter"))) {
                    is_exclude = true
                }
            });
            match [is_include, is_exclude, is_glob_include, is_glob_exclude] {
                [true, true, _, _]
                | [_, _, true, true]
                | [true, _, _, true]
                | [_, true, true, _] => {
                    panic!("Поле может быть только exclude или include")
                }
                [true, false, _, _] => {
                    is_glob_include = true;

                    if is_acc_default {
                        acc_every = vec![];
                        is_acc_default = false;
                    }

                    acc_every.push(tmp())
                }
                [false, true, _, _] => {
                    is_glob_exclude = true;
                }
                [false, false, _, _] => {
                    if is_acc_default {
                        acc_every.push(tmp())
                    }
                }
            }
        },
    );

    let methods = acc_every.into_iter();

    fast_impl(struct_, quote!(#(#methods)*)).into()
}

pub fn expand_setter(input: TokenStream) -> TokenStream {
    expand(input, "set", |ty, ident, func_ident| {
        quote! {
            pub fn #func_ident(&mut self, value: #ty) -> Self {
                self.#ident = value;
                (*self).clone()
            }
        }
    })
}

pub fn expand_getter(input: TokenStream) -> TokenStream {
    expand(input, "get", |ty, ident, func_ident| {
        quote! {
            pub fn #func_ident(&self) -> #ty {
                self.#ident
            }
        }
    })
}
