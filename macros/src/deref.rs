use crate::shared::{get_segment_from_type, tmp};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Field, Fields, FieldsNamed, FieldsUnnamed, ItemStruct, PathSegment};

pub fn expand(input: TokenStream) -> TokenStream {
    let ItemStruct {
        fields,
        ident,
        generics,
        ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let (field, pos) = match fields {
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let pos = if unnamed.len() > 1 {
                let deref_fields = unnamed
                    .iter()
                    .enumerate()
                    .filter(|(i, field)| field.attrs.iter().any(|attr| attr.path.is_ident("deref")))
                    .collect::<Vec<_>>();

                if deref_fields.len() > 1 {
                    panic!("only one field can be marked with the attribute #[deref]");
                } else if deref_fields.len() == 0 {
                    panic!("unnamed fields must be 1, or specify the main field using the attribute #[deref]");
                } else {
                    deref_fields[0].0
                }
            } else {
                0
            };
            (unnamed[pos].clone(), quote! {#pos})
        }
        Fields::Named(FieldsNamed { named, .. }) => {
            let deref_fields = named
                .iter()
                .filter(|field| field.attrs.iter().any(|attr| attr.path.is_ident("deref")))
                .collect::<Vec<_>>();
            if deref_fields.len() > 1 {
                panic!("only one field can be marked with the attribute #[deref]");
            } else if deref_fields.len() == 0 {
                panic!("specify the main field using the attribute #[deref]");
            } else {
                let y = deref_fields[0].ident.clone();
                (deref_fields[0].clone(), quote! {#y})
            }
        }
        _ => panic!(),
    };
    let Field { ty, .. } = field.clone();
    quote! {
        impl #impl_generics std::ops::Deref for #ident #ty_generics {
            type Target = #ty;

            fn deref(&self) -> &Self::Target {
                &self. #pos
            }
        }

        impl #impl_generics std::ops::DerefMut for #ident #ty_generics {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self. #pos
            }
        }
    }
    .into()
}
