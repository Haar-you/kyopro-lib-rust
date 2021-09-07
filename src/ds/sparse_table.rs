use crate::{algebra::traits::*, ds::traits::Foldable};
use std::cmp::min;

pub struct SparseTable<T, A> {
    data: Vec<Vec<T>>,
    log_table: Vec<usize>,
    semilattice: A,
}

impl<T, A> SparseTable<T, A>
where
    T: Clone + Default,
    A: BinaryOp<Output = T>,
{
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
        }
    }
}

impl<T, A> Foldable<usize> for SparseTable<T, A>
where
    T: Clone + Default,
    A: BinaryOp<Output = T>,
{
    type Value = Option<T>;

    fn fold(&self, l: usize, r: usize) -> Option<T> {
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
