//! a!の素因数pの個数を求める。

/// a!の素因数pの個数を求める。
///
/// **Time Complexity O(log a)**
#[inline]
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
