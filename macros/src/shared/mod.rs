use quote::quote;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, Generics, ItemStruct, Path, PathArguments,
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
