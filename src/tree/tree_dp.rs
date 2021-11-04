use crate::tree::template::*;

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

    pub fn run(&self, tree: &Tree<Weight>, root: usize) -> Vec<T> {
        let size = tree.len();
        let mut ret = vec![self.id.clone(); size];

        self.internal(tree, root, &mut ret);

        ret
    }

    fn internal(&self, tree: &Tree<Weight>, cur: usize, ret: &mut Vec<T>) {
        for &TreeEdge { to, weight } in &tree.nodes[cur].children {
            self.internal(tree, to, ret);

            let temp = (self.up)(ret[to].clone(), (to, weight));
            ret[cur] = (self.merge)(ret[cur].clone(), temp);
        }
        ret[cur] = (self.apply)(ret[cur].clone(), cur);
    }
}
