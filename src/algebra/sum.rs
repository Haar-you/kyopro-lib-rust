//! 加法
use std::marker::PhantomData;

pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 加法を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sum<T>(PhantomData<T>);
impl<T> Sum<T> {
    /// [`Sum<T>`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

macro_rules! implement {
    (signed; $($t:ty),*) => {
        $(impl_algebra!(Sum<$t>; set: $t; op: |_, a: $t, b: $t| a + b;
                        id: |_| 0 as $t, |_, &a| a == 0 as $t;
                        inv: |_, a: $t| -a; commu; assoc;);)*
        $(impl Additive for Sum<$t> {})*
    };
    (unsigned; $($t:ty),*) => {
        $(impl_algebra!(Sum<$t>; set: $t; op: |_, a: $t, b: $t| a + b;
                        id: |_| 0 as $t, |_, &a| a == 0 as $t; commu; assoc;);)*
        $(impl Additive for Sum<$t> {})*
    };
}

implement!(signed; i8, i16, i32, i64, i128, isize, f32, f64);
implement!(unsigned; u8, u16, u32, u64, u128, usize);
