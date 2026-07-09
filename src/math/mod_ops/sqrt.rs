//! mod p(素数)での平方根

use crate::math::montgomery::Montgomery;
use crate::rand::rand;

/// 素数$p$と整数$a$について、$x^2 \equiv a \pmod p$を満たす$x$をすべて求める。
pub fn mod_sqrt(a: u64, p: u64) -> Vec<u64> {
    if p == 2 {
        return vec![a % 2];
    }
    if a == 0 {
        return vec![0];
    }

    let mg = Montgomery::new(p);

    let a = mg.wrap(a);
    let b = mg.pow(a, (p - 1) / 2);

    if mg.unwrap(b) == p - 1 {
        return vec![];
    }
    if p % 4 == 3 {
        let t = mg.unwrap(mg.pow(a, (p + 1) / 4));
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
            let z = mg.wrap(rand() % p);
            if mg.unwrap(mg.pow(z, (p - 1) / 2)) == p - 1 {
                ret = z;
                break;
            }
        }
        ret
    };

    let mut m = s;
    let mut c = mg.pow(z, q);
    let mut t = mg.pow(a, q);
    let mut r = mg.pow(a, q.div_ceil(2));

    loop {
        match mg.unwrap(t) {
            0 => {
                return vec![0];
            }
            1 => {
                let r = mg.unwrap(r);
                return if r < p - r {
                    vec![r, p - r]
                } else {
                    vec![p - r, r]
                };
            }
            _ => {}
        }

        let mut i = 1;
        let mut k = t;
        while i < m {
            k = mg.mul(k, k);
            if mg.unwrap(k) == 1 {
                break;
            }

            i += 1;
        }

        let b = mg.pow(c, 1 << (m - i - 1));

        m = i;
        c = mg.mul(b, b);
        t = mg.mul(t, mg.mul(b, b));
        r = mg.mul(r, b);
    }
}
