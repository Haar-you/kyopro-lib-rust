//! 集合冪級数
//!
//! # References
//! - <https://maspypy.com/category/%e9%9b%86%e5%90%88%e3%81%b9%e3%81%8d%e7%b4%9a%e6%95%b0>
//! - <https://suisen-kyopro.hatenablog.com/entry/2023/04/07/041318>

pub mod exp;
pub mod inv;
pub mod log;

use std::ops::{Index, IndexMut, Mul};

use crate::math::convolution::subset::subset_convolution;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::ConstModInt;

/// 集合冪級数
#[derive(Clone, PartialEq, Eq)]
pub struct SetPowerSeries<P: PrimeMod> {
    data: Vec<ConstModInt<P>>,
}

impl<P: PrimeMod> SetPowerSeries<P> {
    /// $n$変数からなる集合冪級数を返す。
    pub fn new(n: usize) -> Self {
        assert!(n.is_power_of_two());

        Self {
            data: vec![0.into(); n],
        }
    }

    /// 内部の`Vec`を返す。
    pub fn into_inner(self) -> Vec<ConstModInt<P>> {
        self.data
    }

    /// 内部の`Vec`の長さを返す。
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<P: PrimeMod> Mul for SetPowerSeries<P> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            data: subset_convolution(self.data, rhs.data),
        }
    }
}

impl<P, T> TryFrom<Vec<T>> for SetPowerSeries<P>
where
    P: PrimeMod,
    T: Into<ConstModInt<P>>,
{
    type Error = &'static str;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.len().is_power_of_two() {
            Ok(Self {
                data: value.into_iter().map(Into::into).collect(),
            })
        } else {
            Err("長さが2のべき乗ではない。")
        }
    }
}

impl<P: PrimeMod> Index<usize> for SetPowerSeries<P> {
    type Output = ConstModInt<P>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<P: PrimeMod> IndexMut<usize> for SetPowerSeries<P> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<P: PrimeMod> AsRef<[ConstModInt<P>]> for SetPowerSeries<P> {
    fn as_ref(&self) -> &[ConstModInt<P>] {
        &self.data
    }
}
