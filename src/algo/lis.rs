//! 最長増加部分列

use crate::algo::bsearch::lower_bound;

/// 列の最長増加部分列の一つを求める。
///
/// **Time complexity** $O(|a| \log |a|)$
pub fn lis<T>(a: &[T]) -> Vec<usize>
where
    T: Ord + Copy,
{
    let n = a.len();
    let mut dp = vec![];
    let mut pos = vec![];
    let mut prev = vec![None; n];

    for (i, x) in a.iter().enumerate() {
        if dp.is_empty() || dp.last().unwrap() < &x {
            dp.push(x);
            if let Some(&last) = pos.last() {
                prev[i] = Some(last);
            }
            pos.push(i);
        } else {
            let k = lower_bound(&dp, &x);
            dp[k] = x;
            if k > 0 {
                prev[i] = Some(pos[k - 1]);
            }
            pos[k] = i;
        }
    }

    let mut ret = vec![];
    let mut i = Some(*pos.last().unwrap());
    while let Some(j) = i {
        ret.push(j);
        i = prev[j];
    }

    ret.reverse();
    ret
}

#[cfg(test)]
mod tests {}
