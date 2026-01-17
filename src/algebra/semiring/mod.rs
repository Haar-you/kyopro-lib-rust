//! 半環

pub mod add_mul;
pub mod add_mul_mod;
pub mod xor_and;

/// 半環
pub trait Semiring {
    /// 元
    type Element;
    /// 加法の単位元
    fn zero(&self) -> Self::Element;
    /// 乗法の単位元
    fn one(&self) -> Self::Element;
    /// 加法(可換で結合的な二項演算)
    fn add(&self, a: Self::Element, b: Self::Element) -> Self::Element;
    /// 乗法(結合的な二項演算)
    fn mul(&self, a: Self::Element, b: Self::Element) -> Self::Element;
}

/// 環
pub trait Ring: Semiring {
    /// 加法の逆元
    fn neg(&self, a: Self::Element) -> Self::Element;
    /// 減法
    fn sub(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        self.add(a, self.neg(b))
    }
}

/// 体
pub trait Field: Ring {
    /// 乗法の逆元
    fn inv(&self, a: Self::Element) -> Self::Element;
}
