//! 素集合データ構造
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc372/tasks/abc372_e>
use std::cell::Cell;

/// 素集合の統合と所属の判定ができるデータ構造。
pub struct UnionFind<'a, T = ()> {
    n: usize,
    count: usize,
    parent: Vec<Cell<usize>>,
    depth: Vec<usize>,
    size: Vec<usize>,
    values: Option<Vec<Option<T>>>,
    merge: Option<Box<dyn 'a + Fn(T, T) -> T>>,
}

impl UnionFind<'_, ()> {
    /// 大きさ`1`の集合を`n`個用意する。
    pub fn new(n: usize) -> Self {
        UnionFind {
            n,
            count: n,
            parent: (0..n).map(Cell::new).collect(),
            depth: vec![1; n],
            size: vec![1; n],
            values: None,
            merge: None,
        }
    }
}

impl<'a, T> UnionFind<'a, T> {
    /// 大きさ`1`の集合を`|values|`個用意する。このとき、各集合`i`に`value[i]`を割り当てる。
    ///
    /// `merge`は、集合を統合する際に、新しい集合に割り当てる値を返す。
    pub fn with_values(values: Vec<T>, merge: Box<impl 'a + Fn(T, T) -> T>) -> Self {
        let n = values.len();
        UnionFind {
            n,
            count: n,
            parent: (0..n).map(Cell::new).collect(),
            depth: vec![1; n],
            size: vec![1; n],
            values: Some(values.into_iter().map(Option::Some).collect()),
            merge: Some(Box::new(merge)),
        }
    }

    /// `i`の属する集合の根を返す。
    pub fn root_of(&self, i: usize) -> usize {
        if self.parent[i].get() == i {
            return i;
        }
        let p = self.parent[i].get();
        self.parent[i].set(self.root_of(p));
        self.parent[i].get()
    }

    /// `i`と`j`が同じ集合に属するならば`true`を返す。
    pub fn is_same(&self, i: usize, j: usize) -> bool {
        self.root_of(i) == self.root_of(j)
    }

    /// `i`の属する集合と`j`の属する集合を統合する。
    pub fn merge(&mut self, i: usize, j: usize) -> usize {
        let i = self.root_of(i);
        let j = self.root_of(j);

        if i == j {
            return i;
        }

        let (p, c) = if self.depth[i] < self.depth[j] {
            (j, i)
        } else {
            (i, j)
        };

        self.count -= 1;

        self.parent[c].set(p);
        self.size[p] += self.size[c];
        if self.depth[p] == self.depth[c] {
            self.depth[p] += 1;
        }

        if let Some(f) = self.merge.as_ref() {
            let t = f(
                self.values.as_mut().unwrap()[p].take().unwrap(),
                self.values.as_mut().unwrap()[c].take().unwrap(),
            );
            self.values.as_mut().unwrap()[p] = Some(t);
        }

        p
    }

    /// `i`の属する集合の大きさを返す。
    pub fn size_of(&self, i: usize) -> usize {
        let i = self.root_of(i);
        self.size[i]
    }

    /// 素集合の個数を返す。
    pub fn count_groups(&self) -> usize {
        self.count
    }

    /// `i`の属する集合のもつ値を返す。
    pub fn value_of(&self, i: usize) -> Option<&T> {
        let i = self.root_of(i);
        self.values.as_ref()?[i].as_ref()
    }

    /// 素集合をすべて列挙する。
    pub fn get_groups(&self) -> Vec<Vec<usize>> {
        let mut ret = vec![vec![]; self.n];

        for i in 0..self.n {
            ret[self.root_of(i)].push(i);
        }

        ret.into_iter().filter(|x| !x.is_empty()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::btreeset;
    use rand::Rng;
    use std::collections::BTreeSet;
    use std::iter::FromIterator;

    #[test]
    fn test() {
        let n = 100;
        let q = 50;
        let mut rng = rand::thread_rng();

        let mut uf = UnionFind::new(n);
        let mut a = (0..n).map(|i| btreeset![i]).collect::<BTreeSet<_>>();

        for _ in 0..q {
            let i = rng.gen_range(0..n);
            let j = rng.gen_range(0..n);

            uf.merge(i, j);

            let mut ai = a.iter().find(|s| s.contains(&i)).unwrap().clone();
            let aj = a.iter().find(|s| s.contains(&j)).unwrap().clone();

            if ai != aj {
                a.remove(&ai);
                a.remove(&aj);
                ai.extend(aj);
                a.insert(ai);
            }
        }

        for _ in 0..q {
            let i = rng.gen_range(0..n);
            let j = rng.gen_range(0..n);

            let ai = a.iter().find(|s| s.contains(&i)).unwrap();

            assert_eq!(uf.is_same(i, j), ai.contains(&j));
        }

        assert_eq!(
            BTreeSet::from_iter(
                uf.get_groups()
                    .into_iter()
                    .map(|s| BTreeSet::from_iter(s.into_iter()))
            ),
            a
        );
    }
}
