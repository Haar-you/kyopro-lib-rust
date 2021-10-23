use crate::tree::template::*;
use std::mem::swap;

pub fn rooting<T>(tr: &mut Tree<T>, root: usize) -> Result<(), &str> {
    let n = tr.len();
    let mut stack = vec![(root, -1)];
    let mut check = vec![false; n];

    while let Some((cur, par)) = stack.pop() {
        if check[cur] {
            return Err("loop detected");
        }
        check[cur] = true;

        if par == -1 {
            if let Some(p) = tr.nodes[cur].parent.take() {
                tr.nodes[cur].children.push(p);
            }
        } else {
            if let Some(mut p) = tr.nodes[cur].parent.take() {
                for mut e in tr.nodes[cur].children.iter_mut() {
                    if e.to == par as usize {
                        swap(&mut p, &mut e);
                        tr.nodes[cur].parent = Some(p);
                        break;
                    }
                }
            } else {
                for (i, e) in tr.nodes[cur].children.iter().enumerate() {
                    if e.to == par as usize {
                        let x = tr.nodes[cur].children.swap_remove(i);
                        tr.nodes[cur].parent = Some(x);
                        break;
                    }
                }
            }
        }

        for &TreeEdge { to, .. } in &tr.nodes[cur].children {
            stack.push((to, cur as isize));
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut tr = Tree::<u32>::new(6);

        tr.add(0, 1, 1);
        tr.add(1, 2, 1);
        tr.add(2, 3, 1);
        tr.add(2, 4, 1);
        tr.add(5, 1, 1);

        assert_eq!(rooting(&mut tr, 0), Ok(()));

        assert_eq!(
            tr.nodes.into_iter().map(|nd| nd.parent.map(|p| p.to)).collect::<Vec<_>>(),
            vec![None, Some(0), Some(1), Some(2), Some(2), Some(1)]
        );


        let mut tr = Tree::<u32>::new(6);

        tr.add(0, 1, 1);
        tr.add(1, 2, 1);
        tr.add(2, 3, 1);
        tr.add(2, 1, 1);
        tr.add(5, 1, 1);

        assert!(rooting(&mut tr, 0).is_err());
    }
}
