pub mod traits;

pub mod prelude {
    pub use super::traits::{of_to::{Of, To}, as_prim::AsPrim};
    pub use std_reset_macros::Default;
}
