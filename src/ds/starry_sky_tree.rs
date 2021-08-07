use crate::ds::traits::{ Foldable, RangeUpdatable };

#[derive(Copy, Clone)]
pub enum StarrySkyTreeMode {
    Max,
    Min
}

/// 区間加算・区間Max(Min)を処理できるデータ構造
pub struct StarrySkyTree<T> {
    size: usize,
    data: Vec<T>,
    zero: T,
    mode: StarrySkyTreeMode
}

impl<T> StarrySkyTree<T>
where
     T: From<i32> + Copy
{
    pub fn new(n: usize, mode: StarrySkyTreeMode) -> Self {
        let size = n.next_power_of_two() * 2;
        let zero = T::from(0);
        StarrySkyTree {
            size: size,
            data: vec![zero; size],
            zero: zero,
            mode: mode
        }
    }
}

impl<T> Foldable<T> for StarrySkyTree<T>
where
    T: std::ops::Add<Output = T> + Ord + Copy
{
    fn fold(&self, l: usize, r: usize) -> T {
        fn rec<T>(data: &[T], i: usize, l: usize, r: usize, s: usize, t: usize, value: T, mode: StarrySkyTreeMode) -> Option<T>
        where
            T: std::ops::Add<Output = T> + Ord + Copy
        {
            if r <= s || t <= l {
                return None;
            }
            if s <= l && r <= t {
                return Some(value + data[i]);
            }

            let a = rec(data, i << 1 | 0, l, (l + r) / 2, s, t, value + data[i], mode);
            let b = rec(data, i << 1 | 1, (l + r) / 2, r, s, t, value + data[i], mode);

            if a.is_none() {
                return b;
            }
            if b.is_none() {
                return a;
            }

            Some(match mode {
                StarrySkyTreeMode::Max => std::cmp::max(a.unwrap(), b.unwrap()),
                StarrySkyTreeMode::Min => std::cmp::min(a.unwrap(), b.unwrap())
            })
        }

        rec(&self.data, 1, 0, self.size / 2, l, r, self.zero, self.mode).unwrap()
    }
}

impl<T> RangeUpdatable<T> for StarrySkyTree<T>
where
    T: std::ops::Add<Output = T> + std::ops::AddAssign + std::ops::SubAssign + Ord + Copy
{
    fn range_update(&mut self, l: usize, r: usize, value: T) {
        let hsize = self.size / 2;
        let mut ll = l + self.size / 2;
        let mut rr = r + self.size / 2;

        while ll < rr {
            if (rr & 1) != 0 {
                rr -= 1;
                self.data[rr] += value;
            }
            if (ll & 1) != 0 {
                self.data[ll] += value;
                ll += 1;
            }
            ll >>= 1;
            rr >>= 1;
        }

        let mut bottom_up = |mut i: usize| {
            if i > self.size {
                return;
            }

            while i >= 1 {
                if i < self.size / 2 {
                    let d = match self.mode {
                        StarrySkyTreeMode::Max => std::cmp::max(self.data[i << 1 | 0], self.data[i << 1 | 1]),
                        StarrySkyTreeMode::Min => std::cmp::min(self.data[i << 1 | 0], self.data[i << 1 | 1])
                    };

                    self.data[i << 1 | 0] -= d;
                    self.data[i << 1 | 1] -= d;
                    self.data[i] += d;
                }

                i >>= 1;
            }
        };

        bottom_up(l + hsize);
        bottom_up(r + hsize);
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_max() {
        let mut rng = rand::thread_rng();

        let size = 100;
        let mut other = vec![0; size];
        let mut s = StarrySkyTree::<i32>::new(size, StarrySkyTreeMode::Max);

        for _ in 0 .. 1000 {
            let ty = rng.gen::<usize>() % 2;
            let l = rng.gen::<usize>() % size;
            let r = l + rng.gen::<usize>() % (size - l) + 1;

            if ty == 0 {
                let x = rng.gen::<i32>() % 1000;

                s.range_update(l, r, x);
                for i in l .. r {
                    other[i] += x;
                }
            }
            else {
                let ans = (l .. r).map(|i| other[i]).max().unwrap();

                assert_eq!(s.fold(l, r), ans);
            }
        }
    }

    #[test]
    fn test_min() {
        let mut rng = rand::thread_rng();

        let size = 100;
        let mut other = vec![0; size];
        let mut s = StarrySkyTree::<i32>::new(size, StarrySkyTreeMode::Min);

        for _ in 0 .. 1000 {
            let ty = rng.gen::<usize>() % 2;
            let l = rng.gen::<usize>() % size;
            let r = l + rng.gen::<usize>() % (size - l) + 1;

            if ty == 0 {
                let x = rng.gen::<i32>() % 1000;

                s.range_update(l, r, x);
                for i in l .. r {
                    other[i] += x;
                }
            }
            else {
                let ans = (l .. r).map(|i| other[i]).min().unwrap();

                assert_eq!(s.fold(l, r), ans);
            }
        }
    }

}
