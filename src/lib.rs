pub mod traits;

pub mod prelude {
    pub use super::traits::{
        as_prim::AsPrim,
        of_to::{Of, To},
    };
    pub use std_reset_macros::{Default, Deref, Setter, Getter, New};
}

mod turbo {
    use std_reset_macros::New;

    #[derive(New)]
    struct Tmp {
        first: i32,
        second: i32,
    }

    
    #[derive(New)]
    struct Tmp2(i32, i32);
}