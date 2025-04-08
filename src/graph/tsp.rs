//! 巡回セールスマン問題
//!
//! # Problems
//! - <https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/all/DPL_2_A>

use crate::graph::*;
use std::ops::Add;

/// 巡回セールスマン問題
pub fn tsp<E: EdgeTrait>(g: &Graph<Directed, E>, src: usize) -> Option<E::Weight>
where
    E::Weight: Copy + Ord + Add<Output = E::Weight>,
{
    let n = g.len();
    let mut dp: Vec<Vec<Option<E::Weight>>> = vec![vec![None; 1 << n]; n];

    for e in g.nodes[src].edges.iter() {
        let (to, cost) = (e.to(), e.weight());
        dp[to][1 << to] = dp[to][1 << to].map(|x| x.min(cost)).or(Some(cost));
    }

    for s in 1..1 << n {
        for i in 0..n {
            if (s & (1 << i)) == 0 {
                continue;
            }

            for e in g.nodes[i].edges.iter() {
                let (to, cost) = (e.to(), e.weight());
                if s & (1 << to) != 0 {
                    continue;
                }

                if let Some(x) = dp[i][s] {
                    let t = s | (1 << to);
                    dp[to][t] = dp[to][t].map(|y| y.min(x + cost)).or(Some(x + cost));
                }
            }
        }
    }

    dp[src][(1 << n) - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/2/DPL_2_A

        let mut g = Graph::<Directed, _>::new(4);
        g.extend(
            vec![
                (0, 1, 2),
                (1, 2, 3),
                (1, 3, 9),
                (2, 0, 1),
                (2, 3, 6),
                (3, 2, 4),
            ]
            .into_iter()
            .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );
        assert_eq!(tsp(&g, 0), Some(16));

        let mut g = Graph::<Directed, _>::new(3);
        g.extend(
            vec![(0, 1, 1), (1, 2, 1), (0, 2, 1)]
                .into_iter()
                .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );
        assert_eq!(tsp(&g, 0), None);
    }
}
