use crate::ds::unionfind::UnionFind;

pub struct FunctionalGraphBuilder {
    next: Vec<Option<usize>>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind {
    Loop(usize),
    Branch(usize),
    Leaf(usize),
}

pub struct FunctionalGraph {
    next: Vec<usize>,
    v_kind: Vec<Kind>,
    g_num: usize,
}

impl FunctionalGraphBuilder {
    pub fn new(n: usize) -> Self {
        Self {
            next: vec![None; n],
        }
    }

    pub fn add(&mut self, from: usize, to: usize) {
        assert!(self.next[from].is_none());
        self.next[from] = Some(to);
    }

    pub fn build(self) -> FunctionalGraph {
        assert!(self.next.iter().all(|a| a.is_some()));

        let next = self.next.into_iter().flatten().collect::<Vec<_>>();
        let n = next.len();

        let mut uf = UnionFind::new(n);
        for (cur, &next) in next.iter().enumerate() {
            uf.merge(cur, next);
        }

        let mut index = vec![0; n];
        let g_num = index
            .iter_mut()
            .enumerate()
            .filter_map(|(i, index)| (uf.root_of(i) == i).then_some(index))
            .enumerate()
            .map(|(i, index)| *index = i)
            .count();

        let mut in_deg = vec![0; n];
        for &next in &next {
            in_deg[next] += 1;
        }

        let mut v_kind = vec![None; n];
        let mut stack = in_deg
            .iter()
            .enumerate()
            .filter_map(|(i, &d)| (d == 0).then_some(i))
            .inspect(|&i| v_kind[i] = Some(Kind::Leaf(index[uf.root_of(i)])))
            .collect::<Vec<_>>();

        while let Some(cur) = stack.pop() {
            if v_kind[cur].is_none() {
                v_kind[cur] = Some(Kind::Branch(index[uf.root_of(cur)]));
            }

            let next = next[cur];
            in_deg[next] -= 1;
            if in_deg[next] == 0 {
                stack.push(next);
            }
        }

        for i in 0..n {
            if in_deg[i] != 0 {
                v_kind[i] = Some(Kind::Loop(index[uf.root_of(i)]));
            }
        }

        FunctionalGraph {
            next,
            v_kind: v_kind.into_iter().flatten().collect(),
            g_num,
        }
    }
}

impl FunctionalGraph {
    pub fn next_of(&self, i: usize) -> usize {
        self.next[i]
    }

    pub fn kind_of(&self, i: usize) -> Kind {
        self.v_kind[i]
    }

    pub fn grp_index_of(&self, i: usize) -> usize {
        match self.v_kind[i] {
            Kind::Loop(x) | Kind::Branch(x) | Kind::Leaf(x) => x,
        }
    }

    pub fn loops(&self) -> Vec<Vec<usize>> {
        let mut ret = vec![vec![]; self.g_num];

        for (i, a) in self.v_kind.iter().enumerate() {
            if let &Kind::Loop(x) = a {
                ret[x].push(i);
            }
        }

        ret
    }
}
