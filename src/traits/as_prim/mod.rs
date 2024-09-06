//! Перобразование примитивных числовых типов через метод трейта [`AsPrim`] (замена опреатора `as`).
//!
//! # Стандартная реализация с оператором `as`.
//! Изначально rust предоставляет нам возможность преобразования примитивных типов с помощью опреатора `as`.
//! В следующих примерах показано, как опреатор `as` может сбивать с толку из-за своей семнтики:
//! ```
//! use core::f64::consts::PI;
//!
//! let radius = 10_i32;
//! let circle_area = 2 as f64 * PI * radius as f64;
//! ```
//! Даже скобки не спасают:
//! ```
//! # use core::f64::consts::PI;
//! # let radius = 10_i32;
//! let circle_area = (2 as f64) * PI * (radius as f64);
//! ```
//! # Реализация AsPrim
//!
//! Трейт [`AsPrim`] и вспомогательные ему [`FromPrim`] и [`ToPrim`] преднозначены для того, чтобы добиться функционального стиля в преобразовании примитивных типов без оператора `as`.
//!
//! ```
//! # use std_reset::prelude::AsPrim;
//! # use core::f64::consts::PI;
//! # let radius = 10_i32;
//! let circle_area = 2.as_::<f64>() * PI * radius.as_::<f64>();
//! let vec: Vec<f32> = vec![2.as_(), circle_area.as_(), 4_isize.as_()];
//! ```
//!
//! Также вы можете использовать [`FromPrim`], как будто используете [`From`]:
//! ```
//! # use std_reset::traits::as_prim::FromPrim;
//! let num = f64::as_from(10_i32);
//!
//! ```
//! Или  использовать [`ToPrim`] для преобразования напрямую:
//! ```
//! # use std_reset::traits::as_prim::ToPrim;
//! let num = 2.to_f32();
//! ```

use paste::paste;

macro_rules! every_type_method {
     ($($t:ty),+) => {
         $(
             paste! {
                 fn [<to_ $t>](self) -> $t {
                     self as $t
                 }
             }
         )+
     };
 }

macro_rules! impl_for_every_num_types {
     ($($t:ty),+) => {
         pub trait ToPrim: ToString {
             $(
                 paste! {
                     fn [<to_ $t>](self) -> $t;
                 }
             )+
         }
         $(
             impl ToPrim for $t {
                 every_type_method!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
             }
         )+
         $(
             impl FromPrim for $t {
                 paste! {
                     fn as_from<F: ToPrim>(value: F) -> $t {
                         value.[<to_ $t>]()
                     }
                 }
             }
         )+
     }
 }

impl_for_every_num_types!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
pub trait FromPrim: ToPrim {
    fn as_from<F: ToPrim>(value: F) -> Self;
}

pub trait AsPrim: FromPrim {
    fn as_<I: FromPrim>(self) -> I;
}

impl<F: FromPrim> AsPrim for F {
    fn as_<I: FromPrim>(self) -> I {
        I::as_from::<F>(self)
    }
}
