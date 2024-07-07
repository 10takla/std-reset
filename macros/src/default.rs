use crate::shared::{get_segment_from_type, type_from_args};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, parse_str, Field, Fields, FieldsNamed, GenericParam, ItemStruct, Path,
    PathSegment, TraitBound, TypeParam, TypeParamBound,
};

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

    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let mut generics = generics.clone();

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
