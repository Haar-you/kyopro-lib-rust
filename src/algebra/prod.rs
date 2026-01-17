//! 乗算
use std::marker::PhantomData;

pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 乗算を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Prod<T>(PhantomData<T>);
impl<T> Prod<T> {
    /// [`Prod<T>`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

macro_rules! implement {
    ($($t:ty),*) => {
        $(impl_algebra!(
            Prod<$t>; set: $t; op: |_, a: $t, b: $t| a * b;
            id: |_| 1 as $t, |_, &a| a == 1 as $t; commu; assoc;);)*
    };
}

implement!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
