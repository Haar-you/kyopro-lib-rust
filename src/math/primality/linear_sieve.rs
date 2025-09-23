//! 線形篩

use crate::math::primality::PrimalityTest;

/// 線形篩
pub struct LinearSieve {
    least_factors: Vec<usize>,
}

impl LinearSieve {
    /// `n`までの自然数の素数判定ができる`LinearSieve`を返す。
    pub fn new(n: usize) -> Self {
        let mut least_factors = vec![0; n + 1];
        let mut primes = vec![];

        for d in 2..=n {
            if least_factors[d] == 0 {
                least_factors[d] = d;
                primes.push(d);
            }

            for &p in &primes {
                if p * d > n || p > least_factors[d] {
                    break;
                }
                least_factors[p * d] = p;
            }
        }

        Self { least_factors }
    }
}

impl PrimalityTest<usize> for LinearSieve {
    fn is_prime(&self, value: usize) -> bool {
        if value == 0 {
            return false;
        }
        self.least_factors[value] == value
    }
}
