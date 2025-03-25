//! 数の性質のトレイト

macro_rules! implement {
    ($tr:ty; $($t:ty),*) => {
        $(
            impl $tr for $t { }
        )*
    }
}

/// 非負の数値型
pub trait Unsigned {}
implement!(Unsigned; u8, u16, u32, u64, u128, usize);

/// 符号付きの数値型
pub trait Signed {}
implement!(Signed; i8, i16, i32, i64, i128, isize, f32, f64);

/// 整数型
pub trait Int {}
implement!(Int; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

/// 浮動小数点型
pub trait Float {}
implement!(Float; f32, f64);
