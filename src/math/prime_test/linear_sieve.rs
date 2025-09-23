//! 線形篩

use crate::math::prime_test::CheckPrime;

/// 線形篩
pub struct LinearSieve {
    least_factors: Vec<u32>,
}

impl LinearSieve {
    /// `n`までの自然数の素数判定ができる`LinearSieve`を返す。
    pub fn new(n: u32) -> Self {
        let mut least_factors = vec![0; n as usize + 1];
        let mut primes = vec![];

        for d in 2..=n {
            if least_factors[d as usize] == 0 {
                least_factors[d as usize] = d;
                primes.push(d);
            }

            for &p in &primes {
                if p * d > n || p > least_factors[d as usize] {
                    break;
                }
                least_factors[(p * d) as usize] = p;
            }
        }

        Self { least_factors }
    }
}

impl CheckPrime<u32> for LinearSieve {
    fn is_prime(&self, value: u32) -> bool {
        if value == 0 {
            return false;
        }
        self.least_factors[value as usize] == value
    }
}
