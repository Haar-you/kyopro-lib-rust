//! 最長増加部分列

use crate::algo::bsearch::lower_bound;

/// 列の最長増加部分列の一つを求める。
///
/// # Complexity
/// Time complexity $O(n \log(n))$
pub fn lis<T>(a: &[T]) -> Vec<usize>
where
    T: Ord + Copy,
{
    let n = a.len();
    let mut dp = vec![];
    let mut pos = vec![];
    let mut prev = vec![None; n];
    let mut ret = vec![];

    for i in 0..n {
        let x = a[i];
        if dp.is_empty() || dp.last().unwrap() < &x {
            dp.push(x);
            if !pos.is_empty() {
                prev[i] = Some(*pos.last().unwrap());
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
