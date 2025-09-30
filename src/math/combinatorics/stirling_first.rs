//! 符号付き第一種スターリング数$s(n, 0), \dots, s(n, n)$を列挙する。
//!
//! $s(n,k)$ は $$x(x-1)\dots (x-(n-1)) = \sum_{k=0}^n s(n,k) x^k$$を満たす。
use crate::math::convolution::ntt::NTT;
use crate::math::polynomial::{polynomial_taylor_shift::*, Polynomial};
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::*;

/// 符号付き第一種スターリング数$s(n, 0), \dots, s(n, n)$を列挙する。
///
/// **Time complexity** $O(n \log n)$
pub fn stirling_first<P: PrimeMod>(n: usize) -> Vec<ConstModInt<P>> {
    let ff = ConstModIntBuilder::new();

    let mut ret = Polynomial::<P>::from(vec![1]);
    let ntt = NTT::<P>::new();

    let mut t: usize = 0;
    let mut check = false;

    for i in (0..usize::BITS).rev() {
        if check {
            let s = ret.clone().taylor_shift(-ff.from_u64(t as u64));
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
    use crate::math::prime_mod::Prime;

    use super::*;

    type P = Prime<998244353>;

    #[test]
    fn test() {
        let ff = ConstModIntBuilder::<P>::new();

        let n = 100;
        let mut ans = Polynomial::from(vec![ff.from_u64(1)]);

        for i in 1..=n {
            let res = stirling_first::<P>(i);

            ans *= Polynomial::from(vec![-ff.from_u64(i as u64 - 1), 1.into()]);

            assert_eq!(res, ans.as_ref());
        }
    }
}
