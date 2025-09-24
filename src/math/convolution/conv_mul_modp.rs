//! 素数$P$に対して、$c_k = \sum_{i \times j = k \pmod P} a_i b_j$を満たす$c$を求める。
use std::iter::successors;

use crate::math::prime_mod::PrimeMod;
use crate::sort_with;
use crate::{
    math::{convolution::ntt::NTT, primitive_root::primitive_root},
    num::const_modint::ConstModInt,
};

/// 素数$M$に対して、$c_k = \sum_{i \times j = k \pmod M} a_i b_j$を満たす$c$を求める。
pub fn convolution_mul_modp<P: PrimeMod>(
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
    use crate::{
        iter::collect::CollectVec,
        math::prime_mod::Prime,
        num::{const_modint::ConstModIntBuilder, ff::FF},
    };

    use super::*;
    use rand::Rng;

    type P = Prime<998244353>;

    #[test]
    fn test() {
        let p = 1009;
        let modulo = ConstModIntBuilder::<P>::new();
        let mut rng = rand::thread_rng();

        let a = (0..p)
            .map(|_| modulo.from_u64(rng.gen::<u64>()))
            .collect_vec();
        let b = (0..p)
            .map(|_| modulo.from_u64(rng.gen::<u64>()))
            .collect_vec();

        let mut ans = vec![modulo.from_u64(0); p];
        for i in 0..p {
            for j in 0..p {
                ans[i * j % p] += a[i] * b[j];
            }
        }

        let res = convolution_mul_modp::<P>(a, b);

        assert_eq!(ans, res);
    }
}
