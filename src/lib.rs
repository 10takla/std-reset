#![feature(negative_impls)]
#![feature(marker_trait_attr)]

pub mod traits;

pub mod prelude {
    pub use super::traits::{
        as_prim::AsPrim,
        of_to::{Of, To},
        try_to::TryTo,
    };
    pub use std_reset_macros::{Default, Deref, Getter, New, Setter, Display};
}