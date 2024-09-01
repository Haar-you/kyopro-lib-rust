//! xᵖ (mod m)を求める。

/// xᵖ (mod m)を求める。
///
/// **Time complexity O(log p)**
#[inline]
pub fn mod_pow(mut x: u64, mut p: u64, m: u64) -> u64 {
    let mut ret = 1;
    while p > 0 {
        if (p & 1) != 0 {
            ret *= x;
            ret %= m;
        }
        x *= x;
        x %= m;

        p >>= 1;
    }
    ret
}
