use crate::tree::*;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

pub fn rooted_isomorphism<E: TreeEdgeTrait>(tree: &Tree<E>, root: usize) -> (usize, Vec<usize>) {
    let n = tree.len();
    let mut ret = vec![0; n];
    let mut map = HashMap::new();
    let mut stack = Vec::with_capacity(2 * n);

    stack.push((false, root, None));

    while let Some((back, cur, par)) = stack.pop() {
        if back {
            let mut children = vec![];

            for e in tree.nodes[cur].neighbors() {
                if Some(e.to()) != par {
                    children.push(ret[e.to()]);
                }
            }

            children.sort_unstable();

            let mut hasher = DefaultHasher::new();
            children.hash(&mut hasher);
            let h = hasher.finish();

            let k = map.len();
            ret[cur] = *map.entry(h).or_insert(k);
        } else {
            stack.push((true, cur, par));
            for e in tree.nodes[cur].neighbors() {
                if Some(e.to()) != par {
                    stack.push((false, e.to(), Some(cur)));
                }
            }
        }
    }

    (map.len(), ret)
}
