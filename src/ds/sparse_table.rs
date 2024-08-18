//! 冪等性と結合性をもつ列の区間取得(O(1))

use crate::{algebra::traits::*, ds::traits::Foldable};
use std::{cmp::min, ops::Range};

pub struct SparseTable<A: BinaryOp + Associative + Idempotence> {
    data: Vec<Vec<A::Output>>,
    log_table: Vec<usize>,
    semilattice: A,
}

impl<T: Clone + Default, A: BinaryOp<Output = T> + Associative + Idempotence> SparseTable<A> {
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

impl<T: Clone + Default, A: BinaryOp<Output = T> + Associative + Idempotence> Foldable<Range<usize>>
    for SparseTable<A>
{
    type Output = Option<T>;

    fn fold(&self, Range { start: l, end: r }: Range<usize>) -> Self::Output {
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
