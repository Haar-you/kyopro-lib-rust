pub use crate::algebra::traits::Group;
pub use crate::ds::traits::Foldable;

#[derive(Debug, Clone)]
pub struct CumulativeSum1D<T, G> {
    data: Vec<T>,
    group: G,
}

pub struct CumulativeSum1DBuilder<T, G> {
    n: usize,
    data: Vec<T>,
    group: G,
}

impl<T, G> CumulativeSum1D<T, G> {
    /// Time complexity O(1)
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T, G> Foldable<T> for CumulativeSum1D<T, G>
where
    T: Clone,
    G: Group<Output = T>,
{
    /// Time complexity O(1)
    fn fold(&self, l: usize, r: usize) -> T {
        self.group
            .op(self.data[r].clone(), self.group.inv(self.data[l].clone()))
    }
}

impl<T, G> std::ops::Index<usize> for CumulativeSum1D<T, G> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<T, G> CumulativeSum1DBuilder<T, G>
where
    T: Clone,
    G: Group<Output = T> + Clone,
{
    pub fn new(n: usize, group: G) -> Self {
        CumulativeSum1DBuilder {
            n: n,
            data: vec![group.id(); n],
            group: group,
        }
    }

    pub fn update(&mut self, i: usize, value: T) -> &Self {
        self.data[i] = self.group.op(self.data[i].clone(), value);
        self
    }

    pub fn build(&self) -> CumulativeSum1D<T, G> {
        let mut data = vec![self.group.id(); self.n + 1];
        for i in 0..self.n {
            data[i + 1] = self.group.op(data[i].clone(), self.data[i].clone());
        }

        CumulativeSum1D {
            data: data,
            group: self.group.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::sum::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 20;
        let mut csb = CumulativeSum1DBuilder::<i32, _>::new(n, Sum::<i32>::new());

        let mut other = vec![0; n];

        for _ in 0..1000 {
            let i = rng.gen::<usize>() % n;
            let x = rng.gen::<i32>() % 1000;

            csb.update(i, x);
            other[i] += x;
        }

        let cs = csb.build();

        for _ in 0..100 {
            let l = rng.gen::<usize>() % n;
            let r = l + rng.gen::<usize>() % (n - l) + 1;

            let mut ans = 0;
            for i in l..r {
                ans += other[i];
            }

            assert_eq!(cs.fold(l, r), ans);
        }
    }
}
