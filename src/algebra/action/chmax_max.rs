//! Range Chmax Range Max

pub use crate::algebra::{action::*, min_max::Max};

/// Range Chmax Range Max用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChmaxMax<T>(pub Max<T>);

impl<T> Action for ChmaxMax<T>
where
    Max<T>: Monoid<Element = T>,
    T: Ord,
{
    type Output = T;
    type Lazy = T;
    type MonoidOutput = Max<T>;
    type MonoidLazy = Max<T>;

    fn monoid_output(&self) -> &Self::MonoidOutput {
        &self.0
    }
    fn monoid_lazy(&self) -> &Self::MonoidLazy {
        &self.0
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, _: usize) -> Self::Output {
        value.max(lazy)
    }
}
