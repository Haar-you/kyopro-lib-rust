//! x² = a (mod p)を満たすxを求める。

use crate::math::mod_ops::pow::*;
use crate::rand::rand;

/// x² = a (mod p)を満たすxをすべて求める。
pub fn mod_sqrt(a: u64, p: u64) -> Vec<u64> {
    if p == 2 {
        return vec![a % 2];
    }
    if a == 0 {
        return vec![0];
    }

    let b = mod_pow(a, (p - 1) / 2, p);

    if b == p - 1 {
        return vec![];
    }
    if p % 4 == 3 {
        let t = mod_pow(a, (p + 1) / 4, p);
        assert!(t != 0);
        return if t < p - t {
            vec![t, p - t]
        } else {
            vec![p - t, t]
        };
    }

    let mut q = p - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    let z = {
        let ret;
        loop {
            let z = rand() % p;
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
    let mut r = mod_pow(a, q.div_ceil(2), p);

    loop {
        if t == 0 {
            return vec![0];
        }
        if t == 1 {
            return if r < p - r {
                vec![r, p - r]
            } else {
                vec![p - r, r]
            };
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
