//! 1次元累積和

pub use crate::algebra::traits::Group;
use std::ops::{Index, Range};

/// 1次元の累積和を扱う
#[derive(Debug, Clone)]
pub struct CumulativeSum1D<G: Group> {
    data: Vec<G>,
}

/// [`CumulativeSum1D`]を構築する
pub struct CumulativeSum1DBuilder<G: Group> {
    data: Vec<G>,
}

impl<G: Group + Copy> CumulativeSum1D<G> {
    /// **Time complexity** $O(1)$
    pub fn fold(&self, Range { start: l, end: r }: Range<usize>) -> G {
        self.data[r].op(self.data[l].inv())
    }
}

impl<G: Group> Index<usize> for CumulativeSum1D<G> {
    type Output = G;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<G: Group + Copy> CumulativeSum1DBuilder<G> {
    /// `CumulativeSum1DBuilder`を生成する
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![G::id(); n + 1],
        }
    }

    /// `i`番目に`value`を代入する
    pub fn assign(&mut self, i: usize, value: G) {
        self.data[i + 1] = value;
    }

    /// 群`G`の演算に`i`番目の値と`value`を適用して`i`番目の値を更新する。
    pub fn update(&mut self, i: usize, value: G) {
        self.data[i + 1] = self.data[i + 1].op(value);
    }

    /// [`CumulativeSum1D`]を構築する
    pub fn build(self) -> CumulativeSum1D<G> {
        let data = self
            .data
            .iter()
            .scan(G::id(), |st, &x| {
                *st = st.op(x);
                Some(*st)
            })
            .collect::<Vec<_>>();

        CumulativeSum1D { data }
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
        let mut csb = CumulativeSum1DBuilder::<Sum<i32>>::new(n);

        let mut other = vec![Sum::id(); n];

        for _ in 0..1000 {
            let i = rng.gen_range(0..n);
            let x = rng.gen_range(-1000..=1000);

            csb.update(i, Sum(x));
            other[i].op_assign_r(Sum(x));
        }

        let cs = csb.build();

        for _ in 0..100 {
            let range = rand_range(&mut rng, 0..n);
            let ans = other[range.clone()].iter().cloned().fold_m();
            assert_eq!(cs.fold(range), ans);
        }
    }
}
