//! 集合冪級数の対数

use crate::{
    math::{
        convolution::subset::subset_convolution,
        prime_mod::PrimeMod,
        set_power_series::{SetPowerSeries, inv::SetPowerSeriesInv},
    },
    num::ff::ZZElem,
};

/// 集合冪級数の対数
pub trait SetPowerSeriesLog {
    /// 戻り値の型
    type Output;

    /// 集合冪級数の対数を求める。
    fn log(self) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> SetPowerSeriesLog for SetPowerSeries<P> {
    type Output = Self;

    /// 集合冪級数の対数を求める。
    ///
    /// 定数項が$1$でないとき、`Err`を返す。
    ///
    /// **Time complexity** $O(n^2 2^n)$
    fn log(self) -> Result<Self::Output, &'static str> {
        let a = self.data;

        if a[0].value() != 1 {
            return Err("定数項は`1`でなければならない。");
        }

        let n = a.len();

        let mut ret = vec![0.into(); n];

        let mut k = 1;
        while k < n {
            let f = a[k..k * 2].to_vec();
            let g = Self {
                data: a[0..k].to_vec(),
            };
            let g = g.inv()?.data;

            let h = subset_convolution(f, g);

            ret[k..2 * k].copy_from_slice(&h);

            k <<= 1;
        }

        Ok(Self { data: ret })
    }
}
