#![allow(unused)]

mod shared;

use paste::paste;
use proc_macro::TokenStream;
use quote::quote;
use shared::{get_segment_from_type, tmp};
use syn::{parse_macro_input, Field, Fields, FieldsNamed, FieldsUnnamed, ItemStruct, PathSegment};

/// Реализация трейта [`Default`] с указанием значений по умолчанию для каждого поля структуры.
///
/// Макрос поддерживает работу с именованными и неименнованными структурами.
///
/// Чтобы указать дефолтное значение поля необходимо использовать атрибут `default` со следующим синтаксисом:
/// ```
/// # use std_reset_macros::Default;
/// # #[derive(Debug, Default, PartialEq)]
/// # struct Wrap(
/// #[default("10_i32")]
/// #   i32
/// # );
/// ```
/// -- выражение, которое будет подставляться в поле как его дефолтное значение указывается внутри скобок и описан как строковый литерал.
///
/// **P.s.** Выражение описывается строке, потому что`rust` требует
/// указывать в атрибутах только литералы. Макрос [`Default`] в свою очередь 
/// преобразует [cтроковый лиетрал](https://doc.rust-lang.org/reference/tokens.html?highlight=literal#string-literals) в выржаение.
///
/// Например, указать дефолтное значение для поля с типом `&str` можно следующим образом:
/// ```
/// # use std_reset_macros::Default;
/// # #[derive(Debug, Default, PartialEq)]
/// # struct Wrap(
/// #[default("\"crab\"")]
/// #   &'static str
/// # );
/// ```
///
/// # Примеры
/// Структура с _именнованными_ полями:
///
/// ```
/// use std_reset_macros::Default;
///
/// #[derive(Debug, Default, PartialEq)]
/// struct User {
///     #[default("String::from(\"Ferris\")")]
///     name: String,
///     #[default("String::from(\"123FerF\")")]
///     password: String,
///     #[default("8_9999_999_999")]
///     number: u128,
///     email: Option<String>,
///     #[default("Some(32)")]
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
/// Структура с _неименнованными_ полями:
/// ```
/// # use std_reset_macros::Default;
/// #
/// #[derive(Debug, Default, PartialEq)]
/// struct User(
///     #[default("String::from(\"Ferris\")")] String,
///     #[default("String::from(\"123FerF\")")] String,
///     #[default("8_9999_999_999")] u128,
///     Option<String>,
///     #[default("Some(32)")] Option<u32>,
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
#[proc_macro_derive(Default, attributes(default))]
pub fn default_macro_derive(input: TokenStream) -> TokenStream {
    default::expand(input)
}
mod default;

/// Автореализация [`Deref`] и [`DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) для структур.
///
/// Макрос поддерживает работу с именованными и неименнованными структурами.
///
/// # Реализация с одним неименованным полем
/// Дефолтная реализация макроса без дополнительных указаний работает только с одним неименованым полем.
/// При разименовании структуры будет возвращены данные этого поле.
/// ```
/// use std_reset_macros::Deref;
///
/// #[derive(Deref)]
/// struct Wrapper(pub Vec<i32>);
///
/// let mut wrapper = Wrapper(vec![1, 2, 3]);
/// assert_eq!(*wrapper, vec![1, 2, 3]);
/// ```
/// # Со множеством неименованных полей
/// Когда появляется несколько полей, макросу необходимо указать конретное поле,
/// которое будет возвращено после разыменования с помощью атрибута `#[deref]`.
/// ```
/// # use std_reset_macros::Deref;
/// #
/// #[derive(Deref)]
/// struct Wrapper(pub Vec<i32>, #[deref] pub String);
/// 
/// let mut wrapper = Wrapper(vec![1, 2, 3], String::from("crab"));
/// assert_eq!(*wrapper, "crab");
/// ```
/// # Со множеством именованных полей
/// Тоже самое работает и с именованными полями:
/// ```
/// # use std_reset_macros::Deref;
/// #
/// #[derive(Deref)]
/// struct Wrapper {
///     pub first: Vec<i32>,
///     #[deref]
///     pub second: String,
/// }
///
/// let mut wrapper = Wrapper {
///     first: vec![1, 2, 3],
///     second: String::from("crab"),
/// };
/// assert_eq!(*wrapper, "crab");
/// ```

#[proc_macro_derive(Deref, attributes(deref))]
pub fn deref_macro_derive(input: TokenStream) -> TokenStream {
    deref::expand(input)
}
mod deref;

/// Автоопределение `set` методов для полей именованых структур.
/// 
/// По умолчанию все поля включены в определение `set_` методов.
/// Также с помощью атрибутов можно опционально исключать полe из определния `set_` метода,
/// а также включать.
/// - Аттрибут `exclude_setter` исключает поле из полей по умолчанию;
/// - Аттрибут `include_setter` заставляет макрос определять метод `set_` только для полей с этим атрибутом.
/// 
/// # Конфликты атрибутов
/// - Поле не может иметь одновременно исключающее и включающее поле, они препятсвуют работе друг друга;
/// - Поле не может быть исключающим, если какое-либо поле до него было определено как включающее, и наоборот.
/// 
/// # Реализация по умолчанию
/// ```
/// use std_reset_macros::Setter;
/// 
/// #[derive(Setter, Clone, Copy, Default, PartialEq, Debug)]
/// struct Tmp {
///     first: i32,
///     second: i32,
/// }
/// let tmp = Tmp::default().set_first(2).set_second(3);
/// assert_eq!(
///     tmp,
///     Tmp {
///         first: 2,
///         second: 3
///     }
/// );
/// ```
/// 
/// # Исключающие поля
/// 
/// C помощью атрибута `exclude_setter` можно исключить поле из определения `set_` метода,
/// таким образом метод будет определен только для дефолтных полей.
/// ## Пример
/// ```
/// # use std_reset_macros::Setter;
/// #[derive(Setter, Clone, Copy, Default, PartialEq, Debug)]
/// struct Tmp {
///     first: i32,
///     #[exclude_setter]
///     second: i32,
/// }
/// # let tmp = Tmp::default().set_first(2);
/// # assert_eq!(
/// #     tmp,
/// #     Tmp {
/// #         first: 2,
/// #         second: 0
/// #     }
/// # );
/// ```
/// -- здесь метод `set_` определен только для поля `first`.
/// 
/// # Включающие поля
/// 
/// Если есть хотябы одно поле с атрибутом `include_setter`, это значит,
/// что макрос перестает опрделеять методы `set_` для полей по умолчанию,
/// а начал назначать их для полей с атрибутом `include_setter`.
/// ## Пример
/// ```
/// # use std_reset_macros::Setter;
/// #[derive(Setter, Clone, Copy, Default, PartialEq, Debug)]
/// struct Tmp {
///     #[include_setter]
///     first: i32,
///     second: i32,
///     #[include_setter]    
///     third: i32
/// }
/// # let tmp = Tmp::default().set_first(2).set_third(5);
/// # assert_eq!(
/// #     tmp,
/// #     Tmp {
/// #         first: 2,
/// #         second: 0,
/// #         third: 5
/// #     }
/// # );
/// ```
/// -- здесь метод `set_` определен только для полей `first` и `third`.
/// 
/// 
/// 
#[proc_macro_derive(Setter, attributes(exclude_setter, include_setter))]
pub fn setter_macro_derive(input: TokenStream) -> TokenStream {
    setter_getter::expand_setter(input)
}
mod setter_getter;

/// Автоопределение `get` методов для полей именованых структур.
/// Тоже самое что и в [`Setter`], но:
/// - вместо аттрибута `exclude_setter` - `exclude_getter`
/// - вместо аттрибута `include_setter` - `include_getter`
/// - вместо `set_` метода - `get_`
#[proc_macro_derive(Getter, attributes(exclude_getter, include_getter))]
pub fn getter_macro_derive(input: TokenStream) -> TokenStream {
    setter_getter::expand_getter(input)
}

/// Прямая реализация метода `new`.
/// 
/// Макрос поддерживает работу с именованными и неименованными полями.
/// 
/// ## Примеры
/// ```
/// use std_reset_macros::New;
/// 
/// #[derive(New)]
/// struct Tmp {
///     first: i32,
///     second: i32,
/// }
/// 
/// Tmp::new(2, 3);
/// ```
/// ```
/// # use std_reset_macros::New;
/// #
/// #[derive(New)]
/// struct Tmp(i32, i32);
/// 
/// Tmp::new((2, 3));
/// ```
#[proc_macro_derive(New)]
pub fn  new_macro_derive(input: TokenStream) -> TokenStream {
    new::expand(input)
}
mod new;