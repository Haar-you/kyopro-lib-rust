//! 二面体群 $D_n$
//!
//! # Problems
//! - <https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0390>

pub use crate::algebra::traits::*;
use crate::impl_algebra;

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
    /// 元の合成
    pub fn compose(self, b: Self) -> Self {
        let n = self.size;
        assert_eq!(b.size, self.size);

        let value = match (self.value, b.value) {
            (R(x), R(y)) => R((x + y) % n),
            (R(x), S(y)) => S((n + y - x) % n),
            (S(x), R(y)) => S((x + y) % n),
            (S(x), S(y)) => R((n + y - x) % n),
        };

        Self { size: n, value }
    }

    /// 単位元
    pub fn id(size: usize) -> Self {
        Self { size, value: R(0) }
    }

    /// 逆元
    pub fn inv(self) -> Self {
        let Self { size, value } = self;
        let value = match value {
            R(x) => R(if x == 0 { 0 } else { size - x }),
            S(_) => value,
        };
        Self { size, value }
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

impl_algebra!(Composition; set: Dihedral;
    op: |_, a: Dihedral, b: Dihedral| a.compose(b);
    id: |s: &Self| Dihedral::id(s.0);
    inv: |_, a: Dihedral| a.inv();
    assoc;
);
