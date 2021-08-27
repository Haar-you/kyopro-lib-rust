use crate::graph::template::{Edge, Graph};

pub struct DoublingLCA {
    log2n: usize,
    parent: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
}

impl DoublingLCA {
    pub fn new<T>(tree: &Graph<T>, root: usize) -> Self {
        let n = tree.len();
        let log2n = ((n as f64).log2().ceil() + 1.0) as usize;
        let mut ret = Self {
            log2n,
            parent: vec![vec![None; log2n]; n],
            depth: vec![0; n],
        };

        ret.dfs(tree, root, None, 0);
        for k in 0..log2n - 1 {
            for v in 0..n {
                match ret.parent[v][k] {
                    Some(p) => ret.parent[v][k + 1] = ret.parent[p][k],
                    None => ret.parent[v][k + 1] = None,
                }
            }
        }

        ret
    }

    fn dfs<T>(&mut self, tree: &Graph<T>, cur: usize, par: Option<usize>, d: usize) {
        self.parent[cur][0] = par;
        self.depth[cur] = d;

        for &Edge { to, .. } in &tree.edges[cur] {
            if Some(to) != par {
                self.dfs(tree, to, Some(cur), d + 1);
            }
        }
    }

    pub fn get_lca(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] >= self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        for k in 0..self.log2n {
            if ((self.depth[b] - self.depth[a]) >> k & 1) != 0 {
                b = self.parent[b][k].unwrap();
            }
        }
        if a == b {
            return a;
        }

        for k in (0..self.log2n).rev() {
            if self.parent[a][k] != self.parent[b][k] {
                a = self.parent[a][k].unwrap();
                b = self.parent[b][k].unwrap();
            }
        }

        self.parent[a][0].unwrap()
    }
}
