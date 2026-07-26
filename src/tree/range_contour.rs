//! Range Contour Query
//!
//! # References
//! - <https://suisen-kyopro.hatenablog.com/entry/2022/03/21/220009>
//! - <https://noshi91.hatenablog.com/entry/2022/03/27/042143>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/vertex_add_range_contour_sum_on_tree>
//! - <https://judge.yosupo.jp/problem/vertex_get_range_contour_add_on_tree>

use std::collections::VecDeque;

use crate::tree::{centroid_decomposition::CentroidDecomposition, *};

/// 範囲に対するクエリ
#[derive(Clone, Copy, Debug)]
pub enum RangeQuery {
    /// `index`番目のデータ構造での、`0..to`の範囲へのクエリを表す。
    Op {
        /// データ構造の番号
        index: usize,
        /// クエリの範囲の終端
        to: usize,
    },
    /// `index`番目のデータ構造での、`0..to`の範囲への逆元を用いたクエリを表す。
    Rev {
        /// データ構造の番号
        index: usize,
        /// クエリの範囲の終端
        to: usize,
    },
}

/// 一点に対するクエリ
#[derive(Clone, Copy, Debug)]
pub struct VertexQuery {
    /// データ構造の番号
    pub index: usize,
    /// クエリの範囲の終端
    pub pos: usize,
}

/// Range Contour query
///
/// クエリで扱う演算は可逆性と可換性を仮定する。
pub struct RangeContour<W, I> {
    c_decomp: CentroidDecomposition<W, I>,
    bfs_range: Vec<Vec<usize>>,
    bfs_sub: Vec<Vec<Vec<usize>>>,
    index_bfs: Vec<Vec<usize>>,
    index_sub: Vec<Vec<Option<(usize, usize)>>>,
    seq_id_size: Vec<Vec<(usize, usize)>>,
    depth: Vec<Vec<usize>>,
}

impl<W, I> RangeContour<W, I> {
    /// 木から[`RangeContour`]を生成する。
    pub fn new(tree: &Tree<W, I>) -> Self {
        let n = tree.len();
        let c_decomp = CentroidDecomposition::new(tree);
        let depth = c_decomp.depth();
        let root = c_decomp.root();

        let mut this = Self {
            c_decomp,
            bfs_range: vec![vec![]; n],
            bfs_sub: vec![vec![]; n],
            index_bfs: vec![vec![0; n]; depth + 1],
            index_sub: vec![vec![None; n]; depth + 1],
            seq_id_size: vec![vec![]; n],
            depth: vec![vec![0; n]; depth + 1],
        };

        this.dfs(tree, root, &mut vec![false; n]);

        for (index, s) in this.seq_id_size.iter_mut().flatten().enumerate() {
            s.0 = index
        }

        this
    }

    fn dfs(&mut self, tree: &Tree<W, I>, cur: usize, incl: &mut Vec<bool>) -> Vec<usize> {
        let node = &self.c_decomp.nodes()[cur];

        let mut sub = vec![cur];
        let children = node.children.clone();
        for c in children {
            let t = self.dfs(tree, c, incl);
            sub.extend(t);
        }

        for &i in &sub {
            incl[i] = true;
        }

        let node = &self.c_decomp.nodes()[cur];

        let children = tree
            .node_of(cur)
            .neighbors()
            .filter_map(|e| incl[e.to].then_some(e.to))
            .collect::<Vec<_>>();

        let br = &mut self.bfs_range[cur];
        let bs = &mut self.bfs_sub[cur];

        let index_bfs = &mut self.index_bfs[node.depth];
        let index_sub = &mut self.index_sub[node.depth];

        br.push(1);
        index_bfs[cur] = 0;

        bs.resize(children.len(), vec![]);

        let mut deq = VecDeque::new();
        for (i, &c) in children.iter().enumerate() {
            deq.push_back((c, cur, i, 1));
            bs[i].push(0);
        }

        while let Some((c, p, i, d)) = deq.pop_front() {
            if br.len() <= d {
                let index = br[d - 1];
                br.push(index);
            }
            index_bfs[c] = br[d];
            br[d] += 1;

            if bs[i].len() <= d {
                let index = bs[i][d - 1];
                bs[i].push(index);
            }
            index_sub[c] = Some((i, bs[i][d]));
            bs[i][d] += 1;

            self.depth[node.depth][c] = d;

            for e in tree.node_of(c).neighbors() {
                if e.to != p && incl[e.to] {
                    deq.push_back((e.to, c, i, d + 1));
                }
            }
        }

        let mut total = 1;
        for (i, _) in children.iter().enumerate() {
            let size = *bs[i].last().unwrap();
            self.seq_id_size[cur].push((!0, size));
            total += size;
        }

        self.seq_id_size[cur].push((!0, total));

        for &i in &sub {
            incl[i] = false;
        }

        sub
    }

    /// 必要なデータ構造の個数と各サイズを返す。
    pub fn seq_infos(&self) -> impl Iterator<Item = usize> {
        self.seq_id_size
            .iter()
            .flatten()
            .map(|(_, size)| size)
            .cloned()
    }

    /// 頂点`v`から距離`lower`以上`upper`以下の頂点へのクエリ
    pub fn range_contour_query(&self, v: usize, lower: usize, upper: usize) -> Vec<RangeQuery> {
        assert!(lower <= upper);

        let mut cur = v;
        let mut ret = vec![];

        loop {
            let node = &self.c_decomp.nodes()[cur];
            let d = self.depth[node.depth][v];

            let lower = lower.saturating_sub(d);
            if upper >= d {
                let upper = upper - d;

                let br = &self.bfs_range[cur];

                let &(index, _) = self.seq_id_size[cur].last().unwrap();
                if let Some(&r) = br.get(upper).or(br.last())
                    && r > 0
                {
                    ret.push(RangeQuery::Op { index, to: r });
                }
                if lower >= 1
                    && let Some(&l) = br.get(lower - 1).or(br.last())
                    && l > 0
                {
                    ret.push(RangeQuery::Rev { index, to: l });
                }

                if let Some((c, _)) = self.index_sub[node.depth][v] {
                    let (index, _) = self.seq_id_size[cur][c];
                    let bs = &self.bfs_sub[cur][c];
                    if let Some(&r) = bs.get(upper).or(bs.last())
                        && r > 0
                    {
                        ret.push(RangeQuery::Rev { index, to: r });
                    }

                    if lower >= 1
                        && let Some(&l) = bs.get(lower - 1).or(bs.last())
                        && l > 0
                    {
                        ret.push(RangeQuery::Op { index, to: l });
                    }
                }
            }

            if let Some(p) = node.par {
                cur = p;
            } else {
                break;
            }
        }

        ret
    }

    /// 頂点`v`に対するクエリ
    pub fn vertex_query(&self, v: usize) -> Vec<VertexQuery> {
        let mut cur = v;
        let mut ret = vec![];

        loop {
            let node = &self.c_decomp.nodes()[cur];

            let &(index, _) = self.seq_id_size[cur].last().unwrap();
            let i = self.index_bfs[node.depth][v];
            ret.push(VertexQuery { index, pos: i });

            if let Some((c, i)) = self.index_sub[node.depth][v] {
                let (index, _) = self.seq_id_size[cur][c];
                ret.push(VertexQuery { index, pos: i });
            }

            if let Some(p) = node.par {
                cur = p;
            } else {
                break;
            }
        }

        ret
    }
}
