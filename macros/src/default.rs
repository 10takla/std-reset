use macro_functions::{get_segment_from_type, type_from_args};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse, parse2, parse_macro_input, parse_str, Field, Fields, FieldsNamed, GenericParam,
    Generics, ItemEnum, ItemStruct, Meta, Path, PathSegment, TraitBound, TypeParam, TypeParamBound,
};

pub fn expand(input: TokenStream) -> TokenStream {
    let get_default_value = |field: &Field| {
        let Field { ty, .. } = field;
        field
            .attrs
            .iter()
            .find_map(|attr| {
                attr.path().is_ident("default").then(|| {
                    let Meta::List(list) = &attr.meta else {
                        unreachable!()
                    };
                    list.tokens.clone()
                })
            })
            .unwrap_or_else(|| {
                quote! { <#ty as std::default::Default>::default() }
            })
    };

    match parse_macro_input!(input as syn::Item) {
        syn::Item::Struct(ItemStruct {
            ident,
            fields,
            generics,
            ..
        }) => {
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
                Fields::Unit => proc_macro2::TokenStream::default(),
            };

            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

            let mut generics = parse2::<Generics>(impl_generics.to_token_stream()).unwrap();

            generics.params.iter_mut().for_each(|type_| {
                if let GenericParam::Type(type_param) = type_ {
                    let TypeParam { bounds, .. } = type_param;

                    if !bounds.iter().any(|trait_| {
                        if let TypeParamBound::Trait(TraitBound { path, .. }) = trait_ {
                            if path.get_ident().unwrap() == "Default" {
                                return true;
                            }
                        }
                        false
                    }) {
                        bounds.push(parse_str::<TypeParamBound>("Default").unwrap())
                    }
                }
            });
            
            quote! {
                impl #generics std::default::Default for #ident #ty_generics #where_clause {
                    fn default() -> Self {
                        Self #body
                    }
                }
            }
            .into()
        }
        syn::Item::Enum(ItemEnum {
            variants,
            generics,
            ident,
            ..
        }) => {
            let default_variant = variants
                .iter()
                .find(|v| v.attrs.iter().any(|attr| attr.path().is_ident("default")))
                .expect("enumeration must have the attribute #[default]");
            let variant_ident = &default_variant.ident;
            let body = match &default_variant.fields {
                Fields::Named(fields_named) => {
                    let field_defaults = fields_named.named.iter().map(|field| {
                        let ident = &field.ident;
                        let default_value = get_default_value(field);
                        quote! {
                            #ident: #default_value
                        }
                    });
                    quote! {
                        { #(#field_defaults),* }
                    }
                }
                Fields::Unnamed(fields_unnamed) => {
                    let field_defaults = fields_unnamed
                        .unnamed
                        .iter()
                        .map(|field| get_default_value(field));
                    quote! {
                        ( #(#field_defaults),* )
                    }
                }
                Fields::Unit => proc_macro2::TokenStream::default(),
            };
            let (_, ty_generics, where_clause) = generics.split_for_impl();
            let mut generics = generics.clone();

            generics.params.iter_mut().for_each(|type_| {
                if let GenericParam::Type(type_param) = type_ {
                    let TypeParam { bounds, .. } = type_param;
                    if !bounds.iter().any(|trait_| {
                        if let TypeParamBound::Trait(TraitBound { path, .. }) = trait_ {
                            path.is_ident("Default")
                        } else {
                            false
                        }
                    }) {
                        bounds.push(parse_str::<TypeParamBound>("Default").unwrap())
                    }
                }
            });

            quote! {
                impl #generics std::default::Default for #ident #ty_generics #where_clause {
                    fn default() -> Self {
                        Self::#variant_ident #body
                    }
                }
            }
            .into()
        }
        _ => panic!("Этот макрос поддерживает только структуры и перечисления"),
    }
}
