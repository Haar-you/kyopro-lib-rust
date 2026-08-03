//! 整数を2つの整数の2乗和で表す。
//!
//! # References
//! - <https://manabitimes.jp/math/844>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/two_square_sum>

use std::collections::{HashMap, HashSet};

use crate::math::factorize::pollard_rho::*;
use crate::math::mod_ops::sqrt::sqrt_mod;

/// 非負整数$n$について、$n = x^2 + y^2$を満たす非負整数$x$, $y$の組をすべて列挙する。
pub fn two_square_sum(n: u64) -> Vec<(u64, u64)> {
    if n == 0 {
        return vec![(0, 0)];
    }
    let mut ps = vec![];
    let mut p4kp3 = HashMap::<u64, u32>::new();

    for p in pollard_rho(n) {
        if p % 4 == 3 {
            *p4kp3.entry(p).or_default() += 1;
        } else {
            ps.push(p);
        }
    }

    let mut x = 1_u128;
    for (p, k) in p4kp3 {
        if k % 2 == 1 {
            return vec![];
        }
        x *= (p as u128).pow(k / 2);
    }

    let mut pairs = HashSet::<(u128, u128)>::new();
    pairs.insert((x, 0));
    pairs.insert((0, x));

    for p in ps {
        let (x1, y1) = two_square_sum_of_prime(p).unwrap();
        let (x1, y1) = (x1 as u128, y1 as u128);

        let mut temp = HashSet::new();

        for &(x2, y2) in &pairs {
            temp.insert(((x1 * y2).abs_diff(x2 * y1), x1 * x2 + y1 * y2));
            temp.insert((x1 * x2 + y1 * y2, (x1 * y2).abs_diff(x2 * y1)));
        }

        pairs = temp;
    }

    pairs
        .into_iter()
        .map(|(x, y)| (x as u64, y as u64))
        .collect()
}

/// 素数$p$について、$p = x^2 + y^2$を満たす非負整数$x$,$y$ ($x \le y$)を求める。
///
/// - $p = 2$のとき: (x, y) = (1, 1)
/// - $p = 4k + 1$のとき: (x, y)のペアがただ一つ存在する。
/// - $p = 4k + 3$のとき: このような(x, y)は存在しない。
pub fn two_square_sum_of_prime(p: u64) -> Option<(u64, u64)> {
    if p == 2 {
        return Some((1, 1));
    }
    if p % 4 == 3 {
        return None;
    }
    assert!(p % 4 == 1);

    let mut x = sqrt_mod(p - 1, p).first().copied().unwrap() as u128;
    let mut y = 1_u128;
    let p = p as u128;
    assert!((x * x + y * y) % p == 0);
    let mut k = (x * x + y * y) / p;

    while k > 1 {
        let mut b = x % k;
        let mut sb = false;
        let mut d = y % k;
        let mut sd = false;

        if b > k / 2 {
            b = k - b;
            sb = true;
        }
        if d > k / 2 {
            d = k - d;
            sd = true;
        }

        assert!((b * b + d * d) % k == 0);
        let k_ = (b * b + d * d) / k;

        let x_ = if sb == sd {
            (x * b + y * d) / k
        } else {
            (x * b).abs_diff(y * d) / k
        };

        let y_ = if sb == sd {
            (x * d).abs_diff(y * b) / k
        } else {
            (x * d + y * b) / k
        };

        (x, y, k) = (x_, y_, k_);
    }

    assert!(x * x + y * y == p);

    if x > y {
        (x, y) = (y, x);
    }

    Some((x as u64, y as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime() {
        let check_4kp1 = |p, x, y| {
            assert_eq!(p, x * x + y * y);
            assert_eq!(two_square_sum_of_prime(p).unwrap(), (x, y));
        };

        check_4kp1(5, 1, 2);
        check_4kp1(13, 2, 3);
        check_4kp1(17, 1, 4);
        check_4kp1(29, 2, 5);
        check_4kp1(37, 1, 6);
        check_4kp1(41, 4, 5);
        check_4kp1(53, 2, 7);
        check_4kp1(1000000009, 3747, 31400);
    }

    #[test]
    fn test() {
        let check = |n| {
            let mut res = two_square_sum(n);
            res.sort();

            let ans: Vec<_> = (0..)
                .take_while(|&x| x * x <= n)
                .flat_map(|x| {
                    (0..)
                        .take_while(|&y| y * y <= n)
                        .filter(move |&y| x * x + y * y == n)
                        .map(move |y| (x, y))
                })
                .collect();

            assert_eq!(res, ans);
        };

        for n in 1..=100 {
            check(n);
        }
    }
}
