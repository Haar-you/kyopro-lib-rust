use std::ops::Neg;

use crate::num::arithmetic::Arithmetic;
pub use crate::num::ops::{Inv, Pow};

#[allow(clippy::wrong_self_convention)]
pub trait FF: Clone {
    type Element;
    fn from_u64(&self, a: u64) -> Self::Element;
    fn from_i64(&self, a: i64) -> Self::Element;
    fn frac(&self, a: i64, b: i64) -> Self::Element;
}

pub trait FFElem:
    Sized + Neg<Output = Self> + PartialEq + Arithmetic + Pow<Output = Self> + Inv<Output = Self>
{
}
