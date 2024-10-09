use std::ops::Neg;

use crate::num::arithmetic::Arithmetic;

pub trait Pow {
    type Output;
    fn pow(self, p: u64) -> Self::Output;
}

pub trait Inv {
    type Output;
    fn inv(self) -> Self::Output;
}

#[allow(clippy::wrong_self_convention)]
pub trait FF: Clone {
    type Output;
    fn from_u64(&self, a: u64) -> Self::Output;
    fn from_i64(&self, a: i64) -> Self::Output;
    fn frac(&self, a: i64, b: i64) -> Self::Output;
}

pub trait FFElem:
    Pow<Output = Self>
    + Inv<Output = Self>
    + Arithmetic
    + Neg<Output = Self>
    + Copy
    + Clone
    + PartialEq
    + Sized
{
}
