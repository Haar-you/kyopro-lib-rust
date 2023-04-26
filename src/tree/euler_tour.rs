use crate::tree::*;

pub struct EulerTour {
    begin: Vec<usize>,
    end: Vec<usize>,
}

impl EulerTour {
    pub fn new<E: TreeEdgeTrait>(tree: &Tree<E>, root: usize) -> Self {
        let n = tree.len();
        let mut ret = Self {
            begin: vec![0; n],
            end: vec![0; n],
        };
        ret.dfs(tree, root, None, &mut 0);
        ret
    }

    fn dfs<E: TreeEdgeTrait>(
        &mut self,
        tree: &Tree<E>,
        cur: usize,
        par: Option<usize>,
        pos: &mut usize,
    ) {
        self.begin[cur] = *pos;
        *pos += 1;

        for e in tree.nodes[cur].neighbors() {
            if Some(e.to()) != par {
                self.dfs(tree, e.to(), Some(cur), pos);
            }
        }

        self.end[cur] = *pos;
    }

    pub fn subtree_query(&self, i: usize) -> (usize, usize) {
        (self.begin[i], self.end[i])
    }

    pub fn point_query(&self, i: usize) -> usize {
        self.begin[i]
    }
}
