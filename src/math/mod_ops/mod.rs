//! mod mでの演算

pub mod enum_inv;
pub mod inv;
pub mod inv_p;
pub mod kth_root;
pub mod log;
pub mod pow;
pub mod sqrt;

#[cfg(test)]
mod tests {
    use crate::math::primality::{PrimalityTest, miller_rabin::MillerRabin};

    use super::{enum_inv::*, inv::*, kth_root::*, log::*, pow::*, sqrt::*};

    #[test]
    fn test_mod_pow() {
        fn straight_forward(x: u64, p: u64, m: u64) -> u64 {
            let mut ret = 1;
            for _ in 0..p {
                ret *= x;
                ret %= m;
            }
            ret
        }

        for x in 1..10 {
            for p in 0..10 {
                for m in 1..10 {
                    assert_eq!(pow_mod(x, p, m), straight_forward(x, p, m));
                }
            }
        }
    }

    #[test]
    fn test_mod_inv() {
        let m = 19;

        for x in 1..m {
            assert_eq!(inv_mod(x, m).unwrap() * x % m, 1);
        }

        assert_eq!(inv_mod(4, 10), None);
        assert_eq!(inv_mod(3, 10), Some(7));
    }

    #[test]
    fn test_mod_log() {
        // https://judge.yosupo.jp/problem/discrete_logarithm_mod
        assert_eq!(log_mod(2, 1, 5), Some(0));
        assert_eq!(log_mod(4, 7, 10), None);
        assert_eq!(log_mod(8, 6, 10), Some(4));
        assert_eq!(log_mod(5, 2, 11), None);
        assert_eq!(log_mod(5, 9, 11), Some(4));
        assert_eq!(log_mod(0, 0, 1), Some(0));
        assert_eq!(log_mod(0, 2, 4), None);
    }

    #[test]
    fn test_mod_sqrt() {
        let n = 1000;

        for p in (2..=n).filter(|&p| MillerRabin.is_prime(p)) {
            for a in 0..p {
                let res: Vec<_> = (0..p).filter(|x| x * x % p == a).collect();

                assert_eq!(res, sqrt_mod(a, p));
            }
        }
    }

    #[test]
    fn test_mod_kth_root() {
        let n = 100;

        for p in (2..=n).filter(|&p| MillerRabin.is_prime(p)) {
            for a in 0..p {
                for k in 1..10 {
                    let ans: Vec<_> = (0..p).filter(|x| pow_mod(*x, k, p) == a).collect();

                    let res = kth_root_mod(a, k, p);

                    if let Some(res) = res {
                        assert!(ans.into_iter().any(|x| x == res));
                    } else {
                        assert!(ans.is_empty());
                    }
                }
            }
        }
    }

    #[test]
    fn test_enumerate_mod_inv() {
        #![allow(clippy::needless_range_loop)]
        let m = 1000000007;
        let n = 100;

        let s = enumerate_inv_mod(n, m);
        for i in 1..=n {
            assert_eq!(i as u64 * s[i] % m, 1);
        }
    }
}
