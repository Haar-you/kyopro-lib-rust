//! 二面体群
pub use crate::algebra::traits::*;

/// 二面体群の元
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DihedralValue {
    R(usize),
    S(usize),
}

/// 二面体群
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dihedral {
    k: usize,
}

impl Dihedral {
    /// `Dihedral`を生成する。
    pub fn new(k: usize) -> Self {
        Self { k }
    }
}

impl Set for Dihedral {
    type Element = DihedralValue;
}

impl BinaryOp for Dihedral {
    fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        match (a, b) {
            (DihedralValue::R(x), DihedralValue::R(y)) => DihedralValue::R((x + y) % self.k),
            (DihedralValue::R(x), DihedralValue::S(y)) => {
                DihedralValue::S((self.k + y - x) % self.k)
            }
            (DihedralValue::S(x), DihedralValue::R(y)) => DihedralValue::S((x + y) % self.k),
            (DihedralValue::S(x), DihedralValue::S(y)) => {
                DihedralValue::R((self.k + y - x) % self.k)
            }
        }
    }
}

impl Identity for Dihedral {
    fn id(&self) -> Self::Element {
        DihedralValue::R(0)
    }
}

impl Inverse for Dihedral {
    fn inv(&self, a: Self::Element) -> Self::Element {
        match a {
            DihedralValue::R(x) => DihedralValue::R(if x == 0 { 0 } else { self.k - 1 }),
            DihedralValue::S(_) => a,
        }
    }
}

impl Associative for Dihedral {}
