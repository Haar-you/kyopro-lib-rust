use crate::math::{gcd_lcm::*, mod_ops::inv::*};

/// ax + b = 0 (mod m) を満たすxを求める。
pub fn linear_congruence(mut a: i64, mut b: i64, mut m: u64) -> Option<i64> {
    if a >= m as i64 {
        a %= m as i64;
    }
    if b >= m as i64 {
        b %= m as i64;
    }
    if a < 0 {
        a += m as i64;
        a %= m as i64;
    }
    if b < 0 {
        b += m as i64;
        b %= m as i64;
    }

    let mut a = a as u64;
    let mut b = b as u64;
    let g = a.gcd(m);
    if b % g == 0 {
        a /= g;
        b /= g;
        m /= g;
    }

    Some(((m - b) * mod_inv(a, m)? % m) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let m = 100;

        for a in 1..=100 {
            for b in 0..=100 {
                if let Some(x) = linear_congruence(a, b, m) {
                    assert_eq!((a * x + b) % m as i64, 0);
                } else {
                    assert!((0..m).all(|x| (a * x as i64 + b) % m as i64 != 0));
                }
            }
        }
    }
}
