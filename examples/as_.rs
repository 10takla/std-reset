#![allow(unused)]

use std::f32::consts::PI;
use std::fmt::Debug;
use std::marker;
use std_reset::traits::as_prim::{AsPrim, FromPrim};
use std_reset_macros::Deref;

#[derive(Deref, Debug, Clone, Copy)]
struct Vector<T>([T; 2]);

macro_rules! multi_operator {
    ($trait_:ident $method:ident $operator:tt) => {
        use std::ops:: $trait_ ;

        impl<F: AsPrim + Copy, I: $trait_ + FromPrim + Copy> $trait_<Vector<I>> for Vector<F>
        where
            [I; 2]: TryFrom<Vec<<I as $trait_>::Output>>,
            <[I; 2] as TryFrom<Vec<<I as $trait_>::Output>>>::Error: Debug,
        {
            type Output = Vector<I>;
            fn $method(self, other: Vector<I>) -> Self::Output {
                Vector(
                    self.into_iter()
                        .zip(other.into_iter())
                        .map(|(x, y)| x.as_::<I>() $operator y)
                        .collect::<Vec<_>>()
                        .try_to()
                        .unwrap(),
                )
            }
        }
        impl<F: AsPrim + Copy, I: FromPrim + $trait_ + Copy> $trait_<I> for Vector<F>
        where
            [I; 2]: TryFrom<Vec<<I as $trait_>::Output>>,
            <[I; 2] as TryFrom<Vec<<I as $trait_>::Output>>>::Error: Debug,
        {
            type Output = Vector<I>;
            fn $method(self, other: I) -> Self::Output {
                Vector (
                    self.into_iter()
                        .map(|x| x.as_::<I>() $operator other)
                        .collect::<Vec<_>>()
                        .try_to()
                        .unwrap(),
                )
            }
        }
    };
    ($($trait:ident $method:ident $operator:tt), +) => {
        use std_reset::traits::try_to::TryTo;
        $(
            multi_operator!($trait $method $operator);
        )+
    };
}

multi_operator!(Add add +, Sub sub -, Mul mul *, Div div /);

impl<T: AsPrim + Clone> Vector<T> {
    pub fn as_<I: FromPrim>(&self) -> Vector<I> {
        Vector(self.0.clone().map(|x| x.as_::<I>()))
    }
}

fn main() {
    let v_f64 = Vector([1.5, 2.2]);
    let v_i32 = Vector([2, 2]);

    let v_i32 = dbg!(v_f64 + v_i32);
    let v_f64 = dbg!(v_i32 + v_f64);

    let scale_factor = 0.3_f64;

    let v = dbg!(dbg!((v_f64 + v_i32)) * scale_factor);
}
