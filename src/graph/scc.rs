//! 強連結成分分解

use crate::graph::*;

pub struct SCC {
    size: usize,
    groups: Vec<Vec<usize>>,
}

impl SCC {
    pub fn new<E: EdgeTrait>(g: &Graph<Directed, E>) -> Self {
        let n = g.len();

        let mut check = vec![false; n];
        let mut ord = Vec::with_capacity(n);
        for i in 0..n {
            if !check[i] {
                Self::dfs(g, i, &mut ord, &mut check);
            }
        }
        ord.reverse();

        let mut rg = vec![vec![]; n];
        for es in &g.edges {
            for e in es {
                rg[e.to()].push(e.from());
            }
        }

        let mut ret = vec![];
        let mut check = vec![false; n];

        let mut stack: Vec<usize> = Vec::with_capacity(n);

        for u in ord {
            if !check[u] {
                ret.push(vec![]);
                stack.push(u);
                while let Some(cur) = stack.pop() {
                    check[cur] = true;
                    for &to in &rg[cur] {
                        if !check[to] {
                            stack.push(to);
                        }
                    }

                    ret.last_mut().unwrap().push(cur);
                }
            }
        }

        Self {
            size: n,
            groups: ret,
        }
    }

    fn dfs<E: EdgeTrait>(
        g: &Graph<Directed, E>,
        cur: usize,
        ord: &mut Vec<usize>,
        check: &mut [bool],
    ) {
        check[cur] = true;
        for e in &g.edges[cur] {
            if !check[e.to()] {
                Self::dfs(g, e.to(), ord, check);
            }
        }

        ord.push(cur);
    }

    pub fn groups(&self) -> &Vec<Vec<usize>> {
        &self.groups
    }

    pub fn to_vec(&self) -> Vec<usize> {
        let mut ret = vec![0; self.size];
        for (i, s) in self.groups.iter().enumerate() {
            for &x in s {
                ret[x] = i;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_3_C

        let mut g = Graph::<Directed, _>::new(5);
        g.extend(
            vec![(0, 1), (1, 0), (1, 2), (2, 4), (4, 3), (3, 2)]
                .into_iter()
                .map(|(u, v)| Edge::new(u, v, (), ())),
        );
        let scc = SCC::new(&g).to_vec();

        assert_eq!(scc[0], scc[1]);
        assert_ne!(scc[0], scc[3]);
        assert_eq!(scc[2], scc[3]);
        assert_eq!(scc[3], scc[4]);
    }
}
