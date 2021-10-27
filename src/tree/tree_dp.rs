use crate::tree::template::*;

pub struct TreeDP<T, U> {
    id: U,
    f: fn(U, U, usize, T) -> U,
}

impl<T, U> TreeDP<T, U>
where
    T: Copy,
    U: Clone,
{
    pub fn new(id: U, f: fn(U, U, usize, T) -> U) -> Self {
        Self { id, f }
    }

    pub fn run(&self, tree: &Tree<T>, root: usize) -> Vec<U> {
        let size = tree.len();
        let mut ret = vec![self.id.clone(); size];

        self.internal(tree, root, &mut ret);

        ret
    }

    fn internal(&self, tree: &Tree<T>, cur: usize, ret: &mut Vec<U>) {
        for &TreeEdge { to, weight } in &tree.nodes[cur].children {
            self.internal(tree, to, ret);
            ret[cur] = (self.f)(ret[cur].clone(), ret[to].clone(), to, weight);
        }
    }
}
