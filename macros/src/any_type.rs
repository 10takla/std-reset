use std::ops::Deref;

use macro_functions::{get_segment, get_segment_from_type, type_from_args};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemImpl, Type, TypeArray};

#[derive(Debug)]
enum Collection<'a> {
    None(&'a Type),
    Vec(&'a Type),
    Arr(&'a Type, usize),
}

impl Collection<'_> {
    fn get_inner_type(&self) -> &Type {
        match self {
            Collection::None(t) => t,
            Collection::Vec(t) => t,
            Collection::Arr(t, _) => t,
        }
    }
    fn get_token(&self) -> proc_macro2::TokenStream {
        match self {
            Collection::None(type_) => {
                quote! { #type_ }
            }
            Collection::Vec(type_) => {
                quote! { Vec<#type_> }
            }
            Collection::Arr(type_, len) => {
                quote! { [#type_; #len] }
            }
        }
    }
    fn define(type_: &Type) -> Collection {
        match type_ {
            Type::Array(TypeArray { elem, len, .. }) => {
                let len = len.to_token_stream().to_string().parse::<usize>().unwrap();
                Collection::Arr(elem.deref(), len)
            }
            Type::Path(_) | Type::Reference(_) => {
                let syn::PathSegment {
                    ident, arguments, ..
                } = get_segment_from_type(match type_ {
                    Type::Path(_) => type_,
                    Type::Reference(syn::TypeReference { elem, .. }) => elem.deref(),
                    _ => panic!(),
                });

                if ident.to_string() == "Vec" {
                    let inner_type =
                        type_from_args(arguments).expect("у вектора дожно быть только один тип");
                    return Collection::Vec(inner_type);
                }

                Collection::None(type_)
            }
            _ => panic!("Что"),
        }
    }

    fn set_type<'a>(&'a self, type_: &'a Type) -> Collection<'a> {
        match self {
            Collection::None(_) => Collection::None(type_),
            Collection::Vec(_) => Collection::Vec(type_),
            Collection::Arr(_, len) => Collection::Arr(type_, *len),
        }
    }
    fn get_transfers(&self) -> Vec<Self> {
        match self {
            Collection::None(type_) => vec![Collection::None(type_)],
            Collection::Vec(type_) => vec![Collection::Vec(type_)],
            Collection::Arr(type_, len) => {
                vec![Collection::Arr(type_, *len), Collection::Vec(type_)]
            }
        }
    }

    fn get_type(&self) -> Type {
        syn::parse2::<Type>(self.get_token()).unwrap()
    }
}

#[derive(Debug)]
enum Reference<'a> {
    NoneRef(&'a Type),
    Ref(&'a Type),
    Rc(&'a Type),
    RefRc(&'a Type),
}

impl Reference<'_> {
    fn set_type<'a>(&'a self, type_: &'a Type) -> Reference<'a> {
        match self {
            Reference::NoneRef(_) => Reference::NoneRef(type_),
            Reference::Ref(_) => Reference::Ref(type_),
            Reference::Rc(_) => Reference::Rc(type_),
            Reference::RefRc(_) => Reference::RefRc(type_),
        }
    }

    fn get_inner_type(&self) -> &Type {
        match self {
            Reference::NoneRef(t) => t,
            Reference::Ref(t) => t,
            Reference::Rc(t) => t,
            Reference::RefRc(t) => t,
        }
    }

    fn get_type(&self) -> Type {
        syn::parse2::<Type>(self.get_token()).unwrap()
    }

    fn define(type_: &Type) -> Reference {
        match type_ {
            Type::Path(syn::TypePath { path, .. }) => {
                let syn::PathSegment {
                    ident, arguments, ..
                } = get_segment(path);
                if ident.to_string() == "Rc" {
                    let inner_type =
                        type_from_args(arguments).expect("Rc должен иметь один параметр");
                    return Reference::Rc(inner_type);
                }
                Reference::NoneRef(type_)
            }
            Type::Array(_) => Reference::NoneRef(type_),
            Type::Reference(syn::TypeReference { elem, .. }) => {
                let type_ = elem.deref();

                let syn::PathSegment {
                    ident, arguments, ..
                } = get_segment_from_type(type_);
                if ident.to_string() == "Rc" {
                    let inner_type =
                        type_from_args(arguments).expect("Rc должен иметь один параметр");
                    return Reference::RefRc(inner_type);
                }
                Reference::Ref(type_)
            }
            _ => panic!("Ro"),
        }
    }

    fn get_transfers(&self) -> Vec<Reference> {
        let type_ = self.get_inner_type();
        vec![
            Reference::NoneRef(type_),
            Reference::Ref(type_),
            Reference::Rc(type_),
            Reference::RefRc(type_),
        ]
    }

    fn get_token(&self) -> proc_macro2::TokenStream {
        let type_ = self.get_inner_type();
        match self {
            Reference::NoneRef(_) => {
                quote! { #type_ }
            }
            Reference::Ref(_) => {
                quote! { &#type_ }
            }
            Reference::Rc(_) => {
                quote! { Rc<#type_> }
            }
            Reference::RefRc(_) => {
                quote! { &Rc<#type_> }
            }
        }
    }

    fn get_token_from_other_type(&self, type_: &Type) -> proc_macro2::TokenStream {
        match self {
            Reference::NoneRef(_) => {
                quote! { #type_ }
            }
            Reference::Ref(_) => {
                quote! { &#type_ }
            }
            Reference::Rc(_) => {
                quote! { Rc<#type_> }
            }
            Reference::RefRc(_) => {
                quote! { &Rc<#type_> }
            }
        }
    }
}

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemImpl {
        self_ty, trait_, ..
    } = parse_macro_input!(item);

    let of_type = {
        let u = trait_.unwrap().1;
        let syn::PathSegment { arguments, .. } = get_segment(&u);
        type_from_args(arguments)
            .expect("Of должен иметь только один тип")
            .clone()
    };
    for of in tmp(&of_type) {
        for to in tmp(&self_ty) {
            dbg!(quote! {#of for #to}.to_string());
        }
    }
    quote! {}.into()
}

fn tmp(type_: &Type) -> Vec<proc_macro2::TokenStream> {
    let ref_ = Reference::define(&type_);
    let mut vec = vec![];
    for ref_1 in ref_.get_transfers() {
        let col = Collection::define(&ref_1.get_inner_type());
        for col in col.get_transfers() {
            let ref_ = Reference::define(&col.get_inner_type());
            for ref_2 in ref_.get_transfers() {
                let y = col.set_type(&ref_2.get_type()).get_type();
                vec.push(ref_1.set_type(&y).get_token());
            }
        }
    }
    vec
}
