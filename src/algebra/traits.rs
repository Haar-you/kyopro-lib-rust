//! 代数的構造に関するトレイトを定義する。
use crate::trait_alias;

/// 集合
pub trait Set: Sized {
    /// 集合の元
    type Element;
}

/// 二項演算をもつ
pub trait BinaryOp: Set {
    /// 二項演算
    fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element;

    /// 二項演算$\circ$で(右側から)代入操作($a \leftarrow a \circ b$)をする。
    fn op_assign_r(&self, a: &mut Self::Element, b: Self::Element)
    where
        Self::Element: Clone,
    {
        *a = self.op(a.clone(), b);
    }

    /// 二項演算$\circ$で(左側から)代入操作($a \leftarrow b \circ a$)をする。
    fn op_assign_l(&self, a: &mut Self::Element, b: Self::Element)
    where
        Self::Element: Clone,
    {
        *a = self.op(b, a.clone());
    }
}

/// 単位元をもつ
pub trait Identity: Set {
    /// 単位元
    fn id(&self) -> Self::Element;
    /// 単位元の判定
    fn is_id(&self, a: &Self::Element) -> bool;
}

/// 逆元をもつ
pub trait Inverse: Set {
    /// 逆元
    fn inv(&self, a: Self::Element) -> Self::Element;
}

/// 可換性をもつ
pub trait Commutative {}
/// 結合性をもつ
pub trait Associative {}
/// 冪等性をもつ
pub trait Idempotence {}

/// 半群
pub trait Semigroup: BinaryOp + Associative {
    /// `iter`が空のとき、`None`を返す。
    /// そうでないとき、`iter`の中身を二項演算で畳み込んで`Some`で返す。
    fn reduce<I>(&self, iter: I) -> Option<Self::Element>
    where
        I: IntoIterator<Item = Self::Element>,
    {
        iter.into_iter().reduce(|a, b| self.op(a, b))
    }
}
impl<T: BinaryOp + Associative> Semigroup for T {}

/// モノイド
pub trait Monoid: Semigroup + Identity {
    /// `iter`が空のとき、モノイドの単位元を返す。
    /// そうでないとき、`iter`の中身を二項演算で畳み込んで返す。
    fn fold_m<I>(&self, iter: I) -> Self::Element
    where
        I: IntoIterator<Item = Self::Element>,
    {
        self.reduce(iter).unwrap_or(self.id())
    }

    /// $\underbrace{a \circ a \circ \dots \circ a \circ a}_{n}$を計算する。
    fn times(&self, mut a: Self::Element, mut n: u64) -> Self::Element
    where
        Self::Element: Clone,
    {
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
impl<T: Semigroup + Identity> Monoid for T {}

trait_alias!(#[doc = "可換モノイド"] AbelianMonoid: Monoid + Commutative);
trait_alias!(#[doc = "群"] Group: Monoid + Inverse);
trait_alias!(#[doc = "可換群"] AbelianGroup: Group + Commutative);

trait_alias!(#[doc = "半束"] Semilattice: Semigroup + Commutative + Idempotence);

/// `fold_m`を提供する。
pub trait FoldM: Iterator {
    /// モノイドで畳み込んだ結果を返す。
    fn fold_m<M>(self, monoid: &M) -> Self::Item
    where
        Self: Sized,
        M: Monoid<Element = Self::Item>,
    {
        monoid.fold_m(self)
    }
}

impl<I> FoldM for I where I: Iterator + ?Sized {}
