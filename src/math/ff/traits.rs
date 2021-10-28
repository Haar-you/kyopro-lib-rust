use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub trait Pow {
    type Output;
    fn pow(self, p: u64) -> Self::Output;
}

pub trait Inv {
    type Output;
    fn inv(self) -> Self::Output;
}

pub trait Frac {
    type Output;
    fn frac(_: i64, _: i64) -> Self::Output;
}

pub trait FF:
    Pow<Output = Self>
    + Inv<Output = Self>
    + Frac<Output = Self>
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Neg<Output = Self>
    + Sum
    + Copy
    + Clone
    + PartialEq
    + Default
    + Sized
{
}
