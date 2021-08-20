use crate::algebra::traits::*;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct BitXor<T>(PhantomData<T>);
impl<T> BitXor<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

macro_rules! int_bitxor_impl {
    ( $($t:ty),* ) => {
        $(
            impl AlgeStruct for BitXor<$t> {
                type Output = $t;
            }
            impl Inverse for BitXor<$t> {
                fn inv(&self, a: Self::Output) -> Self::Output { a }
            }
            impl BinaryOp for BitXor<$t> {
                fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output { a ^ b }
            }
            impl Identity for BitXor<$t> {
                fn id(&self) -> Self::Output { 0 }
            }
        )*
    }
}

int_bitxor_impl!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
