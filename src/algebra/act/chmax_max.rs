//! Range Chmax Range Max
pub use crate::algebra::{act::*, min_max::Max};

/// Range Chmax Range Max 用のモノイド作用
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChmaxMax<T>(pub Max<T>);

impl<T> Act<Max<T>> for ChmaxMax<T>
where
    Max<T>: Monoid<Element = T>,
{
    type Monoid = Max<T>;
    type Element = T;
    fn monoid(&self) -> &Self::Monoid {
        &self.0
    }
    fn act(&self, m: &Max<T>, val: T, a: T) -> T {
        m.op(val, a)
    }
    fn act_n(&self, m: &Max<T>, val: T, a: T, _len: usize) -> T {
        m.op(val, a)
    }
}
