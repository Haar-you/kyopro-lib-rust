use crate::math::{
    gcd_lcm::GcdLcm,
    mod_ops::{inv::*, pow::*},
};
use std::collections::HashMap;

pub fn mod_log(a: u64, mut b: u64, mut m: u64) -> Option<u64> {
    if b == 1 {
        return Some(0);
    }

    let mut d = 0;

    loop {
        let g = a.gcd(m);
        if g != 1 {
            if b % g != 0 {
                return None;
            }

            d += 1;
            m /= g;
            b /= g;
            b *= mod_inv(a / g, m).unwrap();
            b %= m;

            if b == 1 {
                return Some(d);
            }
        } else {
            break;
        }
    }

    let sq = (m as f64).sqrt() as u64 + 1;

    let mut mp = HashMap::new();

    let mut t = 1 % m;

    for i in 0..sq {
        mp.entry(t).or_insert(i);
        t *= a;
        t %= m;
    }

    let x = mod_pow(mod_inv(a, m).unwrap(), sq, m);
    let mut t = b % m;

    for i in 0..sq {
        if let Some(k) = mp.get(&t) {
            return Some(i * sq + k + d);
        }

        t *= x;
        t %= m;
    }

    None
}
