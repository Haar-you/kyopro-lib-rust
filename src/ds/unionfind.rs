use std::cell::Cell;

pub struct UnionFind {
    n: usize,
    count: usize,
    parent: Vec<Cell<usize>>,
    depth: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            n,
            count: n,
            parent: (0..n).map(Cell::new).collect(),
            depth: vec![1; n],
            size: vec![1; n],
        }
    }

    pub fn root_of(&self, i: usize) -> usize {
        if self.parent[i].get() == i {
            return i;
        }
        let p = self.parent[i].get();
        self.parent[i].set(self.root_of(p));
        self.parent[i].get()
    }

    pub fn is_same(&self, i: usize, j: usize) -> bool {
        self.root_of(i) == self.root_of(j)
    }

    pub fn merge(&mut self, i: usize, j: usize) -> usize {
        let i = self.root_of(i);
        let j = self.root_of(j);

        if i == j {
            return i;
        }

        self.count -= 1;

        if self.depth[i] < self.depth[j] {
            self.parent[i].set(j);
            self.size[j] += self.size[i];
            j
        } else {
            self.parent[j].set(i);
            self.size[i] += self.size[j];
            if self.depth[i] == self.depth[j] {
                self.depth[i] += 1;
            }
            i
        }
    }

    pub fn size_of(&self, i: usize) -> usize {
        let i = self.root_of(i);
        self.size[i]
    }

    pub fn count_groups(&self) -> usize {
        self.count
    }

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
