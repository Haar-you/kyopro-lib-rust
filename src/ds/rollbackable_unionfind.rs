//! ロールバック可能Unionfind
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/persistent_unionfind>

enum History {
    Nop,
    Merge(usize, usize, usize),
}

/// ロールバック可能Unionfind
pub struct RollbackableUnionFind {
    parent: Vec<Option<usize>>,
    size: Vec<usize>,
    history: Vec<History>,
}

impl RollbackableUnionFind {
    /// `RollbackableUnionFind`を生成する
    pub fn new(n: usize) -> Self {
        Self {
            parent: vec![None; n],
            size: vec![1; n],
            history: vec![],
        }
    }

    /// `i`の属する素集合の根を返す
    pub fn root_of(&self, i: usize) -> usize {
        if let Some(p) = self.parent[i] {
            self.root_of(p)
        } else {
            i
        }
    }

    /// `i`と`j`が同じ素集合に属するかを判定する
    pub fn is_same(&self, i: usize, j: usize) -> bool {
        self.root_of(i) == self.root_of(j)
    }

    /// `i`の属する素集合と`j`の属する素集合を統合する
    pub fn merge(&mut self, i: usize, j: usize) -> usize {
        let ri = self.root_of(i);
        let rj = self.root_of(j);

        if ri == rj {
            self.history.push(History::Nop);
            return ri;
        }

        if self.size[ri] < self.size[rj] {
            self.history.push(History::Merge(ri, rj, self.size[rj]));
            self.parent[ri] = Some(rj);
            self.size[rj] += self.size[ri];
            rj
        } else {
            self.history.push(History::Merge(rj, ri, self.size[ri]));
            self.parent[rj] = Some(ri);
            self.size[ri] += self.size[rj];
            ri
        }
    }

    /// 直前の`merge`操作を巻き戻す
    pub fn rollback(&mut self) -> bool {
        match self.history.pop() {
            Some(History::Nop) => true,
            Some(History::Merge(c, p, s)) => {
                self.parent[c] = None;
                self.size[p] = s;
                true
            }
            None => false,
        }
    }

    /// `i`の属する素集合の大きさを返す
    pub fn size_of(&self, i: usize) -> usize {
        self.size[self.root_of(i)]
    }
}
