pub mod traits;

pub mod prelude {
    pub use super::traits::{
        as_prim::AsPrim,
        of_to::{Of, To},
    };
    pub use std_reset_macros::Default;
}