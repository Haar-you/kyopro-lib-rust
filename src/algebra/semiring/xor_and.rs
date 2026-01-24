//! XorとAndの半環
use std::marker::PhantomData;

pub use crate::algebra::semiring::*;

/// XorとAndの半環
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XorAnd<T>(PhantomData<T>);
impl<T> XorAnd<T> {
    /// [`XorAnd`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

macro_rules! impl_semiring {
    ($($t:ty),*) => {
        $(impl Semiring for XorAnd<$t> {
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
            fn times(&self, a: Self::Element, n: u64) -> Self::Element {
                if n % 2 == 0 { 0 } else { a }
            }
        })*

        $(impl Ring for XorAnd<$t> {
            fn neg(&self, a: Self::Element) -> Self::Element {
                a
            }
        })*
    };
}

impl_semiring!(u8, u16, u32, u64, u128, usize);
