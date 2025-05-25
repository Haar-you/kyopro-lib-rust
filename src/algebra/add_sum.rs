//! Range Add Range Sum用の代数的構造
use crate::algebra::action::Action;
use crate::algebra::sum::*;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Mul};

/// Range Add Range Sum用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct AddSum<T, U = T>(PhantomData<(T, U)>);

impl<T, U> Action for AddSum<T, U>
where
    Sum<T>: Monoid,
    Sum<U>: Monoid,
    T: Add<Output = T> + TryFrom<U, Error: Debug>,
    U: Mul<Output = U> + TryFrom<usize, Error: Debug>,
{
    type Output = Sum<T>;
    type Lazy = Sum<U>;

    fn convert(value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        Sum(value.0 + T::try_from(lazy.0 * U::try_from(len).unwrap()).unwrap())
    }
}
