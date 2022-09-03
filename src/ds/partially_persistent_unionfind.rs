//! 部分永続UnionFind
//! # Problems
//! - [AGC 002 D - Stamp Rally](https://atcoder.jp/contests/agc002/tasks/agc002_d)
//! - [CODE THANKS FESTIVAL 2017 H - Union Sets](https://atcoder.jp/contests/code-thanks-festival-2017-open/tasks/code_thanks_festival_2017_h)

use crate::algo::bsearch::lower_bound;

pub struct PartiallyPersistentUnionFind {
    time: usize,
    p: Vec<Vec<(usize, usize)>>,
    par: Vec<usize>,
    rank: Vec<usize>,
}

pub struct At<'a> {
    p: &'a Vec<Vec<(usize, usize)>>,
    par: &'a Vec<usize>,
    time: usize,
}

impl PartiallyPersistentUnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            time: 0,
            p: vec![vec![(0, 1)]; size],
            par: (0..size).collect(),
            rank: vec![1; size],
        }
    }

    /// 時刻tでの状態
    pub fn at<'a>(&'a self, t: usize) -> At<'a> {
        assert!(t <= self.time);
        At {
            p: &self.p,
            par: &self.par,
            time: t,
        }
    }

    /// 最新時点の状態
    pub fn latest<'a>(&'a self) -> At<'a> {
        self.at(self.time)
    }

    pub fn merge(&mut self, u: usize, v: usize) {
        let t = self.time;
        self.time += 1;

        let u = self.at(t).root_of(u);
        let v = self.at(t).root_of(v);

        if u == v {
            return;
        }

        let s = self.p[u].last().unwrap().1 + self.p[v].last().unwrap().1;

        if self.rank[u] < self.rank[v] {
            self.par[u] = v;
            self.par[v] = v;
            self.p[u].push((self.time, v));
            self.p[v].push((self.time, s));
        } else {
            self.par[u] = u;
            self.par[v] = u;
            self.p[v].push((self.time, u));
            self.p[u].push((self.time, s));
            if self.rank[u] == self.rank[v] {
                self.rank[u] += 1;
            }
        }
    }
}

impl<'a> At<'a> {
    pub fn root_of(&self, i: usize) -> usize {
        if self.par[i] == i {
            return i;
        }

        if self.p[i].last().unwrap().0 == 0 || self.time < self.p[i].last().unwrap().0 {
            return i;
        } else if self.time == self.p[i].last().unwrap().0 {
            return self.p[i].last().unwrap().1;
        } else {
            return self.root_of(self.par[i]);
        }
    }

    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.root_of(u) == self.root_of(v)
    }

    pub fn size_of(&self, u: usize) -> usize {
        let u = self.root_of(u);

        let i = lower_bound(&self.p[u], &(self.time + 1, 0));
        self.p[u][i - 1].1
    }
}
