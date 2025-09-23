//! Eratosthenesの篩

pub use crate::math::primality::PrimalityTest;

/// Eratosthenesの篩
pub struct EratosthenesSieve {
    data: Vec<bool>,
}

impl EratosthenesSieve {
    /// `size`までの自然数の素数判定ができる`EratosthenesSieve`を構築する。
    pub fn new(size: usize) -> Self {
        let mut data = vec![true; size.div_ceil(2)];
        data[0] = false;

        let mut i = 3;
        while i * i <= size {
            if !data[i / 2] {
                i += 2;
                continue;
            }

            let mut j = i * i;
            while j <= size {
                data[j / 2] = false;
                j += 2 * i;
            }

            i += 2;
        }

        EratosthenesSieve { data }
    }
}

impl PrimalityTest<usize> for EratosthenesSieve {
    fn is_prime(&self, i: usize) -> bool {
        if i == 2 {
            true
        } else if i % 2 == 0 {
            false
        } else {
            self.data[i / 2]
        }
    }
}
