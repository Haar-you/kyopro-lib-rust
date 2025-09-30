//! 多項式の多点評価

use crate::math::polynomial::Polynomial;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::ConstModInt;

/// 多項式の多点評価
pub trait MultipointEval {
    /// 多項式の係数の型
    type Value;

    /// 多項式の多点評価
    ///
    /// 多項式$f(x)$に値$p_0, p_1, \cdots, p_m$を代入した結果$f(p_0), f(p_1), \cdots, f(p_m)$を求める。
    fn multipoint_eval(self, p: Vec<Self::Value>) -> Vec<Self::Value>;
}

impl<P: PrimeMod> MultipointEval for Polynomial<P> {
    type Value = ConstModInt<P>;

    fn multipoint_eval(self, p: Vec<Self::Value>) -> Vec<Self::Value> {
        let m = p.len();

        let k = m.next_power_of_two();

        let mut f = vec![Self::constant(1.into()); k * 2];
        for i in 0..m {
            f[i + k] = vec![-p[i], 1.into()].into();
        }
        for i in (1..k).rev() {
            f[i] = f[i << 1].clone() * f[(i << 1) | 1].clone();
        }

        f[1] = self % f[1].clone();

        for i in 2..k + m {
            f[i] = f[i >> 1].clone() % std::mem::take(&mut f[i]);
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
    use crate::math::polynomial::*;
    use crate::math::prime_mod::Prime;
    use crate::num::const_modint::*;
    use rand::Rng;

    const M: u32 = 998244353;
    type P = Prime<M>;

    #[test]
    fn test() {
        let ff = ConstModIntBuilder::<P>::new();

        let mut rng = rand::thread_rng();

        let n = 100;
        let a = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..P::PRIME_NUM) as u64))
            .take(n)
            .collect::<Vec<_>>();
        let a = Polynomial::from(a);

        let m = 100;
        let p = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..M) as u64))
            .take(m)
            .collect::<Vec<_>>();

        let ans = p.iter().map(|p| a.eval(*p)).collect::<Vec<_>>();
        let res = a.multipoint_eval(p);

        assert_eq!(res, ans);
    }
}
