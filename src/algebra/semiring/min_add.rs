//! MinとAddの半環
pub use crate::algebra::semiring::*;
use std::marker::PhantomData;

/// MinとAddの半環
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MinAdd<T>(PhantomData<T>);
impl<T> MinAdd<T> {
    /// [`MinAdd`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

macro_rules! impl_semiring {
    ($($t:ty),*) => {
        $(impl Semiring for MinAdd<$t> {
            type Element = Option<$t>;
            fn zero(&self) -> Self::Element {
                None // inf
            }
            fn one(&self) -> Self::Element {
                Some(0)
            }
            fn add(&self, a: Self::Element, b: Self::Element) -> Self::Element {
                match (a, b) {
                    (Some(a), Some(b)) => Some(a.min(b)),
                    (None, b) => b,
                    (a, None) => a
                }
            }
            fn mul(&self, a: Self::Element, b: Self::Element) -> Self::Element {
                a.zip(b).map(|(a, b)| a + b)
            }
            fn times(&self, a: Self::Element, n: u64) -> Self::Element {
                if n == 0 { None } else { a }
            }
        })*
    };
}

impl_semiring!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
