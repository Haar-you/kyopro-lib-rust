//! 疎な多項式
use std::collections::HashMap;

use crate::num::const_modint::*;

/// 疎な多項式
#[derive(Clone, Debug, Default)]
pub struct SparsePolynomial<const P: u32> {
    pub(crate) data: HashMap<usize, ConstModInt<P>>,
}

impl<const P: u32> SparsePolynomial<P> {
    /// 零多項式を得る。
    pub fn zero() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// 定数項のみをもつ多項式を生成する。
    pub fn constant(a: ConstModInt<P>) -> Self {
        if a.value() == 0 {
            Self::zero()
        } else {
            let mut data = HashMap::new();
            data.insert(0, a);
            Self { data }
        }
    }

    pub fn add(&mut self, i: usize, x: ConstModInt<P>) {
        *self.data.entry(i).or_default() += x;
    }

    pub fn from_vec(a: Vec<(usize, ConstModInt<P>)>) -> Self {
        let mut data = HashMap::new();

        for (i, x) in a {
            *data.entry(i).or_default() += x;
        }

        Self { data }
    }

    /// $x^i$の係数を得る。
    pub fn coeff_of(&self, i: usize) -> ConstModInt<P> {
        self.data.get(&i).map_or(0.into(), |a| *a)
    }

    // pub fn differential(&mut self) {

    // }
}
