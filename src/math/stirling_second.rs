//! 第二種スターリング数$S(n, 0), \dots, S(n, n)$を列挙する。
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/stirling_number_of_the_second_kind>

use crate::math::ntt::NTT;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::*;

/// 第二種スターリング数$S(n, 0), \dots, S(n, n)$を列挙する。
pub fn stirling_second<P: PrimeMod>(n: usize) -> Vec<ConstModInt<P>> {
    let ntt = NTT::<P>::new();
    let ff = ConstModIntBuilder::new();
    let mut a = vec![ff.from_u64(0); n + 1];
    let mut b = vec![ff.from_u64(0); n + 1];
    let mut m = vec![0; n + 1];

    for i in 2..=n {
        if m[i] == 0 {
            for j in (2 * i..=n).step_by(i) {
                m[j] = i;
            }
        }
    }

    for i in 0..=n {
        if m[i] == 0 {
            a[i] = ff.from_u64(i as u64).pow(n as u64);
        } else {
            a[i] = a[m[i]] * a[i / m[i]];
        }
    }

    let mut f = (1..=n)
        .fold(ff.from_u64(1), |x, y| x * ff.from_u64(y as u64))
        .inv();

    for i in (0..=n).rev() {
        a[i] *= f;
        b[i] = f;
        f *= ff.from_u64(i as u64);

        if i % 2 == 1 {
            b[i] = -b[i];
        }
    }

    let mut ret = ntt.convolve(a, b);
    ret.truncate(n + 1);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::{prime_mod::Prime, stirling_second_table::stirling_second_table};

    type P = Prime<998244353>;

    #[test]
    fn test() {
        let n = 100;
        let ans = stirling_second_table(n, ConstModIntBuilder::new());

        for i in 0..=n {
            assert_eq!(stirling_second::<P>(i), ans[i][0..=i]);
        }
    }
}
