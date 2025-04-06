//! 強連結成分分解

use crate::graph::*;

/// 強連結成分分解
pub struct SCC {
    groups: Vec<Vec<usize>>,
    index: Vec<usize>,
}

impl SCC {
    /// グラフから[`SCC`]を構築する。
    ///
    /// **Time complexity** $O(V + E)$
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
        for e in g.nodes_iter().flat_map(|v| &v.edges) {
            rg[e.to()].push(e.from());
        }

        let mut groups = vec![];
        let mut check = vec![false; n];

        let mut stack: Vec<usize> = Vec::with_capacity(n);

        for u in ord {
            if !check[u] {
                let mut temp = vec![];
                stack.push(u);
                while let Some(cur) = stack.pop() {
                    check[cur] = true;
                    for &to in &rg[cur] {
                        if !check[to] {
                            stack.push(to);
                        }
                    }
                    temp.push(cur);
                }
                groups.push(temp);
            }
        }

        let mut index = vec![0; n];
        for (i, s) in groups.iter().enumerate() {
            for &x in s {
                index[x] = i;
            }
        }

        Self { groups, index }
    }

    fn dfs<E: EdgeTrait>(
        g: &Graph<Directed, E>,
        cur: usize,
        ord: &mut Vec<usize>,
        check: &mut [bool],
    ) {
        check[cur] = true;
        for e in g.nodes[cur].edges.iter() {
            if !check[e.to()] {
                Self::dfs(g, e.to(), ord, check);
            }
        }

        ord.push(cur);
    }

    /// 強連結成分を返す。
    pub fn groups(&self) -> &Vec<Vec<usize>> {
        &self.groups
    }

    /// 頂点がどのグループに属しているかを示した`Vec`への参照を返す。
    pub fn index(&self) -> &Vec<usize> {
        &self.index
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
        let scc = SCC::new(&g);
        let scc = scc.index();

        assert_eq!(scc[0], scc[1]);
        assert_ne!(scc[0], scc[3]);
        assert_eq!(scc[2], scc[3]);
        assert_eq!(scc[3], scc[4]);
    }
}
