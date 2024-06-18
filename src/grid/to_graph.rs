use crate::{graph::*, grid::*};

/// グリッドをグラフに変換する
///
/// `index`はグリッド上の位置をグラフの頂点番号に対応させる関数。
///
/// `edge`はグリッド上のマス目からマス目への辺を与える関数。
/// 通行不可の場合は`None`を返し、通行可能ならば、そのコストを`Some`に包んで返すこと。
pub fn grid_to_graph<T: Clone>(
    h: usize,
    w: usize,
    dirs: &[Dir],
    index: impl Fn(Position) -> usize,
    edge: impl Fn(Position, Position) -> Option<T>,
) -> Graph<Directed, Edge<T, ()>> {
    let mut g = Graph::<Directed, _>::new(h * w);

    for i in 0..h {
        for j in 0..w {
            let p = Position::new(i, j);

            for d in dirs {
                let q = p.mov_strict(*d, h, w);

                if let Some(q) = q {
                    if let Some(c) = edge(p, q) {
                        let e = Edge::new(index(p), index(q), c, ());
                        g.add(e);
                    }
                }
            }
        }
    }

    g
}
