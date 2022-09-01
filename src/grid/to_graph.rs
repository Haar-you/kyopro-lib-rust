use crate::{graph::*, grid::*};

pub fn grid_to_graph<T: Clone>(
    h: usize,
    w: usize,
    dirs: &[Position],
    index: impl Fn(Position) -> usize,
    check_passable: impl Fn(Position, Position) -> bool,
    edge_cost: impl Fn(Position, Position) -> T,
) -> Graph<Directed, Edge<T, ()>> {
    let mut g = Graph::<Directed, _>::new(h * w);

    for i in 0..h {
        for j in 0..w {
            let p = Position::new(i as isize, j as isize);

            for d in dirs {
                let q = p + *d;

                if q.x < 0
                    || q.x >= h as isize
                    || q.y < 0
                    || q.y >= w as isize
                    || !check_passable(p, q)
                {
                    continue;
                }

                let e = Edge::new(index(p), index(q), edge_cost(p, q), ());
                g.add(Some(e));
            }
        }
    }

    g
}
