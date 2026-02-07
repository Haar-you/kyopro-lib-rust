//! $\mathbb{Z} / m \mathbb{Z}$の環
pub use crate::algebra::semiring::*;
use crate::num::ff::*;

/// $\mathbb{Z} / m \mathbb{Z}$の環
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddMulMod<T: ZZ>(pub T);

impl<T: ZZ> Semiring for AddMulMod<T> {
    type Element = T::Element;
    fn zero(&self) -> Self::Element {
        self.0.zero()
    }
    fn one(&self) -> Self::Element {
        self.0.one()
    }
    fn add(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        a + b
    }
    fn mul(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        a * b
    }
    fn times(&self, a: Self::Element, n: u64) -> Self::Element {
        a * self.0.from_u64(n)
    }
}

impl<T: ZZ> Ring for AddMulMod<T> {
    fn neg(&self, a: Self::Element) -> Self::Element {
        -a
    }
}

impl<T: FF> Field for AddMulMod<T>
where
    T::Element: FFElem,
{
    fn inv(&self, a: Self::Element) -> Self::Element {
        a.inv()
    }
}
