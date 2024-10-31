//! 区間スケジューリング問題

/// 半開区間の集合から共通部分を含まないような部分集合のうち、要素数が最大となるものを求める。
///
/// **Time complexity** $O(n \log n)$
pub fn interval_scheduling<T: Ord + Copy>(intervals: &[(T, T)]) -> Vec<usize> {
    let n = intervals.len();
    let mut ret = vec![];
    let mut ord = (0..n).collect::<Vec<_>>();
    ord.sort_by(|&i, &j| intervals[i].1.cmp(&intervals[j].1));

    let mut r = None;

    for i in ord {
        if r.is_none() || intervals[i].0 >= r.unwrap() {
            ret.push(i);
            r = Some(intervals[i].1);
        }
    }

    ret
}
