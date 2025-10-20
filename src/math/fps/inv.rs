//! 形式的冪級数の逆数
use crate::math::polynomial::Polynomial;
use crate::math::prime_mod::PrimeMod;
use crate::num::ff::*;

/// 形式的冪級数の逆数
pub trait FpsInv {
    /// 戻り値の型
    type Output;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\frac{1}{f(x)}$の先頭$n$項を求める。
    fn fps_inv(self) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> FpsInv for Polynomial<P> {
    type Output = Self;

    fn fps_inv(self) -> Result<Self::Output, &'static str> {
        let f: Vec<_> = self.into();

        if f[0].value() == 0 {
            return Err("定数項が`0`の形式的べき級数の逆数を計算しようとした。");
        }
        let n = f.len();

        let mut t = 1;
        let mut ret = vec![f[0].inv()];
        ret.reserve(2 * n);

        loop {
            let mut f = f[0..(2 * t).min(n)].to_vec();
            f.resize(2 * t, 0.into());
            Self::NTT.ntt(&mut f);

            let mut g = ret.clone();
            g.resize(2 * t, 0.into());
            Self::NTT.ntt(&mut g);

            for (f, g) in f.iter_mut().zip(g.iter()) {
                *f *= *g;
            }
            Self::NTT.intt(&mut f);

            let h = f;

            let mut h = h[t..2 * t].to_vec();
            h.resize(2 * t, 0.into());
            Self::NTT.ntt(&mut h);

            for (h, g) in h.iter_mut().zip(g.iter()) {
                *h *= *g;
            }
            Self::NTT.intt(&mut h);

            let g = h;

            ret.resize(2 * t, 0.into());

            for (ret, x) in ret.iter_mut().skip(t).zip(g.into_iter().take(t)) {
                *ret = -x;
            }

            t <<= 1;

            if t >= n {
                break;
            }
        }

        ret.truncate(n);
        Ok(ret.into())
    }
}
