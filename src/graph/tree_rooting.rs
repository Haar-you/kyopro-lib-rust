use crate::graph::template::*;

impl<T> Graph<T> {
    pub fn rooting(&mut self, root: usize) -> Result<&mut Self, &str> {
        let n = self.len();
        let mut stack = vec![(root, -1)];
        let mut check = vec![false; n];

        while let Some((cur, par)) = stack.pop() {
            if check[cur] {
                return Err("loop detected");
            }
            check[cur] = true;

            let mut index = 0;
            while let Some(&Edge {
                from: _,
                to,
                cost: _,
            }) = self.edges[cur].get(index)
            {
                if to as isize == par {
                    self.edges[cur].remove(index);
                } else {
                    index += 1;
                    stack.push((to, cur as isize));
                }
            }
        }

        Ok(self)
    }
}
