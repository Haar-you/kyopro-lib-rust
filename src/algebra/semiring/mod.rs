//! 半環

pub mod add_mul;
pub mod xor_and;

/// 半環
pub trait Semiring {
    /// 加法の単位元
    fn zero() -> Self;
    /// 乗法の単位元
    fn one() -> Self;
    /// 可換で結合的な二項演算
    fn add(self, b: Self) -> Self;
    /// 結合的な二項演算
    fn mul(self, b: Self) -> Self;
}
