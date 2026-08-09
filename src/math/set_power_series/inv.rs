//! 集合冪級数の逆数

use crate::math::convolution::subset::subset_convolution;
use crate::math::prime_mod::PrimeMod;
use crate::math::set_power_series::SetPowerSeries;
use crate::num::ff::{FFElem, ZZElem};

/// 集合冪級数の逆数
pub trait SetPowerSeriesInv {
    /// 戻り値の型
    type Output;

    /// 集合冪級数の逆数を求める。
    fn inv(self) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> SetPowerSeriesInv for SetPowerSeries<P> {
    type Output = Self;

    /// 集合冪級数の逆数を求める。
    ///
    /// 定数項が$0$のとき、`Err`を返す。
    ///
    /// **Time complexity** $O(n^2 2^n)$
    fn inv(self) -> Result<Self::Output, &'static str> {
        let a = self.data;

        if a[0].value() == 0 {
            return Err("定数項は`0`であってはならない。");
        }

        let n = a.len();

        let mut ret = vec![0.into(); n];
        ret[0] = a[0].inv();

        let mut k = 1;
        while k < n {
            let f = ret[0..k].to_vec();
            let g = a[k..k * 2].to_vec();

            let h = subset_convolution(f.clone(), f);
            let h = subset_convolution(h, g);

            for i in 0..k {
                ret[k + i] = -h[i];
            }

            k <<= 1;
        }

        Ok(Self { data: ret })
    }
}
