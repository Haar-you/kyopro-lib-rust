//! 2要素の最大値の総和

/// 2要素の最大値の総和
///
/// Σ{i = 1 ~ N - 1}Σ{j = i + 1 ~ N} max(aᵢ, aⱼ)
///
/// **Time complexity O(|a| log |a|)**
pub fn sum_of_sum_of_max(mut a: Vec<i64>) -> i64 {
    a.sort();
    a.into_iter().enumerate().map(|(i, x)| x * i as i64).sum()
}
