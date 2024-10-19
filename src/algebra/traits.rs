/// 代数構造
pub trait Set {
    /// 演算の対象の型
    type Element;
}

/// 二項演算をもつ
pub trait BinaryOp: Set {
    /// 二項演算
    fn op(&self, _: Self::Element, _: Self::Element) -> Self::Element;
}

/// 単位元をもつ
pub trait Identity: Set {
    /// 単位元
    fn id(&self) -> Self::Element;
}

/// 逆元をもつ
pub trait Inverse: Set {
    /// 逆元
    fn inv(&self, _: Self::Element) -> Self::Element;
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
pub trait Times: BinaryOp + Identity
where
    Self::Element: Clone,
{
    /// `n`個の値`a`に二項演算を適用する。
    fn times(&self, mut a: Self::Element, mut n: u64) -> Self::Element {
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
impl<A: BinaryOp + Identity> Times for A where Self::Element: Clone {}
