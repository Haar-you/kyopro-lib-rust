use crate::tree::template::*;

pub struct TreeDP<Weight, T> {
    id: T,
    accum: fn(T, T, (usize, Weight)) -> T,
    apply: fn(T, usize) -> T,
}

impl<Weight, T> TreeDP<Weight, T>
where
    Weight: Copy,
    T: Clone,
{
    pub fn new(
        id: T,
        accum: fn(prev_acc: T, value: T, edge: (usize, Weight)) -> T,
        apply: fn(acc: T, cur: usize) -> T,
    ) -> Self {
        Self { id, accum, apply }
    }

    pub fn run(&self, tree: &Tree<Weight>, root: usize) -> Vec<T> {
        let size = tree.len();
        let mut ret = vec![self.id.clone(); size];

        self.internal(tree, root, &mut ret);

        ret
    }

    fn internal(&self, tree: &Tree<Weight>, cur: usize, ret: &mut Vec<T>) {
        let mut temp = ret[cur].clone();
        for &TreeEdge { to, weight } in &tree.nodes[cur].children {
            self.internal(tree, to, ret);
            temp = (self.accum)(temp.clone(), ret[to].clone(), (to, weight));
        }
        ret[cur] = (self.apply)(temp, cur);
    }
}
