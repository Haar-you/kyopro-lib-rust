//! mod mでの累乗

/// $x ^ p \mod m$を求める。
///
/// mは$0 < m < 2^{32}$の範囲に収めること。
///
/// **Time complexity** $O(\log p)$
#[inline]
pub const fn pow_mod(mut x: u64, mut p: u64, m: u64) -> u64 {
    assert!(0 < m && m <= 0xFFFFFFFF, "Violated 0 < m <= 0xFFFFFFFF");
    if x >= m {
        x %= m;
    }

    let mut ret = 1;
    while p > 0 {
        if (p & 1) != 0 {
            ret = (ret * x) % m;
        }
        x = (x * x) % m;
        p >>= 1;
    }
    ret
}
