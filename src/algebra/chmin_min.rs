//! Range Chmin Range Min

pub use crate::algebra::{action::*, min_max::Min};

/// Range Chmin Range Min用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChminMin<T>(pub Min<T>);

impl<T> Action for ChminMin<T>
where
    Min<T>: Monoid<Element = T>,
    T: Ord,
{
    type Output = T;
    type Lazy = T;
    type MonoidOutput = Min<T>;
    type MonoidLazy = Min<T>;

    fn monoid_output(&self) -> &Self::MonoidOutput {
        &self.0
    }
    fn monoid_lazy(&self) -> &Self::MonoidLazy {
        &self.0
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, _: usize) -> Self::Output {
        value.min(lazy)
    }
}
