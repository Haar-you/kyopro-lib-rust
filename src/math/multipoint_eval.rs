use crate::math::polynomial::{Polynomial, PolynomialOperator};
use crate::num::const_modint::ConstModInt;

pub fn multipoint_eval<const P: u32>(
    a: Polynomial<P>,
    p: Vec<ConstModInt<P>>,
    po: &PolynomialOperator<P>,
) -> Vec<ConstModInt<P>> {
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
        f[i] = po.mul(f[i << 1].clone(), f[i << 1 | 1].clone());
    }

    f[1] = po.divmod(a, f[1].clone()).1;

    for i in 2..k + m {
        f[i] = po.divmod(f[i >> 1].clone(), f[i].clone()).1;
    }

    f.into_iter()
        .skip(k)
        .take(m)
        .map(|v| v.coeff_of(0))
        .collect()
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

        let size = 1 << 20;

        let ff = ConstModIntBuilder::<M>;
        let ntt = NTT::<M>::new(3, size);
        let po = PolynomialOperator::<M>::new(&ntt);

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
        let res = multipoint_eval(a, p, &po);

        assert_eq!(res, ans);
    }
}
