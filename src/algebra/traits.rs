//! 代数的構造に関するトレイトを定義する。
use crate::trait_alias;

/// 集合
pub trait Set {}

/// 二項演算をもつ
pub trait BinaryOp: Set {
    /// 二項演算
    fn op(self, other: Self) -> Self;

    /// 二項演算$\circ$で(右側から)代入操作($a \leftarrow a \circ b$)をする。
    fn op_assign_r(&mut self, b: Self)
    where
        Self: Clone,
    {
        *self = Self::op(self.clone(), b);
    }

    /// 二項演算$\circ$で(左側から)代入操作($a \leftarrow b \circ a$)をする。
    fn op_assign_l(&mut self, b: Self)
    where
        Self: Clone,
    {
        *self = Self::op(b, self.clone());
    }
}

/// 単位元をもつ
pub trait Identity: Set {
    /// 単位元
    fn id() -> Self;
}

/// 逆元をもつ
pub trait Inverse: Set {
    /// 逆元
    fn inv(self) -> Self;
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
pub trait Times: BinaryOp + Identity + Clone {
    /// $\underbrace{a \circ a \circ \dots \circ a \circ a}_{n}$を計算する。
    ///
    /// **Time complexity** $O(\log n)$
    fn times(self, mut n: u64) -> Self {
        let mut ret = Self::id();
        let mut a = self;

        while n > 0 {
            if n & 1 == 1 {
                ret = Self::op(ret, a.clone());
            }
            a = Self::op(a.clone(), a);
            n >>= 1;
        }

        ret
    }
}
impl<A: BinaryOp + Identity + Clone> Times for A {}

pub trait FoldM: Iterator {
    fn fold_m(self) -> Self::Item
    where
        Self: Sized,
        Self::Item: Monoid,
    {
        self.fold(Self::Item::id(), Self::Item::op)
    }
}

impl<I> FoldM for I where I: Iterator + ?Sized {}
