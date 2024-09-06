#![cfg_attr(feature = "unstable", feature(extend_one))]

use quote::quote;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, ItemStruct, Path, PathArguments,
    PathSegment, Type, TypePath,
};

pub fn get_segment_from_type(type_: &Type) -> &PathSegment {
    get_segment(&get_path(type_))
}

pub fn get_segment(path: &Path) -> &PathSegment {
    let Path { segments, .. } = path;
    segments.into_iter().next().unwrap()
}

pub fn get_path(elem: &Type) -> &Path {
    let Type::Path(TypePath { path, .. }) = elem else {
        panic!()
    };
    path
}

pub fn type_from_args(arguments: &PathArguments) -> Result<&Type, ()> {
    let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = arguments
    else {
        return Err(());
    };
    let GenericArgument::Type(arg) = args.into_iter().next().unwrap() else {
        return Err(());
    };
    Ok(arg)
}

pub fn fast_impl(
    struct_: &ItemStruct,
    methods: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let ItemStruct {
        generics, ident, ..
    } = struct_;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            #methods
        }
    }
}


#[cfg(feature = "unstable")]
pub trait Split {
    fn split(&self, split: &proc_macro2::TokenStream) -> Vec<proc_macro2::TokenStream>;
}
#[cfg(feature = "unstable")]
impl Split for proc_macro2::TokenStream {
    fn split(&self, split: &proc_macro2::TokenStream) -> Vec<proc_macro2::TokenStream> {
        let input = self.clone().into_iter();
        let len = input.clone().count();
        let mut i = 0;

        if len == 0 {
            return vec![];
        }

        let mut arr = vec![proc_macro2::TokenStream::new()];
        let mut ptr = 0;

        while i < len {
            let mut split = split.clone().into_iter();
            let len = split.clone().count();
            let mut s_i = 0;
            let mut f = true;

            while s_i < len {
                let token = input.clone().nth(i + s_i).unwrap();
                let split_t = split.next().unwrap();

                if token.to_string() == split_t.to_string() {
                    s_i += 1;
                } else {
                    f = false;
                    break;
                }
            }

            if f {
                arr.push(proc_macro2::TokenStream::new());
                ptr += 1;
                i += len;
            } else {
                let token = input.clone().nth(i).unwrap();
                arr[ptr].extend_one(token);
                i += 1;
            }
        }
        arr
    }
}