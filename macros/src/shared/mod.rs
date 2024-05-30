use syn::{AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, PathSegment, Type, TypePath};

pub fn get_segment_from_type(type_: &Type) -> PathSegment {
    get_segment(&get_path(type_))
}

pub fn get_segment(path: &Path) -> PathSegment {
    let Path { segments, .. } = path;
    segments.into_iter().next().unwrap().clone()
}

pub fn get_path(elem: &Type) -> Path {
    let Type::Path(TypePath { path, .. }) = elem else {
        panic!()
    };
    path.clone()
}

pub fn tmp(arguments: PathArguments) -> Type {
    let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = arguments
    else {
        panic!()
    };
    let GenericArgument::Type(arg) = args.into_iter().next().unwrap() else {
        panic!()
    };
    arg
}
