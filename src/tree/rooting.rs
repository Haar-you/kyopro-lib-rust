//! 根付き木に変換

use crate::tree::*;
use std::mem::swap;

pub fn rooting<E: TreeEdgeTrait>(tr: &mut Tree<E>, root: usize) -> Result<(), &str> {
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
        } else if let Some(mut p) = tr.nodes[cur].parent.take() {
            for e in tr.nodes[cur].children.iter_mut() {
                if e.to() == par as usize {
                    swap(&mut p, e);
                    tr.nodes[cur].parent = Some(p);
                    break;
                }
            }
        } else {
            for (i, e) in tr.nodes[cur].children.iter().enumerate() {
                if e.to() == par as usize {
                    let x = tr.nodes[cur].children.swap_remove(i);
                    tr.nodes[cur].parent = Some(x);
                    break;
                }
            }
        }

        for e in &tr.nodes[cur].children {
            stack.push((e.to(), cur as isize));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut builder = TreeBuilder::new(6);
        builder.extend(
            vec![(0, 1), (1, 2), (2, 3), (2, 4), (5, 1)]
                .into_iter()
                .map(|(u, v)| TreeEdge::new(u, v, (), ())),
        );
        let mut tr = builder.build();
        assert_eq!(rooting(&mut tr, 0), Ok(()));
        assert_eq!(
            tr.nodes
                .into_iter()
                .map(|nd| nd.parent.map(|p| p.to))
                .collect::<Vec<_>>(),
            vec![None, Some(0), Some(1), Some(2), Some(2), Some(1)]
        );

        let mut builder = TreeBuilder::new(6);
        builder.extend(
            vec![(0, 1), (1, 2), (2, 3), (2, 1), (5, 1)]
                .into_iter()
                .map(|(u, v)| TreeEdge::new(u, v, (), ())),
        );
        let mut tr = builder.build();
        assert!(rooting(&mut tr, 0).is_err());
    }
}
