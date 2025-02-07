//! 2要素のXORの総和

/// 2要素のXORの総和
///
/// Σ{i = 1 ~ N - 1}Σ{j = i + 1 ~ N} aᵢ ⊕ aⱼ
///
/// **Time complexity** $O(64 * |a|)$
///
/// # Problems
/// - <https://atcoder.jp/contests/abc147/tasks/abc147_d>
pub fn sum_of_sum_of_xor(a: Vec<u64>) -> u128 {
    let mut ret = 0;

    for i in 0..64 {
        let mut c = [0, 0];

        for &x in &a {
            if x & (1 << i) != 0 {
                ret += c[0] << i;
                c[1] += 1;
            } else {
                ret += c[1] << i;
                c[0] += 1;
            }
        }
    }

    ret
}
