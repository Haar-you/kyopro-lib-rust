//! $\sum_{i=0}^{\infty} r^ii^d$
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/sum_of_exponential_times_polynomial_limit>

use crate::num::ff::*;

/// $\sum_{i=0}^{\infty} r^ii^d$
///
/// **Time Complexity** $O(d \log p)$
pub fn sum_of_exponential_times_polynomial_limit<Modulo: FF>(
    r: Modulo::Element,
    d: u64,
    m: Modulo,
) -> Modulo::Element
where
    Modulo::Element: FFElem + Copy,
{
    let mut ret = m.from_u64(0);
    let mut r_pow = m.from_u64(1);
    let mut s = vec![m.from_u64(0); d as usize + 1];
    let mut invs = vec![m.from_u64(0); d as usize + 2];
    let p = m.modulo();

    invs[1] = m.from_u64(1);

    for i in 2..=d + 1 {
        invs[i as usize] = m.from_u64(p as u64 / i) * -invs[(p as u64 % i) as usize];
    }

    for i in 0..=d as usize {
        if i > 0 {
            let x = s[i - 1];
            s[i] += x;
        }
        s[i] += m.from_u64(i as u64).pow(d) * r_pow;
        r_pow *= r;
    }

    let mut t = m.from_u64(1);

    for i in 0..=d {
        ret += t * s[(d - i) as usize];
        t *= invs[i as usize + 1] * -r * m.from_u64(d + 1 - i);
    }

    ret * (m.from_u64(1) - r).pow(d + 1).inv()
}
