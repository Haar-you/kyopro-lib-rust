//! 半環

pub mod xor_and;

/// 半環
pub trait Semiring {
    ///集合の要素の型
    type Element;
    /// 加法の単位元
    fn zero(&self) -> Self::Element;
    /// 乗法の単位元
    fn one(&self) -> Self::Element;
    /// 可換で結合的な二項演算
    fn add(&self, a: Self::Element, b: Self::Element) -> Self::Element;
    /// 結合的な二項演算
    fn mul(&self, a: Self::Element, b: Self::Element) -> Self::Element;
}
