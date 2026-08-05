//! 二項係数
//!
//! # References
//! - <https://ferin-tech.hatenablog.com/entry/2018/01/17/010829>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/binomial_coefficient>
use crate::math::crt::crt_vec;
use crate::math::mod_ops::inv::*;
use crate::math::mod_ops::pow::*;

/// 二項係数$_nC_k \pmod{p^q}$($p$は素数)を計算する。
#[derive(Clone)]
pub struct ExtLucas {
    prod: Vec<u64>,
    inv: Vec<u64>,
    p: u64,
    q: u64,
    m: u64,
}

impl ExtLucas {
    /// 素数$p$に対して$\pmod{p^q}$で[`ExtLucas`]を用意する。
    pub fn new(p: u64, q: u64) -> Self {
        let m = p.pow(q as u32);

        let mut prod: Vec<u64> = vec![1; m as usize];
        let mut inv: Vec<u64> = vec![1; m as usize];

        for i in 1..m as usize {
            if (i as u64).is_multiple_of(p) {
                prod[i] = prod[i - 1]
            } else {
                prod[i] = prod[i - 1] * i as u64 % m;
            }
        }

        inv[m as usize - 1] = inv_mod(prod[m as usize - 1], m).unwrap();
        for i in (1..m as usize).rev() {
            if (i as u64).is_multiple_of(p) {
                inv[i - 1] = inv[i]
            } else {
                inv[i - 1] = inv[i] * i as u64 % m;
            }
        }

        Self { prod, inv, p, q, m }
    }

    /// $_nC_k \pmod{p^q}$を計算する。
    pub fn calc(&self, mut n: u64, mut k: u64) -> u64 {
        if n < k {
            return 0;
        }

        let mut r = n - k;
        let mut e = 0;
        let mut eq = 0;
        let mut ret = 1;

        let mut i = 0;
        loop {
            if n == 0 {
                break;
            }

            ret *= self.prod[(n % self.m) as usize];
            ret %= self.m;
            ret *= self.inv[(k % self.m) as usize];
            ret %= self.m;
            ret *= self.inv[(r % self.m) as usize];
            ret %= self.m;

            n /= self.p;
            k /= self.p;
            r /= self.p;

            e += n - k - r;

            if e >= self.q {
                return 0;
            }

            i += 1;
            if i >= self.q {
                eq += n - k - r;
            }
        }

        if (self.p != 2 || self.q < 3) && eq % 2 == 1 {
            ret = self.m - ret;
        }

        ret *= pow_mod(self.p, e, self.m);
        ret %= self.m;

        ret
    }
}

/// 二項係数$_nC_k \pmod m$を計算する。
#[derive(Clone)]
pub struct BinomialCoefficient {
    lu: Vec<ExtLucas>,
    ms: Vec<u64>,
}

impl BinomialCoefficient {
    /// $\pmod m$で[`BinomialCoefficient`]を用意する。
    pub fn new(mut m: u64) -> Self {
        let mut m_primes = vec![];
        let mut ms = vec![];
        let mut lu = vec![];

        let mut i = 2;
        while i * i <= m {
            if m.is_multiple_of(i) {
                let mut t = 1;
                let mut c = 0;
                while m.is_multiple_of(i) {
                    m /= i;
                    c += 1;
                    t *= i;
                }
                m_primes.push((i, c));
                ms.push(t);
            }
            i += 1;
        }

        if m != 1 {
            m_primes.push((m, 1));
            ms.push(m);
        }

        for (p, q) in m_primes {
            lu.push(ExtLucas::new(p, q));
        }

        Self { lu, ms }
    }

    /// $_nC_k \pmod m$を計算する。
    pub fn calc(&self, n: u64, k: u64) -> u64 {
        if n < k {
            0
        } else {
            let bs = self.lu.iter().map(|lu| lu.calc(n, k));
            let a = bs
                .zip(self.ms.iter())
                .map(|(a, &b)| (a as i64, b))
                .collect::<Vec<_>>();
            crt_vec(&a).unwrap().0 as u64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let c = BinomialCoefficient::new(10007);
        assert_eq!(c.calc(4, 2), 6);
        assert_eq!(c.calc(0, 0), 1);
        assert_eq!(c.calc(1000000007, 998244353), 0);

        let c = BinomialCoefficient::new(60);
        assert_eq!(
            (0..=10).map(|i| c.calc(20, i)).collect::<Vec<_>>(),
            [1, 20, 10, 0, 45, 24, 0, 0, 30, 20, 16]
        );
    }
}
