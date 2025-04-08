//! 演算を定義するトレイト

/// 累乗
pub trait Pow {
    /// `pow`の結果の型
    type Output;
    /// `self`の`p`乗を求める。
    fn pow(self, p: u64) -> Self::Output;
}

/// 乗法の逆元
pub trait Inv {
    /// `inv`の結果の型
    type Output;
    /// `self`の乗法の逆元を求める。
    fn inv(self) -> Self::Output;
}
