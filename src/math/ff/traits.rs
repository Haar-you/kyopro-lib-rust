use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Pow {
    type Output;
    fn pow(self, p: u64) -> Self::Output;
}

pub trait Inv {
    type Output;
    fn inv(self) -> Self::Output;
}

pub trait FF: Clone {
    type Output;
    fn from_u64(&self, a: u64) -> Self::Output;
    fn from_i64(&self, a: i64) -> Self::Output;
    fn frac(&self, a: i64, b: i64) -> Self::Output;
}

pub trait FFElem:
    Pow<Output = Self>
    + Inv<Output = Self>
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Neg<Output = Self>
    + Copy
    + Clone
    + PartialEq
    + Sized
{
}
