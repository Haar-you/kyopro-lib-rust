//! 互いに素な数を数える。
use crate::math::factorize::trial::factorize;

/// `n`以下の自然数で、`m`と互いに素なものの個数を求める。
pub fn count_coprime(n: u64, m: u64) -> u64 {
    let ps = factorize(m);
    let k = ps.len();

    let mut ret = 0;

    for i in 0..1_usize << k {
        let mut s = 1;

        for (j, (p, _)) in ps.iter().enumerate() {
            if i & (1 << j) != 0 {
                s *= p;
            }
        }

        if i.count_ones() % 2 == 1 {
            ret -= n / s;
        } else {
            ret += n / s;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::gcd_lcm::GcdLcm;

    #[test]
    fn test() {
        let n = 2000;
        let m = 100;

        let ans = (1..=n).filter(|x| x.gcd(m) == 1).count() as u64;

        assert_eq!(count_coprime(n, m), ans);
    }
}
