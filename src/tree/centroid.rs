//! 重心列挙

use crate::tree::*;

pub fn centroids<E: TreeEdgeTrait>(tree: &Tree<E>) -> Vec<usize> {
    let n = tree.len();
    let mut sub = vec![0; n];
    let mut ret = vec![];
    dfs(tree, &mut sub, &mut ret, n, 0, None);
    ret
}

fn dfs<E: TreeEdgeTrait>(
    tree: &Tree<E>,
    sub: &mut [usize],
    ret: &mut Vec<usize>,
    size: usize,
    cur: usize,
    par: Option<usize>,
) {
    sub[cur] = 1;

    let mut check = true;

    for e in tree.nodes[cur].neighbors() {
        if Some(e.to()) == par {
            continue;
        }

        dfs(tree, sub, ret, size, e.to(), Some(cur));

        if sub[e.to()] > size / 2 {
            check = false;
        }
        sub[cur] += sub[e.to()];
    }

    if size - sub[cur] > size / 2 {
        check = false;
    }

    if check {
        ret.push(cur);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut tree = Tree::new(3);
        tree.extend(
            vec![(0, 1), (1, 2)]
                .into_iter()
                .map(|(u, v)| TreeEdge::new(u, v, (), ())),
        );
        assert_eq!(centroids(&tree), vec![1]);

        let mut tree = Tree::new(4);
        tree.extend(
            vec![(0, 1), (1, 2), (2, 3)]
                .into_iter()
                .map(|(u, v)| TreeEdge::new(u, v, (), ())),
        );
        let mut ans = centroids(&tree);
        ans.sort();
        assert_eq!(ans, vec![1, 2]);
    }
}
