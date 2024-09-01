use std::ops::{Add, Div, Sub};

pub enum SearchResult<T> {
    NgOk { ng: T, ok: T },
    AllOk,
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
