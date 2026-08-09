//! 原始根 (`u64`)
use crate::math::factorize::pollard_rho::*;
use crate::math::montgomery::*;
use crate::math::primality::miller_rabin::*;

/// `u64`で表される素数の原始根を求める。
pub fn primitive_root_u64(p: u64) -> u64 {
    if p == 2 {
        return 1;
    }

    assert!(MillerRabin.is_prime(p), "{p} is not a prime number.");

    let pf = pollard_rho(p - 1);
    let mn = Montgomery::new(p);

    (2..p)
        .find(|&g| {
            pf.iter()
                .all(|f| mn.unwrap(mn.pow(mn.wrap(g), (p - 1) / f)) != 1)
        })
        .expect("No primitive roots.")
}
