//! 高速Ζ変換
use std::ops::Add;

/// $\mathtt{F_j} = \sum_{\mathtt{i \supseteq j}} \mathtt{f_i}$を満たす`F`を求める。
///
/// `f`の長さは$2^n$であるとする。
///
/// **Time complexity** $O(2^n n)$
pub fn fast_zeta_superset<T>(f: &mut [T])
where
    T: Copy + Add<Output = T>,
{
    let n = f.len();
    let t = n.trailing_zeros();
    assert!(n.is_power_of_two());

    for i in 0..t {
        let i = 1 << i;
        for j in 0..n {
            if j & i == 0 {
                f[j] = f[j] + f[j ^ i];
            }
        }
    }
}

/// $\mathtt{F_j} = \sum_{\mathtt{i \subseteq j}} \mathtt{f_i}$を満たす`F`を求める。
///
/// `f`の長さは$2^n$であるとする。
///
/// **Time complexity** $O(2^n n)$
pub fn fast_zeta_subset<T>(f: &mut [T])
where
    T: Copy + Add<Output = T>,
{
    let n = f.len();
    let t = n.trailing_zeros();
    assert!(n.is_power_of_two());

    for i in 0..t {
        let i = 1 << i;
        for j in 0..n {
            if j & i != 0 {
                f[j] = f[j] + f[j ^ i];
            }
        }
    }
}
