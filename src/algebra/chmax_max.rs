//! Range Chmax Range Max
use std::marker::PhantomData;

pub use crate::algebra::{action::*, min_max::Max};

/// Range Chmax Range Max用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChmaxMax<T>(PhantomData<T>);

impl<T> Action for ChmaxMax<T>
where
    Max<T>: Monoid,
    T: Ord,
{
    type Output = Max<T>;
    type Lazy = Max<T>;

    fn convert(value: Self::Output, lazy: Self::Lazy, _: usize) -> Self::Output {
        Max(value.0.max(lazy.0))
    }
}
