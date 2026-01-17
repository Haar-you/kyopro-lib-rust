//! AddとMulの半環
use std::marker::PhantomData;

pub use crate::algebra::semiring::Semiring;

/// AddとMulの半環
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddMul<T>(PhantomData<T>);
impl<T> AddMul<T> {
    /// [`AddMul`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

macro_rules! impl_semiring {
    ($($t:ty),*) => {
        $(impl Semiring for AddMul<$t> {
            type Element = $t;
            fn zero(&self) -> Self::Element {
                0
            }
            fn one(&self) -> Self::Element {
                1
            }
            fn add(&self, a: Self::Element, b: Self::Element) -> Self::Element {
                a + b
            }
            fn mul(&self, a: Self::Element, b: Self::Element) -> Self::Element {
                a * b
            }
        })*
    };
}

impl_semiring!(u8, u16, u32, u64, u128, usize);
