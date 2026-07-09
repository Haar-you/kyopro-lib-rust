//! mod mでの逆元

use crate::math::gcd_lcm::GcdLcm;

/// $ax \equiv 1 \pmod m$を満たす$x$を求める。
///
/// $m$は$0 < m < 2^{32}$の範囲に収めること。
///
/// **Time complexity** $O(\log m)$
pub fn mod_inv(mut a: u64, m: u64) -> Option<u64> {
    assert!(
        0 < m && m <= 0xFFFFFFFF,
        "Violated 0 < m <= 0xFFFFFFFF (m = {m})"
    );
    if a >= m {
        a %= m;
    }

    if a.gcd(m) != 1 {
        return None;
    }

    let mut b = m;
    let mut u = 1;
    let mut v = 0;

    while b > 0 {
        let t = a / b;

        a -= t * b;
        (a, b) = (b, a);

        if u < t * v {
            u += m - (t * v) % m;
            if u >= m {
                u -= m;
            }
        } else {
            u -= t * v;
        }
        (u, v) = (v, u);
    }

    Some(u)
}
