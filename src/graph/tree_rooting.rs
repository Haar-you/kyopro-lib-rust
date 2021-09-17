use crate::graph::template::*;

pub fn rooting<T>(g: &mut Graph<T>, root: usize) -> Result<(), &str> {
    let n = g.len();
    let mut stack = vec![(root, -1)];
    let mut check = vec![false; n];

    while let Some((cur, par)) = stack.pop() {
        if check[cur] {
            return Err("loop detected");
        }
        check[cur] = true;

        let mut index = 0;
        while let Some(&Edge { to, .. }) = g.edges[cur].get(index) {
            if to as isize == par {
                g.edges[cur].remove(index);
            } else {
                index += 1;
                stack.push((to, cur as isize));
            }
        }
    }

    Ok(())
}
