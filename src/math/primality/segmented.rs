//! 区間篩
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc412/tasks/abc412_e>

pub use crate::math::primality::PrimalityTest;

/// 区間篩
pub struct SegmentedSieve {
    l: usize,
    r: usize,
    data: Vec<bool>,
}

impl SegmentedSieve {
    /// `[l, r]`の区間篩を作る。
    ///
    /// `check`は$\sqrt{r}$の以下の素数判定が可能であること。
    pub fn new(l: usize, r: usize, check: &impl PrimalityTest<usize>) -> Self {
        assert!(l <= r);

        let d = r - l + 1;
        let primes = (2..)
            .take_while(|i| i * i <= r)
            .filter(|&i| check.is_prime(i))
            .collect::<Vec<_>>();
        let mut data = vec![true; d];

        for p in primes {
            let mut from = l.div_ceil(p) * p;
            if p == from {
                from = p * 2;
            }

            for i in (from..=r).step_by(p) {
                data[i - l] = false;
            }
        }

        Self { l, r, data }
    }
}

impl PrimalityTest<usize> for SegmentedSieve {
    fn is_prime(&self, value: usize) -> bool {
        assert!(self.l <= value && value <= self.r);
        self.data[value - self.l]
    }
}
