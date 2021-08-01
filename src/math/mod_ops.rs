use std::collections::HashMap;
use rand::Rng;

use crate::math::gcd_lcm::GcdLcm;

/// Time complexity O(log p)
pub fn mod_pow(mut x: u64, mut p: u64, m: u64) -> u64 {
    let mut ret = 1;
    while p > 0 {
        if (p & 1) != 0 {
            ret *= x;
            ret %= m;
        }
        x *= x;
        x %= m;

        p >>= 1;
    }
    ret
}

pub fn mod_inv(mut a: u64, m: u64) -> Option<u64> {
    if a.gcd(m) != 1 {
        return None
    }

    let mut b = m;
    let mut u = 1;
    let mut v = 0;

    while b > 0 {
        let t = a / b;

        a -= t * b;
        std::mem::swap(&mut a, &mut b);

        if u < t * v {
            u += m - (t * v) % m;
            u %= m;
        }
        else {
            u -= t * v;
        }
        std::mem::swap(&mut u, &mut v);
    }

    Some(u)
}

pub fn mod_log(a: u64, mut b: u64, mut m: u64) -> Option<u64> {
    if b == 1 {
        return Some(0);
    }

    let mut d = 0;

    loop {
        let g = a.gcd(m);
        if g != 1 {
            if b % g != 0 {
                return None;
            }

            d += 1;
            m /= g;
            b /= g;
            b *= mod_inv(a / g, m).unwrap();
            b %= m;

            if b == 1 {
                return Some(d);
            }
        }
        else {
            break;
        }
    }

    let sq = (m as f64).sqrt() as u64 + 1;

    let mut mp = HashMap::new();

    let mut t = 1 % m;

    for i in 0 .. sq {
        if !mp.contains_key(&t) {
            mp.insert(t, i);
        }
        t *= a;
        t %= m;
    }

    let x = mod_pow(mod_inv(a, m).unwrap(), sq, m);
    let mut t = b % m;

    for i in 0 .. sq {
        if let Some(k) = mp.get(&t) {
            return Some(i * sq + k + d);
        }

        t *= x;
        t %= m;
    }

    None
}


pub fn mod_sqrt(a: u64, p: u64) -> Option<u64> {
    if p == 2 {
        return Some(a % 2);
    }
    if a == 0 {
        return Some(0);
    }

    let b = mod_pow(a, (p - 1) / 2, p);

    if b == p - 1 {
        return None
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

pub fn enumerate_mod_inv(n: usize, p: u64) -> Vec<u64> {
    let mut ret = vec![0; n + 1];

    ret[1] = 1;

    for i in 2 ..= n {
        ret[i] = (p / i as u64) * (p - ret[(p % i as u64) as usize]) % p;
    }

    ret
}








#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_pow() {
        fn straight_forward(x: u64, p: u64, m: u64) -> u64 {
            let mut ret = 1;
            for _ in 0 .. p {
                ret = ret * x;
                ret = ret % m;
            }
            ret
        }

        for x in 1 .. 10 {
            for p in 0 .. 10 {
                for m in 1 .. 10 {
                    assert_eq!(mod_pow(x, p, m), straight_forward(x, p, m));
                }
            }
        }
    }

    #[test]
    fn test_mod_inv() {
        let m = 19;

        for x in 1 .. m {
            assert_eq!(mod_inv(x, m).unwrap() * x % m, 1);
        }

        assert_eq!(mod_inv(4, 10), None);
        assert_eq!(mod_inv(3, 10), Some(7));
    }

    #[test]
    fn test_mod_log() {
        // https://judge.yosupo.jp/problem/discrete_logarithm_mod
        assert_eq!(mod_log(2, 1, 5), Some(0));
        assert_eq!(mod_log(4, 7, 10), None);
        assert_eq!(mod_log(8, 6, 10), Some(4));
        assert_eq!(mod_log(5, 2, 11), None);
        assert_eq!(mod_log(5, 9, 11), Some(4));
        assert_eq!(mod_log(0, 0, 1), Some(0));
        assert_eq!(mod_log(0, 2, 4), None);
    }

    #[test]
    fn test_mod_sqrt() {
        // https://judge.yosupo.jp/problem/sqrt_mod
        assert_eq!(mod_sqrt(0, 5).unwrap().pow(2) % 5, 0);
        assert_eq!(mod_sqrt(1, 5).unwrap().pow(2) % 5, 1);
        assert_eq!(mod_sqrt(2, 5), None);
        assert_eq!(mod_sqrt(3, 5), None);
        assert_eq!(mod_sqrt(4, 5).unwrap().pow(2) % 5, 4);

        let m = 1000000007;
        for x in 0 .. 100 {
            if let Some(s) = mod_sqrt(x, m) {
                assert_eq!(s * s % m, x);
            }
        }
    }

    #[test]
    fn test_enumerate_mod_inv() {
        let m = 1000000007;
        let n = 100;

        let s = enumerate_mod_inv(n, m);
        for i in 1 ..= n {
            assert_eq!(i as u64 * s[i] % m, 1);
        }
    }
}
