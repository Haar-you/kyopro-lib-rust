//! Range Add Range Min-Count
pub use crate::algebra::{action::Action, min_count::MinCount, sum::Sum, traits::*};
use std::ops::Add;

/// Range Add Range Min-Count用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct AddMinCount<T, U>(MinCount<T, U>, Sum<T>);
impl<T, U> AddMinCount<T, U> {
    /// [`AddMinCount<T, U>`]を作る。
    pub fn new(min_count: MinCount<T, U>, sum: Sum<T>) -> Self {
        Self(min_count, sum)
    }
}

impl<T, U> Action for AddMinCount<T, U>
where
    MinCount<T, U>: Monoid<Element = (Option<T>, U)>,
    Sum<T>: Monoid<Element = T>,
    T: Add<Output = T>,
{
    type Output = (Option<T>, U);
    type Lazy = T;
    type MonoidOutput = MinCount<T, U>;
    type MonoidLazy = Sum<T>;

    fn monoid_output(&self) -> &Self::MonoidOutput {
        &self.0
    }
    fn monoid_lazy(&self) -> &Self::MonoidLazy {
        &self.1
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, _: usize) -> Self::Output {
        (value.0.map(|x| x + lazy), value.1)
    }
}
