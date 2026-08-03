//! 集合冪級数の指数関数

use crate::math::convolution::subset::subset_convolution;
use crate::math::prime_mod::PrimeMod;
use crate::math::set_power_series::SetPowerSeries;
use crate::num::ff::ZZElem;

/// 集合冪級数の指数関数
pub trait SetPowerSeriesExp {
    /// 戻り値の型
    type Output;

    /// 集合冪級数の指数関数を求める。
    fn exp(self) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> SetPowerSeriesExp for SetPowerSeries<P> {
    type Output = Self;

    /// 集合冪級数の指数関数を求める。
    ///
    /// 定数項が$0$でないとき、`Err`を返す。
    ///
    /// **Time complexity** $O(n^2 2^n)$
    fn exp(self) -> Result<Self::Output, &'static str> {
        let a = self.data;

        if a[0].value() != 0 {
            return Err("定数項は`0`でなければならない。");
        }

        let n = a.len();

        let mut ret = vec![0.into(); n];
        ret[0] = 1.into();

        let mut k = 1;
        while k < n {
            let f = ret[0..k].to_vec();
            let g = a[k..k * 2].to_vec();

            let h = subset_convolution(f, g);

            ret[k..2 * k].copy_from_slice(&h);

            k <<= 1;
        }

        Ok(Self { data: ret })
    }
}
