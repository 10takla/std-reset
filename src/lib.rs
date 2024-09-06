#![allow(unused)]

pub mod traits;

pub mod prelude {
    pub use super::traits::{
        as_prim::AsPrim,
        of_to::{Of, To},
        try_to::TryTo,
    };
    pub use std_reset_macros::{Default, Deref, Display, Getter, New, Setter};
}

mod turbo {
    use std::{collections::VecDeque, sync::atomic::AtomicPtr, thread::Thread};
    use std_reset_macros::{Default, New};

    #[test]
    fn tmp() {
        #[derive(Default)]
        struct QueueLock<T> {
            queue: AtomicPtr<VecDeque<Thread>>,
            data: T,
        }
    }
}
