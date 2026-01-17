//! 二面体群 $D_n$
//!
//! # Problems
//! - <https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0390>

pub use crate::algebra::traits::*;

/// 対称変換
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Sym {
    /// 回転
    R(usize),
    /// 鏡映
    S(usize),
}

use Sym::{R, S};

/// 二面体群$D_n$の元
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Dihedral {
    size: usize,
    value: Sym,
}

impl Dihedral {
    fn _op(a: Self, b: Self) -> Self {
        let n = a.size;
        assert_eq!(b.size, a.size);

        let value = match (a.value, b.value) {
            (R(x), R(y)) => R((x + y) % n),
            (R(x), S(y)) => S((n + y - x) % n),
            (S(x), R(y)) => S((x + y) % n),
            (S(x), S(y)) => R((n + y - x) % n),
        };

        Self { size: n, value }
    }

    /// $D_n$の回転を表す元$R_i$を返す。
    pub fn r(i: usize, n: usize) -> Self {
        assert!(n > 0);
        assert!(i < n);
        Self {
            size: n,
            value: R(i),
        }
    }

    /// $D_n$の鏡映を表す元$S_i$を返す。
    pub fn s(i: usize, n: usize) -> Self {
        assert!(n > 0);
        assert!(i < n);
        Self {
            size: n,
            value: S(i),
        }
    }
}

/// 二面体群$D_n$の元の合成
#[derive(Clone, Copy, Debug)]
pub struct Composition(usize);
impl Composition {
    /// 位数`2n`の二面体群を作る。
    pub fn new(n: usize) -> Self {
        Self(n)
    }
}

impl Set for Composition {
    type Element = Dihedral;
}

impl BinaryOp for Composition {
    fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        Dihedral::_op(a, b)
    }
}

impl Identity for Composition {
    fn id(&self) -> Self::Element {
        Dihedral {
            size: self.0,
            value: R(0),
        }
    }
    fn is_id(&self, a: &Self::Element) -> bool {
        a == &self.id()
    }
}

impl Inverse for Composition {
    fn inv(&self, a: Self::Element) -> Self::Element {
        let Dihedral { size, value } = a;
        let value = match value {
            R(x) => R(if x == 0 { 0 } else { size - x }),
            S(_) => value,
        };
        Dihedral { size, value }
    }
}

impl Associative for Composition {}
