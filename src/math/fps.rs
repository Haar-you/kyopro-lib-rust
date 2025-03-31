//! Formal Power Series
//!
//! # References
//! - <https://maspypy.com/%E5%A4%9A%E9%A0%85%E5%BC%8F%E3%83%BB%E5%BD%A2%E5%BC%8F%E7%9A%84%E3%81%B9%E3%81%8D%E7%B4%9A%E6%95%B0%E6%95%B0%E3%81%88%E4%B8%8A%E3%81%92%E3%81%A8%E3%81%AE%E5%AF%BE%E5%BF%9C%E4%BB%98%E3%81%91>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/inv_of_formal_power_series>
//! - <https://judge.yosupo.jp/problem/log_of_formal_power_series>
//! - <https://judge.yosupo.jp/problem/exp_of_formal_power_series>
//! - <https://judge.yosupo.jp/problem/pow_of_formal_power_series>

use crate::math::polynomial::{Polynomial, PolynomialOperator};
use crate::misc::is_none_or::*;
use crate::num::{const_modint::ConstModInt, ff::*};

/// 形式的冪級数
pub trait FPS {
    /// 多項式の型
    type Poly;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\frac{1}{f(x)}$の先頭$n$項を求める。
    fn fps_inv(&self, f: Self::Poly) -> Self::Poly;
    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\log (f(x))$の先頭$n$項を求める。
    fn fps_log(&self, f: Self::Poly) -> Self::Poly;
    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\exp (f(x))$の先頭$n$項を求める。
    fn fps_exp(&self, f: Self::Poly) -> Self::Poly;
    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$(f(x))^m$の先頭$n$項を求める。
    fn fps_pow(&self, f: Self::Poly, m: u64) -> Self::Poly;
}

impl<const P: u32, const PR: u32> FPS for PolynomialOperator<'_, P, PR> {
    type Poly = Polynomial<P>;

    fn fps_inv(&self, f: Self::Poly) -> Self::Poly {
        let f: Vec<_> = f.into();
        assert_ne!(f[0].value(), 0);
        let n = f.len();

        let mut t = 1;
        let mut ret = vec![f[0].inv()];
        ret.reserve(2 * n);

        while t <= n * 2 {
            let mut a = self.ntt.convolve_same(ret.clone());
            a.truncate(t);

            let c = f[0..t.min(n)].to_vec();
            let mut c = self.ntt.convolve(c, a);

            c.truncate(t);
            ret.truncate(t);

            ret.iter_mut().for_each(|x| *x *= ConstModInt::new(2));

            if ret.len() < c.len() {
                ret.resize(c.len().min(t), ConstModInt::new(0));
            }

            ret.iter_mut()
                .zip(c.into_iter())
                .for_each(|(ret, c)| *ret -= c);

            t <<= 1;
        }

        ret.resize(n, ConstModInt::new(0));
        ret.into()
    }

    fn fps_log(&self, f: Self::Poly) -> Self::Poly {
        assert_eq!(f.coeff_of(0).value(), 1);
        let n = f.len();
        let a = self.differentiate(f.clone());
        let b = self.fps_inv(f);
        let c = self.mul(a, b);
        let mut ret = self.integrate(c);
        ret.as_mut().resize(n, ConstModInt::new(0));
        ret
    }

    fn fps_exp(&self, f: Self::Poly) -> Self::Poly {
        let f: Vec<_> = f.into();
        let n = f.len();

        let mut t = 1;
        let mut b = Polynomial::constant(ConstModInt::new(1));

        while t <= n * 2 {
            t <<= 1;

            let mut temp = self.fps_log(b.clone());
            temp.as_mut().resize(t, ConstModInt::new(0));

            let mut temp: Vec<_> = temp.into();
            temp.iter_mut().for_each(|x| *x = -*x);
            temp[0] += ConstModInt::new(1);

            temp.iter_mut()
                .zip(f.iter())
                .take(t.min(n))
                .for_each(|(temp, f)| {
                    *temp += *f;
                });

            b = self.mul(b, temp.into());
            b.as_mut().resize(t, ConstModInt::new(0));
        }

        b.as_mut().resize(n, ConstModInt::new(0));
        b
    }

    fn fps_pow(&self, f: Self::Poly, m: u64) -> Self::Poly {
        if m == 0 {
            let mut f: Vec<_> = f.into();
            f.fill(ConstModInt::new(0));
            f[0] = ConstModInt::new(1);
            return f.into();
        }
        if m == 1 {
            return f;
        }

        let n = f.len();
        let mut k = 0;
        while k < n {
            if f.coeff_of(k).value() != 0 {
                break;
            }
            k += 1;
        }

        if k >= n {
            return f;
        }

        if k.checked_mul(m as usize).is_none_or(|x| x >= n) {
            return vec![ConstModInt::new(0); n].into();
        }

        let a = f.coeff_of(k);

        let ret = self.shift_lower(f, k);
        let ret = self.scale(ret, a.inv());
        let ret = self.scale(self.fps_log(ret), m.into());
        let ret = self.fps_exp(ret);
        let ret = self.scale(ret, a.pow(m));
        let ret = self.shift_higher(ret, m as usize * k);

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::ntt::*;
    use crate::math::polynomial::*;

    #[test]
    fn test_inv() {
        let ntt = NTT998244353::new();
        let po = PolynomialOperator::new(&ntt);

        let a: Vec<u32> = vec![5, 4, 3, 2, 1];
        let b = po.fps_inv(a.clone().into());

        assert_eq!(
            po.mul(a.into(), b).get_until(5),
            Polynomial::constant(1_u32.into())
        );
    }

    #[test]
    fn test_log() {
        let ntt = NTT998244353::new();
        let po = PolynomialOperator::new(&ntt);

        let a: Vec<u32> = vec![1, 1, 499122179, 166374064, 291154613];
        let b = po.fps_log(a.clone().into());

        assert_eq!(b, vec![0, 1, 2, 3, 4].into());
    }

    #[test]
    fn test_exp() {
        let ntt = NTT998244353::new();
        let po = PolynomialOperator::new(&ntt);

        let a: Vec<u32> = vec![0, 1, 2, 3, 4];
        let b = po.fps_exp(a.clone().into());
        let b = po.fps_log(b);

        assert_eq!(b, a.into());
    }

    #[test]
    fn test_pow() {
        let ntt = NTT998244353::new();
        let po = PolynomialOperator::new(&ntt);

        let a: Vec<u32> = vec![0, 0, 9, 2];
        let b = po.fps_pow(a.clone().into(), 3);
        assert_eq!(b, vec![0, 0, 0, 0].into());

        let a: Vec<u32> = vec![1, 1];
        let b = po.fps_pow(a.clone().into(), 2);
        assert_eq!(b, vec![1, 2].into());

        let a: Vec<u32> = vec![0, 0];
        let b = po.fps_pow(a.clone().into(), 0);
        dbg!(b);
    }
}
