//! 遅延セグメント木などに載せる構造
pub mod add_min_count;
pub mod add_sum;
pub mod affine_sum;
pub mod chmax_max;
pub mod chmin_min;
pub mod update_fold;
pub mod update_sum;

pub use crate::algebra::traits::*;

/// モノイド作用
pub trait Act<M: Monoid> {
    /// 作用させるモノイド
    type Monoid: Monoid<Element = Self::Element>;
    /// モノイドの元
    type Element;

    /// 作用させるモノイドへの参照を返す。
    fn monoid(&self) -> &Self::Monoid;
    /// モノイド`Self`を別のモノイド`M`へ作用させる。
    fn act(&self, m: &M, val: M::Element, a: Self::Element) -> M::Element;
    /// $val = x_0 + x_1 + \dots + x_{len}$としたときに、
    /// $(x_0 * a) + (x_1 * a) + \dots + (x_{len} * a)$を求める。
    fn act_n(&self, m: &M, val: M::Element, a: Self::Element, len: usize) -> M::Element;

    /// 二項演算
    fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        self.monoid().op(a, b)
    }
    /// 単位元
    fn id(&self) -> Self::Element {
        self.monoid().id()
    }
}
