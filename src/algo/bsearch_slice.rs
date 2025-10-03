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

/// [`BinarySearch::lower_bound`],[`BinarySearch::upper_bound`]を提供する。
pub trait BinarySearch<T> {
    /// x以上となる最小のindexを求める。
    fn lower_bound(&self, value: &T) -> usize;
    /// xを超える最小のindexを求める。
    fn upper_bound(&self, value: &T) -> usize;
    /// lower_bound, upper_boundの組を求める。
    fn equal_range(&self, value: &T) -> (usize, usize) {
        (self.lower_bound(value), self.upper_bound(value))
    }
}

impl<T: Ord> BinarySearch<T> for [T] {
    /// x以上となる最小のindexを求める。
    ///
    /// **Time complexity** $O(\log n)$
    #[inline]
    fn lower_bound(&self, value: &T) -> usize {
        bsearch_impl!(<, self, value)
    }

    /// xを超える最小のindexを求める。
    ///
    /// **Time complexity** $O(\log n)$
    #[inline]
    fn upper_bound(&self, value: &T) -> usize {
        bsearch_impl!(<=, self, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_lower_bound() {
        let a = [1, 1, 2, 3, 5, 6];
        assert_eq!(a.lower_bound(&1), 0);
        assert_eq!(a.lower_bound(&2), 2);
        assert_eq!(a.lower_bound(&0), 0);
        assert_eq!(a.lower_bound(&4), 4);
        assert_eq!(a.lower_bound(&7), 6);
    }

    #[test]
    fn test_upper_bound() {
        let a = [1, 1, 2, 3, 5, 6];
        assert_eq!(a.upper_bound(&1), 2);
        assert_eq!(a.upper_bound(&2), 3);
        assert_eq!(a.upper_bound(&0), 0);
        assert_eq!(a.upper_bound(&4), 4);
        assert_eq!(a.upper_bound(&7), 6);
    }

    #[test]
    fn test_equal_range() {
        let a = [1, 1, 3, 4, 5, 5, 5, 8];
        assert_eq!(a.equal_range(&1), (0, 2));
        assert_eq!(a.equal_range(&5), (4, 7));
        assert_eq!(a.equal_range(&4), (3, 4));
        assert_eq!(a.equal_range(&6), (7, 7));
    }

    #[test]
    fn test_random() {
        let mut rng = rand::thread_rng();

        let n = 100;
        let a = std::iter::repeat_with(|| rng.gen_range(0..=10))
            .take(n)
            .scan(0, |state, x| {
                *state += x;
                Some(*state)
            })
            .collect::<Vec<_>>();

        for x in 0..=1000 {
            assert_eq!(
                a.lower_bound(&x),
                a.iter()
                    .enumerate()
                    .find(|(_, &y)| y >= x)
                    .map_or(n, |(i, _)| i)
            );

            assert_eq!(
                a.upper_bound(&x),
                a.iter()
                    .enumerate()
                    .find(|(_, &y)| y > x)
                    .map_or(n, |(i, _)| i)
            );
        }
    }
}
