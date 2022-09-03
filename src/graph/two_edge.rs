//! 二重辺連結成分分解

pub use crate::graph::lowlink::*;

pub fn two_edge_connected_components(ll: &Lowlink) -> Vec<Vec<usize>> {
    let mut ret = vec![];

    for i in 0..ll.size {
        if ll.par[i].is_none() {
            let index = ret.len();
            ret.push(vec![]);
            rec(&ll, i, index, &mut ret);
        }
    }

    ret
}

fn rec(ll: &Lowlink, cur: usize, index: usize, ret: &mut Vec<Vec<usize>>) {
    ret[index].push(cur);

    for &to in &ll.ch[cur] {
        if ll.ord[cur] < ll.low[to] {
            let index = ret.len();
            ret.push(vec![]);
            rec(ll, to, index, ret);
        } else {
            rec(ll, to, index, ret);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{btreeset, graph::*};
    use std::{collections::BTreeSet, iter::FromIterator};

    #[test]
    fn test() {
        let mut g = Graph::<Undirected, _>::new(4);
        g.extend(
            vec![(0, 2), (0, 1), (3, 0), (2, 1), (2, 3)]
                .into_iter()
                .map(|(u, v)| Edge::new(u, v, (), ())),
        );

        let ans = two_edge_connected_components(&Lowlink::new(&g));
        let ans = BTreeSet::from_iter(ans.into_iter().map(|s| BTreeSet::from_iter(s)));
        assert_eq!(ans, btreeset! {btreeset!{0, 1, 2, 3}});

        let mut g = Graph::<Undirected, _>::new(13);
        g.extend(
            vec![
                (4, 5),
                (8, 7),
                (12, 3),
                (3, 10),
                (1, 5),
                (10, 2),
                (0, 0),
                (11, 4),
                (2, 12),
                (9, 1),
                (9, 0),
                (7, 8),
                (7, 6),
                (9, 1),
                (8, 2),
                (12, 10),
                (11, 0),
                (8, 6),
                (3, 2),
                (5, 9),
                (4, 11),
            ]
            .into_iter()
            .map(|(u, v)| Edge::new(u, v, (), ())),
        );

        let ans = two_edge_connected_components(&Lowlink::new(&g));
        let ans = BTreeSet::from_iter(ans.into_iter().map(|s| BTreeSet::from_iter(s)));

        assert_eq!(
            ans,
            btreeset! {
                btreeset!{0, 1, 4, 5, 9, 11},
                btreeset!{2, 3, 10, 12},
                btreeset!{6, 7, 8}
            }
        );
    }
}
