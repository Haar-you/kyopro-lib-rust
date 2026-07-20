//! mod p(素数)での逆元の列挙

/// 0,1,..,nのmod pでの逆元を列挙する。
///
/// $p$は$0 < p < 2^{32}$の範囲の素数であること。
///
/// **Time complexity** $O(n)$
#[inline]
pub fn enumerate_inv_mod(n: usize, p: u64) -> Vec<u64> {
    assert!(
        0 < p && p <= 0xFFFFFFFF,
        "Violated 0 < p <= 0xFFFFFFFF (p = {p})"
    );

    let mut ret = vec![0; n + 1];

    ret[1] = 1;

    for i in 2..=n {
        ret[i] = (p / i as u64) * (p - ret[(p % i as u64) as usize]) % p;
    }

    ret
}
