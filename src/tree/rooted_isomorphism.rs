use crate::tree::*;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

pub fn rooted_isomorphism<T>(tree: &Tree<T>, root: usize) -> (usize, Vec<usize>) {
    let n = tree.len();
    let mut ret = vec![0; n];
    let mut map = HashMap::new();
    let mut stack = vec![];

    stack.reserve(2 * n);
    stack.push((false, root, None));

    while let Some((back, cur, par)) = stack.pop() {
        if back {
            let mut children = vec![];

            for &TreeEdge { to, .. } in tree.nodes[cur].neighbors() {
                if Some(to) != par {
                    children.push(ret[to]);
                }
            }

            children.sort();

            let mut hasher = DefaultHasher::new();
            children.hash(&mut hasher);
            let h = hasher.finish();

            let k = map.len();
            ret[cur] = *map.entry(h).or_insert(k);
        } else {
            stack.push((true, cur, par));
            for &TreeEdge { to, .. } in tree.nodes[cur].neighbors() {
                if Some(to) != par {
                    stack.push((false, to, Some(cur)));
                }
            }
        }
    }

    (map.len(), ret)
}