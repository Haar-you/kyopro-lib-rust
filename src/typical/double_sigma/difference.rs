/// 2要素の絶対値の総和
///
/// Σ{i = 1 ~ N - 1}Σ{j = i + 1 ~ N} |aᵢ - aⱼ|
///
/// **Time complexity O(|a| log |a|)**
///
/// # Problems
/// - <https://atcoder.jp/contests/abc186/tasks/abc186_d>
pub fn sum_of_sum_of_difference(mut a: Vec<i64>) -> i64 {
    let mut ret = 0;
    let n = a.len();
    a.sort();

    for (i, x) in a.into_iter().enumerate() {
        ret += x * (i as i64 * 2 + 1 - n as i64);
    }

    ret
}
