//! 遅延セグメント木などに載せる構造
pub use crate::algebra::traits::*;

/// 遅延セグメント木などに載せる構造
pub trait Action {
    /// 範囲取得の型
    type Output;
    /// 範囲更新の型
    type Lazy;
    /// 範囲取得のモノイド
    type FoldMonoid: Monoid<Element = Self::Output>;
    /// 範囲更新のモノイド
    type UpdateMonoid: Monoid<Element = Self::Lazy>;

    /// 範囲取得のモノイドへの参照を返す。
    fn fold_monoid(&self) -> &Self::FoldMonoid;
    /// 範囲更新のモノイドへの参照を返す。
    fn update_monoid(&self) -> &Self::UpdateMonoid;

    /// 範囲取得のモノイドの単位元を返す。
    fn fold_id(&self) -> Self::Output {
        self.fold_monoid().id()
    }
    /// 範囲取得のモノイドの二項演算を適用させる。
    fn fold(&self, a: Self::Output, b: Self::Output) -> Self::Output {
        self.fold_monoid().op(a, b)
    }
    /// 範囲更新のモノイドの単位元を返す。
    fn update_id(&self) -> Self::Lazy {
        self.update_monoid().id()
    }
    /// 範囲更新のモノイドの二項演算を適用させる。
    fn update(&self, cur: Self::Lazy, next: Self::Lazy) -> Self::Lazy {
        self.update_monoid().op(cur, next)
    }

    /// 範囲更新を範囲取得に反映させる。
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output;
}
