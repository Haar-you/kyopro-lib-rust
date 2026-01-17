//! 遅延セグメント木などに載せる構造
pub use crate::algebra::traits::*;

/// 遅延セグメント木などに載せる構造
pub trait Action {
    /// 範囲取得の型
    type Output;
    /// 範囲更新の型
    type Lazy;
    /// 範囲取得のモノイド
    type MonoidOutput: Monoid<Element = Self::Output>;
    /// 範囲更新のモノイド
    type MonoidLazy: Monoid<Element = Self::Lazy>;

    /// 範囲取得のモノイドへの参照を返す。
    fn monoid_output(&self) -> &Self::MonoidOutput;
    /// 範囲更新のモノイドへの参照を返す。
    fn monoid_lazy(&self) -> &Self::MonoidLazy;

    /// 範囲取得のモノイドの単位元を返す。
    fn fold_id(&self) -> Self::Output {
        self.monoid_output().id()
    }
    /// 範囲取得のモノイドの二項演算を適用させる。
    fn fold(&self, a: Self::Output, b: Self::Output) -> Self::Output {
        self.monoid_output().op(a, b)
    }
    /// 範囲更新のモノイドの単位元を返す。
    fn update_id(&self) -> Self::Lazy {
        self.monoid_lazy().id()
    }
    /// 範囲更新のモノイドの二項演算を適用させる。
    fn update(&self, cur: Self::Lazy, next: Self::Lazy) -> Self::Lazy {
        self.monoid_lazy().op(cur, next)
    }
    /// 範囲更新を範囲取得に反映させる。
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output;
}
