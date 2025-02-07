//! 素数mod pでの逆元

use std::mem::swap;

/// 素数mod pでの逆元
///
/// **Time complexity** $O(\log p)$
#[inline]
pub fn mod_inv_p(mut a: u64, p: u64) -> u64 {
    let mut b = p;
    let mut u = 1;
    let mut v = 0;

    while b > 0 {
        let t = a / b;

        a -= t * b;
        swap(&mut a, &mut b);

        if u < t * v {
            u += p - (t * v) % p;
            if u >= p {
                u -= p;
            }
        } else {
            u -= t * v;
        }
        swap(&mut u, &mut v);
    }

    u
}
