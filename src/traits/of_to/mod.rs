//! Трейты для преобразования типов.
//! 
//! Трейты  [`Of`]/[`To`] являются заменой [`From`]/[`Into`] из стандартной библиотеки и имеют более функциональный вид.
//!
//! Данная реализация [`From`]/[`Into`] может быть полензна, когда мы хотим сохранить функциональный стиль, при том что у нашего типа есть множество реализаций трейта [`From`].
//!
//! # Стандартная реализация с [`From`]/[`Into`]
//! Когда у типа `TypeInto` появляется более одной реализации типажа [`From`], нам необходимо указать компилятору, какую из реализаций мы используем в данный момент.
//! В стандартной реализации [`From`]/[`Into`]  это можно сделать с помощью следующих конструкций:
//! - `<TypeFrom as Into<TypeInto>>::`
//! - `Into::<TypeInto>>::`
//! , где `TypeFrom` - тип, который укзан в реализации как `impl From<TypeFrom> for TypeInto`.
//!
//! ## Пример
//! ```
//! struct Rubles;
//! struct Euros;
//! struct Dollars;
//! 
//! impl From<Rubles> for Dollars {
//!     fn from(value: Rubles) -> Self {
//!         Dollars
//!     }
//! }
//! 
//! impl From<Euros> for Dollars {
//!     fn from(value: Euros) -> Self {
//!         Dollars
//!     }
//! }
//! 
//! fn from_into() {
//!     let dollars = <Euros as Into<Dollars>>::into(Euros);
//!     let dollars = Into::<Dollars>::into(Euros);
//! }
//! ```
//!
//! # Реализацис c [`Of`]/[`To`]
//!
//! Для того чтобы указать компилятору сигнатуру опрделеной реализации трейта, мы можем указать границы не только для самого трейта (высокий уровень), но и для функций этого трейта (низкий уровень).
//! Так в трейте [`To`], generic типа, в который мы выполняем преобразование, указывается для метода `to`.
//! Теперь, чтобы указать конкртеную реализацию, нам достаточно конретизировать метод трейта:
//! - `TypeFrom.to::<TypeInto>()`
//!
//! ## Пример
//! ```
//! # use std_reset::prelude::{Of, To};
//! #
//! # struct Rubles;
//! # struct Euros;
//! # struct Dollars;
//! #
//! impl Of<Rubles> for Dollars {
//!     fn of(value: Rubles) -> Self {
//!         Dollars
//!     }
//! }
//! 
//! impl Of<Euros> for Dollars {
//!     fn of(value: Euros) -> Self {
//!         Dollars
//!     }
//! }
//! 
//! fn of_to() {
//!     let dollars = Rubles.to::<Dollars>();
//!     let dollars = Euros.to::<Dollars>();
//! }
//! ```
pub trait Of<F, Output = Self>: To {
    fn of(value: F) -> Output;
}

pub trait To {
    fn to<I: Of<Self>>(self) -> I
    where
        Self: Sized;
}

impl<F> To for F {
    fn to<I: Of<F>>(self) -> I {
        I::of(self)
    }
}


impl<F, I> Of<Vec<F>> for Vec<I>
where
    I: Of<F>,
{
    fn of(vec: Vec<F>) -> Self {
        vec.into_iter().map(I::of).collect()
    }
}

impl<F: Clone, I> Of<&Vec<F>> for Vec<I>
where
    I: Of<F>,
{
    fn of(vec: &Vec<F>) -> Self {
        vec.into_iter().cloned().map(I::of).collect()
    }
}