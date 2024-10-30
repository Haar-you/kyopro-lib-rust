use crate::math::prime_test::traits::*;

pub struct EratosthenesSieve {
    data: Vec<bool>,
}

impl EratosthenesSieve {
    pub fn new(size: usize) -> Self {
        let mut data = vec![true; (size + 1) / 2];
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

impl CheckPrime<usize> for EratosthenesSieve {
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
