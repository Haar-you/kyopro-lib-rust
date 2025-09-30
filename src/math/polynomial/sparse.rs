//! 疎な多項式
use std::collections::BTreeMap;

use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::*;

/// 疎な多項式
#[derive(Clone, Debug, Default)]
pub struct SparsePolynomial<P: PrimeMod> {
    data: BTreeMap<usize, ConstModInt<P>>,
}

impl<P: PrimeMod> SparsePolynomial<P> {
    /// 零多項式を得る。
    pub fn zero() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    /// 定数項のみをもつ多項式を生成する。
    pub fn constant(a: ConstModInt<P>) -> Self {
        if a.value() == 0 {
            Self::zero()
        } else {
            Self {
                data: BTreeMap::from([(0, a)]),
            }
        }
    }

    /// $k x^i$となる項を足す。
    pub fn add_one_term<T>(&mut self, i: usize, k: T)
    where
        ConstModInt<P>: From<T>,
    {
        *self.data.entry(i).or_default() += k.into();
    }

    /// $x^i$の係数を得る。
    pub fn coeff_of(&self, i: usize) -> ConstModInt<P> {
        self.data
            .iter()
            .find(|(&j, _)| j == i)
            .map_or(0.into(), |(_, x)| *x)
    }

    /// 多項式を微分する。
    pub fn differential(&mut self) {
        let a = self
            .data
            .iter()
            .filter_map(|(&i, &x)| {
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
            .map(|(&i, &x)| (i + 1, x * ConstModInt::new(i as u32 + 1).inv()))
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
            .filter_map(|(&i, &x)| if i < k { None } else { Some((i - k, x)) })
            .collect();
    }

    /// 係数を`k`次だけ高次側にずらす。
    pub fn shift_higher(&mut self, k: usize) {
        self.data = self.data.iter().map(|(&i, &x)| (i + k, x)).collect();
    }

    /// 項の次数と係数のペアへのイテレータを返す。
    pub fn iter(&self) -> impl Iterator<Item = (&usize, &ConstModInt<P>)> {
        self.data.iter()
    }
}

impl<P: PrimeMod, T> From<Vec<(usize, T)>> for SparsePolynomial<P>
where
    ConstModInt<P>: From<T>,
{
    fn from(value: Vec<(usize, T)>) -> Self {
        let mut ret = Self::zero();
        for (i, x) in value {
            ret.add_one_term(i, x);
        }
        ret
    }
}
