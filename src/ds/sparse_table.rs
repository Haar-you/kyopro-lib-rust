//! 冪等性と結合性をもつ列の区間取得(O(1))

use crate::algebra::traits::*;
use crate::utils::range::range_bounds_to_range;
use std::{cmp::min, ops::RangeBounds};

pub struct SparseTable<A: BinaryOp + Associative + Idempotence> {
    data: Vec<Vec<A::Output>>,
    log_table: Vec<usize>,
    semilattice: A,
    original_size: usize,
}

impl<T: Clone + Default, A: BinaryOp<Output = T> + Associative + Idempotence> SparseTable<A> {
    /// **Time complexity O(n log n)**
    ///
    /// **Space complexity O(n log n)**
    pub fn new(s: Vec<T>, a: A) -> Self {
        let n = s.len();
        let logn = n.next_power_of_two().trailing_zeros() as usize + 1;

        let mut data = vec![vec![T::default(); n]; logn];

        data[0] = s;

        for i in 1..logn {
            for j in 0..n {
                data[i][j] = a.op(
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
            data,
            log_table,
            semilattice: a,
            original_size: n,
        }
    }

    /// **Time complexity O(1)**
    pub fn fold(&self, range: impl RangeBounds<usize>) -> Option<T> {
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

    use crate::algebra::{bitand::BitAnd, bitor::BitOr, max::Max, min::Min};

    use super::*;
    use rand::Rng;

    fn test<T, A>(s: Vec<T>, a: A)
    where
        T: Clone + Default + PartialEq + Debug + Copy,
        A: BinaryOp<Output = T> + Associative + Idempotence + Identity + Clone,
    {
        let st = SparseTable::new(s.clone(), a.clone());

        for l in 0..s.len() {
            for r in l..=s.len() {
                let ans = &s[l..r].iter().fold(a.id(), |x, y| a.op(x, *y));
                assert_eq!(*ans, st.fold(l..r).unwrap_or(a.id()));
            }
        }
    }

    #[test]
    fn test_max() {
        let mut rng = rand::thread_rng();
        let n = 100;
        let s = (0..n).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();
        let a = Max::<u64>::new();
        test(s, a);
    }

    #[test]
    fn test_min() {
        let mut rng = rand::thread_rng();
        let n = 100;
        let s = (0..n).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();
        let a = Min::<u64>::new();
        test(s, a);
    }

    #[test]
    fn test_bitand() {
        let mut rng = rand::thread_rng();
        let n = 100;
        let s = (0..n).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();
        let a = BitAnd::<u64>::new();
        test(s, a);
    }

    #[test]
    fn test_bitor() {
        let mut rng = rand::thread_rng();
        let n = 100;
        let s = (0..n).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();
        let a = BitOr::<u64>::new();
        test(s, a);
    }
}
