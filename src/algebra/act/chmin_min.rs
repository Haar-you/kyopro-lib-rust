//! Range Chmin Range Min
pub use crate::algebra::{act::*, min_max::Min};

/// Range Chmin Range Min 用のモノイド作用
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChminMin<T>(pub Min<T>);

impl<T> Act<Min<T>> for ChminMin<T>
where
    Min<T>: Monoid<Element = T>,
{
    type Monoid = Min<T>;
    type Element = T;
    fn monoid(&self) -> &Self::Monoid {
        &self.0
    }
    fn act(&self, m: &Min<T>, val: T, a: T, _len: usize) -> T {
        m.op(val, a)
    }
}
