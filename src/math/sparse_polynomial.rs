//! 疎な多項式
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::*;

/// 疎な多項式
#[derive(Clone, Debug, Default)]
pub struct SparsePolynomial<P: PrimeMod> {
    pub(crate) data: Vec<(usize, ConstModInt<P>)>,
}

impl<P: PrimeMod> SparsePolynomial<P> {
    /// 零多項式を得る。
    pub fn zero() -> Self {
        Self { data: vec![] }
    }

    /// 定数項のみをもつ多項式を生成する。
    pub fn constant(a: ConstModInt<P>) -> Self {
        if a.value() == 0 {
            Self::zero()
        } else {
            Self { data: vec![(0, a)] }
        }
    }

    pub fn add(&mut self, i: usize, x: ConstModInt<P>) {
        for (j, y) in self.data.iter_mut() {
            if i == *j {
                *y += x;
                return;
            }
        }
        self.data.push((i, x));
    }

    pub fn from_vec(mut a: Vec<(usize, ConstModInt<P>)>) -> Self {
        a.sort_by_key(|a| a.0);
        Self { data: a }
    }

    /// $x^i$の係数を得る。
    pub fn coeff_of(&self, i: usize) -> ConstModInt<P> {
        self.data
            .iter()
            .find(|(j, _)| *j == i)
            .map_or(0.into(), |(_, x)| *x)
    }

    /// 多項式を微分する。
    pub fn differential(&mut self) {
        let a = self
            .data
            .iter()
            .filter_map(|&(i, x)| {
                if i == 0 {
                    None
                } else {
                    Some((i - 1, x * i.into()))
                }
            })
            .collect();

        self.data = a;
    }

    /// 多項式を積分する。
    pub fn integral(&mut self) {
        let a = self
            .data
            .iter()
            .map(|&(i, x)| (i + 1, x * ConstModInt::new(i as u32 + 1).inv()))
            .collect();

        self.data = a;
    }

    /// 多項式を`k`倍する。
    pub fn scale(&mut self, k: ConstModInt<P>) {
        self.data.iter_mut().for_each(|(_, x)| *x *= k);
    }

    /// 係数を`k`次だけ低次側にずらす。ただし、負の次数の項は無視する。
    pub fn shift_lower(&mut self, k: usize) {
        self.data = self
            .data
            .iter()
            .filter_map(|&(i, x)| if i < k { None } else { Some((i - k, x)) })
            .collect();
    }

    /// 係数を`k`次だけ高次側にずらす。
    pub fn shift_higher(&mut self, k: usize) {
        self.data.iter_mut().for_each(|(i, _)| *i += k);
    }
}
