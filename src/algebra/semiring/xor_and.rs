//! XorとAndの半環
pub use crate::algebra::semiring::Semiring;
use std::marker::PhantomData;

/// XorとAndの半環
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct XorAndSemiring<T>(PhantomData<T>);
impl<T> XorAndSemiring<T> {
    /// [`XorAndSemiring<T>`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

macro_rules! impl_semiring {
    ($($t:ty),*) => {
        $(impl Semiring for XorAndSemiring<$t> {
            type Element = $t;
            fn zero(&self) -> Self::Element {
                0
            }
            fn one(&self) -> Self::Element {
                !0
            }
            fn add(&self, a: Self::Element, b: Self::Element) -> Self::Element {
                a ^ b
            }
            fn mul(&self, a: Self::Element, b: Self::Element) -> Self::Element {
                a & b
            }
        })*
    };
}

impl_semiring!(u8, u16, u32, u64, u128, usize);
