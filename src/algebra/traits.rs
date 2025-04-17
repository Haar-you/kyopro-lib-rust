//! 代数的構造に関するトレイトを定義する。
use crate::trait_alias;

/// 集合
pub trait Set {
    /// 集合の要素の型
    type Element;
}

/// 二項演算をもつ
pub trait BinaryOp: Set {
    /// 二項演算
    fn op(&self, _: Self::Element, _: Self::Element) -> Self::Element;

    /// 二項演算$\circ$で(右側から)代入操作($a \leftarrow a \circ b$)をする。
    fn op_assign(&self, a: &mut Self::Element, b: Self::Element)
    where
        Self::Element: Clone,
    {
        *a = self.op(a.clone(), b);
    }
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

trait_alias!(#[doc = "半群"] Semigroup: BinaryOp + Associative);
trait_alias!(#[doc = "モノイド"] Monoid: Semigroup + Identity);
trait_alias!(#[doc = "可換モノイド"] AbelianMonoid: Monoid + Commutative);
trait_alias!(#[doc = "群"] Group: Monoid + Inverse);
trait_alias!(#[doc = "可換群"] AbelianGroup: Group + Commutative);

/// 値に二項演算を複数回適用する。
pub trait Times: BinaryOp + Identity
where
    Self::Element: Clone,
{
    /// $\underbrace{a \circ a \circ \dots \circ a \circ a}_{n}$を計算する。
    ///
    /// **Time complexity** $O(\log n)$
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
