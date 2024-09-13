//! a!の素因数pの個数を求める。
//!
//! # References
//! - <https://ja.wikipedia.org/wiki/%E3%83%AB%E3%82%B8%E3%83%A3%E3%83%B3%E3%83%89%E3%83%AB%E3%81%AE%E5%85%AC%E5%BC%8F>
//!
//! # Problems
//! - <https://yukicoder.me/problems/no/2380>

/// a!の素因数pの個数を求める。
///
/// **Time Complexity O(log a)**
pub fn factorial_prime_factor(a: u64, p: u64) -> u64 {
    let mut ret = 0;
    let mut q = p;

    while q <= a {
        ret += a / q;
        if let Some(q_) = q.checked_mul(p) {
            q = q_;
        } else {
            break;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for p in vec![2, 3, 5, 7, 11, 13] {
            let a = 1000;

            let mut ans = 0;

            for mut x in 1..=a {
                while x % p == 0 {
                    ans += 1;
                    x /= p;
                }
            }

            assert_eq!(factorial_prime_factor(a, p), ans);
        }
    }
}
