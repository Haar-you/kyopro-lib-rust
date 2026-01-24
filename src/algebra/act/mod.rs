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

    /// $val$を`n`個の値からなる列をモノイド$(\circ, e)$で畳み込んだ値であるとしたとき、
    /// 列の各値に$a$を作用させて畳み込んだ値を求める。
    ///
    /// 畳み込んだ値が同一になるような、長さ`n`のいかなる列に対しても、$a$を作用させて畳み込んだ値はすべて同一でなければならない。
    fn act(&self, m: &M, val: M::Element, a: Self::Element, n: usize) -> M::Element;

    /// `self.act(m, val, a, 1)`
    fn act_one(&self, m: &M, val: M::Element, a: Self::Element) -> M::Element {
        self.act(m, val, a, 1)
    }

    /// 二項演算
    fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        self.monoid().op(a, b)
    }
    /// 単位元
    fn id(&self) -> Self::Element {
        self.monoid().id()
    }
}
