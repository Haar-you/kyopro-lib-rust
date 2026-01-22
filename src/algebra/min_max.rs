//! 最小値・最大値
use std::marker::PhantomData;

pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 最小値を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Min<T>(PhantomData<T>);
impl<T> Min<T> {
    /// [`Min<T>`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

/// 最大値を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Max<T>(PhantomData<T>);
impl<T> Max<T> {
    /// [`Max<T>`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

macro_rules! implement {
    ($($t:tt),*) => {
        $(impl_algebra!(Min<$t>; set: $t; op: |_, a: $t, b: $t| a.min(b);
                        id: |_| $t::MAX; commu; assoc; idem;);)*
        $(impl_algebra!(Max<$t>; set: $t; op: |_, a: $t, b: $t| a.max(b);
                        id: |_| $t::MIN; commu; assoc; idem;);)*

        $(impl Ordered for Min<$t> {})*
        $(impl Ordered for Max<$t> {})*
    };
}

implement!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
