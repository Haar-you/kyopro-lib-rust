//! 二面体群 $D_n$
//!
//! # Problems
//! - <https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0390>

pub use crate::algebra::traits::*;

/// 二面体群$D_n$の元
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DihedralValue {
    /// 回転
    R(usize),
    /// 鏡映
    S(usize),
}

use DihedralValue::{R, S};

/// 二面体群$D_n$
#[derive(Clone, Copy, Debug)]
pub struct Dihedral {
    n: usize,
    value: DihedralValue,
}

impl PartialEq for Dihedral {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self { n: 0, value: R(0) }, Self { value: R(0), .. }) => true,
            (Self { value: R(0), .. }, Self { n: 0, value: R(0) }) => true,
            _ => self.n == other.n && self.value == other.value,
        }
    }
}

impl Dihedral {
    fn _op(a: Self, b: Self) -> Self {
        match (a.n, b.n) {
            (0, _) => return b,
            (_, 0) => return a,
            _ => {}
        }

        let n = a.n;
        assert_eq!(b.n, a.n);

        let value = match (a.value, b.value) {
            (R(x), R(y)) => R((x + y) % n),
            (R(x), S(y)) => S((n + y - x) % n),
            (S(x), R(y)) => S((x + y) % n),
            (S(x), S(y)) => R((n + y - x) % n),
        };

        Self { n, value }
    }

    /// $D_n$の回転を表す元$R_i$を返す。
    pub fn r(i: usize, n: usize) -> Self {
        assert!(n > 0);
        assert!(i < n);
        Self { n, value: R(i) }
    }

    /// $D_n$の鏡映を表す元$S_i$を返す。
    pub fn s(i: usize, n: usize) -> Self {
        assert!(n > 0);
        assert!(i < n);
        Self { n, value: S(i) }
    }
}

impl Set for Dihedral {}

impl BinaryOp for Dihedral {
    fn op(self, b: Self) -> Self {
        Self::_op(self, b)
    }
}

impl Identity for Dihedral {
    fn id() -> Self {
        Self { n: 0, value: R(0) }
    }
}

impl Inverse for Dihedral {
    fn inv(self) -> Self {
        let Self { n, value } = self;
        let value = if n != 0 {
            match value {
                R(x) => R(if x == 0 { 0 } else { n - x }),
                S(_) => value,
            }
        } else {
            value
        };
        Self { n, value }
    }
}

impl Associative for Dihedral {}
