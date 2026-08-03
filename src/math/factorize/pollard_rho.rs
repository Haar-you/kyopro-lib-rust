//! Pollard's rho素因数分解
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/factorize>

use crate::math::gcd_lcm::*;
use crate::math::montgomery::*;
use crate::math::primality::miller_rabin::*;

fn find_factor(n: u64, a: u64) -> Option<u64> {
    assert!(n % 2 == 1);
    let mn = Montgomery::new(n);

    let f = |x| mn.add(mn.mul(x, x), Wrapped(a));

    let mut x = mn.wrap(a);
    let mut y = f(x);

    loop {
        x = f(x);
        y = f(f(y));

        let x = mn.unwrap(x);
        let y = mn.unwrap(y);
        let d = x.abs_diff(y).gcd(n);

        if d == 1 {
            continue;
        }
        return (d < n).then_some(d);
    }
}

/// Pollard's rho素因数分解
pub fn pollard_rho(mut n: u64) -> Vec<u64> {
    assert!(n > 0);
    if n == 1 {
        return vec![];
    }

    let mut ret = vec![];
    while n % 2 == 0 {
        ret.push(2);
        n /= 2;
    }

    let mut remain = vec![n];

    while let Some(n) = remain.pop() {
        if n == 1 {
            continue;
        }
        if MillerRabin.is_prime(n) {
            ret.push(n);
            continue;
        }

        if let Some(p) = (1..).find_map(|a| find_factor(n, a)) {
            remain.push(n / p);
            remain.push(p);
        }
    }

    ret.sort();
    ret
}
