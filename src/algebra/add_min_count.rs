//! Range Add Range Min-Count用の代数的構造
use crate::algebra::{action::Action, min_count::MinCount, sum::Sum, traits::*};
use std::marker::PhantomData;
use std::ops::Add;

/// Range Add Range Min-Count用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct AddMinCount<T, U>(PhantomData<(T, U)>);

impl<T, U> Action for AddMinCount<T, U>
where
    MinCount<T, U>: Monoid,
    Sum<T>: Monoid,
    T: Add<Output = T>,
{
    type Output = MinCount<T, U>;
    type Lazy = Sum<T>;

    fn convert(value: Self::Output, lazy: Self::Lazy, _: usize) -> Self::Output {
        MinCount(value.0.map(|x| x + lazy.0), value.1)
    }
}
