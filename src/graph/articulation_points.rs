//! 関節点の列挙

pub use crate::graph::lowlink::*;

/// 関節点の列挙
pub fn articulation_points(ll: &Lowlink) -> Vec<usize> {
    let Lowlink {
        size,
        ord,
        low,
        par,
        ch,
        ..
    } = ll;

    (0..*size)
        .filter(|&i| {
            (par[i].is_none() && ch[i].len() >= 2)
                || (par[i].is_some() && ch[i].iter().any(|&j| ord[i] <= low[j]))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_3_A
        let mut g = Graph::<Undirected, _>::new(4);
        g.extend(
            vec![(0, 1), (0, 2), (1, 2), (2, 3)]
                .into_iter()
                .map(|(u, v)| Edge::new(u, v, (), ())),
        );
        let mut ans = articulation_points(&Lowlink::new(&g));
        ans.sort();
        assert_eq!(ans, [2]);

        let mut g = Graph::<Undirected, _>::new(5);
        g.extend(
            vec![(0, 1), (1, 2), (2, 3), (3, 4)]
                .into_iter()
                .map(|(u, v)| Edge::new(u, v, (), ())),
        );
        let mut ans = articulation_points(&Lowlink::new(&g));
        ans.sort();
        assert_eq!(ans, [1, 2, 3]);
    }
}
