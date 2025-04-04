//! mod mでの演算

pub mod enum_inv;
pub mod inv;
pub mod inv_p;
pub mod log;
pub mod pow;
pub mod sqrt;

#[cfg(test)]
mod tests {
    use super::{enum_inv::*, inv::*, log::*, pow::*, sqrt::*};

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
                    assert_eq!(mod_pow(x, p, m), straight_forward(x, p, m));
                }
            }
        }
    }

    #[test]
    fn test_mod_inv() {
        let m = 19;

        for x in 1..m {
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
        assert_eq!(mod_sqrt(0, 5).map(|x| x.pow(2) % 5), Some(0));
        assert_eq!(mod_sqrt(1, 5).map(|x| x.pow(2) % 5), Some(1));
        assert_eq!(mod_sqrt(2, 5).map(|x| x.pow(2) % 5), None);
        assert_eq!(mod_sqrt(3, 5).map(|x| x.pow(2) % 5), None);
        assert_eq!(mod_sqrt(4, 5).map(|x| x.pow(2) % 5), Some(4));

        let m = 1000000007;
        for x in 0..100 {
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
        for i in 1..=n {
            assert_eq!(i as u64 * s[i] % m, 1);
        }
    }
}
