//! 2要素の差の絶対値の総和

/// 2要素の差の絶対値の総和
///
/// **Time complexity** $O(|a| \log |a|)$
///
/// # Problems
/// - <https://atcoder.jp/contests/abc186/tasks/abc186_d>
///
/// # Explanation
/// $$\bigstar = \sum_{1 \le i \le N-1} \sum_{i+1 \le j \le N} |a_i - a_j|$$
///
/// $f(i, j) = |a_i - a_j|$は可換なので、$a$を降順にソートされているものとする。
///
/// このとき、$a_i \ge a_j (i \lt j)$なので、絶対値は外してよく、
///
/// $$
/// \begin{aligned}
/// \bigstar &= \sum_{1 \le i \le N-1} \sum_{i+1 \le j \le N} (a_i - a_j) \\\\
/// &= \sum_{1 \le i \le N-1} ((N-i) * a_i - \sum_{i+1 \le j \le N} a_j)
/// \end{aligned}
/// $$
///
/// 各$a_i$が何回だけ加減算されているかを考えると、
/// シグマ内の第一項から$N-i$回足して、第二項から$i-1$回引いているので、
/// $$\bigstar = \sum_{1 \le i \le N} ((N-i)-(i-1))a_i$$
pub fn sum_of_sum_of_difference(mut a: Vec<i64>) -> i64 {
    let mut ret = 0;
    let n = a.len();
    a.sort();

    for (i, x) in a.into_iter().enumerate() {
        ret += x * (i as i64 * 2 + 1 - n as i64);
    }

    ret
}
