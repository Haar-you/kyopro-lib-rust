use std::ops::{Add, Div, Sub};

/// [`bsearch_ng_ok`]の返り値
pub enum SearchResult<T> {
    /// `ng`以下で条件を満たさず、`ok`以上で条件を満たす。
    NgOk {
        /// 条件を満たさない最大値
        ng: T,
        /// 条件を満たす最小値
        ok: T,
    },
    /// 全体で条件を満たす。
    AllOk,
    /// 全体で条件を満たさない。
    AllNg,
}

/// 二分探索
///
/// `f`は、`lower..=upper`の範囲で、ある値を境界にそれ未満では常に`false`、それ以上では常に`true`となる関数
///
/// **Time complexity O(log n)**
pub fn bsearch_ng_ok<
    T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<u8>,
>(
    mut lower: T,
    mut upper: T,
    f: impl Fn(T) -> bool,
) -> SearchResult<T> {
    assert!(lower < upper);

    if f(lower) {
        // all ok
        return SearchResult::AllOk;
    } else if !f(upper) {
        // all ng
        return SearchResult::AllNg;
    }

    while upper - lower > T::from(1) {
        let mid = (lower + upper) / T::from(2);

        if f(mid) {
            upper = mid;
        } else {
            lower = mid
        }
    }

    SearchResult::NgOk {
        ng: lower,
        ok: upper,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::repeat;

    #[test]
    fn test() {
        let n = 100;

        for k in 0..=n {
            let a = repeat(0)
                .take(k)
                .chain(repeat(1).take(n - k))
                .collect::<Vec<u64>>();

            let check = |i| a[i as usize] > 0;

            let res = bsearch_ng_ok(0, n - 1, check);

            match res {
                SearchResult::NgOk { ng, ok } => {
                    assert!(!check(ng));
                    assert!(check(ok));
                    assert_eq!(ng + 1, ok);
                }
                SearchResult::AllOk => {
                    assert!((0..n).all(|i| check(i)));
                }
                SearchResult::AllNg => {
                    assert!((0..n).all(|i| !check(i)));
                }
            }
        }
    }
}
