//! 単調増加な判定関数上の二分探索
use std::ops::{Add, Div, Sub};

/// [`bsearch_ng_ok`]、[`bsearch_ok_ng`]の返り値
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SearchResult<T> {
    /// `ng`以下で条件を満たさず、`ok`以上で条件を満たす。
    NgOk {
        /// 条件を満たさない最大値
        ng: T,
        /// 条件を満たす最小値
        ok: T,
    },
    /// `ok`以下で条件を満たし、`ng`以上で条件を満たさない。
    OkNg {
        /// 条件を満たす最大値
        ok: T,
        /// 条件を満たさない最小値
        ng: T,
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
/// **Time complexity** $O(\log n)$
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

/// 二分探索
///
/// `f`は、`lower..=upper`の範囲で、ある値を境界にそれ未満では常に`true`、それ以上では常に`false`となる関数
///
/// **Time complexity** $O(\log n)$
pub fn bsearch_ok_ng<
    T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<u8>,
>(
    lower: T,
    upper: T,
    f: impl Fn(T) -> bool,
) -> SearchResult<T> {
    assert!(lower < upper);

    match bsearch_ng_ok(lower, upper, |x| !f(x)) {
        SearchResult::AllNg => SearchResult::AllOk,
        SearchResult::AllOk => SearchResult::AllNg,
        SearchResult::NgOk { ng, ok } => SearchResult::OkNg { ok: ng, ng: ok },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::repeat_n;

    #[test]
    fn test() {
        let n = 100;

        for k in 0..=n {
            let a = repeat_n(0, k)
                .chain(repeat_n(1, n - k))
                .collect::<Vec<u64>>();

            let check = |i| a[i] > 0;

            let res = bsearch_ng_ok(0, n - 1, check);

            match res {
                SearchResult::NgOk { ng, ok } => {
                    assert!(!check(ng));
                    assert!(check(ok));
                    assert_eq!(ng + 1, ok);
                }
                SearchResult::AllOk => {
                    assert!((0..n).all(check));
                }
                SearchResult::AllNg => {
                    assert!((0..n).all(|i| !check(i)));
                }
                _ => {}
            }
        }
    }
}
