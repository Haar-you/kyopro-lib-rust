//! 2要素の最大値の総和

/// 2要素の最大値の総和
///
/// **Time complexity** $O(|a| \log |a|)$
///
/// # Explanation
/// $$\bigstar = \sum_{1 \le i \le N-1} \sum_{i+1 \le j \le N} \max(a_i, a_j)$$
///
/// $f(i, j) = \max(a_i, a_j)$は可換なので、$a$を昇順にソートされているものとする。
///
/// このとき、$a_i$が最大値となる$(i,j)$ペアがいくつあるかを考えると、
/// これは、$i-1$個であるので、
/// $$\bigstar = \sum_{1 \le i \le N} (i-1)a_i$$
pub fn sum_of_sum_of_max(mut a: Vec<i64>) -> i64 {
    a.sort();
    a.into_iter().enumerate().map(|(i, x)| x * i as i64).sum()
}
