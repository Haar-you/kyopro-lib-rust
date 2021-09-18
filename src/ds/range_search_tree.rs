use crate::algorithm::bsearch::lower_bound;
use crate::utils::merge::merge;

pub struct RangeSearchTree {
    size: usize,
    cxs: Vec<i64>,
    data: Vec<Vec<(i64, usize)>>,
}

impl RangeSearchTree {
    pub fn get(&self, (sx, sy): (i64, i64), (tx, ty): (i64, i64)) -> Vec<(i64, i64)> {
        assert!(sx < tx);
        assert!(sy < ty);

        let mut ret = vec![];
        let mut l = lower_bound(&self.cxs, &sx) + self.size / 2;
        let mut r = lower_bound(&self.cxs, &tx) + self.size / 2;

        while l < r {
            if (r & 1) != 0 {
                r -= 1;
                let a = &self.data[r];

                let mut i = lower_bound(&a, &(sy, 0));

                while i < a.len() && a[i].0 < ty {
                    ret.push((self.cxs[a[i].1], a[i].0));
                    i += 1;
                }
            }

            if (l & 1) != 0 {
                let a = &self.data[l];
                l += 1;

                let mut i = lower_bound(&a, &(sy, 0));

                while i < a.len() && a[i].0 < ty {
                    ret.push((self.cxs[a[i].1], a[i].0));
                    i += 1;
                }
            }

            l >>= 1;
            r >>= 1;
        }

        ret
    }
}

#[derive(Clone, Default)]
pub struct RangeSearchTreeBuilder {
    size: usize,
    xs: Vec<i64>,
    ys: Vec<i64>,
}

impl RangeSearchTreeBuilder {
    pub fn new() -> Self {
        Self {
            size: 0,
            xs: vec![],
            ys: vec![],
        }
    }

    pub fn add(&mut self, x: i64, y: i64) {
        self.size += 1;
        self.xs.push(x);
        self.ys.push(y);
    }

    pub fn build(self) -> RangeSearchTree {
        let mut cxs = self.xs.clone();
        cxs.sort_unstable();
        cxs.dedup();

        let m = cxs.len();
        let size = m.next_power_of_two() * 2;

        let mut data: Vec<Vec<(i64, usize)>> = vec![vec![]; size];

        for i in 0..self.size {
            let j = lower_bound(&cxs, &self.xs[i]);
            data[size / 2 + j].push((self.ys[i], j));
        }

        for item in data.iter_mut().take(size).skip(size / 2) {
            item.sort_unstable();
        }

        for i in (1..size / 2).rev() {
            data[i] = merge(&data[i << 1], &data[i << 1 | 1]);
        }

        RangeSearchTree { size, cxs, data }
    }
}
