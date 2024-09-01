//! $x^2 = a \bmod p$を満たすxを一つ求める。

use crate::math::mod_ops::pow::*;
use rand::Rng;

/// $x^2 = a \bmod p$を満たすxを一つ求める。
pub fn mod_sqrt(a: u64, p: u64) -> Option<u64> {
    if p == 2 {
        return Some(a % 2);
    }
    if a == 0 {
        return Some(0);
    }

    let b = mod_pow(a, (p - 1) / 2, p);

    if b == p - 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(mod_pow(a, (p + 1) / 4, p));
    }

    let mut q = p - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    let mut rng = rand::thread_rng();

    let z = {
        let ret;
        loop {
            let z = rng.gen::<u64>() % p;
            if mod_pow(z, (p - 1) / 2, p) == p - 1 {
                ret = z;
                break;
            }
        }
        ret
    };

    let mut m = s;
    let mut c = mod_pow(z, q, p);
    let mut t = mod_pow(a, q, p);
    let mut r = mod_pow(a, (q + 1) / 2, p);

    loop {
        if t == 0 {
            return Some(0);
        }
        if t == 1 {
            return Some(r);
        }

        let mut i = 1;
        let mut k = t;
        while i < m {
            k *= k;
            k %= p;
            if k == 1 {
                break;
            }

            i += 1;
        }

        let b = mod_pow(c, 1 << (m - i - 1), p);

        m = i;
        c = b * b % p;
        t *= b * b % p;
        t %= p;
        r *= b;
        r %= p;
    }
}
