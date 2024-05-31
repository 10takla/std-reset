#![allow(unused)]

mod shared;

use proc_macro::TokenStream;
use quote::quote;
use shared::{get_segment_from_type, tmp};
use syn::{parse_macro_input, Field, Fields, FieldsNamed, ItemStruct, PathSegment};

/// Реализация трейта [`Default`] с указанием значений по умолчанию для каждого поля структуры.
///
/// Макрос поддерживает работу с именованными и неименнованными структурами.
///
/// Чтобы указать дефолтное значение поля необходимо использовать атрибут `default_field` и следующий синтаксис с ним:
/// ```
/// # use std_reset_macros::Default;
/// # #[derive(Debug, Default, PartialEq)]
/// # struct Wrap(
/// #[default_field("10_i32")]
/// #   i32
/// # );
/// ```
/// выражение, которое будет подставляться в поле как его дефолтное значение указывается внутри скобок и описан как строковый литерал.
///
/// P.s. Выражение необходимо записывать в виде строки, потому что`rust` требует
/// указывать в атрибутах только литерал. Поэтому под капотом
/// [`Default`] преобразует [cтроковый лиетрал](https://doc.rust-lang.org/reference/tokens.html?highlight=literal#string-literals) в выржаение.
/// 
/// Например для того, чтобы указать дефолтное значение для поля с типом `&str` необходимо написать следующее:
/// ```
/// # use std_reset_macros::Default;
/// # #[derive(Debug, Default, PartialEq)]
/// # struct Wrap(
/// #[default_field("\"crab\"")]
/// #   &'static str
/// # );
/// ```
///
/// # Примеры
/// - Структура с _именнованными_ полями:
///
/// ```
/// use std_reset_macros::Default;
///
/// #[derive(Debug, Default, PartialEq)]
/// struct User {
///     #[default_field("String::from(\"Ferris\")")]
///     name: String,
///     #[default_field("String::from(\"123FerF\")")]
///     password: String,
///     #[default_field("8_9999_999_999")]
///     number: u128,
///     email: Option<String>,
///     #[default_field("Some(32)")]
///     age: Option<u32>,
/// }
///
/// assert_eq!(
///     User::default(),
///     User {
///         name: "Ferris".to_string(),
///         password: "123FerF".to_string(),
///         number: 8_9999_999_999,
///         email: None,
///         age: Some(32),
///     }
/// );
/// ```
/// - Структура с _неименнованными_ полями:
/// ```
/// # use std_reset_macros::Default;
/// #
/// #[derive(Debug, Default, PartialEq)]
/// struct User(
///     #[default_field("String::from(\"Ferris\")")] String,
///     #[default_field("String::from(\"123FerF\")")] String,
///     #[default_field("8_9999_999_999")] u128,
///     Option<String>,
///     #[default_field("Some(32)")] Option<u32>,
/// );
///
/// assert_eq!(
///     User::default(),
///     User(
///         "Ferris".to_string(),
///         "123FerF".to_string(),
///         8_9999_999_999,
///         None,
///         Some(32),
///     )
/// );
/// ```

#[proc_macro_derive(Default, attributes(default_field))]
pub fn default_macro_derive(input: TokenStream) -> TokenStream {
    let ItemStruct { ident, fields, .. } = parse_macro_input!(input);
    let get_default_value = |field: &Field| {
        let Field { ty, .. } = field;
        field
            .attrs
            .iter()
            .find_map(|attr| {
                if attr.path.is_ident("default_field") {
                    attr.parse_args::<syn::LitStr>()
                        .map(|default_value| {
                            default_value
                                .value()
                                .parse::<proc_macro2::TokenStream>()
                                .expect("Failed to parse tokens")
                        })
                        .ok()
                } else {
                    None
                }
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

    quote! {
        impl std::default::Default for #ident {
            fn default() -> Self {
                Self #body
            }
        }
    }
    .into()
}
