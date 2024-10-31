//! Miller-Rabin素数判定法
pub use crate::math::prime_test::CheckPrime;

fn pow(mut a: u128, mut b: u128, p: u128) -> u128 {
    let mut ret = 1;

    while b > 0 {
        if b & 1 == 1 {
            ret = ret * a % p;
        }
        a = a * a % p;
        b >>= 1;
    }

    ret
}

fn is_composite(a: u64, p: u64, s: u64, d: u64) -> bool {
    let p = p as u128;
    let mut x = pow(a as u128, d as u128, p);

    if x == 1 {
        false
    } else {
        for _ in 0..s {
            if x == p - 1 {
                return false;
            }
            x = x * x % p;
        }

        true
    }
}

/// Miller-Rabin素数判定法
pub struct MillerRabin;

impl CheckPrime<u64> for MillerRabin {
    fn is_prime(&self, n: u64) -> bool {
        if n <= 1 {
            false
        } else if n == 2 {
            true
        } else if n % 2 == 0 {
            false
        } else {
            let mut s = 0;
            let mut d = n - 1;
            while d & 1 == 0 {
                s += 1;
                d >>= 1;
            }

            if n < 4_759_123_141 {
                for &x in &[2, 7, 61] {
                    if x < n && is_composite(x, n, s, d) {
                        return false;
                    }
                }

                true
            } else {
                for &x in &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
                    if x < n && is_composite(x, n, s, d) {
                        return false;
                    }
                }

                true
            }
        }
    }
}
