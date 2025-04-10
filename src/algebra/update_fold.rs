//! Range Update Range ~~~ 用の代数的構造
use crate::algebra::action::Action;
use crate::algebra::update::*;
use std::fmt::Debug;

/// Range Update Range ~~~ 用の代数的構造
///
/// `convert`は時間計算量が$O(\log n)$なので、
/// 遅延セグメント木に載せる場合は、更新・取得はともに$O(\log^2 n)$の計算量になることに注意。
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct UpdateFold<M: Monoid> {
    fold_m: M,
    update_m: Update<M::Element>,
}

impl<M: Monoid> UpdateFold<M> {
    /// [`UpdateFold<M>`]を生成する。
    pub fn new(fold_m: M) -> Self {
        Self {
            fold_m,
            update_m: Update::new(),
        }
    }
}

impl<M> Action for UpdateFold<M>
where
    M: Monoid,
    M::Element: Clone,
{
    type FoldMonoid = M;
    type UpdateMonoid = Update<M::Element>;
    type Output = <Self::FoldMonoid as Set>::Element;
    type Lazy = <Self::UpdateMonoid as Set>::Element;

    fn fold_monoid(&self) -> &Self::FoldMonoid {
        &self.fold_m
    }
    fn update_monoid(&self) -> &Self::UpdateMonoid {
        &self.update_m
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        match lazy {
            Some(lazy) => self.fold_monoid().times(lazy, len as u64),
            _ => value,
        }
    }
}
