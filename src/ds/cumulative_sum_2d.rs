//! 2次元累積和

pub use crate::algebra::traits::Group;
pub use crate::ds::traits::Foldable2D;
use std::ops::{Index, Range};

#[derive(Debug, Clone)]
pub struct CumulativeSum2D<T, G> {
    data: Vec<Vec<T>>,
    group: G,
}

pub struct CumulativeSum2DBuilder<T, G> {
    data: Vec<Vec<T>>,
    group: G,
    n: usize,
    m: usize,
}

impl<T, G> Foldable2D<Range<usize>> for CumulativeSum2D<T, G>
where
    T: Copy,
    G: Group<Output = T>,
{
    type Output = T;

    /// Time Complexity O(1)
    fn fold(
        &self,
        Range { start: l, end: r }: Range<usize>,
        Range { start: d, end: u }: Range<usize>,
    ) -> Self::Output {
        let a = self.group.inv(self.data[l][u]);
        let b = self.group.inv(self.data[r][d]);
        let c = self.data[l][d];
        let d = self.data[r][u];

        self.group.op(a, self.group.op(b, self.group.op(c, d)))
    }
}

impl<T, G> Index<(usize, usize)> for CumulativeSum2D<T, G> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[i][j]
    }
}

impl<T, G> CumulativeSum2DBuilder<T, G>
where
    T: Copy,
    G: Group<Output = T> + Clone,
{
    pub fn new(n: usize, m: usize, group: G) -> Self {
        CumulativeSum2DBuilder {
            data: vec![vec![group.id(); m + 1]; n + 1],
            group,
            n,
            m,
        }
    }

    pub fn update(&mut self, i: usize, j: usize, value: T) {
        self.data[i + 1][j + 1] = self.group.op(self.data[i + 1][j + 1], value);
    }

    pub fn build(self) -> CumulativeSum2D<T, G> {
        let mut data = self.data;

        for i in 1..=self.n {
            for j in 0..=self.m {
                data[i][j] = self.group.op(data[i][j], data[i - 1][j]);
            }
        }
        for i in 0..=self.n {
            for j in 1..=self.m {
                data[i][j] = self.group.op(data[i][j], data[i][j - 1]);
            }
        }

        CumulativeSum2D {
            data,
            group: self.group,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::sum::*;
    use crate::testtools::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 20;
        let m = 30;
        let mut csb = CumulativeSum2DBuilder::new(n, m, Sum::<i32>::new());

        let mut other = vec![vec![0; m]; n];

        for _ in 0..1000 {
            let i = rng.gen_range(0..n);
            let j = rng.gen_range(0..m);
            let x = rng.gen_range(-1000..=1000);

            csb.update(i, j, x);
            other[i][j] += x;
        }

        let cs = csb.build();

        for _ in 0..100 {
            let lr = rand_range(&mut rng, 0..n);
            let du = rand_range(&mut rng, 0..m);

            let mut ans = 0;
            for i in lr.clone() {
                for j in du.clone() {
                    ans += other[i][j];
                }
            }

            assert_eq!(cs.fold(lr, du), ans);
        }
    }
}
