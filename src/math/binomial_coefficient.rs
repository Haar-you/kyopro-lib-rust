use crate::math::{
    crt::crt_vec,
    mod_ops::{inv::*, pow::*},
};

#[derive(Clone)]
pub struct ExtLucas {
    prod: Vec<u64>,
    inv: Vec<u64>,
    p: u64,
    q: u64,
    m: u64,
}

impl ExtLucas {
    pub fn new(p: u64, q: u64) -> Self {
        let m = p.pow(q as u32);

        let mut prod: Vec<u64> = vec![1; m as usize];
        let mut inv: Vec<u64> = vec![1; m as usize];

        for i in 1..m as usize {
            prod[i] = prod[i - 1] * (if i as u64 % p == 0 { 1 } else { i as u64 }) % m;
        }

        inv[m as usize - 1] = mod_inv(prod[m as usize - 1], m).unwrap();
        for i in (1..m as usize).rev() {
            inv[i - 1] = inv[i] * (if i as u64 % p == 0 { 1 } else { i as u64 }) % m;
        }

        Self { prod, inv, p, q, m }
    }

    pub fn get(&self, mut n: u64, mut k: u64) -> u64 {
        assert!(n >= k);

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

        ret *= mod_pow(self.p, e, self.m);
        ret %= self.m;

        ret
    }
}

#[derive(Clone)]
pub struct BinomialCoefficient {
    lu: Vec<ExtLucas>,
    ms: Vec<u64>,
}

impl BinomialCoefficient {
    pub fn new(mut m: u64) -> Self {
        let mut m_primes = vec![];
        let mut ms = vec![];
        let mut lu = vec![];

        let mut i = 2;
        while i * i <= m {
            if m % i == 0 {
                let mut t = 1;
                let mut c = 0;
                while m % i == 0 {
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

        Self { ms, lu }
    }

    pub fn get(&self, n: u64, k: u64) -> u64 {
        if n < k {
            0
        } else {
            let bs = self.lu.iter().map(|lu| lu.get(n, k));
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
        assert_eq!(c.get(4, 2), 6);
        assert_eq!(c.get(0, 0), 1);
        assert_eq!(c.get(1000000007, 998244353), 0);

        let c = BinomialCoefficient::new(60);
        assert_eq!(
            (0..=10).map(|i| c.get(20, i)).collect::<Vec<_>>(),
            [1, 20, 10, 0, 45, 24, 0, 0, 30, 20, 16]
        );
    }
}
