//! mod mでの離散対数

use std::collections::HashMap;

use crate::math::gcd_lcm::GcdLcm;
use crate::math::mod_ops::inv::*;
use crate::math::mod_ops::pow::*;

/// $a^x \equiv b \pmod m$を満たす$x$を求める。
///
/// $m$は$0 < m < 2^{32}$の範囲に収めること。
///
/// **Time complexity** $O(\sqrt{m})$
pub fn log_mod(a: u64, mut b: u64, mut m: u64) -> Option<u64> {
    assert!(
        0 < m && m <= 0xFFFFFFFF,
        "Violated 0 < m <= 0xFFFFFFFF (m = {m})"
    );
    if b >= m {
        b %= m;
    }

    if b == 1 {
        return Some(0);
    }

    let mut d = 0;

    loop {
        let g = a.gcd(m);
        if g != 1 {
            if !b.is_multiple_of(g) {
                return None;
            }

            d += 1;
            m /= g;
            b /= g;
            b *= inv_mod(a / g, m).unwrap();
            b %= m;

            if b == 1 {
                return Some(d);
            }
        } else {
            break;
        }
    }

    let sq = (m as f64).sqrt() as u64 + 1;

    let mut mp = HashMap::new();

    let mut t = 1 % m;

    for i in 0..sq {
        mp.entry(t).or_insert(i);
        t *= a;
        t %= m;
    }

    let x = pow_mod(inv_mod(a, m).unwrap(), sq, m);
    let mut t = b % m;

    for i in 0..sq {
        if let Some(k) = mp.get(&t) {
            return Some(i * sq + k + d);
        }

        t *= x;
        t %= m;
    }

    None
}
