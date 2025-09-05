//! Range Chmin Range Min
use std::marker::PhantomData;

pub use crate::algebra::{action::*, min_max::Min};

/// Range Chmin Range Min用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct ChminMin<T>(PhantomData<T>);

impl<T> Action for ChminMin<T>
where
    Min<T>: Monoid,
    T: Ord,
{
    type Output = Min<T>;
    type Lazy = Min<T>;

    fn convert(value: Self::Output, lazy: Self::Lazy, _: usize) -> Self::Output {
        Min(value.0.min(lazy.0))
    }
}
