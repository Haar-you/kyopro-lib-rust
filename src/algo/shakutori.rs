//! 尺取り法
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc038/tasks/abc038_c>
//! - <https://atcoder.jp/contests/arc022/tasks/arc022_2>

/// 尺取り法
///
/// * remove_left - `|l: usize| {尺取りの左端を縮めたときの操作}`
/// * can_append_right - `|l: usize, r: usize| {l..r+1が条件を満たすかを判定}`
/// * append_right - `|r: usize| {尺取りの右端を進めたときの操作}`
pub fn shakutori(
    n: usize,
    mut remove_left: impl FnMut(usize),
    can_append_right: impl Fn(usize, usize) -> bool,
    mut append_right: impl FnMut(usize),
    mut f: impl FnMut(usize, usize),
) {
    let mut right = 0;

    for left in 0..n {
        if right < left {
            right = left;
        }

        while right < n {
            if can_append_right(left, right) {
                append_right(right);
                right += 1;
            } else {
                break;
            }
        }

        f(left, right);

        if left < right {
            remove_left(left);
        }
    }
}
