use crate::math::algebra::*;

#[derive(Clone)]
pub struct SegmentTree<T, M> {
    n: usize,
    data: Vec<T>,
    monoid: M,
}

impl<T, M> SegmentTree<T, M>
where
    T: Clone,
    M: BinaryOp<T> + Identity<T>
{
    pub fn new(n: usize, monoid: M) -> Self {
        let size = n.next_power_of_two() * 2;
        SegmentTree {
            n: size,
            data: vec![monoid.id(); size],
            monoid: monoid
        }
    }

    pub fn len(&self) -> usize {
        self.n / 2
    }

    pub fn fold(&self, l: usize, r: usize) -> T {
        let mut ret_l = self.monoid.id();
        let mut ret_r = self.monoid.id();

        let mut l = l + self.n / 2;
        let mut r = r + self.n / 2;

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

    pub fn to_vec(&self) -> Vec<T> {
        return self.data[self.n / 2 ..].to_vec();
    }

    pub fn update(&mut self, i: usize, value: T) {
        self.set(i, self.monoid.op(self.data[i + self.n / 2].clone(), value));
    }

    pub fn set(&mut self, i: usize, value: T) {
        let mut i = i + self.n / 2;
        self.data[i] = value;

        while i > 1 {
            i >>= 1;
            self.data[i] = self.monoid.op(self.data[i << 1 | 0].clone(), self.data[i << 1 | 1].clone());
        }
    }
}






#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn random_test_helper<T, M, F>(size: usize, m: M, mut gen_value: F)
    where
        T: Clone + Eq + std::fmt::Debug,
        M: BinaryOp<T> + Identity<T> + Clone,
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
        }

    }

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();
        random_test_helper(10, Monoid::<i32>::new(0, |x, y| x + y), || rng.gen::<i32>() % 10000);
    }
}
