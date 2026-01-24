//! 半環

pub mod add_mul;
pub mod add_mul_mod;
pub mod max_add;
pub mod min_add;
pub mod xor_and;

/// 半環
pub trait Semiring {
    /// 集合の元
    type Element;
    /// 加法の単位元
    fn zero(&self) -> Self::Element;
    /// 乗法の単位元
    fn one(&self) -> Self::Element;
    /// 加法$\oplus$
    fn add(&self, a: Self::Element, b: Self::Element) -> Self::Element;
    /// 乗法$\otimes$
    fn mul(&self, a: Self::Element, b: Self::Element) -> Self::Element;
    /// $\underbrace{a \oplus a \oplus  \dots \oplus a \oplus a}_{n}$を計算する。
    fn times(&self, a: Self::Element, n: u64) -> Self::Element;
}

/// 環
pub trait Ring: Semiring {
    /// 加法の逆元 $-a$
    fn neg(&self, a: Self::Element) -> Self::Element;
    /// $a \oplus (-b)$
    fn sub(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        self.add(a, self.neg(b))
    }
}

/// 体
pub trait Field: Ring {
    /// 乗法の逆元 $a^{-1}$
    fn inv(&self, a: Self::Element) -> Self::Element;
    /// $a \otimes b^{-1}$
    fn div(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        self.mul(a, self.inv(b))
    }
}
