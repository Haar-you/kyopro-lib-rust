use crate::graph::{lowlink::*, *};

pub fn articulation_points<E: EdgeTrait>(g: &Graph<E>) -> Vec<usize> {
    let Lowlink {
        size,
        ord,
        low,
        par,
        ch,
    } = Lowlink::new(&g);

    (0..size)
        .filter(|&i| {
            (par[i].is_none() && ch[i].len() >= 2)
                || ch[i].iter().any(|&j| par[i].is_some() && ord[i] <= low[j])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_3_A
        let mut g = Graph::new(4);
        g.add_undirected(
            vec![(0, 1, 1), (0, 2, 1), (1, 2, 1), (2, 3, 1)]
                .into_iter()
                .map(|(u, v, w)| Edge::new(u, v, w, ()))
                .collect::<Vec<_>>(),
        );
        let mut ans = articulation_points(&g);
        ans.sort();
        assert_eq!(ans, [2]);

        let mut g = Graph::new(5);
        g.add_undirected(
            vec![(0, 1, 1), (1, 2, 1), (2, 3, 1), (3, 4, 1)]
                .into_iter()
                .map(|(u, v, w)| Edge::new(u, v, w, ()))
                .collect::<Vec<_>>(),
        );
        let mut ans = articulation_points(&g);
        ans.sort();
        assert_eq!(ans, [1, 2, 3]);
    }
}
