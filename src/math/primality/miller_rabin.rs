//! Miller-Rabin素数判定法
use crate::math::montgomery::*;
pub use crate::math::primality::PrimalityTest;

fn is_composite(a: u64, s: u32, d: u64, mg: Montgomery) -> bool {
    let a = mg.wrap(a);
    let pp = mg.wrap(mg.modulo() - 1);
    let mut x = mg.pow(a, d);

    if mg.unwrap(x) == 1 {
        false
    } else {
        for _ in 0..s {
            if x == pp {
                return false;
            }
            x = mg.mul(x, x);
        }

        true
    }
}

/// Miller-Rabin素数判定法
pub struct MillerRabin;

impl PrimalityTest<u64> for MillerRabin {
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

            let mg = Montgomery::new(n);

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
