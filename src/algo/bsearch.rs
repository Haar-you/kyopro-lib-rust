//! 二分探索

macro_rules! bsearch_impl {
    ($t:tt, $a:expr, $value:expr) => {{
        let n = $a.len();
        let mut b = 0;
        let mut len = n;

        while len > 0 {
            let half = len / 2;
            let mid = b + half;

            if &$a[mid] $t $value {
                len -= half + 1;
                b = mid + 1;
            } else {
                len = half;
            }
        }

        b
    }}
}

/// x以上となる最小のindexを求める。
///
/// **Time complexity** $O(\log n)$
#[inline]
pub fn lower_bound<T: Ord>(a: &[T], value: &T) -> usize {
    bsearch_impl!(<, a, value)
}

/// xを超える最小のindexを求める。
///
/// **Time complexity** $O(\log n)$
#[inline]
pub fn upper_bound<T: Ord>(a: &[T], value: &T) -> usize {
    bsearch_impl!(<=, a, value)
}

/// lower_bound, upper_boundの組を求める。
///
/// **Time complexity** $O(\log n)$
#[inline]
pub fn equal_range<T: Ord>(a: &[T], value: &T) -> (usize, usize) {
    (lower_bound(a, value), upper_bound(a, value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_lower_bound() {
        let a = vec![1, 1, 2, 3, 5, 6];
        assert_eq!(lower_bound(&a, &1), 0);
        assert_eq!(lower_bound(&a, &2), 2);
        assert_eq!(lower_bound(&a, &0), 0);
        assert_eq!(lower_bound(&a, &4), 4);
        assert_eq!(lower_bound(&a, &7), 6);
    }

    #[test]
    fn test_upper_bound() {
        let a = vec![1, 1, 2, 3, 5, 6];
        assert_eq!(upper_bound(&a, &1), 2);
        assert_eq!(upper_bound(&a, &2), 3);
        assert_eq!(upper_bound(&a, &0), 0);
        assert_eq!(upper_bound(&a, &4), 4);
        assert_eq!(upper_bound(&a, &7), 6);
    }

    #[test]
    fn test_equal_range() {
        let a = vec![1, 1, 3, 4, 5, 5, 5, 8];
        assert_eq!(equal_range(&a, &1), (0, 2));
        assert_eq!(equal_range(&a, &5), (4, 7));
        assert_eq!(equal_range(&a, &4), (3, 4));
        assert_eq!(equal_range(&a, &6), (7, 7));
    }

    #[test]
    fn test_random() {
        let mut rng = rand::thread_rng();

        let n = 100;
        let a = (0..n)
            .map(|_| rng.gen_range(0..=10))
            .scan(0, |state, x| {
                *state += x;
                Some(*state)
            })
            .collect::<Vec<_>>();

        for x in 0..=1000 {
            assert_eq!(
                lower_bound(&a, &x),
                a.iter()
                    .enumerate()
                    .find(|(_, &y)| y >= x)
                    .map(|(i, _)| i)
                    .unwrap_or(n)
            );

            assert_eq!(
                upper_bound(&a, &x),
                a.iter()
                    .enumerate()
                    .find(|(_, &y)| y > x)
                    .map(|(i, _)| i)
                    .unwrap_or(n)
            );
        }
    }
}
