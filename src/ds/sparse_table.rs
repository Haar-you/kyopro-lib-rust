//! 冪等性と結合性をもつ列の区間取得($O(1)$)ができる。

use crate::algebra::traits::*;
use crate::misc::range::range_bounds_to_range;
use std::{cmp::min, ops::RangeBounds};

/// 冪等性と結合性をもつ列の区間取得($O(1)$)ができる。
pub struct SparseTable<A: Semilattice> {
    semilattice: A,
    data: Vec<Vec<A::Element>>,
    log_table: Vec<usize>,
    original_size: usize,
}

impl<A: Semilattice> SparseTable<A>
where
    A::Element: Clone + Default,
{
    /// **Time complexity** $O(n \log n)$
    ///
    /// **Space complexity** $O(n \log n)$
    pub fn new(semilattice: A, s: Vec<A::Element>) -> Self {
        let n = s.len();
        let logn = n.next_power_of_two().trailing_zeros() as usize + 1;

        let mut data = vec![vec![A::Element::default(); n]; logn];

        data[0] = s;

        for i in 1..logn {
            for j in 0..n {
                data[i][j] = semilattice.op(
                    data[i - 1][j].clone(),
                    data[i - 1][min(n - 1, j + (1 << (i - 1)))].clone(),
                );
            }
        }

        let mut log_table = vec![0; n + 1];
        for i in 2..=n {
            log_table[i] = log_table[i >> 1] + 1;
        }

        Self {
            semilattice,
            data,
            log_table,
            original_size: n,
        }
    }

    /// **Time complexity** $O(1)$
    pub fn fold(&self, range: impl RangeBounds<usize>) -> Option<A::Element> {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);

        if l >= r {
            None
        } else {
            let k = self.log_table[r - l];
            Some(
                self.semilattice
                    .op(self.data[k][l].clone(), self.data[k][r - (1 << k)].clone()),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::algebra::{
        bit::{BitAnd, BitOr},
        min_max::{Max, Min},
    };

    use super::*;
    use rand::Rng;

    fn test<A>(a: A, s: Vec<A::Element>)
    where
        A: Semilattice + Identity + Clone,
        A::Element: Copy + Default + PartialEq + Debug,
    {
        let st = SparseTable::new(a.clone(), s.clone());

        for l in 0..s.len() {
            for r in l..=s.len() {
                let ans = &s[l..r].iter().cloned().fold_m(&a);
                assert_eq!(*ans, st.fold(l..r).unwrap_or(a.id()));
            }
        }
    }

    #[test]
    fn test_max() {
        let mut rng = rand::thread_rng();
        let n = 100;
        let s = std::iter::repeat_with(|| rng.gen::<u64>())
            .take(n)
            .collect::<Vec<_>>();
        test(Max::<u64>::new(), s);
    }

    #[test]
    fn test_min() {
        let mut rng = rand::thread_rng();
        let n = 100;
        let s = std::iter::repeat_with(|| rng.gen::<u64>())
            .take(n)
            .collect::<Vec<_>>();
        test(Min::<u64>::new(), s);
    }

    #[test]
    fn test_bitand() {
        let mut rng = rand::thread_rng();
        let n = 100;
        let s = std::iter::repeat_with(|| rng.gen::<u64>())
            .take(n)
            .collect::<Vec<_>>();
        test(BitAnd::<u64>::new(), s);
    }

    #[test]
    fn test_bitor() {
        let mut rng = rand::thread_rng();
        let n = 100;
        let s = std::iter::repeat_with(|| rng.gen::<u64>())
            .take(n)
            .collect::<Vec<_>>();
        test(BitOr::<u64>::new(), s);
    }
}
