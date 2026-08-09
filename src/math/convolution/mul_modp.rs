//! 添字積$\pmod p$畳み込み
use std::iter::successors;

use crate::math::convolution::ntt::NTT;
use crate::math::prime_mod::PrimeMod;
use crate::math::primitive_root::primitive_root;
use crate::num::const_modint::ConstModInt;
use crate::sort_with;

/// 素数$P$に対して、$c_k = \sum_{i \times j = k \pmod P} a_i b_j$を満たす$c$を求める。
///
/// # Requirements
/// `f.len()` = `g.len()`
pub fn mul_modp_convolution<P: PrimeMod>(
    mut a: Vec<ConstModInt<P>>,
    mut b: Vec<ConstModInt<P>>,
) -> Vec<ConstModInt<P>> {
    assert_eq!(a.len(), b.len());
    let p = a.len();
    let p_root = primitive_root(p as u32) as usize;

    let ntt = NTT::<P>::new();

    let mut index = vec![0; p];
    successors(Some(1), |&s| Some(s * p_root % p))
        .take(p)
        .enumerate()
        .for_each(|(i, s)| index[s] = i);

    let mut zero = a[0] * b[0];
    for i in 1..p {
        zero += a[0] * b[i] + a[i] * b[0];
    }

    a[0] = 0.into();
    b[0] = 0.into();

    sort_with!(|&i, &j| index[i].cmp(&index[j]), a, b);

    let c = ntt.convolve(a, b);

    let mut ret = vec![0.into(); p];

    successors(Some(1), |&s| Some(s * p_root % p))
        .zip(c)
        .for_each(|(s, x)| ret[s] += x);

    ret[0] = zero;

    ret
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;
    use crate::iter::collect::CollectVec;
    use crate::math::prime_mod::Prime;
    use crate::num::const_modint::ConstModIntBuilder;
    use crate::num::ff::*;

    type P = Prime<998244353>;

    #[test]
    fn test() {
        let p = 1009;
        let modulo = ConstModIntBuilder::<P>::new();
        let mut rng = rand::rng();

        let a = std::iter::repeat_with(|| modulo.from_u64(rng.random::<u64>()))
            .take(p)
            .collect_vec();
        let b = std::iter::repeat_with(|| modulo.from_u64(rng.random::<u64>()))
            .take(p)
            .collect_vec();

        let mut ans = vec![modulo.from_u64(0); p];
        for i in 0..p {
            for j in 0..p {
                ans[i * j % p] += a[i] * b[j];
            }
        }

        let res = mul_modp_convolution::<P>(a, b);

        assert_eq!(ans, res);
    }
}
