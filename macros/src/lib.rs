mod shared;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Field, Fields, FieldsNamed, ItemStruct, Meta, MetaNameValue, PathSegment,
};
use shared::{get_segment_from_type, tmp};

/// реализация трейта Default с укаазнием значения по умолчанию
///
/// # Examples
///
/// ```
/// use macros::Default;
/// #[derive(Default)]
/// struct User {
///     #[default_field = 10]
///     age: u32,
/// }
/// ```

#[proc_macro_derive(Default, attributes(default_field))]
pub fn default_macro_derive(input: TokenStream) -> TokenStream {
    let ItemStruct { ident, fields, .. } = parse_macro_input!(input);

    let field_defaults = {
        let Fields::Named(FieldsNamed { named: fields, .. }) = fields else {
            panic!("Only works on structs with named fields");
        };

        fields.into_iter().map(|field| {
            let Field { ident, ty, .. } = field;

            let default_value = field
                .attrs
                .iter()
                .find_map(|attr| {
                    if attr.path.is_ident("default_field") {
                        match attr.parse_meta() {
                            Ok(Meta::NameValue(MetaNameValue { lit, .. })) => {
                                return Some(quote! { #lit });
                            }
                            _ => {}
                        }
                    }
                    None
                })
                .unwrap_or_else(|| {
                    let PathSegment {
                        ident, arguments, ..
                    } = get_segment_from_type(&ty);
                    if ident == "Option" {
                        let type_ = tmp(arguments);
                        return quote! { Some(<#type_ as std::default::Default>::default()) };
                    }
                    quote! { <#ty as std::default::Default>::default() }
                });

            quote! {
                #ident: #default_value
            }
        })
    };

    let expanded = quote! {
        impl std::default::Default for #ident {
            fn default() -> Self {
                Self {
                    #(#field_defaults),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
