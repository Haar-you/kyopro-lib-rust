//! Functional Graph
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc357/tasks/abc357_e>

use crate::ds::unionfind::UnionFind;

/// [`FunctionalGraph`]を構築するための構造体。
pub struct FunctionalGraphBuilder {
    next: Vec<Option<usize>>,
}

/// 頂点の種類
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// 閉路を構成している頂点。
    Loop,
    /// `Loop`でも`Leaf`でもない頂点。
    Branch,
    /// 入次数が`0`の頂点。
    Leaf,
}

impl Kind {
    /// `Loop`ならば`true`を返す。
    pub fn is_loop(self) -> bool {
        matches!(self, Self::Loop)
    }

    /// `Branch`ならば`true`を返す。
    pub fn is_branch(self) -> bool {
        matches!(self, Self::Branch)
    }

    /// `Leaf`ならば`true`を返す。
    pub fn is_leaf(self) -> bool {
        matches!(self, Self::Leaf)
    }
}

/// 連結成分
#[derive(Clone, Default, Debug)]
pub struct Group {
    /// 閉路
    pub loops: Vec<usize>,
    /// 枝
    pub branches: Vec<usize>,
    /// 葉
    pub leaves: Vec<usize>,
}

/// 各頂点の出次数が`1`である、`n`頂点`n`辺の有向グラフ
pub struct FunctionalGraph {
    next: Vec<usize>,
    v_kind: Vec<Kind>,
    group_index: Vec<usize>,
    groups: Vec<Group>,
    children: Vec<Vec<usize>>,
}

impl FunctionalGraphBuilder {
    /// `n`頂点の空なグラフを用意する。
    pub fn new(n: usize) -> Self {
        Self {
            next: vec![None; n],
        }
    }

    /// `from`から`to`への有向辺を追加する。
    pub fn add(&mut self, from: usize, to: usize) {
        assert!(self.next[from].is_none());
        self.next[from] = Some(to);
    }

    /// [`FunctionalGraph`]を構築する。
    pub fn build(self) -> FunctionalGraph {
        assert!(self.next.iter().all(|a| a.is_some()));

        let next = self.next.into_iter().flatten().collect::<Vec<_>>();
        let n = next.len();

        let mut uf = UnionFind::new(n);
        for (cur, &next) in next.iter().enumerate() {
            uf.merge(cur, next);
        }

        let mut index = vec![0; n];
        let g_num = index
            .iter_mut()
            .enumerate()
            .filter_map(|(i, index)| (uf.root_of(i) == i).then_some(index))
            .enumerate()
            .map(|(i, index)| *index = i)
            .count();

        let mut groups = vec![Group::default(); g_num];
        let mut group_index = vec![0; n];

        let mut in_deg = vec![0; n];
        for &next in &next {
            in_deg[next] += 1;
        }

        let mut v_kind = vec![None; n];
        let mut stack = in_deg
            .iter()
            .enumerate()
            .filter_map(|(i, &d)| (d == 0).then_some(i))
            .inspect(|&i| {
                v_kind[i] = Some(Kind::Leaf);
                group_index[i] = index[uf.root_of(i)];
                groups[group_index[i]].leaves.push(i);
            })
            .collect::<Vec<_>>();

        while let Some(cur) = stack.pop() {
            if v_kind[cur].is_none() {
                v_kind[cur] = Some(Kind::Branch);
                group_index[cur] = index[uf.root_of(cur)];
                groups[group_index[cur]].branches.push(cur);
            }

            let next = next[cur];
            in_deg[next] -= 1;
            if in_deg[next] == 0 {
                stack.push(next);
            }
        }

        for i in 0..n {
            if in_deg[i] != 0 {
                v_kind[i] = Some(Kind::Loop);
                group_index[i] = index[uf.root_of(i)];
                groups[group_index[i]].loops.push(i);
            }
        }

        let mut children = vec![vec![]; n];
        for (i, &p) in next.iter().enumerate() {
            if !v_kind[i].unwrap().is_loop() {
                children[p].push(i);
            }
        }

        FunctionalGraph {
            next,
            v_kind: v_kind.into_iter().flatten().collect(),
            group_index,
            groups,
            children,
        }
    }
}

impl FunctionalGraph {
    /// 頂点`i`から辺を辿った次の頂点を返す。
    pub fn next_of(&self, i: usize) -> usize {
        self.next[i]
    }

    /// 頂点`i`の種類を返す。
    pub fn kind_of(&self, i: usize) -> Kind {
        self.v_kind[i]
    }

    /// 頂点`i`が属する連結成分に割り当てられた番号を返す。
    pub fn group_index_of(&self, i: usize) -> usize {
        self.group_index[i]
    }

    /// 頂点`i`の属する連結成分を返す。
    pub fn group_of(&self, i: usize) -> &Group {
        &self.groups[self.group_index_of(i)]
    }

    /// すべての連結成分への参照を返す。
    pub fn groups(&self) -> &[Group] {
        &self.groups
    }

    /// 閉路を切断して、根付き森として見たときの、頂点`i`の子頂点列を返す。
    pub fn children(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        self.children[i].iter().copied()
    }
}
