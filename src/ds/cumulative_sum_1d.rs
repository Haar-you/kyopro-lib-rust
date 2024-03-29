//! 1次元累積和

pub use crate::algebra::traits::Group;
pub use crate::ds::traits::Foldable;
use std::ops::{Index, Range};

#[derive(Debug, Clone)]
pub struct CumulativeSum1D<T, G> {
    data: Vec<T>,
    group: G,
}

pub struct CumulativeSum1DBuilder<T, G> {
    data: Vec<T>,
    group: G,
}

impl<T, G> Foldable<Range<usize>> for CumulativeSum1D<T, G>
where
    T: Copy,
    G: Group<Output = T>,
{
    type Output = T;

    /// Time complexity O(1)
    fn fold(&self, Range { start: l, end: r }: Range<usize>) -> Self::Output {
        self.group.op(self.data[r], self.group.inv(self.data[l]))
    }
}

impl<T, G> Index<usize> for CumulativeSum1D<T, G> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<T, G> CumulativeSum1DBuilder<T, G>
where
    T: Copy,
    G: Group<Output = T> + Clone,
{
    pub fn new(n: usize, group: G) -> Self {
        CumulativeSum1DBuilder {
            data: vec![group.id(); n + 1],
            group,
        }
    }

    pub fn update(&mut self, i: usize, value: T) {
        self.data[i + 1] = self.group.op(self.data[i + 1], value);
    }

    pub fn build(self) -> CumulativeSum1D<T, G> {
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
        let mut csb = CumulativeSum1DBuilder::<i32, _>::new(n, Sum::<i32>::new());

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
