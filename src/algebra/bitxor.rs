use std::marker::PhantomData;
use crate::algebra::traits::*;

#[derive(Clone)]
pub struct BitXor<T>(PhantomData<T>);
impl<T> BitXor<T> {
    pub fn new() -> Self { Self (PhantomData) }
}

macro_rules! int_bitxor_impl {
    ( $($t:ty),* ) => {
        $(
            impl Inverse<$t> for BitXor<$t> {
                fn inv(&self, a: $t) -> $t { a }
            }
            impl BinaryOp<$t> for BitXor<$t> {
                fn op(&self, a: $t, b: $t) -> $t { a ^ b }
            }
            impl Identity<$t> for BitXor<$t> {
                fn id(&self) -> $t { 0 }
            }
        )*
    }
}

int_bitxor_impl!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
