//! 形式的冪級数の平方根
use crate::math::fps::inv::FpsInv;
use crate::math::mod_ops::sqrt::sqrt_mod;
use crate::math::polynomial::Polynomial;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::ConstModInt;
use crate::num::ff::*;

/// 形式的冪級数の平方根
pub trait FpsSqrt {
    /// 戻り値の型
    type Output;

    /// 形式的冪級数の平方根を求める。
    fn fps_sqrt(self) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> FpsSqrt for Polynomial<P> {
    type Output = Self;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\sqrt{f(x)}$の先頭$n$項を求める。
    ///
    /// 平方根が存在しないとき、`Err`を返す。
    ///
    /// **Time complexity** $O(N \log N)$
    fn fps_sqrt(self) -> Result<Self::Output, &'static str> {
        let f: Vec<_> = self.into();

        let n = f.len();
        let k = f
            .iter()
            .enumerate()
            .find(|&(_, &x)| x.value() != 0)
            .map_or(n, |(k, _)| k);

        if k == n {
            return Ok(f.into());
        }
        if k % 2 == 1 {
            return Err("最小次数が偶数ではない。");
        }

        let x = sqrt_mod(f[k].value() as u64, P::PRIME_NUM as u64)
            .first()
            .copied()
            .ok_or("最小次数項の係数に平方根が存在しない。")?;
        let m = n - k;

        let half = ConstModInt::new(2).inv();
        let mut t = 1;
        let mut ret = vec![ConstModInt::new(x as u32)];

        loop {
            let mut f = f[k..k + t.min(m)].to_vec();
            f.resize(t, 0.into());

            ret.resize(t, 0.into());
            let h = Self::from(ret.clone()).fps_inv()?;
            let h = Self::from(f) * h;
            let h: Vec<_> = h.into();

            for (x, y) in ret.iter_mut().zip(h) {
                *x = (*x + y) * half;
            }

            if t >= m {
                break;
            }

            t <<= 1;
        }

        ret.resize(n, 0.into());
        let mut ret: Self = ret.into();
        ret.shift_higher(k / 2);
        Ok(ret)
    }
}
