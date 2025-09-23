//! Miller-Rabin素数判定法
pub use crate::math::prime_test::CheckPrime;

const B: u32 = 64;
const R: u128 = 1 << B;
const MASK: u128 = R - 1;

type Montgomery = (u64, u128, u64);

fn montgomery(modulo: u64) -> Montgomery {
    assert!(modulo % 2 != 0);
    assert!(modulo > 0);

    let r = R % modulo as u128;
    let r2 = r * r % modulo as u128;
    let m = {
        let mut ret: u64 = 0;
        let mut r = R;
        let mut i = 1;
        let mut t = 0;
        while r > 1 {
            if t % 2 == 0 {
                t += modulo;
                ret += i;
            }
            t >>= 1;
            r >>= 1;
            i <<= 1;
        }
        ret
    };

    (modulo, r2, m)
}

fn reduce(value: u128, modulo: u64, m: u64) -> u64 {
    let mut ret = (((((value & MASK) * m as u128) & MASK) * modulo as u128 + value) >> B) as u64;
    if ret >= modulo {
        ret -= modulo;
    }
    ret
}

fn pow(mut a: u64, mut p: u64, mg: Montgomery) -> u64 {
    let (modulo, r2, m) = mg;

    let mut value = reduce(r2, modulo, m);

    while p > 0 {
        if (p & 1) != 0 {
            value = reduce(value as u128 * a as u128, modulo, m);
        }
        a = reduce(a as u128 * a as u128, modulo, m);
        p >>= 1;
    }

    value
}

fn is_composite(a: u64, s: u32, d: u64, mg: Montgomery) -> bool {
    let (p, r2, m) = mg;
    let a = reduce(a as u128 * r2, p, m);
    let pp = reduce((p as u128 - 1) * r2, p, m);
    let mut x = pow(a, d, mg);

    if reduce(x as u128, p, m) == 1 {
        false
    } else {
        for _ in 0..s {
            if x == pp {
                return false;
            }
            x = reduce(x as u128 * x as u128, p, m);
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
            let s = (n - 1).trailing_zeros();
            let d = (n - 1) >> s;

            let mg = montgomery(n);

            if n < 4_759_123_141 {
                ![2, 7, 61]
                    .into_iter()
                    .any(|a| a < n && is_composite(a, s, d, mg))
            } else {
                ![2, 325, 9375, 28178, 450775, 9780504, 1795265022]
                    .into_iter()
                    .any(|a| a < n && is_composite(a, s, d, mg))
            }
        }
    }
}
