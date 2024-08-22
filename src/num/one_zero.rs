/// 加算についての単位元をもつ
pub trait Zero {
    type Output;
    fn zero() -> Self::Output;
}

/// 乗算についての単位元をもつ
pub trait One {
    type Output;
    fn one() -> Self::Output;
}

macro_rules! impl_one_zero {
    ($($t:ty),*) => {
        $(
            impl Zero for $t {
                type Output = $t;
                fn zero() -> Self::Output { 0 as $t }
            }

            impl One for $t {
                type Output = $t;
                fn one() -> Self::Output { 1 as $t }
            }
        )*
    }
}

impl_one_zero!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
