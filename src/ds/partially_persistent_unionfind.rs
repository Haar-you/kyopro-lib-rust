//! 部分永続UnionFind
//!
//! # Problems
//! - [AGC 002 D - Stamp Rally](https://atcoder.jp/contests/agc002/tasks/agc002_d)
//! - [CODE THANKS FESTIVAL 2017 H - Union Sets](https://atcoder.jp/contests/code-thanks-festival-2017-open/tasks/code_thanks_festival_2017_h)

/// 部分永続UnionFind
pub struct PartiallyPersistentUnionFind {
    time: usize,
    p: Vec<Vec<(usize, usize)>>,
    par: Vec<usize>,
    rank: Vec<usize>,
}

/// ある時間での[`PartiallyPersistentUnionFind`]の状態を参照するための構造体。
pub struct At<'a> {
    time: usize,
    p: &'a Vec<Vec<(usize, usize)>>,
    par: &'a Vec<usize>,
}

impl PartiallyPersistentUnionFind {
    /// 大きさ`size`の[`PartiallyPersistentUnionFind`]を生成する。
    pub fn new(size: usize) -> Self {
        Self {
            time: 0,
            p: vec![vec![(0, 1)]; size],
            par: (0..size).collect(),
            rank: vec![1; size],
        }
    }

    /// 時刻tでの状態
    pub fn at(&self, t: usize) -> At {
        assert!(t <= self.time);
        At {
            time: t,
            p: &self.p,
            par: &self.par,
        }
    }

    /// 最新時点の状態
    pub fn latest(&self) -> At {
        self.at(self.time)
    }

    /// `u`を含む素集合と`v`を含む素集合を融合する。
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
    /// `i`を含む素集合の代表の値を返す。
    pub fn root_of(&self, i: usize) -> usize {
        let &(t, r) = self.p[i].last().unwrap();

        if self.par[i] == i || t == 0 || self.time < t {
            i
        } else if self.time == t {
            r
        } else {
            self.root_of(self.par[i])
        }
    }

    /// `u`と`v`が同じ素集合に含まれていれば`true`を返す。
    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.root_of(u) == self.root_of(v)
    }

    /// `u`が属する素集合の大きさを返す。
    ///
    /// **Time Complexity** $O(\log t)$
    pub fn size_of(&self, u: usize) -> usize {
        let u = self.root_of(u);

        match self.p[u].binary_search(&(self.time + 1, 0)) {
            Ok(i) | Err(i) => self.p[u][i - 1].1,
        }
    }
}
