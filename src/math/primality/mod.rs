//! 素数判定

pub mod eratosthenes;
pub mod linear_sieve;
pub mod miller_rabin;
pub mod segmented;

/// 素数判定
pub trait PrimalityTest<T> {
    /// `value`が素数ならば`true`を返す。
    fn is_prime(&self, value: T) -> bool;
}

#[cfg(test)]
mod tests {

    use super::{eratosthenes::*, linear_sieve::LinearSieve, miller_rabin::*, PrimalityTest};

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

    #[test]
    fn test_linear_sieve() {
        let n = 10000;
        let sieve1 = EratosthenesSieve::new(n);
        let sieve2 = LinearSieve::new(n);

        for i in 1..=n {
            assert_eq!(sieve1.is_prime(i), sieve2.is_prime(i));
        }
    }

    #[test]
    fn test_miller_rabin() {
        let n = 1000;
        let sieve = EratosthenesSieve::new(n);

        let primes = (1..=n).filter(|&i| sieve.is_prime(i)).collect::<Vec<_>>();

        assert_eq!(
            (1..=n)
                .filter(|&i| MillerRabin.is_prime(i as u64))
                .collect::<Vec<_>>(),
            primes
        );
    }
}
