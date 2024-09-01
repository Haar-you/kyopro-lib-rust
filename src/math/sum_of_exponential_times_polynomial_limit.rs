//! ∑{i=0 → ∞} rⁱiᵈ

use crate::math::mod_ops::inv_p::mod_inv_p;
use crate::math::mod_ops::pow::mod_pow;

/// ∑{i=0 → ∞} rⁱiᵈ
///
/// **Time Complexity O(d log p)**
pub fn sum_of_exponential_times_polynomial_limit(r: u64, d: u64, p: u64) -> u64 {
    let mut ret = 0;
    let mut r_pow = 1;
    let mut s = vec![0; d as usize + 1];
    let mut invs = vec![0; d as usize + 2];

    invs[1] = 1;

    for i in 2..=d + 1 {
        invs[i as usize] = (p / i) * (p - invs[(p % i) as usize]) % p;
    }

    for i in 0..=d as usize {
        if i > 0 {
            s[i] += s[i - 1];
            if s[i] >= p {
                s[i] -= p;
            }
        }
        s[i] = (s[i] + mod_pow(i as u64, d, p) * r_pow) % p;
        r_pow = (r_pow * r) % p;
    }

    let mut t = 1;

    for i in 0..=d {
        ret = (ret + t * s[(d - i) as usize]) % p;
        t = t * (invs[i as usize + 1] * (p - r) % p * (d + 1 - i) % p) % p;
    }

    ret * mod_inv_p(mod_pow(p + 1 - r, d + 1, p), p) % p
}
