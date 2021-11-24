use crate::graph::*;

pub struct SCC {
    size: usize,
    groups: Vec<Vec<usize>>,
}

impl SCC {
    pub fn new<E: EdgeTrait>(g: &Graph<E>) -> Self {
        let n = g.len();

        let mut check = vec![false; n];
        let mut ord = vec![];
        ord.reserve(n);
        for i in 0..n {
            if !check[i] {
                Self::dfs(&g, i, &mut ord, &mut check);
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

        let mut stack: Vec<usize> = vec![];
        stack.reserve(n);

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

    fn dfs<E: EdgeTrait>(g: &Graph<E>, cur: usize, ord: &mut Vec<usize>, check: &mut [bool]) {
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

        let mut g = Graph::new(5);
        g.add_directed(
            vec![
                (0, 1, 1),
                (1, 0, 1),
                (1, 2, 1),
                (2, 4, 1),
                (4, 3, 1),
                (3, 2, 1),
            ]
            .into_iter()
            .map(|(u, v, w)| Edge::new(u, v, w, ()))
            .collect::<Vec<_>>(),
        );
        let scc = SCC::new(&g).to_vec();

        assert_eq!(scc[0], scc[1]);
        assert_ne!(scc[0], scc[3]);
        assert_eq!(scc[2], scc[3]);
        assert_eq!(scc[3], scc[4]);
    }
}
