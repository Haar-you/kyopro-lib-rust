//! 遅延セグメント木などに載せる構造
pub use crate::algebra::traits::*;

/// 遅延セグメント木などに載せる構造
pub trait Action {
    /// 範囲取得の型
    type Output: Monoid;
    /// 範囲更新の型
    type Lazy: Monoid;

    /// 範囲取得のモノイドの単位元を返す。
    fn fold_id() -> Self::Output {
        Self::Output::id()
    }
    /// 範囲取得のモノイドの二項演算を適用させる。
    fn fold(a: Self::Output, b: Self::Output) -> Self::Output {
        a.op(b)
    }
    /// 範囲更新のモノイドの単位元を返す。
    fn update_id() -> Self::Lazy {
        Self::Lazy::id()
    }
    /// 範囲更新のモノイドの二項演算を適用させる。
    fn update(cur: Self::Lazy, next: Self::Lazy) -> Self::Lazy {
        cur.op(next)
    }
    /// 範囲更新を範囲取得に反映させる。
    fn convert(value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output;
}
