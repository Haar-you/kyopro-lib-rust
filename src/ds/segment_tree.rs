use crate::algebra::traits::Monoid;
use crate::ds::traits::{ Foldable, Updatable, Assignable };

#[derive(Clone)]
pub struct SegmentTree<T, M> {
    original_size: usize,
    size: usize,
    data: Vec<T>,
    monoid: M,
}

impl<T, M> SegmentTree<T, M>
where
    T: Clone,
    M: Monoid<Output = T>
{
    pub fn new(n: usize, monoid: M) -> Self {
        let size = n.next_power_of_two() * 2;
        SegmentTree {
            original_size: n,
            size: size,
            data: vec![monoid.id(); size],
            monoid: monoid
        }
    }

    pub fn len(&self) -> usize {
        self.size / 2
    }
}

impl<T, M> Foldable<T> for SegmentTree<T, M>
where
    T: Clone,
    M: Monoid<Output = T>
{
    fn fold(&self, l: usize, r: usize) -> T {
        let mut ret_l = self.monoid.id();
        let mut ret_r = self.monoid.id();

        let mut l = l + self.size / 2;
        let mut r = r + self.size / 2;

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                ret_r = self.monoid.op(self.data[r].clone(), ret_r);
            }
            if l & 1 == 1 {
                ret_l = self.monoid.op(ret_l, self.data[l].clone());
                l += 1;
            }
            r >>= 1;
            l >>= 1;
        }

        self.monoid.op(ret_l, ret_r)
    }
}

impl<T, M> Assignable<T> for SegmentTree<T, M>
where
    T: Clone,
    M: Monoid<Output = T>
{
    fn assign(&mut self, i: usize, value: T) {
        let mut i = i + self.size / 2;
        self.data[i] = value;

        while i > 1 {
            i >>= 1;
            self.data[i] = self.monoid.op(self.data[i << 1 | 0].clone(), self.data[i << 1 | 1].clone());
        }
    }
}

impl<T, M> Updatable<T> for SegmentTree<T, M>
where
    T: Clone,
    M: Monoid<Output = T>
{
    fn update(&mut self, i: usize, value: T) {
        self.assign(i, self.monoid.op(self.data[i + self.size / 2].clone(), value));
    }
}

impl<T, M> From<SegmentTree<T, M>> for Vec<T>
where
    T: Clone
{
    fn from(from: SegmentTree<T, M>) -> Vec<T> {
        from.data[from.size / 2 .. from.size / 2 + from.original_size].to_vec()
    }
}

impl<T, M> std::ops::Index<usize> for SegmentTree<T, M> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[self.size / 2 + i]
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn random_test_helper<T, M, F>(size: usize, m: M, mut gen_value: F)
    where
        T: Clone + Eq + std::fmt::Debug,
        M: Monoid<Output = T> + Clone,
        F: FnMut() -> T
    {
        let mut rng = rand::thread_rng();

        let mut other = vec![m.id().clone(); size];
        let mut s = SegmentTree::<T, _>::new(size, m.clone());

        for _ in 0 .. 1000 {
            let ty = rng.gen::<usize>() % 2;

            if ty == 0 {
                let i = rng.gen::<usize>() % size;
                let x = gen_value();

                other[i] = m.op(other[i].clone(), x.clone());
                s.update(i, x);
            }
            else {
                let l = rng.gen::<usize>() % size;
                let r = l + rng.gen::<usize>() % (size - l) + 1;

                let mut temp = m.id().clone();
                for i in l .. r {
                    temp = m.op(temp.clone(), other[i].clone());
                }

                assert_eq!(s.fold(l, r), temp);
            }

            let i = rng.gen::<usize>() % size;
            assert_eq!(s[i], other[i]);
        }

        assert_eq!(Vec::<T>::from(s), other);
    }

    use crate::algebra::sum::Sum;
    use crate::algebra::bitxor::BitXor;
    use crate::algebra::min::Min;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        random_test_helper(10, Sum::<i32>::new(), || rng.gen::<i32>() % 10000);
        random_test_helper(10, BitXor::<u32>::new(), || rng.gen::<u32>() % 10000);
        random_test_helper(10, Min::<i32>::new(), || Some(rng.gen::<i32>() % 10000));
    }
}
