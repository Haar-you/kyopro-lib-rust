//! 加法と乗法の単位元を定める。

/// 加算についての単位元をもつ。
pub trait Zero {
    /// 加法の単位元を返す。
    fn zero() -> Self;
}

/// 乗算についての単位元をもつ。
pub trait One {
    /// 乗法の単位元を返す。
    fn one() -> Self;
}

use std::num::Wrapping;
macro_rules! impl_one_zero {
    ($($t:tt),*) => {
        $(
            impl Zero for $t { fn zero() -> Self { 0 as $t } }
            impl One for $t { fn one() -> Self { 1 as $t } }
            impl Zero for Wrapping<$t> { fn zero() -> Self { Wrapping($t::zero()) }}
            impl One for Wrapping<$t> { fn one() -> Self { Wrapping($t::one()) }}
        )*
    }
}

impl_one_zero!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
