use crate::algebra::traits::*;
use std::ops::Range;

pub struct Segtree2D<M: Monoid + Commutative> {
    data: Vec<Vec<M::Element>>,
    w: usize,
    h: usize,
    monoid: M,
}

impl<M: Monoid + Commutative> Segtree2D<M>
where
    M::Element: Clone,
{
    /// **Time complexity O(wh)**
    ///
    /// **Space complexity O(wh)**
    pub fn new(w: usize, h: usize, monoid: M) -> Self {
        let w = w.next_power_of_two() * 2;
        let h = h.next_power_of_two() * 2;

        Self {
            data: vec![vec![monoid.id(); h]; w],
            w,
            h,
            monoid,
        }
    }

    fn __fold(&self, l: usize, r: usize, x: usize) -> M::Element {
        let mut l = l + self.h / 2;
        let mut r = r + self.h / 2;

        let mut ret = self.monoid.id();
        let a = &self.data[x];

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                ret = self.monoid.op(ret, a[r].clone());
            }
            if l & 1 == 1 {
                ret = self.monoid.op(ret, a[l].clone());
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }

        ret
    }

    /// **Time complexity O(log w log h)**
    pub fn fold_2d(
        &self,
        Range { start: x1, end: x2 }: Range<usize>,
        Range { start: y1, end: y2 }: Range<usize>,
    ) -> M::Element {
        let mut l = x1 + self.w / 2;
        let mut r = x2 + self.w / 2;

        let mut ret = self.monoid.id();

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                ret = self.monoid.op(ret, self.__fold(y1, y2, r));
            }
            if l & 1 == 1 {
                ret = self.monoid.op(ret, self.__fold(y1, y2, l));
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }

        ret
    }

    /// **Time complexity O(1)**
    pub fn get(&self, i: usize, j: usize) -> M::Element {
        self.data[i + self.w / 2][j + self.h / 2].clone()
    }

    /// **Time complexity O(log w log h)**
    pub fn assign(&mut self, i: usize, j: usize, value: M::Element) {
        let i = i + self.w / 2;
        let j = j + self.h / 2;

        self.data[i][j] = value;

        let mut x = i >> 1;
        while x > 0 {
            self.data[x][j] = self.monoid.op(
                self.data[x << 1][j].clone(),
                self.data[x << 1 | 1][j].clone(),
            );
            x >>= 1;
        }

        let mut y = j >> 1;
        while y > 0 {
            let mut x = i;
            while x > 0 {
                self.data[x][y] = self.monoid.op(
                    self.data[x][y << 1].clone(),
                    self.data[x][y << 1 | 1].clone(),
                );
                x >>= 1;
            }
            y >>= 1;
        }
    }

    /// **Time complexity O(log w log h)**
    pub fn update(&mut self, i: usize, j: usize, value: M::Element) {
        let value = self.monoid.op(value, self.get(i, j));
        self.assign(i, j, value);
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
        let w = 300;
        let h = 100;

        let m = Sum::<u64>::new();
        let mut rng = rand::thread_rng();

        let mut seg = Segtree2D::new(w, h, m);
        let mut a = vec![vec![0; h]; w];

        for i in 0..w {
            for j in 0..h {
                let x = rng.gen::<u64>() % 10000;

                a[i][j] = x;
                seg.assign(i, j, x);
            }
        }

        for _ in 0..100 {
            let i = rng.gen::<usize>() % w;
            let j = rng.gen::<usize>() % h;
            let x = rng.gen::<u64>() % 10000;

            seg.assign(i, j, x);
            a[i][j] = x;

            let wr = rand_range(&mut rng, 0..w);
            let hr = rand_range(&mut rng, 0..h);

            let res = seg.fold_2d(wr.clone(), hr.clone());

            let ans = a[wr]
                .iter()
                .map(|a| a[hr.clone()].iter().sum::<u64>())
                .sum::<u64>();

            assert_eq!(res, ans);
        }
    }
}
