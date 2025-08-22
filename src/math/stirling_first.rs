//! 符号付き第一種スターリング数$s(n, 0), \dots, s(n, n)$を列挙する。
//!
//! $s(n,k)$ は $$x(x-1)\dots (x-(n-1)) = \sum_{k=0}^n s(n,k) x^k$$を満たす。
use crate::math::ntt::NTT;
use crate::math::polynomial::{Polynomial, PolynomialOperator};
use crate::math::polynomial_taylor_shift::*;
use crate::num::const_modint::*;

/// 符号付き第一種スターリング数$s(n, 0), \dots, s(n, n)$を列挙する。
///
/// **Time complexity** $O(n \log n)$
pub fn stirling_first<const P: u32, const PR: u32>(
    n: usize,
    ntt: &NTT<P, PR>,
) -> Vec<ConstModInt<P>> {
    let ff = ConstModIntBuilder;

    let mut ret = Polynomial::<P>::from(vec![1]);
    let op = PolynomialOperator::new(&ntt);

    let mut t: usize = 0;
    let mut check = false;

    for i in (0..usize::BITS).rev() {
        if check {
            let s = op.taylor_shift(ret.clone(), -ff.from_u64(t as u64));
            ret = ntt.convolve(ret.into(), s.into()).into();
            ret.as_mut().truncate(t * 2 + 1);
            t *= 2;
        }

        if (n & (1 << i)) != 0 {
            let a = vec![-ff.from_u64(t as u64), ff.from_u64(1)];
            ret = ntt.convolve(ret.into(), a).into();
            t += 1;

            check = true;
        }
    }

    ret.as_mut().truncate(n + 1);
    ret.into()
}

#[cfg(test)]
mod tests {
    use crate::math::ntt::NTT998244353;

    use super::*;

    #[test]
    fn test() {
        const M: u32 = 998244353;
        let ff = ConstModIntBuilder::<M>;
        let ntt = NTT998244353::new();
        let op = PolynomialOperator::new(&ntt);

        let n = 100;
        let mut ans = Polynomial::from(vec![ff.from_u64(1)]);

        for i in 1..=n {
            let res = stirling_first(i, &ntt);

            ans = op.mul(
                ans,
                Polynomial::from(vec![-ff.from_u64(i as u64 - 1), 1.into()]),
            );

            assert_eq!(res, ans.as_ref());
        }
    }
}
