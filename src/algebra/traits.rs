/// 代数構造
pub trait AlgeStruct {
    /// 演算の対象の型
    type Output;
}

/// 二項演算をもつ
pub trait BinaryOp: AlgeStruct {
    /// 二項演算
    fn op(&self, _: Self::Output, _: Self::Output) -> Self::Output;
}

/// 単位元をもつ
pub trait Identity: AlgeStruct {
    /// 単位元
    fn id(&self) -> Self::Output;
}

/// 逆元をもつ
pub trait Inverse: AlgeStruct {
    /// 逆元
    fn inv(&self, _: Self::Output) -> Self::Output;
}

/// 可換性をもつ
pub trait Commutative {}
/// 結合性をもつ
pub trait Associative {}
/// 冪等性をもつ
pub trait Idempotence {}

/// 半群
pub trait Semigroup: BinaryOp + Associative {}
impl<T: BinaryOp + Associative> Semigroup for T {}

/// モノイド
pub trait Monoid: Semigroup + Identity {}
impl<T: Semigroup + Identity> Monoid for T {}

/// 可換モノイド
pub trait AbelianMonoid: Monoid + Commutative {}
impl<T: Monoid + Commutative> AbelianMonoid for T {}

/// 群
pub trait Group: Monoid + Inverse {}
impl<T: Monoid + Inverse> Group for T {}

/// 可換群
pub trait AbelianGroup: Group + Commutative {}
impl<T: Group + Commutative> AbelianGroup for T {}

/// 値に二項演算を複数回適用する。
pub trait Times<T: Clone>: BinaryOp<Output = T> + Identity {
    /// `n`個の値`a`に二項演算を適用する。
    fn times(&self, mut a: Self::Output, mut n: u64) -> Self::Output {
        let mut ret = self.id();

        while n > 0 {
            if n & 1 == 1 {
                ret = self.op(ret, a.clone());
            }
            a = self.op(a.clone(), a);
            n >>= 1;
        }

        ret
    }
}
impl<T: Clone, A: BinaryOp<Output = T> + Identity> Times<T> for A {}
