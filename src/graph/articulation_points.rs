use crate::graph::{lowlink::*, *};

pub fn articulation_points<T>(g: &Graph<T>) -> Vec<usize> {
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
        let g =
            Graph::<u32>::from_tuples_undirected(4, &[(0, 1, 1), (0, 2, 1), (1, 2, 1), (2, 3, 1)]);
        let mut ans = articulation_points(&g);
        ans.sort();
        assert_eq!(ans, [2]);

        let g =
            Graph::<u32>::from_tuples_undirected(5, &[(0, 1, 1), (1, 2, 1), (2, 3, 1), (3, 4, 1)]);
        let mut ans = articulation_points(&g);
        ans.sort();
        assert_eq!(ans, [1, 2, 3]);
    }
}
