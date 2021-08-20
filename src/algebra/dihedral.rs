use crate::algebra::traits::*;

#[derive(Clone, Debug, Copy)]
pub enum DihedralValue {
    R(usize),
    S(usize),
}

#[derive(Clone)]
pub struct Dihedral {
    k: usize,
}

impl Dihedral {
    pub fn new(k: usize) -> Self {
        Self { k }
    }
}

impl AlgeStruct for Dihedral {
    type Output = DihedralValue;
}

impl BinaryOp for Dihedral {
    fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
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
    fn id(&self) -> Self::Output {
        DihedralValue::R(0)
    }
}

impl Inverse for Dihedral {
    fn inv(&self, a: Self::Output) -> Self::Output {
        match a {
            DihedralValue::R(x) => DihedralValue::R(if x == 0 { 0 } else { self.k - 1 }),
            DihedralValue::S(_) => a,
        }
    }
}
