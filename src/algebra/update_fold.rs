//! Range Update Range ~~~
pub use crate::algebra::{action::Action, first_last::Last, traits::*};
use std::fmt::Debug;

/// Range Update Range ~~~ 用の代数的構造
///
/// `convert`は時間計算量が$O(\log n)$なので、
/// 遅延セグメント木に載せる場合は、更新・取得はともに$O(\log^2 n)$の計算量になることに注意。
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UpdateFold<M: Monoid>(M, Last<M::Element>);
impl<M: Monoid> UpdateFold<M> {
    /// [`UpdateFold`]を作る。
    pub fn new(m: M) -> Self {
        Self(m, Last::new())
    }
}

impl<M> Action for UpdateFold<M>
where
    M: Monoid,
    M::Element: Clone,
{
    type Output = M::Element;
    type Lazy = Option<M::Element>;
    type MonoidOutput = M;
    type MonoidLazy = Last<M::Element>;

    fn monoid_output(&self) -> &Self::MonoidOutput {
        &self.0
    }
    fn monoid_lazy(&self) -> &Self::MonoidLazy {
        &self.1
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        match lazy {
            Some(m) => self.0.times(m, len as u64),
            _ => value,
        }
    }
}
