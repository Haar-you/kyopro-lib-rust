//! 2要素の和の総和

/// 2要素の和の総和
///
/// Σ{i = 1 ~ N - 1}Σ{j = i + 1 ~ N} aᵢ + aⱼ
///
/// **Time complexity O(|a|)**
pub fn sum_of_sum_of_sum(a: Vec<i64>) -> i64 {
    let n = a.len();
    a.into_iter().map(|x| x * (n - 1) as i64).sum()
}
