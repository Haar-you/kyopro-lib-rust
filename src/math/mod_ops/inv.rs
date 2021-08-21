use crate::math::gcd_lcm::GcdLcm;
use std::mem::swap;

pub fn mod_inv(mut a: u64, m: u64) -> Option<u64> {
    if a.gcd(m) != 1 {
        return None;
    }

    let mut b = m;
    let mut u = 1;
    let mut v = 0;

    while b > 0 {
        let t = a / b;

        a -= t * b;
        swap(&mut a, &mut b);

        if u < t * v {
            u += m - (t * v) % m;
            u %= m;
        } else {
            u -= t * v;
        }
        swap(&mut u, &mut v);
    }

    Some(u)
}
