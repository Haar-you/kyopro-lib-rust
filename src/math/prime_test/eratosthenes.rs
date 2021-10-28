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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eratosthenes() {
        let n = 100;
        let sieve = EratosthenesSieve::new(n);

        let primes = (1..=n).filter(|&i| sieve.is_prime(i)).collect::<Vec<_>>();

        assert_eq!(
            primes,
            [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }
}
