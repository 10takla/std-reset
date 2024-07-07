use crate::shared::{get_segment_from_type, type_from_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Field, Fields, FieldsNamed, ItemStruct, PathSegment};

pub fn expand(input: TokenStream) -> TokenStream {
    let ItemStruct {
        ident,
        fields,
        generics,
        ..
    } = parse_macro_input!(input);
    let get_default_value = |field: &Field| {
        let Field { ty, .. } = field;
        field
            .attrs
            .iter()
            .find_map(|attr| {
                attr.path.is_ident("default").then(|| {
                    attr.parse_args::<syn::LitStr>()
                        .map(|default_value| {
                            default_value
                                .value()
                                .parse::<proc_macro2::TokenStream>()
                                .expect("Failed to parse tokens")
                        })
                        .expect("Failed to parse default value")
                })
            })
            .unwrap_or_else(|| {
                quote! { <#ty as std::default::Default>::default() }
            })
    };
    let body = match fields {
        Fields::Named(_) => {
            let field_defaults = fields.iter().map(|field| {
                let Field { ident, .. } = field;
                let default_value = get_default_value(field);
                quote! {
                    #ident: #default_value
                }
            });
            quote! {
                {
                    #(#field_defaults),*
                }
            }
        }
        Fields::Unnamed(_) => {
            let field_defaults = fields.iter().map(|field| get_default_value(field));
            quote! {
                (#(#field_defaults),*)
            }
        }
        _ => panic!("Struct must have named or unnamed fields"),
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics std::default::Default #ty_generics for #ident #where_clause {
            fn default() -> Self {
                Self #body
            }
        }
    }
    .into()
}
