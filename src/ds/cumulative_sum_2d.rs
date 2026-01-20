//! 2次元累積和

pub use crate::algebra::traits::Group;
use std::ops::{Index, Range};

/// 2次元の累積和を扱う
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct CumulativeSum2D<G: Group> {
    group: G,
    data: Vec<Vec<G::Element>>,
}

/// [`CumulativeSum2D`]を構築する
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct CumulativeSum2DBuilder<G: Group> {
    group: G,
    data: Vec<Vec<G::Element>>,
    n: usize,
    m: usize,
}

impl<G: Group> CumulativeSum2D<G>
where
    G::Element: Copy,
{
    /// **Time Complexity** $O(1)$
    pub fn fold_2d(
        &self,
        Range { start: l, end: r }: Range<usize>,
        Range { start: d, end: u }: Range<usize>,
    ) -> G::Element {
        let g = &self.group;
        let a = g.inv(self.data[l][u]);
        let b = g.inv(self.data[r][d]);
        let c = self.data[l][d];
        let d = self.data[r][u];

        g.fold_m([a, b, c, d])
    }
}

impl<G: Group> Index<(usize, usize)> for CumulativeSum2D<G> {
    type Output = G::Element;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[i][j]
    }
}

impl<G: Group> CumulativeSum2DBuilder<G>
where
    G::Element: Copy,
{
    /// `CumulativeSum2DBuilder`を生成する
    pub fn new(group: G, n: usize, m: usize) -> Self {
        Self {
            data: vec![vec![group.id(); m + 1]; n + 1],
            group,
            n,
            m,
        }
    }

    /// `[i][j]`番目に`value`を代入する
    pub fn assign(&mut self, i: usize, j: usize, value: G::Element) {
        self.data[i + 1][j + 1] = value;
    }

    /// 群`G`の演算に`[i][j]`番目の値と`value`を適用して`[i][j]`番目の値を更新する。
    pub fn update(&mut self, i: usize, j: usize, value: G::Element) {
        self.data[i + 1][j + 1] = self.group.op(self.data[i + 1][j + 1], value);
    }

    /// [`CumulativeSum2D`]を構築する
    pub fn build(self) -> CumulativeSum2D<G> {
        let mut data = self.data;
        let group = self.group;

        for i in 1..=self.n {
            for j in 0..=self.m {
                data[i][j] = group.op(data[i][j], data[i - 1][j]);
            }
        }
        for data in data.iter_mut().take(self.n + 1) {
            for j in 1..=self.m {
                data[j] = group.op(data[j], data[j - 1]);
            }
        }

        CumulativeSum2D { group, data }
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

        let g = Sum::<i32>::new();
        let mut csb = CumulativeSum2DBuilder::new(g, n, m);

        let mut other = vec![vec![g.id(); m]; n];

        for _ in 0..1000 {
            let i = rng.gen_range(0..n);
            let j = rng.gen_range(0..m);
            let x = rng.gen_range(-1000..=1000);

            csb.update(i, j, x);
            g.op_assign_r(&mut other[i][j], x);
        }

        let cs = csb.build();

        for _ in 0..100 {
            let lr = rand_range(&mut rng, 0..n);
            let du = rand_range(&mut rng, 0..m);

            let ans = other[lr.clone()]
                .iter()
                .map(|v| v[du.clone()].iter().cloned().fold_m(&g))
                .fold_m(&g);

            assert_eq!(cs.fold_2d(lr, du), ans);
        }
    }
}
