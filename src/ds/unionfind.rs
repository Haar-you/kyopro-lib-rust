pub struct UnionFind {
    n: usize,
    count: usize,
    parent: Vec<usize>,
    depth: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            n,
            count: n,
            parent: (0..n).collect(),
            depth: vec![1; n],
            size: vec![1; n],
        }
    }

    pub fn root_of(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        self.parent[i] = self.root_of(self.parent[i]);
        self.parent[i]
    }

    pub fn is_same(&mut self, i: usize, j: usize) -> bool {
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
            self.parent[i] = j;
            self.size[j] += self.size[i];
            j
        } else {
            self.parent[j] = i;
            self.size[i] += self.size[j];
            if self.depth[i] == self.depth[j] {
                self.depth[i] += 1;
            }
            i
        }
    }

    pub fn size_of(&mut self, i: usize) -> usize {
        let i = self.root_of(i);
        self.size[i]
    }

    pub fn count_groups(&self) -> usize {
        self.count
    }

    pub fn get_groups(&mut self) -> Vec<Vec<usize>> {
        let mut ret = vec![vec![]; self.n];

        for i in 0..self.n {
            ret[self.root_of(i)].push(i);
        }

        ret.into_iter().filter(|x| !x.is_empty()).collect()
    }
}

#[cfg(test)]
mod tests {}
