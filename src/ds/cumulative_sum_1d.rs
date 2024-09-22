//! 1次元累積和

pub use crate::algebra::traits::Group;
use std::ops::{Index, Range};

/// 1次元の累積和を扱う
#[derive(Debug, Clone)]
pub struct CumulativeSum1D<G: Group> {
    data: Vec<G::Output>,
    group: G,
}

/// [`CumulativeSum1D`]を構築する
pub struct CumulativeSum1DBuilder<G: Group> {
    data: Vec<G::Output>,
    group: G,
}

impl<T: Copy, G: Group<Output = T>> CumulativeSum1D<G> {
    /// Time complexity O(1)
    pub fn fold(&self, Range { start: l, end: r }: Range<usize>) -> T {
        self.group.op(self.data[r], self.group.inv(self.data[l]))
    }
}

impl<T, G: Group<Output = T>> Index<usize> for CumulativeSum1D<G> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<T: Copy, G: Group<Output = T>> CumulativeSum1DBuilder<G> {
    /// `CumulativeSum1DBuilder`を生成する
    pub fn new(n: usize, group: G) -> Self {
        CumulativeSum1DBuilder {
            data: vec![group.id(); n + 1],
            group,
        }
    }

    /// `i`番目に`value`を代入する
    pub fn assign(&mut self, i: usize, value: T) {
        self.data[i + 1] = value;
    }

    /// 群`G`の演算に`i`番目の値と`value`を適用して`i`番目の値を更新する。
    pub fn update(&mut self, i: usize, value: T) {
        self.data[i + 1] = self.group.op(self.data[i + 1], value);
    }

    /// [`CumulativeSum1D`]を構築する
    pub fn build(self) -> CumulativeSum1D<G> {
        let data = self
            .data
            .iter()
            .scan(self.group.id(), |st, &x| {
                *st = self.group.op(*st, x);
                Some(*st)
            })
            .collect::<Vec<_>>();

        CumulativeSum1D {
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
        let mut csb = CumulativeSum1DBuilder::new(n, Sum::<i32>::new());

        let mut other = vec![0; n];

        for _ in 0..1000 {
            let i = rng.gen_range(0..n);
            let x = rng.gen_range(-1000..=1000);

            csb.update(i, x);
            other[i] += x;
        }

        let cs = csb.build();

        for _ in 0..100 {
            let range = rand_range(&mut rng, 0..n);

            let mut ans = 0;
            for i in range.clone() {
                ans += other[i];
            }

            assert_eq!(cs.fold(range), ans);
        }
    }
}
