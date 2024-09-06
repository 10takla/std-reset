//! Функциональная реализация [`TryInto`]
//! 
//! ## Пример
//! ```
//! use std_reset::prelude::TryTo;
//! 
//! vec![1, 2, 3].try_to::<[u8; 3]>().unwrap();
//! ```

use std::convert::TryInto as TryInto_;
pub trait TryTo {
    fn try_to<T>(self) -> Result<T, Self::Error>
    where
        Self: TryInto_<T>,
    {
        TryInto_::try_into(self)
    }
}

impl<T> TryTo for T {}