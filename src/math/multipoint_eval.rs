//! 多項式の多点評価

use crate::math::polynomial::{Polynomial, PolynomialOperator};
use crate::num::const_modint::ConstModInt;

/// 多項式の多点評価
pub trait MultipointEval {
    /// 多項式の型
    type Poly;
    /// 多項式の係数の型
    type Value;

    /// 多項式の多点評価
    ///
    /// 多項式$f(x)$に値$p_0, p_1, \cdots, p_m$を代入した結果$f(p_0), f(p_1), \cdots, f(p_m)$を求める。
    fn multipoint_eval(&self, a: Self::Poly, p: Vec<Self::Value>) -> Vec<Self::Value>;
}

impl<const P: u32, const PR: u32> MultipointEval for PolynomialOperator<'_, P, PR> {
    type Poly = Polynomial<P>;
    type Value = ConstModInt<P>;

    fn multipoint_eval(&self, a: Self::Poly, p: Vec<Self::Value>) -> Vec<Self::Value> {
        let m = p.len();

        let mut k = 1;
        while k < m {
            k *= 2;
        }

        let mut f = vec![Polynomial::constant(ConstModInt::new(1)); k * 2];
        for i in 0..m {
            f[i + k] = Polynomial::from(vec![-p[i], ConstModInt::new(1)]);
        }
        for i in (1..k).rev() {
            f[i] = self.mul(f[i << 1].clone(), f[(i << 1) | 1].clone());
        }

        f[1] = self.divmod(a, f[1].clone()).1;

        for i in 2..k + m {
            f[i] = self.divmod(f[i >> 1].clone(), f[i].clone()).1;
        }

        f.into_iter()
            .skip(k)
            .take(m)
            .map(|v| v.coeff_of(0))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{ntt::*, polynomial::*};
    use crate::num::const_modint::*;
    use rand::Rng;

    #[test]
    fn test() {
        const M: u32 = 998244353;

        let ff = ConstModIntBuilder::<M>;
        let ntt = NTT::<M, 3>::new();
        let po = PolynomialOperator::new(&ntt);

        let mut rng = rand::thread_rng();

        let n = 100;
        let a = (0..n)
            .map(|_| ff.from_u64(rng.gen_range(0..M) as u64))
            .collect::<Vec<_>>();
        let a = Polynomial::from(a);

        let m = 100;
        let p = (0..m)
            .map(|_| ff.from_u64(rng.gen_range(0..M) as u64))
            .collect::<Vec<_>>();

        let ans = p.iter().map(|p| a.eval(*p)).collect::<Vec<_>>();
        let res = po.multipoint_eval(a, p);

        assert_eq!(res, ans);
    }
}
