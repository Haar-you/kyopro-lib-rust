//! 冪等性と結合性をもつ2次元列の区間取得($O(1)$)ができる。
use crate::algebra::traits::*;
use std::{
    cmp::{max, min},
    ops::Range,
};

/// 冪等性と結合性をもつ2次元列の区間取得($O(1)$)ができる。
pub struct SparseTable2D<A: Semigroup + Idempotence> {
    data: Vec<Vec<Vec<Vec<A::Element>>>>,
    log_table: Vec<usize>,
    semilattice: A,
}

impl<A: Semigroup + Idempotence> SparseTable2D<A>
where
    A::Element: Clone + Default,
{
    /// **Time complexity** $O(nm \log n \log m)$
    ///
    /// **Space complexity** $O(nm \log n \log m)$
    pub fn new(s: Vec<Vec<A::Element>>, a: A) -> Self {
        let n = s.len();
        let m = s[0].len();
        let logn = n.next_power_of_two().trailing_zeros() as usize + 1;
        let logm = m.next_power_of_two().trailing_zeros() as usize + 1;

        let mut data = vec![vec![vec![vec![A::Element::default(); logm]; m]; logn]; n];

        for i in 0..n {
            for j in 0..m {
                data[i][0][j][0] = s[i][j].clone();
            }

            for y in 1..logm {
                for j in 0..m {
                    data[i][0][j][y] = a.op(
                        data[i][0][j][y - 1].clone(),
                        data[i][0][min(m - 1, j + (1 << (y - 1)))][y - 1].clone(),
                    );
                }
            }
        }

        for x in 1..logn {
            for i in 0..n {
                for y in 0..logm {
                    for j in 0..m {
                        data[i][x][j][y] = a.op(
                            data[i][x - 1][j][y].clone(),
                            data[min(n - 1, i + (1 << (x - 1)))][x - 1][j][y].clone(),
                        );
                    }
                }
            }
        }

        let mut log_table = vec![0; max(n, m) + 1];
        for i in 2..=max(n, m) {
            log_table[i] = log_table[i >> 1] + 1;
        }

        Self {
            data,
            log_table,
            semilattice: a,
        }
    }

    /// **Time complexity** $O(1)$
    pub fn fold_2d(
        &self,
        Range { start: r1, end: r2 }: Range<usize>,
        Range { start: c1, end: c2 }: Range<usize>,
    ) -> Option<A::Element> {
        if r1 == r2 || c1 == c2 {
            return None;
        }
        let kr = self.log_table[r2 - r1];
        let kc = self.log_table[c2 - c1];

        let x = self.semilattice.op(
            self.data[r1][kr][c1][kc].clone(),
            self.data[r1][kr][c2 - (1 << kc)][kc].clone(),
        );
        let y = self.semilattice.op(
            self.data[r2 - (1 << kr)][kr][c1][kc].clone(),
            self.data[r2 - (1 << kr)][kr][c2 - (1 << kc)][kc].clone(),
        );

        Some(self.semilattice.op(x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{algebra::max::*, iter::collect::CollectVec};
    use rand::Rng;
    use std::fmt::Debug;

    fn test<A>(s: Vec<Vec<A::Element>>, a: A)
    where
        A: Semigroup + Idempotence + Identity + Copy,
        A::Element: Clone + Default + PartialEq + Debug + Copy,
    {
        let st = SparseTable2D::new(s.clone(), a);
        let n = s.len();
        let m = s[0].len();

        for x1 in 0..n {
            for x2 in x1..=n {
                for y1 in 0..m {
                    for y2 in y1..=m {
                        let ans = &s[x1..x2].iter().fold(a.id(), |x, v| {
                            a.op(x, v[y1..y2].iter().fold(a.id(), |p, q| a.op(p, *q)))
                        });

                        assert_eq!(*ans, st.fold_2d(x1..x2, y1..y2).unwrap_or(a.id()));
                    }
                }
            }
        }
    }

    #[test]
    fn test_max() {
        let mut rng = rand::thread_rng();
        let n = 30;
        let m = 30;
        let s = (0..n)
            .map(|_| (0..m).map(|_| rng.gen::<u64>()).collect_vec())
            .collect_vec();
        let a = Max::<u64>::new();
        test(s, a);
    }
}
