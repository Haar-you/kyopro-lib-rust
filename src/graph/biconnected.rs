//! 二重頂点連結分解

pub use crate::graph::lowlink::Lowlink;

type Vertices = Vec<usize>;
type Edges = Vec<(usize, usize)>;

pub fn biconnected(ll: &Lowlink) -> Vec<(Vertices, Edges)> {
    let n = ll.size;

    let mut check = vec![false; n];
    let mut ret = vec![];
    let mut temp = vec![];
    let mut vcheck = vec![false; n];

    for i in 0..n {
        if !check[i] {
            if ll.ch[i].is_empty() && ll.par[i].is_none() {
                ret.push((vec![i], vec![]));
            } else {
                dfs(i, ll, &mut check, &mut vcheck, &mut temp, &mut ret);
            }
        }
    }

    ret
}

fn dfs(
    cur: usize,
    ll: &Lowlink,
    check: &mut [bool],
    vcheck: &mut [bool],
    stack: &mut Vec<(usize, usize)>,
    ret: &mut Vec<(Vertices, Edges)>,
) {
    check[cur] = true;

    for &to in ll.ch[cur].iter().chain(ll.back[cur].iter()) {
        if !check[to] || ll.ord[to] < ll.ord[cur] {
            stack.push((cur, to));
        }
        if !check[to] {
            dfs(to, ll, check, vcheck, stack, ret);

            if ll.low[to] >= ll.ord[cur] {
                let mut es = vec![];
                let mut vs = vec![];

                while let Some(e) = stack.pop() {
                    let (u, v) = e;
                    es.push(e);

                    if !vcheck[u] {
                        vs.push(u);
                        vcheck[u] = true;
                    }
                    if !vcheck[v] {
                        vs.push(v);
                        vcheck[v] = true;
                    }

                    if u == cur && v == to {
                        break;
                    }
                }

                for &i in &vs {
                    vcheck[i] = false;
                }

                ret.push((vs, es));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::biconnected;
    use crate::btreeset;
    use crate::graph::{lowlink::Lowlink, *};
    use std::collections::BTreeSet;
    use std::iter::FromIterator;

    #[test]
    fn test() {
        let mut g = Graph::<Undirected, _>::new(4);
        g.extend(
            vec![(0, 3), (0, 1), (3, 0), (2, 1), (2, 3)]
                .into_iter()
                .map(|(u, v)| Edge::new(u, v, (), ())),
        );

        assert_eq!(
            BTreeSet::from_iter(
                biconnected(&Lowlink::new(&g))
                    .into_iter()
                    .map(|(v, _)| BTreeSet::from_iter(v))
            ),
            btreeset! {
                btreeset!{0, 1, 2, 3}
            }
        );

        let mut g = Graph::<Undirected, _>::new(10);
        g.extend(
            vec![
                (0, 6),
                (0, 8),
                (1, 2),
                (1, 6),
                (2, 6),
                (3, 6),
                (3, 9),
                (4, 9),
                (4, 7),
                (5, 6),
                (5, 9),
                (6, 8),
            ]
            .into_iter()
            .map(|(u, v)| Edge::new(u, v, (), ())),
        );

        assert_eq!(
            BTreeSet::from_iter(
                biconnected(&Lowlink::new(&g))
                    .into_iter()
                    .map(|(v, _)| BTreeSet::from_iter(v))
            ),
            btreeset! {
                btreeset!{0, 6, 8},
                btreeset!{1, 2, 6},
                btreeset!{3, 5, 6, 9},
                btreeset!{4, 7},
                btreeset!{4, 9}
            }
        );

        let mut g = Graph::<Undirected, _>::new(5);
        g.extend(
            vec![(0, 1), (1, 0), (0, 1)]
                .into_iter()
                .map(|(u, v)| Edge::new(u, v, (), ())),
        );

        assert_eq!(
            BTreeSet::from_iter(
                biconnected(&Lowlink::new(&g))
                    .into_iter()
                    .map(|(v, _)| BTreeSet::from_iter(v))
            ),
            btreeset! {
                btreeset!{0, 1},
                btreeset!{2},
                btreeset!{3},
                btreeset!{4}
            }
        );
    }
}
