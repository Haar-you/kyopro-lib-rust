//! 2次元累積和

pub use crate::algebra::traits::Group;
use std::ops::{Index, Range};

/// 2次元の累積和を扱う
#[derive(Debug, Clone)]
pub struct CumulativeSum2D<G: Group> {
    data: Vec<Vec<G>>,
}

/// [`CumulativeSum2D`]を構築する
pub struct CumulativeSum2DBuilder<G: Group> {
    data: Vec<Vec<G>>,
    n: usize,
    m: usize,
}

impl<G: Group + Copy> CumulativeSum2D<G> {
    /// **Time Complexity** $O(1)$
    pub fn fold_2d(
        &self,
        Range { start: l, end: r }: Range<usize>,
        Range { start: d, end: u }: Range<usize>,
    ) -> G {
        let a = self.data[l][u].inv();
        let b = self.data[r][d].inv();
        let c = self.data[l][d];
        let d = self.data[r][u];

        a.op(b.op(c.op(d)))
    }
}

impl<G: Group> Index<(usize, usize)> for CumulativeSum2D<G> {
    type Output = G;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[i][j]
    }
}

impl<G: Group + Copy> CumulativeSum2DBuilder<G> {
    /// `CumulativeSum2DBuilder`を生成する
    pub fn new(n: usize, m: usize) -> Self {
        CumulativeSum2DBuilder {
            data: vec![vec![G::id(); m + 1]; n + 1],
            n,
            m,
        }
    }

    /// `[i][j]`番目に`value`を代入する
    pub fn assign(&mut self, i: usize, j: usize, value: G) {
        self.data[i + 1][j + 1] = value;
    }

    /// 群`G`の演算に`[i][j]`番目の値と`value`を適用して`[i][j]`番目の値を更新する。
    pub fn update(&mut self, i: usize, j: usize, value: G) {
        self.data[i + 1][j + 1] = self.data[i + 1][j + 1].op(value);
    }

    /// [`CumulativeSum2D`]を構築する
    pub fn build(self) -> CumulativeSum2D<G> {
        let mut data = self.data;

        for i in 1..=self.n {
            for j in 0..=self.m {
                data[i][j] = data[i][j].op(data[i - 1][j]);
            }
        }
        for data in data.iter_mut().take(self.n + 1) {
            for j in 1..=self.m {
                data[j] = data[j].op(data[j - 1]);
            }
        }

        CumulativeSum2D { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::sum::*;
    use my_testtools::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 20;
        let m = 30;
        let mut csb = CumulativeSum2DBuilder::<Sum<i32>>::new(n, m);

        let mut other = vec![vec![Sum::id(); m]; n];

        for _ in 0..1000 {
            let i = rng.gen_range(0..n);
            let j = rng.gen_range(0..m);
            let x = rng.gen_range(-1000..=1000);

            csb.update(i, j, Sum(x));
            other[i][j].op_assign(Sum(x));
        }

        let cs = csb.build();

        for _ in 0..100 {
            let lr = rand_range(&mut rng, 0..n);
            let du = rand_range(&mut rng, 0..m);

            let ans = other[lr.clone()]
                .iter()
                .map(|v| v[du.clone()].iter().cloned().fold_m())
                .fold_m();

            assert_eq!(cs.fold_2d(lr, du), ans);
        }
    }
}
