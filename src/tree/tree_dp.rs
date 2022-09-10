//! 木DP

use crate::tree::*;

pub struct TreeDP<'a, Weight, T> {
    id: T,
    merge: Box<dyn 'a + Fn(T, T) -> T>,
    up: Box<dyn 'a + Fn(T, (usize, Weight)) -> T>,
    apply: Box<dyn 'a + Fn(T, usize) -> T>,
}

impl<'a, Weight, T> TreeDP<'a, Weight, T>
where
    Weight: Copy,
    T: Clone,
{
    pub fn new(
        id: T,
        merge: Box<impl 'a + Fn(T, T) -> T>,
        up: Box<impl 'a + Fn(T, (usize, Weight)) -> T>,
        apply: Box<impl 'a + Fn(T, usize) -> T>,
    ) -> Self {
        Self {
            id,
            merge,
            up,
            apply,
        }
    }

    pub fn run<E: TreeEdgeTrait<Weight = Weight>>(&self, tree: &Tree<E>, root: usize) -> Vec<T> {
        let size = tree.len();
        let mut ret = vec![self.id.clone(); size];

        self.internal(tree, root, None, &mut ret);

        ret
    }

    fn internal<E: TreeEdgeTrait<Weight = Weight>>(
        &self,
        tree: &Tree<E>,
        cur: usize,
        par: Option<usize>,
        ret: &mut Vec<T>,
    ) {
        for e in tree.nodes[cur].neighbors() {
            if Some(e.to()) == par {
                continue;
            }

            self.internal(tree, e.to(), Some(cur), ret);

            let temp = (self.up)(ret[e.to()].clone(), (e.to(), e.weight()));
            ret[cur] = (self.merge)(ret[cur].clone(), temp);
        }
        ret[cur] = (self.apply)(ret[cur].clone(), cur);
    }
}
