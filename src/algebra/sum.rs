use std::marker::PhantomData;
use crate::algebra::traits::*;

#[derive(Clone)]
pub struct Sum<T>(PhantomData<T>);
impl<T> Sum<T> {
    pub fn new() -> Self { Self (PhantomData) }
}

macro_rules! signed_int_sum_impl {
    ( $($t:ty),* ) => {
        unsigned_int_sum_impl!($($t),*);
        $(
            impl Inverse<$t> for Sum<$t> {
                fn inv(&self, a: $t) -> $t { -a }
            }
        )*
    }
}

macro_rules! unsigned_int_sum_impl {
    ( $($t:ty),* ) => {
        $(
            impl BinaryOp<$t> for Sum<$t> {
                fn op(&self, a: $t, b: $t) -> $t { a + b }
            }
            impl Identity<$t> for Sum<$t> {
                fn id(&self) -> $t { 0 }
            }
        )*
    }
}

signed_int_sum_impl!(i8, i16, i32, i64, i128, isize);
unsigned_int_sum_impl!(u8, u16, u32, u64, u128, usize);
