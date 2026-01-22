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
                        id: |_| 0 as $t; inv: |_, a: $t| -a; commu; assoc;);)*
        $(impl Additive for Sum<$t> { fn times(&self, a: $t, n: u64) -> $t { a * <$t>::try_from(n).unwrap() }})*
        $(impl Ordered for Sum<$t>{})*
    };
    (unsigned; $($t:ty),*) => {
        $(impl_algebra!(Sum<$t>; set: $t; op: |_, a: $t, b: $t| a + b;
                        id: |_| 0 as $t; commu; assoc;);)*
        $(impl Additive for Sum<$t> { fn times(&self, a: $t, n: u64) -> $t { a * <$t>::try_from(n).unwrap() }})*
        $(impl Ordered for Sum<$t>{})*
    };
    (float; $($t:ty),*) => {
        $(impl_algebra!(Sum<$t>; set: $t; op: |_, a: $t, b: $t| a + b;
                        id: |_| 0 as $t; inv: |_, a: $t| -a; commu; assoc;);)*
        $(impl Additive for Sum<$t> { fn times(&self, a: $t, n: u64) -> $t { a * n as $t }})*
    }
}

implement!(signed; i8, i16, i32, i64, i128, isize);
implement!(unsigned; u8, u16, u32, u64, u128, usize);
implement!(float; f32, f64);
