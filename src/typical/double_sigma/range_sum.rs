//! 区間和の総和

/// 区間和の総和
///
/// Σ{i = 1 ~ N}Σ{j = i ~ N} aᵢ + aᵢ ₊ ₁ + ... + aⱼ
///
/// **Time complexity** $O(|a|)$
pub fn sum_of_sum_of_range_sum(a: Vec<i64>) -> i64 {
    let n = a.len();
    a.into_iter()
        .enumerate()
        .map(|(i, x)| x * (i + 1) as i64 * (n - i) as i64)
        .sum()
}
