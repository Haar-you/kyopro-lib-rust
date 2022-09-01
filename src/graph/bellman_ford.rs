//! 負閉路を持つグラフの最短経路 (Bellman-Ford)

use crate::graph::*;
pub use crate::utils::num_inf::NumInf;
use std::{cmp::min, ops::Add};

pub fn bellman_ford<D: Direction, T, E: EdgeTrait<Weight = T>>(
    g: &Graph<D, E>,
    src: usize,
) -> Vec<NumInf<T>>
where
    T: Copy + Ord + Default + Add<Output = T>,
{
    use self::NumInf::*;

    let n = g.len();
    let mut ret = vec![Inf; n];

    ret[src] = Value(T::default());

    for i in 0..n {
        for s in 0..n {
            for e in &g.edges[s] {
                let (to, cost) = (e.to(), e.weight());
                if let Value(x) = ret[s] {
                    match ret[to] {
                        Value(y) => {
                            if x + cost < y && i == n - 1 {
                                ret[to] = NegInf;
                            } else {
                                ret[to] = Value(min(y, x + cost));
                            }
                        }
                        Inf => {
                            ret[to] = Value(x + cost);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    for _ in 0..n {
        for s in 0..n {
            for e in &g.edges[s] {
                if ret[s].is_neg_inf() {
                    ret[e.to()] = NegInf;
                }
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::{NumInf::*, *};

    #[test]
    fn test() {
        let mut g = Graph::<Directed, _>::new(4);
        g.add(
            vec![(0, 1, 2), (0, 2, 3), (1, 2, -5), (1, 3, 1), (2, 3, 2)]
                .into_iter()
                .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );
        assert_eq!(
            bellman_ford(&g, 0),
            [Value(0), Value(2), Value(-3), Value(-1)]
        );

        let mut g = Graph::<Directed, _>::new(4);
        g.add(
            vec![(0, 1, 2), (0, 2, 3), (1, 2, -5), (1, 3, 1), (2, 3, 2)]
                .into_iter()
                .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );
        assert_eq!(bellman_ford(&g, 1), [Inf, Value(0), Value(-5), Value(-3)]);
    }
}
