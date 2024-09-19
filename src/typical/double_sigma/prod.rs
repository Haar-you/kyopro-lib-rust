//! 2要素の積の総和

/// 2要素の積の総和
///
/// Σ{i = 1 ~ N - 1}Σ{j = i + 1 ~ N} aᵢ * aⱼ
///
/// **Time complexity O(|a|)**
pub fn sum_of_sum_of_prod(a: Vec<i64>) -> i64 {
    let mut ret = 0;
    let mut acc = 0;

    for x in a {
        ret += x * acc;
        acc += x;
    }

    ret
}
