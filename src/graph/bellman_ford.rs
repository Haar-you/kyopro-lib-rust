use crate::graph::*;
use std::{cmp::min, ops::Add};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BFResult<T> {
    NegInf,
    Value(T),
    PosInf,
}

pub fn bellman_ford<T>(g: &Graph<T>, src: usize) -> Vec<BFResult<T>>
where
    T: Copy + Ord + Default + Add<Output = T>,
{
    use self::BFResult::*;

    let n = g.len();
    let mut ret = vec![PosInf; n];

    ret[src] = Value(T::default());

    for i in 0..n {
        for s in 0..n {
            for &Edge { to, cost, .. } in &g.edges[s] {
                if let Value(x) = ret[s] {
                    match ret[to] {
                        Value(y) => {
                            if x + cost < y && i == n - 1 {
                                ret[to] = NegInf;
                            } else {
                                ret[to] = Value(min(y, x + cost));
                            }
                        }
                        PosInf => {
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
            for &Edge { to, .. } in &g.edges[s] {
                if matches!(ret[s], NegInf) {
                    ret[to] = NegInf;
                }
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::{BFResult::*, *};

    #[test]
    fn test() {
        let g =
            Graph::<i32>::from_tuples(4, &[(0, 1, 2), (0, 2, 3), (1, 2, -5), (1, 3, 1), (2, 3, 2)]);
        assert_eq!(
            bellman_ford(&g, 0),
            [Value(0), Value(2), Value(-3), Value(-1)]
        );

        let g =
            Graph::<i32>::from_tuples(4, &[(0, 1, 2), (0, 2, 3), (1, 2, -5), (1, 3, 1), (2, 3, 2)]);

        assert_eq!(
            bellman_ford(&g, 1),
            [PosInf, Value(0), Value(-5), Value(-3)]
        );
    }
}
