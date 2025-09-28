//! Range Affine Range Sum
pub use crate::algebra::{action::Action, affine::Affine, dual::Dual, sum::Sum, traits::*};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Mul};

/// Range Affine Range Sum用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AffineSum<T, U = T>(PhantomData<(T, U)>);

impl<T, U> Action for AffineSum<T, U>
where
    Sum<T>: Monoid,
    Affine<U>: Monoid,
    T: Add<Output = T> + Mul<Output = T> + TryFrom<U, Error: Debug>,
    U: Mul<Output = U> + TryFrom<usize, Error: Debug>,
{
    type Output = Sum<T>;
    type Lazy = Dual<Affine<U>>;

    fn convert(value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        let len = U::try_from(len).unwrap();
        let Dual(lazy) = lazy;
        Sum(T::try_from(lazy.0).unwrap() * value.0 + T::try_from(lazy.1 * len).unwrap())
    }
}
