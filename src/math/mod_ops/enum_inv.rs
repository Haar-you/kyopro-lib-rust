//! 0,1,..,nの素数mod pでの逆元を列挙する。

/// 0,1,..,nの素数mod pでの逆元を列挙する。
///
/// **Time complexity** $O(n)$
#[inline]
pub fn enumerate_mod_inv(n: usize, p: u64) -> Vec<u64> {
    let mut ret = vec![0; n + 1];

    ret[1] = 1;

    for i in 2..=n {
        ret[i] = (p / i as u64) * (p - ret[(p % i as u64) as usize]) % p;
    }

    ret
}
