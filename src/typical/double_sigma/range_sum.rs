//! 区間和の総和

/// 区間和の総和
///
/// **Time complexity** $O(|a|)$
///
/// # Explanation
/// $$\bigstar = \sum_{1 \le i \le n} \sum_{i \le j \le n} (a_i + \ldots + a_j)$$
///
/// 各$a_i$について、それを含む総和$a_x + \ldots + a_i + \ldots + a_y$がいくつあるかを考えると、
/// これは$i \times (n - i + 1)$回だけあるので、
/// $$\bigstar = \sum_{1 \le i \le n} i \times (n-i+1) \times a_i$$
pub fn sum_of_sum_of_range_sum(a: Vec<i64>) -> i64 {
    let n = a.len();
    a.into_iter()
        .enumerate()
        .map(|(i, x)| x * (i + 1) as i64 * (n - i) as i64)
        .sum()
}
