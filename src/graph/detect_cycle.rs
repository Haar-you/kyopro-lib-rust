//! 有向グラフの閉路検出

use crate::graph::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Unchecked,
    Searched,
    Searching,
}

/// 有向グラフの閉路検出
pub fn detect_cycle<D: Direction, E: EdgeTrait>(g: &Graph<D, E>) -> Option<Vec<&E>> {
    let size = g.len();
    let mut check = vec![Status::Unchecked; size];

    for i in 0..size {
        if check[i] == Status::Unchecked {
            let mut ret = vec![];
            rec(g, i, &mut ret, &mut check);

            if !ret.is_empty() {
                ret.reverse();
                return Some(ret);
            }
        }
    }

    None
}

fn rec<'a, D: Direction, E: EdgeTrait>(
    g: &'a Graph<D, E>,
    cur: usize,
    ret: &mut Vec<&'a E>,
    check: &mut [Status],
) -> Option<isize> {
    match check[cur] {
        Status::Searched => None,
        Status::Searching => Some(cur as isize),
        Status::Unchecked => {
            check[cur] = Status::Searching;

            for e in g.nodes[cur].edges.iter() {
                if let Some(res) = rec(g, e.to(), ret, check) {
                    if res != -1 {
                        ret.push(e);
                        if res == cur as isize {
                            return Some(-1);
                        }
                    }

                    return Some(res);
                }
            }

            check[cur] = Status::Searched;

            None
        }
    }
}
